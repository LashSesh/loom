//! Die schreiblose Kanzel-API (C.12, F.1 j) — Client des Gateways,
//! nie eigener Netzpfad. Drei Faehigkeiten: form / explain / propose.
//!
//! SCHREIBLOS heisst strukturell: jede Funktion nimmt &self/&-Referenzen
//! und liefert Texte oder InferenceRequests — es gibt keinen Parameter
//! und keinen Rueckgabetyp, ueber den ein Gate-Urteil, ein Residuum
//! oder ein Ledger-Eintrag geschrieben werden koennte.

use crate::request::{ContextSlice, InferenceRequest};
use cce_core::gate::GateReport;
use cce_core::residue::Residue;

/// Die Kanzel: zustandslos gegenueber dem Motor.
pub struct Kanzel;

impl Kanzel {
    /// Wuensche formulieren HELFEN (S4-A4): formt einen InferenceRequest
    /// — bindend wird erst Motor-Validierung + Bestaetigung.
    pub fn form_wish_request(&self, wish_text: &str, projection_id: &str) -> InferenceRequest {
        let mut req = InferenceRequest::example("kanzel:form");
        req.projection_id = projection_id.to_string();
        req.allowed_context = vec!["wunsch".to_string()];
        req.context = vec![ContextSlice {
            name: "wunsch".to_string(),
            content: wish_text.to_string(),
        }];
        req.output_schema = "wunsch-entwurf".to_string();
        req.system_contract = "annahmen als modellgeformt markieren".to_string();
        req
    }

    /// ProjectionPackets/Residuen ERKLAEREN — reine Textausgabe.
    pub fn explain_residue(&self, residue: &Residue) -> String {
        format!(
            "Erklaerung (Einschaetzung, kein Urteil): Residuum {} aus {} — {}. \
             Es bleibt sichtbar, bis der Motor es schliesst.",
            residue.id, residue.origin, residue.content
        )
    }

    /// Reparaturvorschlaege FORMULIEREN — als Vorschlagstext, nie als
    /// Aktion; materiell wird daraus erst ein ActionCandidate unter
    /// CapabilityLock + HumanConfirmationGate.
    pub fn propose_repair(&self, gate_report: &GateReport) -> String {
        format!(
            "Vorschlag (kandidat, kein urteil) zu Gate {}: {} — \
             Umsetzung nur via Motor-Gates + Bestaetigung.",
            gate_report.gate_id, gate_report.reason
        )
    }
}
