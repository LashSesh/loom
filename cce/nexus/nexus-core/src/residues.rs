//! Die 18 CSA-Residuen (CSA.8) — jedes sichtbar, jedes im ResidueReport,
//! jedes mit Negativ-Zeuge (CSA.12).

use cce_core::residue::{Residue, ResidueKind, Severity};

pub const ALL_CSA_RESIDUES: [&str; 18] = [
    "source_unknown",
    "manifest_missing",
    "terms_unknown",
    "robots_blocked",
    "access_blocked",
    "license_incompatible",
    "license_attribution_required",
    "privacy_risk",
    "rate_budget_exceeded",
    "schema_unparseable",
    "provenance_gap",
    "evidence_missing",
    "quality_axis_missing",
    "score_as_gate_attempt",
    "replay_drift",
    "hbm_projection_missing",
    "phc_projection_missing",
    "export_blocked",
];

/// Erzeugt ein benanntes CSA-Residuum. `access_blocked`/`robots_blocked`
/// sind ENDZUSTAENDE (keine Herausforderungen, CSA.14).
pub fn csa_residue(kind: &str, detail: &str) -> Residue {
    debug_assert!(ALL_CSA_RESIDUES.contains(&kind), "unbekanntes CSA-Residuum");
    let severity = match kind {
        "license_attribution_required" => Severity::Warning,
        _ => Severity::Blocking,
    };
    Residue::new(
        &format!("csa:{kind}"),
        "csa",
        ResidueKind::named(kind),
        severity,
        detail,
    )
}
