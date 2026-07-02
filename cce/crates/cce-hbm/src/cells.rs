//! EphemeralMiningCells (neutralisierte „Heavenly Hosts", Rebase §3):
//! `Cell = (scope, budget, projection, allowedOps, gate, ttl, trace)` —
//! bearbeitet NUR den projizierten Teilraum; Dissolution: nur Trace/
//! Evidence/Kristalle persistieren (HBM-Axiom 5.4, HBM-12).
//! Fehlersprache: `cell_scope_leak`, `persistent_ephemeral`.

use cce_core::residue::{Residue, ResidueKind, Severity};
use cce_core::signature::Digest;

#[derive(Debug)]
pub struct EphemeralMiningCell {
    pub scope: String,
    pub budget: u32,
    pub projection: String,
    pub allowed_ops: Vec<String>,
    pub gate: String,
    pub ttl: u32,
    pub trace: Vec<String>,
    spent: u32,
}

#[derive(Debug)]
pub struct CellHarvest {
    pub trace: Vec<String>,
    pub evidence_refs: Vec<Digest>,
    pub crystal_classes: Vec<Digest>,
}

impl EphemeralMiningCell {
    pub fn spawn(scope: &str, budget: u32, ttl: u32) -> Self {
        Self {
            scope: scope.to_string(),
            budget,
            projection: format!("proj:{scope}"),
            allowed_ops: vec!["extract".into(), "project".into(), "gate".into()],
            gate: "Gate_A".to_string(),
            ttl,
            trace: Vec::new(),
            spent: 0,
        }
    }

    /// Arbeit NUR im projizierten Teilraum; Scope-Verlassen = Residuum.
    pub fn work(&mut self, target_scope: &str, note: &str) -> Result<(), Box<Residue>> {
        if target_scope != self.scope {
            return Err(Box::new(Residue::new(
                "cell-scope-leak",
                "mining-cell",
                ResidueKind::named("cell_scope_leak"),
                Severity::Blocking,
                &format!("Zelle {} beruehrte {target_scope}", self.scope),
            )));
        }
        if self.spent >= self.budget || self.ttl == 0 {
            return Err(Box::new(Residue::new(
                "cell-budget",
                "mining-cell",
                ResidueKind::named("expansion_budget_exceeded"),
                Severity::Blocking,
                "Budget/TTL erschoepft",
            )));
        }
        self.spent += 1;
        self.ttl -= 1;
        self.trace.push(note.to_string());
        Ok(())
    }

    /// Dissolution: die Zelle wird KONSUMIERT; nur das
    /// Persistierungswuerdige bleibt (HBM 5.4). Eine Zelle, die sich der
    /// Aufloesung entzieht, waere `persistent_ephemeral` — der Typ macht
    /// das unmoeglich (self by value).
    pub fn dissolve(self, evidence_refs: Vec<Digest>, crystal_classes: Vec<Digest>) -> CellHarvest {
        CellHarvest {
            trace: self.trace,
            evidence_refs,
            crystal_classes,
        }
    }
}
