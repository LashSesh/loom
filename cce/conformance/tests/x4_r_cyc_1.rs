//! Etappe X4 (Dokument 16 §1) — R-CYC-1: der Paradigma-Vollzyklus-
//! Zeuge. Acht Stationen, EIN Test, dauerhaft im Waechter: Welt ->
//! Arbeit -> Verbund -> Selbstbezug -> Gedaechtnis -> Rueckwirkung ->
//! sichtbare Erosion -> Replay des Ganzen. Schliesskriterium (§1):
//! R-CYC-1 besteht genau dann, wenn alle acht Stationen in EINEM
//! Zeugen unter dem einen Waechter gruen sind.

use cce_bridge::lifecycle::check_erosion;
use cce_bridge::types::NormStatus;
use cce_core::replay::RunDescriptor;
use cce_core::signature::sha256;
use cce_materialize::adapter::DomainAdapter;
use cce_materialize::document::DocumentAdapter;
use cce_materialize::scale3_project::{
    materialize_index, parse_index, project_equivalent, project_seams_valid,
};
use cce_runner::runner::{Run, RunStatus};
use cce_spiral::bluered::{close_red, BlueCube, RedCube, RedVerdict};
use cce_spiral::s15::{multi_scale_closure, Multicube};
use loom_cites::{citation_gate, hex34, CitationOutcome, SeedResolver};
use loom_conformance::{build_r_cyc_1, build_welt_kristall_wikimedia, InMemoryResolver};

fn seed_dir() -> std::path::PathBuf {
    std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../library/seed"))
}

/// R-CYC-1: alle acht Stationen (§1) in einem Zeugen.
#[test]
fn r_cyc_1_the_full_paradigm_cycle_closes() {
    let run_a = build_r_cyc_1();

    // ---------------------------------------------------------------
    // Station 1 — Quelle: das bestehende, CSA-getragene Welt-Crystal.
    // ---------------------------------------------------------------
    let welt = build_welt_kristall_wikimedia();
    assert_eq!(hex34(&welt.core_root), run_a.welt_root_hex);
    let welt_verdict = loom_verify::verify(&welt.bytes);
    assert!(matches!(
        welt_verdict.verdict,
        loom_verify::Verdict::Valid | loom_verify::Verdict::ValidWithResidues
    ));

    // ---------------------------------------------------------------
    // Station 2 — Arbeit: das neue Memo ist vollstaendig geschlossen
    // (claims.closed via LEDGER) UND sein `supports`-cite auf den
    // Welt-Kristall loest ueber den echten SeedResolver GRUEN auf.
    // ---------------------------------------------------------------
    let memo_verdict = loom_verify::verify(&run_a.memo.bytes);
    assert_eq!(
        memo_verdict.verdict,
        loom_verify::Verdict::Valid,
        "{:?}",
        memo_verdict.diagnoses
    );
    let seed_resolver = SeedResolver {
        search_dirs: vec![seed_dir()],
    };
    let memo_gate = citation_gate(&run_a.memo.bytes, &seed_resolver);
    assert_eq!(memo_gate.entries.len(), 2);
    assert!(
        memo_gate
            .entries
            .iter()
            .all(|e| e.outcome == CitationOutcome::Ok),
        "Station 2: beide cites-Eintraege muessen gruen aufloesen: {:?}",
        memo_gate.entries
    );
    assert!(
        memo_gate.closure_pass,
        "Station 2: CitationGate muss gruen sein"
    );

    // ---------------------------------------------------------------
    // Station 3 — Verbund: Memo in Mappe, Mappe in Projektraum;
    // Red(3) geschlossen, MSC(1->2->3).
    // ---------------------------------------------------------------
    let folder_verdict = loom_verify::verify(&run_a.folder.bytes);
    assert_eq!(folder_verdict.verdict, loom_verify::Verdict::Valid);
    assert!(
        project_seams_valid(&run_a.project),
        "Station 3: Projekt-Naehte muessen konsistent sein"
    );
    let index = materialize_index(&run_a.project);
    let back = parse_index(&index).expect("Station 3: Projekt-Reanalyse");
    assert!(
        project_equivalent(&back, &run_a.project),
        "Station 3: Projekt-Reanalyse muss aequivalent sein"
    );
    // Die deklarierte cites-Naht (Mappe -> Welt-Kristall) ist keine
    // blosse Behauptung: das gebuendelte Memo zitiert den Welt-Kristall
    // tatsaechlich, gate-passend.
    let handle = loom_mount::open(&run_a.folder.bytes).expect("Mappe oeffnen");
    let children = loom_mount::all_cas_blobs(&handle).expect("Mappe hat ein gebuendeltes Kind");
    assert!(
        children.iter().any(|child| {
            let gate = citation_gate(child, &seed_resolver);
            gate.entries.iter().any(|v| {
                v.entry.target_core_root_hex == run_a.welt_root_hex
                    && v.outcome == CitationOutcome::Ok
            })
        }),
        "Station 3: mindestens ein Mappen-Kind muss den Welt-Kristall real zitieren"
    );
    // MSC(1->2->3): dieselbe abstrakte Kernpruefung wie R-CIT-3 (Ring
    // E2) — die Spiral-Kinematik selbst ist unveraendert, skalen-agnostisch.
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
        evidence_refs: vec![sha256(b"r-cyc-1-ev")],
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
        "Station 3: MSC(1->2->3) muss schliessen"
    );

    // ---------------------------------------------------------------
    // Station 4 — Selbstbezug: der HBM-Lauf ueber den realen
    // Projektraum-Bestand ist deterministisch (Replay klassenidentisch).
    // ---------------------------------------------------------------
    let run_a2 = build_r_cyc_1();
    assert_eq!(
        run_a.blueprint_class_hex, run_a2.blueprint_class_hex,
        "Station 4: HBM-Lauf muss klassenidentisch reproduzierbar sein"
    );

    // ---------------------------------------------------------------
    // Station 5 — Gedaechtnis: BridgeGate ALLOW, aktive Norm mit
    // `derives`-cites auf die Herkunft (Station-2/3-Koerper + eine
    // bestehende Familien-Referenz-Cube).
    // ---------------------------------------------------------------
    assert_eq!(
        run_a.bridge_report.verdict(),
        cce_bridge::BridgeVerdict::Allow
    );
    assert_eq!(run_a.norm.status, NormStatus::Active);
    assert_eq!(run_a.norm.provenance_set.len(), 3);
    let norm_verdict = loom_verify::verify(&run_a.norm_sealed.bytes);
    assert_eq!(
        norm_verdict.verdict,
        loom_verify::Verdict::Valid,
        "{:?}",
        norm_verdict.diagnoses
    );
    let norm_resolver = InMemoryResolver::from_sealed(&run_a.provenance_members);
    let norm_gate = citation_gate(&run_a.norm_sealed.bytes, &norm_resolver);
    assert_eq!(norm_gate.entries.len(), 3);
    assert!(
        norm_gate.closure_pass,
        "Station 5: Norm-cites muessen gruen aufloesen"
    );

    // ---------------------------------------------------------------
    // Station 6 — Rueckwirkung: ein Lauf mit norm_profile=[norm_id]
    // erzeugt einen weiteren, unter der Norm geschlossenen Workbody;
    // der RD listet die Norm; Replay klassenidentisch.
    // ---------------------------------------------------------------
    let adapter = DocumentAdapter;
    let crystal = adapter.reference_cube();
    let rd = RunDescriptor::new(adapter.canonicalize(&crystal).0, "document", 42)
        .with_norm(run_a.norm.norm_id.clone());

    let mut activated_first = Run::submit(crystal.clone(), rd.clone()).unwrap();
    activated_first.activated_norms.push(run_a.norm.clone());
    activated_first.run_to_end(None).unwrap();
    assert_eq!(activated_first.status, RunStatus::Closed);
    assert!(activated_first
        .rd
        .norm_profile
        .contains(&run_a.norm.norm_id));

    let mut activated_second = Run::submit(crystal, rd).unwrap();
    activated_second.activated_norms.push(run_a.norm.clone());
    activated_second.run_to_end(None).unwrap();
    assert_eq!(activated_second.status, RunStatus::Closed);
    assert_eq!(
        activated_first.result_class(),
        activated_second.result_class(),
        "Station 6: gleiches norm_profile muss dieselbe Klasse ergeben"
    );

    // ---------------------------------------------------------------
    // Station 7 — Erosionsprobe (zerstoerungsfrei): in einer
    // Test-KOPIE des Resolvers (nicht der Produktiv-Registry) wird ein
    // Herkunfts-Mitglied "invalidiert" (nicht mehr aufloesbar) -> die
    // Norm faellt auf deprecated; das ORIGINAL bleibt unberuehrt.
    // ---------------------------------------------------------------
    let eroded_resolver = InMemoryResolver::from_sealed_subset(&run_a.provenance_members, &[0, 1]);
    let deprecated = check_erosion(&run_a.norm, &eroded_resolver)
        .expect("Station 7: Erosion muss erkannt werden");
    assert_eq!(deprecated.status, NormStatus::Deprecated);
    // Zerstoerungsfrei: das Original ist unveraendert Active geblieben.
    assert_eq!(run_a.norm.status, NormStatus::Active);

    // ---------------------------------------------------------------
    // Station 8 — Replay des Ganzen: ein zweiter, unabhaengiger
    // Gesamtaufbau ist an JEDER Station klassenidentisch.
    // ---------------------------------------------------------------
    let run_b = build_r_cyc_1();
    assert_eq!(
        run_a.welt_root_hex, run_b.welt_root_hex,
        "Station 8: Quelle"
    );
    assert_eq!(
        hex34(&run_a.memo.core_root),
        hex34(&run_b.memo.core_root),
        "Station 8: Arbeit"
    );
    assert_eq!(
        hex34(&run_a.folder.core_root),
        hex34(&run_b.folder.core_root),
        "Station 8: Verbund (Mappe)"
    );
    assert!(
        project_equivalent(&run_a.project, &run_b.project),
        "Station 8: Verbund (Projektraum)"
    );
    assert_eq!(
        run_a.blueprint_class_hex, run_b.blueprint_class_hex,
        "Station 8: Selbstbezug"
    );
    assert_eq!(
        run_a.norm.norm_id, run_b.norm.norm_id,
        "Station 8: Gedaechtnis"
    );
    assert_eq!(
        hex34(&run_a.norm_sealed.core_root),
        hex34(&run_b.norm_sealed.core_root),
        "Station 8: Norm-Workbody-Bytes"
    );
}
