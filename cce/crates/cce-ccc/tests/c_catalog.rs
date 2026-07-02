//! Abnahmekatalog CCC C1–C14 (Bauverfassung Phase D / Teil 7.5).
//! C-Nummern materialisiert aus den verankerten CCC-Saetzen: §1 Can/σ,
//! §2 Randprojektoren, §3/§6 Faser+Closure, §7/§9 Chordalitaet/PEO,
//! §8 Junction Tree/RIP, Lemma 12.2 + Satz 13.1 Verklebung, §14/Satz 19.4
//! Zwei-Sweep, Bi-Temporalitaet, §18 Kristall-Protokoll, C13 Score/Gate
//! (explizit zitiert in Bauverfassung P8), Satz 19.5 Replay/Konfluenz.
//! Zuordnung: reports/G01_bericht.md; Interpretations-Residuum R-Agent-2.

use cce_ccc::bitemporal::BiTemporal;
use cce_ccc::crystal_protocol::{is_crystal, CrystalCandidate};
use cce_ccc::fiber::{separator_consistent, NodeFiber};
use cce_ccc::glue::{glue, GlueError};
use cce_ccc::sweeps::{is_calibrated, two_sweep, ClusterTree};
use cce_core::canonical::Canonicalize;
use cce_core::gate::{GateChain, GateReport};
use cce_core::reflection::{involute, project_accepted, project_residual, reflect, PolarItem};
use cce_core::residue::ResidueField;
use cce_core::signature::sign_value;
use cce_core::value::CanonValue;
use cce_lattice::graph::{Graph, JunctionTree};
use cce_lattice::propagation::{Constraint, DomainState};
use std::collections::BTreeSet;

fn vars(list: &[&str]) -> BTreeSet<String> {
    list.iter().map(|s| s.to_string()).collect()
}

fn dstate(entries: &[(&str, &[&str])]) -> DomainState {
    entries
        .iter()
        .map(|(k, vs)| (k.to_string(), vs.iter().map(|v| v.to_string()).collect()))
        .collect()
}

fn eq_pairs(a: &str, b: &str, pairs: &[(&str, &str)]) -> Constraint {
    Constraint::AllowedPairs {
        a: a.to_string(),
        b: b.to_string(),
        pairs: pairs
            .iter()
            .map(|(x, y)| (x.to_string(), y.to_string()))
            .collect(),
    }
}

/// Beispiel-Baum: Cliquen {x,y} und {y,z} mit Separator {y}.
fn chain_tree() -> ClusterTree {
    let mut g = Graph::new(&["x", "y", "z"]);
    g.add_edge("x", "y");
    g.add_edge("y", "z");
    let jt = JunctionTree::build(&g).expect("Kette ist chordal");
    let fibers = jt
        .cliques
        .iter()
        .map(|c| {
            let state: DomainState = c
                .iter()
                .map(|v| {
                    let dom = match v.as_str() {
                        "x" => vec!["a".to_string(), "b".to_string()],
                        "y" => vec!["1".to_string(), "2".to_string()],
                        _ => vec!["q".to_string(), "r".to_string()],
                    };
                    (v.clone(), dom)
                })
                .collect();
            NodeFiber::new(c.clone(), state)
        })
        .collect();
    ClusterTree {
        jt,
        fibers,
        constraints: vec![
            eq_pairs("x", "y", &[("a", "1")]),
            eq_pairs("y", "z", &[("1", "q"), ("2", "r")]),
        ],
    }
}

/// C1 — Kanonisierung/Signatur: σ = σ∘Can (CCC §1).
#[test]
fn c01_signature_stable_under_can() {
    let raw = CanonValue::Decimal {
        mantissa: 500,
        exponent: -2,
    };
    assert_eq!(sign_value(&raw), sign_value(&raw.normalize()));
}

/// C2 — Randprojektoren: B+B⁻=I, B·B⁻=0, R²=I (CCC §2).
#[test]
fn c02_boundary_projectors() {
    let items = vec![
        PolarItem {
            value: CanonValue::text("p"),
            accepted: true,
        },
        PolarItem {
            value: CanonValue::text("q"),
            accepted: false,
        },
    ];
    assert_eq!(involute(&involute(&items)), items);
    let b = project_accepted(&items);
    let bm = project_residual(&items);
    assert_eq!(b.len() + bm.len(), items.len());
    assert!(project_residual(&b).is_empty());
}

/// C3 — Tripolare Faser: Closed ⟺ Residualpol leer UND ausgewiesen (CCC §3/§6).
#[test]
fn c03_fiber_closure_law() {
    let closed = reflect(
        &[PolarItem {
            value: CanonValue::text("ok"),
            accepted: true,
        }],
        "m",
    );
    assert!(closed.is_closed());
    assert!(closed.residual.is_empty()); // ausgewiesen UND leer
    let open = reflect(
        &[PolarItem {
            value: CanonValue::text("rest"),
            accepted: false,
        }],
        "m",
    );
    assert!(!open.is_closed());
}

/// C4 — Chordales Skelett: Adaequatheit = Chordalitaet (CCC §7/§9).
#[test]
fn c04_chordal_skeleton() {
    let mut g = Graph::new(&["x", "y", "z"]);
    g.add_edge("x", "y");
    g.add_edge("y", "z");
    assert!(g.is_chordal());
    let mut order = g.mcs_order();
    order.reverse();
    assert!(g.is_peo(&order));
}

/// C5 — Junction Tree mit RIP = QSR-Stabilitaet (CCC §8, Identitaet I-2).
#[test]
fn c05_junction_tree_rip() {
    let tree = chain_tree();
    assert!(tree.jt.has_rip());
    assert_eq!(tree.jt.cliques.len(), 2);
}

/// C6 — Verklebung Lemma 12.2: lokale Closure uebertraegt sich auf den
/// Separator (separator-konsistente Familie klebt).
#[test]
fn c06_local_closure_transfers_to_separator() {
    let f1 = NodeFiber::new(vars(&["x", "y"]), dstate(&[("x", &["a"]), ("y", &["1"])]));
    let f2 = NodeFiber::new(vars(&["y", "z"]), dstate(&[("y", &["1"]), ("z", &["q"])]));
    let sep = vars(&["y"]);
    assert!(f1.locally_closed() && f2.locally_closed());
    assert!(separator_consistent(&f1, &f2, &sep));
    assert!(glue(&[f1, f2], &[(0, 1, sep)]).is_ok());
}

/// C7 — Verklebung ist eindeutig (Satz 13.1): gleiche Familie ⇒ gleicher
/// globaler Zustand, unabhaengig von der Knotenreihenfolge.
#[test]
fn c07_gluing_is_unique() {
    let f1 = NodeFiber::new(vars(&["x", "y"]), dstate(&[("x", &["a"]), ("y", &["1"])]));
    let f2 = NodeFiber::new(vars(&["y", "z"]), dstate(&[("y", &["1"]), ("z", &["q"])]));
    let sep = vars(&["y"]);
    let g1 = glue(&[f1.clone(), f2.clone()], &[(0, 1, sep.clone())]).unwrap();
    let g2 = glue(&[f2, f1], &[(1, 0, sep)]).unwrap();
    assert_eq!(g1, g2);
}

/// C8 — Verklebung verweigert bei Seam-Inkonsistenz (V4: Naht vor Sprung).
#[test]
fn c08_gluing_refuses_inconsistent_seam() {
    let f1 = NodeFiber::new(vars(&["x", "y"]), dstate(&[("x", &["a"]), ("y", &["1"])]));
    let f2 = NodeFiber::new(vars(&["y", "z"]), dstate(&[("y", &["2"]), ("z", &["q"])]));
    let sep = vars(&["y"]);
    assert!(matches!(
        glue(&[f1, f2], &[(0, 1, sep)]),
        Err(GlueError::SeamInconsistent { .. })
    ));
}

/// C9 — Collect-Korrektheit: nach Collect traegt die Wurzel den akzeptierten
/// Pol Bx (konsistente Wertemengen; CCC Satz 19.4-Anteil).
#[test]
fn c09_collect_correct() {
    let mut tree = chain_tree();
    cce_ccc::sweeps::collect_sweep(&mut tree);
    let root = &tree.fibers[tree.jt.root];
    assert!(root.locally_closed());
}

/// C10 — Zwei-Sweep-Korrektheit (INV-7): nach Collect∘Distribute ist der Baum
/// kalibriert (Separator-konsistent) und jede Faser global konsistent.
#[test]
fn c10_two_sweep_calibrates() {
    let mut tree = chain_tree();
    two_sweep(&mut tree);
    assert!(
        is_calibrated(&tree),
        "Separator-Konsistenz nach zwei Sweeps"
    );
    for f in &tree.fibers {
        assert!(f.locally_closed());
    }
    // x=a erzwingt y=1 erzwingt z=q — die globale Marginale ist angekommen.
    let yz = tree
        .fibers
        .iter()
        .find(|f| f.vars.contains("z"))
        .expect("Clique {y,z}");
    assert_eq!(yz.state["y"], vec!["1".to_string()]);
    assert_eq!(yz.state["z"], vec!["q".to_string()]);
}

/// C11 — Bi-Temporalitaet: T2 rotiert frei, T1 nur via Commit, n0 existiert
/// als nicht-traversierbarer Anker.
#[test]
fn c11_bitemporal() {
    let mut bt = BiTemporal::new();
    bt.rotate(5);
    bt.rotate(7);
    assert_eq!(bt.t1(), 0, "Rotation erhoeht NIE den Commit-Index");
    bt.commit();
    assert_eq!(bt.t1(), 1);
    assert_eq!(bt.anchor.id, "n0");
}

/// C12 — Kristall-Protokoll (CCC §18): alle fuenf Konjunkte noetig.
#[test]
fn c12_crystal_protocol() {
    let mut gates = GateChain::new();
    for g in cce_core::gate::mandatory_gates() {
        gates.push(GateReport::pass(&g.id, "ok"));
    }
    let good = CrystalCandidate {
        closed: true,
        qsr_stable: true,
        gates: gates.clone(),
        replay_ok: true,
        residue_field: Some(ResidueField::new()),
    };
    assert!(is_crystal(&good).is_ok());
    // fehlendes Residuenfeld (auch leer waere Pflicht!) ⇒ kein Kristall.
    let missing_field = CrystalCandidate {
        residue_field: None,
        ..good.clone()
    };
    let errs = is_crystal(&missing_field).unwrap_err();
    assert!(errs.iter().any(|e| e.contains("Residuenfeld")));
    // fehlendes Replay ⇒ kein Kristall.
    let no_replay = CrystalCandidate {
        replay_ok: false,
        ..good
    };
    assert!(is_crystal(&no_replay).is_err());
}

/// C13 — Score misst, Gate entscheidet (CCC C13, zitiert in Bauverfassung P8).
#[test]
fn c13_score_measures_gate_decides() {
    let attempt = CanonValue::map([
        ("gate_id", CanonValue::text("crystal-accept")),
        ("score", CanonValue::Int(100)),
        ("verdict", CanonValue::text("pass")),
        ("reason", CanonValue::text("hoher score")),
    ]);
    assert!(
        cce_core::gate::GateReport::from_untyped(&attempt).is_err(),
        "Score-als-Gate MUSS abgelehnt werden (V1)"
    );
}

/// C14 — Konfluenz/Replay (CCC Satz 19.5): gleicher Input ⇒ gleiche
/// Commit-Klasse, reihenfolgeunabhaengig.
#[test]
fn c14_replay_confluence() {
    let run = || {
        let mut tree = chain_tree();
        two_sweep(&mut tree);
        let states: Vec<CanonValue> = tree
            .fibers
            .iter()
            .map(|f| {
                CanonValue::Map(
                    f.state
                        .iter()
                        .map(|(k, v)| {
                            (
                                k.clone(),
                                CanonValue::List(v.iter().map(CanonValue::text).collect()),
                            )
                        })
                        .collect(),
                )
            })
            .collect();
        CanonValue::List(states).canonical_class()
    };
    assert_eq!(run(), run());
}
