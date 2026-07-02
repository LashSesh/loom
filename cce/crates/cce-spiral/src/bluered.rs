//! Blue/Red-Skalengeometrie (Formel 2, L5): *BlueCube schliesst Phase,
//! RedCube schliesst Skala, geschlossene Skala wird Zelle der naechsten
//! Skala.* `Close(Blue) ⟺ Typed ∧ BoundaryValid ∧ Gate=Pass ∧ Evidence ∧
//! Replay ∧ WrapStable` ⇒ PhaseBlock. `Close(Red) ⟺ ∀p Close(Blue) ∧
//! SeamsValid ∧ Replay ∧ Resid sichtbar` ⇒ Promote(s→s+1).

use cce_core::gate::GateChain;
use cce_core::residue::{Residue, ResidueField};
use cce_core::signature::Digest;

/// BlueCube Blue_{s,p} = (Σ, Ω, B, K, Cand, G, R, L) — lokaler Phasenraum.
#[derive(Debug, Clone)]
pub struct BlueCube {
    pub scale: u8,
    pub phase: String,
    pub typed: bool,
    pub boundary_valid: bool,
    pub gates: GateChain,
    pub evidence_refs: Vec<Digest>,
    pub replay_ok: bool,
    pub wrap_stable: bool,
    pub residues: ResidueField,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CloseVerdict {
    Closed,
    Open(Vec<String>),
}

/// Close(Blue): jede Bedingung einzeln, fail-closed.
pub fn close_blue(b: &BlueCube) -> CloseVerdict {
    let mut missing = Vec::new();
    if !b.typed {
        missing.push("Typed".to_string());
    }
    if !b.boundary_valid {
        missing.push("BoundaryValid".to_string());
    }
    if !b.gates.all_pass() {
        missing.push("Gate=Pass".to_string());
    }
    if b.evidence_refs.is_empty() {
        missing.push("Evidence".to_string());
    }
    if !b.replay_ok {
        missing.push("Replay".to_string());
    }
    if !b.wrap_stable {
        missing.push("WrapStable".to_string());
    }
    if b.residues.has_blocking() {
        missing.push("blocking Residuum offen".to_string());
    }
    if missing.is_empty() {
        CloseVerdict::Closed
    } else {
        CloseVerdict::Open(missing)
    }
}

/// RedCube Red_s = ({Blue_{s,p}}, E_phase, E_seam, E_gate, Close_s) —
/// Phasenmatrix einer Skala.
#[derive(Debug, Clone)]
pub struct RedCube {
    pub scale: u8,
    pub blues: Vec<BlueCube>,
    pub seams_valid: bool,
    pub replay_ok: bool,
}

#[derive(Debug)]
pub enum RedVerdict {
    /// Promotion s → s+1 (via ScaleRatchet).
    Promote {
        to_scale: u8,
    },
    Open(Vec<Residue>),
}

/// Close(Red): ∀p Close(Blue) ∧ SeamsValid ∧ Replay ∧ Residuen sichtbar.
pub fn close_red(r: &RedCube) -> RedVerdict {
    let mut residues = Vec::new();
    for b in &r.blues {
        if let CloseVerdict::Open(missing) = close_blue(b) {
            residues.push(crate::residues::bluecube_not_closed(&format!(
                "Blue_{{{},{}}}: {}",
                b.scale,
                b.phase,
                missing.join(", ")
            )));
        }
    }
    if !r.seams_valid {
        residues.push(crate::residues::redcube_seam_gap(&format!(
            "Red_{}: Phasen-Naehte inkonsistent",
            r.scale
        )));
    }
    if !r.replay_ok {
        residues.push(crate::residues::dispersion_not_replayable(&format!(
            "Red_{}: Replay nicht nachgewiesen",
            r.scale
        )));
    }
    if residues.is_empty() {
        RedVerdict::Promote {
            to_scale: r.scale + 1,
        }
    } else {
        RedVerdict::Open(residues)
    }
}
