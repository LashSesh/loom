//! G6-Ausgangs-Gate (01_MASTER_BUILD):
//! - CAS-Round-trip aller Objekttypen
//! - Ref-Konflikt sichtbar + operator-aufloesbar
//! - Waechter blockiert nachweislich einen absichtlich eingeschleusten Bruch
//! - Dokument-Zeugen: alle Referenzen gruen, alle Negativen rot

use cce_core::signature::sha256;
use cce_library::guard::{run_guard, GuardOutcome};
use cce_library::registry::{AssetKind, Registry, RegistryError};
use cce_materialize::adapter::DomainAdapter;
use cce_materialize::document::{DocumentAdapter, UnitType};
use cce_store::cas::{Cas, FsCas, MemoryCas, ObjectKind};

/// CAS-Round-trip aller adressierten Objekttypen (S9-A1).
#[test]
fn cas_roundtrip_all_object_kinds() {
    let kinds = [
        ObjectKind::Crystal,
        ObjectKind::PhaseBlock,
        ObjectKind::Mef,
        ObjectKind::Frontier,
        ObjectKind::ReplayPack,
        ObjectKind::ResidueReport,
        ObjectKind::Artifact,
        ObjectKind::LibraryAsset,
    ];
    let mut mem = MemoryCas::new();
    for (i, k) in kinds.iter().enumerate() {
        let payload = format!("objekt-{i}-{}", k.as_str());
        let d = mem.put(*k, payload.as_bytes());
        assert_eq!(mem.get(d).unwrap(), payload.as_bytes());
    }
    assert_eq!(mem.len(), kinds.len());
    // Dateisystem-CAS: derselbe Vertrag.
    let dir = std::env::temp_dir().join(format!("cce-cas-test-{}", std::process::id()));
    let mut fs = FsCas::open(&dir).unwrap();
    let d = fs.put(ObjectKind::Crystal, b"persistiert");
    assert!(fs.contains(d));
    assert_eq!(fs.get(d).unwrap(), b"persistiert");
    // gleicher Inhalt ⇒ gleiche Adresse (Dedup, Sync-Grundlage S9.6).
    assert_eq!(fs.put(ObjectKind::Crystal, b"persistiert"), d);
    let _ = std::fs::remove_dir_all(&dir);
}

/// Saat-Bibliothek: alle Referenzen gruen, alle Negativen rot — und der
/// Waechter zaehlt sie.
#[test]
fn seed_library_witnesses_green_and_red() {
    let reg = Registry::seed().expect("Saat-Bibliothek selbstvalidiert");
    match run_guard(&reg) {
        GuardOutcome::Green { witnesses_checked } => {
            assert!(witnesses_checked >= 5, "1 Referenz + ≥4 Negative");
        }
        GuardOutcome::Broken(b) => panic!("Waechter rot auf Saat: {b:?}"),
    }
}

/// DER Kernnachweis: der Waechter blockiert einen absichtlich
/// eingeschleusten Bruch — an beiden Verteidigungslinien.
#[test]
fn guard_blocks_injected_break() {
    let adapter = DocumentAdapter;
    // Linie 1 (Eintritt): eine "Referenz", die nicht schliesst, wird von
    // der Selbstvalidierung abgewiesen (S8.2).
    let mut reg = Registry::seed().unwrap();
    let mut broken_ref = adapter.reference_cube();
    broken_ref.units.retain(|u| u.id != "c1");
    match reg.admit(
        AssetKind::ReferenceCube,
        "eingeschleust",
        broken_ref,
        "angreifer",
        None,
    ) {
        Err(RegistryError::ReferenceDoesNotClose(reason)) => {
            assert!(reason.contains("DocG-Support"), "{reason}");
        }
        other => panic!("Eintritts-Selbstvalidierung durchbrochen: {other:?}"),
    }
    // Linie 2 (Bestand): ein bereits liegendes Referenz-Asset driftet
    // (Manipulation am Bestand, an admit vorbei). Der Waechterlauf prueft
    // JEDES Asset erneut gegen den Motor und meldet BAU ROT.
    let mut infected = Registry::seed().unwrap();
    let mut drifted = infected.get("drei-risiken-memo").unwrap().clone();
    drifted.crystal.units.retain(|u| u.id != "c2");
    infected.inject_unvalidated_for_tests(drifted);
    match run_guard(&infected) {
        GuardOutcome::Broken(breaks) => {
            assert!(
                breaks.iter().any(|b| b.contains("REFERENZ GEBROCHEN")),
                "{breaks:?}"
            );
        }
        GuardOutcome::Green { .. } => {
            panic!("Waechter hat den eingeschleusten Bruch NICHT geblockt")
        }
    }
    // Linie 2b: ein entschaerfter Negativ-Cube (gesunder Kristall unter
    // Negativ-Etikett) wird vom Waechter ebenfalls gemeldet.
    let mut weakened = Registry::seed().unwrap();
    let healthy = adapter.reference_cube();
    weakened.inject_unvalidated_for_tests(cce_library::registry::LibraryAsset {
        kind: AssetKind::NegativeCube,
        domain: "D01-document".into(),
        name: "negativ-1-uncovered_topic".into(),
        class_digest: sha256(b"x"),
        author: "drift".into(),
        validation: "MANIPULIERT".into(),
        retired: false,
        expected_residue: Some(cce_core::residue::ResidueKind::named("uncovered_topic")),
        crystal: healthy,
    });
    match run_guard(&weakened) {
        GuardOutcome::Broken(breaks) => {
            assert!(
                breaks.iter().any(|b| b.contains("ENTSCHAERFT")),
                "{breaks:?}"
            );
        }
        GuardOutcome::Green { .. } => panic!("entschaerfter Negativ-Cube unbemerkt"),
    }
}

/// „Zahnloser" Negativ-Cube wird beim Eintritt abgewiesen (S8.8).
#[test]
fn toothless_negative_cube_rejected() {
    let adapter = DocumentAdapter;
    let mut reg = Registry::seed().unwrap();
    // Ein voellig gesunder Kristall als "Negativ-Cube" deklariert:
    match reg.admit(
        AssetKind::NegativeCube,
        "zahnlos",
        adapter.reference_cube(),
        "autor",
        Some(cce_core::residue::ResidueKind::named("uncovered_topic")),
    ) {
        Err(RegistryError::NegativeNotRejected(_)) => {}
        other => panic!("zahnloser Negativ-Cube aufgenommen: {other:?}"),
    }
    // Falscher Ablehnungsgrund wird ebenfalls abgewiesen:
    let mut wrong_reason = adapter.reference_cube();
    wrong_reason.units.retain(|u| u.id != "c3"); // erzeugt unsupported_unit
    match reg.admit(
        AssetKind::NegativeCube,
        "falscher-grund",
        wrong_reason,
        "autor",
        Some(cce_core::residue::ResidueKind::named("contradiction")),
    ) {
        Err(RegistryError::WrongRejectionReason { expected, .. }) => {
            assert_eq!(expected, "contradiction");
        }
        other => panic!("falscher Grund akzeptiert: {other:?}"),
    }
}

/// Ref-Konflikt: sichtbar + operator-aufloesbar (Sync-Vorbereitung, S9.6).
#[test]
fn ref_conflict_visible_operator_resolvable() {
    use cce_store::refs::RefStore;
    let mut refs = RefStore::new();
    let a = sha256(b"geraet-a");
    let b = sha256(b"geraet-b");
    refs.set("neueste", a, None).unwrap();
    let conflict = refs.set("neueste", b, None).unwrap_err();
    assert_eq!(refs.open_conflicts.len(), 1, "Konflikt sichtbar");
    refs.resolve(&conflict, false); // Operator behaelt a
    assert_eq!(refs.get("neueste"), Some(a));
    assert!(refs.open_conflicts.is_empty());
}

/// Stilllegung ist explizit; stillgelegte Assets verlassen den Waechter
/// sichtbar (nie stilles Loeschen).
#[test]
fn retire_is_explicit() {
    let mut reg = Registry::seed().unwrap();
    assert!(reg.retire("drei-risiken-memo"));
    assert!(reg.get("drei-risiken-memo").unwrap().retired);
    // Waechter bleibt gruen (Asset sichtbar stillgelegt, nicht geloescht).
    assert!(matches!(run_guard(&reg), GuardOutcome::Green { .. }));
    let _ = UnitType::Risk; // Typ-Anker
}
