//! EnginePort (S3.1.2): die deklarierten Motor-Ports als Trait —
//! und die Referenz-Implementierung ueber cce-runner (Dokument-Domaene,
//! R-Plan-2). Alle Rueckgaben sind Motor-Fakten (read-only fuer GUI
//! und Kanzel).

use cce_core::gate::GateReport;
use cce_core::ledger::Ledger;
use cce_core::replay::RunDescriptor;
use cce_core::signature::Digest;
use cce_materialize::document::{DocArtifact, DocCrystal};
use cce_runner::runner::{Run, RunError, RunStatus};

/// Der Port: exakt die Bedienebene der Bauverfassungs-Ports.
pub trait EnginePort {
    /// Schema-/Wohlgeformtheits-Pruefung des geformten Crystals
    /// (gruen/rot MIT Grund — der Motor prueft, das Cockpit zeigt).
    fn validate_crystal(&self, crystal: &DocCrystal) -> GateReport;
    /// Startet den Lauf (nach Bestaetigung).
    fn submit(&mut self, crystal: DocCrystal, rd: RunDescriptor) -> Result<(), RunError>;
    fn run_to_end(&mut self) -> Result<(), RunError>;
    fn status(&self) -> Option<RunStatus>;
    fn gate_reports(&self) -> Vec<GateReport>;
    fn residues(&self) -> Vec<(String, String, String)>;
    fn ledger(&self) -> Option<&Ledger>;
    fn artifact(&self) -> Option<&DocArtifact>;
    /// Replay: reproduziert aus BESTAETIGTEM Crystal + RD dieselbe
    /// Klasse — der Port hat strukturell KEINEN Kanzel-Zugang
    /// (COCK-INV-5: nie LLM-Neuabfrage im Replay-Pfad).
    fn replay_class(&self) -> Option<Digest>;
}

/// Referenz-Engine ueber cce-runner.
#[derive(Default)]
pub struct MotorEngine {
    run: Option<Run>,
}

impl EnginePort for MotorEngine {
    fn validate_crystal(&self, crystal: &DocCrystal) -> GateReport {
        if crystal.units.is_empty() {
            return GateReport::hold(
                "schema:crystal",
                "Crystal ohne Einheiten — nicht wohlgeformt",
            );
        }
        if !crystal.no_score_fields {
            return GateReport::hold("schema:crystal", "Score-Felder deklariert — V1-Verstoss");
        }
        GateReport::pass("schema:crystal", "Wunsch-Normalform wohlgeformt")
    }

    fn submit(&mut self, crystal: DocCrystal, rd: RunDescriptor) -> Result<(), RunError> {
        self.run = Some(Run::submit(crystal, rd)?);
        Ok(())
    }

    fn run_to_end(&mut self) -> Result<(), RunError> {
        match &mut self.run {
            Some(r) => r.run_to_end(None),
            None => Ok(()),
        }
    }

    fn status(&self) -> Option<RunStatus> {
        self.run.as_ref().map(|r| r.status.clone())
    }

    fn gate_reports(&self) -> Vec<GateReport> {
        self.run
            .as_ref()
            .map(|r| r.gates.reports.clone())
            .unwrap_or_default()
    }

    fn residues(&self) -> Vec<(String, String, String)> {
        // (quelle, inhalt, severity) — aus den Gate-Holds abgeleitet
        // plus dem sichtbaren Leerfeld.
        self.gate_reports()
            .iter()
            .filter(|g| !g.is_pass())
            .map(|g| (g.gate_id.clone(), g.reason.clone(), "blocking".to_string()))
            .collect()
    }

    fn ledger(&self) -> Option<&Ledger> {
        self.run.as_ref().map(|r| &r.ledger)
    }

    fn artifact(&self) -> Option<&DocArtifact> {
        self.run.as_ref().and_then(|r| r.artifact.as_ref())
    }

    fn replay_class(&self) -> Option<Digest> {
        self.run.as_ref().map(|r| r.result_class())
    }
}
