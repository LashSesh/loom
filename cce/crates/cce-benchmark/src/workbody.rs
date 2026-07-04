//! Dokument 20 §5: der Benchmark-Workbody-Bau. Ein `.loom` der neuen
//! Klasse `"benchmark"` bindet BenchmarkTaskPackage, RawRunResult,
//! CceRunResult (inkl. dessen eigenem "repo"-Workbody via `cites`) und
//! die ComparisonMatrix. Kein neues Segment noetig — dieselbe Disziplin
//! wie `cce-swe`s `"repo"` / `cce-bridge`s `"norm"`.

use crate::model::{BenchmarkTaskPackage, CceRunResult, ComparisonMatrix, MatrixRow, RawRunResult};
use loom_canon::Cv;
use loom_cites::{cites_field, external_citations_hex, CiteEntry, CiteKind};
use loom_codec::{seal_canonical, PackError, Sealed, Segment};
use loom_format::{
    KIND_CANDIDATE_OUTPUTS, KIND_CANON_DESC, KIND_CL_SUBSTRATE, KIND_EVIDENCE, KIND_LEDGER,
    KIND_MANIFEST, KIND_RESIDUE,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkbodyError {
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

/// Der eine cite des Benchmark-Koerpers: auf den zertifizierten
/// "repo"-Workbody des CCE-Arms (D6-Auditierbarkeit).
fn repo_workbody_cite(repo_workbody_ref: &str) -> CiteEntry {
    CiteEntry {
        unit_id: "cce_arm_repo_workbody".to_string(),
        target_core_root_hex: repo_workbody_ref.to_string(),
        target_unit_ref: None,
        cite_kind: CiteKind::Derives,
    }
}

fn benchmark_manifest_cv(package: &BenchmarkTaskPackage, repo_workbody_ref: &str) -> Cv {
    let external = external_citations_hex(&[repo_workbody_cite(repo_workbody_ref)]);
    Cv::map(vec![
        (
            "title",
            Cv::Text(format!(
                "Benchmark ({}): {}",
                package.task_class.as_str(),
                package.package_id
            )),
        ),
        ("container_class", Cv::Text("benchmark".into())),
        ("domain_refs", Cv::Array(vec![Cv::Text("benchmark".into())])),
        ("scale", Cv::Uint(1)),
        ("pl_level", Cv::Text("PL2".into())),
        ("claims", Cv::map(vec![("closed", Cv::Bool(true))])),
        (
            "origin",
            Cv::map(vec![
                ("tool", Cv::Text("cce-benchmark-0.1".into())),
                ("package_id", Cv::Text(package.package_id.clone())),
                ("task_package_digest", Cv::Text(package.digest_hex())),
            ]),
        ),
        (
            "profiles_required",
            Cv::Array(vec![Cv::Text("benchmark".into())]),
        ),
        ("profiles_optional", Cv::Array(vec![])),
        (
            "residue_summary",
            Cv::map(vec![("count", Cv::Uint(0)), ("kinds", Cv::Array(vec![]))]),
        ),
        (
            "capability_declarations",
            Cv::Array(vec![Cv::Text("read_segment".into())]),
        ),
        ("license_summary", Cv::Text("cc0".into())),
        (
            "created",
            Cv::Tag(0, Box::new(Cv::Text("2026-07-04T00:00:00Z".into()))),
        ),
        (
            "external_citations",
            Cv::Array(external.into_iter().map(Cv::Text).collect()),
        ),
        // Benchmark-spezifische additive Felder.
        ("task_class", Cv::Text(package.task_class.as_str().into())),
        ("task_package_digest", Cv::Text(package.digest_hex())),
    ])
}

fn matrix_row_cv(row: &MatrixRow) -> Cv {
    Cv::map(vec![
        ("dimension", Cv::Text(row.dimension.clone())),
        (
            "raw",
            Cv::map(vec![
                ("verdict", Cv::Text(row.raw.verdict.clone())),
                ("beleg", Cv::Text(row.raw.beleg.clone())),
            ]),
        ),
        (
            "cce",
            Cv::map(vec![
                ("verdict", Cv::Text(row.cce.verdict.clone())),
                ("beleg", Cv::Text(row.cce.beleg.clone())),
            ]),
        ),
    ])
}

fn benchmark_cl_substrate_cv(
    package: &BenchmarkTaskPackage,
    matrix: &ComparisonMatrix,
    repo_workbody_ref: &str,
) -> Cv {
    // cites auf den "repo"-Workbody des CCE-Arms (D6-Auditierbarkeit:
    // der CCE-Arm haengt an einem eigenstaendigen, zertifizierten
    // .loom-Koerper).
    Cv::map(vec![
        ("cubes", Cv::Array(vec![])),
        ("constraints", Cv::Array(vec![])),
        (
            "cites",
            cites_field(&[repo_workbody_cite(repo_workbody_ref)]),
        ),
        (
            "task_package",
            Cv::map(vec![
                ("package_id", Cv::Text(package.package_id.clone())),
                ("task_class", Cv::Text(package.task_class.as_str().into())),
                ("task_package_digest", Cv::Text(package.digest_hex())),
                (
                    "success_criteria",
                    Cv::Text(package.success_criteria.clone()),
                ),
            ]),
        ),
        (
            "comparison_matrix",
            Cv::Array(matrix.rows.iter().map(matrix_row_cv).collect()),
        ),
    ])
}

fn benchmark_evidence_cv(raw: &RawRunResult, cce: &CceRunResult) -> Cv {
    let opt_bool = |b: Option<bool>| match b {
        Some(v) => Cv::Bool(v),
        None => Cv::Null,
    };
    Cv::map(vec![
        (
            "raw_result",
            Cv::map(vec![
                ("arm", Cv::Text("raw".into())),
                ("output_digest", Cv::Text(raw.output_digest.clone())),
                ("wall_time_ms", Cv::Uint(raw.wall_time_ms)),
                ("build_pass", opt_bool(raw.build_pass)),
                ("test_pass", opt_bool(raw.test_pass)),
                (
                    "human_interventions_count",
                    Cv::Uint(u64::from(raw.human_interventions_count)),
                ),
                ("evidence_present", Cv::Bool(raw.evidence_present)),
                ("submission_order", Cv::Uint(raw.submission_order)),
            ]),
        ),
        (
            "cce_result",
            Cv::map(vec![
                ("arm", Cv::Text("cce".into())),
                ("output_digest", Cv::Text(cce.output_digest.clone())),
                ("wall_time_ms", Cv::Uint(cce.wall_time_ms)),
                ("build_pass", opt_bool(cce.build_pass)),
                ("test_pass", opt_bool(cce.test_pass)),
                (
                    "human_interventions_count",
                    Cv::Uint(u64::from(cce.human_interventions_count)),
                ),
                ("evidence_present", Cv::Bool(cce.evidence_present)),
                ("submission_order", Cv::Uint(cce.submission_order)),
                ("repo_workbody_ref", Cv::Text(cce.repo_workbody_ref.clone())),
                (
                    "gate_report_count",
                    Cv::Uint(u64::from(cce.gate_report_count)),
                ),
                ("replay_confirmed", Cv::Bool(cce.replay_confirmed)),
            ]),
        ),
    ])
}

fn benchmark_candidate_outputs_cv(cce: &CceRunResult) -> Cv {
    Cv::map(vec![(
        "outputs",
        Cv::Array(vec![Cv::map(vec![
            ("evidence_ref", Cv::Text(cce.output_digest.clone())),
            ("is_commit", Cv::Bool(false)),
            ("arm", Cv::Text("cce".into())),
            ("repo_workbody_ref", Cv::Text(cce.repo_workbody_ref.clone())),
        ])]),
    )])
}

fn benchmark_ledger_cv() -> Cv {
    Cv::map(vec![
        (
            "commits",
            Cv::Array(vec![Cv::Text("benchmark:comparison_sealed".into())]),
        ),
        (
            "gate_reports_for",
            Cv::Array(vec![Cv::Text("benchmark:comparison_sealed".into())]),
        ),
        ("closure_proof", Cv::Bool(true)),
        (
            "hdag_projection",
            Cv::Text("n/a: Benchmark ist ein Vergleichs-Meta-Koerper, kein Motorlauf".into()),
        ),
    ])
}

/// Siegelt den Benchmark-Workbody (§5). Voraussetzung (Aufrufer prueft
/// via `comparison_seal_gate`): beide Arme abgeschlossen, identischer
/// `task_package_digest`, Matrix vollstaendig.
pub fn seal_benchmark_workbody(
    package: &BenchmarkTaskPackage,
    raw: &RawRunResult,
    cce: &CceRunResult,
    matrix: &ComparisonMatrix,
) -> Result<Sealed, WorkbodyError> {
    let manifest = benchmark_manifest_cv(package, &cce.repo_workbody_ref);
    let cl = benchmark_cl_substrate_cv(package, matrix, &cce.repo_workbody_ref);
    let evidence = benchmark_evidence_cv(raw, cce);
    let candidate_outputs = benchmark_candidate_outputs_cv(cce);
    let ledger = benchmark_ledger_cv();
    let residues = Cv::map(vec![("residues", Cv::Array(vec![]))]);

    seal_canonical(
        "benchmark",
        &["benchmark"],
        &[
            seg(KIND_MANIFEST, &manifest),
            canon_desc_segment(),
            seg(KIND_CL_SUBSTRATE, &cl),
            seg(KIND_LEDGER, &ledger),
            seg(KIND_RESIDUE, &residues),
            seg(KIND_EVIDENCE, &evidence),
            seg(KIND_CANDIDATE_OUTPUTS, &candidate_outputs),
        ],
    )
    .map_err(WorkbodyError::Pack)
}
