//! Der Lauf-Lebenszyklus (S5.1) ueber dem geschlossenen Dokument-Pfad:
//! encode → project → loom → materialize → reanalyze → equivalent.
//! Pausierbar, fortsetzbar, HITL-faehig — und dennoch deterministisch:
//! die menschliche Entscheidung wird EINMAL erfasst und ist danach ein
//! fixer Eingang (S5.4); Replay spielt die Aufzeichnung ab.

use crate::checkpoint::Checkpoint;
use cce_core::gate::{GateChain, GateReport};
use cce_core::ledger::{Ledger, LedgerEventKind};
use cce_core::replay::{HitlDecision, RunDescriptor};
use cce_core::signature::{sha256, Digest};
use cce_materialize::adapter::DomainAdapter;
use cce_materialize::document::{DocArtifact, DocCrystal, DocumentAdapter};
use cce_phc::projection_calc::project;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    Encode,
    Project,
    Loom,
    Materialize,
    Reanalyze,
    Equivalent,
}

pub const STAGES: [Stage; 6] = [
    Stage::Encode,
    Stage::Project,
    Stage::Loom,
    Stage::Materialize,
    Stage::Reanalyze,
    Stage::Equivalent,
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RunStatus {
    Submitted,
    Running,
    Paused,
    AtGate { gate: String },
    Rejected { reason: String },
    Closed,
}

#[derive(Debug)]
pub enum RunError {
    /// Harte Gates pausieren NIE fuer menschliche Eingabe (S5.4).
    HardGateNotPausable(String),
    Internal(String),
}

/// Quelle einer Ermessens-Entscheidung (nur im Erstlauf befragt;
/// beim Replay werden die aufgezeichneten Entscheidungen abgespielt).
pub trait DecisionProvider {
    fn decide(&self, gate: &str, context: &str) -> HitlDecision;
}

/// Der Lauf.
pub struct Run {
    pub rd: RunDescriptor,
    pub crystal: DocCrystal,
    pub status: RunStatus,
    pub stage_index: usize,
    pub checkpoints: Vec<Checkpoint>,
    pub ledger: Ledger,
    pub gates: GateChain,
    pub artifact: Option<DocArtifact>,
    pub reanalyzed: Option<DocCrystal>,
    /// Ermessens-Gates, die auf diesem Lauf liegen (Name → Kontext).
    pub discretionary_gates: Vec<(String, String)>,
    /// S-E5 §5/§10(g): die vom Aufrufer bereits AUFGELOESTEN Normen, die
    /// laut `rd.norm_profile` aktiviert werden sollen. Die Aufloesung
    /// selbst (norm_id -> BridgeNorm, ueber die Klassen-Registry) ist
    /// Sache der Schicht oberhalb des Runners (dieselbe Trennung wie
    /// SeedResolver/RegistryResolver vs. CitationGate) — der Runner
    /// prueft nur noch die Aktivierungs-Voraussetzung, fail-closed.
    pub activated_norms: Vec<cce_bridge::BridgeNorm>,
    decisions_replayed: usize,
    adapter: DocumentAdapter,
}

impl Run {
    pub fn submit(crystal: DocCrystal, rd: RunDescriptor) -> Result<Run, RunError> {
        rd.validate()
            .map_err(|e| RunError::Internal(format!("{e:?}")))?;
        Ok(Run {
            rd,
            crystal,
            status: RunStatus::Submitted,
            stage_index: 0,
            checkpoints: Vec::new(),
            ledger: Ledger::new(),
            gates: GateChain::new(),
            artifact: None,
            reanalyzed: None,
            discretionary_gates: Vec::new(),
            activated_norms: Vec::new(),
            decisions_replayed: 0,
            adapter: DocumentAdapter,
        })
    }

    fn state_value(&self) -> cce_core::value::CanonValue {
        use cce_core::canonical::Canonicalize;
        self.crystal.canonical_value()
    }

    fn checkpoint(&mut self) {
        let cp = Checkpoint::capture(self.stage_index, &self.state_value(), &self.rd.decisions);
        self.ledger.append(LedgerEventKind::Execute, cp.state_class);
        self.checkpoints.push(cp);
    }

    /// Versucht, ein HARTES Gate zu pausieren: baulich unmoeglich (S5.4).
    pub fn request_pause_at_hard_gate(&self, gate_id: &str) -> Result<(), RunError> {
        Err(RunError::HardGateNotPausable(format!(
            "{gate_id}: harte Gates sind automatisch fail-closed und pausieren nie"
        )))
    }

    /// Operator-Pause (nur zwischen Stufen; Bedien-Bequemlichkeit, S5.3).
    pub fn pause(&mut self) {
        if self.status == RunStatus::Running || self.status == RunStatus::Submitted {
            self.checkpoint();
            self.status = RunStatus::Paused;
        }
    }

    pub fn resume(&mut self) {
        if self.status == RunStatus::Paused {
            self.status = RunStatus::Running;
        }
    }

    /// Ermessens-Gate erreichen: Entscheidung aus Aufzeichnung (Replay)
    /// oder vom Provider (Erstlauf); wird Ledger- und RD-gebunden (S5.4/5.5).
    fn resolve_discretionary(
        &mut self,
        gate: &str,
        context: &str,
        provider: Option<&dyn DecisionProvider>,
    ) -> Result<HitlDecision, RunError> {
        if let Some(d) = self.rd.decisions.get(self.decisions_replayed) {
            // Replay: aufgezeichnete Entscheidung abspielen, NIE neu fragen.
            let d = d.clone();
            self.decisions_replayed += 1;
            self.ledger
                .append(LedgerEventKind::Decision, sha256(d.decision.as_bytes()));
            return Ok(d);
        }
        let provider = provider.ok_or_else(|| {
            RunError::Internal(format!("Ermessens-Gate {gate} ohne Entscheidungsquelle"))
        })?;
        self.status = RunStatus::AtGate {
            gate: gate.to_string(),
        };
        let mut d = provider.decide(gate, context);
        d.rd_ref = Some("rd-1".to_string());
        self.rd.decisions.push(d.clone());
        self.decisions_replayed += 1;
        self.ledger
            .append(LedgerEventKind::Decision, sha256(d.decision.as_bytes()));
        self.status = RunStatus::Running;
        Ok(d)
    }

    /// Fuehrt den Lauf bis zum Ende (oder Ablehnung) aus.
    pub fn run_to_end(&mut self, provider: Option<&dyn DecisionProvider>) -> Result<(), RunError> {
        self.status = RunStatus::Running;
        while self.stage_index < STAGES.len() {
            if self.status == RunStatus::Paused {
                return Ok(());
            }
            self.step(provider)?;
            if matches!(self.status, RunStatus::Rejected { .. }) {
                return Ok(());
            }
        }
        Ok(())
    }

    /// Eine Stufe (deterministisch); Checkpoint an jeder Grenze (S5.2).
    pub fn step(&mut self, provider: Option<&dyn DecisionProvider>) -> Result<(), RunError> {
        let stage = STAGES[self.stage_index];
        match stage {
            Stage::Encode => {
                let pkg = self.adapter.encode(&self.crystal);
                let root = pkg.root_hash();
                cce_phc::loader::load_phc(&pkg, Some(root))
                    .map_err(|e| self.reject(&format!("Loader {:?}: {}", e.phase, e.reason)))
                    .ok();
                self.ledger.append(LedgerEventKind::Decode, root);
                self.gates
                    .push(GateReport::pass("G3-Type", "Paket typkorrekt, V0–V9 gruen"));
            }
            Stage::Project => {
                let pkg = self.adapter.encode(&self.crystal);
                let proj = project(&pkg, "proj:materialize")
                    .map_err(|e| RunError::Internal(format!("{e:?}")))?;
                self.ledger
                    .append(LedgerEventKind::Project, sha256(proj.cell_id.as_bytes()));
                self.gates
                    .push(GateReport::pass("G1-Scope", "Projektion im Scope"));
            }
            Stage::Loom => {
                // S-E5 §5: norm_profile ist ein fail-closed Zusatz-Gate
                // dieses Laufs, VOR dem Weben (wie die Ermessens-Gates).
                self.apply_norm_profile();
                if matches!(self.status, RunStatus::Rejected { .. }) {
                    return Ok(());
                }
                // Ermessens-Gates dieses Laufs (falls konfiguriert) VOR dem Weben.
                let pending: Vec<(String, String)> = self.discretionary_gates.clone();
                for (gate, ctx) in pending {
                    let d = self.resolve_discretionary(&gate, &ctx, provider)?;
                    if d.decision == "reject" {
                        self.status = RunStatus::Rejected {
                            reason: format!("Operator-Entscheidung an {gate}: reject"),
                        };
                        return Ok(());
                    }
                }
                self.gates.push(GateReport::pass(
                    "G2-Boundary",
                    "Nullanker markiert, Naht gebunden",
                ));
            }
            Stage::Materialize => {
                let pkg = self.adapter.encode(&self.crystal);
                let proj = project(&pkg, "proj:materialize")
                    .map_err(|e| RunError::Internal(format!("{e:?}")))?;
                let weave = self.adapter.loom(&proj).map_err(RunError::Internal)?;
                let artifact = self.adapter.materialize(&weave);
                self.ledger
                    .append(LedgerEventKind::Materialize, artifact.byte_digest());
                self.gates.push(GateReport::pass(
                    "G6-Export",
                    "Materialisierung nach Gate/Evidence",
                ));
                self.artifact = Some(artifact);
            }
            Stage::Reanalyze => {
                let artifact = self
                    .artifact
                    .as_ref()
                    .ok_or_else(|| RunError::Internal("kein Artefakt".into()))?;
                match self.adapter.reanalyze(artifact) {
                    Ok(back) => {
                        self.gates
                            .push(GateReport::pass("G4-Residue", "Reanalyse vollstaendig"));
                        self.reanalyzed = Some(back);
                    }
                    Err(e) => {
                        self.status = RunStatus::Rejected {
                            reason: format!("semantic_loss: {e}"),
                        };
                        return Ok(());
                    }
                }
            }
            Stage::Equivalent => {
                let back = self
                    .reanalyzed
                    .as_ref()
                    .ok_or_else(|| RunError::Internal("keine Reanalyse".into()))?;
                if self.adapter.equivalent(back, &self.crystal) {
                    self.gates
                        .push(GateReport::pass("G7-Reanalysis", "q(Obs(A)) = q(C)"));
                    self.ledger
                        .append(LedgerEventKind::Commit, self.result_class());
                    self.status = RunStatus::Closed;
                } else {
                    self.status = RunStatus::Rejected {
                        reason: "Reanalysis-Divergenz (G7 rot)".to_string(),
                    };
                    return Ok(());
                }
            }
        }
        self.stage_index += 1;
        self.checkpoint();
        Ok(())
    }

    /// S-E5 §5/§10(g): der Aktivierungs-Hook. Jede in `rd.norm_profile`
    /// gelistete `norm_id` MUSS unter `activated_norms` als `Active`
    /// aufloesbar sein — sonst rejected der Lauf fail-closed
    /// (`norm_not_activated`, N-NRM-6/PROD-INV-22). Ist `norm_profile`
    /// leer, wirkt KEINE Norm, selbst wenn `activated_norms` befuellt
    /// ist (A3: strikt opt-in).
    fn apply_norm_profile(&mut self) {
        if self.rd.norm_profile.is_empty() {
            return;
        }
        let profile = cce_bridge::activation::NormProfile::new(self.rd.norm_profile.clone());
        for norm_id in self.rd.norm_profile.clone() {
            let resolved = self.activated_norms.iter().find(|n| n.norm_id == norm_id);
            let outcome = match resolved {
                Some(norm) => cce_bridge::activation::activate(norm, &profile),
                None => Err(cce_bridge::activation::ActivationError::NotActivated),
            };
            match outcome {
                Ok(marker) => {
                    self.gates.push(GateReport::pass("G-Norm", &marker));
                }
                Err(e) => {
                    self.status = RunStatus::Rejected {
                        reason: format!("norm_profile[{norm_id}]: {}", e.residue()),
                    };
                    return;
                }
            }
        }
    }

    fn reject(&mut self, reason: &str) -> RunError {
        self.status = RunStatus::Rejected {
            reason: reason.to_string(),
        };
        RunError::Internal(reason.to_string())
    }

    /// Die Ergebnis-KLASSE des Laufs (INV-10-Traeger): Inhaltsklasse des
    /// reanalysierten Kristalls (nie Artefakt-Bytes).
    pub fn result_class(&self) -> Digest {
        self.reanalyzed
            .as_ref()
            .map(|c| self.adapter.canonicalize(c).0)
            .unwrap_or(Digest([0u8; 32]))
    }
}
