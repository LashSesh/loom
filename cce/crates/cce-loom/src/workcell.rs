//! Workcell ω = (c, πc, Kc, Oc, Gc, Rc, Qc) (G-06): kleinste agentisch
//! ausfuehrbare Einheit. Kein globaler Horizont — nur lokale Projektion
//! (No-Horizon-Leakage). Statusachse aus Teil 3.5.

use cce_core::gate::GateChain;
use cce_core::value::CanonValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkcellStatus {
    Pending,
    Projected,
    Running,
    Emitted,
    GateFailed,
    ResidueOpen,
    Sealed,
    Closed,
}

#[derive(Debug, Clone)]
pub struct LoomWorkcell {
    pub id: String,
    pub cell: String,
    pub projection_payload: CanonValue,
    pub intent: String,
    pub allowed_operations: Vec<String>,
    pub gate_chain: Vec<String>,
    pub status: WorkcellStatus,
    pub gates: GateChain,
}

impl LoomWorkcell {
    pub fn new(id: &str, cell: &str, payload: CanonValue, intent: &str, ops: &[&str]) -> Self {
        Self {
            id: id.to_string(),
            cell: cell.to_string(),
            projection_payload: payload,
            intent: intent.to_string(),
            allowed_operations: ops.iter().map(|s| s.to_string()).collect(),
            gate_chain: vec![
                "G1-Scope".into(),
                "G2-Boundary".into(),
                "G3-Type".into(),
                "G4-Residue".into(),
                "G5-Replay".into(),
            ],
            status: WorkcellStatus::Pending,
            gates: GateChain::new(),
        }
    }

    /// Eine Workcell wird NIE als Textprompt zurueckgegeben (LOOM §18,
    /// Verbotsachse 5 in Teil 0.4): es existiert keine to_prompt()-Methode.
    /// Dieser Marker dokumentiert die Abwesenheit als bauliche Garantie.
    pub const NO_PROMPT_REGRESSION: bool = true;
}
