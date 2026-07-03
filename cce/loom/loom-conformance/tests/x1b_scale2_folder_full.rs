//! Etappe X1(b) (Oekosystem-Karte §2/E1) — Zeuge: die SCALE-2-Mappe
//! buendelt ihre Memo-.looms PHYSISCH (CAS_BLOB je Kind). Mappen-
//! Extraktion liefert n eigenstaendig gueltige Kind-Arbeitskoerper —
//! nicht nur eine content_class-Referenzliste.

use loom_conformance::build_scale2_folder_full;

#[test]
fn folder_container_verifies_valid() {
    let sealed = build_scale2_folder_full();
    let report = loom_verify::verify(&sealed.bytes);
    assert_eq!(
        report.verdict,
        loom_verify::Verdict::Valid,
        "Diagnosen: {:?}",
        report.diagnoses
    );
}

#[test]
fn folder_extraction_yields_two_independently_valid_children() {
    let sealed = build_scale2_folder_full();
    let handle = loom_mount::open(&sealed.bytes).expect("Mappe oeffnen");

    let children = loom_mount::all_cas_blobs(&handle).expect("CAS_BLOBs");
    assert_eq!(children.len(), 2, "Mappe->n: hier n=2 Kinder erwartet");

    for (i, child_bytes) in children.iter().enumerate() {
        let child_handle = loom_mount::open(child_bytes)
            .unwrap_or_else(|e| panic!("Kind {i} laesst sich nicht oeffnen: {e:?}"));
        let child_report = loom_verify::verify(child_bytes);
        assert_eq!(
            child_report.verdict,
            loom_verify::Verdict::Valid,
            "Kind {i} muss eigenstaendig Valid sein: {:?}",
            child_report.diagnoses
        );
        // Jedes Kind traegt SELBST wieder ein ARTIFACT+CAS_BLOB (X1a) —
        // die Buendelung ist rekursiv konsistent, kein Sonderfall.
        assert!(
            loom_mount::extract_artifact(&child_handle).is_ok(),
            "Kind {i} muss sein eigenes Artefakt extrahieren lassen"
        );
    }
}

#[test]
fn folder_children_core_roots_match_declared_entries() {
    // Cross-Check: die im DOC-Segment deklarierten child_core_root-Werte
    // muessen GENAU den core_roots der tatsaechlich extrahierten Kinder
    // entsprechen (Mengen-Vergleich, ordnungsunabhaengig — die Segtab-
    // Reihenfolge ist digest-sortiert, nicht Einfuege-Reihenfolge).
    use loom_canon::Cv;
    use loom_format::KIND_DOC;

    let sealed = build_scale2_folder_full();
    let handle = loom_mount::open(&sealed.bytes).expect("oeffnen");

    let doc_frame = handle
        .decoded
        .frames
        .iter()
        .find(|(e, _)| e.kind == KIND_DOC)
        .expect("DOC-Segment fehlt")
        .1
        .clone();
    let doc = loom_canon::decode(&doc_frame.payload).expect("DOC dekodieren");
    let Cv::Map(top) = &doc else {
        panic!("DOC ist keine Map")
    };
    let entries = top
        .iter()
        .find(|(k, _)| matches!(k, Cv::Text(s) if s == "entries"))
        .map(|(_, v)| v)
        .expect("entries fehlt");
    let Cv::Array(entries) = entries else {
        panic!("entries ist kein Array")
    };
    let mut declared_roots: Vec<String> = entries
        .iter()
        .map(|e| {
            let Cv::Map(fields) = e else {
                panic!("Eintrag ist keine Map")
            };
            fields
                .iter()
                .find(|(k, _)| matches!(k, Cv::Text(s) if s == "child_core_root"))
                .and_then(|(_, v)| match v {
                    Cv::Text(s) => Some(s.clone()),
                    _ => None,
                })
                .expect("child_core_root fehlt")
        })
        .collect();
    declared_roots.sort();

    let children = loom_mount::all_cas_blobs(&handle).expect("CAS_BLOBs");
    let mut actual_roots: Vec<String> = children
        .iter()
        .map(|bytes| {
            let h = loom_mount::open(bytes).expect("Kind oeffnen");
            h.decoded
                .footer
                .core_root
                .iter()
                .map(|b| format!("{b:02x}"))
                .collect::<String>()
        })
        .collect();
    actual_roots.sort();

    assert_eq!(
        declared_roots, actual_roots,
        "deklarierte Kind-core_roots muessen exakt den extrahierten entsprechen"
    );
}

#[test]
fn folder_is_deterministic() {
    let a = build_scale2_folder_full();
    let b = build_scale2_folder_full();
    assert_eq!(a.bytes, b.bytes);
}

#[test]
fn seed_file_matches_builder() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap();
    let sealed = build_scale2_folder_full();
    let stored = std::fs::read(root.join("library/seed/scale2_projektmappe_full.loom"))
        .expect("Seed-Datei fehlt — mit `cargo run -p loom-conformance --bin scale2-folder-gen -- .` erzeugen");
    assert_eq!(
        sealed.bytes, stored,
        "committeter Seed weicht vom Builder ab"
    );
}
