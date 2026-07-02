//! PRODUKT-KERNTEST (G11-Ausgangs-Gate, S2/S10): Drei-Risiken-Memo-
//! Wunsch → Bestaetigung → Lauf → Pruefung „geschlossen (∅)" →
//! Artefakt-Export → Re-Import klassenidentisch — vollstaendig ueber
//! die Cockpit-Zustandsmaschine, ohne Konsole. Dazu: Fehl-Reisen enden
//! lesbar; Doku=Verhalten-Stichproben.

use cce_core::replay::RunDescriptor;
use cce_core::signature::sha256;
use cockpit_core::engine::{EnginePort, MotorEngine};
use cockpit_core::journey::{
    export_lossy_plaintext, reimport, replay_matches, take_artifact, ReimportVerdict,
};
use cockpit_core::kanzel::{KanzelPort, LocalKanzel};
use cockpit_core::state::{CockpitCore, CockpitState, Confirmation};
use cockpit_core::views::residue_view;

fn confirmation(action: &str) -> Option<Confirmation> {
    Some(Confirmation {
        operator: "operator:sk".into(),
        action: action.into(),
        statement: "explizit bestaetigt".into(),
    })
}

fn rd() -> RunDescriptor {
    RunDescriptor::new(sha256(b"memo-journey"), "document", 7)
}

#[test]
fn produkt_kerntest_ueber_sechs_naehte() {
    // Naht 0: Oeffnen → Wuenschen (Zustandsmaschine deterministisch).
    let mut core = CockpitCore::new(MotorEngine::default());
    assert_eq!(core.state, CockpitState::Leer);
    core.enter_wish(
        "Ich brauche ein zweiseitiges Memo, das die drei Projektrisiken \
         benennt, jede mit Gegenmassnahme, ohne Bewertungszahlen.",
    )
    .unwrap();

    // Naht 1 (BESTAETIGUNGSGRENZE): Kanzel formt (Interpretation),
    // Motor validiert, Operator bestaetigt.
    let (crystal, interp) = LocalKanzel.form_wish("drei risiken memo").unwrap();
    assert!(
        interp.text.contains("modellgeformt"),
        "Annahmen-Liste sichtbar"
    );
    core.crystal_formed(crystal).unwrap();
    core.confirm_crystal(confirmation("confirm_crystal"))
        .unwrap();

    // Naht 2: Bestaetigt → Lauf (fester RD, materielle Aktion).
    core.start_run(rd(), confirmation("start_run")).unwrap();
    assert_eq!(core.state, CockpitState::ArtefaktVerfuegbar);

    // Naht 3: Pruefung — alle Gates gruen, Residuenfeld EXPLIZIT
    // „geschlossen (∅)".
    assert!(core.engine.gate_reports().iter().all(|g| g.is_pass()));
    let residues = residue_view(&core.engine.residues());
    assert_eq!(residues[0].value, "geschlossen (∅)");

    // Naht 4: Entnehmen — Artefakt mit ZWEI Digests + Zertifikat.
    let cert = take_artifact(&mut core, confirmation("export")).unwrap();
    assert_eq!(cert.format, ".md");
    assert_ne!(cert.content_class.to_hex(), cert.byte_digest.to_hex());

    // Naht 5: Re-Import klassenidentisch.
    match reimport(&cert, &cert.bytes) {
        ReimportVerdict::SameClass { class } => {
            assert_eq!(class, cert.content_class.to_hex())
        }
        other => panic!("Re-Import muss klassenidentisch sein, war {other:?}"),
    }
    // Replay reproduziert die Klasse; Byte-Digest bindet die Datei.
    assert!(replay_matches(&core, &cert));
}

#[test]
fn aeussere_bearbeitung_bricht_zertifikat_nachweisbar() {
    let mut core = CockpitCore::new(MotorEngine::default());
    core.enter_wish("memo").unwrap();
    let (crystal, _) = LocalKanzel.form_wish("memo").unwrap();
    core.crystal_formed(crystal).unwrap();
    core.confirm_crystal(confirmation("c")).unwrap();
    core.start_run(rd(), confirmation("s")).unwrap();
    let cert = take_artifact(&mut core, confirmation("e")).unwrap();
    // Aeussere Bearbeitung: Inhaltszeile veraendern.
    let edited = String::from_utf8(cert.bytes.clone())
        .unwrap()
        .replace("Serverausfall", "Serverausfall (extern editiert)");
    match reimport(&cert, edited.as_bytes()) {
        ReimportVerdict::CertificateBroken {
            certified,
            reimported,
        } => {
            assert_ne!(certified, reimported, "beide Klassen benannt");
        }
        other => panic!("Bearbeitung muss das Zertifikat brechen, war {other:?}"),
    }
}

#[test]
fn fehlreise_formatverlust_ist_sichtbare_entscheidung() {
    let mut core = CockpitCore::new(MotorEngine::default());
    core.enter_wish("memo").unwrap();
    let (crystal, _) = LocalKanzel.form_wish("memo").unwrap();
    core.crystal_formed(crystal).unwrap();
    core.confirm_crystal(confirmation("c")).unwrap();
    core.start_run(rd(), confirmation("s")).unwrap();
    let cert = take_artifact(&mut core, confirmation("e")).unwrap();
    // Ohne Bestaetigung: KEIN verlustiger Export, Residuum benannt.
    let err = export_lossy_plaintext(&cert, None).unwrap_err();
    assert!(err.id.contains("format_loss"));
    // Mit Bestaetigung: Export MIT sichtbarem Residuum.
    let conf = confirmation("lossy").unwrap();
    let (bytes, residue) = export_lossy_plaintext(&cert, Some(&conf)).unwrap();
    assert!(residue.id.contains("format_loss"));
    assert!(!String::from_utf8_lossy(&bytes).contains("<!--cce:"));
}

#[test]
fn fehlreise_abgelehnt_ist_lesbarer_endzustand() {
    // Ein Crystal ohne Einheiten: Motor-Schema rot ⇒ zurueck mit Grund.
    let mut core = CockpitCore::new(MotorEngine::default());
    core.enter_wish("leer").unwrap();
    let mut broken = cce_materialize::document::assets::three_risks_memo();
    broken.units.clear();
    core.crystal_formed(broken).unwrap();
    match &core.state {
        CockpitState::WunschErfasst { wunsch } => assert!(wunsch.contains("abgewiesen")),
        s => panic!("erwartet benannten Nicht-Abschluss, war {s:?}"),
    }
}

// ---------- Doku=Verhalten-Stichproben (S12.3/S12-DoD⁺) ----------

fn handbuch() -> String {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    std::fs::read_to_string(root.join("docs/operator/handbuch.md")).expect("handbuch.md")
}

#[test]
fn doku_verhalten_geschlossen_leer_wird_angezeigt() {
    // Doku sagt: leeres Residuenfeld erscheint als „geschlossen (∅)".
    assert!(handbuch().contains("geschlossen (∅)"));
    // Verhalten:
    assert_eq!(residue_view(&[])[0].value, "geschlossen (∅)");
}

#[test]
fn doku_verhalten_kein_trotzdem_durchlassen() {
    // Doku sagt: es gibt keinen „Trotzdem durchlassen"-Knopf.
    assert!(handbuch().contains("Trotzdem durchlassen"));
    // Verhalten: der Pfad existiert nicht (liefert immer Fehler).
    let mut core = CockpitCore::new(MotorEngine::default());
    assert!(core.force_through().is_err());
}

#[test]
fn doku_verhalten_kanzel_markiert_und_degradierbar() {
    let text = handbuch();
    assert!(text.contains("Interpretation, kein Motor-Urteil"));
    assert!(text.contains("sichtbar degradiert"));
    // Verhalten:
    use cockpit_core::kanzel::{DegradedKanzel, INTERPRETATION_MARKER};
    assert!(DegradedKanzel.form_wish("x").is_none());
    let (_c, i) = LocalKanzel.form_wish("x").unwrap();
    assert_eq!(i.marker, INTERPRETATION_MARKER);
}

#[test]
fn doku_lesarten_vollstaendig_und_unter_claim_schranke() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let text = std::fs::read_to_string(root.join("docs/operator/lesarten.md")).unwrap();
    for lesart in [
        "Phase & BlueCube",
        "Skala & RedCube",
        "Ratchet & Frontier",
        "HBM-Kandidat",
    ] {
        assert!(text.contains(lesart), "Lesart '{lesart}' fehlt");
    }
    assert!(text.contains("Nexus"), "Nexus-Ausblick fehlt");
    assert!(text.contains("kein Funktionsversprechen"));
    assert!(text.contains("Claim-Schranke"));
}
