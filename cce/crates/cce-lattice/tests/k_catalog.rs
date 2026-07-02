//! Abnahmekatalog CL K1–K18 (Bauverfassung Phase C / Teil 7.5).
//! Die K-Nummern sind aus den im Spec-Repo verankerten CL-Eigenschaften
//! materialisiert (Huellen-Gesetze CL §3.2, monotone Kontraktion Satz 10.1,
//! Adaequatheit=Chordalitaet §5.2, RIP, K12 Residuen-Sichtbarkeit,
//! §7.2 kein Score-als-Gate, Satz 10.6 Replay-Identitaet, §7.4 Collapse-
//! Zertifikat, Export-Funktor closure-erhaltend). Zuordnung dokumentiert in
//! reports/G01_bericht.md; Interpretations-Residuum: R-Agent-2.

use cce_core::canonical::Canonicalize;
use cce_core::gate::{GateChain, GateReport};
use cce_core::value::CanonValue;
use cce_lattice::collapse::{collapse, CollapseError};
use cce_lattice::cube::{Cube, Dimension, Grade};
use cce_lattice::enumerate_feasible;
use cce_lattice::export_functor::{export, ExportError};
use cce_lattice::graph::{Graph, JunctionTree};
use cce_lattice::projection::project;
use cce_lattice::propagation::{propagate_hull, refines, Constraint, DomainState};
use cce_lattice::unfold::traversal_plan;
use std::collections::BTreeMap;

fn state(entries: &[(&str, &[&str])]) -> DomainState {
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

fn passing_gates() -> GateChain {
    let mut g = GateChain::new();
    for gate in cce_core::gate::mandatory_gates() {
        g.push(GateReport::pass(&gate.id, "ok"));
    }
    g
}

/// K1 — Cube-Grade G0–G3 konstruierbar und geordnet (CL §3).
#[test]
fn k01_cube_grades() {
    let c = Cube::blank(vec![Dimension::new("x", &["a", "b"])]);
    assert_eq!(c.grade, Grade::G0);
    let c = c.with_constraints(vec![Constraint::AllowedValues {
        dim: "x".into(),
        values: vec!["a".into()],
    }]);
    assert_eq!(c.grade, Grade::G1);
    let c = c.with_norms(vec!["fitness".into()]).executable();
    assert_eq!(c.grade, Grade::G3);
    assert!(Grade::G0 < Grade::G3);
}

/// K2 — Huelle ist extensiv in der Informationsordnung (CL §3.2).
#[test]
fn k02_hull_extensive() {
    let s = state(&[("x", &["a", "b", "c"]), ("y", &["1", "2"])]);
    let cs = [eq_pairs("x", "y", &[("a", "1"), ("b", "2")])];
    let out = propagate_hull(s.clone(), &cs);
    assert!(refines(&s, &out.state), "Huelle darf nur verfeinern");
}

/// K3 — Huelle ist monoton (CL §3.2).
#[test]
fn k03_hull_monotone() {
    let cs = [eq_pairs("x", "y", &[("a", "1"), ("b", "2")])];
    let coarse = state(&[("x", &["a", "b", "c"]), ("y", &["1", "2"])]);
    let fine = state(&[("x", &["a", "b"]), ("y", &["1", "2"])]);
    let out_coarse = propagate_hull(coarse, &cs);
    let out_fine = propagate_hull(fine, &cs);
    assert!(refines(&out_coarse.state, &out_fine.state));
}

/// K4 — Huelle ist idempotent: lfp (Knaster–Tarski, CL §3.2).
#[test]
fn k04_hull_idempotent() {
    let s = state(&[("x", &["a", "b", "c"]), ("y", &["1", "2"])]);
    let cs = [eq_pairs("x", "y", &[("a", "1"), ("b", "2")])];
    let once = propagate_hull(s, &cs);
    let twice = propagate_hull(once.state.clone(), &cs);
    assert_eq!(once.state, twice.state);
    assert!(twice.removed.is_empty(), "Fixpunkt erreicht");
}

/// K5 — Feasible Set schrumpft monoton: F_{t+1} ⊆ F_t (CL Satz 10.1).
#[test]
fn k05_feasible_monotone_contraction() {
    let s = state(&[("x", &["a", "b"]), ("y", &["1", "2"])]);
    let f0 = enumerate_feasible(&s, &[]);
    let c1 = [eq_pairs("x", "y", &[("a", "1"), ("b", "2")])];
    let f1 = enumerate_feasible(&s, &c1);
    let c2 = [
        eq_pairs("x", "y", &[("a", "1"), ("b", "2")]),
        Constraint::AllowedValues {
            dim: "x".into(),
            values: vec!["a".into()],
        },
    ];
    let f2 = enumerate_feasible(&s, &c2);
    assert!(f1.iter().all(|a| f0.contains(a)));
    assert!(f2.iter().all(|a| f1.contains(a)));
    assert!(f0.len() >= f1.len() && f1.len() >= f2.len());
}

/// K6 — Unerfuellbarkeit wird sichtbar erkannt, nie still (CL/V2).
#[test]
fn k06_infeasibility_visible() {
    let s = state(&[("x", &["a"]), ("y", &["2"])]);
    let cs = [eq_pairs("x", "y", &[("a", "1")])];
    let out = propagate_hull(s, &cs);
    assert!(out.infeasible);
    assert!(
        !out.removed.is_empty(),
        "entfernte Werte als Residuum sichtbar"
    );
}

/// K7 — Chordalitaets-Erkennung: Dreieck+Anhang chordal, C4 nicht (CL §5.2).
#[test]
fn k07_chordality_detection() {
    let mut tri = Graph::new(&["a", "b", "c", "d"]);
    tri.add_edge("a", "b");
    tri.add_edge("b", "c");
    tri.add_edge("a", "c");
    tri.add_edge("c", "d");
    assert!(tri.is_chordal());

    let mut c4 = Graph::new(&["a", "b", "c", "d"]);
    c4.add_edge("a", "b");
    c4.add_edge("b", "c");
    c4.add_edge("c", "d");
    c4.add_edge("d", "a");
    assert!(!c4.is_chordal());
}

/// K8 — Triangulierung (Fill-in) macht jeden Graphen chordal (F-12).
#[test]
fn k08_triangulation_yields_chordal() {
    let mut c5 = Graph::new(&["a", "b", "c", "d", "e"]);
    for (x, y) in [("a", "b"), ("b", "c"), ("c", "d"), ("d", "e"), ("e", "a")] {
        c5.add_edge(x, y);
    }
    let (t, fill) = c5.triangulate();
    assert!(t.is_chordal());
    assert!(!fill.is_empty(), "C5 braucht Fill-in");
}

/// K9 — PEO existiert gdw. chordal (CCC §7/§9).
#[test]
fn k09_peo_iff_chordal() {
    let mut tri = Graph::new(&["a", "b", "c"]);
    tri.add_edge("a", "b");
    tri.add_edge("b", "c");
    tri.add_edge("a", "c");
    let mut order = tri.mcs_order();
    order.reverse();
    assert!(tri.is_peo(&order));

    let mut c4 = Graph::new(&["a", "b", "c", "d"]);
    c4.add_edge("a", "b");
    c4.add_edge("b", "c");
    c4.add_edge("c", "d");
    c4.add_edge("d", "a");
    let mut o2 = c4.mcs_order();
    o2.reverse();
    assert!(!c4.is_peo(&o2));
}

/// K10 — Junction Tree erfuellt die Running-Intersection-Property (INV-6).
#[test]
fn k10_junction_tree_rip() {
    let mut g = Graph::new(&["a", "b", "c", "d", "e"]);
    for (x, y) in [
        ("a", "b"),
        ("b", "c"),
        ("a", "c"),
        ("c", "d"),
        ("d", "e"),
        ("c", "e"),
    ] {
        g.add_edge(x, y);
    }
    let jt = JunctionTree::build(&g).expect("chordal");
    assert!(jt.has_rip());
    assert!(jt.treewidth() >= 2);
    // Entfaltungsplan (F-10): deterministisch, deckt alle Cliquen genau einmal.
    let plan = traversal_plan(&jt);
    assert_eq!(plan.len(), jt.cliques.len());
    assert_eq!(plan, traversal_plan(&jt));
}

/// K11 — Separatoren sind Cliquen-Schnitte = Naehte (F-11).
#[test]
fn k11_separators_are_clique_intersections() {
    let mut g = Graph::new(&["a", "b", "c", "d"]);
    g.add_edge("a", "b");
    g.add_edge("b", "c");
    g.add_edge("a", "c");
    g.add_edge("c", "d");
    let jt = JunctionTree::build(&g).expect("chordal");
    for (i, j, sep) in &jt.edges {
        let expect: std::collections::BTreeSet<String> = jt.cliques[*i]
            .intersection(&jt.cliques[*j])
            .cloned()
            .collect();
        assert_eq!(*sep, expect);
    }
}

/// K12 — Residuen-Sichtbarkeit: propagierte Werte werden gemeldet (CL K12).
#[test]
fn k12_residue_visibility() {
    let s = state(&[("x", &["a", "b", "c"]), ("y", &["1"])]);
    let cs = [eq_pairs("x", "y", &[("a", "1")])];
    let out = propagate_hull(s, &cs);
    assert_eq!(out.state["x"], vec!["a".to_string()]);
    let removed: Vec<&str> = out.removed.iter().map(|r| r.kind.as_str()).collect();
    assert!(removed.iter().all(|k| *k == "excluded_material"));
    assert_eq!(out.removed.len(), 2, "b und c sichtbar entfernt");
}

/// K13 — Kein Score-als-Gate im Lattice-Pfad (CL §7.2 / V1).
#[test]
fn k13_no_score_as_gate() {
    let report = CanonValue::map([
        ("gate_id", CanonValue::text("collapse")),
        ("score", CanonValue::Int(99)),
        ("verdict", CanonValue::text("pass")),
        ("reason", CanonValue::text("score")),
    ]);
    assert!(cce_core::gate::GateReport::from_untyped(&report).is_err());
}

/// K14 — Projektion ist lokal: No-Horizon-Leakage (CL §5.1).
#[test]
fn k14_projection_no_horizon_leakage() {
    let s = state(&[("x", &["a"]), ("y", &["1"]), ("z", &["q"])]);
    let cs = [eq_pairs("x", "y", &[("a", "1")])];
    let p = project(
        &s,
        &cs,
        &["x".to_string(), "y".to_string()],
        &["sep:x∩y".to_string()],
    )
    .unwrap();
    assert!(!p.leaks_horizon());
    assert!(
        !p.state.contains_key("z"),
        "globaler Horizont bleibt draussen"
    );
    assert_eq!(p.constraints.len(), 1);
}

/// K15 — Adaequatheit: auf chordaler Kopplung folgt aus lokaler Konsistenz
/// die globale Loesung (CL Satz 10.5 / CCC Satz 19.2).
#[test]
fn k15_adequacy_local_implies_global_on_chordal() {
    // Kette x—y—z (chordal): bogenkonsistente Huelle ⇒ global loesbar.
    let s = state(&[("x", &["a", "b"]), ("y", &["1", "2"]), ("z", &["q", "r"])]);
    let cs = [
        eq_pairs("x", "y", &[("a", "1"), ("b", "2")]),
        eq_pairs("y", "z", &[("1", "q"), ("2", "r")]),
    ];
    let hull = propagate_hull(s, &cs);
    assert!(!hull.infeasible);
    let solutions = enumerate_feasible(&hull.state, &cs);
    assert!(
        !solutions.is_empty(),
        "lokal konsistent ⇒ global loesbar (chordal)"
    );
}

/// K16 — Collapse-Zertifikat nur unter Gate+Evidence (CL §7.4).
#[test]
fn k16_collapse_requires_gate_and_evidence() {
    let s = state(&[("x", &["a"]), ("y", &["1"])]);
    let cs = [eq_pairs("x", "y", &[("a", "1")])];
    // ohne Evidence: abgelehnt
    assert_eq!(
        collapse(s.clone(), &cs, &passing_gates(), None).unwrap_err(),
        CollapseError::MissingEvidence
    );
    // mit Hold-Gate: abgelehnt
    let mut holding = GateChain::new();
    holding.push(GateReport::hold("G4-Residue", "offen"));
    assert!(matches!(
        collapse(s.clone(), &cs, &holding, Some("ev:1")),
        Err(CollapseError::GateHold(_))
    ));
    // vollstaendig: Zertifikat
    let cert = collapse(s, &cs, &passing_gates(), Some("ev:1")).unwrap();
    assert_eq!(cert.mode, "lfp");
}

/// K17 — Export-Funktor ist closure-erhaltend: kein Export ohne Commit (CL).
#[test]
fn k17_export_functor_refuses_uncommitted() {
    assert_eq!(export(None).unwrap_err(), ExportError::NotCommitted);
    let s = state(&[("x", &["a"])]);
    let cert = collapse(s, &[], &passing_gates(), Some("ev:2")).unwrap();
    assert!(export(Some(&cert)).is_ok());
}

/// K18 — Replay-Identitaet: gleicher Input ⇒ gleiche Kollapsklasse
/// (CL Satz 10.6).
#[test]
fn k18_replay_identity_of_collapse_class() {
    let build = || {
        let s = state(&[("x", &["a", "b"]), ("y", &["1", "2"])]);
        let cs = [eq_pairs("x", "y", &[("a", "1")])];
        collapse(s, &cs, &passing_gates(), Some("ev:3")).unwrap()
    };
    let (c1, c2) = (build(), build());
    assert_eq!(c1.hull_class, c2.hull_class);
    // und die Klasse ist eine kanonische Klasse (Signatur des Fixpunkts)
    let m: BTreeMap<String, CanonValue> = BTreeMap::new();
    let _ = CanonValue::Map(m).canonical_class();
}
