//! Etappe X1(a) (Oekosystem-Karte §2/E1) — Zeuge: der Container traegt
//! die materialisierten Artefakt-Bytes SELBST (CAS_BLOB), nicht nur
//! ihren Digest. `loom_mount::extract_artifact` liefert sie byte-
//! identisch zu `materialize()`; Re-Import aus dem Container ist
//! Re-Import aus der Datei — schliesst das im Welt-Crystal-Bericht
//! (reports/welt_crystal_wikimedia.md) sichtbar gefuehrte Residuum.

use cce_core::canonical::Canonicalize;
use cce_core::replay::RunDescriptor;
use cce_core::signature::sha256;
use cce_materialize::document::parse::parse_markdown;
use cce_runner::runner::Run;
use loom_conformance::{build_welt_kristall_wikimedia, kristall_memo_from_wikimedia};
use nexus_adapter::port::SourceAdapter;
use nexus_adapter_wikimedia::WikimediaAdapter;
use nexus_core::objects::RawObservation;

fn repo_root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}

/// Baut die reale DocArtifact unabhaengig neu — derselbe Pfad wie der
/// Builder, aber hier zum GEGENPRUEFEN aufgerufen, nicht zum Erzeugen.
fn rebuild_real_artifact_bytes() -> (Vec<u8>, cce_materialize::document::DocCrystal) {
    let root = repo_root();
    let fixture = std::fs::read(root.join("conformance/fixtures/wikimedia_kristall.json")).unwrap();
    let raw = RawObservation {
        locator: loom_conformance::WELT_KRISTALL_LOCATOR.to_string(),
        bytes: fixture,
        fetched_via: "test".to_string(),
        snapshot_id: "snap-test".to_string(),
    };
    let adapter = WikimediaAdapter;
    let recs = adapter.extract(&raw).unwrap();
    let csu = &adapter.normalize(&recs[0])[0];
    let extract = match csu.payload.get("extract") {
        Some(cce_core::value::CanonValue::Text(t)) => t.replace('\n', " "),
        _ => panic!("extract fehlt"),
    };
    let attribution = adapter.cite(csu).remove(0);
    let crystal = kristall_memo_from_wikimedia(&extract, &attribution);
    let rd = RunDescriptor::new(sha256(b"welt-crystal-wikimedia"), "document", 7);
    let mut run = Run::submit(crystal.clone(), rd).unwrap();
    run.run_to_end(None).unwrap();
    (run.artifact.unwrap().bytes, crystal)
}

#[test]
fn container_carries_cas_blob_and_extract_is_byte_identical_to_materialize() {
    let sealed = build_welt_kristall_wikimedia();
    let handle = loom_mount::open(&sealed.bytes).expect("oeffnen");

    // Reader-Prinzip: extract_artifact braucht keinen Motor, nur den
    // Container selbst.
    let extracted = loom_mount::extract_artifact(&handle).expect("extract_artifact");

    let (real_materialize_bytes, _crystal) = rebuild_real_artifact_bytes();
    assert_eq!(
        extracted, real_materialize_bytes,
        "extract() muss byte-identisch zu materialize() sein"
    );
}

#[test]
fn reimport_from_container_equals_reimport_from_file() {
    let sealed = build_welt_kristall_wikimedia();
    let handle = loom_mount::open(&sealed.bytes).expect("oeffnen");
    let extracted = loom_mount::extract_artifact(&handle).expect("extract_artifact");

    // "Re-Import aus dem Container = Re-Import aus der Datei": dieselben
    // Bytes zuerst auf Platte geschrieben, dann ueber parse_markdown
    // reanalysiert — beide Wege muessen zur SELBEN kanonischen Klasse
    // fuehren wie das urspruengliche Crystal.
    let tmp = std::env::temp_dir().join("x1a-extract-test.md");
    std::fs::write(&tmp, &extracted).unwrap();
    let from_file = std::fs::read(&tmp).unwrap();
    std::fs::remove_file(&tmp).ok();

    let reimported_from_container = parse_markdown(&extracted).expect("reanalyze (Container)");
    let reimported_from_file = parse_markdown(&from_file).expect("reanalyze (Datei)");
    assert_eq!(
        reimported_from_container.canonical_class(),
        reimported_from_file.canonical_class(),
        "Container- und Datei-Reimport muessen dieselbe Klasse ergeben"
    );

    let (_bytes, original_crystal) = rebuild_real_artifact_bytes();
    assert_eq!(
        reimported_from_container.canonical_class(),
        original_crystal.canonical_class(),
        "Reimport muss die urspruengliche Crystal-Klasse reproduzieren (kein semantic_loss)"
    );
}

#[test]
fn extract_fails_closed_on_containers_without_cas_blob() {
    // R1 (minimaler Inspect-Container) hat gar kein ARTIFACT-Segment.
    let r1 = loom_conformance::build_r1();
    let handle = loom_mount::open(&r1.bytes).expect("oeffnen");
    assert_eq!(
        loom_mount::extract_artifact(&handle),
        Err(loom_mount::ExtractError::NoArtifact)
    );
    // R7 HAT ein ARTIFACT-Segment (Digest-Metadaten), aber (bewusst
    // unveraendert, s. X1a-Bericht) keinen CAS_BLOB — ein Fakt ueber
    // diesen aelteren, illustrativen Container, kein Absturz.
    let r7 = loom_conformance::build_r7();
    let handle7 = loom_mount::open(&r7.bytes).expect("oeffnen");
    assert_eq!(
        loom_mount::extract_artifact(&handle7),
        Err(loom_mount::ExtractError::NoCasBlob)
    );
}
