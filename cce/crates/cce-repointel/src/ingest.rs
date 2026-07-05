//! Der Einzug: fremdes Repo → NexusSourceBundle, WOERTLICH ueber die
//! bestehende CSA-Kette (Dokument 23 Track B: „Lizenz-/Policy-Gates
//! unveraendert scharf", „kein Netz-Fetch ohne die volle CSA-Kette").
//!
//! Reihenfolge exakt wie der CSA-Referenzzeuge (`csa_catalog.rs`):
//! manifest → preflight → plan → `approve_fetch` (sechs Policy-Gates,
//! fail-closed, versiegelter Plan) → `fetch` (NUR mit versiegeltem
//! Plan) → extract → normalize → validate + schema/quality/provenance
//! → EvidencePack je CSU → NexusSourceBundle → Ledger-Head.

use crate::observe::{observe_repo_kv, RepoFile};
use crate::residues::repointel_residue;
use cce_core::residue::Residue;
use cce_core::signature::Digest;
use nexus_adapter::port::SourceAdapter;
use nexus_adapter_git::GitRepositoryAdapter;
use nexus_core::objects::{NexusSourceBundle, SourceHorizon, TaskSpec};
use nexus_evidence::build_pack;
use nexus_fetch::{fetch, FetchCache, SnapshotTransport};
use nexus_ledger::ledger_for_bundle;
use nexus_policy::{approve_fetch, SourceDeclaration};
use nexus_validate::{dedup, provenance_gate, quality_gate, schema_gate, QualityThresholds};

/// Der Auftrag: ein fremdes Repo an einem FIXEN Commit, mit
/// deklarierter Lizenz und eingelesenen Dateien.
#[derive(Debug, Clone)]
pub struct RepoIntelInput {
    pub repo_id: String,
    /// Fixer Commit/Stand — die Replay-Achse (GitRepositoryAdapter).
    pub commit_sha: String,
    /// Deklarierte Lizenz des Ziel-Repos (Policy-Gate-Eingang).
    pub declared_license: String,
    pub files: Vec<RepoFile>,
}

/// Ergebnis des Einzugs: das Bundle + der Ledger-Head + die
/// Adapter-Zitate (menschenlesbare Quell-Attribution).
#[derive(Debug)]
pub struct IngestedRepo {
    pub nsb: NexusSourceBundle,
    pub ledger_head: Digest,
    pub adapter_cites: Vec<String>,
    /// Locator der einen Struktur-Beobachtung (commit-gebunden).
    pub observation_locator: String,
}

/// Deklaration fuer das Policy-Gate — Tatsachen des Auftrags, keine
/// Wertung. `license` traegt die DEKLARIERTE Repo-Lizenz; das
/// `license_gate` entscheidet fail-closed (proprietary_no_reuse ⇒ Hold).
fn declaration(license: &str) -> SourceDeclaration {
    SourceDeclaration {
        adapter_class: "git_repository".to_string(),
        access_method: "api_key_configured".to_string(),
        robots_or_terms: Some("permitted".to_string()),
        license: Some(license.to_string()),
        contains_pii: false,
        requested_actions: vec![],
    }
}

/// Der volle Einzug. Fail-closed: haelt die Policy (z. B.
/// `proprietary_no_reuse`), gibt es KEIN Bundle — und damit weder
/// Destillation noch Bauplan.
pub fn ingest_repo(input: &RepoIntelInput) -> Result<IngestedRepo, Box<Residue>> {
    let adapter =
        GitRepositoryAdapter::new(&input.repo_id, &input.commit_sha, &input.declared_license);

    let hs = SourceHorizon::example_local();
    let task = TaskSpec::new("repointel", "scope:repo-structure");

    // 1. Preflight (fixer Commit = Replay-Anker).
    let pre = adapter.preflight(&task);
    if !pre.is_pass() {
        return Err(Box::new(repointel_residue(
            "repointel_policy_blocked",
            &format!("Preflight hielt: {}", pre.reason),
        )));
    }

    // 2. Plan → Policy (sechs Gates, fail-closed, versiegelter Plan).
    let manifest = adapter.manifest();
    let decl = declaration(&input.declared_license);
    let plan = adapter.plan(&task, task.budget_requests);
    let endpoint = plan.operations[0].endpoint_template.clone();
    let approved = match approve_fetch(&hs, &task, &manifest, &decl, plan) {
        Ok(a) => a,
        Err(verdict) => {
            return Err(Box::new(repointel_residue(
                "repointel_policy_blocked",
                &format!(
                    "CSA-Policy hielt fuer {}@{}: {:?}",
                    input.repo_id, input.commit_sha, verdict
                ),
            )));
        }
    };

    // 3. Transport: die deterministische Struktur-Beobachtung
    //    `structural_kv_v1` als Snapshot am fixen Commit. Kein Socket —
    //    und selbst dieser Snapshot ist NUR ueber den versiegelten
    //    Plan erreichbar.
    let mut transport = SnapshotTransport {
        snapshot_id: format!("repointel:{}@{}", input.repo_id, input.commit_sha),
        ..Default::default()
    };
    transport.responses.insert(
        endpoint.clone(),
        (
            observe_repo_kv(&input.repo_id, &input.commit_sha, &input.files),
            None,
        ),
    );
    let mut cache = FetchCache::default();
    let raws = match fetch(&approved, &transport, &mut cache, task.budget_requests) {
        Ok(r) => r,
        Err(verdict) => {
            return Err(Box::new(repointel_residue(
                "repointel_policy_blocked",
                &format!("Fetch hielt: {verdict:?}"),
            )));
        }
    };

    // 4. Extract → Normalize → Validate + Schema/Quality/Provenance,
    //    EvidencePack je CSU (N5) — wortidentisch zur Referenzkette.
    let mut csus = Vec::new();
    let mut packs = Vec::new();
    let mut cites = Vec::new();
    for raw in &raws {
        let records = adapter.extract(raw).map_err(|e| {
            Box::new(repointel_residue(
                "repointel_empty_ingest",
                &format!("Extraktion scheiterte: {e}"),
            ))
        })?;
        for rec in records {
            for csu in adapter.normalize(&rec) {
                let v = adapter.validate(&csu);
                if !v.is_pass() {
                    return Err(Box::new(repointel_residue(
                        "repointel_empty_ingest",
                        &format!("Adapter-Validierung hielt: {}", v.reason),
                    )));
                }
                for (name, gate) in [
                    ("schema", schema_gate(&csu)),
                    ("quality", quality_gate(&csu, &QualityThresholds::default())),
                    ("provenance", provenance_gate(&csu)),
                ] {
                    if !gate.allows() {
                        return Err(Box::new(repointel_residue(
                            "repointel_empty_ingest",
                            &format!("{name}_gate hielt fuer {}", csu.uid),
                        )));
                    }
                }
                cites.extend(adapter.cite(&csu));
                packs.push(build_pack(
                    &csu,
                    raw,
                    &["observe_structural_v1", "decode", "normalize"],
                    Some(&format!(
                        "repo {} @ {} (deklarierte Lizenz: {})",
                        input.repo_id, input.commit_sha, input.declared_license
                    )),
                ));
                csus.push(csu);
            }
        }
    }
    let csus = dedup(csus);
    if csus.is_empty() {
        return Err(Box::new(repointel_residue(
            "repointel_empty_ingest",
            "kein einziges gueltiges CSU aus dem Einzug",
        )));
    }

    let nsb = NexusSourceBundle {
        bundle_id: format!("nsb:repointel:{}@{}", input.repo_id, input.commit_sha),
        run_id: format!("repointel:{}", input.commit_sha),
        csu_set: csus,
        evidence_packs: packs,
        source_graph: vec![],
        quality_summary: "structural_kv_v1: alle Achsen ueber Schwellen".to_string(),
        residues: vec![],
        export_contract: "blueprint_workbody".to_string(),
    };
    let ledger_head = ledger_for_bundle(&nsb).head();

    Ok(IngestedRepo {
        nsb,
        ledger_head,
        adapter_cites: cites,
        observation_locator: endpoint,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input(license: &str) -> RepoIntelInput {
        RepoIntelInput {
            repo_id: "beispiel/numkit".to_string(),
            commit_sha: "abc1234def".to_string(),
            declared_license: license.to_string(),
            files: vec![
                RepoFile::new("src/lib.rs", b"pub fn max_of(v: &[i64]) -> i64 { v[0] }\n"),
                RepoFile::new(
                    "Cargo.toml",
                    b"[package]\nname = \"numkit\"\nedition = \"2021\"\n",
                ),
            ],
        }
    }

    #[test]
    fn ingest_yields_bundle_with_evidence_per_csu() {
        let ing = ingest_repo(&input("mit")).expect("Einzug gruen");
        assert!(!ing.nsb.csu_set.is_empty());
        assert!(ing.nsb.every_csu_has_evidence());
        assert!(ing.adapter_cites[0].contains("commit abc1234def"));
        assert!(ing.observation_locator.starts_with("git://"));
    }

    #[test]
    fn proprietary_license_blocks_before_any_distillation() {
        let e = ingest_repo(&input("proprietary_no_reuse")).unwrap_err();
        assert!(e.id.contains("repointel_policy_blocked"));
    }
}
