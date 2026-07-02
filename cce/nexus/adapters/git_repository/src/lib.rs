//! Git-RepositoryAdapter — Referenzadapter Klasse `git_repository`
//! (CSA.15): Provenienz = Commit-SHA + Pfad, Lizenz aus dem
//! Repo-Manifest (LICENSE-Deklaration), Replay ueber fixen Commit.

use cce_core::gate::GateReport;
use nexus_adapter::manifest::AdapterManifest;
use nexus_adapter::port::{ExtractedRecord, FetchPlan, PlannedOp, SourceAdapter};
use nexus_core::objects::{Csu, RawObservation, TaskSpec};
use nexus_decode::decode_kv_lines;
use nexus_normalize::normalize_record;

pub struct GitRepositoryAdapter {
    pub repo: String,
    /// Fixer Commit — die Replay-Achse dieses Adapters.
    pub commit_sha: String,
    pub declared_license: String,
}

impl GitRepositoryAdapter {
    pub fn new(repo: &str, commit_sha: &str, declared_license: &str) -> Self {
        Self {
            repo: repo.to_string(),
            commit_sha: commit_sha.to_string(),
            declared_license: declared_license.to_string(),
        }
    }
}

impl SourceAdapter for GitRepositoryAdapter {
    fn manifest(&self) -> AdapterManifest {
        let mut m = AdapterManifest::complete(
            &format!("git:{}", self.repo),
            "git_repository",
            &self.declared_license,
        );
        m.provenance_strategy = Some("commit_sha+path".to_string());
        m.replay_strategy = Some(format!("fixed_commit:{}", self.commit_sha));
        m
    }

    fn preflight(&self, _task: &TaskSpec) -> GateReport {
        if self.commit_sha.len() >= 7 {
            GateReport::pass("preflight:git", "fixer Commit deklariert (Replay-Anker)")
        } else {
            GateReport::hold(
                "preflight:git",
                "kein fixer Commit — Replay nicht garantierbar",
            )
        }
    }

    fn plan(&self, _task: &TaskSpec, budget_requests: u32) -> FetchPlan {
        FetchPlan {
            plan_id: format!("plan:git:{}@{}", self.repo, self.commit_sha),
            adapter_id: format!("git:{}", self.repo),
            operations: vec![PlannedOp {
                method: "read".to_string(),
                endpoint_template: format!("git://{}@{}/{{path}}", self.repo, self.commit_sha),
                expected_status: 200,
                cost_requests: budget_requests.min(4),
            }],
            gates: vec![
                "rate_budget_gate".to_string(),
                "schema_gate".to_string(),
                "provenance_gate".to_string(),
            ],
            cache_etag: None,
            cache_cursor: Some(self.commit_sha.clone()),
            backoff_policy: "none".to_string(),
        }
    }

    fn acquire(&self, raw: &[RawObservation]) -> Vec<RawObservation> {
        raw.to_vec()
    }

    fn extract(&self, raw: &RawObservation) -> Result<Vec<ExtractedRecord>, String> {
        let fields = decode_kv_lines(raw).map_err(|r| r.id.clone())?;
        Ok(vec![ExtractedRecord {
            record_id: format!("rec:{}@{}", self.commit_sha, raw.digest().to_hex()),
            locator: raw.locator.clone(),
            fields,
        }])
    }

    fn normalize(&self, record: &ExtractedRecord) -> Vec<Csu> {
        vec![normalize_record(
            record,
            "repo_file",
            "kv_lines",
            (900, 950, 900),
            &self.declared_license,
            cce_core::signature::sha256(
                format!("{}@{}", record.locator, self.commit_sha).as_bytes(),
            ),
            "code",
        )]
    }

    fn validate(&self, unit: &Csu) -> GateReport {
        if unit.provenance.contains(&self.commit_sha) || unit.provenance.starts_with("git://") {
            GateReport::pass("validate:git", "Provenienz commit-gebunden")
        } else {
            GateReport::hold("validate:git", "provenance_gap: kein Commit-Bezug")
        }
    }

    fn cite(&self, unit: &Csu) -> Vec<String> {
        vec![format!(
            "repo {} · commit {} · {} · lizenz {}",
            self.repo, self.commit_sha, unit.provenance, unit.license
        )]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nexus_adapter::port::check_source_adapter_parity;

    #[test]
    fn parity_and_commit_bound_provenance() {
        let a = GitRepositoryAdapter::new("beispiel/repo", "abc1234def", "mit");
        assert!(check_source_adapter_parity(&a).is_pass());
        let raw = RawObservation {
            locator: "git://beispiel/repo@abc1234def/README".to_string(),
            bytes: b"titel: Readme\ninhalt: Hallo".to_vec(),
            fetched_via: "snapshot".to_string(),
            snapshot_id: "snap-g1".to_string(),
        };
        let recs = a.extract(&raw).unwrap();
        assert!(recs[0].record_id.contains("abc1234def"));
        let csu = &a.normalize(&recs[0])[0];
        assert!(a.validate(csu).is_pass());
        assert!(a.cite(csu)[0].contains("commit abc1234def"));
    }
}
