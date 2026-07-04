//! Die SWE-Residuen (Dokument 18 §4/§8) — jedes sichtbar, keines still.
//! Namensraum wie `cce_inference::residues`, eigenes Praefix `swe:`.

use cce_core::residue::{Residue, ResidueKind, Severity};

pub const ALL_SWE_RESIDUES: [&str; 13] = [
    "tool_capability_denied",
    "tool_scope_violation",
    "build_unverified",
    "tests_unverified",
    "tests_red",
    "regression_detected",
    "diff_apply_conflict",
    "model_replay_weak",
    // Agent-Grounding (Dokument 21 §5), additiv:
    "rule_missing_evidence",
    "decision_left_open",
    "context_budget_exceeded",
    "delta_budget_exceeded",
    "rule_violation",
];

/// Erzeugt ein benanntes SWE-Residuum. `model_replay_weak` und
/// `rule_missing_evidence` (sichtbare automatische Herabstufung, kein
/// Fehlschlag) sind Warning; alles andere ist ein Blocking-Halt.
pub fn swe_residue(kind: &str, detail: &str) -> Residue {
    debug_assert!(ALL_SWE_RESIDUES.contains(&kind), "unbekanntes SWE-Residuum");
    let severity = match kind {
        "model_replay_weak" | "rule_missing_evidence" => Severity::Warning,
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
    fn all_residues_constructible() {
        for k in ALL_SWE_RESIDUES {
            let r = swe_residue(k, "test");
            assert!(r.id.contains(k));
        }
    }
}
