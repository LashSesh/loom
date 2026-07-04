//! §8(d): die Kette aus Dokument 19 §4, verdrahtet ausschliesslich
//! ueber die BESTEHENDEN P1/P2-Bausteine — keine neue Kern-Logik. Diese
//! Funktion fuegt NUR die drei neuen Vor-Gates (TaskProposalGate/
//! ProtectedPathGate/BranchIsolationGate) VOR `cce_swe::kette
//! ::run_swe_task` ein, dann laeuft die unveraenderte P2-Kern-Kette
//! (apply → BuildRun → TestRun → BuildEvidenceGate/TestEvidenceGate/
//! RegressionGate → PhaseBlock).
//!
//! Was HIER bewusst NICHT passiert: `git commit` (das bleibt eine
//! separate, spaetere Aktion ueber `ToolGateway::run_git`, das die
//! HumanConfirmationGate-Aufzeichnung schon selbst erzwingt, s. P2) und
//! `git push`/Merge nach `main` (das existiert im System schlicht
//! nicht — s. `merge_exclusion_gate`).

use crate::gates::{branch_isolation_gate, protected_path_gate, task_proposal_gate, InfVerdict};
use crate::proposal::{DogfoodRun, OperatorDecision};
use cce_core::residue::Residue;
use cce_core::signature::Digest;
use cce_phaseblock::phaseblock::PhaseBlock;
use cce_swe::kette::SweOutcome;
use cce_swe::model::{DiffCandidate, RepoSnapshot};
use cce_toolgateway::gateway::{BuildTool, FsWriteTool, TestTool, ToolGateway};
use cce_toolgateway::manifest::ToolManifest;

/// Ergebnis eines Dogfood-Kern-Kette-Laufs.
#[derive(Debug)]
pub enum DogfoodOutcome {
    /// Kandidat + neuer Snapshot — der EINZIGE Erfolgspfad.
    Candidate(Box<(PhaseBlock, RepoSnapshot)>),
    /// Auftraggeber hat den Vorschlag ABGELEHNT (R-DOG-2) — Lauf endet
    /// sauber, VOR jeder Gate-Pruefung und VOR jeder Werkzeug-Beruehrung.
    RejectedByOperator { reason: String },
    /// Gate-Halt VOR jeder Werkzeug-Wirkung (TaskProposal/ProtectedPath/
    /// BranchIsolation).
    BlockedBeforeApply(Vec<InfVerdict>),
    /// P2-Kern-Kette hielt NACH dem Apply (Build/Test/Regression).
    Hold(Vec<InfVerdict>),
    /// `apply()` selbst schlug fehl.
    ApplyFailed(Box<Residue>),
}

/// Der Dogfood-Lauf. `run.decision` MUSS `Approved` sein, sonst endet
/// die Funktion SOFORT (R-DOG-2) — es wird nicht einmal ein Gate
/// geprueft, geschweige denn ein Werkzeug beruehrt.
#[allow(clippy::too_many_arguments)]
pub fn run_dogfood_task(
    run: &DogfoodRun,
    diff: &DiffCandidate,
    base: &RepoSnapshot,
    gw: &mut ToolGateway,
    fs_write_manifest: &ToolManifest,
    fs_write_tool: &mut FsWriteTool,
    build_manifest: &ToolManifest,
    build_tool: &BuildTool,
    test_manifest: &ToolManifest,
    test_tool: &TestTool,
    rd_ref: Digest,
) -> DogfoodOutcome {
    let reason = match &run.decision {
        OperatorDecision::Rejected { reason } => Some(reason.clone()),
        OperatorDecision::Approved { .. } => None,
    };
    if let Some(reason) = reason {
        return DogfoodOutcome::RejectedByOperator { reason };
    }

    // Die drei neuen Vor-Gates — VOR jeder Werkzeug-Wirkung.
    let pre_verdicts = [
        task_proposal_gate(&run.proposal),
        protected_path_gate(&run.proposal.target_scope),
        branch_isolation_gate(&run.branch_name),
    ];
    let pre_failed: Vec<InfVerdict> = pre_verdicts
        .iter()
        .filter(|v| !v.allows())
        .cloned()
        .collect();
    if !pre_failed.is_empty() {
        return DogfoodOutcome::BlockedBeforeApply(pre_failed);
    }
    // Auch jeder Diff-Hunk-Pfad muss innerhalb des freigegebenen Scopes
    // liegen (nicht nur der deklarierte target_scope selbst).
    for hunk in &diff.hunks {
        if !run
            .proposal
            .target_scope
            .iter()
            .any(|s| hunk.path.starts_with(s.as_str()))
        {
            return DogfoodOutcome::BlockedBeforeApply(vec![protected_path_gate(
                std::slice::from_ref(&hunk.path),
            )]);
        }
    }

    // Die UNVERAENDERTE P2-Kern-Kette (kein neuer Code, nur wiederverwendet).
    match cce_swe::kette::run_swe_task(
        diff,
        base,
        gw,
        fs_write_manifest,
        fs_write_tool,
        build_manifest,
        build_tool,
        test_manifest,
        test_tool,
        false,
        rd_ref,
    ) {
        SweOutcome::Candidate(boxed) => DogfoodOutcome::Candidate(boxed),
        SweOutcome::Hold(v) => DogfoodOutcome::Hold(v),
        SweOutcome::BlockedBeforeApply(v) => DogfoodOutcome::BlockedBeforeApply(v),
        SweOutcome::ApplyFailed(r) => DogfoodOutcome::ApplyFailed(r),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::proposal::TaskProposal;
    use cce_core::signature::sha256;
    use cce_swe::model::{CodeUnitRole, DiffHunk, ProducedBy};
    use cce_toolgateway::manifest::Egress;

    const ORIGINAL_DOC: &str = "# cce-swe\n\nEin Blatt-Crate.\n";
    const NEW_DOC: &str = "# cce-swe\n\nEin Blatt-Crate.\n\nSiehe Dokument 18 fuer Details.\n";
    const DIFF: &str = "@@ -1,3 +1,3 @@\n # cce-swe\n \n-Ein Blatt-Crate.\n+Ein Blatt-Crate.\n+\n+Siehe Dokument 18 fuer Details.\n";

    fn tool_manifest(class: &str, scope: &str) -> ToolManifest {
        ToolManifest {
            tool_id: format!("{class}-1"),
            tool_class: class.to_string(),
            scope: vec![scope.to_string()],
            side_effects: true,
            egress: Egress::None,
            budget_calls: 10,
            replay_strategy: "recorded".to_string(),
            lock_ref: format!("lock:{class}"),
        }
    }

    fn approved_run() -> DogfoodRun {
        let proposal = TaskProposal {
            proposal_id: "p3-demo".to_string(),
            wish_text: "Ergaenze einen Hinweis in der README von cce-swe".to_string(),
            task_class: "doc".to_string(),
            target_scope: vec!["cce/crates/cce-swe/README.md".to_string()],
            estimated_cost_budget: 1,
            rationale: "reine Dokumentations-Ergaenzung".to_string(),
        };
        DogfoodRun::new(
            proposal,
            "dogfood/p3-demo",
            OperatorDecision::Approved {
                confirmation_ref: "operator:test;ledger:e1".to_string(),
            },
        )
    }

    #[test]
    fn rejected_operator_decision_halts_before_any_gate() {
        let mut run = approved_run();
        run.decision = OperatorDecision::Rejected {
            reason: "noch nicht jetzt".to_string(),
        };
        let base = cce_swe::model::RepoSnapshot::default();
        let diff = DiffCandidate {
            base_snapshot_root: base.snapshot_root(),
            hunks: vec![],
            rationale: "irrelevant".to_string(),
            produced_by: ProducedBy::Operator {
                operator: "op".to_string(),
            },
        };
        let mut gw = ToolGateway::new();
        let mut fs_write_tool = FsWriteTool::default();
        let fs_write_manifest = tool_manifest("fs_write", "cce/crates/cce-swe/");
        let build_manifest = tool_manifest("build", "cce/crates/cce-swe/");
        let build_tool = BuildTool::default();
        let test_manifest = tool_manifest("test", "cce/crates/cce-swe/");
        let test_tool = TestTool::default();

        let outcome = run_dogfood_task(
            &run,
            &diff,
            &base,
            &mut gw,
            &fs_write_manifest,
            &mut fs_write_tool,
            &build_manifest,
            &build_tool,
            &test_manifest,
            &test_tool,
            sha256(b"rd:dogfood-rejected"),
        );
        match outcome {
            DogfoodOutcome::RejectedByOperator { reason } => {
                assert_eq!(reason, "noch nicht jetzt");
            }
            other => panic!("erwartet RejectedByOperator, war {other:?}"),
        }
        // Kein Lock geoeffnet, kein Tool beruehrt.
        assert!(gw.records.is_empty());
    }

    #[test]
    fn approved_doc_task_runs_full_chain_to_phaseblock() {
        let run = approved_run();
        let base = cce_swe::model::RepoSnapshot::from_files(
            &[(
                "cce/crates/cce-swe/README.md",
                "markdown",
                CodeUnitRole::Doc,
                ORIGINAL_DOC.as_bytes(),
            )],
            "rustc-1.0",
            None,
        );
        let diff = DiffCandidate {
            base_snapshot_root: base.snapshot_root(),
            hunks: vec![DiffHunk {
                path: "cce/crates/cce-swe/README.md".to_string(),
                unified_diff: DIFF.to_string(),
            }],
            rationale: run.proposal.rationale.clone(),
            produced_by: ProducedBy::Operator {
                operator: "operator:sk".to_string(),
            },
        };
        let mut gw = ToolGateway::new();
        gw.open_lock("fs_write", "op", "l1");
        gw.open_lock("build", "op", "l1");
        gw.open_lock("test", "op", "l1");
        let mut fs_write_tool =
            FsWriteTool::with_files(&[("cce/crates/cce-swe/README.md", ORIGINAL_DOC.as_bytes())]);
        let fs_write_manifest = tool_manifest("fs_write", "cce/crates/cce-swe/");
        let build_manifest = tool_manifest("build", "cce/crates/cce-swe/");
        let build_tool = BuildTool::with_fixture(&["cargo", "build"], 0, "Compiling ok");
        let test_manifest = tool_manifest("test", "cce/crates/cce-swe/");
        let test_tool = TestTool::with_fixture(&["cargo", "test"], 0, "test result: ok");

        let outcome = run_dogfood_task(
            &run,
            &diff,
            &base,
            &mut gw,
            &fs_write_manifest,
            &mut fs_write_tool,
            &build_manifest,
            &build_tool,
            &test_manifest,
            &test_tool,
            sha256(b"rd:dogfood-approved"),
        );
        match outcome {
            DogfoodOutcome::Candidate(_) => {}
            other => panic!("erwartet Candidate, war {other:?}"),
        }
        assert_eq!(
            fs_write_tool
                .files
                .get("cce/crates/cce-swe/README.md")
                .unwrap(),
            NEW_DOC.as_bytes()
        );
    }

    #[test]
    fn diff_hunk_outside_approved_scope_is_blocked() {
        let run = approved_run();
        let base = cce_swe::model::RepoSnapshot::default();
        let diff = DiffCandidate {
            base_snapshot_root: base.snapshot_root(),
            hunks: vec![DiffHunk {
                path: "cce/crates/cce-core/src/gate.rs".to_string(),
                unified_diff: "irrelevant".to_string(),
            }],
            rationale: "sollte nie ankommen".to_string(),
            produced_by: ProducedBy::Operator {
                operator: "op".to_string(),
            },
        };
        let mut gw = ToolGateway::new();
        gw.open_lock("fs_write", "op", "l1");
        gw.open_lock("build", "op", "l1");
        gw.open_lock("test", "op", "l1");
        let mut fs_write_tool = FsWriteTool::default();
        let fs_write_manifest = tool_manifest("fs_write", "cce/crates/cce-swe/");
        let build_manifest = tool_manifest("build", "cce/crates/cce-swe/");
        let build_tool = BuildTool::default();
        let test_manifest = tool_manifest("test", "cce/crates/cce-swe/");
        let test_tool = TestTool::default();

        let outcome = run_dogfood_task(
            &run,
            &diff,
            &base,
            &mut gw,
            &fs_write_manifest,
            &mut fs_write_tool,
            &build_manifest,
            &build_tool,
            &test_manifest,
            &test_tool,
            sha256(b"rd:dogfood-outside-scope"),
        );
        match outcome {
            DogfoodOutcome::BlockedBeforeApply(failed) => {
                assert!(failed.iter().any(|v| v.gate() == "ProtectedPathGate"));
            }
            other => panic!("erwartet BlockedBeforeApply, war {other:?}"),
        }
        assert!(gw.records.is_empty());
    }
}
