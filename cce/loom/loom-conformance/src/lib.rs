//! loom-conformance — Referenzdateien R1–R8 (Teil 10.1 + Overlay 05
//! Teil E) als deterministische Builder. Die Golden Files unter
//! `loom/golden/` sind die canonical-stored-Ausprägungen; der Test
//! vergleicht Builder-Bytes gegen die eingecheckten Dateien (C0).

use cce_core::canonical::Canonicalize;
use loom_canon::Cv;
use loom_codec::{seal_canonical, Sealed, Segment};
use loom_format::{
    KIND_ARTIFACT, KIND_CANDIDATE_OUTPUTS, KIND_CANON_DESC, KIND_CAS_BLOB, KIND_CL_SUBSTRATE,
    KIND_CSA_NSB, KIND_DOC, KIND_EVIDENCE, KIND_GATE_REPORTS, KIND_HBM, KIND_INFERENCE_PROFILE,
    KIND_LEDGER, KIND_MANIFEST, KIND_PHC, KIND_PROVIDER_MANIFEST, KIND_REPLAY_MANIFEST,
    KIND_RESIDUE, KIND_RUNTIME_PROFILE, KIND_TOOL_PROFILE,
};

/// MANIFEST mit allen 13 Pflichtfeldern (Teil 3.5).
#[allow(clippy::too_many_arguments)] // Spez-Feldliste (Teil 3.5)
pub fn manifest_cv(
    title: &str,
    container_class: &str,
    pl_level: &str,
    claims_closed: bool,
    residue_count: u64,
    residue_kinds: &[&str],
    capabilities: &[&str],
    license_summary: &str,
) -> Cv {
    Cv::map(vec![
        ("title", Cv::Text(title.into())),
        ("container_class", Cv::Text(container_class.into())),
        (
            "domain_refs",
            Cv::Array(vec![Cv::Text("dom:document".into())]),
        ),
        ("scale", Cv::Uint(1)),
        ("pl_level", Cv::Text(pl_level.into())),
        ("claims", Cv::map(vec![("closed", Cv::Bool(claims_closed))])),
        (
            "origin",
            Cv::map(vec![
                ("tool", Cv::Text("cce-workbench-0.1".into())),
                ("rd_digest", Cv::Text("sha256:rd".into())),
            ]),
        ),
        (
            "profiles_required",
            Cv::Array(vec![Cv::Text(container_class.into())]),
        ),
        ("profiles_optional", Cv::Array(vec![])),
        (
            "residue_summary",
            Cv::map(vec![
                ("count", Cv::Uint(residue_count)),
                (
                    "kinds",
                    Cv::Array(
                        residue_kinds
                            .iter()
                            .map(|k| Cv::Text((*k).into()))
                            .collect(),
                    ),
                ),
            ]),
        ),
        (
            "capability_declarations",
            Cv::Array(capabilities.iter().map(|c| Cv::Text((*c).into())).collect()),
        ),
        ("license_summary", Cv::Text(license_summary.into())),
        // Evidence-Zeitfeld: deklariert, deterministisch (kein Wall-Clock).
        (
            "created",
            Cv::Tag(0, Box::new(Cv::Text("2026-01-01T00:00:00Z".into()))),
        ),
    ])
}

pub fn canon_desc_segment() -> Segment {
    Segment {
        kind: KIND_CANON_DESC,
        seg_flags: 0,
        payload: Cv::Text(loom_canon::CANON_RULES_TEXT.into())
            .encode()
            .unwrap(),
        deps: vec![],
    }
}

fn seg(kind: u16, v: &Cv) -> Segment {
    Segment::canonical(kind, v).expect("kanonisches Segment")
}

/// X1(c) (Oekosystem-Karte §2/E1): erweitert ein bereits gebautes
/// MANIFEST-Cv additiv um die deklarierten Hash-Profile — rein
/// informativ (kein Frame-/Segtab-Digest wechselt das Verfahren; die
/// Frame-Digests bleiben immer sha2-256). Zweitprofile (z. B. blake3)
/// werden von einem CLI-Blatt (loom-cli::hashprofile) berechnet und
/// hier nur DEKLARIERT, damit ein Leser weiss, was zusaetzlich
/// verfuegbar/nachrechenbar ist.
pub fn manifest_declare_hash_profiles(manifest: Cv, profiles: &[&str]) -> Cv {
    let Cv::Map(mut entries) = manifest else {
        panic!("manifest_cv liefert immer eine Map")
    };
    entries.push((
        Cv::Text("hash_profiles".to_string()),
        Cv::Array(
            profiles
                .iter()
                .map(|p| Cv::Text((*p).to_string()))
                .collect(),
        ),
    ));
    Cv::Map(entries)
}

/// S-E2a I.4: erweitert ein MANIFEST-Cv additiv um `external_citations`
/// — MUSS exakt der (dedupliziert-sortierten) Menge der tatsaechlichen
/// `target_core_root`s im CL_SUBSTRATE entsprechen, sonst weist
/// loom-verify mit `manifest_citation_mismatch` zurueck (N-CIT-5).
pub fn manifest_declare_external_citations(manifest: Cv, roots_hex: &[String]) -> Cv {
    let Cv::Map(mut entries) = manifest else {
        panic!("manifest_cv liefert immer eine Map")
    };
    entries.push((
        Cv::Text("external_citations".to_string()),
        Cv::Array(roots_hex.iter().map(|r| Cv::Text(r.clone())).collect()),
    ));
    Cv::Map(entries)
}

fn residue_segment(entries: &[(&str, &str)]) -> Cv {
    Cv::map(vec![(
        "residues",
        Cv::Array(
            entries
                .iter()
                .map(|(k, d)| {
                    Cv::map(vec![
                        ("kind", Cv::Text((*k).into())),
                        ("detail", Cv::Text((*d).into())),
                    ])
                })
                .collect(),
        ),
    )])
}

/// R1 — minimaler Inspect-Container: MANIFEST, SEGTAB, CANON_DESC +
/// leeres-aber-valides CL-Segment.
pub fn build_r1() -> Sealed {
    let m = manifest_cv(
        "R1 minimal inspect",
        "inspection",
        "PL0",
        false,
        0,
        &[],
        &["read_segment"],
        "cc0",
    );
    let cl = Cv::map(vec![
        ("cubes", Cv::Array(vec![])),
        ("constraints", Cv::Array(vec![])),
    ]);
    seal_canonical(
        "inspection",
        &["inspection"],
        &[
            seg(KIND_MANIFEST, &m),
            canon_desc_segment(),
            seg(KIND_CL_SUBSTRATE, &cl),
        ],
    )
    .unwrap()
}

/// Deterministische CSU-/EP-Kennungen ueber den Motor-Port (nexus-core):
/// dieselbe kanonische Payload-Klasse wie ein echter LocalCorpus-Lauf.
fn csa_content() -> (String, Cv, Cv) {
    let mut payload_map = std::collections::BTreeMap::new();
    payload_map.insert(
        "titel".to_string(),
        cce_core::value::CanonValue::Text("Referenz".to_string()),
    );
    payload_map.insert(
        "inhalt".to_string(),
        cce_core::value::CanonValue::Text("Text".to_string()),
    );
    let csu = nexus_core::objects::Csu {
        uid: String::new(),
        kind: "doc".to_string(),
        payload: cce_core::value::CanonValue::Map(payload_map),
        schema: "kv_lines".to_string(),
        quality: (900, 900, 900),
        provenance: "corpus://k1/doc1".to_string(),
        license: "cc-by-4.0".to_string(),
        source_hash: cce_core::signature::sha256(b"corpus://k1/doc1"),
        residues: vec![],
        domain_facet: "docs".to_string(),
    };
    let uid = format!("csu:{}", csu.canonical_class().0.to_hex());
    let nsb = Cv::map(vec![
        ("bundle_id", Cv::Text("nsb:local_corpus".into())),
        (
            "source_horizon",
            Cv::Text("hs:local_corpus+official_api+git".into()),
        ),
        ("csu_uids", Cv::Array(vec![Cv::Text(uid.clone())])),
        ("evidence_for", Cv::Array(vec![Cv::Text(uid.clone())])),
    ]);
    let evidence = Cv::map(vec![(
        "packs",
        Cv::Array(vec![Cv::map(vec![
            ("evidence_id", Cv::Text(format!("ep:{uid}"))),
            ("record_id", Cv::Text(uid.clone())),
            ("locator", Cv::Text("corpus://k1/doc1".into())),
            ("license", Cv::Text("cc-by-4.0".into())),
            ("attribution", Cv::Text("Korpus k1, CC BY 4.0".into())),
        ])]),
    )]);
    (uid, nsb, evidence)
}

/// R2 — LocalCorpus-Container (source-Profil): CSU/EP → NSB → PHC-Ref.
pub fn build_r2() -> Sealed {
    let (_uid, nsb, evidence) = csa_content();
    let m = manifest_cv(
        "R2 local corpus",
        "source",
        "PL1",
        false,
        0,
        &[],
        &["read_segment", "inspect_evidence"],
        "cc-by-4.0",
    );
    seal_canonical(
        "source",
        &["source"],
        &[
            seg(KIND_MANIFEST, &m),
            canon_desc_segment(),
            seg(KIND_CSA_NSB, &nsb),
            seg(KIND_EVIDENCE, &evidence),
        ],
    )
    .unwrap()
}

/// HBM-Inhalt ueber den Motor-Port (cce-hbm): Facetten aus dem
/// Referenzkorpus, deterministisch.
fn hbm_content() -> Cv {
    let facets = cce_hbm::facet::extract_facets(
        "hbm:r3",
        &[
            "problem: Serverausfall gefaehrdet Betrieb",
            "mechanismus: Redundanz senkt Ausfallrisiko",
        ],
    );
    Cv::map(vec![
        (
            "facets",
            Cv::Array(
                facets
                    .iter()
                    .map(|f| {
                        Cv::map(vec![
                            ("facet_type", Cv::Text(f.facet_type.clone())),
                            ("scope", Cv::Text(f.scope.clone())),
                        ])
                    })
                    .collect(),
            ),
        ),
        ("skeleton", Cv::Text("jt:chordal".into())),
        ("candidates", Cv::Array(vec![Cv::Text("cand:c1".into())])),
        (
            "crystals",
            Cv::Array(vec![Cv::Text("crystal:blueprint-1".into())]),
        ),
    ])
}

/// R3 — HBM-Blueprint-Container (hbm-Profil).
pub fn build_r3() -> Sealed {
    let m = manifest_cv(
        "R3 hbm blueprint",
        "hbm",
        "PL1",
        false,
        0,
        &[],
        &["read_segment"],
        "cc0",
    );
    let (_, _, evidence) = csa_content();
    seal_canonical(
        "hbm",
        &["hbm"],
        &[
            seg(KIND_MANIFEST, &m),
            canon_desc_segment(),
            seg(KIND_HBM, &hbm_content()),
            seg(KIND_EVIDENCE, &evidence),
        ],
    )
    .unwrap()
}

/// PHC-/Workcell-Inhalt ueber den Motor-Port (cce-phc via loom-project).
fn workcell_segments() -> Vec<Segment> {
    let projection = cce_phc::projection_calc::LocalProjection {
        cell_id: "w:memo".to_string(),
        payload: cce_core::value::CanonValue::Text("drei risiken projektion".to_string()),
        allowed_ops: vec!["draft".to_string()],
        gate_chain: vec!["G1".to_string()],
        export_formats: vec!["markdown".to_string()],
        domain_mode: "document".to_string(),
    };
    let phc = loom_project::phc_segment(&[projection]);
    let gates = loom_gate::gate_reports_segment(&[
        cce_core::gate::GateReport::pass("G1", "scope ok"),
        cce_core::gate::GateReport::pass("G2", "boundary ok"),
    ]);
    let runtime = Cv::map(vec![
        ("allowed_ops", Cv::Array(vec![Cv::Text("draft".into())])),
        (
            "capability_locks",
            Cv::Array(vec![Cv::Text("project_workcell".into())]),
        ),
    ]);
    let cl = Cv::map(vec![
        ("cubes", Cv::Array(vec![Cv::Text("cube:memo".into())])),
        ("constraints", Cv::Array(vec![])),
    ]);
    vec![
        seg(KIND_CL_SUBSTRATE, &cl),
        seg(KIND_PHC, &phc),
        seg(KIND_RUNTIME_PROFILE, &runtime),
        seg(KIND_GATE_REPORTS, &gates),
    ]
}

/// R4 — Workcell-Container.
pub fn build_r4() -> Sealed {
    let m = manifest_cv(
        "R4 workcell",
        "workcell",
        "PL1",
        false,
        0,
        &[],
        &["read_segment", "project_workcell"],
        "cc0",
    );
    let mut segs = vec![seg(KIND_MANIFEST, &m), canon_desc_segment()];
    segs.extend(workcell_segments());
    seal_canonical("workcell", &["workcell"], &segs).unwrap()
}

/// Deterministische Commit-Klasse fuer R5/C4 (Motor-Port cce-core).
pub fn r5_commit_class() -> String {
    let payload = cce_core::value::CanonValue::Text("memo-commit".to_string());
    payload.canonical_class().0.to_hex()
}

fn ledger_segment(closure_proof: bool) -> Cv {
    Cv::map(vec![
        ("commits", Cv::Array(vec![Cv::Text("commit:1".into())])),
        (
            "gate_reports_for",
            Cv::Array(vec![Cv::Text("commit:1".into())]),
        ),
        ("closure_proof", Cv::Bool(closure_proof)),
        (
            "hdag_projection",
            Cv::Text("verify_hdag_projection:pass".into()),
        ),
    ])
}

/// R5 — Replay-Container (runtime-Profil): Ledger + ReplayManifest.
pub fn build_r5() -> Sealed {
    let m = manifest_cv(
        "R5 replay",
        "runtime",
        "PL1",
        false,
        0,
        &[],
        &["read_segment"],
        "cc0",
    );
    let replay = loom_replay::replay_manifest_segment("sha256:rd", 7, &r5_commit_class());
    seal_canonical(
        "runtime",
        &["runtime"],
        &[
            seg(KIND_MANIFEST, &m),
            canon_desc_segment(),
            seg(KIND_LEDGER, &ledger_segment(false)),
            seg(KIND_REPLAY_MANIFEST, &replay),
        ],
    )
    .unwrap()
}

/// R6 — Source-Container mit SourceHorizon-Beleg + EvidencePacks.
pub fn build_r6() -> Sealed {
    let (_uid, nsb, evidence) = csa_content();
    let m = manifest_cv(
        "R6 source horizon",
        "source",
        "PL1",
        false,
        1,
        &["license_attribution_required"],
        &["read_segment", "source_acquisition"],
        "cc-by-4.0",
    );
    let residues = residue_segment(&[(
        "license_attribution_required",
        "Attribution wird transportiert",
    )]);
    seal_canonical(
        "source",
        &["source"],
        &[
            seg(KIND_MANIFEST, &m),
            canon_desc_segment(),
            seg(KIND_CSA_NSB, &nsb),
            seg(KIND_EVIDENCE, &evidence),
            seg(KIND_RESIDUE, &residues),
        ],
    )
    .unwrap()
}

/// R7 — Full-Workbody: Drei-Risiken-Memo (Motor-Port cce-materialize).
pub fn build_r7() -> Sealed {
    let memo = cce_materialize::document::assets::three_risks_memo();
    let doc_meta = Cv::map(vec![
        ("title", Cv::Text(memo.title.clone())),
        ("units", Cv::Uint(memo.units.len() as u64)),
        (
            "covers",
            Cv::Array(memo.covers.iter().map(|c| Cv::Text(c.clone())).collect()),
        ),
        ("class", Cv::Text(memo.canonical_class().to_string())),
    ]);
    let artifact = Cv::map(vec![
        ("artifact_id", Cv::Text("artifact:memo-md".into())),
        (
            "two_digest",
            Cv::map(vec![
                (
                    "content_class",
                    Cv::Text(memo.canonical_class().to_string()),
                ),
                ("byte_digest", Cv::Text("sha256:artifact-bytes".into())),
            ]),
        ),
    ]);
    let (_uid, nsb, evidence) = csa_content();
    let m = manifest_cv(
        "R7 full workbody: drei risiken memo",
        "full",
        "PL2",
        true,
        0,
        &[],
        &["read_segment", "project_workcell", "export_artifact"],
        "cc-by-4.0",
    );
    let mut segs = vec![
        seg(KIND_MANIFEST, &m),
        canon_desc_segment(),
        seg(KIND_LEDGER, &ledger_segment(true)),
        seg(KIND_RESIDUE, &residue_segment(&[])),
        seg(KIND_EVIDENCE, &evidence),
        seg(
            KIND_REPLAY_MANIFEST,
            &loom_replay::replay_manifest_segment("sha256:rd", 7, &r5_commit_class()),
        ),
        seg(KIND_CSA_NSB, &nsb),
        seg(KIND_HBM, &hbm_content()),
        seg(KIND_DOC, &doc_meta),
        seg(KIND_ARTIFACT, &artifact),
    ];
    segs.extend(workcell_segments());
    seal_canonical("full", &["full"], &segs).unwrap()
}

/// R8 (Overlay 05 Teil E) — Deklariert-nicht-aktiviert:
/// PROVIDER_MANIFEST + INFERENCE_PROFILE + TOOL_PROFILE, alles
/// deklarativ, kein Aktivierungsfeld — valid.
pub fn build_r8() -> Sealed {
    let providers = Cv::map(vec![(
        "providers",
        Cv::Array(vec![Cv::map(vec![
            ("provider_id", Cv::Text("local:kernmodell".into())),
            ("provider_class", Cv::Text("local_model".into())),
            ("model_id", Cv::Text("kernmodell".into())),
            ("replay_policy", Cv::Text("strict".into())),
        ])]),
    )]);
    let inference_profile = Cv::map(vec![
        (
            "provider_refs",
            Cv::Array(vec![Cv::Text("local:kernmodell".into())]),
        ),
        ("mode_recommendation", Cv::Text("fully_local".into())),
    ]);
    let tool_profile = Cv::map(vec![(
        "tools",
        Cv::Array(vec![Cv::map(vec![
            ("tool_id", Cv::Text("fsr-1".into())),
            ("tool_class", Cv::Text("fs_read".into())),
            ("lock_ref", Cv::Text("lock:fs_read".into())),
        ])]),
    )]);
    let candidates = Cv::map(vec![(
        "outputs",
        Cv::Array(vec![Cv::map(vec![
            ("candidate_id", Cv::Text("cand:1".into())),
            ("evidence_ref", Cv::Text("iev:1".into())),
        ])]),
    )]);
    let m = manifest_cv(
        "R8 declared not activated",
        "inspection",
        "PL1",
        false,
        0,
        &[],
        &["read_segment", "model_egress", "tool_egress"],
        "cc0",
    );
    seal_canonical(
        "inspection",
        &["inspection"],
        &[
            seg(KIND_MANIFEST, &m),
            canon_desc_segment(),
            seg(KIND_PROVIDER_MANIFEST, &providers),
            seg(KIND_INFERENCE_PROFILE, &inference_profile),
            seg(KIND_TOOL_PROFILE, &tool_profile),
            seg(KIND_CANDIDATE_OUTPUTS, &candidates),
        ],
    )
    .unwrap()
}

/// Ein benannter Referenz-Builder.
pub type ReferenceBuilder = (&'static str, fn() -> Sealed);

pub const REFERENCE_BUILDERS: [ReferenceBuilder; 8] = [
    ("R1", build_r1),
    ("R2", build_r2),
    ("R3", build_r3),
    ("R4", build_r4),
    ("R5", build_r5),
    ("R6", build_r6),
    ("R7", build_r7),
    ("R8", build_r8),
];

// ---------- Vollausbau Block 3: erstes Welt-Crystal (Wikimedia) ----------
//
// Anders als R1-R8 (illustrative Golden Files mit Platzhalterwerten wie
// "sha256:artifact-bytes"): hier sind Quellen-CSU, Attribution, Crystal-
// Klasse und Artefakt-Digest ECHT berechnet — kein Platzhalter. Die
// eingefrorene, echte Wikimedia-Antwort (conformance/fixtures/
// wikimedia_kristall.json) laeuft durch den echten JSON->CSU-Extraktor
// (WikimediaAdapter, nexus-decode::decode_json) UND durch den echten
// Motor (cce-runner) — dieselbe Kette, die auch der Cockpit-Produktpfad
// nutzt, nur ohne GUI.

/// Der Locator, unter dem die eingefrorene Fixture "abgerufen" gilt —
/// exportiert, damit ein unabhaengiger Nachrechner (Zeuge) exakt denselben
/// CSU/Cite-Fussabdruck erzeugt (die Attribution bindet den Locator ein).
pub const WELT_KRISTALL_LOCATOR: &str = "https://de.wikipedia.org/w/api.php?action=query&prop=extracts&exintro=1&explaintext=1&titles=Kristall&format=json&redirects=1";

/// Baut die reale DocCrystal, deren Quellenzelle (Einheit `d1`) der ECHTE
/// Wikimedia-Auszug ist — keine erfundene Beispielzelle. `extract_text`
/// ist bereits zeilenkollabiert (parse_markdown ist zeilenbasiert, s.
/// document/parse.rs; ein eingebettetes '\n' wuerde beim Reanalyze
/// verloren gehen — das waere ein echter Bug, kein akzeptables Detail).
pub fn kristall_memo_from_wikimedia(
    extract_text: &str,
    attribution: &str,
) -> cce_materialize::document::DocCrystal {
    use cce_materialize::document::{DocCrystal, DocUnit, UnitType};
    DocCrystal {
        title: "Kristall — Quellenbeleg aus Wikimedia".to_string(),
        units: vec![
            DocUnit::new(
                "s1",
                UnitType::Section,
                "Quelle: Wikipedia-Artikel \u{201e}Kristall\u{201c} (CC BY-SA 4.0)",
            ),
            DocUnit::new("d1", UnitType::Definition, extract_text).with_seam("refers", "s1"),
            DocUnit::new("a1", UnitType::Support, attribution).with_seam("supports", "d1"),
        ],
        covers: vec!["Kristall".to_string()],
        required_sections: vec!["Quelle".to_string()],
        no_score_fields: true,
        ordering: "neutral".to_string(),
    }
}

/// Der Milestone-Builder: JSON->CSU (echter Adapter) -> DocCrystal ->
/// echter Motor-Lauf (cce-runner) -> versiegeltes .loom mit ECHTEN
/// Digests + ECHTER CSA-Evidence (Attribution transportiert, PROD-INV-16).
pub fn build_welt_kristall_wikimedia() -> Sealed {
    use cce_core::replay::RunDescriptor;
    use cce_core::signature::sha256;
    use cce_core::value::CanonValue;
    use cce_runner::runner::Run;
    use nexus_adapter::port::SourceAdapter;
    use nexus_adapter_wikimedia::WikimediaAdapter;
    use nexus_core::objects::RawObservation;
    use nexus_evidence::build_pack;

    let fixture = include_bytes!("../../../conformance/fixtures/wikimedia_kristall.json");
    let raw = RawObservation {
        locator: WELT_KRISTALL_LOCATOR.to_string(),
        bytes: fixture.to_vec(),
        fetched_via: "snapshot-frozen".to_string(),
        snapshot_id: "snap-wikimedia-frozen".to_string(),
    };

    // -- Echter Adapterpfad: extract -> normalize -> validate -> cite --
    let adapter = WikimediaAdapter;
    let records = adapter
        .extract(&raw)
        .expect("echte, eingefrorene Wikimedia-JSON-Antwort muss extrahieren");
    let csu = &adapter.normalize(&records[0])[0];
    assert!(
        adapter.validate(csu).is_pass(),
        "CSU aus der echten Fixture muss wohlgeformt sein"
    );
    let attribution = adapter.cite(csu).remove(0);
    let extract_text = match csu.payload.get("extract") {
        Some(CanonValue::Text(t)) => t.replace('\n', " "),
        other => panic!("extract fehlt/kein Text in der echten CSU: {other:?}"),
    };

    // -- Echte DocCrystal + echter Motor-Lauf (kein Platzhalter-Digest) --
    let crystal = kristall_memo_from_wikimedia(&extract_text, &attribution);
    let rd = RunDescriptor::new(sha256(b"welt-crystal-wikimedia"), "document", 7);
    let mut run = Run::submit(crystal.clone(), rd.clone()).expect("Motor-Submit");
    run.run_to_end(None).expect("Motor-Lauf");
    let artifact = run
        .artifact
        .as_ref()
        .expect("Crystal muss real materialisieren (Gates gruen)");
    let content_class = crystal.canonical_class().0;
    let byte_digest = artifact.byte_digest();

    // -- Echte CSA-Evidence (nicht csa_content()-Platzhalter) --
    let ep = build_pack(
        csu,
        &raw,
        &["decode_json", "extract", "normalize"],
        Some(&attribution),
    );
    let nsb = Cv::map(vec![
        ("bundle_id", Cv::Text("nsb:welt-kristall-wikimedia".into())),
        ("source_horizon", Cv::Text("hs:official_api".into())),
        ("csu_uids", Cv::Array(vec![Cv::Text(csu.uid.clone())])),
        ("evidence_for", Cv::Array(vec![Cv::Text(csu.uid.clone())])),
    ]);
    let evidence = Cv::map(vec![(
        "packs",
        Cv::Array(vec![Cv::map(vec![
            ("evidence_id", Cv::Text(ep.evidence_id.clone())),
            ("record_id", Cv::Text(ep.record_id.clone())),
            ("locator", Cv::Text(ep.locator.clone())),
            ("license", Cv::Text(ep.license.clone())),
            (
                "attribution",
                Cv::Text(ep.attribution.clone().unwrap_or_default()),
            ),
            ("raw_hash", Cv::Text(ep.raw_hash.to_hex())),
        ])]),
    )]);
    let doc_meta = Cv::map(vec![
        ("title", Cv::Text(crystal.title.clone())),
        ("units", Cv::Uint(crystal.units.len() as u64)),
        (
            "covers",
            Cv::Array(crystal.covers.iter().map(|c| Cv::Text(c.clone())).collect()),
        ),
        ("class", Cv::Text(content_class.to_hex())),
    ]);
    let artifact_cv = Cv::map(vec![
        (
            "artifact_id",
            Cv::Text("artifact:kristall-wikimedia-md".into()),
        ),
        (
            "two_digest",
            Cv::map(vec![
                ("content_class", Cv::Text(content_class.to_hex())),
                ("byte_digest", Cv::Text(byte_digest.to_hex())),
            ]),
        ),
    ]);
    let m = manifest_cv(
        "Kristall — Quellenbeleg aus Wikimedia (Vollausbau Block 3)",
        "full",
        "PL2",
        true,
        0,
        &[],
        &[
            "read_segment",
            "project_workcell",
            "export_artifact",
            "source_acquisition",
            "inspect_evidence",
        ],
        nexus_adapter_wikimedia::WIKIMEDIA_LICENSE,
    );
    let mut segs = vec![
        seg(KIND_MANIFEST, &m),
        canon_desc_segment(),
        seg(KIND_LEDGER, &ledger_segment(true)),
        seg(KIND_RESIDUE, &residue_segment(&[])),
        seg(KIND_EVIDENCE, &evidence),
        seg(
            KIND_REPLAY_MANIFEST,
            &loom_replay::replay_manifest_segment(
                &rd.crystal_digest.to_hex(),
                rd.seed,
                &content_class.to_hex(),
            ),
        ),
        seg(KIND_CSA_NSB, &nsb),
        seg(KIND_HBM, &hbm_content_kristall()),
        seg(KIND_DOC, &doc_meta),
        seg(KIND_ARTIFACT, &artifact_cv),
        // X1a (Oekosystem-Karte §2/E1): der Container traegt die
        // materialisierten Bytes SELBST (nicht nur ihren Digest) — CAS_BLOB
        // ist per Definition content-adressiert (Frame-Multihash), das
        // ARTIFACT-Segment darueber verweist per byte_digest darauf
        // (loom_mount::extract_artifact prueft beide Werte gegeneinander).
        seg(KIND_CAS_BLOB, &Cv::Bytes(artifact.bytes.clone())),
    ];
    // Profil "full" verlangt zusaetzlich CL_SUBSTRATE/PHC/RUNTIME_PROFILE/
    // GATE_REPORTS (wie R7) — dieselben Workcell-Segmente, kein neuer Pfad.
    segs.extend(workcell_segments());
    seal_canonical("full", &["full"], &segs).unwrap()
}

/// HBM-Facetten (S1-A2-Vokabular) ECHT aus dem Wikimedia-Auszug abgeleitet
/// — anders als `hbm_content()` (R3-Platzhalter, andere Domaene) sind das
/// die tatsaechlichen, im Artikel „Kristall" belegten Fachbegriffe.
fn hbm_content_kristall() -> Cv {
    let facets = cce_hbm::facet::extract_facets(
        "hbm:welt-kristall-wikimedia",
        &[
            "entity: Kristall — Festkoerper mit regelmaessig angeordneten Bausteinen",
            "constraint: Fernordnung durch Translationssymmetrie (Kristallstruktur)",
        ],
    );
    Cv::map(vec![
        (
            "facets",
            Cv::Array(
                facets
                    .iter()
                    .map(|f| {
                        Cv::map(vec![
                            ("facet_type", Cv::Text(f.facet_type.clone())),
                            ("scope", Cv::Text(f.scope.clone())),
                        ])
                    })
                    .collect(),
            ),
        ),
        ("skeleton", Cv::Text("jt:chordal".into())),
        (
            "candidates",
            Cv::Array(vec![Cv::Text("cand:kristall-wikimedia-1".into())]),
        ),
        (
            "crystals",
            Cv::Array(vec![Cv::Text("crystal:welt-kristall-wikimedia".into())]),
        ),
    ])
}

// ---------- Etappe X1(b): SCALE-2-Vollmaterialisierung ----------
//
// Baut auf X1(a) auf: die Mappe buendelt ihre Memo-.looms PHYSISCH
// (je ein CAS_BLOB pro Kind, jedes Kind selbst ein vollstaendig
// versiegelter, unabhaengig gueltiger Arbeitskoerper) statt sie nur
// per content_class zu REFERENZIEREN (der bisherige DocFolder-Stand,
// unveraendert — kein Eingriff in cce-materialize::scale2_folder).

fn hex34(b: &[u8; 34]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

/// Zweiter, inhaltlich echter Kurz-Hinweis (nicht die Drei-Risiken-
/// Referenz dupliziert) — zeigt, dass die Mappe heterogene Kinder
/// buendelt, keine Klone.
fn scale2_second_memo() -> cce_materialize::document::DocCrystal {
    use cce_materialize::document::{DocUnit, UnitType};
    cce_materialize::document::DocCrystal {
        title: "Kurzhinweis: Nahtstabilitaet".to_string(),
        units: vec![
            DocUnit::new("s1", UnitType::Section, "Hinweis"),
            DocUnit::new(
                "d1",
                UnitType::Definition,
                "Nahtstabilitaet bezeichnet die Eigenschaft, dass eine Referenz \
                 zwischen zwei Einheiten auch nach Reanalyse erhalten bleibt.",
            )
            .with_seam("refers", "s1"),
        ],
        covers: vec!["Nahtstabilitaet".to_string()],
        required_sections: vec!["Hinweis".to_string()],
        no_score_fields: true,
        ordering: "neutral".to_string(),
    }
}

/// Versiegelt EIN Dokument-Crystal als eigenstaendigen, unabhaengig
/// gueltigen Arbeitskoerper (Profil "workcell": MANIFEST/CANON_DESC +
/// CL_SUBSTRATE/PHC/RUNTIME_PROFILE/GATE_REPORTS, s. loom-verify
/// required_kinds) — plus DOC/ARTIFACT/CAS_BLOB fuer den echten Inhalt.
/// Real materialisiert (cce-runner), kein Platzhalter-Digest.
fn seal_document_workbody(
    crystal: &cce_materialize::document::DocCrystal,
    artifact_id: &str,
    seed_label: &str,
) -> Sealed {
    use cce_core::replay::RunDescriptor;
    use cce_core::signature::sha256;
    use cce_runner::runner::Run;

    let rd = RunDescriptor::new(sha256(seed_label.as_bytes()), "document", 7);
    let mut run = Run::submit(crystal.clone(), rd).expect("Motor-Submit");
    run.run_to_end(None).expect("Motor-Lauf");
    let artifact = run
        .artifact
        .as_ref()
        .expect("Crystal muss real materialisieren (Gates gruen)");
    let content_class = crystal.canonical_class().0;
    let byte_digest = artifact.byte_digest();

    let doc_meta = Cv::map(vec![
        ("title", Cv::Text(crystal.title.clone())),
        ("units", Cv::Uint(crystal.units.len() as u64)),
        ("class", Cv::Text(content_class.to_hex())),
    ]);
    let artifact_cv = Cv::map(vec![
        ("artifact_id", Cv::Text(artifact_id.to_string())),
        (
            "two_digest",
            Cv::map(vec![
                ("content_class", Cv::Text(content_class.to_hex())),
                ("byte_digest", Cv::Text(byte_digest.to_hex())),
            ]),
        ),
    ]);
    let m = manifest_cv(
        &format!("SCALE-2-Kind: {}", crystal.title),
        "workcell",
        "PL2",
        false, // kein LEDGER-Segment hier -> keine Abschluss-Behauptung
        0,
        &[],
        &["read_segment", "project_workcell", "export_artifact"],
        "cc0",
    );
    let mut segs = vec![
        seg(KIND_MANIFEST, &m),
        canon_desc_segment(),
        seg(KIND_DOC, &doc_meta),
        seg(KIND_ARTIFACT, &artifact_cv),
        seg(KIND_CAS_BLOB, &Cv::Bytes(artifact.bytes.clone())),
    ];
    segs.extend(workcell_segments());
    seal_canonical("workcell", &["workcell"], &segs).unwrap()
}

/// Der Milestone-Builder X1(b): die SCALE-2-Dokumentenmappe, PHYSISCH
/// gebuendelt — Mappen-Extraktion liefert n (hier 2) eigenstaendig
/// gueltige Kind-Arbeitskoerper (loom_mount::all_cas_blobs + je ein
/// loom_verify::verify ⇒ Valid).
pub fn build_scale2_folder_full() -> Sealed {
    use cce_materialize::document::assets::three_risks_memo;
    use cce_materialize::scale2_folder::{folder_seams_valid, DocFolder};

    let memo_a = three_risks_memo();
    let memo_b = scale2_second_memo();

    let folder = DocFolder::from_memos(
        "Projektmappe Vollausbau X1",
        &[("memo_a", &memo_a), ("memo_b", &memo_b)],
        &[("memo_a", "precedes", "memo_b")],
    );
    assert!(
        folder_seams_valid(&folder),
        "Mappe-Naehte muessen konsistent sein"
    );

    let sealed_a = seal_document_workbody(&memo_a, "artifact:scale2-memo-a", "scale2-child-a");
    let sealed_b = seal_document_workbody(&memo_b, "artifact:scale2-memo-b", "scale2-child-b");

    let folder_meta = Cv::map(vec![
        ("title", Cv::Text(folder.title.clone())),
        ("scale", Cv::Uint(2)),
        ("class", Cv::Text(folder.canonical_class().0.to_hex())),
        (
            "entries",
            Cv::Array(vec![
                Cv::map(vec![
                    ("id", Cv::Text("memo_a".into())),
                    (
                        "content_class",
                        Cv::Text(memo_a.canonical_class().0.to_hex()),
                    ),
                    ("child_core_root", Cv::Text(hex34(&sealed_a.core_root))),
                ]),
                Cv::map(vec![
                    ("id", Cv::Text("memo_b".into())),
                    (
                        "content_class",
                        Cv::Text(memo_b.canonical_class().0.to_hex()),
                    ),
                    ("child_core_root", Cv::Text(hex34(&sealed_b.core_root))),
                ]),
            ]),
        ),
    ]);

    let m = manifest_cv(
        "SCALE-2 Dokumentenmappe — physisch gebuendelt (Oekosystem-Karte X1b)",
        "workcell",
        "PL2",
        false, // kein LEDGER-Segment hier -> keine Abschluss-Behauptung
        0,
        &[],
        &["read_segment", "project_workcell"],
        "cc0",
    );
    let mut segs = vec![
        seg(KIND_MANIFEST, &m),
        canon_desc_segment(),
        seg(KIND_DOC, &folder_meta),
        seg(KIND_CAS_BLOB, &Cv::Bytes(sealed_a.bytes.clone())),
        seg(KIND_CAS_BLOB, &Cv::Bytes(sealed_b.bytes.clone())),
    ];
    segs.extend(workcell_segments());
    seal_canonical("workcell", &["workcell"], &segs).unwrap()
}

// ---------- Etappe X2/E2 (S-E2a Teil I): die cites-Naht ----------
//
// Das erste Memo, das sich real — ueber Workbody-Grenzen hinweg — auf
// einen ANDEREN zertifizierten Container stuetzt: den echten
// Welt-Kristall (Block 3). Zwei cites-Eintraege in EINEM Memo decken
// beide Karten-Zeugen ab: R-CIT-1 (unit `d1`, `supports`, kein
// target_unit_ref) und R-CIT-2 (unit `d2`, `derives`, target_unit_ref
// auf die Definitionseinheit `d1` DES Welt-Kristalls — Namensgleichheit
// mit der lokalen Einheit `d1` hier ist Zufall zweier unabhaengiger
// Domaenen, keine Verwechslung: das Feld referenziert IMMER
// `target_core_root` + `target_unit_ref` gemeinsam).

/// Das zitierende Memo (rein lokale Struktur — die Naht nach aussen
/// lebt im CL_SUBSTRATE, nicht in `DocUnit.seams`, s. I.4).
pub fn citing_memo_welt_kristall() -> cce_materialize::document::DocCrystal {
    use cce_materialize::document::{DocUnit, UnitType};
    cce_materialize::document::DocCrystal {
        title: "Kurzmemo: Kristallstruktur, gestuetzt auf den Welt-Kristall".to_string(),
        units: vec![
            DocUnit::new("s1", UnitType::Section, "Bezug"),
            DocUnit::new(
                "d1",
                UnitType::Definition,
                "Diese Kurzeinschaetzung stuetzt sich auf die im Welt-Kristall belegte Definition.",
            )
            .with_seam("refers", "s1"),
            DocUnit::new(
                "d2",
                UnitType::Definition,
                "Diese Ableitung bezieht sich gezielt auf die Definitionseinheit des Welt-Kristalls.",
            )
            .with_seam("refers", "s1"),
        ],
        covers: vec!["Kristallstruktur".to_string()],
        required_sections: vec!["Bezug".to_string()],
        no_score_fields: true,
        ordering: "neutral".to_string(),
    }
}

/// Baut UND versiegelt das zitierende Memo gegen einen bereits
/// versiegelten Ziel-Container (typischerweise der echte Welt-Kristall).
/// `target_unit_ref` fuer R-CIT-2 muss eine Einheit sein, die im Ziel
/// TATSAECHLICH existiert (hier: "d1", s. `kristall_memo_from_wikimedia`).
pub fn seal_citing_memo_welt_kristall(target_core_root_hex: &str) -> Sealed {
    use cce_core::replay::RunDescriptor;
    use cce_core::signature::sha256;
    use cce_runner::runner::Run;
    use loom_cites::{cites_field, external_citations_hex, CiteEntry, CiteKind};

    let crystal = citing_memo_welt_kristall();
    let rd = RunDescriptor::new(sha256(b"citing-memo-welt-kristall"), "document", 7);
    let mut run = Run::submit(crystal.clone(), rd).expect("Motor-Submit");
    run.run_to_end(None).expect("Motor-Lauf");
    let artifact = run
        .artifact
        .as_ref()
        .expect("Crystal muss real materialisieren (Gates gruen)");
    let content_class = crystal.canonical_class().0;
    let byte_digest = artifact.byte_digest();

    let cites = vec![
        // R-CIT-1: einfacher Stuetzungsanspruch, kein target_unit_ref.
        CiteEntry {
            unit_id: "d1".to_string(),
            target_core_root_hex: target_core_root_hex.to_string(),
            target_unit_ref: None,
            cite_kind: CiteKind::Supports,
        },
        // R-CIT-2: Ableitung MIT target_unit_ref auf die Definitionseinheit
        // "d1" des Welt-Kristalls (nicht zu verwechseln mit der lokalen "d2").
        CiteEntry {
            unit_id: "d2".to_string(),
            target_core_root_hex: target_core_root_hex.to_string(),
            target_unit_ref: Some("d1".to_string()),
            cite_kind: CiteKind::Derives,
        },
    ];
    let external_citations = external_citations_hex(&cites);

    let doc_meta = Cv::map(vec![
        ("title", Cv::Text(crystal.title.clone())),
        ("units", Cv::Uint(crystal.units.len() as u64)),
        ("class", Cv::Text(content_class.to_hex())),
    ]);
    let artifact_cv = Cv::map(vec![
        ("artifact_id", Cv::Text("artifact:citing-memo-md".into())),
        (
            "two_digest",
            Cv::map(vec![
                ("content_class", Cv::Text(content_class.to_hex())),
                ("byte_digest", Cv::Text(byte_digest.to_hex())),
            ]),
        ),
    ]);
    let cl = Cv::map(vec![
        (
            "cubes",
            Cv::Array(vec![Cv::Text("cube:citing-memo".into())]),
        ),
        ("constraints", Cv::Array(vec![])),
        ("cites", cites_field(&cites)),
    ]);
    let mut m = manifest_cv(
        "Kurzmemo: Kristallstruktur, gestuetzt auf den Welt-Kristall (X2/E2)",
        "workcell",
        "PL2",
        false, // kein LEDGER-Segment hier -> keine Abschluss-Behauptung
        0,
        &[],
        &["read_segment", "project_workcell", "export_artifact"],
        "cc0",
    );
    m = manifest_declare_external_citations(m, &external_citations);

    let phc = loom_project::phc_segment(&[cce_phc::projection_calc::LocalProjection {
        cell_id: "w:citing-memo".to_string(),
        payload: cce_core::value::CanonValue::Text("citing memo projektion".to_string()),
        allowed_ops: vec!["draft".to_string()],
        gate_chain: vec!["G1".to_string()],
        export_formats: vec!["markdown".to_string()],
        domain_mode: "document".to_string(),
    }]);
    let gates = loom_gate::gate_reports_segment(&[
        cce_core::gate::GateReport::pass("G1", "scope ok"),
        cce_core::gate::GateReport::pass("G2", "boundary ok"),
    ]);
    let runtime = Cv::map(vec![
        ("allowed_ops", Cv::Array(vec![Cv::Text("draft".into())])),
        (
            "capability_locks",
            Cv::Array(vec![Cv::Text("project_workcell".into())]),
        ),
    ]);

    let segs = vec![
        seg(KIND_MANIFEST, &m),
        canon_desc_segment(),
        seg(KIND_DOC, &doc_meta),
        seg(KIND_ARTIFACT, &artifact_cv),
        seg(KIND_CAS_BLOB, &Cv::Bytes(artifact.bytes.clone())),
        seg(KIND_CL_SUBSTRATE, &cl),
        seg(KIND_PHC, &phc),
        seg(KIND_RUNTIME_PROFILE, &runtime),
        seg(KIND_GATE_REPORTS, &gates),
    ];
    seal_canonical("workcell", &["workcell"], &segs).unwrap()
}

// ---------- Etappe X2/E2 (I.6): SCALE-3 „Projektraum" ----------
//
// Zellen: 2 SCALE-2-Mappen + 1 Quellen-Workbody (Welt-Kristall) + 1
// Blueprint-Kristall. Mappe B buendelt das zitierende Memo als eines
// ihrer Kinder — die SCALE-3-"cites"-Naht Mappe-B->Quelle ist damit
// eine ECHTE, ueberpruefbare Tatsache (nicht nur ein Label): das
// gebuendelte Kind zitiert den Welt-Kristall wirklich, CitationGate
// darauf reproduziert das.

/// Mappe B: buendelt das zitierende Memo (statt eines neutralen
/// Kurzhinweises wie in Mappe A) — Grundlage der SCALE-3-cites-Naht.
pub fn build_scale2_folder_b(welt_root_hex: &str) -> Sealed {
    use cce_materialize::scale2_folder::{folder_seams_valid, DocFolder};

    let citing_crystal = citing_memo_welt_kristall();
    let memo_b = scale2_second_memo();

    let folder = DocFolder::from_memos(
        "Projektmappe B (mit Zitat)",
        &[("citing", &citing_crystal), ("memo_b", &memo_b)],
        &[("citing", "precedes", "memo_b")],
    );
    assert!(folder_seams_valid(&folder), "Mappe-B-Naehte konsistent");

    let sealed_citing = seal_citing_memo_welt_kristall(welt_root_hex);
    let sealed_b = seal_document_workbody(&memo_b, "artifact:scale2b-memo-b", "scale2b-child-b");

    let folder_meta = Cv::map(vec![
        ("title", Cv::Text(folder.title.clone())),
        ("scale", Cv::Uint(2)),
        ("class", Cv::Text(folder.canonical_class().0.to_hex())),
        (
            "entries",
            Cv::Array(vec![
                Cv::map(vec![
                    ("id", Cv::Text("citing".into())),
                    (
                        "content_class",
                        Cv::Text(citing_crystal.canonical_class().0.to_hex()),
                    ),
                    ("child_core_root", Cv::Text(hex34(&sealed_citing.core_root))),
                ]),
                Cv::map(vec![
                    ("id", Cv::Text("memo_b".into())),
                    (
                        "content_class",
                        Cv::Text(memo_b.canonical_class().0.to_hex()),
                    ),
                    ("child_core_root", Cv::Text(hex34(&sealed_b.core_root))),
                ]),
            ]),
        ),
    ]);

    let m = manifest_cv(
        "SCALE-2 Dokumentenmappe B — mit Zitat auf den Welt-Kristall (X2/E2)",
        "workcell",
        "PL2",
        false, // kein LEDGER-Segment hier -> keine Abschluss-Behauptung
        0,
        &[],
        &["read_segment", "project_workcell"],
        "cc0",
    );
    let mut segs = vec![
        seg(KIND_MANIFEST, &m),
        canon_desc_segment(),
        seg(KIND_DOC, &folder_meta),
        seg(KIND_CAS_BLOB, &Cv::Bytes(sealed_citing.bytes.clone())),
        seg(KIND_CAS_BLOB, &Cv::Bytes(sealed_b.bytes.clone())),
    ];
    segs.extend(workcell_segments());
    seal_canonical("workcell", &["workcell"], &segs).unwrap()
}

/// Der Blueprint-Kristall-Platzhalter fuer SCALE-3 (X2/E2): strukturell
/// derselbe "hbm"-Profil-Aufbau wie R3 (echte Facetten ueber den
/// Motor-Port cce-hbm, kein erfundener Text). Die ECHTE, aus dem
/// Eigenkorpus zertifizierte Blueprint-Kette liefert erst Etappe X2/E3
/// (Karte §2/E3) — hier zaehlt nur die STRUKTURELLE Zellen-Eigenschaft
/// "ist ein Blueprint-Kristall", nicht die materielle Reife.
pub fn build_blueprint_reference_cube() -> Sealed {
    build_r3()
}

// ---------- Etappe X2/E3 (Karte §2/E3): HBM produktiv auf Eigenkorpus ----------
//
// Der Eigenkorpus ist gewoehnlicher Ingest (Dokument 14 §I Kopfsatz):
// die 213 Familien-Referenzprofile (DocProfile.rule — die Naht-Regel
// jeder Domaene) + ihre Katalog-Kern-Gates/-Residuen (CATALOG) werden
// als `typ: inhalt`-Zeilen (loom_hbm-Facet-Vokabular) durch die
// UNVERAENDERTE cce-hbm-Pipeline geschickt. Jede Content-Zeile traegt
// die Domaenen-Kennung, damit KEINE zwei Facets denselben Scope tragen
// (ExclusionGate: redundante Struktur waere sonst ein Hold).

fn domain_rule_fact(id: &str, rule: &cce_materialize::family_a::DomainRule) -> String {
    use cce_materialize::family_a::DomainRule;
    match rule {
        DomainRule::Relation { seam } => format!("invariant: {id}|regel=Relation|naht={seam}"),
        DomainRule::AcyclicRelation { seam } => {
            format!("invariant: {id}|regel=AcyclicRelation|naht={seam}")
        }
        DomainRule::ChainedRelation { seam } => {
            format!("invariant: {id}|regel=ChainedRelation|naht={seam}")
        }
        DomainRule::UniqueSubjects => format!("invariant: {id}|regel=UniqueSubjects"),
        DomainRule::OrderedSteps { seam } => {
            format!("invariant: {id}|regel=OrderedSteps|naht={seam}")
        }
        DomainRule::StructuralPresence { markers } => format!(
            "invariant: {id}|regel=StructuralPresence|marker={}",
            markers.join(",")
        ),
    }
}

/// Alle 213 Familien-Referenzprofile (16 Familien-Module) als
/// `(id, DomainRule)`-Paare — die tatsaechliche, im Code lebende
/// Naht-Regel jeder Domaene (S1_DOMAENENKATALOG), keine erfundene
/// Beispielzeile.
fn all_domain_rules() -> Vec<(&'static str, cce_materialize::family_a::DomainRule)> {
    macro_rules! collect {
        ($($m:path),+ $(,)?) => {{
            let mut v = Vec::new();
            $(v.extend($m().into_iter().map(|p| (p.id, p.rule)));)+
            v
        }};
    }
    collect!(
        cce_materialize::family_a_domains::all_profiles,
        cce_materialize::family_b_domains::all_profiles,
        cce_materialize::family_c_domains::all_profiles,
        cce_materialize::family_d_domains::all_profiles,
        cce_materialize::family_e_domains::all_profiles,
        cce_materialize::family_f_domains::all_profiles,
        cce_materialize::family_g_domains::all_profiles,
        cce_materialize::family_h_domains::all_profiles,
        cce_materialize::family_i_domains::all_profiles,
        cce_materialize::family_j_domains::all_profiles,
        cce_materialize::family_k_domains::all_profiles,
        cce_materialize::family_l_domains::all_profiles,
        cce_materialize::family_m_domains::all_profiles,
        cce_materialize::family_n_domains::all_profiles,
        cce_materialize::family_o_domains::all_profiles,
        cce_materialize::family_p_domains::all_profiles,
    )
}

/// Der reale Eigenkorpus: 213 `invariant`-Zeilen (Naht-Regel je Domaene,
/// aus `all_domain_rules()`) + je 213 `gate`-/`constraint`-Zeilen aus dem
/// Katalog (Kern-Gate/-Residuum je Domaene, `CATALOG`) — durchgehend
/// domaenen-eindeutig, keine Platzhalter.
pub fn eigenkorpus_ingest_lines() -> Vec<String> {
    let mut lines: Vec<String> = all_domain_rules()
        .iter()
        .map(|(id, rule)| domain_rule_fact(id, rule))
        .collect();
    for e in cce_materialize::catalog::CATALOG.iter() {
        lines.push(format!("gate: {}|{}", e.id, e.core_gate));
        lines.push(format!("constraint: {}|{}", e.id, e.core_residue));
    }
    lines
}

/// Der HBM-Lauf ueber den Eigenkorpus: expansion_budget deckt die volle
/// Vollprojektion C6 ab. theta_d=0 (statt =1 wie beim kleinen
/// HBM-Katalog-Demo-Korpus): Score_D waechst quadratisch mit der
/// Kosten-Strafe ueber die Facet-Anzahl (score.rs), bei ~600+ Facetten
/// in C6 unterschreitet der Rohscore selbst den Schwellwert 1 real
/// (kein Bug, dieselbe Formel) — θ_D bleibt reine VORAUSWAHL (nie
/// Abnahme, s. score.rs-Kommentar), 0 laesst alle Kandidaten zu den
/// Gates durch, OHNE die Gate-Entscheidung selbst zu veraendern.
pub fn build_eigenkorpus_mining_input() -> cce_hbm::pipeline::MiningInput {
    let lines = eigenkorpus_ingest_lines();
    let budget = lines.len() + 8;
    cce_hbm::pipeline::MiningInput {
        corpus_id: "eigenkorpus:213-familien".to_string(),
        lines,
        weights: cce_hbm::score::ScoreWeights::default(),
        theta_d: 0,
        expansion_budget: budget,
    }
}

/// Der Blueprint-Kristall aus realer Eigenarbeit: hbm-Profil-Container
/// mit den ECHTEN Facetten/Kandidaten/zertifizierten Klassen des
/// Eigenkorpus-Laufs (kein Platzhaltertext wie `build_r3`/
/// `hbm_content_kristall`). Ersetzt den in X2/E2 strukturellen
/// Blueprint-Platzhalter (R-Agent-9) durch eine materiell reale Zelle.
pub fn build_blueprint_eigenkorpus() -> Sealed {
    let input = build_eigenkorpus_mining_input();
    // Dieselbe Facet-Extraktion, die run_pipeline intern zuerst ausfuehrt
    // (Phase 0-1) — hier separat aufgerufen, weil PipelineOutcome nur die
    // Facet-ANZAHL traegt, nicht die Liste selbst (Kern-API unveraendert).
    let line_refs: Vec<&str> = input.lines.iter().map(String::as_str).collect();
    let facets = cce_hbm::facet::extract_facets(&input.corpus_id, &line_refs);

    let out =
        cce_hbm::pipeline::run_pipeline(&input).expect("Gate_A darf am Eigenkorpus nie halten");
    assert!(
        !out.certified.is_empty(),
        "Eigenkorpus muss mindestens einen Blueprint zertifizieren"
    );
    assert_eq!(
        facets.len(),
        out.facets,
        "Facet-Anzahl muss uebereinstimmen"
    );

    let hbm_cv = Cv::map(vec![
        (
            "facets",
            Cv::Array(
                facets
                    .iter()
                    .map(|f| {
                        Cv::map(vec![
                            ("facet_type", Cv::Text(f.facet_type.clone())),
                            ("scope", Cv::Text(f.scope.clone())),
                        ])
                    })
                    .collect(),
            ),
        ),
        (
            "skeleton",
            Cv::Text(match out.treewidth {
                Some(w) => format!("jt:treewidth={w}"),
                None => "jt:none".to_string(),
            }),
        ),
        (
            "candidates",
            Cv::Array(
                out.ranking
                    .iter()
                    .map(|(id, _)| Cv::Text(id.clone()))
                    .collect(),
            ),
        ),
        (
            "crystals",
            Cv::Array(
                out.certified
                    .iter()
                    .map(|(id, _)| Cv::Text(id.clone()))
                    .collect(),
            ),
        ),
    ]);
    let evidence = Cv::map(vec![(
        "packs",
        Cv::Array(vec![Cv::map(vec![
            ("evidence_id", Cv::Text("ep:eigenkorpus-213-familien".into())),
            ("record_id", Cv::Text(input.corpus_id.clone())),
            (
                "locator",
                Cv::Text(
                    "cce-materialize::catalog + family_*_domains::all_profiles (Eigencode, kein externer Fetch)"
                        .to_string(),
                ),
            ),
            ("license", Cv::Text("SEE-REPO-ROOT".into())),
            (
                "attribution",
                Cv::Text("Eigenkorpus: 213 Domaenen-Referenzprofile + Katalog-Kern-Gates/-Residuen".into()),
            ),
        ])]),
    )]);
    let m = manifest_cv(
        "Blueprint-Kristall aus Eigenkorpus (X2/E3, Naht-Regel-Struktur ueber 213 Familien)",
        "hbm",
        "PL1",
        false,
        0,
        &[],
        &["read_segment", "inspect_evidence"],
        "SEE-REPO-ROOT",
    );
    seal_canonical(
        "hbm",
        &["hbm"],
        &[
            seg(KIND_MANIFEST, &m),
            canon_desc_segment(),
            seg(KIND_HBM, &hbm_cv),
            seg(KIND_EVIDENCE, &evidence),
        ],
    )
    .unwrap()
}

// ---------- Etappe X2/E4a (S-E4a Teil II): CE-1 Tabellen-Zellentyp ----------
//
// R-TBL-1: das Referenz-Memo "Risikomatrix" — eine Table-Einheit (3
// Zeilen) + drei Gegenmassnahmen-Einheiten, je mit einer supports-Naht
// AUF die Tabelle (haelt sie nicht-verwaist UND belegt real, dass Table
// Naehte traegt wie jede andere Einheit, S-E4a II.2).

/// Das Referenz-Memo fuer CE-1: R-TBL-1.
pub fn risikomatrix_memo() -> cce_materialize::document::DocCrystal {
    use cce_materialize::document::{DocUnit, TableCell, UnitType};
    let tabelle = DocUnit::new_table(
        "tbl1",
        &["Risiko", "Wahrscheinlichkeit", "Kosten"],
        vec![
            vec![
                TableCell::Text("Serverausfall".to_string()),
                TableCell::DecFrac { num: 15, scale: 1 },
                TableCell::Int(5000),
            ],
            vec![
                TableCell::Text("Datenverlust".to_string()),
                TableCell::DecFrac { num: 5, scale: 1 },
                TableCell::Int(12000),
            ],
            vec![
                TableCell::Text("Lieferverzug".to_string()),
                TableCell::DecFrac { num: 30, scale: 1 },
                TableCell::Int(2000),
            ],
        ],
    )
    .with_seam("refers", "s1");
    cce_materialize::document::DocCrystal {
        title: "Risikomatrix".to_string(),
        units: vec![
            DocUnit::new("s1", UnitType::Section, "Risikomatrix"),
            tabelle,
            DocUnit::new(
                "c1",
                UnitType::Countermeasure,
                "Failover-Cluster gegen Serverausfall",
            )
            .with_seam("supports", "tbl1"),
            DocUnit::new(
                "c2",
                UnitType::Countermeasure,
                "Taegliche Backups gegen Datenverlust",
            )
            .with_seam("supports", "tbl1"),
            DocUnit::new(
                "c3",
                UnitType::Countermeasure,
                "Zweitlieferant gegen Lieferverzug",
            )
            .with_seam("supports", "tbl1"),
        ],
        covers: vec!["Risikomatrix".to_string()],
        required_sections: vec!["Risikomatrix".to_string()],
        no_score_fields: true,
        ordering: "neutral".to_string(),
    }
}

/// R-TBL-1 versiegelt: voller Motorpfad (cce-runner), "full"-Profil wie
/// R7/der Welt-Kristall — alle sieben DocG-Gates gruen, inkl. der
/// verschaerften DocG-Structure (ragged_table/invalid_cell_type).
pub fn build_risikomatrix_workbody() -> Sealed {
    use cce_core::replay::RunDescriptor;
    use cce_core::signature::sha256;
    use cce_runner::runner::Run;

    let crystal = risikomatrix_memo();
    let rd = RunDescriptor::new(sha256(b"risikomatrix-ce1"), "document", 7);
    let mut run = Run::submit(crystal.clone(), rd).expect("Motor-Submit");
    run.run_to_end(None)
        .expect("Motor-Lauf (alle DocG-Gates gruen)");
    let artifact = run
        .artifact
        .as_ref()
        .expect("Crystal muss real materialisieren (Gates gruen)");
    let content_class = crystal.canonical_class().0;
    let byte_digest = artifact.byte_digest();

    let doc_meta = Cv::map(vec![
        ("title", Cv::Text(crystal.title.clone())),
        ("units", Cv::Uint(crystal.units.len() as u64)),
        ("class", Cv::Text(content_class.to_hex())),
    ]);
    let artifact_cv = Cv::map(vec![
        ("artifact_id", Cv::Text("artifact:risikomatrix-md".into())),
        (
            "two_digest",
            Cv::map(vec![
                ("content_class", Cv::Text(content_class.to_hex())),
                ("byte_digest", Cv::Text(byte_digest.to_hex())),
            ]),
        ),
    ]);
    let m = manifest_cv(
        "Risikomatrix — CE-1 Referenz-Memo (X2/E4a)",
        "workcell",
        "PL2",
        false,
        0,
        &[],
        &["read_segment", "project_workcell", "export_artifact"],
        "cc0",
    );
    let mut segs = vec![
        seg(KIND_MANIFEST, &m),
        canon_desc_segment(),
        seg(KIND_DOC, &doc_meta),
        seg(KIND_ARTIFACT, &artifact_cv),
        seg(KIND_CAS_BLOB, &Cv::Bytes(artifact.bytes.clone())),
    ];
    segs.extend(workcell_segments());
    seal_canonical("workcell", &["workcell"], &segs).unwrap()
}

// ---------- Etappe X2/E4c (Karte §2/E4c): Klassen-Registry ----------
//
// Ein Verzeichnis zertifizierter core_roots als eigener .loom-Katalog-
// Workbody — Grundlage fuer E2-cites und jedes spaetere Teilen. Jeder
// Eintrag wird aus dem ECHTEN, bereits versiegelten Container gelesen
// (container_class/domain_refs aus dessen eigenem MANIFEST, Signaturen
// ueber loom_cli::sign::verify_sig_all) — keine von Hand gepflegte,
// driftanfaellige Zweitquelle.

fn cv_get<'a>(map: &'a Cv, key: &str) -> Option<&'a Cv> {
    if let Cv::Map(entries) = map {
        entries.iter().find_map(|(k, v)| match k {
            Cv::Text(s) if s == key => Some(v),
            _ => None,
        })
    } else {
        None
    }
}

fn catalog_entry_from_sealed(sealed: &Sealed, path: &str) -> loom_cites::ClassRegistryEntry {
    let dec = loom_codec::decode_sealed(&sealed.bytes).expect("dekodieren");
    let manifest = dec
        .frames
        .iter()
        .find_map(|(e, f)| {
            if e.kind == KIND_MANIFEST {
                loom_canon::decode(&f.payload).ok()
            } else {
                None
            }
        })
        .expect("jeder versiegelte Container traegt ein MANIFEST");
    let class = match cv_get(&manifest, "container_class") {
        Some(Cv::Text(s)) => s.clone(),
        _ => String::new(),
    };
    let domain = match cv_get(&manifest, "domain_refs") {
        Some(Cv::Array(items)) => items
            .first()
            .and_then(|v| match v {
                Cv::Text(s) => Some(s.clone()),
                _ => None,
            })
            .unwrap_or_default(),
        _ => String::new(),
    };
    // Keine der aktuell committeten Seed-Container traegt ein
    // SIGNATURE-Segment (der Ed25519-Pfad, P6c/X1e, wurde bisher nur
    // manuell ueber die CLI auf temporaeren Kopien vorgefuehrt, nie
    // programmatisch in einen Seed eingebrannt) — daher hier ehrlich
    // leer statt einer neuen Kern-Abhaengigkeit auf loom-cli (das
    // ed25519-dalek/zstd/blake3 traegt) nur fuer ein derzeit stets
    // leeres Feld. Sobald ein signierter Seed existiert, liest man die
    // Rollen direkt aus dessen SIGNATURE-Frames (dieselbe Cv-Struktur
    // wie in loom_cli::sign::verify_sig_all, hier ohne die Kiste).
    let signatures: Vec<String> = Vec::new();
    loom_cites::ClassRegistryEntry {
        core_root_hex: hex34(&sealed.core_root),
        class,
        domain,
        signatures,
        path: path.to_string(),
    }
}

/// Der Registry-Workbody: katalogisiert die real committeten Seed-
/// Container. "inspection"-Profil (nur MANIFEST/CANON_DESC Pflicht) +
/// additives KIND_DOC (dieselbe generische Kind-Wiederverwendung wie
/// `folder_meta`/`doc_meta` andernorts — kein neues Segment-Kind).
pub fn build_class_registry() -> Sealed {
    let welt = build_welt_kristall_wikimedia();
    let welt_root_hex = hex34(&welt.core_root);
    let citing = seal_citing_memo_welt_kristall(&welt_root_hex);
    let mappe_a = build_scale2_folder_full();
    let mappe_b = build_scale2_folder_b(&welt_root_hex);
    let blueprint_ref = build_blueprint_reference_cube();
    let blueprint_eigen = build_blueprint_eigenkorpus();
    let risikomatrix = build_risikomatrix_workbody();

    let entries = vec![
        catalog_entry_from_sealed(&welt, "library/seed/kristall_wikimedia_workbody.loom"),
        catalog_entry_from_sealed(&citing, "library/seed/citing_memo_welt_kristall.loom"),
        catalog_entry_from_sealed(&mappe_a, "library/seed/scale2_projektmappe_full.loom"),
        catalog_entry_from_sealed(&mappe_b, "library/seed/scale2_projektmappe_b.loom"),
        catalog_entry_from_sealed(&blueprint_ref, "library/seed/blueprint_reference_cube.loom"),
        catalog_entry_from_sealed(&blueprint_eigen, "library/seed/blueprint_eigenkorpus.loom"),
        catalog_entry_from_sealed(
            &risikomatrix,
            "library/seed/risikomatrix_memo_workbody.loom",
        ),
    ];
    let doc = loom_cites::class_registry_field(&entries);
    let m = manifest_cv(
        "Klassen-Registry (X2/E4c) — zertifizierte core_roots",
        "inspection",
        "PL1",
        false,
        0,
        &[],
        &["read_segment"],
        "SEE-REPO-ROOT",
    );
    seal_canonical(
        "inspection",
        &["inspection"],
        &[
            seg(KIND_MANIFEST, &m),
            canon_desc_segment(),
            seg(KIND_DOC, &doc),
        ],
    )
    .unwrap()
}
