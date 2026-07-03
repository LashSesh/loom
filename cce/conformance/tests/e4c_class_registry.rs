//! Etappe X2/E4c (Karte §2/E4c) — Klassen-Registry als `.loom`-Katalog-
//! Workbody + zweite `CitationResolver`-Implementierung. Exit-Zeugen:
//! "Registry-Workbody selbst Valid und cites-auflösend".

use loom_cites::{citation_gate, hex34, CitationOutcome, RegistryResolver};
use loom_conformance::{
    build_class_registry, build_welt_kristall_wikimedia, seal_citing_memo_welt_kristall,
};

fn seed_dir() -> std::path::PathBuf {
    std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../library/seed"))
}

/// Registry-Eintraege tragen `path` relativ zur Repo-Wurzel
/// (`library/seed/...`) — der Resolver braucht also die Repo-Wurzel als
/// `base_dir`, nicht das Seed-Verzeichnis selbst.
fn repo_root() -> std::path::PathBuf {
    std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/.."))
}

#[test]
fn class_registry_workbody_is_valid() {
    let sealed = build_class_registry();
    let report = loom_verify::verify(&sealed.bytes);
    assert_eq!(report.verdict, loom_verify::Verdict::Valid);
}

#[test]
fn class_registry_catalogs_the_real_welt_kristall_with_correct_class() {
    let welt = build_welt_kristall_wikimedia();
    let welt_root_hex = hex34(&welt.core_root);
    let sealed = build_class_registry();

    let dec = loom_codec::decode_sealed(&sealed.bytes).unwrap();
    let doc = dec
        .frames
        .iter()
        .find_map(|(e, f)| {
            if e.kind == loom_format::KIND_DOC {
                loom_canon::decode(&f.payload).ok()
            } else {
                None
            }
        })
        .expect("Registry traegt ein KIND_DOC");
    let entries = loom_cites::parse_class_registry(&doc).expect("lesbare Registry");
    let welt_entry = entries
        .iter()
        .find(|e| e.core_root_hex == welt_root_hex)
        .expect("Welt-Kristall muss katalogisiert sein");
    assert_eq!(welt_entry.class, "full");
    assert_eq!(
        welt_entry.path,
        "library/seed/kristall_wikimedia_workbody.loom"
    );
}

/// "cites-aufloesend": dieselbe R-CIT-1/2-Szene aus Ring E2, aber mit
/// dem RegistryResolver (der ZWEITEN CitationResolver-Implementierung)
/// statt dem SeedResolver — E2 haengt nicht an einer bestimmten
/// Resolver-Implementierung (I.2).
#[test]
fn registry_resolver_resolves_the_same_citation_scene_as_seed_resolver() {
    let welt = build_welt_kristall_wikimedia();
    let welt_root_hex = hex34(&welt.core_root);
    let memo = seal_citing_memo_welt_kristall(&welt_root_hex);

    let registry_bytes = std::fs::read(seed_dir().join("class_registry.loom"))
        .expect("Registry-Seed muss vorhanden sein");
    let resolver = RegistryResolver::from_registry_bytes(&registry_bytes, repo_root())
        .expect("Registry muss einen Resolver liefern");

    let gate = citation_gate(&memo.bytes, &resolver);
    assert_eq!(gate.entries.len(), 2);
    assert!(
        gate.entries
            .iter()
            .all(|e| e.outcome == CitationOutcome::Ok),
        "beide cites-Eintraege muessen ueber den Registry-Resolver gruen aufloesen: {:?}",
        gate.entries
    );
    assert!(gate.closure_pass);
}

#[test]
fn seed_file_matches_builder() {
    let built = build_class_registry();
    let on_disk = std::fs::read(seed_dir().join("class_registry.loom"))
        .expect("Seed-Datei muss vorhanden sein");
    assert_eq!(built.bytes, on_disk);
}
