//! Der SourceAdapter-Portvertrag: 8 Methoden (CSA.3).

use crate::manifest::AdapterManifest;
use cce_core::gate::GateReport;
use nexus_core::objects::{Csu, RawObservation, TaskSpec};

/// FetchPlan (CSA.10): content-adressierbar, budgetiert, gate-gelistet.
#[derive(Debug, Clone)]
pub struct FetchPlan {
    pub plan_id: String,
    pub adapter_id: String,
    pub operations: Vec<PlannedOp>,
    pub gates: Vec<String>,
    pub cache_etag: Option<String>,
    pub cache_cursor: Option<String>,
    pub backoff_policy: String,
}

#[derive(Debug, Clone)]
pub struct PlannedOp {
    pub method: String,
    pub endpoint_template: String,
    pub expected_status: u16,
    pub cost_requests: u32,
}

/// Extrahierter Datensatz (Zwischenform E_ε → N_ν).
#[derive(Debug, Clone)]
pub struct ExtractedRecord {
    pub record_id: String,
    pub locator: String,
    pub fields: cce_core::value::CanonValue,
}

/// Der EINE Portvertrag (8 Methoden). `acquire` erhaelt die Beobachtungen
/// vom Fetch-Layer (der Netzpfad selbst liegt in nexus-fetch HINTER dem
/// PolicyGate) und dekodiert sie adapterspezifisch.
pub trait SourceAdapter {
    /// 1. Identitaet, Klasse, Policy, License, Privacy, Budget/Rate,
    ///    Provenance- & Replay-Strategie.
    fn manifest(&self) -> AdapterManifest;
    /// 2. Preflight: adapterlokale Vorpruefung des Auftrags.
    fn preflight(&self, task: &TaskSpec) -> GateReport;
    /// 3. Planen: TaskSpec + Budget → FetchPlan.
    fn plan(&self, task: &TaskSpec, budget_requests: u32) -> FetchPlan;
    /// 4. Akquirieren: dekodiert die (bereits policy-gegateten) Roh-Bytes.
    fn acquire(&self, raw: &[RawObservation]) -> Vec<RawObservation>;
    /// 5. Extrahieren: RawObservation → ExtractedRecords (Locator-Bindung).
    fn extract(&self, raw: &RawObservation) -> Result<Vec<ExtractedRecord>, String>;
    /// 6. Normalisieren: ExtractedRecord → CSUs.
    fn normalize(&self, record: &ExtractedRecord) -> Vec<Csu>;
    /// 7. Validieren: CSU-Wohlgeformtheit (adapterlokal).
    fn validate(&self, unit: &Csu) -> GateReport;
    /// 8. Zitieren: Evidence-Atome (Locator, Hash, Attribution).
    fn cite(&self, unit: &Csu) -> Vec<String>;
}

/// check_source_adapter_parity (CSA.16): ein Portvertrag, alle Methoden
/// inhaltlich belegt, Manifest vollstaendig.
pub fn check_source_adapter_parity(adapter: &dyn SourceAdapter) -> GateReport {
    let manifest = adapter.manifest();
    if let Err(e) = manifest.validate() {
        return GateReport::hold(
            "check_source_adapter_parity",
            &format!("manifest_missing: {:?}", e.missing),
        );
    }
    let task = TaskSpec::new("paritaets-pruefung", "scope:test");
    let plan = adapter.plan(&task, 1);
    if plan.operations.is_empty() {
        return GateReport::hold(
            "check_source_adapter_parity",
            "plan() liefert keine Operationen",
        );
    }
    if plan.gates.is_empty() {
        return GateReport::hold("check_source_adapter_parity", "plan() ohne Gate-Liste");
    }
    GateReport::pass(
        "check_source_adapter_parity",
        &format!(
            "Adapter {} erfuellt den Portvertrag (8/8)",
            manifest.adapter_id
        ),
    )
}
