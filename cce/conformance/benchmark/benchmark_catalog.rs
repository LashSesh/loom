//! Benchmark-Zeugenkatalog (Dokument 20, P4) — Ablage
//! `conformance/benchmark/`, dauerhaft im einen Waechter. Die
//! Objekte/Gates/Container-Klasse sind hermetisch (§7) und laufen gruen
//! in der normalen CI: R-BENCH-STRUCT (positiver Container-Verify) +
//! N-BENCH-1/2 (Fairness-/Seal-Verstoesse). Die ZWEI realen
//! Vergleichslaeufe (R-BENCH-1 Coding, R-BENCH-2 Dokument) leben separat
//! als `#[ignore]`-Betriebs-Harness (Details:
//! `reports/P4_vergleichslaeufe_bericht.md`).

use cce_benchmark::assemble::{assemble_benchmark, AssembleError};
use cce_benchmark::gates::{comparison_seal_gate, fairness_gate};
use cce_benchmark::model::{BenchmarkTaskPackage, CceRunResult, RawRunResult, TaskClass};

// Ein 68-Hex-Zeichen langer Platzhalter fuer den core_root (34-Byte-
// Multihash) des CCE-Arm-RepoWorkbody. Der Container-Verify ist
// hermetisch (nur Bytes dieses Containers, keine Cross-Container-
// Aufloesung) — die realen Laeufe tragen hier den echten core_root.
const REPO_REF: &str = "12200000000000000000000000000000000000000000000000000000000000000042";

fn coding_package() -> BenchmarkTaskPackage {
    BenchmarkTaskPackage {
        package_id: "bench-coding-witness".to_string(),
        task_class: TaskClass::Coding,
        task_text: "behebe den Vorzeichenfehler in add()".to_string(),
        starting_files: vec![
            (
                "src/lib.rs".to_string(),
                b"pub fn add(a:i32,b:i32)->i32{a-b}".to_vec(),
            ),
            (
                "tests/it.rs".to_string(),
                b"assert_eq!(add(2,2),4);".to_vec(),
            ),
        ],
        success_criteria: "cargo test gruen".to_string(),
        build_command: vec!["cargo".to_string(), "build".to_string()],
        test_command: vec!["cargo".to_string(), "test".to_string()],
        target_path: "src/lib.rs".to_string(),
    }
}

fn raw_arm(digest: &str, order: u64) -> RawRunResult {
    RawRunResult::observed(
        "bench-coding-witness",
        digest,
        "output-digest-raw",
        1200,
        Some(true),
        Some(true),
        3,
        order,
    )
}

fn cce_arm(digest: &str, order: u64) -> CceRunResult {
    CceRunResult::certified(
        "bench-coding-witness",
        digest,
        "output-digest-cce",
        1500,
        Some(true),
        Some(true),
        0,
        order,
        REPO_REF,
        12,
        true,
    )
}

/// R-BENCH-STRUCT: ein fair zusammengebauter Benchmark-Workbody der
/// Klasse "benchmark" ist `verify == Valid`, und die D1–D6-Matrix ist
/// vollstaendig (Raw-Arm strukturell nicht vorhanden in D1/D2/D3/D6,
/// beide Arme criteria_met in D4).
#[test]
fn r_bench_struct_sealed_benchmark_workbody_is_valid() {
    let pkg = coding_package();
    let d = pkg.digest_hex();
    let raw = raw_arm(&d, 1);
    let cce = cce_arm(&d, 2);

    let assembled = assemble_benchmark(&pkg, &raw, &cce).expect("Zusammenbau gelingt");
    assert!(assembled.matrix.is_complete());
    // D1/D2/D3/D6: Raw strukturell nicht vorhanden.
    for i in [0usize, 1, 2, 5] {
        assert_eq!(assembled.matrix.rows[i].raw.verdict, "structurally_absent");
        assert_eq!(assembled.matrix.rows[i].cce.verdict, "present");
    }
    // D4: beide Arme criteria_met (Paritaet).
    assert_eq!(assembled.matrix.rows[3].raw.verdict, "criteria_met");
    assert_eq!(assembled.matrix.rows[3].cce.verdict, "criteria_met");

    let verification = loom_verify::verify(&assembled.sealed.bytes);
    assert_eq!(
        verification.verdict,
        loom_verify::Verdict::Valid,
        "{:?}",
        verification.diagnoses
    );
}

/// N-BENCH-1: Fairness-Verstoss — der CCE-Arm startet mit abweichendem
/// `task_package_digest` ⇒ FairnessGate reject, keine Matrix gebaut.
#[test]
fn n_bench_1_fairness_digest_mismatch_rejected_no_matrix() {
    let pkg = coding_package();
    let raw = raw_arm(&pkg.digest_hex(), 1);
    // CCE-Arm mit ABWEICHENDEM Digest.
    let cce = cce_arm("ein-voellig-anderer-digest", 2);

    // Direkt am Gate:
    let v = fairness_gate(&raw, &cce);
    assert!(!v.allows());
    assert!(v
        .residue()
        .unwrap()
        .id
        .contains("benchmark_fairness_digest_mismatch"));

    // Und am vollen Zusammenbau: keine Matrix, keine Versiegelung.
    match assemble_benchmark(&pkg, &raw, &cce) {
        Err(AssembleError::Gate(g)) => assert_eq!(g.gate(), "FairnessGate"),
        Err(other) => panic!("erwartet FairnessGate-Halt, war {other:?}"),
        Ok(_) => panic!("Fairness-Verstoss darf niemals versiegeln"),
    }
}

/// N-BENCH-2: Matrixbau vor Abschluss beider Arme ⇒ ComparisonSealGate
/// reject.
#[test]
fn n_bench_2_matrix_before_both_arms_done_rejected() {
    let pkg = coding_package();
    let raw = raw_arm(&pkg.digest_hex(), 1);

    // Nur der Raw-Arm liegt vor, der CCE-Arm fehlt.
    let v = comparison_seal_gate(Some(&raw), None);
    assert!(!v.allows());
    assert!(v
        .residue()
        .unwrap()
        .id
        .contains("benchmark_arms_incomplete"));

    // Auch: gar kein Arm.
    let v_none = comparison_seal_gate(None, None);
    assert!(!v_none.allows());
}

/// Zusatz: die D4/D5-Paritaet ist ehrlich abgebildet — wenn der Raw-Arm
/// die success_criteria NICHT besteht, steht das auch so in der Matrix
/// (kein geschoentes Ergebnis).
#[test]
fn matrix_reflects_raw_arm_failure_honestly() {
    let pkg = coding_package();
    let d = pkg.digest_hex();
    // Raw-Arm: Test rot.
    let raw = RawRunResult::observed(
        "bench-coding-witness",
        &d,
        "output-digest-raw",
        1200,
        Some(true),
        Some(false),
        5,
        1,
    );
    let cce = cce_arm(&d, 2);
    let assembled = assemble_benchmark(&pkg, &raw, &cce).expect("Zusammenbau gelingt");
    // D4: Raw criteria_failed, CCE criteria_met.
    assert_eq!(assembled.matrix.rows[3].raw.verdict, "criteria_failed");
    assert_eq!(assembled.matrix.rows[3].cce.verdict, "criteria_met");
    // Trotz Raw-Fehlschlag ist der Vergleichs-Container selbst gueltig
    // (er dokumentiert die Wahrheit, er bewertet sie nicht).
    let verification = loom_verify::verify(&assembled.sealed.bytes);
    assert_eq!(
        verification.verdict,
        loom_verify::Verdict::Valid,
        "{:?}",
        verification.diagnoses
    );
}
