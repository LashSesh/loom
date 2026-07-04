//! Der Zusammenbau (Dokument 20 §4): FairnessGate → ComparisonSealGate
//! → Matrixbau → Versiegelung. Reine Komposition bestehender Bausteine,
//! keine neue Kern-Logik. Die tatsaechlichen Arm-Laeufe (echte
//! Modell-/Build-/Test-Aufrufe) leben im `#[ignore]`-Betriebs-Harness
//! (§7), nicht hier — diese Funktion nimmt die fertigen Arm-Ergebnisse
//! entgegen und stellt sie fair gegenueber.

use crate::gates::{comparison_seal_gate, fairness_gate, matrix_complete_gate, InfVerdict};
use crate::matrix::build_matrix;
use crate::model::{BenchmarkTaskPackage, CceRunResult, ComparisonMatrix, RawRunResult};
use crate::workbody::{seal_benchmark_workbody, WorkbodyError};
use loom_codec::Sealed;

/// Fehler beim Zusammenbau — ein Gate-Halt (mit Verdikt) oder ein
/// Pack-Fehler.
#[derive(Debug)]
pub enum AssembleError {
    /// FairnessGate/ComparisonSealGate/MatrixCompleteGate hielt.
    Gate(Box<InfVerdict>),
    Pack(WorkbodyError),
}

/// Ergebnis eines erfolgreichen Zusammenbaus: die Matrix + der
/// versiegelte `benchmark`-Workbody.
pub struct Assembled {
    pub matrix: ComparisonMatrix,
    pub sealed: Sealed,
}

/// Stellt beide Arm-Ergebnisse fair gegenueber und versiegelt den
/// Benchmark-Workbody. FairnessGate + ComparisonSealGate laufen VOR
/// jedem Matrixbau (§2/§4).
pub fn assemble_benchmark(
    package: &BenchmarkTaskPackage,
    raw: &RawRunResult,
    cce: &CceRunResult,
) -> Result<Assembled, AssembleError> {
    // 1. FairnessGate: identischer Digest, Raw-Arm strikt zuerst.
    let fairness = fairness_gate(raw, cce);
    if !fairness.allows() {
        return Err(AssembleError::Gate(Box::new(fairness)));
    }
    // 2. ComparisonSealGate: beide Arme abgeschlossen, Digest identisch.
    let seal = comparison_seal_gate(Some(raw), Some(cce));
    if !seal.allows() {
        return Err(AssembleError::Gate(Box::new(seal)));
    }
    // 3. Erst JETZT die Matrix bauen.
    let matrix = build_matrix(package.task_class, raw, cce);
    let complete = matrix_complete_gate(&matrix);
    if !complete.allows() {
        return Err(AssembleError::Gate(Box::new(complete)));
    }
    // 4. Versiegeln.
    let sealed =
        seal_benchmark_workbody(package, raw, cce, &matrix).map_err(AssembleError::Pack)?;
    Ok(Assembled { matrix, sealed })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::TaskClass;

    fn package() -> BenchmarkTaskPackage {
        BenchmarkTaskPackage {
            package_id: "bench-assemble".to_string(),
            task_class: TaskClass::Coding,
            task_text: "fix".to_string(),
            starting_files: vec![("src/lib.rs".to_string(), b"x".to_vec())],
            success_criteria: "test gruen".to_string(),
            build_command: vec!["cargo".to_string(), "build".to_string()],
            test_command: vec!["cargo".to_string(), "test".to_string()],
            target_path: "src/lib.rs".to_string(),
        }
    }

    fn arms(
        pkg: &BenchmarkTaskPackage,
        raw_order: u64,
        cce_order: u64,
    ) -> (RawRunResult, CceRunResult) {
        let d = pkg.digest_hex();
        (
            RawRunResult::observed(
                &pkg.package_id,
                &d,
                "o-raw",
                100,
                Some(true),
                Some(true),
                4,
                raw_order,
            ),
            CceRunResult::certified(
                &pkg.package_id,
                &d,
                "o-cce",
                150,
                Some(true),
                Some(true),
                0,
                cce_order,
                "abcd",
                12,
                true,
            ),
        )
    }

    #[test]
    fn assemble_succeeds_for_fair_complete_arms() {
        let pkg = package();
        let (raw, cce) = arms(&pkg, 1, 2);
        let a = assemble_benchmark(&pkg, &raw, &cce).expect("Zusammenbau gelingt");
        assert!(a.matrix.is_complete());
        assert!(!a.sealed.bytes.is_empty());
    }

    #[test]
    fn assemble_rejects_fairness_order_violation_no_seal() {
        let pkg = package();
        let (raw, cce) = arms(&pkg, 5, 2); // CCE vor Raw
        match assemble_benchmark(&pkg, &raw, &cce) {
            Err(AssembleError::Gate(v)) => {
                assert_eq!(v.gate(), "FairnessGate");
            }
            Err(other) => panic!("erwartet FairnessGate-Halt, war {other:?}"),
            Ok(_) => panic!("erwartet FairnessGate-Halt, war Ok"),
        }
    }
}
