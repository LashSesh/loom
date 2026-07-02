//! G5-Ausgangs-Gate (01_MASTER_BUILD):
//! - Replay-Identitaet: gleicher RD + Inputs ⇒ gleiche Klasse (INV-10)
//! - Pause/Resume aendert die Klasse nicht
//! - harte Gates pausieren nie
//! - Reise-PhaseLadder p₀…p₅ als Red(SCALE-1) schliessbar

use cce_core::canonical::Canonicalize;
use cce_core::replay::{HitlDecision, RunDescriptor};
use cce_core::signature::sha256;
use cce_core::value::CanonValue;
use cce_materialize::adapter::DomainAdapter;
use cce_materialize::document::DocumentAdapter;
use cce_runner::closure_report::closure_report;
use cce_runner::exec::{execute_cascade, execute_hyperdag, execute_pipeline};
use cce_runner::journey::{journey_red, JOURNEY_PHASES};
use cce_runner::{DecisionProvider, Run, RunError, RunStatus};
use cce_spiral::bluered::{close_red, RedVerdict};

fn rd() -> RunDescriptor {
    let adapter = DocumentAdapter;
    let crystal = adapter.reference_cube();
    RunDescriptor::new(adapter.canonicalize(&crystal).0, "document", 42)
}

struct AcceptAll;
impl DecisionProvider for AcceptAll {
    fn decide(&self, gate: &str, _context: &str) -> HitlDecision {
        HitlDecision {
            gate: gate.to_string(),
            decision: "accept".to_string(),
            operator: "op-1".to_string(),
            rd_ref: None,
            evidence_ref: Some("ev:hitl".to_string()),
        }
    }
}

/// INV-10: gleicher RD + Inputs ⇒ gleiche Ergebnisklasse; HITL-Läufe sind
/// über die aufgezeichneten Entscheidungen exakt reproduzierbar (S5.4/5.5).
#[test]
fn replay_identity_including_hitl() {
    let adapter = DocumentAdapter;
    // Erstlauf mit Ermessens-Gate: Entscheidung wird aufgezeichnet.
    let mut first = Run::submit(adapter.reference_cube(), rd()).unwrap();
    first.discretionary_gates.push((
        "coverage-borderline".into(),
        "Grenzfall der Abdeckung".into(),
    ));
    first.run_to_end(Some(&AcceptAll)).unwrap();
    assert_eq!(first.status, RunStatus::Closed);
    assert_eq!(first.rd.decisions.len(), 1, "Entscheidung aufgezeichnet");

    // Replay: gleicher RD (inkl. decisions) — OHNE Provider. Die
    // Aufzeichnung wird abgespielt, nie neu gefragt.
    let mut replay = Run::submit(adapter.reference_cube(), first.rd.clone()).unwrap();
    replay.discretionary_gates.push((
        "coverage-borderline".into(),
        "Grenzfall der Abdeckung".into(),
    ));
    replay.run_to_end(None).unwrap();
    assert_eq!(replay.status, RunStatus::Closed);
    assert_eq!(
        first.result_class(),
        replay.result_class(),
        "gleicher RD ⇒ gleiche Klasse (INV-10)"
    );
    // ClosureReport bestaetigt ClosedCCE.
    assert!(closure_report(&replay).closed_cce);
}

/// Pause/Fortsetzen erhaelt die kanonische Inhaltsklasse (S5.3).
#[test]
fn pause_resume_preserves_class() {
    let adapter = DocumentAdapter;
    // Durchlauf ohne Pause:
    let mut straight = Run::submit(adapter.reference_cube(), rd()).unwrap();
    straight.run_to_end(None).unwrap();
    assert_eq!(straight.status, RunStatus::Closed);
    // Durchlauf mit Pause nach jeder Stufe:
    let mut paused = Run::submit(adapter.reference_cube(), rd()).unwrap();
    paused.status = RunStatus::Running;
    for i in 0..6 {
        paused.step(None).unwrap();
        paused.pause();
        if i < 5 {
            assert_eq!(paused.status, RunStatus::Paused);
        }
        paused.resume();
    }
    assert_eq!(paused.status, RunStatus::Closed);
    assert_eq!(straight.result_class(), paused.result_class());
    // Checkpoints sind content-adressiert und stimmen paarweise ueberein.
    let cp_a: Vec<_> = straight.checkpoints.iter().map(|c| c.state_class).collect();
    // Pausen erzeugen zusaetzliche (identische) Checkpoints an derselben
    // Lage — inhaltlich dieselbe Folge nach Dedup benachbarter Duplikate.
    let mut cp_b: Vec<_> = paused.checkpoints.iter().map(|c| c.state_class).collect();
    cp_b.dedup();
    assert_eq!(cp_a, cp_b, "gleiche Lage ⇒ gleicher Checkpoint-Digest");
}

/// Harte Gates pausieren NIE (S5.4): der Pausierversuch ist baulich
/// unmoeglich und liefert einen benannten Fehler.
#[test]
fn hard_gates_never_pause() {
    let adapter = DocumentAdapter;
    let run = Run::submit(adapter.reference_cube(), rd()).unwrap();
    for hard in ["G4-Residue", "G5-Replay", "G7-Reanalysis"] {
        match run.request_pause_at_hard_gate(hard) {
            Err(RunError::HardGateNotPausable(msg)) => {
                assert!(msg.contains(hard));
            }
            other => panic!("hartes Gate {hard} pausierbar: {other:?}"),
        }
    }
}

/// Die Reise ist als Red(SCALE-1) ueber p₀…p₅ schliessbar (S2-A1).
#[test]
fn journey_phase_ladder_closes_as_red_scale1() {
    let red = journey_red(sha256(b"reise-evidence"));
    assert_eq!(red.blues.len(), 6);
    assert_eq!(
        red.blues
            .iter()
            .map(|b| b.phase.as_str())
            .collect::<Vec<_>>(),
        JOURNEY_PHASES
    );
    match close_red(&red) {
        RedVerdict::Promote { to_scale } => assert_eq!(to_scale, 2),
        RedVerdict::Open(rs) => panic!("Reise nicht geschlossen: {rs:?}"),
    }
    // Fehl-Reise: eine Phase ohne Evidence ⇒ Red offen, sichtbar (S2-A2).
    let mut broken = journey_red(sha256(b"x"));
    broken.blues[2].evidence_refs.clear();
    assert!(matches!(close_red(&broken), RedVerdict::Open(_)));
}

/// Die drei Ausfuehrungsformen sind deterministisch (S5-A3).
#[test]
fn three_execution_forms_deterministic() {
    // (a) HyperDAG
    use cce_phaseblock::accept::{accept_block, AcceptContext};
    use cce_phaseblock::hyperdag::HyperDag;
    use cce_phaseblock::phaseblock::PhaseBlock;
    let build_dag = || {
        let mut h = HyperDag::new();
        for tag in ["a", "b"] {
            let gates: Vec<_> = cce_core::gate::mandatory_gates()
                .iter()
                .map(|g| cce_core::gate::GateReport::pass(&g.id, "ok"))
                .collect();
            let mut b = PhaseBlock::candidate(
                1,
                "p",
                &CanonValue::text(tag),
                vec![],
                gates,
                vec![sha256(tag.as_bytes())],
                cce_core::residue::ResidueField::new(),
                sha256(b"rd"),
                vec![],
            );
            accept_block(&mut b, &AcceptContext::all_true());
            h.insert_block(b);
        }
        h
    };
    assert_eq!(
        execute_hyperdag(&build_dag()).unwrap(),
        execute_hyperdag(&build_dag()).unwrap()
    );

    // (b) Kaskade
    use cce_core::gate::GateReport;
    use cce_spiral::ratchet::{Ratchet, RatchetKind, RatchetState};
    let mut levels = vec![
        {
            let mut r = Ratchet::new(RatchetKind::Cell, "cell");
            r.step(&GateReport::pass("g", "ok"), Some(sha256(b"e1")));
            r
        },
        {
            let mut r = Ratchet::new(RatchetKind::Phase, "phase");
            r.step(&GateReport::pass("g", "ok"), Some(sha256(b"e2")));
            r
        },
    ];
    execute_cascade(&mut levels).unwrap();
    assert!(levels.iter().all(|r| r.state == RatchetState::Lock));

    // (c) Pipeline
    fn upper(v: &CanonValue) -> CanonValue {
        match v {
            CanonValue::Text(t) => CanonValue::text(t.to_uppercase()),
            other => other.clone(),
        }
    }
    let stages: [cce_runner::exec::PipelineStage; 2] = [("s1", upper), ("s2", upper)];
    let (out1, reports, head1) = execute_pipeline(&CanonValue::text("abc"), &stages);
    let (out2, _, head2) = execute_pipeline(&CanonValue::text("abc"), &stages);
    assert_eq!(out1, out2);
    assert_eq!(head1, head2);
    assert_eq!(reports.len(), 2);
    assert_eq!(
        out1.canonical_class(),
        CanonValue::text("ABC").canonical_class()
    );
}
