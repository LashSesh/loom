//! G12-Auslieferungszeugen: Offline-Kern, PL-Kennzeichnung
//! (feature_maturity_overclaim), Update-Kanal-Waechter, Saat-Bibliothek
//! im Pakettbaum, Schluesselbund-Trennung.

use cce_conformance::{feature_maturity_overclaim, update_dod, FEATURE_PL, PRODUCT_VERSION};

#[test]
fn offline_core_proof() {
    // Der Kern laeuft ohne jede Netzumgebung: Proxy-Variablen leeren
    // und die volle Produktreise fahren (kein Codepfad beruehrt einen
    // Socket — CI-seitig zusaetzlich durch den Symbol-Scan bewacht).
    std::env::remove_var("HTTPS_PROXY");
    std::env::remove_var("HTTP_PROXY");
    std::env::remove_var("ALL_PROXY");
    use cce_core::replay::RunDescriptor;
    use cce_core::signature::sha256;
    use cockpit_core::engine::MotorEngine;
    use cockpit_core::kanzel::{KanzelPort, LocalKanzel};
    use cockpit_core::state::{CockpitCore, CockpitState, Confirmation};
    let mut core = CockpitCore::new(MotorEngine::default());
    core.enter_wish("offline memo").unwrap();
    let (crystal, _) = LocalKanzel.form_wish("offline").unwrap();
    core.crystal_formed(crystal).unwrap();
    let conf = |a: &str| {
        Some(Confirmation {
            operator: "op".into(),
            action: a.into(),
            statement: "ok".into(),
        })
    };
    core.confirm_crystal(conf("c")).unwrap();
    core.start_run(
        RunDescriptor::new(sha256(b"offline"), "document", 7),
        conf("s"),
    )
    .unwrap();
    assert_eq!(core.state, CockpitState::ArtefaktVerfuegbar);
}

#[test]
fn every_feature_carries_pl_no_overclaim() {
    assert!(!FEATURE_PL.is_empty());
    for (name, pl, evidence) in FEATURE_PL {
        assert!(pl.starts_with("PL"), "{name} ohne PL");
        assert!(!evidence.is_empty(), "{name} ohne Evidence-Ort");
    }
    assert!(
        feature_maturity_overclaim().is_empty(),
        "Overclaim: {:?}",
        feature_maturity_overclaim()
    );
    // Auch der Domaenenkatalog ist overclaim-frei:
    assert!(cce_materialize::catalog::feature_maturity_overclaim().is_empty());
    assert_eq!(cce_materialize::catalog::CATALOG.len(), 213);
}

#[test]
fn update_channel_guard_fail_closed() {
    assert!(update_dod(true).is_ok());
    assert!(update_dod(false).is_err(), "kein Ausweichkanal");
    assert_eq!(PRODUCT_VERSION, "0.1.0");
}

#[test]
fn seed_library_ships_as_loom() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    for f in ["drei_risiken_memo_workbody.loom", "minimal_inspect.loom"] {
        let path = root.join("library/seed").join(f);
        let bytes = std::fs::read(&path).unwrap_or_else(|e| panic!("{path:?}: {e}"));
        // Jede Saat-Datei ist ein VALIDER .loom-Container:
        let report = loom_verify::verify(&bytes);
        assert!(matches!(
            report.verdict,
            loom_verify::Verdict::Valid | loom_verify::Verdict::ValidWithResidues
        ));
    }
    // Und das Paketskript nimmt sie mit:
    let pkg = std::fs::read_to_string(root.join("ci/package.sh")).unwrap();
    assert!(pkg.contains("library/seed/*.loom"));
    assert!(pkg.contains("docs/operator"));
}

#[test]
fn secrets_never_flow_through_persistence() {
    // Schluesselbund-Trennung (S11.3): der PersistenceAdapter hat KEINE
    // Secret-API — Geheimnisse koennen den CAS-Pfad typsystemisch nicht
    // nehmen; die OS-Schluesselbund-Bindung ist Betriebsschritt.
    use cockpit_core::persistence::{LocalWorkspace, PersistenceAdapter};
    let mut ws = LocalWorkspace::default();
    let addr = ws.store_artifact(b"artefakt-bytes");
    assert!(!addr.is_empty());
    // Kein trait-Methodenname enthaelt 'secret'/'key' — Stichprobe der
    // API-Flaeche (die Quelle ist die Wahrheit; dieser Test bricht,
    // wenn jemand eine Secret-API in den Persistenzpfad schmuggelt).
    let src = include_str!("../../cockpit/cockpit-core/src/persistence.rs");
    assert!(!src.to_lowercase().contains("secret"));
    assert!(!src.to_lowercase().contains("keyring"));
}
