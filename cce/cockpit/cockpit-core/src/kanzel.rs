//! Die KI-Kanzel als PORT (S3.4 + Overlay C.12): formen/erklaeren —
//! read-only gegenueber dem Motor, jede Ausgabe als INTERPRETATION
//! markiert (COCK-INV-4), ohne konfigurierten Anbieter lauffaehig-
//! DEGRADIERT (Kanzel-Aus-Modus: Cockpit voll funktionsfaehig).
//! Es gibt strukturell keinen Schreibpfad: alle Methoden nehmen
//! &self + Motor-Fakten und liefern Texte/Entwuerfe.

use cce_core::gate::GateReport;
use cce_materialize::document::DocCrystal;

/// Jede Kanzel-Ausgabe traegt diese Markierung fest im Typ.
#[derive(Debug, Clone)]
pub struct Interpretation {
    pub text: String,
    /// fest: "Interpretation, kein Motor-Urteil"
    pub marker: &'static str,
}

pub const INTERPRETATION_MARKER: &str = "Interpretation, kein Motor-Urteil";

pub trait KanzelPort {
    /// Formt aus Klartext einen Crystal-ENTWURF (bindend wird er erst
    /// durch Motor-Validierung + Operator-Bestaetigung, S4-A4).
    fn form_wish(&self, wunsch: &str) -> Option<(DocCrystal, Interpretation)>;
    /// Erklaert Motor-Fakten in Klartext.
    fn explain_gate(&self, report: &GateReport) -> Interpretation;
    /// Sichtbarer Zustand der Kanzel (fuer ProviderStatus-Ansicht).
    fn status(&self) -> &'static str;
}

/// Kanzel-Aus / DegradedMode: KEIN Anbieter. form liefert None
/// (sichtbar degradiert), explain liefert die Roh-Begruendung des
/// Motors mit Degradations-Hinweis — das Cockpit bleibt voll bedienbar
/// (der Operator formt den Crystal selbst).
pub struct DegradedKanzel;

impl KanzelPort for DegradedKanzel {
    fn form_wish(&self, _wunsch: &str) -> Option<(DocCrystal, Interpretation)> {
        None
    }

    fn explain_gate(&self, report: &GateReport) -> Interpretation {
        Interpretation {
            text: format!(
                "[degradiert — keine Kanzel] Motor-Begruendung woertlich: {}: {}",
                report.gate_id, report.reason
            ),
            marker: INTERPRETATION_MARKER,
        }
    }

    fn status(&self) -> &'static str {
        "degradiert (kein Anbieter konfiguriert) — Kern voll funktionsfaehig"
    }
}

/// Referenz-Kanzel ueber das InferenceGateway (Block 2, Track C):
/// formt den Drei-Risiken-Memo-Crystal (Motor-Referenzstruktur,
/// unveraendert) UND laesst die Annahmen-Erlaeuterung real vom
/// `LocalExtractiveModel` bilden — durch das UNVERAENDERTE
/// InferenceGateway (`run_inference`), kein Stub, kein fester String.
/// Die tiefe Crystal-STRUKTUR bleibt die gepruefte Referenzform (deren
/// Aenderung waere Motor-/Domain-Adapter-Arbeit, nicht Kanzel-Arbeit);
/// was echt modellgeformt ist, ist die INTERPRETATION — exakt der Teil,
/// den Handbuch §3 als "Annahmen ... als menschen- oder modellgeformt
/// gekennzeichnet" beschreibt.
pub struct LocalKanzel;

impl LocalKanzel {
    /// Ruft das InferenceGateway wirklich auf (kein Stub): baut den
    /// Request ueber die bestehende `cce_inference::kanzel::Kanzel`
    /// (C.12), laeuft durch die volle, unveraenderte Vor-Egress-Gate-
    /// Kette und liefert den ECHTEN Modelltext + die Gateway-Evidence
    /// als Herkunftsbeleg (manifest_ref/evidence_id) zurueck.
    fn model_formed_text(&self, wunsch: &str) -> String {
        use cce_inference::gateway::{run_inference, GatewayOutcome, InferenceRecorder};
        use cce_inference::providers::local_extractive::LocalExtractiveModel;

        let gateway_kanzel = cce_inference::kanzel::Kanzel;
        let req = gateway_kanzel.form_wish_request(wunsch, "proj:cockpit-wunsch");
        let provider = LocalExtractiveModel::new("kernmodell");
        let boundary = vec!["wunsch".to_string()];
        let mut recorder = InferenceRecorder::default();
        match run_inference(
            &provider,
            &req,
            &boundary,
            "no_pii",
            None,
            0,
            "draft",
            &mut recorder,
        ) {
            GatewayOutcome::Candidate(boxed) => {
                let (candidate, evidence) = *boxed;
                format!(
                    "Modellgeformt (Provider {}, Beleg {}): {}",
                    evidence.manifest_ref, evidence.evidence_id, candidate.content
                )
            }
            other => format!(
                "Kanzel-Anfrage vor Egress gestoppt ({other:?}) — Annahmen bitte \
                 manuell pruefen; die Crystal-Referenzstruktur bleibt unberuehrt."
            ),
        }
    }
}

impl KanzelPort for LocalKanzel {
    fn form_wish(&self, wunsch: &str) -> Option<(DocCrystal, Interpretation)> {
        // Referenzpfad (Dokument-Domaene): der Wunschtext wird auf die
        // Wunsch-Normalform des Memo-Assets abgebildet — dieser Teil ist
        // Motor-Struktur, kein Kanzel-Ermessen.
        let crystal = cce_materialize::document::assets::three_risks_memo();
        let model_text = self.model_formed_text(wunsch);
        Some((
            crystal,
            Interpretation {
                text: format!(
                    "Aus dem Wunsch '{wunsch}': {model_text} \
                     Struktur: 3 Risiko-Zellen je mit Gegenmassnahme-Naht, keine \
                     Score-Felder (V1). Annahmen: modellgeformt — bitte pruefen und \
                     bestaetigen."
                ),
                marker: INTERPRETATION_MARKER,
            },
        ))
    }

    fn explain_gate(&self, report: &GateReport) -> Interpretation {
        Interpretation {
            text: format!(
                "Das Gate {} ist {}: {}. Dies ist eine Uebersetzung des Motor-Fakts.",
                report.gate_id,
                if report.is_pass() { "gruen" } else { "rot" },
                report.reason
            ),
            marker: INTERPRETATION_MARKER,
        }
    }

    fn status(&self) -> &'static str {
        "aktiv (lokal, kein Egress)"
    }
}
