//! Etappe X2/E2 (S-E2a Teil I) — Zeugen R-CIT-1/2, N-CIT-1, N-CIT-5 auf
//! den ECHTEN, committeten Seed-Containern (Welt-Kristall + zitierendes
//! Memo). N-CIT-2/3/4 sind bereits in loom-cites (Mock-Resolver,
//! deterministisch, jeder Gate-Schritt isoliert) abgedeckt — hier
//! zaehlt die reale Container-Kette.

use loom_cites::{citation_gate, hex34, CitationOutcome, SeedResolver};
use loom_conformance::{build_welt_kristall_wikimedia, seal_citing_memo_welt_kristall};

fn seed_dir() -> std::path::PathBuf {
    std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../library/seed"))
}

#[test]
fn r_cit_1_and_2_supports_and_derives_chain_on_the_real_welt_kristall_resolve_green() {
    let welt = build_welt_kristall_wikimedia();
    let welt_root_hex = hex34(&welt.core_root);
    let memo = seal_citing_memo_welt_kristall(&welt_root_hex);

    // Der zitierende Container selbst bleibt ganz normal Valid.
    let report = loom_verify::verify(&memo.bytes);
    assert_eq!(report.verdict, loom_verify::Verdict::Valid);

    let resolver = SeedResolver::new(vec![seed_dir()]);
    let gate = citation_gate(&memo.bytes, &resolver);
    assert_eq!(gate.entries.len(), 2, "R-CIT-1 und R-CIT-2 in einem Memo");
    assert!(
        gate.entries
            .iter()
            .all(|e| e.outcome == CitationOutcome::Ok),
        "beide cites-Eintraege muessen gruen aufloesen: {:?}",
        gate.entries
    );
    assert!(gate.closure_pass, "Close(x)=1 ⟺ CitationGate(x)=Pass");
}

#[test]
fn n_cit_1_unresolved_target_stays_transportable_but_blocks_closure() {
    let welt = build_welt_kristall_wikimedia();
    let welt_root_hex = hex34(&welt.core_root);
    let memo = seal_citing_memo_welt_kristall(&welt_root_hex);

    // Container bleibt transportierbar/Valid — verify() ist hermetisch
    // und braucht keinen Resolver, um konsistent zu sein.
    let report = loom_verify::verify(&memo.bytes);
    assert_eq!(report.verdict, loom_verify::Verdict::Valid);

    // Ein Resolver OHNE das Ziel (leeres Verzeichnis) findet nichts —
    // Closure/Commit muss blockieren, aber niemals „reject" vortaeuschen.
    let empty_dir = std::env::temp_dir().join(format!("e2-cites-empty-{}", std::process::id()));
    std::fs::create_dir_all(&empty_dir).expect("temp dir");
    let resolver = SeedResolver::new(vec![empty_dir.clone()]);
    let gate = citation_gate(&memo.bytes, &resolver);
    assert!(gate
        .entries
        .iter()
        .all(|e| e.outcome == CitationOutcome::UnresolvedCitation));
    assert!(!gate.closure_pass, "Closure-Hold ohne aufloesbares Ziel");
    std::fs::remove_dir_all(&empty_dir).ok();
}

/// N-CIT-5: MANIFEST.external_citations weicht von den tatsaechlichen
/// cites-Zielen im CL_SUBSTRATE ab ⇒ reject (loom-verify, hermetisch,
/// kein Resolver noetig).
#[test]
fn n_cit_5_manifest_citations_list_mismatch_is_rejected() {
    use loom_canon::Cv;
    use loom_codec::{seal_canonical, Segment};
    use loom_format::{KIND_HEADER, KIND_MANIFEST, KIND_SEGTAB};

    let welt = build_welt_kristall_wikimedia();
    let welt_root_hex = hex34(&welt.core_root);
    let memo = seal_citing_memo_welt_kristall(&welt_root_hex);

    let dec = loom_codec::decode_sealed(&memo.bytes).expect("dekodieren");
    let mut segments: Vec<Segment> = Vec::new();
    for (e, f) in &dec.frames {
        if e.kind == KIND_HEADER || e.kind == KIND_SEGTAB {
            continue;
        }
        if e.kind == KIND_MANIFEST {
            let Cv::Map(fields) = loom_canon::decode(&f.payload).unwrap() else {
                panic!("MANIFEST ist keine Map")
            };
            // external_citations auf eine leere Liste kappen — weicht
            // damit garantiert von den zwei echten cites-Zielen ab.
            let tampered: Vec<(Cv, Cv)> = fields
                .into_iter()
                .map(|(k, v)| {
                    if matches!(&k, Cv::Text(s) if s == "external_citations") {
                        (k, Cv::Array(vec![]))
                    } else {
                        (k, v)
                    }
                })
                .collect();
            segments.push(Segment {
                kind: KIND_MANIFEST,
                seg_flags: e.seg_flags,
                payload: Cv::Map(tampered).encode().unwrap(),
                deps: e.deps.clone(),
            });
        } else {
            segments.push(Segment {
                kind: e.kind,
                seg_flags: e.seg_flags,
                payload: f.payload.clone(),
                deps: e.deps.clone(),
            });
        }
    }
    let resealed = seal_canonical("workcell", &["workcell"], &segments).unwrap();
    let report = loom_verify::verify(&resealed.bytes);
    assert_eq!(report.verdict, loom_verify::Verdict::Reject);
    assert!(report
        .diagnoses
        .iter()
        .any(|d| d.point == "manifest_citation_mismatch"));
}

#[test]
fn seed_file_matches_builder() {
    let welt = build_welt_kristall_wikimedia();
    let welt_root_hex = hex34(&welt.core_root);
    let built = seal_citing_memo_welt_kristall(&welt_root_hex);
    let on_disk = std::fs::read(seed_dir().join("citing_memo_welt_kristall.loom"))
        .expect("Seed-Datei muss vorhanden sein");
    assert_eq!(
        built.bytes, on_disk,
        "Generator und Seed-Datei muessen byte-identisch sein"
    );
}
