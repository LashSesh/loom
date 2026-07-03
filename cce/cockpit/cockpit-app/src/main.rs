//! cce-cockpit — native Desktop-App (S3.1.1: reines Rust, egui, kein
//! Webview). Vier Flaechen: Wunsch / Lauf / Pruef / Artefakt.
//! Das GUI RENDERT Motor-Fakten (CockpitCore) — es erzeugt keine.

use eframe::egui;

use cce_core::canonical::Canonicalize;
use cce_core::replay::RunDescriptor;
use cce_core::signature::sha256;
use cockpit_core::engine::{EnginePort, MotorEngine};
use cockpit_core::kanzel::{DegradedKanzel, KanzelPort, LocalKanzel};
use cockpit_core::state::{CockpitCore, CockpitState, Confirmation};
use cockpit_core::views::{
    gate_report_view, ledger_view, manifest_view, residue_view, segment_list_view, verdict_view,
};

struct CockpitApp {
    core: CockpitCore<MotorEngine>,
    kanzel_on: bool,
    wish_input: String,
    tab: Tab,
    kanzel_text: String,
    export_status: String,
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
            export_status: String::new(),
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

    /// Wie `debug_rect`: reine UI-Introspektion, kein Motor-/Gate-Bezug.
    /// Protokolliert den Zustand NACH einer Aktion, damit ein Klick-
    /// Durchlauf ohne sichtbare Glyphen (Software-Renderer-Befund)
    /// trotzdem verifizierbar bleibt.
    fn debug_state(&self, after: &str) {
        if std::env::var("COCKPIT_DEBUG_RECTS").is_ok() {
            eprintln!("STATE after {after}: {:?}", self.core.state);
        }
    }
}

/// Optionale UI-Introspektion (kein Motor-/Gate-Bezug, rein fuer
/// Automatisierung/Barrierefreiheits-Tools): unter
/// `COCKPIT_DEBUG_RECTS=1` protokolliert jeder interaktive
/// Kontrollpunkt sein Klick-Rechteck nach stderr. Notwendig, weil unter
/// manchen Software-Renderern (bestaetigter Befund, s. reports/ux/)
/// Glyphen nicht rasterisieren und Widgets dadurch ohne sichtbare
/// Beschriftung erscheinen — die Koordinaten bleiben trotzdem exakt
/// bestimmbar.
fn debug_rect(name: &str, rect: egui::Rect) {
    if std::env::var("COCKPIT_DEBUG_RECTS").is_ok() {
        eprintln!(
            "RECT {name} x={:.0}..{:.0} y={:.0}..{:.0} center=({:.0},{:.0})",
            rect.min.x,
            rect.max.x,
            rect.min.y,
            rect.max.y,
            rect.center().x,
            rect.center().y
        );
    }
}

impl eframe::App for CockpitApp {
    fn ui(&mut self, root: &mut egui::Ui, _frame: &mut eframe::Frame) {
        {
            let ui = &mut *root;
            ui.horizontal(|ui| {
                debug_rect(
                    "tab:Wunsch",
                    ui.selectable_value(&mut self.tab, Tab::Wunsch, "Wunsch")
                        .rect,
                );
                debug_rect(
                    "tab:Lauf",
                    ui.selectable_value(&mut self.tab, Tab::Lauf, "Lauf").rect,
                );
                debug_rect(
                    "tab:Pruefung",
                    ui.selectable_value(&mut self.tab, Tab::Pruef, "Pruefung")
                        .rect,
                );
                debug_rect(
                    "tab:Artefakt",
                    ui.selectable_value(&mut self.tab, Tab::Artefakt, "Artefakt")
                        .rect,
                );
                ui.separator();
                debug_rect(
                    "checkbox:Kanzel",
                    ui.checkbox(&mut self.kanzel_on, "Kanzel").rect,
                );
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
                debug_rect(
                    "wish_input",
                    ui.text_edit_multiline(&mut self.wish_input).rect,
                );
                let btn = ui.button("Wunsch erfassen");
                debug_rect("button:Wunsch erfassen", btn.rect);
                if btn.clicked() {
                    let _ = self.core.enter_wish(&self.wish_input);
                    self.debug_state("enter_wish");
                }
                let btn = ui.button("Kanzel formt Crystal");
                debug_rect("button:Kanzel formt Crystal", btn.rect);
                if btn.clicked() {
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
                    self.debug_state("kanzel_formt_crystal");
                }
                if !self.kanzel_text.is_empty() {
                    ui.label(&self.kanzel_text);
                }
                if matches!(self.core.state, CockpitState::CrystalGeformt { .. }) {
                    let btn = ui.button("BESTAETIGEN (materielle Aktion)");
                    debug_rect("button:BESTAETIGEN", btn.rect);
                    if btn.clicked() {
                        let c = self.confirmation("confirm_crystal");
                        let _ = self.core.confirm_crystal(c);
                        self.debug_state("confirm_crystal");
                    }
                }
                ui.label(format!("Zustand: {:?}", self.core.state));
            }
            Tab::Lauf => {
                ui.heading("Lauf-Flaeche");
                if self.core.state == CockpitState::Bestaetigt {
                    let btn = ui.button("Lauf STARTEN (materielle Aktion)");
                    debug_rect("button:Lauf STARTEN", btn.rect);
                    if btn.clicked() {
                        let rd = RunDescriptor::new(sha256(b"cockpit-run"), "document", 7);
                        let c = self.confirmation("start_run");
                        let _ = self.core.start_run(rd, c);
                        self.debug_state("start_run");
                    }
                }
                ui.label(format!("Zustand: {:?}", self.core.state));
            }
            Tab::Pruef => {
                // GUI-Feindesign LC-R5: fuenf Pflichtansichten, read-only,
                // ohne Ausfuehrung/Netz/Schreibpfad — bezogen auf DIESEN
                // Arbeitskoerper (Dokument-Domaene-Lauf), s. views.rs.
                ui.heading("Pruef-Flaeche — fuenf Pflichtansichten (LC-R5)");
                let gate_reports = self.core.engine.gate_reports();

                ui.label(egui::RichText::new("1. Manifest").strong());
                match &self.core.confirmed_crystal {
                    Some(crystal) => {
                        let pl = cce_materialize::catalog::by_id("D01")
                            .map(|e| e.level.as_str())
                            .unwrap_or("PL?");
                        let class_hex = crystal.canonical_class().0.to_hex();
                        for item in manifest_view("document/D01", pl, &class_hex, ".md") {
                            ui.label(format!(
                                "{}: {}  [{}]",
                                item.label, item.value, item.source_ref
                            ));
                        }
                    }
                    None => {
                        ui.label("Manifest: noch kein bestaetigter Crystal (Naht 1 ausstehend)");
                    }
                }
                ui.separator();

                ui.label(egui::RichText::new("2. Segmentliste mit Digest-Status").strong());
                match &self.core.confirmed_crystal {
                    Some(crystal) => {
                        let byte_digest_hex = self
                            .core
                            .engine
                            .artifact()
                            .map(|a| a.byte_digest().to_hex());
                        for item in segment_list_view(crystal, byte_digest_hex.as_deref()) {
                            ui.label(format!(
                                "{}: {}  [{}]",
                                item.label, item.value, item.source_ref
                            ));
                        }
                    }
                    None => {
                        ui.label("Segmentliste: noch kein bestaetigter Crystal");
                    }
                }
                ui.separator();

                ui.label(egui::RichText::new("3. Residuen + Verdikt").strong());
                let verdict = verdict_view(&gate_reports);
                ui.label(format!(
                    "{}: {}  [{}]",
                    verdict.label, verdict.value, verdict.source_ref
                ));
                for item in residue_view(&self.core.engine.residues()) {
                    ui.label(format!(
                        "{}: {}  [{}]",
                        item.label, item.value, item.source_ref
                    ));
                }
                ui.separator();

                ui.label(egui::RichText::new("4. Gate-Reports").strong());
                for item in gate_report_view(&gate_reports) {
                    ui.label(format!(
                        "{}: {}  [{}]",
                        item.label, item.value, item.source_ref
                    ));
                }
                ui.separator();

                ui.label(egui::RichText::new("5. Ledger/PhaseBlocks").strong());
                match self.core.engine.ledger() {
                    Some(ledger) => {
                        for item in ledger_view(ledger) {
                            ui.label(format!(
                                "{}: {}  [{}]",
                                item.label, item.value, item.source_ref
                            ));
                        }
                    }
                    None => {
                        ui.label("Ledger: noch kein Lauf (Naht 2 ausstehend)");
                    }
                }
                ui.separator();

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
                    let btn = ui.button("Exportieren (materielle Aktion)");
                    debug_rect("button:Exportieren", btn.rect);
                    if btn.clicked() {
                        let c = self.confirmation("export_artifact");
                        // take_artifact() vollzieht die materielle Aktion
                        // (Bestaetigung + Ledger-Verankerung, wie zuvor)
                        // UND liefert die Bytes, die write_to() REAL auf
                        // Platte schreibt — kein reiner Ledger-Eintrag mehr.
                        match cockpit_core::journey::take_artifact(&mut self.core, c) {
                            Ok(cert) => {
                                let dir = std::env::var("COCKPIT_EXPORT_DIR")
                                    .unwrap_or_else(|_| "./cockpit-exports".to_string());
                                if let Err(e) = std::fs::create_dir_all(&dir) {
                                    self.export_status = format!("Verzeichnisfehler: {e}");
                                } else {
                                    let hash12 = &cert.byte_digest.to_hex()[..12];
                                    let filename = format!("export-{hash12}{}", cert.format);
                                    let path = std::path::Path::new(&dir).join(&filename);
                                    match cert.write_to(&path) {
                                        Ok(()) => {
                                            self.export_status = format!(
                                                "exportiert: {} ({} Bytes, byte_digest {}...)",
                                                path.display(),
                                                cert.bytes.len(),
                                                hash12
                                            );
                                        }
                                        Err(e) => {
                                            self.export_status = format!("Schreibfehler: {e}");
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                self.export_status = format!("Export-Fehler: {e:?}");
                            }
                        }
                    }
                    if !self.export_status.is_empty() {
                        ui.label(&self.export_status);
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
