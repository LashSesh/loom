//! Etappe X2/E2 (S-E2a I.6) — Zeuge R-CIT-3: Red(3)-Referenzprojekt
//! (2 Mappen + Welt-Crystal + 1 Blueprint), MSC(1→2→3) gruen. Die
//! SCALE-3-"cites"-Naht (Mappe B -> Welt-Kristall) ist eine ECHTE,
//! ueberpruefbare Tatsache: Mappe B buendelt das zitierende Memo als
//! Kind, CitationGate auf diesem Kind reproduziert die Stuetzung.

use cce_materialize::scale3_project::{
    materialize_index, parse_index, project_equivalent, project_seams_valid, CellKind,
    ProjectEntry, Scale3Project, PROJECT_ROOT,
};
use cce_spiral::bluered::{close_red, BlueCube, RedCube, RedVerdict};
use cce_spiral::s15::{
    check_scale_adapter_parity, multi_scale_closure, scale1_adapter, Multicube, ScaleAdapter,
};
use loom_cites::{citation_gate, hex34, CitationOutcome, CitationResolver, SeedResolver};
use loom_conformance::{
    build_blueprint_reference_cube, build_scale2_folder_b, build_scale2_folder_full,
    build_welt_kristall_wikimedia,
};

fn seed_dir() -> std::path::PathBuf {
    std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../library/seed"))
}

/// Analog `scale2_adapter()` (conformance/tests/scale2_msc.rs) — dieselbe
/// 8/8-Vertragsform, eine Skala hoeher.
fn scale3_adapter() -> ScaleAdapter {
    ScaleAdapter {
        scale: 3,
        cell_type: "Projektraum (Mappen + Quellen + Blueprints)".to_string(),
        phase_set: vec!["buendeln".into(), "verbinden".into(), "verdichten".into()],
        close_blue_rule: "jede Projekt-Naht (contains/cites/precedes) konsistent".to_string(),
        close_red_rule:
            "alle Zellen geschlossen ∧ alle supports/derives-cites Gate-Pass ∧ Projekt-Index reanalyze-klassenidentisch"
                .to_string(),
        wrap_policy_desc: "Projekt-Index traegt Zellordnung; annullierter Drift sichtbar".to_string(),
        promotion_gate: "Promote(SCALE-3) ⟺ MultiScaleClosure(1→2→3)".to_string(),
        residue_vocab: vec!["project_seam_gap".into(), "project_reanalyze_drift".into()],
        replay_contract: "fixe Zellen-Klassen ⇒ fixe Projekt-Klasse".to_string(),
    }
}

fn real_project() -> (Scale3Project, [u8; 34]) {
    let welt = build_welt_kristall_wikimedia();
    let welt_root_hex = hex34(&welt.core_root);
    let mappe_a = build_scale2_folder_full();
    let mappe_b = build_scale2_folder_b(&welt_root_hex);
    let blueprint = build_blueprint_reference_cube();

    let project = Scale3Project::new(
        "Referenzprojekt X2/E2",
        vec![
            ProjectEntry {
                id: "mappe_a".to_string(),
                cell_kind: CellKind::Folder,
                core_root_hex: hex34(&mappe_a.core_root),
            },
            ProjectEntry {
                id: "mappe_b".to_string(),
                cell_kind: CellKind::Folder,
                core_root_hex: hex34(&mappe_b.core_root),
            },
            ProjectEntry {
                id: "welt_kristall".to_string(),
                cell_kind: CellKind::Source,
                core_root_hex: welt_root_hex,
            },
            ProjectEntry {
                id: "blueprint".to_string(),
                cell_kind: CellKind::Blueprint,
                core_root_hex: hex34(&blueprint.core_root),
            },
        ],
        &[
            (PROJECT_ROOT, "contains", "mappe_a"),
            (PROJECT_ROOT, "contains", "mappe_b"),
            (PROJECT_ROOT, "contains", "welt_kristall"),
            (PROJECT_ROOT, "contains", "blueprint"),
            ("mappe_a", "precedes", "mappe_b"),
            ("mappe_b", "cites", "welt_kristall"),
        ],
    );
    (project, welt.core_root)
}

#[test]
fn scale3_adapter_parity_8_of_8() {
    assert!(check_scale_adapter_parity(&scale3_adapter()).is_pass());
    // SCALE-1/2-Adapter bleiben unveraendert gueltig.
    assert!(check_scale_adapter_parity(&scale1_adapter()).is_pass());
}

#[test]
fn red3_kerntest_project_seams_valid_and_reanalyze_identical() {
    let (project, _) = real_project();
    assert!(project_seams_valid(&project));
    let index = materialize_index(&project);
    let back = parse_index(&index).expect("Reanalyse des Projekts");
    assert!(
        project_equivalent(&back, &project),
        "Projekt: Reanalyze ≄ id"
    );
}

#[test]
fn multi_scale_closure_1_to_3_green() {
    let blue = |scale: u8| BlueCube {
        scale,
        phase: "phase".to_string(),
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
        blues: vec![blue(scale)],
        seams_valid: true,
        replay_ok: true,
    };
    assert!(matches!(
        close_red(&red(3)),
        RedVerdict::Promote { to_scale: 4 }
    ));
    let mc = Multicube {
        reds: vec![red(0), red(1), red(2), red(3)],
        capsules_active: 0,
    };
    assert!(
        multi_scale_closure(&mc, 3).is_ok(),
        "MSC(1→2→3) muss schliessen"
    );
}

/// R-CIT-3-Kern: "Schliessung" ist keine Behauptung ohne Gegenprobe —
/// jede Zelle muss real aufloesbar+nicht-rot sein, UND die deklarierte
/// SCALE-3-cites-Naht (mappe_b -> welt_kristall) muss durch eine ECHTE,
/// gate-passende cites-Kante in einem der Mappe-B-Kinder gedeckt sein.
#[test]
fn project_closure_is_backed_by_real_resolvable_cells_and_a_real_cites_edge() {
    let (project, welt_root) = real_project();
    let welt_root_hex = hex34(&welt_root);
    let resolver = SeedResolver::new(vec![seed_dir()]);

    // 1) alle enthaltenen Koerper sind real aufloesbar und nicht rot.
    for e in &project.entries {
        let target = resolver
            .resolve(&e.core_root_hex)
            .unwrap_or_else(|| panic!("Zelle {} muss auflösbar sein", e.id));
        assert!(
            matches!(
                target.verdict,
                loom_verify::Verdict::Valid | loom_verify::Verdict::ValidWithResidues
            ),
            "Zelle {} darf nicht rot sein: {:?}",
            e.id,
            target.verdict
        );
    }

    // 2) die "cites"-Naht mappe_b -> welt_kristall ist real: mindestens
    // eines der in Mappe B gebuendelten Kinder zitiert (supports/derives,
    // Gate-Pass) exakt den Welt-Kristall-core_root.
    let mappe_b_bytes = std::fs::read(seed_dir().join("scale2_projektmappe_b.loom"))
        .expect("Mappe B muss als Seed vorliegen");
    let handle = loom_mount::open(&mappe_b_bytes).expect("Mappe B oeffnen");
    let children = loom_mount::all_cas_blobs(&handle).expect("Mappe B hat gebuendelte Kinder");
    let any_child_cites_welt = children.iter().any(|child_bytes| {
        let gate = citation_gate(child_bytes, &resolver);
        gate.entries.iter().any(|v| {
            v.entry.target_core_root_hex == welt_root_hex && v.outcome == CitationOutcome::Ok
        })
    });
    assert!(
        any_child_cites_welt,
        "mindestens ein Kind von Mappe B muss den Welt-Kristall real (gate-pass) zitieren"
    );
}

#[test]
fn seed_files_match_builders() {
    let (project, _) = real_project();
    let mappe_a_entry = project.entries.iter().find(|e| e.id == "mappe_a").unwrap();
    let mappe_a_on_disk = std::fs::read(seed_dir().join("scale2_projektmappe_full.loom")).unwrap();
    let dec = loom_codec::decode_sealed(&mappe_a_on_disk).unwrap();
    assert_eq!(hex34(&dec.footer.core_root), mappe_a_entry.core_root_hex);
}
