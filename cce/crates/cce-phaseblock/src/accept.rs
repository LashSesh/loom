//! Accept-8-Kriterien (Rebase §1.1 / 01_MASTER_BUILD G2):
//! `Accept ⟺ Typed ∧ BoundaryValid ∧ SeamConsistent ∧ Gate=Pass
//!         ∧ EvidenceComplete ∧ ResidueVisible ∧ Replayable
//!         ∧ ReanalysisCompatible`.
//! JEDE fehlende Bedingung ⇒ Hold, NIE Commit (fail-closed, Driftverbot 6).

use crate::phaseblock::{BlockStatus, PhaseBlock};

/// Externe Nachweise, die der Block selbst nicht tragen kann.
#[derive(Debug, Clone, Copy)]
pub struct AcceptContext {
    pub typed: bool,
    pub boundary_valid: bool,
    pub seam_consistent: bool,
    pub replayable: bool,
    pub reanalysis_compatible: bool,
}

impl AcceptContext {
    pub fn all_true() -> Self {
        Self {
            typed: true,
            boundary_valid: true,
            seam_consistent: true,
            replayable: true,
            reanalysis_compatible: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AcceptOutcome {
    Accepted,
    /// Hold mit benannten fehlenden Kriterien (nie stiller Nicht-Commit).
    Hold(Vec<String>),
}

/// Prueft die acht Kriterien EINZELN; akzeptiert nur bei 8/8.
pub fn accept_block(block: &mut PhaseBlock, ctx: &AcceptContext) -> AcceptOutcome {
    let mut missing = Vec::new();
    if !ctx.typed {
        missing.push("Typed".to_string());
    }
    if !ctx.boundary_valid {
        missing.push("BoundaryValid".to_string());
    }
    if !ctx.seam_consistent {
        missing.push("SeamConsistent".to_string());
    }
    let gates_pass =
        !block.gate_reports.is_empty() && block.gate_reports.iter().all(|g| g.is_pass());
    if !gates_pass {
        missing.push("Gate=Pass (Kette leer oder Hold)".to_string());
    }
    if block.evidence_refs.is_empty() {
        missing.push("EvidenceComplete (keine Evidence — Kandidaten-Commit verboten)".to_string());
    }
    // ResidueVisible: das Feld existiert immer (Typ), blockierende Eintraege
    // verhindern Accept.
    if block.residue_field.has_blocking() {
        missing.push("ResidueVisible: blocking Residuum offen".to_string());
    }
    if !ctx.replayable {
        missing.push("Replayable".to_string());
    }
    if !ctx.reanalysis_compatible {
        missing.push("ReanalysisCompatible".to_string());
    }
    if missing.is_empty() {
        block.status = BlockStatus::Accepted;
        AcceptOutcome::Accepted
    } else {
        block.status = BlockStatus::Hold;
        AcceptOutcome::Hold(missing)
    }
}
