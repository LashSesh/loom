//! Golden-File-Wache (C0, canonical-stored): die Builder muessen die
//! eingecheckten Bytes EXAKT reproduzieren — jede Format-Regression
//! faellt hier, bevor sie ein Release erreicht.

use loom_conformance::REFERENCE_BUILDERS;

fn repo_root() -> std::path::PathBuf {
    // CARGO_MANIFEST_DIR = .../cce/loom/loom-conformance
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}

#[test]
fn golden_files_byte_identical() {
    let root = repo_root();
    for (name, builder) in REFERENCE_BUILDERS {
        let sealed = builder();
        let path = root.join(format!("loom/golden/{}.loom", name.to_lowercase()));
        let stored =
            std::fs::read(&path).unwrap_or_else(|e| panic!("Golden File {path:?} fehlt: {e}"));
        assert_eq!(
            sealed.bytes, stored,
            "{name}: Builder weicht vom Golden File ab"
        );
    }
}

#[test]
fn seed_library_is_r7_and_r1() {
    let root = repo_root();
    let r7 = REFERENCE_BUILDERS
        .iter()
        .find(|(n, _)| *n == "R7")
        .unwrap()
        .1();
    let seed = std::fs::read(root.join("library/seed/drei_risiken_memo_workbody.loom")).unwrap();
    assert_eq!(r7.bytes, seed);
    let r1 = REFERENCE_BUILDERS
        .iter()
        .find(|(n, _)| *n == "R1")
        .unwrap()
        .1();
    let seed1 = std::fs::read(root.join("library/seed/minimal_inspect.loom")).unwrap();
    assert_eq!(r1.bytes, seed1);
}

#[test]
fn viewer_renders_golden_r7_without_motor() {
    // Reader-Prinzip-Nachweis zur Laufzeit: der motorfreie Viewer-Pfad
    // (loom_mount/loom_verify) validiert das Golden File ohne
    // Projektwissen bis L2.
    let root = repo_root();
    let bytes = std::fs::read(root.join("loom/golden/r7.loom")).unwrap();
    let report = loom_verify::verify(&bytes);
    assert!(matches!(
        report.verdict,
        loom_verify::Verdict::Valid | loom_verify::Verdict::ValidWithResidues
    ));
}
