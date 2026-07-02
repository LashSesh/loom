//! DER MOTOR-KERNTEST (VC1⁺, Bauverfassung Teil 0/7.5.6, 01_MASTER_BUILD G3):
//!
//!     Reanalyze(Materialize(LOOM(Project(PHC(Crystal))))) ≃ Crystal
//!
//! fuer den Referenz-Cube „Drei-Risiken-Memo" — plus: alle 7 Dokument-Gates
//! gruen am Referenz-Cube, rot an je einem Negativ-Cube; Zwei-Digest-
//! Trennung (Format-Kosmetik aendert Byte-, nie Klassen-Digest).

use cce_materialize::adapter::{check_adapter_parity_typed, DomainAdapter};
use cce_materialize::document::{DocArtifact, DocumentAdapter};
use cce_phc::loader::load_phc;
use cce_phc::projection_calc::project;

/// Der geschlossene Pfad, domaenen-konkret (S1.3).
fn closed_path(adapter: &DocumentAdapter) -> (cce_materialize::document::DocCrystal, DocArtifact) {
    let crystal = adapter.reference_cube();
    // encode: Crystal → PHC
    let package = adapter.encode(&crystal);
    // Loader validiert V0–V9 mit Root-Hash (Can(P), PHC V2).
    let root = package.root_hash();
    load_phc(&package, Some(root)).expect("Loader V0–V9 gruen");
    // project: PHC → lokale Projektion
    let projection = project(&package, "proj:materialize").expect("Projektion");
    // loom: Projektion → Gewebe (radiale Spindel, Nullanker markiert)
    let weave = adapter.loom(&projection).expect("Webstuhl");
    assert_eq!(
        cce_loom::radial_spindle::RadialSpindle::NULLPOINT_TRAVERSAL,
        "forbidden"
    );
    assert!(!weave.weave.spindle.boundary_traces.is_empty());
    // materialize: Gewebe → Artefakt
    let artifact = adapter.materialize(&weave);
    (crystal, artifact)
}

/// VC1⁺ — der Kerntest.
#[test]
fn motor_kerntest_reanalyze_materialize_loom_project_phc_is_identity() {
    let adapter = DocumentAdapter;
    let (crystal, artifact) = closed_path(&adapter);
    // reanalyze: Artefakt → Crystal'
    let back = adapter.reanalyze(&artifact).expect("Reanalyse");
    // equivalent: kanonische Inhaltsklasse (nie Bytes)
    assert!(
        adapter.equivalent(&back, &crystal),
        "≃ verletzt: {:?} ≠ {:?}",
        adapter.canonicalize(&back),
        adapter.canonicalize(&crystal)
    );
    // Der Collect-Sweep erzeugt zusaetzlich einen MatrixCrystal (Obs(A)).
    let mc = cce_observe::reanalyze::observe(
        &artifact.bytes,
        |bytes| {
            cce_materialize::document::parse::parse_markdown(bytes)
                .map(|c| cce_core::canonical::Canonicalize::canon(&c))
        },
        "kerntest",
    )
    .expect("Obs(A)");
    use cce_core::canonical::Canonicalize;
    assert_eq!(
        mc.class(),
        crystal.canonical_class(),
        "q(Obs(A)) = q(C) verletzt"
    );
}

/// Alle 7 Dokument-Gates gruen am Referenz-Cube.
#[test]
fn seven_document_gates_green_on_reference() {
    let adapter = DocumentAdapter;
    let (crystal, artifact) = closed_path(&adapter);
    let reports = adapter.run_domain_gates(&crystal, Some(&artifact));
    assert_eq!(reports.len(), 7);
    for r in &reports {
        assert!(
            r.is_pass(),
            "{} rot am Referenz-Cube: {}",
            r.gate_id,
            r.reason
        );
    }
}

/// Jeder Negativ-Cube wird mit dem ERWARTETEN Residuum abgelehnt.
#[test]
fn negative_cubes_red_with_expected_residue() {
    let adapter = DocumentAdapter;
    for (cube, expected) in adapter.negative_cubes() {
        let reports = adapter.run_domain_gates(&cube, None);
        let holds: Vec<&cce_core::gate::GateReport> =
            reports.iter().filter(|r| !r.is_pass()).collect();
        assert!(
            holds.iter().any(|h| h.reason.contains(expected.as_str())),
            "Negativ-Cube ({}) wurde nicht mit erwartetem Residuum abgelehnt: {holds:?}",
            expected.as_str()
        );
    }
}

/// invented_semantic / semantic_loss: manipulierte Artefakte brechen den
/// Round-Trip mit benanntem Residuum (S1.9 Negativ-Cube 4).
#[test]
fn tampered_artifacts_break_roundtrip_with_named_residue() {
    let adapter = DocumentAdapter;
    let (crystal, artifact) = closed_path(&adapter);
    // Erfundener Absatz: zusaetzliche Einheit injizieren.
    let mut invented = String::from_utf8(artifact.bytes.clone()).unwrap();
    invented.push_str("Frei erfundene Aussage ohne Kristall-Herkunft\n");
    invented.push_str("<!--cce:unit id=fake1;type=claim;seams=-->\n");
    let report = cce_materialize::document::gates::roundtrip(
        &adapter,
        &crystal,
        &DocArtifact {
            bytes: invented.into_bytes(),
            format: ".md",
        },
    );
    assert!(!report.is_pass());
    assert!(
        report.reason.contains("invented_semantic"),
        "{}",
        report.reason
    );
    // Verlorene Einheit: eine Einheit entfernen.
    let text = String::from_utf8(artifact.bytes.clone()).unwrap();
    let lost: String = text
        .lines()
        .filter(|l| !l.contains("id=c2"))
        .map(|l| format!("{l}\n"))
        .collect::<String>()
        .replace("Vollstaendige Sicherung und Probelauf der Migration\n", "");
    let report = cce_materialize::document::gates::roundtrip(
        &adapter,
        &crystal,
        &DocArtifact {
            bytes: lost.into_bytes(),
            format: ".md",
        },
    );
    assert!(!report.is_pass());
    assert!(report.reason.contains("semantic_loss"), "{}", report.reason);
}

/// Zwei-Digest-Trennung (S7.2): Format-Kosmetik aendert den Byte-Digest,
/// NIE den Inhaltsklassen-Digest.
#[test]
fn two_digest_separation() {
    let adapter = DocumentAdapter;
    let (crystal, artifact) = closed_path(&adapter);
    // Kosmetik: zusaetzliche Leerzeilen + Trailing-Whitespace.
    let cosmetic = String::from_utf8(artifact.bytes.clone())
        .unwrap()
        .lines()
        .map(|l| format!("{l}   \n\n"))
        .collect::<String>();
    let cosmetic_artifact = DocArtifact {
        bytes: cosmetic.into_bytes(),
        format: ".md",
    };
    // Byte-Digest: verschieden.
    assert_ne!(artifact.byte_digest(), cosmetic_artifact.byte_digest());
    // Inhaltsklassen-Digest: identisch.
    let back = adapter.reanalyze(&cosmetic_artifact).expect("Reanalyse");
    assert!(adapter.equivalent(&back, &crystal));
    assert_eq!(adapter.canonicalize(&back), adapter.canonicalize(&crystal));
}

/// Adapter-Paritaet: Dokument erfuellt die kanonische Teileliste (11/11).
#[test]
fn document_adapter_parity_green() {
    let r = check_adapter_parity_typed(&DocumentAdapter);
    assert!(r.is_pass(), "{}", r.reason);
}
