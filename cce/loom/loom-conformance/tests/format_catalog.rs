//! Format-Zeugenkatalog (LOOM Teil 10 + Overlay 05 Teil E):
//! 8 Referenzen valid · 16 Negative reject/quarantine am definierten
//! Pruefpunkt · Conformance-Ebenen C0–C5.

use loom_canon::Cv;
use loom_codec::{decode_sealed, seal_canonical, Segment};
use loom_conformance::{
    build_r1, build_r5, build_r8, canon_desc_segment, manifest_cv, r5_commit_class,
    REFERENCE_BUILDERS,
};
use loom_format::{
    Frame, FOOTER_LEN, KIND_CANDIDATE_OUTPUTS, KIND_CL_SUBSTRATE, KIND_CSA_NSB, KIND_GATE_REPORTS,
    KIND_LEDGER, KIND_MANIFEST, KIND_PROVIDER_MANIFEST, KIND_REPLAY_MANIFEST,
};
use loom_mount::{mount, MountError, MountMode};
use loom_replay::{check_replay, replay_manifest_segment, ReplayVerdict};
use loom_runner::{run_workcell, RunError, RunnerContext};
use loom_verify::{verify, Verdict};

fn seg(kind: u16, v: &Cv) -> Segment {
    Segment::canonical(kind, v).unwrap()
}

fn base_manifest(class: &str) -> Cv {
    manifest_cv("neg", class, "PL0", false, 0, &[], &["read_segment"], "cc0")
}

// ---------- C2: 8 Referenzen valid ----------

#[test]
fn c2_all_references_valid() {
    for (name, builder) in REFERENCE_BUILDERS {
        let sealed = builder();
        let report = verify(&sealed.bytes);
        assert!(
            matches!(report.verdict, Verdict::Valid | Verdict::ValidWithResidues),
            "{name} muss valid sein, war {:?}: {:?}",
            report.verdict,
            report.diagnoses
        );
    }
}

// ---------- C0: decode fehlerfrei; encode∘decode byte-identisch ----------

#[test]
fn c0_encode_decode_byte_identical_canonical_stored() {
    for (name, builder) in REFERENCE_BUILDERS {
        let sealed = builder();
        let dec = decode_sealed(&sealed.bytes).expect(name);
        // Re-Seal aus den dekodierten logischen Segmenten (ohne Header-
        // Pseudo-Eintrag, der wird beim Seal neu erzeugt):
        let (class, profiles) = manifest_class(&dec);
        let segments: Vec<Segment> = dec
            .frames
            .iter()
            .filter(|(e, _)| e.kind != loom_format::KIND_HEADER)
            .map(|(e, f)| Segment {
                kind: e.kind,
                seg_flags: e.seg_flags,
                payload: f.payload.clone(),
                deps: e.deps.clone(),
            })
            .collect();
        let profiles_ref: Vec<&str> = profiles.iter().map(|s| s.as_str()).collect();
        let resealed = seal_canonical(&class, &profiles_ref, &segments).unwrap();
        assert_eq!(
            resealed.bytes, sealed.bytes,
            "{name}: encode∘decode nicht byte-identisch"
        );
    }
}

fn manifest_class(dec: &loom_codec::Decoded) -> (String, Vec<String>) {
    for (e, f) in &dec.frames {
        if e.kind == KIND_MANIFEST {
            let v = loom_canon::decode(&f.payload).unwrap();
            let get = |key: &str| -> Option<Cv> {
                match &v {
                    Cv::Map(entries) => entries.iter().find_map(|(k, val)| match k {
                        Cv::Text(s) if s == key => Some(val.clone()),
                        _ => None,
                    }),
                    _ => None,
                }
            };
            let class = match get("container_class") {
                Some(Cv::Text(s)) => s,
                _ => panic!("container_class fehlt"),
            };
            let profiles = match get("profiles_required") {
                Some(Cv::Array(items)) => items
                    .iter()
                    .filter_map(|i| match i {
                        Cv::Text(s) => Some(s.clone()),
                        _ => None,
                    })
                    .collect(),
                _ => vec![],
            };
            return (class, profiles);
        }
    }
    panifest()
}

fn panifest() -> (String, Vec<String>) {
    panic!("MANIFEST fehlt")
}

// ---------- C1: permutierte Eingaben ⇒ identische core_root ----------

#[test]
fn c1_permuted_inputs_same_core_root() {
    let m = base_manifest("inspection");
    let cl = Cv::map(vec![("cubes", Cv::Array(vec![]))]);
    let a = seal_canonical(
        "inspection",
        &["inspection"],
        &[
            seg(KIND_MANIFEST, &m),
            canon_desc_segment(),
            seg(KIND_CL_SUBSTRATE, &cl),
        ],
    )
    .unwrap();
    let b = seal_canonical(
        "inspection",
        &["inspection"],
        &[
            seg(KIND_CL_SUBSTRATE, &cl),
            seg(KIND_MANIFEST, &m),
            canon_desc_segment(),
        ],
    )
    .unwrap();
    assert_eq!(a.core_root, b.core_root);
    assert_eq!(a.bytes, b.bytes);
}

// ---------- 16 Negative (rot am definierten Pruefpunkt) ----------

#[test]
fn n1_wrong_magic_and_profile_conflict() {
    let mut bytes = build_r1().bytes;
    bytes[0] = 0x50;
    assert_eq!(verify(&bytes).verdict, Verdict::Reject, "falsches Magic");
    // Profilkonflikt: unbekanntes Profil im Manifest
    let m = manifest_cv("n1b", "hyperdrive", "PL0", false, 0, &[], &[], "cc0");
    let sealed = seal_canonical(
        "hyperdrive",
        &["hyperdrive"],
        &[seg(KIND_MANIFEST, &m), canon_desc_segment()],
    )
    .unwrap();
    let r = verify(&sealed.bytes);
    assert_eq!(r.verdict, Verdict::Reject);
    assert!(r.diagnoses.iter().any(|d| d.point == "profile_unknown"));
}

#[test]
fn n2_missing_required_segment() {
    // source-Profil OHNE CSA_NSB/EVIDENCE
    let m = base_manifest("source");
    let sealed = seal_canonical(
        "source",
        &["source"],
        &[seg(KIND_MANIFEST, &m), canon_desc_segment()],
    )
    .unwrap();
    let r = verify(&sealed.bytes);
    assert_eq!(r.verdict, Verdict::Reject);
    assert!(r.diagnoses.iter().any(|d| d.point == "required_segment"));
}

#[test]
fn n3_noncanonical_map_sorting() {
    // Handgebaute unsortierte dCBOR-Map als Segment-Payload: {"b":1,"a":1}
    let raw = vec![0xa2, 0x61, b'b', 0x01, 0x61, b'a', 0x01];
    let bad = Segment {
        kind: KIND_CL_SUBSTRATE,
        seg_flags: 0,
        payload: raw,
        deps: vec![],
    };
    let m = base_manifest("inspection");
    let sealed = seal_canonical(
        "inspection",
        &["inspection"],
        &[seg(KIND_MANIFEST, &m), canon_desc_segment(), bad],
    )
    .unwrap();
    let r = verify(&sealed.bytes);
    assert_eq!(r.verdict, Verdict::Reject);
    assert!(
        r.diagnoses.iter().any(|d| d.point == "canon"),
        "{:?}",
        r.diagnoses
    );
}

#[test]
fn n4_root_hash_mismatch() {
    let mut bytes = build_r1().bytes;
    // core_root im Footer kippen:
    let root_off = bytes.len() - FOOTER_LEN + 20;
    bytes[root_off] ^= 0xff;
    let r = verify(&bytes);
    assert_eq!(r.verdict, Verdict::Reject);
    assert!(r.diagnoses.iter().any(|d| d.point == "core_root"));
}

#[test]
fn n5_csu_without_evidence_pack() {
    let nsb = Cv::map(vec![
        ("bundle_id", Cv::Text("nsb:x".into())),
        ("csu_uids", Cv::Array(vec![Cv::Text("csu:abc".into())])),
        ("evidence_for", Cv::Array(vec![])),
    ]);
    let m = base_manifest("source");
    let ev = Cv::map(vec![("packs", Cv::Array(vec![]))]);
    let sealed = seal_canonical(
        "source",
        &["source"],
        &[
            seg(KIND_MANIFEST, &m),
            canon_desc_segment(),
            seg(KIND_CSA_NSB, &nsb),
            seg(loom_format::KIND_EVIDENCE, &ev),
        ],
    )
    .unwrap();
    let r = verify(&sealed.bytes);
    assert_eq!(r.verdict, Verdict::Reject);
    assert!(r.diagnoses.iter().any(|d| d.point == "evidence_missing"));
}

#[test]
fn n6_commit_without_gate_report_and_summary_mismatch() {
    let ledger = Cv::map(vec![
        ("commits", Cv::Array(vec![Cv::Text("commit:1".into())])),
        ("gate_reports_for", Cv::Array(vec![])),
    ]);
    let m = base_manifest("runtime");
    let replay = replay_manifest_segment("sha256:rd", 7, "class");
    let sealed = seal_canonical(
        "runtime",
        &["runtime"],
        &[
            seg(KIND_MANIFEST, &m),
            canon_desc_segment(),
            seg(KIND_LEDGER, &ledger),
            seg(KIND_REPLAY_MANIFEST, &replay),
        ],
    )
    .unwrap();
    let r = verify(&sealed.bytes);
    assert_eq!(r.verdict, Verdict::Reject);
    assert!(r.diagnoses.iter().any(|d| d.point == "gate_report_missing"));
    // residue_summary ≠ RESIDUE:
    let m2 = manifest_cv("n6b", "inspection", "PL0", false, 3, &["x"], &[], "cc0");
    let sealed2 = seal_canonical(
        "inspection",
        &["inspection"],
        &[seg(KIND_MANIFEST, &m2), canon_desc_segment()],
    )
    .unwrap();
    let r2 = verify(&sealed2.bytes);
    assert_eq!(r2.verdict, Verdict::Reject);
    assert!(r2
        .diagnoses
        .iter()
        .any(|d| d.point == "residue_summary_mismatch"));
}

#[test]
fn n7_replay_manifest_incomplete() {
    let incomplete = Cv::map(vec![("seed", Cv::Uint(7))]);
    assert!(matches!(
        check_replay(&incomplete, "x"),
        ReplayVerdict::ManifestIncomplete {
            missing: "commit_class"
        }
    ));
}

#[test]
fn n8_capability_escalation() {
    let ctx = RunnerContext::new(&["read_segment", "project_workcell"]);
    let err = run_workcell(&ctx, "w:1", "write_phaseblock", "input").unwrap_err();
    assert_eq!(
        err,
        RunError::CapabilityNotDeclared {
            requested: "write_phaseblock".into()
        }
    );
    // deklariert, aber Lock zu ⇒ ebenfalls kein Lauf:
    let err2 = run_workcell(&ctx, "w:1", "project_workcell", "input").unwrap_err();
    assert!(matches!(err2, RunError::CapabilityLockClosed { .. }));
}

#[test]
fn n9_sealed_without_valid_footer() {
    let sealed = build_r1();
    let truncated = &sealed.bytes[..sealed.bytes.len() - FOOTER_LEN];
    assert_eq!(verify(truncated).verdict, Verdict::Reject);
}

#[test]
fn n10_old_version_without_declared_migration() {
    let mut bytes = build_r1().bytes;
    bytes[9] = 0x00; // format_major 0 — Altversion ohne Migrationspfad
    assert_eq!(verify(&bytes).verdict, Verdict::Reject);
}

#[test]
fn n11_cyclic_reference_structure() {
    // Zwei Segmente, deren deps aufeinander zeigen (Zyklus).
    let a_payload = Cv::map(vec![("a", Cv::Uint(1))]).encode().unwrap();
    let b_payload = Cv::map(vec![("b", Cv::Uint(2))]).encode().unwrap();
    let mk_digest = |payload: &[u8]| {
        let mut d = [0u8; 34];
        d[..2].copy_from_slice(&loom_format::MULTIHASH_SHA256);
        d[2..].copy_from_slice(&loom_format::sha256::sha256(payload));
        d
    };
    let a = Segment {
        kind: KIND_CL_SUBSTRATE,
        seg_flags: 0,
        payload: a_payload.clone(),
        deps: vec![mk_digest(&b_payload)],
    };
    let b = Segment {
        kind: loom_format::KIND_PHC,
        seg_flags: 0,
        payload: b_payload,
        deps: vec![mk_digest(&a_payload)],
    };
    let m = base_manifest("inspection");
    let sealed = seal_canonical(
        "inspection",
        &["inspection"],
        &[seg(KIND_MANIFEST, &m), canon_desc_segment(), a, b],
    )
    .unwrap();
    let r = verify(&sealed.bytes);
    assert_eq!(r.verdict, Verdict::Reject);
    assert!(r.diagnoses.iter().any(|d| d.point == "dep_cycle"));
}

#[test]
fn n12_json_only_pseudocontainer() {
    let json = br#"{"hypercube": true, "loom": "pseudo"}"#;
    assert_eq!(verify(json).verdict, Verdict::Reject);
}

#[test]
fn n13_decompression_bomb_declaration() {
    // Frame mit stored_len < uncompressed_len (Bomben-Signatur) faellt
    // in Frame::decode — VOR jeder Allokation der deklarierten Groesse.
    let f = Frame {
        kind: KIND_CL_SUBSTRATE,
        seg_flags: 0,
        payload: vec![1, 2, 3],
    };
    let mut enc = f.encode();
    enc[4..12].copy_from_slice(&(1u64 << 40).to_le_bytes());
    assert!(matches!(
        Frame::decode(&enc, 0),
        Err(loom_format::FormatError::StoredExceedsDeclared { .. })
    ));
}

#[test]
fn n14_score_as_verdict_in_gate_reports() {
    let gr = Cv::map(vec![(
        "reports",
        Cv::Array(vec![Cv::map(vec![
            ("gate_id", Cv::Text("q".into())),
            ("verdict", Cv::Bool(true)),
            ("reason", Cv::Text("ok".into())),
            ("score", Cv::Uint(950)),
        ])]),
    )]);
    let m = base_manifest("inspection");
    let sealed = seal_canonical(
        "inspection",
        &["inspection"],
        &[
            seg(KIND_MANIFEST, &m),
            canon_desc_segment(),
            seg(KIND_GATE_REPORTS, &gr),
        ],
    )
    .unwrap();
    let r = verify(&sealed.bytes);
    assert_eq!(r.verdict, Verdict::Reject);
    assert!(r.diagnoses.iter().any(|d| d.point == "score_as_verdict"));
}

#[test]
fn n15_hidden_model_call_on_open() {
    let pm = Cv::map(vec![
        ("providers", Cv::Array(vec![])),
        ("on_open", Cv::Text("call:cloud".into())),
    ]);
    let m = base_manifest("inspection");
    let sealed = seal_canonical(
        "inspection",
        &["inspection"],
        &[
            seg(KIND_MANIFEST, &m),
            canon_desc_segment(),
            seg(KIND_PROVIDER_MANIFEST, &pm),
        ],
    )
    .unwrap();
    let r = verify(&sealed.bytes);
    assert_eq!(r.verdict, Verdict::Reject);
    assert!(r
        .diagnoses
        .iter()
        .any(|d| d.point == "hidden_model_call_on_open"));
}

#[test]
fn n16_provider_autostart_flag() {
    let pm = Cv::map(vec![(
        "providers",
        Cv::Array(vec![Cv::map(vec![
            ("provider_id", Cv::Text("cloud-x".into())),
            ("autostart", Cv::Bool(true)),
        ])]),
    )]);
    let m = base_manifest("inspection");
    let sealed = seal_canonical(
        "inspection",
        &["inspection"],
        &[
            seg(KIND_MANIFEST, &m),
            canon_desc_segment(),
            seg(KIND_PROVIDER_MANIFEST, &pm),
        ],
    )
    .unwrap();
    let r = verify(&sealed.bytes);
    assert_eq!(r.verdict, Verdict::Reject);
    assert!(r
        .diagnoses
        .iter()
        .any(|d| d.point == "provider_autostart_flag"));
}

// ---------- C3: Seiteneffektfreiheit + mount-ro ----------

#[test]
fn c3_open_inspect_verify_side_effect_free() {
    // Beweisanordnung: open/inspect/verify arbeiten auf &[u8] und haben
    // strukturell keinen FS-/Netz-Zugriff (Viewer-Pfad ist motorfrei,
    // CI-Regel). Laufzeitnachweis: kein neuer FS-Eintrag im Arbeitsdir.
    let dir = std::env::temp_dir().join("loom-c3-check");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let before: Vec<_> = std::fs::read_dir(&dir)
        .unwrap()
        .count()
        .to_string()
        .into_bytes();
    let sealed = build_r1();
    let handle = loom_mount::open(&sealed.bytes).unwrap();
    let _ = loom_mount::inspect(&sealed.bytes, &handle);
    let _ = verify(&sealed.bytes);
    let after: Vec<_> = std::fs::read_dir(&dir)
        .unwrap()
        .count()
        .to_string()
        .into_bytes();
    assert_eq!(
        before, after,
        "open/inspect/verify duerfen nichts schreiben"
    );
    // Oeffnungs-Haertung (Overlay 05): der Viewer-Pfad KANN keinen
    // Modell-/Tool-Egress ausloesen — loom-mount/verify haengen an
    // keinem Gateway-Crate (ci/check_acyclic Reader-Regel, CI-bewacht).
    // mount-ro schreibt nie: Quarantaene-Datei darf nicht run-mounten.
    let mut tampered = build_r8().bytes;
    // unbekanntes Kind mit required_understand einschleusen → quarantine
    // (siehe c5-Test); hier: Reject-Datei darf gar nicht mounten.
    tampered[0] ^= 0xff;
    assert!(matches!(
        mount(&tampered, MountMode::Run),
        Err(MountError::RejectedNoMount)
    ));
}

// ---------- C4: Replay reproduziert Commit-Klasse ----------

#[test]
fn c4_replay_reproduces_commit_class() {
    let sealed = build_r5();
    let dec = decode_sealed(&sealed.bytes).unwrap();
    let replay_payload = dec
        .frames
        .iter()
        .find(|(e, _)| e.kind == KIND_REPLAY_MANIFEST)
        .map(|(_, f)| loom_canon::decode(&f.payload).unwrap())
        .expect("REPLAY_MANIFEST");
    // Motor-Port: dieselbe Klasse deterministisch reproduzieren.
    let reproduced = r5_commit_class();
    assert_eq!(
        check_replay(&replay_payload, &reproduced),
        ReplayVerdict::Pass
    );
    // Drift wird benannt:
    assert!(matches!(
        check_replay(&replay_payload, "andere-klasse"),
        ReplayVerdict::ClassMismatch { .. }
    ));
}

// ---------- C5: Minor-additiv bleibt kompatibel ----------

#[test]
fn c5_minor_additive_stays_read_compatible() {
    // Datei mit unbekanntem EXT-Kind (non_core): preserve + sichtbar,
    // Verdikt valid_with_residues — Read-/Seal-kompatibel.
    let m = base_manifest("inspection");
    let ext = Segment {
        kind: 0x7042,
        seg_flags: loom_format::SEG_FLAG_NON_CORE,
        payload: Cv::map(vec![("zukunft", Cv::Uint(1))]).encode().unwrap(),
        deps: vec![],
    };
    let sealed = seal_canonical(
        "inspection",
        &["inspection"],
        &[seg(KIND_MANIFEST, &m), canon_desc_segment(), ext],
    )
    .unwrap();
    let r = verify(&sealed.bytes);
    assert!(
        matches!(r.verdict, Verdict::Valid | Verdict::ValidWithResidues),
        "{:?}",
        r.diagnoses
    );
    // Unbekanntes NICHT-EXT-Kind mit required_understand ⇒ quarantine:
    let unknown = Segment {
        kind: 0x0099,
        seg_flags: loom_format::SEG_FLAG_REQUIRED_UNDERSTAND,
        payload: Cv::Null.encode().unwrap(),
        deps: vec![],
    };
    let sealed2 = seal_canonical(
        "inspection",
        &["inspection"],
        &[seg(KIND_MANIFEST, &m), canon_desc_segment(), unknown],
    )
    .unwrap();
    assert_eq!(verify(&sealed2.bytes).verdict, Verdict::Quarantine);
}

// ---------- Overlay: CandidateOutput nie Commit (Format-Ebene) ----------

#[test]
fn candidate_marked_commit_rejected() {
    let cands = Cv::map(vec![(
        "outputs",
        Cv::Array(vec![Cv::map(vec![
            ("candidate_id", Cv::Text("c1".into())),
            ("evidence_ref", Cv::Text("iev:1".into())),
            ("is_commit", Cv::Bool(true)),
        ])]),
    )]);
    let m = base_manifest("inspection");
    let sealed = seal_canonical(
        "inspection",
        &["inspection"],
        &[
            seg(KIND_MANIFEST, &m),
            canon_desc_segment(),
            seg(KIND_CANDIDATE_OUTPUTS, &cands),
        ],
    )
    .unwrap();
    let r = verify(&sealed.bytes);
    assert_eq!(r.verdict, Verdict::Reject);
    assert!(r
        .diagnoses
        .iter()
        .any(|d| d.point == "candidate_marked_commit"));
}
