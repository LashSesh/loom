//! COCK-INV-1..8 als Tests (G10-Ausgangs-Gate) + Wurzel-Rueckfuehrung
//! + Kanzel-Aus-Modus + „kein Trotzdem durchlassen".

use cce_core::gate::GateReport;
use cce_core::replay::RunDescriptor;
use cce_core::signature::sha256;
use cce_core::value::CanonValue;
use cce_materialize::document::assets::three_risks_memo;
use cockpit_core::engine::{EnginePort, MotorEngine};
use cockpit_core::kanzel::{DegradedKanzel, KanzelPort, LocalKanzel, INTERPRETATION_MARKER};
use cockpit_core::state::{CockpitCore, CockpitError, CockpitState, Confirmation};
use cockpit_core::views::{
    candidate_output_view, gate_report_view, hbm_board, provider_status_view, residue_view,
    RANKING_LABEL,
};

fn confirmation(action: &str) -> Option<Confirmation> {
    Some(Confirmation {
        operator: "operator:sk".into(),
        action: action.into(),
        statement: "geprueft und bestaetigt".into(),
    })
}

fn rd() -> RunDescriptor {
    RunDescriptor::new(sha256(b"memo"), "document", 7)
}

/// Voller Bedienpfad: Wunsch → geformt → bestaetigt → Lauf →
/// Artefakt — vollstaendig ueber die Zustandsmaschine.
fn drive_to_artifact() -> CockpitCore<MotorEngine> {
    let mut core = CockpitCore::new(MotorEngine::default());
    core.enter_wish("Memo mit drei Projektrisiken, je Gegenmassnahme, ohne Bewertungszahlen")
        .unwrap();
    let kanzel = LocalKanzel;
    let (crystal, interp) = kanzel.form_wish("drei risiken memo").unwrap();
    assert_eq!(interp.marker, INTERPRETATION_MARKER);
    core.crystal_formed(crystal).unwrap();
    core.confirm_crystal(confirmation("confirm_crystal"))
        .unwrap();
    core.start_run(rd(), confirmation("start_run")).unwrap();
    core
}

#[test]
fn full_journey_reaches_artifact() {
    let core = drive_to_artifact();
    assert_eq!(core.state, CockpitState::ArtefaktVerfuegbar);
}

#[test]
fn cock_inv_1_no_gui_path_creates_or_changes_verdicts() {
    // Die Ansicht entsteht NUR aus Motor-GateReports; ein rotes Gate
    // bleibt in der Anzeige rot samt Begruendung — es gibt keinen
    // Manipulationsweg (ViewItem hat keinen Urteils-Setter; der
    // Zustandsuebergang liest ausschliesslich engine.gate_reports()).
    let red = GateReport::hold("G3", "Risiko 2 ohne Gegenmassnahme-Naht");
    let view = gate_report_view(std::slice::from_ref(&red));
    assert!(view[0].value.starts_with("rot"));
    assert!(view[0].value.contains("Gegenmassnahme"));
    // Und: Score-als-Gate wird strukturell abgewiesen (V1).
    let mut m = std::collections::BTreeMap::new();
    m.insert("gate_id".to_string(), CanonValue::Text("g".into()));
    m.insert("score".to_string(), CanonValue::Int(999));
    assert!(GateReport::from_untyped(&CanonValue::Map(m)).is_err());
}

#[test]
fn cock_inv_2_no_residue_hidden_empty_shown_explicitly() {
    // Leer wird EXPLIZIT gezeigt:
    let empty = residue_view(&[]);
    assert_eq!(empty.len(), 1);
    assert_eq!(empty[0].value, "geschlossen (∅)");
    // Nicht-leer: jedes Residuum erscheint, mit Severity:
    let residues = vec![(
        "G3".to_string(),
        "Naht fehlt".to_string(),
        "blocking".to_string(),
    )];
    let view = residue_view(&residues);
    assert_eq!(view.len(), 1);
    assert!(view[0].value.contains("[blocking]"));
}

#[test]
fn cock_inv_3_material_actions_require_confirmation() {
    let mut core = CockpitCore::new(MotorEngine::default());
    core.enter_wish("wunsch").unwrap();
    let (crystal, _) = LocalKanzel.form_wish("w").unwrap();
    core.crystal_formed(crystal).unwrap();
    // Ohne Bestaetigung: keine materielle Aktion.
    assert_eq!(
        core.confirm_crystal(None),
        Err(CockpitError::ConfirmationRequired {
            action: "confirm_crystal".into()
        })
    );
    core.confirm_crystal(confirmation("confirm_crystal"))
        .unwrap();
    assert_eq!(
        core.start_run(rd(), None),
        Err(CockpitError::ConfirmationRequired {
            action: "start_run".into()
        })
    );
    core.start_run(rd(), confirmation("start_run")).unwrap();
    // Export ebenso:
    assert!(matches!(
        core.export_artifact(None),
        Err(CockpitError::ConfirmationRequired { .. })
    ));
    assert!(core.export_artifact(confirmation("export")).is_ok());
    // Alle Bestaetigungen sind AUFGEZEICHNET:
    assert_eq!(core.confirmations.len(), 3);
}

#[test]
fn cock_inv_4_kanzel_output_always_marked_interpretation() {
    let k = LocalKanzel;
    let (_c, interp) = k.form_wish("w").unwrap();
    assert_eq!(interp.marker, INTERPRETATION_MARKER);
    let g = GateReport::hold("G4", "residuum offen");
    assert_eq!(k.explain_gate(&g).marker, INTERPRETATION_MARKER);
    assert_eq!(
        DegradedKanzel.explain_gate(&g).marker,
        INTERPRETATION_MARKER
    );
}

#[test]
fn cock_inv_5_replay_from_crystal_and_rd_never_llm() {
    // Zwei unabhaengige Laeufe aus demselben bestaetigten Crystal + RD
    // liefern dieselbe Klasse — der Replay-Pfad (EnginePort::replay_class)
    // hat keinerlei Kanzel-/LLM-Parameter (typsystemisch belegt).
    let a = drive_to_artifact();
    let b = drive_to_artifact();
    assert_eq!(a.replay_class_hex(), b.replay_class_hex());
    assert!(a.replay_class_hex().is_some());
}

#[test]
fn cock_inv_6_no_ui_path_makes_score_a_decision() {
    let rows = vec![
        ("c1".to_string(), "Pass".to_string(), 1),
        (
            "c2".to_string(),
            "Hold: exclusion_fail(widerspruch)".to_string(),
            2,
        ),
    ];
    let board = hbm_board(&rows);
    // Ranking-Spalte traegt IMMER das feste Label:
    for row in &board {
        assert!(row.ranking_display.contains(RANKING_LABEL));
    }
    // Status kommt NUR vom Gate — ein besser gerankter Kandidat mit
    // Hold bleibt Hold (die Struktur hat keinen Pfad Rang→Status).
    assert!(board[1].gate_status.starts_with("Hold"));
}

#[test]
fn cock_inv_7_no_ui_path_activates_provider_without_confirmation() {
    // Provider-Aktivierung ist eine materielle Aktion: der einzige
    // Aktivierungsweg (CapabilityLock::open) verlangt Operator +
    // Ledger-Ref; das Cockpit besitzt keinen Auto-Aktivierungscode —
    // ein geschlossener Lock haelt das ModelCapabilityGate.
    use cce_core::capability::CapabilityLock;
    use cce_inference::gates::model_capability_gate;
    use cce_inference::manifest::{ModelManifest, ProviderClass};
    let m = ModelManifest::complete("cloud", ProviderClass::CloudModel, "m");
    let closed = CapabilityLock::closed("model_egress:cloud");
    assert!(!model_capability_gate(&m, Some(&closed), "draft").allows());
    let mut opened = CapabilityLock::closed("model_egress:cloud");
    opened.open("operator:sk", "ledger:e77"); // Bestaetigung + Ledger-Anker
    assert!(model_capability_gate(&m, Some(&opened), "draft").allows());
}

#[test]
fn cock_inv_8_confidence_never_rendered_as_verdict() {
    let view = candidate_output_view(
        "c1",
        "local:kernmodell",
        "G1 gruen",
        "→PhaseBlock",
        true,
        Some(940),
    );
    let assessment = view.iter().find(|i| i.label == "Einschaetzung").unwrap();
    assert!(assessment.value.contains("Einschaetzung, kein Urteil"));
    // Kein ViewItem stellt die Zahl als Gate-/Verdikt-Feld dar:
    assert!(view.iter().all(|i| !i.value.contains("verdict")));
}

#[test]
fn no_force_through_button_exists() {
    // Harte UI-Regel (S3.2.3): kein „Trotzdem durchlassen".
    let mut core = drive_to_artifact();
    assert_eq!(core.force_through(), Err(CockpitError::NoOverridePath));
}

#[test]
fn rejected_is_honest_end_state_with_reason() {
    // Ein Crystal ohne Einheiten faellt am Motor-Schema: zurueck zu
    // WUNSCH_ERFASST mit Grund (kein verstecktes Scheitern).
    let mut core = CockpitCore::new(MotorEngine::default());
    core.enter_wish("leerer wunsch").unwrap();
    let mut broken = three_risks_memo();
    broken.units.clear();
    core.crystal_formed(broken).unwrap();
    match &core.state {
        CockpitState::WunschErfasst { wunsch } => {
            assert!(
                wunsch.contains("abgewiesen"),
                "Grund muss sichtbar sein: {wunsch}"
            )
        }
        s => panic!("erwartet WunschErfasst mit Grund, war {s:?}"),
    }
}

#[test]
fn kanzel_off_mode_fully_functional() {
    // Kanzel-Aus (DegradedKanzel): form liefert None — der Operator
    // formt selbst; der volle Pfad bleibt begehbar.
    let mut core = CockpitCore::new(MotorEngine::default());
    core.enter_wish("selbst geformt").unwrap();
    assert!(
        DegradedKanzel.form_wish("x").is_none(),
        "sichtbar degradiert"
    );
    core.crystal_formed(three_risks_memo()).unwrap(); // Operator-geformt
    core.confirm_crystal(confirmation("confirm")).unwrap();
    core.start_run(rd(), confirmation("start")).unwrap();
    assert_eq!(core.state, CockpitState::ArtefaktVerfuegbar);
}

#[test]
fn views_are_root_traceable_sample() {
    // Stichproben-Test (G10-Gate): jede Anzeige traegt source_ref auf
    // ein Motor-Artefakt.
    let core = drive_to_artifact();
    let gates = gate_report_view(&core.engine.gate_reports());
    assert!(!gates.is_empty());
    for item in &gates {
        assert!(item.source_ref.starts_with("gate:"), "{:?}", item);
    }
    let residues = residue_view(&core.engine.residues());
    for item in &residues {
        assert!(item.source_ref.starts_with("residue"), "{:?}", item);
    }
    let m = cce_inference::manifest::ModelManifest::complete(
        "local:kernmodell",
        cce_inference::manifest::ProviderClass::LocalModel,
        "kernmodell",
    );
    let ps = provider_status_view("fully_local", &m, LocalKanzel.status());
    for item in &ps {
        assert!(!item.source_ref.is_empty());
    }
}
