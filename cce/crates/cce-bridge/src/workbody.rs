//! S-E5 §10(e): der Norm-Workbody-Bau. Eine Norm IST ein `.loom`-
//! Workbody der neuen Containerklasse `"norm"` — sie erbt damit alles
//! Bestehende (verify, Signatur, Transport, Registry-Eintrag). Nur ein
//! `BridgeVerdict::Allow` darf hier ankommen (das Kandidaten-Commit-
//! Verbot gilt unveraendert, s. `gate.rs`).

use crate::gate::BridgeGateReport;
use crate::types::{BridgeNorm, BridgeVerdict, NormCandidate, NormStatus};
use cce_core::canonical::Canonicalize;
use cce_core::replay::HitlDecision;
use loom_canon::Cv;
use loom_cites::{cites_field, external_citations_hex};
use loom_codec::{seal_canonical, PackError, Sealed, Segment};
use loom_format::{
    KIND_CANON_DESC, KIND_CL_SUBSTRATE, KIND_EVIDENCE, KIND_LEDGER, KIND_MANIFEST,
    KIND_REPLAY_MANIFEST, KIND_RESIDUE,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkbodyError {
    /// Nur `Allow` darf einen Norm-Workbody erzeugen.
    NotAllowed(BridgeVerdict),
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

fn norm_manifest_cv(norm: &BridgeNorm) -> Cv {
    let external_citations = external_citations_hex(&norm.derives_cites());
    Cv::map(vec![
        (
            "title",
            Cv::Text(format!(
                "Norm ({}): {}",
                norm.nexus_class().as_str(),
                norm.pattern.describe()
            )),
        ),
        ("container_class", Cv::Text("norm".into())),
        (
            "domain_refs",
            Cv::Array(vec![Cv::Text(norm.scope.as_string())]),
        ),
        ("scale", Cv::Uint(1)),
        ("pl_level", Cv::Text("PL2".into())),
        ("claims", Cv::map(vec![("closed", Cv::Bool(true))])),
        (
            "origin",
            Cv::map(vec![
                ("tool", Cv::Text("cce-bridge-0.1".into())),
                ("rd_digest", Cv::Text(norm.norm_id.clone())),
            ]),
        ),
        (
            "profiles_required",
            Cv::Array(vec![Cv::Text("norm".into())]),
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
            Cv::Tag(0, Box::new(Cv::Text("2026-01-01T00:00:00Z".into()))),
        ),
        (
            "external_citations",
            Cv::Array(external_citations.into_iter().map(Cv::Text).collect()),
        ),
        // Norm-spezifische additive Felder (S-E5 §2) — additiv/minor
        // neben den 13 Pflichtfeldern (Teil 3.5).
        ("nexus_class", Cv::Text(norm.nexus_class().as_str().into())),
        ("norm_id", Cv::Text(norm.norm_id.clone())),
        ("norm_scope", Cv::Text(norm.scope.as_string())),
        ("norm_status", Cv::Text(norm.status.as_str().into())),
        (
            "supersedes",
            match &norm.supersedes {
                Some(s) => Cv::Text(s.clone()),
                None => Cv::Null,
            },
        ),
    ])
}

fn norm_cl_substrate_cv(norm: &BridgeNorm) -> Cv {
    Cv::map(vec![
        ("cubes", Cv::Array(vec![])),
        ("constraints", Cv::Array(vec![])),
        ("cites", cites_field(&norm.derives_cites())),
        (
            "norm_pattern",
            Cv::map(vec![
                ("nexus_class", Cv::Text(norm.nexus_class().as_str().into())),
                ("pattern", norm.pattern.to_cv()),
                ("scope", Cv::Text(norm.scope.as_string())),
            ]),
        ),
    ])
}

fn norm_evidence_cv(
    candidate: &NormCandidate,
    report: &BridgeGateReport,
    hitl_decisions: &[HitlDecision],
) -> Cv {
    let verdict_str = match report.verdict() {
        BridgeVerdict::Allow => "allow",
        BridgeVerdict::Hold => "hold",
        BridgeVerdict::Reject => "reject",
    };
    Cv::map(vec![
        ("bridgegate_verdict", Cv::Text(verdict_str.into())),
        ("n_support", Cv::Uint(candidate.n_support)),
        ("n_counter", Cv::Uint(candidate.n_counter)),
        (
            "known_counterexamples",
            Cv::Array(
                candidate
                    .known_counterexamples
                    .iter()
                    .map(|c| {
                        Cv::map(vec![
                            ("core_root", Cv::Text(c.core_root_hex.clone())),
                            ("reason", Cv::Text(c.reason.clone())),
                        ])
                    })
                    .collect(),
            ),
        ),
        (
            "hitl_decisions",
            Cv::Array(
                hitl_decisions
                    .iter()
                    .map(|d| {
                        Cv::map(vec![
                            ("gate", Cv::Text(d.gate.clone())),
                            ("decision", Cv::Text(d.decision.clone())),
                            ("operator", Cv::Text(d.operator.clone())),
                        ])
                    })
                    .collect(),
            ),
        ),
    ])
}

fn norm_ledger_cv(allowed: bool) -> Cv {
    Cv::map(vec![
        (
            "commits",
            Cv::Array(vec![Cv::Text("bridgegate:promotion".into())]),
        ),
        (
            "gate_reports_for",
            Cv::Array(vec![Cv::Text("bridgegate:promotion".into())]),
        ),
        ("closure_proof", Cv::Bool(allowed)),
        (
            "hdag_projection",
            Cv::Text(
                "n/a: Destillation ist Meta-Ebene ueber bereits geschlossenen Arbeiten, kein Motorlauf"
                    .into(),
            ),
        ),
    ])
}

fn norm_replay_manifest_cv(candidate: &NormCandidate) -> Cv {
    let commit_class_hex = candidate.canonical_class().0.to_hex();
    let inputs: Vec<&str> = candidate
        .provenance_set
        .members
        .iter()
        .map(|s| s.as_str())
        .collect();
    loom_replay::replay_manifest_segment_with_inputs(
        &candidate.distillation_rd_class_hex,
        0,
        &commit_class_hex,
        &inputs,
    )
}

/// Siegelt den Norm-Workbody. `norm_id` = die kanonische Klasse des
/// `NormCandidate` (Pattern + Herkunft + Scope + RD-Klasse) — die Norm
/// IST dieser Inhalt, nicht ein Verweis darauf.
pub fn seal_norm(
    candidate: &NormCandidate,
    report: &BridgeGateReport,
    hitl_decisions: &[HitlDecision],
) -> Result<(BridgeNorm, Sealed), WorkbodyError> {
    if report.verdict() != BridgeVerdict::Allow {
        return Err(WorkbodyError::NotAllowed(report.verdict()));
    }
    let norm_id = candidate.canonical_class().0.to_hex();
    let norm = BridgeNorm {
        norm_id,
        pattern: candidate.pattern.clone(),
        provenance_set: candidate.provenance_set.clone(),
        known_counterexamples: candidate.known_counterexamples.clone(),
        scope: candidate.scope.clone(),
        status: NormStatus::Active,
        promotion_evidence: format!(
            "bridgegate:allow;n_support={};n_counter={}",
            candidate.n_support, candidate.n_counter
        ),
        supersedes: None,
    };

    let manifest = norm_manifest_cv(&norm);
    let cl = norm_cl_substrate_cv(&norm);
    let evidence = norm_evidence_cv(candidate, report, hitl_decisions);
    let ledger = norm_ledger_cv(true);
    let residues = Cv::map(vec![("residues", Cv::Array(vec![]))]);
    let replay = norm_replay_manifest_cv(candidate);

    let sealed = seal_canonical(
        "norm",
        &["norm"],
        &[
            seg(KIND_MANIFEST, &manifest),
            canon_desc_segment(),
            seg(KIND_CL_SUBSTRATE, &cl),
            seg(KIND_LEDGER, &ledger),
            seg(KIND_RESIDUE, &residues),
            seg(KIND_EVIDENCE, &evidence),
            seg(KIND_REPLAY_MANIFEST, &replay),
        ],
    )
    .map_err(WorkbodyError::Pack)?;

    Ok((norm, sealed))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gate::{bridge_gate, BridgeGateContext};
    use crate::pattern::{DomainRuleForm, Pattern};
    use crate::types::{ProvenanceSet, Scope};

    fn base_candidate() -> NormCandidate {
        NormCandidate {
            pattern: Pattern::StructuralRule(DomainRuleForm::Relation {
                seam: "refers".to_string(),
            }),
            provenance_set: ProvenanceSet::new(vec![
                "a1".repeat(34),
                "a2".repeat(34),
                "a3".repeat(34),
            ]),
            n_support: 3,
            n_counter: 0,
            known_counterexamples: vec![],
            scope: Scope::Global,
            distillation_rd_class_hex: "rdclass".to_string(),
        }
    }

    #[test]
    fn allowed_candidate_seals_valid_norm_container() {
        let candidate = base_candidate();
        let domains = vec!["dom:a".to_string(), "dom:b".to_string()];
        let ctx = BridgeGateContext::new(&domains, &[]);
        let report = bridge_gate(&candidate, &ctx);
        assert_eq!(report.verdict(), BridgeVerdict::Allow);

        let (norm, sealed) = seal_norm(&candidate, &report, &[]).expect("Norm-Siegelung gelingt");
        assert_eq!(norm.status, NormStatus::Active);

        let verification = loom_verify::verify(&sealed.bytes);
        assert_eq!(
            verification.verdict,
            loom_verify::Verdict::Valid,
            "{:?}",
            verification.diagnoses
        );
    }

    /// Kandidaten-Commit-Verbot: ein Hold/Reject-Verdikt darf NIEMALS
    /// einen Norm-Workbody erzeugen.
    #[test]
    fn non_allow_verdict_is_refused() {
        let mut candidate = base_candidate();
        candidate.provenance_set = ProvenanceSet::new(vec!["a1".repeat(34)]);
        let domains = vec!["dom:a".to_string()];
        let ctx = BridgeGateContext::new(&domains, &[]);
        let report = bridge_gate(&candidate, &ctx);
        assert_eq!(report.verdict(), BridgeVerdict::Hold);
        match seal_norm(&candidate, &report, &[]) {
            Err(err) => assert_eq!(err, WorkbodyError::NotAllowed(BridgeVerdict::Hold)),
            Ok(_) => panic!("Hold-Verdikt darf niemals einen Norm-Workbody erzeugen"),
        }
    }
}
