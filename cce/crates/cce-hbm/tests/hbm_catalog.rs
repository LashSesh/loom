//! Abnahmekatalog HBM-01…HBM-20 (REBASE_KONSOLIDIERUNG §3, S15.8).
//! Verankerte Nummern auf ihren Spec-Positionen: HBM-10 (PhaseSwitch
//! geloggt), HBM-12 (EphemeralMiningCells/Dissolution), HBM-16
//! (Safety-by-abstraction/Risk-Scopes), HBM-17 (Hold-Diagnose maschinen-
//! lesbar), HBM-18 (Registry: Provenienz/Version/Score-als-Metadatum),
//! HBM-20 (End-to-End + Replay klassenidentisch). Uebrige Nummern
//! materialisieren die Axiome 5.1–5.7, die Pipeline 0–9 und die
//! Operatoralgebra (R-Agent-5-Disziplin).

use cce_core::capability::CapabilityLock;
use cce_core::gate::GateReportError;
use cce_core::residue::Severity;
use cce_core::signature::sha256;
use cce_hbm::calibration::{Calibration, MiningPhase, PhaseSwitchController};
use cce_hbm::candidate::{exclusion_gate, generate_candidates, CandidateStatus};
use cce_hbm::cells::EphemeralMiningCell;
use cce_hbm::facet::{extract_facets, gate_a, Facet, FACET_TYPES};
use cce_hbm::pipeline::{materialize_candidate, run_pipeline, MiningInput};
use cce_hbm::score::{attempt_score_as_gate, rank, ScoreWeights};
use cce_hbm::specialization::SpecializationEngine;

fn corpus() -> Vec<String> {
    vec![
        "entity: Dokumentkern".to_string(),
        "operator: materialize".to_string(),
        "constraint: covers(risiken)".to_string(),
        "gate: DocG-Support".to_string(),
        "risk: Serverausfall".to_string(),
        "invariant: residuum sichtbar".to_string(),
    ]
}

fn input() -> MiningInput {
    MiningInput {
        corpus_id: "korpus-1".to_string(),
        lines: corpus(),
        weights: ScoreWeights::default(),
        theta_d: 1,
        expansion_budget: 16,
    }
}

/// HBM-01 — Typisierte Projektion (Axiom 5.1): nur getypte Facets entstehen.
#[test]
fn hbm01_typed_projection() {
    let facets = extract_facets("k", &["entity: a", "unfug: b", "gate: c"]);
    assert_eq!(facets.len(), 2, "untypisierte Zeile erzeugt KEINE Facet");
    assert!(facets
        .iter()
        .all(|f| FACET_TYPES.contains(&f.facet_type.as_str())));
}

/// HBM-02 — Explizite Kopplung (Axiom 5.2): unbelegte Facet = Residuum.
#[test]
fn hbm02_unsourced_facet_held() {
    let mut f = extract_facets("k", &["entity: a"]);
    f[0].evidence = None;
    let r = gate_a(&f);
    assert!(!r.is_pass());
    assert!(r.reason.contains("facet_unsourced"));
}

/// HBM-03 — Fail-closed Mining (Axiom 5.3): Gate_A-Hold stoppt die Pipeline.
#[test]
fn hbm03_gate_a_fail_closed() {
    // Widerspruechliche Facet-Typen (gleiche Kennung, anderer Typ):
    let mut facets = extract_facets("k", &["entity: x"]);
    facets.push(Facet {
        id: "f0".to_string(),
        facet_type: "gate".to_string(),
        scope: "x".to_string(),
        source: "k".to_string(),
        evidence: Some("k#L9".to_string()),
        confidence_permille: 500,
    });
    assert!(!gate_a(&facets).is_pass());
}

/// HBM-04 — Adapter-Vereinheitlichung (S1-A1): Gate_A ist der EINE
/// Adapterbegriff; ungetypter Adapter wird abgewiesen.
#[test]
fn hbm04_adapter_untyped_rejected() {
    let f = vec![Facet {
        id: "fx".to_string(),
        facet_type: "geist".to_string(),
        scope: "s".to_string(),
        source: "k".to_string(),
        evidence: Some("e".to_string()),
        confidence_permille: 100,
    }];
    let r = gate_a(&f);
    assert!(!r.is_pass());
    assert!(r.reason.contains("adapter_untyped"));
}

/// HBM-05 — Baumweitenbewusstsein (Axiom 5.5): die Pipeline berechnet die
/// Baumweite des Facet-Skeletts.
#[test]
fn hbm05_treewidth_awareness() {
    let out = run_pipeline(&input()).unwrap();
    assert!(out.treewidth.is_some());
}

/// HBM-06 — Kandidaten-Generatoren C1–C6 vollstaendig und deterministisch.
#[test]
fn hbm06_generators_c1_to_c6() {
    let facets = extract_facets(
        "k",
        &corpus().iter().map(String::as_str).collect::<Vec<_>>(),
    );
    let cands = generate_candidates(&facets);
    for gen in ["C1", "C2", "C3", "C4", "C5", "C6"] {
        assert!(
            cands.iter().any(|c| c.generator == gen),
            "Generator {gen} fehlt"
        );
    }
    let again = generate_candidates(&facets);
    assert_eq!(cands.len(), again.len());
}

/// HBM-07 — Score ordnet, Gate entscheidet (A7): Ranking veraendert
/// keinen Status.
#[test]
fn hbm07_score_ranks_gate_decides() {
    let facets = extract_facets("k", &["entity: a", "gate: b"]);
    let cands = generate_candidates(&facets);
    let ranking = rank(&cands, &ScoreWeights::default(), 0);
    assert!(!ranking.is_empty());
    assert!(cce_hbm::score::ranking_leaves_status_untouched(&cands));
}

/// HBM-08 — θ_D ist Vorauswahl, NIE Abnahme: unter der Schwelle heisst
/// "nicht weitergerechnet", nie "abgelehnt-als-Urteil".
#[test]
fn hbm08_theta_d_is_preselection() {
    let mut inp = input();
    inp.theta_d = u64::MAX; // nichts wird weitergerechnet
    let out = run_pipeline(&inp).unwrap();
    assert!(out.certified.is_empty());
    assert!(out
        .holds
        .iter()
        .all(|(_, d)| d.contains("KEINE Abnahmeentscheidung")));
    assert!(out
        .candidates
        .iter()
        .all(|c| c.status == CandidateStatus::Hold));
}

/// HBM-09 — ExclusionGate: EXC = Reality ∧ Constraint ∧ Topo ∧ Evidence,
/// Fehlersprache exclusion_fail(...).
#[test]
fn hbm09_exclusion_gate_language() {
    let facets = extract_facets("k", &["entity: a"]);
    let mut cands = generate_candidates(&facets);
    cands[0].facets[0].evidence = None;
    let r = exclusion_gate(&cands[0]);
    assert!(!r.is_pass());
    assert!(r.reason.contains("exclusion_fail(evidence)"));
}

/// HBM-10 — FixpointCalibration: Phasenwechsel NUR protokolliert (geloggt).
#[test]
fn hbm10_phase_switch_logged() {
    let mut ctrl = PhaseSwitchController::new();
    let c = Calibration {
        psi: 500,
        rho: 500,
        omega: 500,
    };
    assert_eq!(ctrl.observe(c), MiningPhase::Explorative);
    assert_eq!(ctrl.observe(c), MiningPhase::Explorative);
    // dritte Stagnation ⇒ Wechsel MIT Log:
    assert_eq!(ctrl.observe(c), MiningPhase::Contractive);
    assert_eq!(ctrl.switch_log.len(), 1);
    assert!(ctrl.switch_log[0].contains("RD-geloggt"));
    // ungeloggter Wechsel ist als blocking Residuum typisiert:
    assert_eq!(
        PhaseSwitchController::unlogged_switch_residue().severity,
        Severity::Blocking
    );
}

/// HBM-11 — Kalibrierte Exploration (Axiom 5.6): keine ungeseedete
/// Zufaelligkeit — die gesamte Pipeline ist seedfrei-deterministisch.
#[test]
fn hbm11_deterministic_no_unseeded_randomness() {
    let a = run_pipeline(&input()).unwrap();
    let b = run_pipeline(&input()).unwrap();
    assert_eq!(a.ranking, b.ranking);
    assert_eq!(a.certified, b.certified);
    assert_eq!(a.ledger.head(), b.ledger.head());
}

/// HBM-12 — EphemeralMiningCells: Scope-Bindung + Dissolution
/// (nur Trace/Evidence/Kristalle persistieren).
#[test]
fn hbm12_ephemeral_cells() {
    let mut cell = EphemeralMiningCell::spawn("scope:doc", 4, 4);
    cell.work("scope:doc", "extrahiere facet").unwrap();
    // Scope-Leak wird abgewiesen:
    let leak = cell.work("scope:fremd", "boese").unwrap_err();
    assert_eq!(leak.kind.as_str(), "cell_scope_leak");
    // Dissolution konsumiert die Zelle:
    let harvest = cell.dissolve(vec![sha256(b"ev")], vec![sha256(b"k")]);
    assert_eq!(harvest.trace, vec!["extrahiere facet"]);
    assert_eq!(harvest.evidence_refs.len(), 1);
}

/// HBM-13 — Klonung fail-closed deaktiviert (R-13): Aktivierungsversuch
/// ohne CapabilityLock wird abgewiesen (unbounded_cloning).
#[test]
fn hbm13_cloning_deactivated_without_lock() {
    let mut engine = SpecializationEngine::product_default(2);
    let err = engine
        .clone_operator("op", "scope:a", "scope:a/sub", 5, 5)
        .unwrap_err();
    assert_eq!(err.kind.as_str(), "unbounded_cloning");
    assert!(err.content.contains("R-13"));
    assert_eq!(engine.descendants(), 0);
}

/// HBM-14 — Klon-Schranken unter offenem Lock: Scope ⊆ Eltern, Budget
/// endlich, |descendants| ≤ B_O.
#[test]
fn hbm14_clone_bounds_under_open_lock() {
    let mut engine = SpecializationEngine::product_default(1);
    engine.lock = {
        let mut l = CapabilityLock::closed("operator_cloning");
        l.open("test-operator", "ledger:test");
        l
    };
    // Scope-Eskalation abgewiesen:
    let e = engine
        .clone_operator("op", "scope:a", "scope:b", 5, 5)
        .unwrap_err();
    assert_eq!(e.kind.as_str(), "clone_scope_escalation");
    // gueltiger Klon:
    engine
        .clone_operator("op", "scope:a", "scope:a/x", 5, 5)
        .unwrap();
    // B_O erreicht:
    let e = engine
        .clone_operator("op", "scope:a", "scope:a/y", 5, 5)
        .unwrap_err();
    assert_eq!(e.kind.as_str(), "unbounded_cloning");
}

/// HBM-15 — Gate-Dominanz: Materialize ohne Gate=Pass UNMOEGLICH.
#[test]
fn hbm15_gate_dominance_materialize() {
    let facets = extract_facets("k", &["entity: a"]);
    let cands = generate_candidates(&facets);
    // Kandidat ist Hold (frisch generiert):
    let err = materialize_candidate(&cands[0]).unwrap_err();
    assert!(err.reason.contains("Gate-Dominanz"));
    // Nach vollem Pipeline-Lauf ist ein Pass-Kandidat materialisierbar:
    let out = run_pipeline(&input()).unwrap();
    let pass = out
        .candidates
        .iter()
        .find(|c| c.status == CandidateStatus::Pass)
        .expect("mindestens ein zertifizierter Kandidat");
    assert!(materialize_candidate(pass).is_ok());
}

/// HBM-16 — Safety-by-abstraction: ausgeschlossene Scope-Klassen
/// (Tarnung/Umgehung/Offensive/Selbstvermehrung) sind im Vokabular
/// NICHT vorhanden (Abwesenheits-Nachweis).
#[test]
fn hbm16_safety_by_abstraction() {
    for forbidden in ["tarnung", "umgehung", "offensiv", "selbstvermehrung"] {
        assert!(
            !FACET_TYPES.contains(&forbidden),
            "verbotene Scope-Klasse {forbidden} im Vokabular"
        );
    }
    // Klonung traegt Budget/TTL/Ledger-Strukturmuster, sonst nichts:
    let engine = SpecializationEngine::product_default(3);
    assert!(!engine.lock.is_open());
}

/// HBM-17 — Hold-Diagnose maschinenlesbar: jeder Hold traegt seinen Grund.
#[test]
fn hbm17_hold_diagnosis_machine_readable() {
    let mut inp = input();
    inp.theta_d = u64::MAX;
    let out = run_pipeline(&inp).unwrap();
    for c in &out.candidates {
        if c.status == CandidateStatus::Hold {
            assert!(c.hold_diagnosis.is_some(), "{} ohne Diagnose", c.id);
        }
    }
}

/// HBM-18 — Registry: zertifizierte Kristalle sind content-adressiert im
/// Ledger; Score bleibt Metadatum (im Ranking, nie am Kristall-Urteil).
#[test]
fn hbm18_registry_provenance() {
    let out = run_pipeline(&input()).unwrap();
    assert!(!out.certified.is_empty());
    // Jeder zertifizierte Eintrag hat einen Commit im Ledger:
    let commits = out
        .ledger
        .events()
        .iter()
        .filter(|e| e.kind == cce_core::ledger::LedgerEventKind::Commit)
        .count();
    assert_eq!(commits, out.certified.len());
    assert!(cce_core::ledger::verify_ledger(&out.ledger).is_ok());
}

/// HBM-19 — Negativ: score_as_gate_attempt wird abgewiesen (V1/A7).
#[test]
fn hbm19_score_as_gate_attempt_red() {
    let facets = extract_facets("k", &["entity: a"]);
    let cands = generate_candidates(&facets);
    let err = attempt_score_as_gate(&cands[0], 9999);
    assert!(matches!(err, GateReportError::ScoreAsGateAttempt(_)));
}

/// HBM-20 — End-to-End: kleiner Korpus ⇒ zertifizierter Blueprint-Kristall;
/// Replay klassenidentisch.
#[test]
fn hbm20_end_to_end_replay_class_identical() {
    let out1 = run_pipeline(&input()).unwrap();
    assert!(
        !out1.certified.is_empty(),
        "kein zertifizierter Blueprint-Kristall"
    );
    let out2 = run_pipeline(&input()).unwrap();
    assert_eq!(
        out1.certified, out2.certified,
        "Replay NICHT klassenidentisch (HBM-20 verletzt)"
    );
    assert_eq!(out1.ledger.head(), out2.ledger.head());
}
