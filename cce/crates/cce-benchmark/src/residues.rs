//! Die Benchmark-Residuen (Dokument 20 §4/§6) — jedes sichtbar, keines
//! still. Namensraum-Praefix `bench:`.

use cce_core::residue::{Residue, ResidueKind, Severity};

pub const ALL_BENCHMARK_RESIDUES: [&str; 6] = [
    "benchmark_fairness_digest_mismatch",
    "benchmark_fairness_order_violation",
    "benchmark_arms_incomplete",
    "benchmark_matrix_incomplete",
    // Dokument 22 (P4-Ext): der dritte Arm.
    "benchmark_packet_digest_mismatch",
    "task_class_unsupported_by_tool",
];

/// Die Fairness-/Vollstaendigkeitsverletzungen sind Blocking (die Matrix
/// darf NICHT gebaut werden, §4). Einzige Ausnahme (Dok 22 §4):
/// `task_class_unsupported_by_tool` ist Warning — ein Fremdwerkzeug, das
/// eine Aufgabenklasse schlicht nicht unterstuetzt, wird als sichtbares
/// Residuum GEFUEHRT, nicht erzwungen (R-BENCH-EXT-2).
pub fn benchmark_residue(kind: &str, detail: &str) -> Residue {
    debug_assert!(
        ALL_BENCHMARK_RESIDUES.contains(&kind),
        "unbekanntes Benchmark-Residuum"
    );
    let severity = match kind {
        "task_class_unsupported_by_tool" => Severity::Warning,
        _ => Severity::Blocking,
    };
    Residue::new(
        &format!("bench:{kind}"),
        "cce-benchmark",
        ResidueKind::named(kind),
        severity,
        detail,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_benchmark_residues_constructible() {
        for k in ALL_BENCHMARK_RESIDUES {
            let r = benchmark_residue(k, "test");
            assert!(r.id.contains(k));
        }
    }

    #[test]
    fn unsupported_task_class_is_warning_others_blocking() {
        assert_eq!(
            benchmark_residue("task_class_unsupported_by_tool", "x").severity,
            Severity::Warning
        );
        assert_eq!(
            benchmark_residue("benchmark_packet_digest_mismatch", "x").severity,
            Severity::Blocking
        );
    }
}
