//! Die Dogfood-Residuen (Dokument 19 §5/§7) — jedes sichtbar, keines
//! still. Namensraum wie `cce_swe::residues`, eigenes Praefix `dog:`.

use cce_core::residue::{Residue, ResidueKind, Severity};

pub const ALL_DOGFOOD_RESIDUES: [&str; 6] = [
    "protected_path_violation",
    "task_class_not_whitelisted",
    "task_scope_empty",
    "task_budget_missing",
    "branch_isolation_violation",
    "merge_to_main_attempted",
];

/// Erzeugt ein benanntes Dogfood-Residuum — alle Blocking (jede
/// Verletzung dieser Schutzzone ist eine harte Sicherheitsgrenze, keine
/// Nichtdeterminismus-Anzeige wie `model_replay_weak`).
pub fn dogfood_residue(kind: &str, detail: &str) -> Residue {
    debug_assert!(
        ALL_DOGFOOD_RESIDUES.contains(&kind),
        "unbekanntes Dogfood-Residuum"
    );
    Residue::new(
        &format!("dog:{kind}"),
        "cce-dogfood",
        ResidueKind::named(kind),
        Severity::Blocking,
        detail,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn six_residues_all_constructible() {
        for k in ALL_DOGFOOD_RESIDUES {
            let r = dogfood_residue(k, "test");
            assert!(r.id.contains(k));
        }
    }
}
