//! TAT-Abnahme P1–P7 (Bauverfassung Teil 7.5 / Phase G) + Rollen-Tests
//! QLOGIC/DZ. Nummernzuordnung: R-Agent-5.

use cce_core::objects::Marker;
use cce_core::value::CanonValue;
use cce_core::wheel_window::WheelWindow;
use cce_observe::optics::optical_pass;
use cce_observe::qlogic::{proof_of_resonance, SpectralRegister};
use cce_observe::substrate::DyadicCell;
use cce_observe::tat::{collect, CollectInput};

fn input() -> CollectInput {
    CollectInput {
        embedded: CanonValue::map([
            ("thema", CanonValue::text("risiko")),
            ("stuetze", CanonValue::text("massnahme")),
        ]),
        markers: vec![
            Marker {
                id: "m1".into(),
                marker_type: "field".into(),
                query: "thema".into(),
                scope: None,
            },
            Marker {
                id: "m2".into(),
                marker_type: "field".into(),
                query: "stuetze".into(),
                scope: None,
            },
            Marker {
                id: "m3".into(),
                marker_type: "field".into(),
                query: "fehlt".into(),
                scope: None,
            },
        ],
        null_models: vec!["reine Selbstbestaetigung".into()],
        provenance: "tat-test".into(),
    }
}

/// P1 — Embed: der Kern wird kanonisiert eingebettet.
#[test]
fn p1_embed_canonical() {
    let mc = collect(&input()).unwrap();
    assert_eq!(mc.trace[0], "Embed");
}

/// P2 — Mark/Respond: jeder Marker erhaelt seinen Response-Schnitt;
/// unbeantwortete werden latent, nie verschwiegen.
#[test]
fn p2_marker_response() {
    let mc = collect(&input()).unwrap();
    assert_eq!(mc.horizons.visible, vec!["m1", "m2"]);
    assert_eq!(mc.horizons.latent, vec!["m3"]);
}

/// P3 — Horizon/Gegenhorizont: Nullmodelle verhindern Selbstbestaetigung.
#[test]
fn p3_counter_horizon_present() {
    let mc = collect(&input()).unwrap();
    assert!(!mc.counter_horizon.null_models.is_empty());
    assert_eq!(mc.null_models, vec!["reine Selbstbestaetigung"]);
}

/// P4 — Triangulation: sichtbare Marker werden trianguliert.
#[test]
fn p4_triangulation() {
    let mc = collect(&input()).unwrap();
    assert_eq!(mc.triangulation, vec![("m1".to_string(), "m2".to_string())]);
}

/// P5 — Gate vor Emission: PoR haelt bei unvollstaendigem Register.
#[test]
fn p5_gate_before_emission() {
    let incomplete = SpectralRegister {
        relation: Some("x".into()),
        ..Default::default()
    };
    assert!(!proof_of_resonance(&incomplete).is_pass());
    let mc = collect(&input()).unwrap();
    assert!(mc.gates.iter().any(|g| g.gate_id == "G-PoR" && g.is_pass()));
}

/// P6 — MatrixCrystal: die Collect-Kette endet im geschlossenen Kristall
/// mit vollstaendigem Trace.
#[test]
fn p6_matrix_crystal_complete() {
    let mc = collect(&input()).unwrap();
    assert_eq!(
        mc.trace,
        vec![
            "Embed",
            "Mark",
            "Respond",
            "Horizon",
            "Triangulate",
            "Gate",
            "Crystalize"
        ]
    );
    assert_eq!(mc.crystal.residue_state, "geschlossen (∅)");
    assert_eq!(mc.replay_hash, mc.class().0);
}

/// P7 — Reanalyse-Divergenz: verschiedener Kern ⇒ verschiedene Klasse;
/// gleicher Kern ⇒ gleiche Klasse (Nachfuehrung erhaelt Fixpunkt).
#[test]
fn p7_reanalysis_divergence_and_fixpoint() {
    let a = collect(&input()).unwrap();
    let b = collect(&input()).unwrap();
    assert_eq!(a.class(), b.class());
    let mut other = input();
    other.embedded = CanonValue::map([("thema", CanonValue::text("anders"))]);
    let c = collect(&other).unwrap();
    assert_ne!(a.class(), c.class());
}

/// DZ-Rolle: dyadische Verfeinerung konsistent.
#[test]
fn dz_role_dyadic_cells() {
    let c = DyadicCell::root().child(1).child(1);
    assert_eq!(c.level(), 2);
    assert_eq!(c.k, 3);
}

/// PIO-Rolle: Apertur vor Fokus — Ueberschuss SICHTBAR als Residuum.
#[test]
fn pio_role_excess_visible() {
    let w = WheelWindow::new("w", "cell:x", "p", "G1-Scope");
    let raw = vec![
        ("cell:x".to_string(), CanonValue::Int(1)),
        ("cell:y".to_string(), CanonValue::Int(2)),
    ];
    let pass = optical_pass(&w, &raw);
    assert_eq!(pass.focused.len(), 1);
    assert_eq!(pass.excess.len(), 1);
    assert!(pass.excess[0].content.contains("sichtbar"));
}
