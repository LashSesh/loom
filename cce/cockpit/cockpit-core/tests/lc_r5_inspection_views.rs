//! Zeuge fuer das GUI-Feindesign (Block 2, LC-R5): die fuenf
//! Pflichtansichten (Manifest, Segmentliste+Digest-Status, Residuen+
//! Verdikt, Gate-Reports, Ledger/PhaseBlocks) liefern echte,
//! wurzel-rueckfuehrbare Werte aus einem realen Lauf — read-only, ohne
//! Ausfuehrung/Netz/Schreibpfad ausgeloest durch die Ansicht selbst.

use cce_core::canonical::Canonicalize;
use cce_core::ledger::Ledger;
use cce_core::replay::RunDescriptor;
use cce_core::signature::sha256;
use cockpit_core::engine::{EnginePort, MotorEngine};
use cockpit_core::kanzel::{KanzelPort, LocalKanzel};
use cockpit_core::state::{CockpitCore, CockpitState, Confirmation};
use cockpit_core::views::{
    gate_report_view, ledger_view, manifest_view, residue_view, segment_list_view, verdict_view,
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

#[test]
fn all_five_lc_r5_views_populate_from_a_real_run() {
    let mut core = CockpitCore::new(MotorEngine::default());
    core.enter_wish("Memo mit drei Projektrisiken, je Gegenmassnahme, ohne Bewertungszahlen")
        .unwrap();
    let (crystal, _interp) = LocalKanzel.form_wish("drei risiken memo").unwrap();
    core.crystal_formed(crystal).unwrap();
    core.confirm_crystal(confirmation("confirm_crystal"))
        .unwrap();

    // -- Vor dem Lauf (Naht 1 fertig, Naht 2 noch offen) --
    let confirmed = core.confirmed_crystal.clone().expect("bestaetigt");
    let pl = cce_materialize::catalog::by_id("D01").map(|e| e.level.as_str());
    assert_eq!(
        pl,
        Some("PL4"),
        "D01 ist die eine Katalog-Wahrheit fuer PL4"
    );
    let class_hex = confirmed.canonical_class().0.to_hex();
    let manifest = manifest_view("document/D01", pl.unwrap(), &class_hex, ".md");
    assert!(manifest.iter().any(|i| i.label == "PL" && i.value == "PL4"));
    assert!(manifest
        .iter()
        .any(|i| i.label == "Klasse" && i.value.contains(&class_hex)));

    let pre_run_segments = segment_list_view(&confirmed, None);
    assert_eq!(pre_run_segments.len(), confirmed.units.len() + 1);
    assert!(pre_run_segments
        .last()
        .unwrap()
        .value
        .contains("noch kein Artefakt-Digest"));
    assert!(core.engine.ledger().is_none(), "vor start_run kein Ledger");

    core.start_run(rd(), confirmation("start_run")).unwrap();
    assert_eq!(core.state, CockpitState::ArtefaktVerfuegbar);

    // -- Kategorie 1: Manifest --
    for item in &manifest {
        assert!(!item.source_ref.is_empty());
    }

    // -- Kategorie 2: Segmentliste mit Digest-Status --
    let byte_digest_hex = core.engine.artifact().map(|a| a.byte_digest().to_hex());
    assert!(
        byte_digest_hex.is_some(),
        "nach dem Lauf gibt es ein Artefakt"
    );
    let segments = segment_list_view(&confirmed, byte_digest_hex.as_deref());
    assert_eq!(segments.len(), confirmed.units.len() + 1);
    for u in &confirmed.units {
        assert!(segments
            .iter()
            .any(|s| s.label == format!("Segment {}", u.id)));
    }
    assert!(segments
        .last()
        .unwrap()
        .value
        .contains(byte_digest_hex.as_deref().unwrap()));

    // -- Kategorie 3: Residuen + Verdikt --
    let gate_reports = core.engine.gate_reports();
    assert!(!gate_reports.is_empty(), "ein echter Lauf hat Gate-Reports");
    let verdict = verdict_view(&gate_reports);
    assert!(verdict.value.starts_with("Valid"), "{}", verdict.value);
    let residues = residue_view(&core.engine.residues());
    assert_eq!(residues[0].value, "geschlossen (∅)");

    // -- Kategorie 4: Gate-Reports --
    let gates = gate_report_view(&gate_reports);
    assert!(!gates.is_empty());
    for item in &gates {
        assert!(item.source_ref.starts_with("gate:"));
    }

    // -- Kategorie 5: Ledger/PhaseBlocks --
    let ledger = core
        .engine
        .ledger()
        .expect("nach dem Lauf gibt es einen Ledger");
    let ledger_items = ledger_view(ledger);
    assert!(
        ledger_items.len() > 1,
        "ein echter Lauf traegt mehrere Kettenpositionen: {ledger_items:?}"
    );
    let chain_status = ledger_items.last().unwrap();
    assert_eq!(chain_status.label, "Kettenstatus");
    assert!(chain_status.value.starts_with("gruen"));
    // Mindestens decode/materialize/commit muessen als Kettenpositionen
    // sichtbar sein (echte Motor-Ledger-Eintraege, keine Attrappe;
    // LedgerEventKind::as_str() ist kleingeschrieben).
    for kind in ["decode", "materialize", "commit"] {
        assert!(
            ledger_items.iter().any(|i| i.label.contains(kind)),
            "Ledger-Ereignis {kind} fehlt: {ledger_items:?}"
        );
    }
}

#[test]
fn ledger_view_on_fresh_ledger_shows_green_chain_no_entries() {
    let l = Ledger::new();
    let items = ledger_view(&l);
    assert_eq!(
        items.len(),
        1,
        "nur die Kettenstatus-Zeile, keine Eintraege"
    );
    assert_eq!(items[0].label, "Kettenstatus");
    assert!(items[0].value.starts_with("gruen"));
}

#[test]
fn verdict_view_reflects_red_gate_honestly() {
    use cce_core::gate::GateReport;
    let reports = vec![
        GateReport::pass("G1", "ok"),
        GateReport::hold("G2", "Naht fehlt"),
    ];
    let v = verdict_view(&reports);
    assert!(v.value.contains("Hold"), "{}", v.value);
    assert!(v.value.contains("1 von 2"), "{}", v.value);
}
