//! LocalCorpusAdapter — Referenzadapter Klasse `local_corpus` (CSA.15):
//! liest einen deklarierten, eingefrorenen Korpus (Fixture-Map).
//! Kein Netz, keine Terms-Fragen (not_applicable), Lizenz deklariert.

use cce_core::gate::GateReport;
use nexus_adapter::manifest::AdapterManifest;
use nexus_adapter::port::{ExtractedRecord, FetchPlan, PlannedOp, SourceAdapter};
use nexus_core::objects::{Csu, RawObservation, TaskSpec};
use nexus_decode::decode_kv_lines;
use nexus_normalize::normalize_record;

pub struct LocalCorpusAdapter {
    pub corpus_id: String,
}

impl LocalCorpusAdapter {
    pub fn new(corpus_id: &str) -> Self {
        Self {
            corpus_id: corpus_id.to_string(),
        }
    }
}

impl SourceAdapter for LocalCorpusAdapter {
    fn manifest(&self) -> AdapterManifest {
        AdapterManifest::complete(
            &format!("local-corpus:{}", self.corpus_id),
            "local_corpus",
            "cc-by-4.0",
        )
    }

    fn preflight(&self, task: &TaskSpec) -> GateReport {
        if task.data_policy == "no_pii" {
            GateReport::pass("preflight:local_corpus", "Korpus PII-frei deklariert")
        } else {
            GateReport::hold("preflight:local_corpus", "unbekannte Datenpolitik")
        }
    }

    fn plan(&self, _task: &TaskSpec, budget_requests: u32) -> FetchPlan {
        FetchPlan {
            plan_id: format!("plan:local:{}", self.corpus_id),
            adapter_id: format!("local-corpus:{}", self.corpus_id),
            operations: vec![PlannedOp {
                method: "read".to_string(),
                endpoint_template: format!("corpus://{}/{{key}}", self.corpus_id),
                expected_status: 200,
                cost_requests: budget_requests.min(4),
            }],
            gates: vec![
                "rate_budget_gate".to_string(),
                "schema_gate".to_string(),
                "quality_gate".to_string(),
            ],
            cache_etag: None,
            cache_cursor: None,
            backoff_policy: "none".to_string(),
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
            "doc",
            "kv_lines",
            (900, 900, 900),
            "cc-by-4.0",
            cce_core::signature::sha256(record.locator.as_bytes()),
            "docs",
        )]
    }

    fn validate(&self, unit: &Csu) -> GateReport {
        if unit.uid.starts_with("csu:") && !unit.provenance.is_empty() {
            GateReport::pass("validate:local_corpus", "CSU wohlgeformt")
        } else {
            GateReport::hold("validate:local_corpus", "UID/Provenienz fehlt")
        }
    }

    fn cite(&self, unit: &Csu) -> Vec<String> {
        vec![format!(
            "korpus {} · locator {} · hash {}",
            self.corpus_id,
            unit.provenance,
            unit.source_hash.to_hex()
        )]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nexus_adapter::port::check_source_adapter_parity;

    #[test]
    fn parity_and_roundtrip() {
        let a = LocalCorpusAdapter::new("k1");
        assert!(check_source_adapter_parity(&a).is_pass());
        let raw = RawObservation {
            locator: "corpus://k1/doc1".to_string(),
            bytes: b"titel: Referenz\ninhalt: Text".to_vec(),
            fetched_via: "fixture".to_string(),
            snapshot_id: "snap-1".to_string(),
        };
        let recs = a.extract(&raw).unwrap();
        let csus = a.normalize(&recs[0]);
        assert_eq!(csus.len(), 1);
        assert!(a.validate(&csus[0]).is_pass());
        assert!(!a.cite(&csus[0]).is_empty());
    }
}
