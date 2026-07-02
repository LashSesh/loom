//! cce-cockpit — native Desktop-App (S3.1.1: reines Rust, egui, kein
//! Webview). Vier Flaechen: Wunsch / Lauf / Pruef / Artefakt.
//! Das GUI RENDERT Motor-Fakten (CockpitCore) — es erzeugt keine.

use eframe::egui;

use cce_core::replay::RunDescriptor;
use cce_core::signature::sha256;
use cockpit_core::engine::{EnginePort, MotorEngine};
use cockpit_core::kanzel::{DegradedKanzel, KanzelPort, LocalKanzel};
use cockpit_core::state::{CockpitCore, CockpitState, Confirmation};
use cockpit_core::views::{gate_report_view, residue_view};

struct CockpitApp {
    core: CockpitCore<MotorEngine>,
    kanzel_on: bool,
    wish_input: String,
    tab: Tab,
    kanzel_text: String,
}

#[derive(PartialEq)]
enum Tab {
    Wunsch,
    Lauf,
    Pruef,
    Artefakt,
}

impl Default for CockpitApp {
    fn default() -> Self {
        Self {
            core: CockpitCore::new(MotorEngine::default()),
            kanzel_on: true,
            wish_input: String::new(),
            tab: Tab::Wunsch,
            kanzel_text: String::new(),
        }
    }
}

impl CockpitApp {
    fn confirmation(&self, action: &str) -> Option<Confirmation> {
        Some(Confirmation {
            operator: "operator:cockpit".into(),
            action: action.into(),
            statement: "explizit bestaetigt (Dialog)".into(),
        })
    }
}

impl eframe::App for CockpitApp {
    fn ui(&mut self, root: &mut egui::Ui, _frame: &mut eframe::Frame) {
        {
            let ui = &mut *root;
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.tab, Tab::Wunsch, "Wunsch");
                ui.selectable_value(&mut self.tab, Tab::Lauf, "Lauf");
                ui.selectable_value(&mut self.tab, Tab::Pruef, "Pruefung");
                ui.selectable_value(&mut self.tab, Tab::Artefakt, "Artefakt");
                ui.separator();
                ui.checkbox(&mut self.kanzel_on, "Kanzel");
                let status = if self.kanzel_on {
                    LocalKanzel.status()
                } else {
                    DegradedKanzel.status()
                };
                ui.label(format!("Kanzel: {status}"));
            });
            ui.separator();
        }
        let ui = root;
        match self.tab {
            Tab::Wunsch => {
                ui.heading("Wunsch-Flaeche");
                ui.text_edit_multiline(&mut self.wish_input);
                if ui.button("Wunsch erfassen").clicked() {
                    let _ = self.core.enter_wish(&self.wish_input);
                }
                if ui.button("Kanzel formt Crystal").clicked() {
                    let formed = if self.kanzel_on {
                        LocalKanzel.form_wish(&self.wish_input)
                    } else {
                        DegradedKanzel.form_wish(&self.wish_input)
                    };
                    match formed {
                        Some((crystal, interp)) => {
                            self.kanzel_text = format!("[{}] {}", interp.marker, interp.text);
                            let _ = self.core.crystal_formed(crystal);
                        }
                        None => {
                            self.kanzel_text =
                                "Kanzel degradiert — bitte Crystal manuell formen".to_string();
                        }
                    }
                }
                if !self.kanzel_text.is_empty() {
                    ui.label(&self.kanzel_text);
                }
                if matches!(self.core.state, CockpitState::CrystalGeformt { .. })
                    && ui.button("BESTAETIGEN (materielle Aktion)").clicked()
                {
                    let c = self.confirmation("confirm_crystal");
                    let _ = self.core.confirm_crystal(c);
                }
                ui.label(format!("Zustand: {:?}", self.core.state));
            }
            Tab::Lauf => {
                ui.heading("Lauf-Flaeche");
                if self.core.state == CockpitState::Bestaetigt
                    && ui.button("Lauf STARTEN (materielle Aktion)").clicked()
                {
                    let rd = RunDescriptor::new(sha256(b"cockpit-run"), "document", 7);
                    let c = self.confirmation("start_run");
                    let _ = self.core.start_run(rd, c);
                }
                ui.label(format!("Zustand: {:?}", self.core.state));
            }
            Tab::Pruef => {
                ui.heading("Pruef-Flaeche");
                for item in gate_report_view(&self.core.engine.gate_reports()) {
                    ui.label(format!(
                        "{}: {}  [{}]",
                        item.label, item.value, item.source_ref
                    ));
                }
                ui.separator();
                for item in residue_view(&self.core.engine.residues()) {
                    ui.label(format!(
                        "{}: {}  [{}]",
                        item.label, item.value, item.source_ref
                    ));
                }
                if let Some(class) = self.core.replay_class_hex() {
                    if ui.button("identisch wiederholen (Replay)").clicked() {
                        // Replay reproduziert dieselbe Klasse (COCK-INV-5).
                    }
                    ui.label(format!("Commit-Klasse: {class}"));
                }
                // Bewusst KEIN "Trotzdem durchlassen"-Knopf (V7).
            }
            Tab::Artefakt => {
                ui.heading("Artefakt-Flaeche");
                if self.core.state == CockpitState::ArtefaktVerfuegbar {
                    ui.label("Artefakt verfuegbar (content-adressiert, zertifiziert).");
                    if ui.button("Exportieren (materielle Aktion)").clicked() {
                        let c = self.confirmation("export_artifact");
                        let _ = self.core.export_artifact(c);
                    }
                } else {
                    ui.label(format!(
                        "Noch kein Artefakt — Zustand: {:?}",
                        self.core.state
                    ));
                }
            }
        }
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "CCE Cockpit",
        options,
        Box::new(|_cc| Ok(Box::new(CockpitApp::default()))),
    )
}
