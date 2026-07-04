//! Die SWE-Residuen (Dokument 18 §4/§8) — jedes sichtbar, keines still.
//! Namensraum wie `cce_inference::residues`, eigenes Praefix `swe:`.

use cce_core::residue::{Residue, ResidueKind, Severity};

pub const ALL_SWE_RESIDUES: [&str; 8] = [
    "tool_capability_denied",
    "tool_scope_violation",
    "build_unverified",
    "tests_unverified",
    "tests_red",
    "regression_detected",
    "diff_apply_conflict",
    "model_replay_weak",
];

/// Erzeugt ein benanntes SWE-Residuum. `model_replay_weak` ist die
/// sichtbare Nichtdeterminismus-Anzeige (wie in cce-inference) —
/// Warning; alles andere ist ein Blocking-Halt vor der Kern-Kette.
pub fn swe_residue(kind: &str, detail: &str) -> Residue {
    debug_assert!(ALL_SWE_RESIDUES.contains(&kind), "unbekanntes SWE-Residuum");
    let severity = match kind {
        "model_replay_weak" => Severity::Warning,
        _ => Severity::Blocking,
    };
    Residue::new(
        &format!("swe:{kind}"),
        "cce-swe",
        ResidueKind::named(kind),
        severity,
        detail,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eight_residues_all_constructible() {
        for k in ALL_SWE_RESIDUES {
            let r = swe_residue(k, "test");
            assert!(r.id.contains(k));
        }
    }
}
