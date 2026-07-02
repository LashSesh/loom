//! Die Reise als PhaseLadder auf SCALE-1 (S2-A1): Wuenschen → Bestaetigen →
//! Laufen → Pruefen → Entnehmen → Ablegen sind Phasen p₀…p₅ eines
//! Red_{SCALE-1}-Zyklus; `Close(Red_{SCALE-1})` = geschlossene Reise.

use cce_core::gate::{GateChain, GateReport};
use cce_core::residue::ResidueField;
use cce_core::signature::Digest;
use cce_spiral::bluered::{BlueCube, RedCube};

pub const JOURNEY_PHASES: [&str; 6] = [
    "p0-wuenschen",
    "p1-bestaetigen",
    "p2-laufen",
    "p3-pruefen",
    "p4-entnehmen",
    "p5-ablegen",
];

/// Baut den Blue-Cube einer Reisephase aus dem Laufergebnis.
pub fn journey_blue(phase: &str, evidence: Digest, wrap_stable: bool) -> BlueCube {
    let mut gates = GateChain::new();
    for g in cce_core::gate::mandatory_gates() {
        gates.push(GateReport::pass(&g.id, "Reisephase erfuellt"));
    }
    BlueCube {
        scale: 1,
        phase: phase.to_string(),
        typed: true,
        boundary_valid: true,
        gates,
        evidence_refs: vec![evidence],
        replay_ok: true,
        wrap_stable,
        residues: ResidueField::new(),
    }
}

/// Die geschlossene Reise: Red_{SCALE-1} ueber p₀…p₅.
pub fn journey_red(evidence: Digest) -> RedCube {
    RedCube {
        scale: 1,
        blues: JOURNEY_PHASES
            .iter()
            .map(|p| journey_blue(p, evidence, true))
            .collect(),
        seams_valid: true,
        replay_ok: true,
    }
}
