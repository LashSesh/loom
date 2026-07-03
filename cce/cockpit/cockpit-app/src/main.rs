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

/// Headless Datei-Export (#26): faehrt die Referenzreise (Drei-Risiken-
/// Memo) ueber cockpit-core OHNE GUI und schreibt das materialisierte
/// Artefakt + Zertifikat. Derselbe Zustandsmaschinen-Pfad wie im
/// Cockpit — nur ohne Fenster (fuer Umgebungen ohne Display/GPU).
fn export_headless(path: &std::path::Path) -> Result<(), String> {
    use cockpit_core::journey::take_artifact;
    let mut core = CockpitCore::new(MotorEngine::default());
    core.enter_wish("Drei-Risiken-Memo, je Gegenmassnahme, ohne Bewertungszahlen")
        .map_err(|e| format!("{e:?}"))?;
    let (crystal, _interp) = LocalKanzel
        .form_wish("drei risiken memo")
        .ok_or("Kanzel degradiert")?;
    core.crystal_formed(crystal).map_err(|e| format!("{e:?}"))?;
    let conf = |a: &str| {
        Some(Confirmation {
            operator: "operator:cli".into(),
            action: a.into(),
            statement: "headless export bestaetigt".into(),
        })
    };
    core.confirm_crystal(conf("confirm_crystal"))
        .map_err(|e| format!("{e:?}"))?;
    core.start_run(
        RunDescriptor::new(sha256(b"headless-export"), "document", 7),
        conf("start_run"),
    )
    .map_err(|e| format!("{e:?}"))?;
    if core.state != CockpitState::ArtefaktVerfuegbar {
        return Err(format!("kein Artefakt: Zustand {:?}", core.state));
    }
    let cert = take_artifact(&mut core, conf("export")).map_err(|e| format!("{e:?}"))?;
    cert.write_to(path)
        .map_err(|e| format!("Schreibfehler: {e}"))?;
    println!(
        "geschrieben: {} ({} Bytes) · content_class {} · byte_digest {}",
        path.display(),
        cert.bytes.len(),
        cert.content_class.to_hex(),
        cert.byte_digest.to_hex()
    );
    Ok(())
}

fn main() -> eframe::Result {
    // Headless-Export-Modus: `cce-cockpit --export <pfad>` (kein Fenster).
    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 3 && args[1] == "--export" {
        match export_headless(std::path::Path::new(&args[2])) {
            Ok(()) => return Ok(()),
            Err(e) => {
                eprintln!("export fehlgeschlagen: {e}");
                std::process::exit(1);
            }
        }
    }
    let mut options = eframe::NativeOptions::default();
    // Default bleibt glow; COCKPIT_RENDERER=wgpu waehlt den Vulkan-Pfad
    // (WO-2-Test auf Software-Vulkan). Kein Verhaltenswechsel ohne die Var.
    if std::env::var("COCKPIT_RENDERER").as_deref() == Ok("wgpu") {
        options.renderer = eframe::Renderer::Wgpu;
    }
    eframe::run_native(
        "CCE Cockpit",
        options,
        Box::new(|_cc| Ok(Box::new(CockpitApp::default()))),
    )
}
