//! Die Benchmark-Residuen (Dokument 20 §4/§6) — jedes sichtbar, keines
//! still. Namensraum-Praefix `bench:`.

use cce_core::residue::{Residue, ResidueKind, Severity};

pub const ALL_BENCHMARK_RESIDUES: [&str; 4] = [
    "benchmark_fairness_digest_mismatch",
    "benchmark_fairness_order_violation",
    "benchmark_arms_incomplete",
    "benchmark_matrix_incomplete",
];

/// Alle Blocking — jede Fairness-/Vollstaendigkeitsverletzung ist eine
/// harte Grenze (die Matrix darf NICHT gebaut werden, §4).
pub fn benchmark_residue(kind: &str, detail: &str) -> Residue {
    debug_assert!(
        ALL_BENCHMARK_RESIDUES.contains(&kind),
        "unbekanntes Benchmark-Residuum"
    );
    Residue::new(
        &format!("bench:{kind}"),
        "cce-benchmark",
        ResidueKind::named(kind),
        Severity::Blocking,
        detail,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn four_residues_all_constructible() {
        for k in ALL_BENCHMARK_RESIDUES {
            let r = benchmark_residue(k, "test");
            assert!(r.id.contains(k));
        }
    }
}
