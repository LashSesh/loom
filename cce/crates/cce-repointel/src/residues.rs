//! Die RepoIntelligence-Residuen — jedes sichtbar, keines still.
//! Namensraum-Praefix `rig:`.

use cce_core::residue::{Residue, ResidueKind, Severity};

pub const ALL_REPOINTEL_RESIDUES: [&str; 5] = [
    // Die CSA-Policy (Lizenz/Zugriff/Robots/PII) hielt — kein Einzug,
    // keine Destillation, kein Bauplan (fail-closed VOR allem anderen).
    "repointel_policy_blocked",
    // Die CSA-Kette lieferte keine einzige gueltige CSU.
    "repointel_empty_ingest",
    // Die HBM-Kette zertifizierte keinen einzigen Blueprint.
    "repointel_no_certified_blueprint",
    // Eine architektonische Frage blieb im Bauplan offen (DecisionSlot
    // open/proposed) — sichtbar gefuehrt, blockiert die Siegelung nicht.
    "repointel_decision_left_open",
    // Der Bauplan-Digest weicht beim Replay ab.
    "repointel_replay_divergent",
];

/// `repointel_decision_left_open` ist Warning (sichtbare Wahrheit,
/// kein Baumangel); alles andere ist Blocking.
pub fn repointel_residue(kind: &str, detail: &str) -> Residue {
    debug_assert!(
        ALL_REPOINTEL_RESIDUES.contains(&kind),
        "unbekanntes RepoIntel-Residuum"
    );
    let severity = match kind {
        "repointel_decision_left_open" => Severity::Warning,
        _ => Severity::Blocking,
    };
    Residue::new(
        &format!("rig:{kind}"),
        "cce-repointel",
        ResidueKind::named(kind),
        severity,
        detail,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_repointel_residues_constructible() {
        for k in ALL_REPOINTEL_RESIDUES {
            let r = repointel_residue(k, "test");
            assert!(r.id.contains(k));
        }
    }

    #[test]
    fn open_decision_is_warning_rest_blocking() {
        assert_eq!(
            repointel_residue("repointel_decision_left_open", "x").severity,
            Severity::Warning
        );
        assert_eq!(
            repointel_residue("repointel_policy_blocked", "x").severity,
            Severity::Blocking
        );
    }
}
