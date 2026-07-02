//! BoundedOperatorSpecialization (neutralisierter „Fallen Seraph",
//! Rebase §3): Operator-Klon `O' = (O, scope, budget, mutation, expiry,
//! ledgerRef)`; gueltig ⟺ Scope ⊆ Elternscope ∧ Budget endlich ∧ Ergebnisse
//! gegatet ∧ |descendants(O)| ≤ B_O (RD-Parameter).
//!
//! IM PRODUKT FAIL-CLOSED DEAKTIVIERT (R-13 / PROD-INV-11): der
//! Aktivierungspfad existiert NUR via CapabilityLock; jeder Versuch ohne
//! offenen Lock wird abgewiesen (`unbounded_cloning` / `capability_violation`).

use cce_core::capability::CapabilityLock;
use cce_core::residue::{Residue, ResidueKind, Severity};

#[derive(Debug, Clone)]
pub struct OperatorClone {
    pub parent: String,
    pub scope: String,
    pub budget: u32,
    pub mutation: String,
    pub expiry: u32,
    pub ledger_ref: String,
}

#[derive(Debug)]
pub struct SpecializationEngine {
    /// Der Klonungs-Lock: im Produktbau IMMER geschlossen (R-13).
    pub lock: CapabilityLock,
    /// B_O: maximale Nachkommenzahl (RD-Parameter).
    pub max_descendants: usize,
    descendants: Vec<OperatorClone>,
}

impl SpecializationEngine {
    /// Produktzustand: Lock geschlossen — Klonung deaktiviert.
    pub fn product_default(max_descendants: usize) -> Self {
        Self {
            lock: CapabilityLock::closed("operator_cloning"),
            max_descendants,
            descendants: Vec::new(),
        }
    }

    /// Klon-Versuch: NUR unter offenem Lock + allen Schranken.
    pub fn clone_operator(
        &mut self,
        parent: &str,
        parent_scope: &str,
        requested_scope: &str,
        budget: u32,
        expiry: u32,
    ) -> Result<&OperatorClone, Box<Residue>> {
        if !self.lock.is_open() {
            return Err(Box::new(Residue::new(
                "cloning-locked",
                "specialization",
                ResidueKind::named("unbounded_cloning"),
                Severity::Blocking,
                "Klonung fail-closed deaktiviert (R-13): CapabilityLock geschlossen",
            )));
        }
        if !requested_scope.starts_with(parent_scope) {
            return Err(Box::new(Residue::new(
                "clone-scope",
                "specialization",
                ResidueKind::named("clone_scope_escalation"),
                Severity::Blocking,
                &format!("Scope {requested_scope} ⊄ Elternscope {parent_scope}"),
            )));
        }
        if budget == 0 || expiry == 0 {
            return Err(Box::new(Residue::new(
                "clone-budget",
                "specialization",
                ResidueKind::named("unbounded_cloning"),
                Severity::Blocking,
                "Budget/Expiry muss endlich und > 0 sein",
            )));
        }
        if self.descendants.len() >= self.max_descendants {
            return Err(Box::new(Residue::new(
                "clone-count",
                "specialization",
                ResidueKind::named("unbounded_cloning"),
                Severity::Blocking,
                &format!("|descendants| ≥ B_O = {}", self.max_descendants),
            )));
        }
        self.descendants.push(OperatorClone {
            parent: parent.to_string(),
            scope: requested_scope.to_string(),
            budget,
            mutation: "none".to_string(),
            expiry,
            ledger_ref: self
                .lock
                .ledger_ref
                .clone()
                .unwrap_or_else(|| "?".to_string()),
        });
        Ok(self.descendants.last().expect("just pushed"))
    }

    pub fn descendants(&self) -> usize {
        self.descendants.len()
    }
}
