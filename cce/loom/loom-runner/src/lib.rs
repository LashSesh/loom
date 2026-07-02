//! loom-runner — Ausfuehrung unter CapabilityLocks (LOOM Teil 6/9.3).
//! Netz NUR via CSA-Kette; `run` erzeugt CandidateOutputs, NIE Commits;
//! eine Workcell, die eine NICHT DEKLARIERTE Capability fordert, ist N8
//! (capability_escalation).

use cce_core::capability::CapabilityLock;
use std::collections::BTreeMap;

pub const CAPABILITY_CLASSES: [&str; 11] = [
    "read_segment",
    "inspect_evidence",
    "project_workcell",
    "run_local_tool",
    "write_phaseblock",
    "export_artifact",
    // Aufspaltung von network_request (Overlay 05 Teil E):
    "source_acquisition",
    "model_egress",
    "tool_egress",
    "memory_commit",
    // Altform, nur read-kompatibel deklarierbar, nie aktivierbar:
    "network_request",
];

#[derive(Debug, PartialEq, Eq)]
pub enum RunError {
    CapabilityNotDeclared { requested: String },
    CapabilityLockClosed { requested: String },
    LegacyNetworkRequest,
}

/// Runner-Kontext: deklarierte Capabilities (aus MANIFEST/RUNTIME_PROFILE)
/// + lokal geoeffnete Locks (S13). Deklaration ist NIE Aktivierung.
pub struct RunnerContext {
    pub declared: Vec<String>,
    locks: BTreeMap<String, CapabilityLock>,
}

impl RunnerContext {
    pub fn new(declared: &[&str]) -> Self {
        Self {
            declared: declared.iter().map(|s| s.to_string()).collect(),
            locks: BTreeMap::new(),
        }
    }

    pub fn open_lock(&mut self, capability: &str, operator: &str, ledger_ref: &str) {
        let mut l = CapabilityLock::closed(capability);
        l.open(operator, ledger_ref);
        self.locks.insert(capability.to_string(), l);
    }

    /// Fordert eine Capability fuer einen Workcell-Lauf an.
    pub fn request(&self, capability: &str) -> Result<(), RunError> {
        if capability == "network_request" {
            // Altform: konservativ KEINE der drei Egress-Capabilities
            // ohne Neu-Deklaration (Overlay 05 Teil E).
            return Err(RunError::LegacyNetworkRequest);
        }
        if !self.declared.iter().any(|d| d == capability) {
            return Err(RunError::CapabilityNotDeclared {
                requested: capability.to_string(),
            });
        }
        match self.locks.get(capability) {
            Some(l) if l.is_open() => Ok(()),
            _ => Err(RunError::CapabilityLockClosed {
                requested: capability.to_string(),
            }),
        }
    }
}

/// RunReport: CandidateOutput, KEIN Commit (LOOM Teil 5.4).
#[derive(Debug)]
pub struct RunReport {
    pub workcell_id: String,
    pub candidate_output: String,
    pub gates_pending: bool,
}

pub fn run_workcell(
    ctx: &RunnerContext,
    workcell_id: &str,
    required_capability: &str,
    input: &str,
) -> Result<RunReport, RunError> {
    ctx.request(required_capability)?;
    Ok(RunReport {
        workcell_id: workcell_id.to_string(),
        candidate_output: format!("kandidat({workcell_id}): {input}"),
        gates_pending: true,
    })
}
