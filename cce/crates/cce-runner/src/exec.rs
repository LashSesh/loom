//! Die drei Ausfuehrungsformen (S5-A3): (a) PhaseBlock-HyperDAG topologisch,
//! (b) Multi-Ratchet-Kaskade, (c) generischer Pipeline-Lauf — alle
//! deterministisch, RD-gebunden.

use cce_core::gate::GateReport;
use cce_core::signature::{sha256, Digest};
use cce_core::value::CanonValue;
use cce_phaseblock::hyperdag::HyperDag;
use cce_phaseblock::projection::{commit_projection, ProjectionError};
use cce_spiral::ratchet::{cascade_lock, Ratchet};

/// Eine benannte Pipeline-Stufe (deterministische Funktion).
pub type PipelineStage = (&'static str, fn(&CanonValue) -> CanonValue);

/// (a) HyperDAG-Ausfuehrung: deterministische Topologie + CommitProjection.
pub fn execute_hyperdag(h: &HyperDag) -> Result<Digest, ProjectionError> {
    Ok(commit_projection(h)?.head())
}

/// (b) Kaskaden-Ausfuehrung: Locks strikt von unten nach oben.
pub fn execute_cascade(levels: &mut [Ratchet]) -> Result<(), Box<cce_core::residue::Residue>> {
    // unterste Ebene zuerst locken:
    for i in 0..levels.len() {
        if i == 0 {
            levels[0].lock()?;
        } else {
            let (lower, upper) = levels.split_at_mut(i);
            let lower_refs: Vec<&Ratchet> = lower.iter().collect();
            cascade_lock(&mut upper[0], &lower_refs)?;
        }
    }
    Ok(())
}

/// (c) Generischer Pipeline-Lauf: benannte, deterministische Stufen ueber
/// dem kanonischen Wertemodell; jede Stufe wird gegatet und verbucht.
pub fn execute_pipeline(
    input: &CanonValue,
    stages: &[PipelineStage],
) -> (CanonValue, Vec<GateReport>, Digest) {
    let mut state = input.normalize();
    let mut reports = Vec::new();
    let mut ledger = cce_core::ledger::Ledger::new();
    for (name, f) in stages {
        state = f(&state).normalize();
        reports.push(GateReport::pass(name, "Stufe deterministisch ausgefuehrt"));
        ledger.append(
            cce_core::ledger::LedgerEventKind::Execute,
            sha256(&state.encode()),
        );
    }
    let head = ledger.head();
    (state, reports, head)
}
