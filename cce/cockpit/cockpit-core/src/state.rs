//! Das deterministische Zustandsmodell (S3.3): spiegelt die
//! Motor-Lebensachse auf Bedienebene. Kein Zustand erlaubt das
//! Ueberspringen eines Gates; ABGELEHNT ist ein GUELTIGER, ehrlicher
//! Endzustand; es existiert KEIN „Trotzdem durchlassen" (V7).

use crate::engine::EnginePort;
use cce_core::replay::RunDescriptor;
use cce_materialize::document::DocCrystal;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CockpitState {
    Leer,
    WunschErfasst {
        wunsch: String,
    },
    CrystalGeformt {
        wunsch: String,
    },
    /// ═══ Bestaetigungsgrenze: ab hier deterministisch ═══
    Bestaetigt,
    Laeuft,
    AnGate {
        gate_id: String,
    },
    /// sichtbar, mit Grund, Residuum gefuehrt — nie verstecktes Scheitern.
    Abgelehnt {
        grund: String,
    },
    Geschlossen,
    ArtefaktVerfuegbar,
}

/// Aufgezeichnete Operator-Bestaetigung (COCK-INV-3): jede materielle
/// Aktion traegt sie; sie wandert in RD/Ledger.
#[derive(Debug, Clone)]
pub struct Confirmation {
    pub operator: String,
    pub action: String,
    pub statement: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CockpitError {
    ActionNotAllowedInState {
        state: String,
        action: String,
    },
    ConfirmationRequired {
        action: String,
    },
    /// Es gibt KEINEN Pfad an einem roten harten Gate vorbei.
    NoOverridePath,
}

pub struct CockpitCore<E: EnginePort> {
    pub state: CockpitState,
    pub engine: E,
    pub confirmations: Vec<Confirmation>,
    pub formed_crystal: Option<DocCrystal>,
    pub confirmed_crystal: Option<DocCrystal>,
    pub rd: Option<RunDescriptor>,
}

impl<E: EnginePort> CockpitCore<E> {
    pub fn new(engine: E) -> Self {
        Self {
            state: CockpitState::Leer,
            engine,
            confirmations: Vec::new(),
            formed_crystal: None,
            confirmed_crystal: None,
            rd: None,
        }
    }

    fn confirm(&mut self, c: Option<Confirmation>, action: &str) -> Result<(), CockpitError> {
        match c {
            Some(conf) if !conf.operator.is_empty() => {
                self.confirmations.push(conf);
                Ok(())
            }
            _ => Err(CockpitError::ConfirmationRequired {
                action: action.to_string(),
            }),
        }
    }

    /// Wunsch eingeben (Wunsch-Flaeche, keine materielle Aktion).
    pub fn enter_wish(&mut self, wunsch: &str) -> Result<(), CockpitError> {
        match self.state {
            CockpitState::Leer | CockpitState::WunschErfasst { .. } => {
                self.state = CockpitState::WunschErfasst {
                    wunsch: wunsch.to_string(),
                };
                Ok(())
            }
            _ => Err(CockpitError::ActionNotAllowedInState {
                state: format!("{:?}", self.state),
                action: "enter_wish".into(),
            }),
        }
    }

    /// Kanzel hat geformt: der Crystal liegt zur Pruefung vor.
    /// Motor-Schema rot ⇒ zurueck zu WUNSCH_ERFASST (mit Grund).
    pub fn crystal_formed(&mut self, crystal: DocCrystal) -> Result<(), CockpitError> {
        let CockpitState::WunschErfasst { wunsch } = &self.state else {
            return Err(CockpitError::ActionNotAllowedInState {
                state: format!("{:?}", self.state),
                action: "crystal_formed".into(),
            });
        };
        let wunsch = wunsch.clone();
        let report = self.engine.validate_crystal(&crystal);
        if report.is_pass() {
            self.formed_crystal = Some(crystal);
            self.state = CockpitState::CrystalGeformt { wunsch };
        } else {
            // rot: zurueck mit Grund — der Grund ist Motor-Fakt.
            self.state = CockpitState::WunschErfasst {
                wunsch: format!("{wunsch} [abgewiesen: {}]", report.reason),
            };
        }
        Ok(())
    }

    /// MATERIELLE AKTION: Crystal bestaetigen (Bestaetigungsgrenze).
    pub fn confirm_crystal(&mut self, c: Option<Confirmation>) -> Result<(), CockpitError> {
        if !matches!(self.state, CockpitState::CrystalGeformt { .. }) {
            return Err(CockpitError::ActionNotAllowedInState {
                state: format!("{:?}", self.state),
                action: "confirm_crystal".into(),
            });
        }
        self.confirm(c, "confirm_crystal")?;
        self.confirmed_crystal = self.formed_crystal.take();
        self.state = CockpitState::Bestaetigt;
        Ok(())
    }

    /// MATERIELLE AKTION: Lauf starten.
    pub fn start_run(
        &mut self,
        rd: RunDescriptor,
        c: Option<Confirmation>,
    ) -> Result<(), CockpitError> {
        if self.state != CockpitState::Bestaetigt {
            return Err(CockpitError::ActionNotAllowedInState {
                state: format!("{:?}", self.state),
                action: "start_run".into(),
            });
        }
        self.confirm(c, "start_run")?;
        let crystal = self
            .confirmed_crystal
            .clone()
            .expect("bestaetigt ⇒ Crystal");
        self.rd = Some(rd.clone());
        self.state = CockpitState::Laeuft;
        match self
            .engine
            .submit(crystal, rd)
            .and_then(|()| self.engine.run_to_end())
        {
            Ok(()) => {
                // Motor-Fakten entscheiden den Folgezustand:
                if let Some(hold) = self.engine.gate_reports().iter().find(|g| !g.is_pass()) {
                    self.state = CockpitState::Abgelehnt {
                        grund: format!("{}: {}", hold.gate_id, hold.reason),
                    };
                } else {
                    self.state = CockpitState::Geschlossen;
                    if self.engine.artifact().is_some() {
                        self.state = CockpitState::ArtefaktVerfuegbar;
                    }
                }
                Ok(())
            }
            Err(e) => {
                self.state = CockpitState::Abgelehnt {
                    grund: format!("{e:?}"),
                };
                Ok(())
            }
        }
    }

    /// Es gibt bewusst KEINE Methode, die aus `Abgelehnt` heraus
    /// „durchlaesst": der einzige Weg ist ein NEUER Wunsch/Crystal.
    /// Dieser Aufruf existiert nur, um den Verbotstest sprechend zu
    /// machen — er schlaegt IMMER fehl.
    pub fn force_through(&mut self) -> Result<(), CockpitError> {
        Err(CockpitError::NoOverridePath)
    }

    /// MATERIELLE AKTION: Artefakt exportieren.
    pub fn export_artifact(&mut self, c: Option<Confirmation>) -> Result<String, CockpitError> {
        if self.state != CockpitState::ArtefaktVerfuegbar {
            return Err(CockpitError::ActionNotAllowedInState {
                state: format!("{:?}", self.state),
                action: "export_artifact".into(),
            });
        }
        self.confirm(c, "export_artifact")?;
        Ok("export:ledger-verankert".to_string())
    }

    /// Replay (COCK-INV-5): reproduziert aus bestaetigtem Crystal + RD —
    /// der Pfad hat keinen Kanzel-Parameter und keinen LLM-Zugang.
    pub fn replay_class_hex(&self) -> Option<String> {
        self.engine.replay_class().map(|d| d.to_hex())
    }
}
