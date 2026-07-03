//! E/P8-Zeugen (SCALE-2): Red(2)-Kerntest der Dokumentenmappe +
//! MultiScaleClosure(1→2) über die S15-Multicube + Scale-2-Adapter-
//! Parität. SCALE-1-Pfade bleiben unberührt (der eine Wächter beweist
//! es: alle bisherigen Zeugen laufen weiter).

use cce_materialize::document::assets::three_risks_memo;
use cce_materialize::scale2_folder::{
    folder_equivalent, folder_seams_valid, materialize_index, parse_index, DocFolder,
};
use cce_spiral::bluered::{close_red, BlueCube, RedCube, RedVerdict};
use cce_spiral::s15::{check_scale_adapter_parity, multi_scale_closure, scale1_adapter, Multicube};

fn scale2_adapter() -> cce_spiral::s15::ScaleAdapter {
    cce_spiral::s15::ScaleAdapter {
        scale: 2,
        cell_type: "Dokument-Workbody (D01-Mappe)".to_string(),
        phase_set: vec!["sammeln".into(), "verknuepfen".into(), "buendeln".into()],
        close_blue_rule: "jede Mappe-Naht (Verweis/Reihenfolge) konsistent".to_string(),
        close_red_rule: "alle Memos geschlossen ∧ Mappen-Index reanalyze-klassenidentisch"
            .to_string(),
        wrap_policy_desc: "Mappen-Index tragt Ordnung; annullierter Drift sichtbar".to_string(),
        promotion_gate: "Promote(SCALE-2) ⟺ MultiScaleClosure(1→2)".to_string(),
        residue_vocab: vec!["folder_seam_gap".into(), "folder_reanalyze_drift".into()],
        replay_contract: "fixe Memo-Klassen ⇒ fixe Mappen-Klasse".to_string(),
    }
}

fn folder3() -> DocFolder {
    let m = three_risks_memo();
    DocFolder::from_memos(
        "Projektmappe",
        &[("memo_a", &m), ("memo_b", &m), ("memo_c", &m)],
        &[
            ("memo_a", "precedes", "memo_b"),
            ("memo_b", "precedes", "memo_c"),
        ],
    )
}

#[test]
fn scale2_adapter_parity_8_of_8() {
    assert!(check_scale_adapter_parity(&scale2_adapter()).is_pass());
    // SCALE-1-Adapter bleibt unveraendert gueltig.
    assert!(check_scale_adapter_parity(&scale1_adapter()).is_pass());
}

#[test]
fn red2_kerntest_three_memos_folder_closes_reanalyze_identical() {
    let f = folder3();
    assert!(folder_seams_valid(&f));
    let back = parse_index(&materialize_index(&f)).unwrap();
    assert!(
        folder_equivalent(&back, &f),
        "Red(2)-Kerntest: Reanalyze ≄ id"
    );
}

#[test]
fn multi_scale_closure_1_to_2_green() {
    // RedCube je Skala 0,1,2 — alle geschlossen ⇒ MSC(2) = Ok.
    let blue = |scale: u8, phase: &str| BlueCube {
        scale,
        phase: phase.to_string(),
        typed: true,
        boundary_valid: true,
        gates: {
            let mut g = cce_core::gate::GateChain::new();
            g.push(cce_core::gate::GateReport::pass("G1", "scope ok"));
            g
        },
        evidence_refs: vec![cce_core::signature::sha256(b"ev")],
        replay_ok: true,
        wrap_stable: true,
        residues: cce_core::residue::ResidueField::new(),
    };
    let red = |scale: u8| RedCube {
        scale,
        blues: vec![blue(scale, "phase")],
        seams_valid: true,
        replay_ok: true,
    };
    // SCALE-2-Red schliesst (Promote nach 3).
    assert!(matches!(
        close_red(&red(2)),
        RedVerdict::Promote { to_scale: 3 }
    ));
    let mc = Multicube {
        reds: vec![red(0), red(1), red(2)],
        capsules_active: 0,
    };
    assert!(
        multi_scale_closure(&mc, 2).is_ok(),
        "MSC(1→2) muss schliessen"
    );
}
