//! Dokument 18 §6: der RepoWorkbody-Bau. Ein zertifizierter Bauauftrag
//! IST ein `.loom`-Workbody der Containerklasse `"repo"` — erbt damit
//! alles Bestehende (verify, Signatur, Transport, `cites`). Kein neues
//! Segment noetig: CodeUnits als ARTIFACT+CAS_BLOB, TaskLedger im
//! LEDGER, ToolEvidence im EVIDENCE, der DiffCandidate in
//! CANDIDATE_OUTPUTS (0x0063), Tool-Deklarationen in TOOL_PROFILE
//! (0x0064) — dieselbe Disziplin wie `cce-bridge`s `"norm"`-Workbody.

use crate::model::{DiffCandidate, ProducedBy, RepoSnapshot, TaskLedger};
use cce_core::signature::sha256;
use loom_canon::Cv;
use loom_cites::cites_field;
use loom_codec::{seal_canonical, PackError, Sealed, Segment};
use loom_format::{
    KIND_ARTIFACT, KIND_CANDIDATE_OUTPUTS, KIND_CANON_DESC, KIND_CAS_BLOB, KIND_CL_SUBSTRATE,
    KIND_EVIDENCE, KIND_LEDGER, KIND_MANIFEST, KIND_REPLAY_MANIFEST, KIND_RESIDUE,
    KIND_TOOL_PROFILE,
};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkbodyError {
    /// Nur ein vollstaendig akzeptierter TaskLedger darf einen
    /// RepoWorkbody erzeugen (Kandidaten-Commit-Verbot, C.7).
    NotAllAccepted,
    Pack(PackError),
}

fn seg(kind: u16, v: &Cv) -> Segment {
    Segment::canonical(kind, v).expect("kanonisches Segment")
}

fn canon_desc_segment() -> Segment {
    Segment {
        kind: KIND_CANON_DESC,
        seg_flags: 0,
        payload: Cv::Text(loom_canon::CANON_RULES_TEXT.into())
            .encode()
            .unwrap(),
        deps: vec![],
    }
}

fn repo_manifest_cv(task_id: &str, snapshot: &RepoSnapshot, closed: bool) -> Cv {
    Cv::map(vec![
        ("title", Cv::Text(format!("RepoWorkbody: {task_id}"))),
        ("container_class", Cv::Text("repo".into())),
        ("domain_refs", Cv::Array(vec![Cv::Text("swe".into())])),
        ("scale", Cv::Uint(1)),
        ("pl_level", Cv::Text("PL2".into())),
        ("claims", Cv::map(vec![("closed", Cv::Bool(closed))])),
        (
            "origin",
            Cv::map(vec![
                ("tool", Cv::Text("cce-swe-0.1".into())),
                ("task_id", Cv::Text(task_id.into())),
            ]),
        ),
        (
            "profiles_required",
            Cv::Array(vec![Cv::Text("repo".into())]),
        ),
        ("profiles_optional", Cv::Array(vec![])),
        (
            "residue_summary",
            Cv::map(vec![("count", Cv::Uint(0)), ("kinds", Cv::Array(vec![]))]),
        ),
        (
            "capability_declarations",
            Cv::Array(vec![
                Cv::Text("fs_write".into()),
                Cv::Text("build".into()),
                Cv::Text("test".into()),
            ]),
        ),
        ("license_summary", Cv::Text("cc0".into())),
        (
            "created",
            Cv::Tag(0, Box::new(Cv::Text("2026-07-04T00:00:00Z".into()))),
        ),
        ("external_citations", Cv::Array(vec![])),
        // P2-spezifische additive Felder (neben den 13 Pflichtfeldern):
        ("snapshot_root", Cv::Text(snapshot.snapshot_root().to_hex())),
        ("toolchain_pin", Cv::Text(snapshot.toolchain_pin.clone())),
    ])
}

fn repo_cl_substrate_cv(snapshot: &RepoSnapshot) -> Cv {
    let units = snapshot
        .units
        .iter()
        .map(|u| {
            Cv::map(vec![
                ("path", Cv::Text(u.path.clone())),
                ("language", Cv::Text(u.language.clone())),
                ("role", Cv::Text(u.role.as_str().into())),
                ("content_digest", Cv::Text(u.content_digest.to_hex())),
            ])
        })
        .collect();
    Cv::map(vec![
        ("cubes", Cv::Array(vec![])),
        ("constraints", Cv::Array(vec![])),
        ("cites", cites_field(&[])),
        (
            "repo_snapshot",
            Cv::map(vec![
                ("snapshot_root", Cv::Text(snapshot.snapshot_root().to_hex())),
                ("units", Cv::Array(units)),
            ]),
        ),
    ])
}

fn repo_ledger_cv(ledger: &TaskLedger) -> Cv {
    let commit_ids: Vec<Cv> = ledger
        .blocks
        .iter()
        .map(|b| Cv::Text(b.id.clone()))
        .collect();
    Cv::map(vec![
        ("commits", Cv::Array(commit_ids.clone())),
        ("gate_reports_for", Cv::Array(commit_ids)),
        ("closure_proof", Cv::Bool(ledger.all_accepted())),
        (
            "hdag_projection",
            Cv::Text(
                "n/a: TaskLedger ist die PhaseBlock-Kette EINES Bauauftrags, kein \
                 voller HyperDAG-Motorlauf"
                    .into(),
            ),
        ),
    ])
}

fn repo_evidence_cv(ledger: &TaskLedger) -> Cv {
    Cv::map(vec![(
        "blocks",
        Cv::Array(
            ledger
                .blocks
                .iter()
                .map(|b| {
                    Cv::map(vec![
                        ("id", Cv::Text(b.id.clone())),
                        ("phase", Cv::Text(b.phase.clone())),
                        ("gate_count", Cv::Uint(b.gate_reports.len() as u64)),
                        (
                            "evidence_refs",
                            Cv::Array(
                                b.evidence_refs
                                    .iter()
                                    .map(|d| Cv::Text(d.to_hex()))
                                    .collect(),
                            ),
                        ),
                    ])
                })
                .collect(),
        ),
    )])
}

fn repo_residue_cv() -> Cv {
    Cv::map(vec![("residues", Cv::Array(vec![]))])
}

fn repo_candidate_outputs_cv(diff: &DiffCandidate, evidence_ref: &str) -> Cv {
    let produced_by = match &diff.produced_by {
        ProducedBy::Provider {
            provider_id,
            manifest_ref,
        } => Cv::map(vec![
            ("kind", Cv::Text("provider".into())),
            ("provider_id", Cv::Text(provider_id.clone())),
            ("manifest_ref", Cv::Text(manifest_ref.clone())),
        ]),
        ProducedBy::Operator { operator } => Cv::map(vec![
            ("kind", Cv::Text("operator".into())),
            ("operator", Cv::Text(operator.clone())),
        ]),
    };
    let hunks = diff
        .hunks
        .iter()
        .map(|h| {
            Cv::map(vec![
                ("path", Cv::Text(h.path.clone())),
                ("unified_diff", Cv::Text(h.unified_diff.clone())),
            ])
        })
        .collect();
    Cv::map(vec![(
        "outputs",
        Cv::Array(vec![Cv::map(vec![
            ("evidence_ref", Cv::Text(evidence_ref.into())),
            ("is_commit", Cv::Bool(false)),
            ("rationale", Cv::Text(diff.rationale.clone())),
            ("produced_by", produced_by),
            ("hunks", Cv::Array(hunks)),
        ])]),
    )])
}

fn repo_tool_profile_cv(manifests: &[&cce_toolgateway::manifest::ToolManifest]) -> Cv {
    Cv::map(vec![(
        "tools",
        Cv::Array(
            manifests
                .iter()
                .map(|m| {
                    Cv::map(vec![
                        ("tool_id", Cv::Text(m.tool_id.clone())),
                        ("tool_class", Cv::Text(m.tool_class.clone())),
                        (
                            "scope",
                            Cv::Array(m.scope.iter().map(|s| Cv::Text(s.clone())).collect()),
                        ),
                        ("budget_calls", Cv::Uint(u64::from(m.budget_calls))),
                        ("replay_strategy", Cv::Text(m.replay_strategy.clone())),
                    ])
                })
                .collect(),
        ),
    )])
}

/// ARTIFACT + CAS_BLOB: der Container traegt die materialisierten
/// Bytes SELBST (nicht nur ihren Digest, dieselbe Disziplin wie X1a) —
/// alle Dateien der Arbeitskopie, sortiert nach Pfad, in einem Blob
/// verkettet mit Index.
fn repo_artifact_and_blob(files: &BTreeMap<String, Vec<u8>>) -> (Segment, Segment) {
    let mut buf: Vec<u8> = Vec::new();
    let mut index = Vec::new();
    for (path, content) in files {
        let start = buf.len() as u64;
        buf.extend_from_slice(content);
        let end = buf.len() as u64;
        index.push(Cv::map(vec![
            ("path", Cv::Text(path.clone())),
            ("start", Cv::Uint(start)),
            ("end", Cv::Uint(end)),
            ("byte_digest", Cv::Text(sha256(content).to_hex())),
        ]));
    }
    let content_class = sha256(&buf).to_hex();
    let artifact_cv = Cv::map(vec![
        ("artifact_id", Cv::Text("artifact:repo-snapshot".into())),
        (
            "two_digest",
            Cv::map(vec![
                ("content_class", Cv::Text(content_class.clone())),
                ("byte_digest", Cv::Text(content_class)),
            ]),
        ),
        ("index", Cv::Array(index)),
    ]);
    (
        seg(KIND_ARTIFACT, &artifact_cv),
        seg(KIND_CAS_BLOB, &Cv::Bytes(buf)),
    )
}

/// Siegelt den RepoWorkbody. Nur ein VOLLSTAENDIG akzeptierter
/// TaskLedger darf einen Workbody erzeugen (Kandidaten-Commit-Verbot).
pub fn seal_repo_workbody(
    task_id: &str,
    snapshot: &RepoSnapshot,
    files: &BTreeMap<String, Vec<u8>>,
    ledger: &TaskLedger,
    diff: &DiffCandidate,
    tool_manifests: &[&cce_toolgateway::manifest::ToolManifest],
) -> Result<Sealed, WorkbodyError> {
    if !ledger.all_accepted() {
        return Err(WorkbodyError::NotAllAccepted);
    }
    let manifest = repo_manifest_cv(task_id, snapshot, true);
    let cl = repo_cl_substrate_cv(snapshot);
    let ledger_cv = repo_ledger_cv(ledger);
    let residues = repo_residue_cv();
    let evidence = repo_evidence_cv(ledger);
    let evidence_ref = ledger
        .blocks
        .first()
        .and_then(|b| b.evidence_refs.first())
        .map(|d| d.to_hex())
        .unwrap_or_else(|| "evidence:none".to_string());
    let candidate_outputs = repo_candidate_outputs_cv(diff, &evidence_ref);
    let tool_profile = repo_tool_profile_cv(tool_manifests);
    let rd_hex = ledger
        .rd_ref
        .map(|d| d.to_hex())
        .unwrap_or_else(|| "rd:none".to_string());
    let replay =
        loom_replay::replay_manifest_segment(&rd_hex, 0, &snapshot.snapshot_root().to_hex());
    let (artifact_seg, cas_blob_seg) = repo_artifact_and_blob(files);

    let sealed = seal_canonical(
        "repo",
        &["repo"],
        &[
            seg(KIND_MANIFEST, &manifest),
            canon_desc_segment(),
            seg(KIND_CL_SUBSTRATE, &cl),
            seg(KIND_LEDGER, &ledger_cv),
            seg(KIND_RESIDUE, &residues),
            seg(KIND_EVIDENCE, &evidence),
            seg(KIND_REPLAY_MANIFEST, &replay),
            seg(KIND_CANDIDATE_OUTPUTS, &candidate_outputs),
            seg(KIND_TOOL_PROFILE, &tool_profile),
            artifact_seg,
            cas_blob_seg,
        ],
    )
    .map_err(WorkbodyError::Pack)?;

    Ok(sealed)
}
