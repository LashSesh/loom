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

/// Referenz-Kanzel ueber das InferenceGateway (LocalModel/Mock):
/// formt den Drei-Risiken-Memo-Crystal deterministisch.
pub struct LocalKanzel;

impl KanzelPort for LocalKanzel {
    fn form_wish(&self, wunsch: &str) -> Option<(DocCrystal, Interpretation)> {
        // Referenzpfad (Dokument-Domaene): der Wunschtext wird auf die
        // Wunsch-Normalform des Memo-Assets abgebildet.
        let crystal = cce_materialize::document::assets::three_risks_memo();
        Some((
            crystal,
            Interpretation {
                text: format!(
                    "Aus dem Wunsch '{wunsch}' wurde ein Dokument-Crystal geformt: \
                     3 Risiko-Zellen je mit Gegenmassnahme-Naht, keine Score-Felder (V1). \
                     Annahmen: modellgeformt — bitte pruefen und bestaetigen."
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
