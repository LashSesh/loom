//! `Ledger = CommitProjection(H)` (Rebase §1.1, S9-A1): der Ledger IST die
//! deterministische Linearisierung der akzeptierten Bloecke des HyperDAG —
//! kein Blockchain-Import. `verify_hdag_projection` erkennt
//! `ledger_hdag_mismatch` (blocking).

use crate::hyperdag::{DagError, HyperDag};
use cce_core::ledger::{Ledger, LedgerEventKind};
use cce_core::residue::{Residue, ResidueKind, Severity};

#[derive(Debug, Clone)]
pub enum ProjectionError {
    Dag(DagError),
    /// ledger_hdag_mismatch — blocking (S9-A1). Geboxt (Err-Groesse).
    Mismatch(Box<Residue>),
}

/// Projiziert die akzeptierten Bloecke deterministisch in einen Ledger.
pub fn commit_projection(h: &HyperDag) -> Result<Ledger, ProjectionError> {
    let mut ledger = Ledger::new();
    for b in h.accepted_in_order().map_err(ProjectionError::Dag)? {
        ledger.append(LedgerEventKind::Commit, b.payload_digest);
    }
    Ok(ledger)
}

/// Prueft, dass ein vorliegender Ledger EXAKT die Commit-Projektion von H ist.
pub fn verify_hdag_projection(h: &HyperDag, ledger: &Ledger) -> Result<(), ProjectionError> {
    ledger.verify().map_err(|e| {
        ProjectionError::Mismatch(Box::new(mismatch_residue(&format!(
            "Ledger-Kette gebrochen: {e:?}"
        ))))
    })?;
    let expected = commit_projection(h)?;
    let exp_events = expected.events();
    let got_events = ledger.events();
    if exp_events.len() != got_events.len() {
        return Err(ProjectionError::Mismatch(Box::new(mismatch_residue(
            &format!(
                "Laenge {} ≠ Projektion {}",
                got_events.len(),
                exp_events.len()
            ),
        ))));
    }
    for (e, g) in exp_events.iter().zip(got_events) {
        if e.payload_digest != g.payload_digest || g.kind != LedgerEventKind::Commit {
            return Err(ProjectionError::Mismatch(Box::new(mismatch_residue(
                &format!("Eintrag {} weicht von CommitProjection(H) ab", g.seq),
            ))));
        }
    }
    Ok(())
}

fn mismatch_residue(detail: &str) -> Residue {
    Residue::new(
        "ledger-hdag-mismatch",
        "verify_hdag_projection",
        ResidueKind::named("ledger_hdag_mismatch"),
        Severity::Blocking,
        detail,
    )
}
