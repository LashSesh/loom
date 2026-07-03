//! loom-conformance — Referenzdateien R1–R8 (Teil 10.1 + Overlay 05
//! Teil E) als deterministische Builder. Die Golden Files unter
//! `loom/golden/` sind die canonical-stored-Ausprägungen; der Test
//! vergleicht Builder-Bytes gegen die eingecheckten Dateien (C0).

use cce_core::canonical::Canonicalize;
use loom_canon::Cv;
use loom_codec::{seal_canonical, Sealed, Segment};
use loom_format::{
    KIND_ARTIFACT, KIND_CANDIDATE_OUTPUTS, KIND_CANON_DESC, KIND_CL_SUBSTRATE, KIND_CSA_NSB,
    KIND_DOC, KIND_EVIDENCE, KIND_GATE_REPORTS, KIND_HBM, KIND_INFERENCE_PROFILE, KIND_LEDGER,
    KIND_MANIFEST, KIND_PHC, KIND_PROVIDER_MANIFEST, KIND_REPLAY_MANIFEST, KIND_RESIDUE,
    KIND_RUNTIME_PROFILE, KIND_TOOL_PROFILE,
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
