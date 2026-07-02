//! Wikimedia-OfficialAPIAdapter — Referenzadapter Klasse `official_api`
//! (CSA.15): offizielle API, deklarierte Rate, CC-BY-SA MIT
//! ATTRIBUTIONS-TRANSPORT (PROD-INV-16). Der Netzpfad laeuft in
//! nexus-fetch hinter dem PolicyGate — hier nur Plan/Dekodierung.

use cce_core::gate::GateReport;
use nexus_adapter::manifest::AdapterManifest;
use nexus_adapter::port::{ExtractedRecord, FetchPlan, PlannedOp, SourceAdapter};
use nexus_core::objects::{Csu, RawObservation, TaskSpec};
use nexus_decode::decode_kv_lines;
use nexus_normalize::normalize_record;

pub struct WikimediaAdapter;

pub const WIKIMEDIA_LICENSE: &str = "cc-by-sa-4.0";

impl SourceAdapter for WikimediaAdapter {
    fn manifest(&self) -> AdapterManifest {
        let mut m = AdapterManifest::complete("wikimedia-api", "official_api", WIKIMEDIA_LICENSE);
        m.policy = Some("official_api_terms".to_string());
        m.rate_limit_per_run = Some(2);
        m
    }

    fn preflight(&self, task: &TaskSpec) -> GateReport {
        if task.budget_requests == 0 {
            GateReport::hold("preflight:wikimedia", "Budget 0 — kein Abruf planbar")
        } else {
            GateReport::pass("preflight:wikimedia", "offizielle API, Budget vorhanden")
        }
    }

    fn plan(&self, task: &TaskSpec, budget_requests: u32) -> FetchPlan {
        FetchPlan {
            plan_id: format!("plan:wikimedia:{}", task.scope),
            adapter_id: "wikimedia-api".to_string(),
            operations: vec![PlannedOp {
                method: "get".to_string(),
                endpoint_template: "api://wikimedia/page/{title}".to_string(),
                expected_status: 200,
                cost_requests: budget_requests.min(2),
            }],
            gates: vec![
                "rate_budget_gate".to_string(),
                "schema_gate".to_string(),
                "quality_gate".to_string(),
                "evidence_gate".to_string(),
            ],
            cache_etag: Some("etag:wikimedia".to_string()),
            cache_cursor: None,
            backoff_policy: "exponential".to_string(),
        }
    }

    fn acquire(&self, raw: &[RawObservation]) -> Vec<RawObservation> {
        raw.to_vec()
    }

    fn extract(&self, raw: &RawObservation) -> Result<Vec<ExtractedRecord>, String> {
        let fields = decode_kv_lines(raw).map_err(|r| r.id.clone())?;
        Ok(vec![ExtractedRecord {
            record_id: format!("rec:{}", raw.digest().to_hex()),
            locator: raw.locator.clone(),
            fields,
        }])
    }

    fn normalize(&self, record: &ExtractedRecord) -> Vec<Csu> {
        vec![normalize_record(
            record,
            "wiki_page",
            "kv_lines",
            (850, 900, 950),
            WIKIMEDIA_LICENSE,
            cce_core::signature::sha256(record.locator.as_bytes()),
            "encyclopedia",
        )]
    }

    fn validate(&self, unit: &Csu) -> GateReport {
        if unit.license.starts_with("cc-by-sa") {
            GateReport::pass("validate:wikimedia", "Lizenz + CSU wohlgeformt")
        } else {
            GateReport::hold("validate:wikimedia", "Lizenzangabe fehlt")
        }
    }

    /// Attribution ist hier PFLICHT-Transportgut (PROD-INV-16).
    fn cite(&self, unit: &Csu) -> Vec<String> {
        vec![format!(
            "Wikimedia-Beitraegerinnen und -Beitraeger · {} · Lizenz {} · hash {}",
            unit.provenance,
            unit.license,
            unit.source_hash.to_hex()
        )]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nexus_adapter::port::check_source_adapter_parity;

    #[test]
    fn parity_and_attribution_present() {
        let a = WikimediaAdapter;
        assert!(check_source_adapter_parity(&a).is_pass());
        let raw = RawObservation {
            locator: "api://wikimedia/page/Kristall".to_string(),
            bytes: b"titel: Kristall\nauszug: Festkoerper".to_vec(),
            fetched_via: "snapshot".to_string(),
            snapshot_id: "snap-w1".to_string(),
        };
        let recs = a.extract(&raw).unwrap();
        let csu = &a.normalize(&recs[0])[0];
        let cites = a.cite(csu);
        assert!(
            cites[0].contains("cc-by-sa"),
            "Attribution muss Lizenz transportieren"
        );
        assert!(cites[0].contains("Beitraeger"));
    }
}
