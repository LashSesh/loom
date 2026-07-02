//! G4-Ausgangs-Gate (01_MASTER_BUILD): alle 8 Spiral-Gates implementiert +
//! getestet · alle 12 Spiral-Residuen erzeugbar + sichtbar · Negativ-Zeugen
//! (external_drift_detected, wrap_support_violation,
//! ratchet_lock_without_evidence, fibonacci_dogma_import) rot ·
//! Kaskadentest: hoeheres Lock ohne untere Locks/Residuen unmoeglich.

use cce_core::gate::GateReport;
use cce_core::residue::{ResidueField, Severity};
use cce_core::signature::sha256;
use cce_core::value::CanonValue;
use cce_spiral::address::SpiralAddress;
use cce_spiral::area_class::area_class;
use cce_spiral::bluered::{close_blue, close_red, BlueCube, CloseVerdict, RedCube, RedVerdict};
use cce_spiral::dispersion::{DispersionDeclaration, DispersionProfile};
use cce_spiral::expansion::{expand, SpiralState};
use cce_spiral::gates::*;
use cce_spiral::ratchet::{cascade_lock, Ratchet, RatchetKind, RatchetState};
use cce_spiral::residues;
use cce_spiral::s15::{check_scale_adapter_parity, scale1_adapter, Capsule};
use cce_spiral::wrap::{wrap, WrapPolicy};

fn state() -> SpiralState {
    SpiralState::new(
        SpiralAddress::origin(1, 0),
        &["a", "b"],
        CanonValue::map([("kern", CanonValue::Int(1))]),
    )
}

fn policy() -> WrapPolicy {
    WrapPolicy::new(&["a", "b", "c"], &["seam:ab"])
}

fn passing_ratchet() -> Ratchet {
    let mut r = Ratchet::new(RatchetKind::Phase, "rp");
    r.step(&GateReport::pass("g", "ok"), Some(sha256(b"ev")));
    r
}

/// Gate 1+3: Wrap annulliert externe Drift SICHTBAR; Support-Boundedness.
#[test]
fn g1_wrap_and_g3_drift() {
    let z = expand(&state(), &["extern-x"], 100); // waechst UEBER den Boundary
    let w = wrap(&z, &policy(), &ResidueField::new());
    assert!(wrap_gate(&w, &policy()).is_pass());
    assert_eq!(w.annulled_drift.len(), 1);
    assert_eq!(w.annulled_drift[0].kind.as_str(), "external_drift_detected");
    assert_eq!(w.annulled_drift[0].severity, Severity::Blocking);
    assert!(drift_gate(&w).is_pass());
    // Negativ-Zeuge wrap_support_violation: ungewickelter Zustand am Gate.
    let unwrapped = expand(&state(), &["extern-x"], 0);
    let fake = cce_spiral::wrap::Wrapped {
        state: unwrapped,
        preserved_residues: ResidueField::new(),
        annulled_drift: vec![],
        seams_preserved: true,
        replay_preserved: true,
    };
    let r = wrap_gate(&fake, &policy());
    assert!(!r.is_pass());
    assert!(r.reason.contains("wrap_support_violation"));
}

/// Gate 2: AreaClass-Invarianz unter expand∘wrap; Mismatch wird rot.
#[test]
fn g2_area_class() {
    let z = state();
    let boundary = policy().boundary;
    let z2 = expand(&z, &["extern-y"], 250);
    let w = wrap(&z2, &policy(), &ResidueField::new());
    assert!(area_class_gate(&z, &w.state, &boundary).is_pass());
    // Budgetklassenwechsel = Klassenbruch.
    let mut differed = w.state.clone();
    differed.budget_class = 99;
    let r = area_class_gate(&z, &differed, &boundary);
    assert!(!r.is_pass());
    assert!(r.reason.contains("area_class_mismatch"));
    assert_ne!(area_class(&z, &boundary), area_class(&differed, &boundary));
}

/// Gate 4: Phasen-Ratchet — Lock ohne Evidence unmoeglich (Negativ-Zeuge).
#[test]
fn g4_phase_ratchet_lock_without_evidence_red() {
    let mut r = Ratchet::new(RatchetKind::Phase, "r1");
    // Schritt ohne Evidence ⇒ Hold + sichtbares Residuum.
    assert_eq!(
        r.step(&GateReport::pass("g", "ok"), None),
        RatchetState::Hold
    );
    assert_eq!(
        r.visible_residues[0].kind.as_str(),
        "ratchet_lock_without_evidence"
    );
    // Lock aus Hold ⇒ verweigert.
    assert!(r.lock().is_err());
    assert!(phase_ratchet_gate(&r).is_pass(), "kein Lock ⇒ konsistent");
    // Erzwungener inkonsistenter Zustand wird vom Gate erkannt:
    let mut forged = Ratchet::new(RatchetKind::Phase, "r2");
    forged.state = RatchetState::Lock;
    assert!(!phase_ratchet_gate(&forged).is_pass());
}

/// Gate 5: Skalen-Promotion nur nach Close(Red) + WrapStability.
#[test]
fn g5_scale_ratchet() {
    let good_blue = BlueCube {
        scale: 1,
        phase: "p0".into(),
        typed: true,
        boundary_valid: true,
        gates: {
            let mut g = cce_core::gate::GateChain::new();
            for gate in cce_core::gate::mandatory_gates() {
                g.push(GateReport::pass(&gate.id, "ok"));
            }
            g
        },
        evidence_refs: vec![sha256(b"ev")],
        replay_ok: true,
        wrap_stable: true,
        residues: ResidueField::new(),
    };
    assert_eq!(close_blue(&good_blue), CloseVerdict::Closed);
    let red = RedCube {
        scale: 1,
        blues: vec![good_blue.clone()],
        seams_valid: true,
        replay_ok: true,
    };
    assert!(scale_ratchet_gate(&red, true).is_pass());
    // ohne WrapStability: rot mit benanntem Residuum.
    let r = scale_ratchet_gate(&red, false);
    assert!(!r.is_pass());
    assert!(r.reason.contains("scale_promotion_without_wrap_stability"));
    // offener Blue ⇒ Red offen ⇒ keine Promotion.
    let mut open_blue = good_blue;
    open_blue.evidence_refs.clear();
    let red_open = RedCube {
        scale: 1,
        blues: vec![open_blue],
        seams_valid: true,
        replay_ok: true,
    };
    assert!(matches!(close_red(&red_open), RedVerdict::Open(_)));
    assert!(!scale_ratchet_gate(&red_open, true).is_pass());
}

/// Gate 6: Profil statt Dogma — undeklariertes Profil wird abgewiesen
/// (Negativ-Zeugen fibonacci_dogma_import / golden_angle_unjustified).
#[test]
fn g6_dispersion_profile_negatives_red() {
    let undeclared = DispersionDeclaration::default();
    // Golden ohne Deklaration ⇒ golden_angle_unjustified.
    let r = dispersion_profile_gate(&undeclared, DispersionProfile::Golden);
    assert!(!r.is_pass());
    assert!(r.reason.contains("golden_angle_unjustified"));
    // Dyadic ohne Deklaration ⇒ phase_profile_undeclared.
    let r = dispersion_profile_gate(&undeclared, DispersionProfile::Dyadic);
    assert!(!r.is_pass());
    assert!(r.reason.contains("phase_profile_undeclared"));
    // Deklariert ≠ benutzt ⇒ fibonacci_dogma_import.
    let declared_dyadic = DispersionDeclaration {
        declared: Some(DispersionProfile::Dyadic),
    };
    let r = dispersion_profile_gate(&declared_dyadic, DispersionProfile::Golden);
    assert!(!r.is_pass());
    assert!(r.reason.contains("fibonacci_dogma_import"));
    // Korrekt deklariert ⇒ gruen (Golden ist zulaessig ALS PROFIL).
    let declared_golden = DispersionDeclaration {
        declared: Some(DispersionProfile::Golden),
    };
    assert!(dispersion_profile_gate(&declared_golden, DispersionProfile::Golden).is_pass());
}

/// Gate 7: Sektor-Scope-Leak wird erkannt.
#[test]
fn g7_aperture_sector() {
    assert!(aperture_sector_gate("cell:a", &["cell:a".into()]).is_pass());
    let r = aperture_sector_gate("cell:a", &["cell:a".into(), "cell:b".into()]);
    assert!(!r.is_pass());
    assert!(r.reason.contains("aperture_sector_scope_leak"));
}

/// Gate 8: Spiral-Replay — deterministische Kinematik.
#[test]
fn g8_spiral_replay() {
    let profile = DispersionProfile::Dyadic;
    let run = || {
        let mut z = state();
        for k in 0..4 {
            z = expand(&z, &[], profile.step_milliturns(k));
            z = wrap(&z, &policy(), &ResidueField::new()).state;
        }
        z
    };
    let (a, b) = (run(), run());
    assert!(spiral_replay_gate(&a, &b).is_pass());
    let mut c = run();
    c.address = c.address.rotate(7);
    assert!(!spiral_replay_gate(&a, &c).is_pass());
}

/// Alle 12 Residuen erzeugbar + sichtbar (blocking) — Vollstaendigkeit.
#[test]
fn all_12_residues_constructible_and_visible() {
    let all = [
        residues::wrap_support_violation("t"),
        residues::area_class_mismatch("t"),
        residues::external_drift_detected("t"),
        residues::phase_profile_undeclared("t"),
        residues::dispersion_not_replayable("t"),
        residues::ratchet_lock_without_evidence("t"),
        residues::bluecube_not_closed("t"),
        residues::redcube_seam_gap("t"),
        residues::scale_promotion_without_wrap_stability("t"),
        residues::aperture_sector_scope_leak("t"),
        residues::fibonacci_dogma_import("t"),
        residues::golden_angle_unjustified("t"),
    ];
    assert_eq!(all.len(), 12);
    assert_eq!(residues::ALL_SPIRAL_RESIDUES.len(), 12);
    for (r, expected) in all.iter().zip(residues::ALL_SPIRAL_RESIDUES) {
        assert_eq!(r.kind.as_str(), expected);
        assert_eq!(r.severity, Severity::Blocking);
    }
    assert_eq!(ALL_SPIRAL_GATES.len(), 8);
}

/// Kaskadentest: hoeheres Lock ohne untere Locks/Residuen UNMOEGLICH.
#[test]
fn cascade_lock_impossible_without_lower() {
    // Unteres Ratchet weder gelockt noch residuen-sichtbar:
    let clean_lower = Ratchet::new(RatchetKind::Cell, "cell-1");
    let mut ring = passing_ratchet();
    ring.kind = RatchetKind::Ring;
    assert!(cascade_lock(&mut ring, &[&clean_lower]).is_err());
    assert!(!ring.visible_residues.is_empty(), "Verweigerung sichtbar");
    // Unteres gelockt ⇒ Kaskade zulaessig.
    let mut locked_lower = passing_ratchet();
    locked_lower.lock().unwrap();
    let mut ring2 = passing_ratchet();
    ring2.kind = RatchetKind::Ring;
    assert!(cascade_lock(&mut ring2, &[&locked_lower]).is_ok());
    assert_eq!(ring2.state, RatchetState::Lock);
    // Unteres mit sichtbarem Residuum (Counter-Horizon) ⇒ ebenfalls zulaessig.
    let mut hold_lower = Ratchet::new(RatchetKind::Cell, "cell-2");
    hold_lower.step(&GateReport::pass("g", "ok"), None); // Hold + Residuum
    let mut ring3 = passing_ratchet();
    ring3.kind = RatchetKind::Ring;
    assert!(cascade_lock(&mut ring3, &[&hold_lower]).is_ok());
}

/// Irreversibilitaet: ein Lock kennt keinen Ruecklauf; freie Rotation
/// beruehrt den Commitzaehler nie (F6).
#[test]
fn ratchet_irreversible_and_rotation_free() {
    let mut r = passing_ratchet();
    r.lock().unwrap();
    let c = r.commit_counter;
    assert_eq!(
        r.step(&GateReport::pass("g", "ok"), Some(sha256(b"x"))),
        RatchetState::Lock
    );
    assert_eq!(
        r.commit_counter, c,
        "Lock ist irreversibel, kein Weiterzaehlen"
    );
    let mut r2 = Ratchet::new(RatchetKind::Cell, "rot");
    for _ in 0..17 {
        r2.rotate(123);
    }
    assert_eq!(r2.commit_counter, 0, "Rotation zaehlt NIE (F6)");
}

/// S15-Strukturen (DoD-Matrix f): ScaleAdapter-Paritaet, Capsule-Dissolution.
#[test]
fn s15_structures_built() {
    assert!(check_scale_adapter_parity(&scale1_adapter()).is_pass());
    let mut broken = scale1_adapter();
    broken.phase_set.clear();
    assert!(!check_scale_adapter_parity(&broken).is_pass());
    // Capsule-Dissolution: nur Trace/Evidence/Kristalle persistieren.
    let c = Capsule {
        scope: "cell:doc".into(),
        budget: 10,
        projection: "proj:m".into(),
        allowed_ops: vec!["render".into()],
        gates: vec!["G1-Scope".into()],
        evidence_schema: "mef-1".into(),
        ttl: 1,
        ledger_ref: "l0".into(),
    };
    let d = c.dissolve(vec!["t1".into()], vec![sha256(b"e")], vec![sha256(b"k")]);
    assert_eq!(d.trace.len(), 1);
    assert_eq!(d.evidence_refs.len(), 1);
    assert_eq!(d.crystal_classes.len(), 1);
}
