//! Die 8 Spiral-Pflichtgates (S15.14): WrapGate, AreaClassGate, DriftGate,
//! PhaseRatchetGate, ScaleRatchetGate, DispersionProfileGate,
//! ApertureSectorGate, SpiralReplayGate — alle boolesch, fail-closed,
//! begruendet, kein Score.

use crate::area_class::area_class;
use crate::bluered::{close_red, RedCube, RedVerdict};
use crate::dispersion::{DispersionDeclaration, DispersionProfile};
use crate::expansion::SpiralState;
use crate::ratchet::{Ratchet, RatchetState};
use crate::wrap::{wrap, WrapPolicy, Wrapped};
use cce_core::gate::GateReport;
use cce_core::residue::ResidueField;
use std::collections::BTreeSet;

/// 1. WrapGate: die vier Wickel-Pflichten sind erfuellt.
pub fn wrap_gate(w: &Wrapped, policy: &WrapPolicy) -> GateReport {
    let inside = w.state.support.iter().all(|s| policy.boundary.contains(s));
    if !inside {
        return GateReport::hold("WrapGate", "wrap_support_violation: Supp(W_s(z)) ⊄ B_s");
    }
    if !w.seams_preserved {
        return GateReport::hold("WrapGate", "Seam-Preservation verletzt");
    }
    if !w.replay_preserved {
        return GateReport::hold("WrapGate", "Replay-Preservation verletzt");
    }
    GateReport::pass("WrapGate", "vier Wickel-Pflichten erfuellt")
}

/// 2. AreaClassGate: [E^k(X)]_{µs} = [X]_{µs} nach Wicklung.
pub fn area_class_gate(
    before: &SpiralState,
    after_wrap: &SpiralState,
    boundary: &BTreeSet<String>,
) -> GateReport {
    let a = area_class(before, boundary);
    let b = area_class(after_wrap, boundary);
    if a == b {
        GateReport::pass("AreaClassGate", "Flaechenklasse invariant unter E∘W")
    } else {
        GateReport::hold(
            "AreaClassGate",
            "area_class_mismatch: Klassen-Digest weicht ab",
        )
    }
}

/// 3. DriftGate: externe Drift d⊥ ist annulliert UND sichtbar gefuehrt.
pub fn drift_gate(w: &Wrapped) -> GateReport {
    let all_visible = w
        .annulled_drift
        .iter()
        .all(|r| r.kind.as_str() == "external_drift_detected");
    if all_visible {
        GateReport::pass(
            "DriftGate",
            &format!(
                "d⊥ annulliert ({} Eintraege sichtbar), d∥ frei",
                w.annulled_drift.len()
            ),
        )
    } else {
        GateReport::hold(
            "DriftGate",
            "external_drift_detected ohne sichtbare Fuehrung",
        )
    }
}

/// 4. PhaseRatchetGate: Phasen-Lock nur mit Commit + Evidence.
pub fn phase_ratchet_gate(r: &Ratchet) -> GateReport {
    if r.state == RatchetState::Lock && (r.commit_counter == 0 || r.evidence_refs.is_empty()) {
        GateReport::hold(
            "PhaseRatchetGate",
            "ratchet_lock_without_evidence: Lock ohne Commit/Evidence",
        )
    } else {
        GateReport::pass("PhaseRatchetGate", "Ratchet-Zustand konsistent")
    }
}

/// 5. ScaleRatchetGate: Promotion nur nach Close(Red) + WrapStability.
pub fn scale_ratchet_gate(red: &RedCube, wrap_stable: bool) -> GateReport {
    if !wrap_stable {
        return GateReport::hold("ScaleRatchetGate", "scale_promotion_without_wrap_stability");
    }
    match close_red(red) {
        RedVerdict::Promote { to_scale } => GateReport::pass(
            "ScaleRatchetGate",
            &format!("Promotion → SCALE-{to_scale} zulaessig"),
        ),
        RedVerdict::Open(rs) => GateReport::hold(
            "ScaleRatchetGate",
            &format!(
                "Red nicht geschlossen: {}",
                rs.iter()
                    .map(|r| r.kind.as_str().to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        ),
    }
}

/// 6. DispersionProfileGate: Profil statt Dogma — undeklariertes Profil
///    wird abgewiesen (fibonacci_dogma_import / golden_angle_unjustified).
pub fn dispersion_profile_gate(
    declaration: &DispersionDeclaration,
    used: DispersionProfile,
) -> GateReport {
    match declaration.declared {
        None => GateReport::hold(
            "DispersionProfileGate",
            &match used {
                DispersionProfile::Golden => {
                    "golden_angle_unjustified: Golden-Profil ohne RD-Deklaration".to_string()
                }
                _ => "phase_profile_undeclared: kein Profil im RD deklariert".to_string(),
            },
        ),
        Some(declared) if declared != used => GateReport::hold(
            "DispersionProfileGate",
            &format!(
                "fibonacci_dogma_import: benutztes Profil {} ≠ deklariertes {}",
                used.as_str(),
                declared.as_str()
            ),
        ),
        Some(_) => GateReport::pass(
            "DispersionProfileGate",
            &format!("Profil {} RD-deklariert", used.as_str()),
        ),
    }
}

/// 7. ApertureSectorGate: der aktive Sektor bleibt im Zellen-Scope
///    (Binnenpupille/FBC — kein Scope-Leak).
pub fn aperture_sector_gate(sector_scope: &str, touched_scopes: &[String]) -> GateReport {
    let leaks: Vec<&String> = touched_scopes
        .iter()
        .filter(|s| s.as_str() != sector_scope)
        .collect();
    if leaks.is_empty() {
        GateReport::pass("ApertureSectorGate", "Sektor scope-treu")
    } else {
        GateReport::hold(
            "ApertureSectorGate",
            &format!("aperture_sector_scope_leak: {leaks:?} ausserhalb {sector_scope}"),
        )
    }
}

/// 8. SpiralReplayGate: gleicher Startzustand + Profil ⇒ gleiche Adresse
///    und Klasse (deterministische Kinematik).
pub fn spiral_replay_gate(a: &SpiralState, b: &SpiralState) -> GateReport {
    if a.address == b.address && a.content_class() == b.content_class() {
        GateReport::pass(
            "SpiralReplayGate",
            "Kinematik reproduziert Klasse und Adresse",
        )
    } else {
        GateReport::hold(
            "SpiralReplayGate",
            "dispersion_not_replayable: Wiederholung weicht ab",
        )
    }
}

/// Alle 8 Gate-Namen (Vollstaendigkeitstest).
pub const ALL_SPIRAL_GATES: [&str; 8] = [
    "WrapGate",
    "AreaClassGate",
    "DriftGate",
    "PhaseRatchetGate",
    "ScaleRatchetGate",
    "DispersionProfileGate",
    "ApertureSectorGate",
    "SpiralReplayGate",
];

/// Bequemer Voll-Durchlauf fuer Blue-Schliessung: expand → wrap → gates.
pub fn wrap_stability(
    z: &SpiralState,
    policy: &WrapPolicy,
    residues: &ResidueField,
) -> (Wrapped, GateReport) {
    let w = wrap(z, policy, residues);
    let g = wrap_gate(&w, policy);
    (w, g)
}
