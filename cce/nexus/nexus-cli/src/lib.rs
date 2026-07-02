//! nexus-cli — duenner Treiber: fuehrt einen deterministischen
//! Referenzlauf (LocalCorpus, Fixture-Transport) durch die volle
//! CSA-Kette und druckt die Gate-Bilanz. KEIN Netz: der einzige
//! Abrufpfad ist fetch(&ApprovedFetchPlan, …) hinter dem PolicyGate.

use nexus_adapter::port::SourceAdapter;
use nexus_adapter_local_corpus::LocalCorpusAdapter;
use nexus_core::objects::{SourceHorizon, TaskSpec};
use nexus_fetch::{FetchCache, SnapshotTransport};
use nexus_policy::{approve_fetch, SourceDeclaration};

/// Ergebnis eines Referenzlaufs: Gate-Namen mit Pass/Hold.
pub struct RunSummary {
    pub gates: Vec<(String, bool)>,
    pub csu_count: usize,
}

pub fn reference_run() -> Result<RunSummary, String> {
    let hs = SourceHorizon::example_local();
    let task = TaskSpec::new("referenzlauf", "scope:docs");
    let adapter = LocalCorpusAdapter::new("k1");
    let manifest = adapter.manifest();
    let decl = SourceDeclaration {
        adapter_class: "local_corpus".to_string(),
        access_method: "filesystem".to_string(),
        robots_or_terms: Some("not_applicable".to_string()),
        license: Some("cc-by-4.0".to_string()),
        contains_pii: false,
        requested_actions: vec![],
    };
    let plan = adapter.plan(&task, task.budget_requests);
    let approved = approve_fetch(&hs, &task, &manifest, &decl, plan)
        .map_err(|v| format!("policy hold: {} Gates", v.len()))?;

    let mut gates: Vec<(String, bool)> = approved
        .gate_reports
        .iter()
        .map(|g| (g.gate_id.clone(), g.is_pass()))
        .collect();

    let mut transport = SnapshotTransport {
        snapshot_id: "snap-cli".to_string(),
        ..Default::default()
    };
    transport.responses.insert(
        "corpus://k1/{key}".to_string(),
        (b"titel: Referenz\ninhalt: Klartext".to_vec(), None),
    );
    let mut cache = FetchCache::default();
    let raws = nexus_fetch::fetch(&approved, &transport, &mut cache, task.budget_requests)
        .map_err(|v| format!("{}: hold", v.gate()))?;
    gates.push(("rate_budget_gate".to_string(), true));

    let mut csus = Vec::new();
    for raw in &raws {
        for rec in adapter.extract(raw)? {
            csus.extend(adapter.normalize(&rec));
        }
    }
    for c in &csus {
        gates.push((format!("validate:{}", c.uid), adapter.validate(c).is_pass()));
    }
    Ok(RunSummary {
        gates,
        csu_count: csus.len(),
    })
}
