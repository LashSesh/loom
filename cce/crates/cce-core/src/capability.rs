//! CapabilityLock (S13-A1/S15.10): jede materielle Faehigkeit — Lauf,
//! Promotion, Klonung, Export, Netzzugriff, Modell-/Tool-Egress — ist
//! EINZELN gelockt, fail-closed, ledger-protokolliert. Text handelt nicht:
//! nur ActionCandidates unter Lock + Gate + Evidence werden ausgefuehrt.

use crate::gate::GateReport;

/// Ein einzelner, fail-closed Lock. Default: GESCHLOSSEN.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityLock {
    pub capability: String,
    open: bool,
    pub ledger_ref: Option<String>,
    pub opened_by: Option<String>,
}

impl CapabilityLock {
    /// Default-Konstruktor: geschlossen (fail-closed).
    pub fn closed(capability: &str) -> Self {
        Self {
            capability: capability.to_string(),
            open: false,
            ledger_ref: None,
            opened_by: None,
        }
    }

    /// Oeffnen ist eine MATERIELLE Aktion: verlangt Operator-Kennung und
    /// Ledger-Referenz (Protokollpflicht) — sonst bleibt der Lock zu.
    pub fn open(&mut self, operator: &str, ledger_ref: &str) {
        self.open = true;
        self.opened_by = Some(operator.to_string());
        self.ledger_ref = Some(ledger_ref.to_string());
    }

    pub fn is_open(&self) -> bool {
        self.open
    }

    /// Gate-Form: prueft den Lock fail-closed.
    pub fn gate(&self) -> GateReport {
        if self.open {
            GateReport::pass(
                "CapabilityGate",
                &format!(
                    "Lock {} offen (Operator: {}, Ledger: {})",
                    self.capability,
                    self.opened_by.as_deref().unwrap_or("?"),
                    self.ledger_ref.as_deref().unwrap_or("?")
                ),
            )
        } else {
            GateReport::hold(
                "CapabilityGate",
                &format!("capability_violation: Lock {} geschlossen", self.capability),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Fail-closed Default; Oeffnen ist protokollpflichtige Handlung.
    #[test]
    fn lock_default_closed_open_is_recorded() {
        let mut l = CapabilityLock::closed("model_egress");
        assert!(!l.is_open());
        assert!(!l.gate().is_pass());
        l.open("op-1", "ledger:42");
        assert!(l.is_open());
        assert!(l.gate().is_pass());
        assert_eq!(l.opened_by.as_deref(), Some("op-1"));
    }
}
