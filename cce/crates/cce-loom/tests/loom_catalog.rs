//! LOOM-Abnahmekatalog — 10 Tests (Bauverfassung Teil 7.5 / Phase F;
//! Themenfelder aus Teil 6.1: Nullanker/Singularitaet/Dyaden/Collect/
//! Distribute/Nadel/Workcell/Replay/Reanalyse). Nummernzuordnung: R-Agent-5.

use cce_core::canonical::Canonicalize;
use cce_core::gate::GateReport;
use cce_core::value::CanonValue;
use cce_core::wheel_window::WheelWindow;
use cce_loom::distribute::{loom_generate, LoomOutcome, WorkcellRuntime};
use cce_loom::radial_spindle::RadialSpindle;
use cce_loom::weave::{weave, Weave};
use cce_loom::workcell::{LoomWorkcell, WorkcellStatus};
use cce_phc::projection_calc::LocalProjection;

fn projection() -> LocalProjection {
    LocalProjection {
        cell_id: "cell:test".into(),
        payload: CanonValue::map([
            ("ordering", CanonValue::text("meaningful")),
            (
                "units",
                CanonValue::List(vec![
                    CanonValue::map([
                        ("id", CanonValue::text("u1")),
                        ("type", CanonValue::text("step")),
                        ("text", CanonValue::text("erster Schritt")),
                    ]),
                    CanonValue::map([
                        ("id", CanonValue::text("u2")),
                        ("type", CanonValue::text("step")),
                        ("text", CanonValue::text("zweiter Schritt")),
                    ]),
                ]),
            ),
        ]),
        allowed_ops: vec!["render".into()],
        gate_chain: vec!["G1-Scope".into()],
        export_formats: vec![".md".into()],
        domain_mode: "document".into(),
    }
}

struct EchoRuntime;
impl WorkcellRuntime for EchoRuntime {
    fn execute(&self, cell: &LoomWorkcell) -> CanonValue {
        cell.projection_payload.clone()
    }
}

/// L1 — Nullanker: markiert, nie durchlaufen; Naeherung erzeugt Trace (P7/V3).
#[test]
fn l01_null_anchor_marked_never_traversed() {
    let mut s = RadialSpindle::new("t");
    // Abwesenheits-Nachweis als const-Vertrag (kein Laufzeitpfad existiert):
    const _: () = assert!(RadialSpindle::BOUNDARY_TRACE_REQUIRED);
    assert_eq!(RadialSpindle::NULLPOINT_TRAVERSAL, "forbidden");
    let t = s.approach_zero("nullnaehe");
    assert!(t.anchor_id.starts_with("Z0"));
    assert_eq!(s.boundary_traces.len(), 1);
}

/// L2 — Singularitaets-Regularisierung: Mandorla-Naht koppelt Collect und
/// Distribute mit residue_policy=visible.
#[test]
fn l02_mandorla_regularizes() {
    let s = RadialSpindle::new("t");
    assert_eq!(s.mandorla_seam.cells, vec!["collect", "distribute"]);
    assert_eq!(cce_core::objects::Seam::RESIDUE_POLICY, "visible");
    assert_eq!(s.orientation, "4pi");
}

/// L3 — Dyaden/Strickung: das Gewebe traegt eine teilgeordnete Blockfolge
/// mit deterministischen Positionen.
#[test]
fn l03_weave_partial_order() {
    let w = weave(&projection()).unwrap();
    assert_eq!(w.blocks.len(), 2);
    assert!(w.blocks[0].position < w.blocks[1].position);
}

/// L4 — Collect-Anschluss: das Gewebe ist kanonisierbar (Klassentraeger),
/// Grundlage der Reanalyse.
#[test]
fn l04_weave_is_canonicalizable() {
    let w = weave(&projection()).unwrap();
    assert_eq!(
        w.canonical_class(),
        weave(&projection()).unwrap().canonical_class()
    );
}

/// L5 — Distribute: loom_generate committed alle Zellen bei Gate-Pass.
#[test]
fn l05_distribute_commits_under_gates() {
    let mut cells = vec![
        LoomWorkcell::new("w1", "c1", CanonValue::Int(1), "t", &["render"]),
        LoomWorkcell::new("w2", "c2", CanonValue::Int(2), "t", &["render"]),
    ];
    let out = loom_generate(&mut cells, &EchoRuntime, |cell, _| {
        GateReport::pass("gate_cell", &format!("{} ok", cell.id))
    });
    match out {
        LoomOutcome::Committed { results, ledger } => {
            assert_eq!(results.len(), 2);
            assert!(cce_core::ledger::verify_ledger(&ledger).is_ok());
        }
        LoomOutcome::RepairOrReweave { .. } => panic!("unerwarteter Hold"),
    }
    assert!(cells.iter().all(|c| c.status == WorkcellStatus::Sealed));
}

/// L6 — Nadel = Radfenster: der generative Faden nutzt DENSELBEN Mechanismus
/// wie die beobachtende Apertur (I-7 — der mechanische Kern des ≃).
#[test]
fn l06_needle_equals_wheel_window() {
    let w = WheelWindow::new("w", "cell:a", "p", "G1-Scope");
    let parts = vec![
        ("cell:a".to_string(), CanonValue::text("faden")),
        ("cell:b".to_string(), CanonValue::text("fremd")),
    ];
    let needle = w.needle(&parts);
    let (aperture, _) = w.aperture(&parts);
    assert_eq!(needle, aperture.into_iter().cloned().collect::<Vec<_>>());
}

/// L7 — Workcell-Vertrag: keine Prompt-Regression; Statusachse vollstaendig.
#[test]
fn l07_workcell_contract() {
    const _: () = assert!(LoomWorkcell::NO_PROMPT_REGRESSION);
    let c = LoomWorkcell::new("w", "c", CanonValue::Null, "t", &["op"]);
    assert_eq!(c.status, WorkcellStatus::Pending);
    assert_eq!(c.gate_chain.len(), 5, "G1–G5 je Workcell (Teil 7.3)");
}

/// L8 — gate_failed wird NIE verschmolzen: Hold beendet den Lauf mit
/// Diagnose (repair_or_reweave).
#[test]
fn l08_gate_failed_never_merged() {
    let mut cells = vec![
        LoomWorkcell::new("w1", "c1", CanonValue::Int(1), "t", &["render"]),
        LoomWorkcell::new("w2", "c2", CanonValue::Int(2), "t", &["render"]),
    ];
    let out = loom_generate(&mut cells, &EchoRuntime, |cell, _| {
        if cell.id == "w1" {
            GateReport::hold("gate_cell", "w1 verletzt Scope")
        } else {
            GateReport::pass("gate_cell", "ok")
        }
    });
    match out {
        LoomOutcome::RepairOrReweave {
            failed_cell,
            report,
            ..
        } => {
            assert_eq!(failed_cell, "w1");
            assert!(!report.is_pass());
            assert_eq!(cells[0].status, WorkcellStatus::GateFailed);
            // w2 wurde nie ausgefuehrt — kein Weiterlaufen ueber Hold hinweg.
            assert_eq!(cells[1].status, WorkcellStatus::Pending);
        }
        LoomOutcome::Committed { .. } => panic!("Hold wurde verschmolzen (verboten)"),
    }
}

/// L9 — Replay: gleicher Eingang ⇒ gleicher Ledger-Kopf (Klassenidentitaet).
#[test]
fn l09_replay_identity() {
    let run = || {
        let mut cells = vec![LoomWorkcell::new(
            "w1",
            "c1",
            CanonValue::Int(7),
            "t",
            &["render"],
        )];
        match loom_generate(&mut cells, &EchoRuntime, |_, _| {
            GateReport::pass("gate_cell", "ok")
        }) {
            LoomOutcome::Committed { ledger, .. } => ledger.head(),
            _ => panic!(),
        }
    };
    assert_eq!(run(), run());
}

/// L10 — Reanalyse-Gate: AcceptLOOM verlangt Obs(A) ≃ C — ein divergentes
/// Gewebe faellt durch (hier: Klassenvergleich zweier Gewebe).
#[test]
fn l10_reanalysis_gate() {
    let a = weave(&projection()).unwrap();
    let mut p2 = projection();
    if let CanonValue::Map(m) = &mut p2.payload {
        m.insert("ordering".into(), CanonValue::text("neutral"));
    }
    let b: Weave = weave(&p2).unwrap();
    assert_ne!(
        a.canonical_class(),
        b.canonical_class(),
        "Ordnungswechsel ist semantisch (meaningful↔neutral) und MUSS die Klasse aendern"
    );
}
