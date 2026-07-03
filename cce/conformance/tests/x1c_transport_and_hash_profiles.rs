//! Etappe X1(c) (Oekosystem-Karte §2/E1) — Zeuge: zstd-Transportprofil
//! (Ganzdatei-Kompression, canonical-stored bleibt Golden-Referenz,
//! Klassen-Digest unberuehrt) + blake3-Zweitprofil (Header-deklariert,
//! additiv zu den sha2-256-Frame-Digests). Beide Kisten leben
//! ausschliesslich im CLI-Blatt (loom-cli), dieselbe Disziplin wie
//! Ed25519/JSON.

use loom_canon::Cv;
use loom_cli::hashprofile::blake3_hex;
use loom_cli::transport::{compress, decompress};
use loom_conformance::{build_r1, build_scale2_folder_full, build_welt_kristall_wikimedia};

fn repo_root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .canonicalize()
        .unwrap()
}

/// Exit-Zeuge: "zstd-Container klassenidentisch zu stored" — auf ALLEN
/// drei realen, committeten Arbeitskoerpern (nicht nur einem
/// Kunstbeispiel).
#[test]
fn zstd_roundtrip_is_class_identical_to_stored_on_real_workbodies() {
    let root = repo_root();
    let files = [
        "library/seed/kristall_wikimedia_workbody.loom",
        "library/seed/scale2_projektmappe_full.loom",
        "library/seed/drei_risiken_memo_workbody.loom",
    ];
    for f in files {
        let stored = std::fs::read(root.join(f)).unwrap_or_else(|e| panic!("{f}: {e}"));
        let packed = compress(&stored);
        assert!(
            packed.len() < stored.len(),
            "{f}: zstd sollte einen realen Arbeitskoerper verkleinern"
        );
        let unpacked = decompress(&packed).unwrap_or_else(|e| panic!("{f}: {e:?}"));
        assert_eq!(
            unpacked, stored,
            "{f}: zstd-Roundtrip muss byte-identisch sein"
        );

        // "Klassenidentisch": derselbe core_root, dasselbe Verdikt —
        // die kanonisch-gespeicherte Form bleibt die Golden-Referenz.
        let stored_report = loom_verify::verify(&stored);
        let unpacked_report = loom_verify::verify(&unpacked);
        assert_eq!(stored_report.verdict, unpacked_report.verdict);
        let stored_handle = loom_mount::open(&stored).unwrap();
        let unpacked_handle = loom_mount::open(&unpacked).unwrap();
        assert_eq!(
            stored_handle.decoded.footer.core_root, unpacked_handle.decoded.footer.core_root,
            "{f}: core_root muss durch zstd-Transport unberuehrt bleiben"
        );
    }
}

#[test]
fn zstd_rejects_tampered_transport_bytes() {
    let sealed = build_welt_kristall_wikimedia();
    let mut packed = compress(&sealed.bytes);
    let last = packed.len() - 1;
    packed[last] ^= 0xff;
    assert!(
        decompress(&packed).is_err(),
        "manipulierte zstd-Bytes muessen fail-closed abgewiesen werden"
    );
}

/// Exit-relevant fuer (c): blake3-Zweitprofil ist Header-deklariert und
/// unabhaengig nachrechenbar — additiv, die Frame-Digests bleiben
/// sha2-256 (unveraendert, kein Container-Format-Wechsel).
#[test]
fn blake3_second_profile_is_declared_and_independently_reproducible() {
    // Deklaration: ein MANIFEST mit hash_profiles=[sha2-256, blake3].
    let base = build_r1();
    let handle = loom_mount::open(&base.bytes).unwrap();
    let manifest_frame = handle
        .decoded
        .frames
        .iter()
        .find(|(e, _)| e.kind == loom_format::KIND_MANIFEST)
        .unwrap()
        .1
        .clone();
    let manifest_cv = loom_canon::decode(&manifest_frame.payload).unwrap();
    let declared =
        loom_conformance::manifest_declare_hash_profiles(manifest_cv, &["sha2-256", "blake3"]);
    let Cv::Map(fields) = &declared else {
        panic!("Map erwartet")
    };
    let hash_profiles: Cv = fields
        .iter()
        .find(|(k, _)| matches!(k, Cv::Text(s) if s == "hash_profiles"))
        .map(|(_, v)| v.clone())
        .expect("hash_profiles fehlt");
    assert_eq!(
        hash_profiles,
        Cv::Array(vec![
            Cv::Text("sha2-256".to_string()),
            Cv::Text("blake3".to_string())
        ])
    );

    // Unabhaengige Nachrechenbarkeit: derselbe Container, zweimal
    // gehasht, liefert denselben blake3-Digest (Determinismus) — und
    // ein ANDERER Arbeitskoerper liefert einen ANDEREN Digest
    // (Kontextabhaengigkeit, keine feste Attrappe).
    let d1 = blake3_hex(&base.bytes);
    let d2 = blake3_hex(&base.bytes);
    assert_eq!(d1, d2);
    let other = build_scale2_folder_full();
    assert_ne!(d1, blake3_hex(&other.bytes));
}
