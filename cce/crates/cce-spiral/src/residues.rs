//! Die 12 Spiral-Residuen (S15.14, Spiralprozessschicht) — jedes erzeugbar,
//! jedes sichtbar, jedes mit Negativ-Zeuge (S8-A1).

use cce_core::residue::{Residue, ResidueKind, Severity};

fn mk(kind: &str, severity: Severity, detail: &str) -> Residue {
    Residue::new(
        &format!("spiral:{kind}"),
        "cce-spiral",
        ResidueKind::named(kind),
        severity,
        detail,
    )
}

pub fn wrap_support_violation(detail: &str) -> Residue {
    mk("wrap_support_violation", Severity::Blocking, detail)
}

pub fn area_class_mismatch(detail: &str) -> Residue {
    mk("area_class_mismatch", Severity::Blocking, detail)
}

pub fn external_drift_detected(detail: &str) -> Residue {
    mk("external_drift_detected", Severity::Blocking, detail)
}

pub fn phase_profile_undeclared(detail: &str) -> Residue {
    mk("phase_profile_undeclared", Severity::Blocking, detail)
}

pub fn dispersion_not_replayable(detail: &str) -> Residue {
    mk("dispersion_not_replayable", Severity::Blocking, detail)
}

pub fn ratchet_lock_without_evidence(detail: &str) -> Residue {
    mk("ratchet_lock_without_evidence", Severity::Blocking, detail)
}

pub fn bluecube_not_closed(detail: &str) -> Residue {
    mk("bluecube_not_closed", Severity::Blocking, detail)
}

pub fn redcube_seam_gap(detail: &str) -> Residue {
    mk("redcube_seam_gap", Severity::Blocking, detail)
}

pub fn scale_promotion_without_wrap_stability(detail: &str) -> Residue {
    mk(
        "scale_promotion_without_wrap_stability",
        Severity::Blocking,
        detail,
    )
}

pub fn aperture_sector_scope_leak(detail: &str) -> Residue {
    mk("aperture_sector_scope_leak", Severity::Blocking, detail)
}

pub fn fibonacci_dogma_import(detail: &str) -> Residue {
    mk("fibonacci_dogma_import", Severity::Blocking, detail)
}

pub fn golden_angle_unjustified(detail: &str) -> Residue {
    mk("golden_angle_unjustified", Severity::Blocking, detail)
}

/// Alle 12 Arten (fuer Vollstaendigkeits-/Waechter-Tests).
pub const ALL_SPIRAL_RESIDUES: [&str; 12] = [
    "wrap_support_violation",
    "area_class_mismatch",
    "external_drift_detected",
    "phase_profile_undeclared",
    "dispersion_not_replayable",
    "ratchet_lock_without_evidence",
    "bluecube_not_closed",
    "redcube_seam_gap",
    "scale_promotion_without_wrap_stability",
    "aperture_sector_scope_leak",
    "fibonacci_dogma_import",
    "golden_angle_unjustified",
];
