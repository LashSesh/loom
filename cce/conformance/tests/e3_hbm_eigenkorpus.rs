//! Etappe X2/E3 (Ökosystem-Expansionskarte §2/E3) — HBM produktiv auf
//! Eigenkorpus. Exit-Zeugen: HBM-End-to-End auf Eigenkorpus,
//! Blueprint-Kristall zertifiziert, Replay klassenidentisch,
//! Klonungs-Lock-Negativzeuge (R-13) weiterhin rot.

use cce_hbm::pipeline::run_pipeline;
use cce_hbm::specialization::SpecializationEngine;
use loom_conformance::{build_blueprint_eigenkorpus, build_eigenkorpus_mining_input};

fn seed_dir() -> std::path::PathBuf {
    std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../library/seed"))
}

#[test]
fn eigenkorpus_end_to_end_certifies_at_least_one_blueprint() {
    let input = build_eigenkorpus_mining_input();
    let out = run_pipeline(&input).expect("Gate_A darf am Eigenkorpus nie halten");
    assert!(
        !out.certified.is_empty(),
        "der Eigenkorpus (213 Familien) muss mindestens einen Blueprint-Kristall zertifizieren"
    );
    // HBM-18-Disziplin: jede zertifizierte Klasse hat einen Commit im
    // Ledger, der Ledger ist selbst verifizierbar.
    let commits = out
        .ledger
        .events()
        .iter()
        .filter(|e| e.kind == cce_core::ledger::LedgerEventKind::Commit)
        .count();
    assert_eq!(commits, out.certified.len());
    assert!(cce_core::ledger::verify_ledger(&out.ledger).is_ok());
}

/// Die "Vollprojektion" (C6) traegt ALLE Facetten des Eigenkorpus —
/// genau das Struktur-Muster "wiederkehrende Naht-Regeln ueber
/// Familien", das die Karte als Beispielkandidat nennt. Kein
/// Teilausschnitt, keine Behauptung ohne Datenabdeckung.
#[test]
fn full_projection_candidate_covers_the_entire_eigenkorpus() {
    let input = build_eigenkorpus_mining_input();
    let expected_facet_count = input.lines.len();
    let out = run_pipeline(&input).unwrap();
    let c6 = out
        .candidates
        .iter()
        .find(|c| c.generator == "C6")
        .expect("C6-Vollprojektion muss existieren");
    assert_eq!(
        c6.facets.len(),
        expected_facet_count,
        "C6 muss ALLE Facetten des Eigenkorpus tragen"
    );
    assert_eq!(
        c6.status,
        cce_hbm::candidate::CandidateStatus::Pass,
        "C6 muss zertifiziert sein: {:?}",
        c6.hold_diagnosis
    );
}

/// HBM-20-Disziplin auf dem realen Eigenkorpus: zwei Laeufe desselben
/// MiningInput zertifizieren dieselben Klassen und enden im selben
/// Ledger-Head — Replay klassenidentisch.
#[test]
fn eigenkorpus_replay_is_class_identical() {
    let input = build_eigenkorpus_mining_input();
    let out1 = run_pipeline(&input).unwrap();
    let out2 = run_pipeline(&input).unwrap();
    assert_eq!(out1.certified, out2.certified);
    assert_eq!(out1.ledger.head(), out2.ledger.head());
}

/// R-13: der Klonungs-Lock bleibt geschlossen — dieser Ring aendert
/// cce_hbm::specialization NICHT an, der Negativzeuge bleibt rot.
#[test]
fn r13_cloning_lock_stays_closed_negative_witness_remains_red() {
    let mut engine = SpecializationEngine::product_default(8);
    let err = engine
        .clone_operator("parent", "scope", "scope.child", 10, 100)
        .expect_err("Klonung muss ohne offenen Lock fail-closed abgewiesen werden");
    assert_eq!(err.kind.as_str(), "unbounded_cloning");
    assert!(err.content.contains("R-13"));
    assert_eq!(engine.descendants(), 0);
}

#[test]
fn blueprint_eigenkorpus_seed_is_valid_and_matches_builder() {
    let built = build_blueprint_eigenkorpus();
    let on_disk = std::fs::read(seed_dir().join("blueprint_eigenkorpus.loom"))
        .expect("Seed-Datei muss vorhanden sein");
    assert_eq!(
        built.bytes, on_disk,
        "Generator und Seed-Datei muessen byte-identisch sein"
    );
    let report = loom_verify::verify(&built.bytes);
    assert_eq!(report.verdict, loom_verify::Verdict::Valid);
}
