//! Etappe X2/E4a (S-E4a Teil II) — CoreExtension CE-1 "Tabellen-
//! Zellentyp". Zeugen R-TBL-1 (Risikomatrix, voller Motorpfad,
//! Pipe-Roundtrip) und R-TBL-2 (D06 Angebot: Positionstabelle,
//! Familien-Kern unveraendert). N-TBL-1/2 (ragged_table/
//! invalid_cell_type) sind bereits als deterministische Unit-Tests in
//! cce-materialize (document::ce1_table_tests) abgedeckt; N-TBL-3
//! (Alt-Motor-Simulation) steht unten.

use cce_materialize::document::assets::three_risks_memo;
use cce_materialize::document::{parse, DocUnit, TableCell};
use cce_materialize::family_a::{domain_core_gate, DomainRule};
use cce_materialize::family_a_domains::d06;
use loom_conformance::{build_risikomatrix_workbody, risikomatrix_memo};

fn seed_dir() -> std::path::PathBuf {
    std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../library/seed"))
}

/// R-TBL-1: das reale, versiegelte Risikomatrix-Memo ist Valid und
/// traegt die echte Pipe-Tabelle byte-identisch extrahierbar.
#[test]
fn r_tbl_1_risikomatrix_workbody_is_valid_and_pipe_table_extracts_correctly() {
    let sealed = build_risikomatrix_workbody();
    let report = loom_verify::verify(&sealed.bytes);
    assert_eq!(report.verdict, loom_verify::Verdict::Valid);

    let handle = loom_mount::open(&sealed.bytes).expect("oeffnen");
    let extracted = loom_mount::extract_artifact(&handle).expect("Artefakt extrahieren");
    let text = String::from_utf8(extracted).expect("UTF-8");
    assert!(text.contains("| Risiko | Wahrscheinlichkeit | Kosten |"));
    assert!(text.contains("Serverausfall"));
    assert!(
        text.contains("1.5"),
        "DecFrac-Zelle muss dezimal erscheinen:\n{text}"
    );

    // Reanalyse aus den extrahierten Bytes muss klassenidentisch sein —
    // das ist die "Pipe-Roundtrip"-Haelfte des R-TBL-1-Kerntests.
    let back = parse::parse_markdown(text.as_bytes()).expect("Reanalyse");
    let original = risikomatrix_memo();
    assert_eq!(
        cce_core::canonical::Canonicalize::canonical_class(&back),
        cce_core::canonical::Canonicalize::canonical_class(&original),
    );
}

/// R-TBL-2: eine Table-Einheit in einer ECHTEN PL3-Domaene (D06
/// Angebot) — der Familien-Kern (domain_core_gate, family_a_domains::d06)
/// bleibt UNVERAENDERT: die Positionstabelle ist kein "subject"
/// (UnitType::Definition), die Kern-Regel (Relation{seam:"priced"})
/// ignoriert sie vollstaendig.
#[test]
fn r_tbl_2_table_in_d06_angebot_leaves_the_family_kernel_unchanged() {
    let profile = d06();
    let mut crystal = (profile.reference)();

    let positionstabelle = DocUnit::new_table(
        "postbl",
        &["Position", "Menge", "Einzelpreis"],
        vec![
            vec![
                TableCell::Text("Lizenz".to_string()),
                TableCell::Int(10),
                TableCell::DecFrac {
                    num: 4999,
                    scale: 2,
                },
            ],
            vec![
                TableCell::Text("Support".to_string()),
                TableCell::Int(1),
                TableCell::DecFrac {
                    num: 1200,
                    scale: 2,
                },
            ],
        ],
    )
    .with_seam("refers", "s0");
    crystal.units.push(positionstabelle);

    // Der reale, UNVERAENDERTE Familien-Kern muss weiterhin Pass melden —
    // die Tabelle stoert die Relation-Regel ueber "priced" nicht.
    let report = domain_core_gate(&profile, &crystal);
    assert!(report.is_pass(), "{}", report.reason);
    assert!(matches!(
        profile.rule,
        DomainRule::Relation { seam: "priced" }
    ));

    // Und die Tabelle selbst bleibt ganz normal reanalyse-faehig.
    let table_unit = crystal.units.iter().find(|u| u.id == "postbl").unwrap();
    assert!(table_unit.as_table().is_some());
}

/// N-TBL-3: ein Motor OHNE Table-Kenntnis (simuliert — die sieben
/// Alt-Typen) haelt fail-closed mit `unsupported_unit_type`, statt still
/// zu ueberspringen. Der REALE (neue) Parser demonstriert dasselbe
/// Verhalten fuer JEDEN unbekannten Typ generisch — "table" waere fuer
/// einen Alt-Motor genau so ein unbekannter Typ gewesen.
#[test]
fn n_tbl_3_old_engine_without_table_support_holds_not_skips() {
    fn old_engine_known_unit_type(s: &str) -> Result<(), String> {
        match s {
            "section" | "claim" | "support" | "risk" | "countermeasure" | "definition" | "step" => {
                Ok(())
            }
            other => Err(format!("unsupported_unit_type: {other}")),
        }
    }
    let err = old_engine_known_unit_type("table").unwrap_err();
    assert!(err.contains("unsupported_unit_type"));

    // Der reale, aktuelle Parser haelt genauso fail-closed bei JEDEM
    // unbekannten Typ — kein stilles Ueberspringen, kein Crash.
    let bad = "# X\n\n<!--cce:doc ordering=neutral;no_score_fields=false;covers=;sections=-->\n\nText\n<!--cce:unit id=u1;type=voellig_unbekannt;seams=-->\n";
    let err2 = parse::parse_markdown(bad.as_bytes()).unwrap_err();
    assert!(err2.contains("unbekannter Einheitstyp"));
}

/// Alt-Zeugen unveraendert: Drei-Risiken-Memo (ohne Table) reanalysiert
/// weiterhin klassenidentisch ueber den echten Motorpfad — CE-1 aendert
/// nichts an Nicht-Table-Pfaden.
#[test]
fn non_table_reference_cube_is_unaffected() {
    use cce_materialize::adapter::DomainAdapter;
    use cce_materialize::document::DocumentAdapter;

    let crystal = three_risks_memo();
    let adapter = DocumentAdapter;
    let pkg = adapter.encode(&crystal);
    let proj = cce_phc::projection_calc::project(&pkg, "proj:materialize").expect("projizieren");
    let weave = adapter.loom(&proj).expect("weben");
    let artifact = adapter.materialize(&weave);
    let back = adapter.reanalyze(&artifact).expect("Reanalyse");
    assert!(adapter.equivalent(&back, &crystal));
}

#[test]
fn seed_file_matches_builder() {
    let built = build_risikomatrix_workbody();
    let on_disk = std::fs::read(seed_dir().join("risikomatrix_memo_workbody.loom"))
        .expect("Seed-Datei muss vorhanden sein");
    assert_eq!(built.bytes, on_disk);
}
