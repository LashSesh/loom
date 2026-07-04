//! Die zwei neuen P4-Gates (Dokument 20 §4): FairnessGate +
//! ComparisonSealGate. Dieselbe `InfVerdict`-Form wie alle P1/P2/P3-Gates
//! (aus `cce_swe::gates` wiederverwendet, nicht dupliziert).

use crate::model::{CceRunResult, ComparisonMatrix, RawRunResult};
use crate::residues::benchmark_residue;
pub use cce_swe::gates::InfVerdict;

/// FairnessGate (§2, strukturell): beide Arme MUESSEN denselben
/// `task_package_digest` tragen, und der ungegatete Arm MUSS VOR dem
/// CCE-Arm abgegeben haben (`submission_order` streng kleiner). Geprueft
/// ueber Digest-Gleichheit + Reihenfolge — keine Seite sieht die Antwort
/// der anderen vor der eigenen Abgabe.
pub fn fairness_gate(raw: &RawRunResult, cce: &CceRunResult) -> InfVerdict {
    if raw.task_package_digest != cce.task_package_digest {
        return InfVerdict::Reject {
            gate: "FairnessGate".into(),
            residue: Box::new(benchmark_residue(
                "benchmark_fairness_digest_mismatch",
                &format!(
                    "Raw-Arm task_package_digest {} != CCE-Arm {}",
                    raw.task_package_digest, cce.task_package_digest
                ),
            )),
        };
    }
    if raw.submission_order >= cce.submission_order {
        return InfVerdict::Reject {
            gate: "FairnessGate".into(),
            residue: Box::new(benchmark_residue(
                "benchmark_fairness_order_violation",
                &format!(
                    "Raw-Arm submission_order {} nicht strikt vor CCE-Arm {} — der ungegatete \
                     Arm muss zuerst und isoliert abgeben",
                    raw.submission_order, cce.submission_order
                ),
            )),
        };
    }
    InfVerdict::Allow {
        gate: "FairnessGate".into(),
        reason: "identischer task_package_digest, Raw-Arm strikt vor CCE-Arm".into(),
    }
}

/// ComparisonSealGate (§4): die Matrix darf NUR gebaut/versiegelt werden,
/// wenn beide Arme abgeschlossen sind UND `task_package_digest` auf
/// beiden Seiten identisch ist.
pub fn comparison_seal_gate(raw: Option<&RawRunResult>, cce: Option<&CceRunResult>) -> InfVerdict {
    let (raw, cce) = match (raw, cce) {
        (Some(r), Some(c)) => (r, c),
        _ => {
            return InfVerdict::Reject {
                gate: "ComparisonSealGate".into(),
                residue: Box::new(benchmark_residue(
                    "benchmark_arms_incomplete",
                    "Matrixbau vor Abschluss beider Arme — nicht zulaessig",
                )),
            };
        }
    };
    if raw.task_package_digest != cce.task_package_digest {
        return InfVerdict::Reject {
            gate: "ComparisonSealGate".into(),
            residue: Box::new(benchmark_residue(
                "benchmark_fairness_digest_mismatch",
                "task_package_digest der Arme weicht ab — keine Matrix",
            )),
        };
    }
    InfVerdict::Allow {
        gate: "ComparisonSealGate".into(),
        reason: "beide Arme abgeschlossen, identischer task_package_digest".into(),
    }
}

/// Matrix-Vollstaendigkeit (§5, fuer den Container-Verify gespiegelt):
/// genau sechs D-Zeilen.
pub fn matrix_complete_gate(matrix: &ComparisonMatrix) -> InfVerdict {
    if matrix.is_complete() {
        InfVerdict::Allow {
            gate: "MatrixCompleteGate".into(),
            reason: "D1–D6 vollstaendig".into(),
        }
    } else {
        InfVerdict::Reject {
            gate: "MatrixCompleteGate".into(),
            residue: Box::new(benchmark_residue(
                "benchmark_matrix_incomplete",
                &format!("{} Matrix-Zeilen statt 6", matrix.rows.len()),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::TaskClass;

    fn raw(digest: &str, order: u64) -> RawRunResult {
        RawRunResult::observed("p", digest, "o-raw", 100, Some(true), Some(true), 2, order)
    }
    fn cce(digest: &str, order: u64) -> CceRunResult {
        CceRunResult::certified(
            "p",
            digest,
            "o-cce",
            120,
            Some(true),
            Some(true),
            0,
            order,
            "root",
            12,
            true,
        )
    }

    #[test]
    fn fairness_gate_allows_correct_order_and_matching_digest() {
        assert!(fairness_gate(&raw("d", 1), &cce("d", 2)).allows());
    }

    #[test]
    fn fairness_gate_rejects_digest_mismatch() {
        let v = fairness_gate(&raw("d-raw", 1), &cce("d-cce", 2));
        assert!(!v.allows());
        assert!(v
            .residue()
            .unwrap()
            .id
            .contains("benchmark_fairness_digest_mismatch"));
    }

    #[test]
    fn fairness_gate_rejects_cce_before_raw() {
        let v = fairness_gate(&raw("d", 5), &cce("d", 2));
        assert!(!v.allows());
        assert!(v
            .residue()
            .unwrap()
            .id
            .contains("benchmark_fairness_order_violation"));
    }

    #[test]
    fn comparison_seal_gate_rejects_incomplete_arms() {
        let v = comparison_seal_gate(Some(&raw("d", 1)), None);
        assert!(!v.allows());
        assert!(v
            .residue()
            .unwrap()
            .id
            .contains("benchmark_arms_incomplete"));
    }

    #[test]
    fn comparison_seal_gate_allows_both_done_matching() {
        assert!(comparison_seal_gate(Some(&raw("d", 1)), Some(&cce("d", 2))).allows());
    }

    #[test]
    fn matrix_complete_gate_rejects_short_matrix() {
        let m = ComparisonMatrix {
            package_id: "p".to_string(),
            task_class: TaskClass::Coding,
            rows: vec![],
        };
        assert!(!matrix_complete_gate(&m).allows());
    }
}
