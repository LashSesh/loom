//! Vollausbau Block 3 — Zeuge: das erste Welt-Crystal (echte Wikimedia-
//! Quelle) ist deterministisch, im committeten Seed identisch, motorfrei
//! verifizierbar (loom_verify) UND traegt nachweislich den echten Inhalt
//! + die echte CC-BY-SA-Attribution — nicht nur eine gueltige Huelle.

use loom_canon::Cv;
use loom_conformance::build_welt_kristall_wikimedia;
use loom_format::{KIND_CSA_NSB, KIND_DOC, KIND_EVIDENCE};

fn repo_root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}

fn cv_get<'a>(v: &'a Cv, key: &str) -> Option<&'a Cv> {
    match v {
        Cv::Map(entries) => entries
            .iter()
            .find(|(k, _)| matches!(k, Cv::Text(s) if s == key))
            .map(|(_, v)| v),
        _ => None,
    }
}

fn cv_text<'a>(v: &'a Cv, key: &str) -> Option<&'a str> {
    match cv_get(v, key) {
        Some(Cv::Text(s)) => Some(s.as_str()),
        _ => None,
    }
}

#[test]
fn builder_is_deterministic() {
    let a = build_welt_kristall_wikimedia();
    let b = build_welt_kristall_wikimedia();
    assert_eq!(
        a.bytes, b.bytes,
        "zwei Baulaeufe muessen byte-identisch sein"
    );
    assert_eq!(a.core_root, b.core_root);
}

#[test]
fn seed_file_matches_builder() {
    let root = repo_root();
    let sealed = build_welt_kristall_wikimedia();
    let stored = std::fs::read(root.join("library/seed/kristall_wikimedia_workbody.loom"))
        .expect("Seed-Datei fehlt — mit `cargo run -p loom-conformance --bin welt-kristall-gen -- .` erzeugen");
    assert_eq!(
        sealed.bytes, stored,
        "committeter Seed weicht vom Builder ab — neu generieren"
    );
}

#[test]
fn seed_verifies_valid_without_motor() {
    // Reader-Prinzip (wie golden.rs): der motorfreie Viewer-Pfad
    // (loom_verify) validiert das Werk OHNE Projektwissen.
    let root = repo_root();
    let bytes = std::fs::read(root.join("library/seed/kristall_wikimedia_workbody.loom")).unwrap();
    let report = loom_verify::verify(&bytes);
    assert_eq!(
        report.verdict,
        loom_verify::Verdict::Valid,
        "Diagnosen: {:?}",
        report.diagnoses
    );
    assert_eq!(report.residue_count, 0, "geschlossen (∅) erwartet");
}

#[test]
fn seed_carries_the_real_wikimedia_extract_and_attribution() {
    // Beweist: kein Platzhalter, keine erfundene Beispielzelle — der
    // DOC/EVIDENCE-Inhalt ist wortwoertlich aus der eingefrorenen, echten
    // Wikimedia-Antwort (conformance/fixtures/wikimedia_kristall.json).
    let root = repo_root();
    let bytes = std::fs::read(root.join("library/seed/kristall_wikimedia_workbody.loom")).unwrap();
    let handle = loom_mount::open(&bytes).expect("oeffnen");

    let doc_frame = handle
        .decoded
        .frames
        .iter()
        .find(|(e, _)| e.kind == KIND_DOC)
        .expect("DOC-Segment fehlt")
        .1
        .clone();
    let doc = loom_canon::decode(&doc_frame.payload).expect("DOC dekodieren");
    assert_eq!(
        cv_text(&doc, "title"),
        Some("Kristall — Quellenbeleg aus Wikimedia")
    );
    // Der Container speichert (wie R7) Digest-Metadaten, nicht die
    // Roh-Bytes des Artefakts — die Klasse muss trotzdem auf die
    // UNABHAENGIG aus der echten Fixture neu berechnete Klasse zeigen,
    // nicht auf einen erstarrten/verwaisten Wert. "Unabhaengig" heisst
    // hier: der komplette Fixture->Adapter->Crystal-Pfad laeuft ein
    // zweites Mal, mit demselben Locator (die Attribution bindet ihn
    // ein) — kein blosses Zurücklesen des Containers.
    let independently_recomputed_class = {
        use cce_core::canonical::Canonicalize;
        use nexus_adapter::port::SourceAdapter;
        use nexus_adapter_wikimedia::WikimediaAdapter;
        use nexus_core::objects::RawObservation;
        let fixture = std::fs::read(root.join("conformance/fixtures/wikimedia_kristall.json"))
            .expect("Fixture lesen");
        let raw = RawObservation {
            locator: loom_conformance::WELT_KRISTALL_LOCATOR.to_string(),
            bytes: fixture,
            fetched_via: "test".to_string(),
            snapshot_id: "snap-test".to_string(),
        };
        let adapter = WikimediaAdapter;
        let recs = adapter.extract(&raw).expect("extrahieren");
        let csu = &adapter.normalize(&recs[0])[0];
        let extract = match csu.payload.get("extract") {
            Some(cce_core::value::CanonValue::Text(t)) => t.replace('\n', " "),
            _ => panic!("extract fehlt"),
        };
        let attribution = adapter.cite(csu).remove(0);
        let crystal = loom_conformance::kristall_memo_from_wikimedia(&extract, &attribution);
        crystal.canonical_class().0.to_hex()
    };
    assert_eq!(
        cv_text(&doc, "class"),
        Some(independently_recomputed_class.as_str()),
        "DOC.class muss die echte, unabhaengig nachgerechnete Kristall-Klasse tragen"
    );

    let evidence_frame = handle
        .decoded
        .frames
        .iter()
        .find(|(e, _)| e.kind == KIND_EVIDENCE)
        .expect("EVIDENCE-Segment fehlt")
        .1
        .clone();
    let evidence = loom_canon::decode(&evidence_frame.payload).expect("EVIDENCE dekodieren");
    let packs = cv_get(&evidence, "packs").expect("packs");
    let Cv::Array(packs) = packs else {
        panic!("packs ist kein Array")
    };
    assert_eq!(packs.len(), 1);
    let pack = &packs[0];
    assert_eq!(cv_text(pack, "license"), Some("cc-by-sa-4.0"));
    let attribution = cv_text(pack, "attribution").expect("Attribution muss transportiert sein");
    assert!(
        attribution.contains("Beitraeger") && attribution.contains("cc-by-sa"),
        "{attribution}"
    );

    let nsb_frame = handle
        .decoded
        .frames
        .iter()
        .find(|(e, _)| e.kind == KIND_CSA_NSB)
        .expect("CSA_NSB-Segment fehlt")
        .1
        .clone();
    let nsb = loom_canon::decode(&nsb_frame.payload).expect("NSB dekodieren");
    assert!(
        cv_get(&nsb, "csu_uids").is_some(),
        "CSU-UID muss referenziert sein"
    );
}
