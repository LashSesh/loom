//! Dogfooding-Negativzeugenkatalog (Dokument 19, P3) — Ablage
//! `conformance/dogfood/`, dauerhaft im einen Waechter. N-DOG-1..4
//! laufen hermetisch (kein Netz, kein echter Subprozess) und gruen in
//! der normalen CI. Der eine reale Beweis (R-DOG-1) ist separat als
//! `#[ignore]`-Test in `crates/cce-dogfood/tests/
//! r_dog_1_betriebsverifikation.rs` gefuehrt — er lief real am
//! 2026-07-04 gegen das echte Repository (Details:
//! `reports/P3_dogfooding_bericht.md`).

use cce_core::signature::sha256;
use cce_dogfood::gates::task_proposal_gate;
use cce_dogfood::kette::{run_dogfood_task, DogfoodOutcome};
use cce_dogfood::proposal::{DogfoodRun, OperatorDecision, TaskProposal};
use cce_swe::model::{CodeUnitRole, DiffCandidate, DiffHunk, ProducedBy, RepoSnapshot};
use cce_toolgateway::gateway::{
    BuildTool, FsWriteTool, GitOperation, GitTool, TestTool, ToolGateway,
};
use cce_toolgateway::manifest::{Egress, ToolManifest};

const ORIGINAL_DOC: &str = "# cce-swe\n\nEin Blatt-Crate.\n";
const FIX_DIFF: &str =
    "@@ -1,3 +1,3 @@\n # cce-swe\n \n-Ein Blatt-Crate.\n+Ein Blatt-Crate (P3).\n";

fn tool_manifest(class: &str) -> ToolManifest {
    ToolManifest {
        tool_id: format!("{class}-1"),
        tool_class: class.to_string(),
        scope: vec!["cce/crates/cce-swe/".to_string()],
        side_effects: true,
        egress: Egress::None,
        budget_calls: 5,
        replay_strategy: "recorded".to_string(),
        lock_ref: format!("lock:{class}"),
    }
}

fn base_snapshot() -> RepoSnapshot {
    RepoSnapshot::from_files(
        &[(
            "cce/crates/cce-swe/README.md",
            "markdown",
            CodeUnitRole::Doc,
            ORIGINAL_DOC.as_bytes(),
        )],
        "rustc-1.0",
        None,
    )
}

fn whitelisted_proposal() -> TaskProposal {
    TaskProposal {
        proposal_id: "n-dog-demo".to_string(),
        wish_text: "Beispiel-Wunsch fuer den Waechter".to_string(),
        task_class: "doc".to_string(),
        target_scope: vec!["cce/crates/cce-swe/README.md".to_string()],
        estimated_cost_budget: 1,
        rationale: "Zeugen-Fixture".to_string(),
    }
}

fn approved_run(proposal: TaskProposal, branch: &str) -> DogfoodRun {
    DogfoodRun::new(
        proposal,
        branch,
        OperatorDecision::Approved {
            confirmation_ref: "operator:waechter-fixture".to_string(),
        },
    )
}

fn gateway_with_locks() -> ToolGateway {
    let mut gw = ToolGateway::new();
    gw.open_lock("fs_write", "op", "l1");
    gw.open_lock("build", "op", "l1");
    gw.open_lock("test", "op", "l1");
    gw
}

/// N-DOG-1: Task ausserhalb der Whitelist-Klassen ⇒ TaskProposalGate
/// reject.
#[test]
fn n_dog_1_task_outside_whitelist_is_rejected() {
    let mut proposal = whitelisted_proposal();
    proposal.task_class = "refactor".to_string();
    let verdict = task_proposal_gate(&proposal);
    assert!(!verdict.allows());
    assert!(verdict
        .residue()
        .unwrap()
        .id
        .contains("task_class_not_whitelisted"));

    // Auch am vollen Kern-Kette-Einstieg: BlockedBeforeApply, kein Apply.
    let run = approved_run(proposal.clone(), "dogfood/p3-n-dog-1");
    let base = base_snapshot();
    let diff = DiffCandidate {
        base_snapshot_root: base.snapshot_root(),
        hunks: vec![DiffHunk {
            path: "cce/crates/cce-swe/README.md".to_string(),
            unified_diff: FIX_DIFF.to_string(),
        }],
        rationale: "sollte nie ankommen".to_string(),
        produced_by: ProducedBy::Operator {
            operator: "op".to_string(),
        },
    };
    let mut gw = gateway_with_locks();
    let mut fs_write_tool =
        FsWriteTool::with_files(&[("cce/crates/cce-swe/README.md", ORIGINAL_DOC.as_bytes())]);
    let fs_write_manifest = tool_manifest("fs_write");
    let build_manifest = tool_manifest("build");
    let build_tool = BuildTool::with_fixture(&["cargo", "build"], 0, "ok");
    let test_manifest = tool_manifest("test");
    let test_tool = TestTool::with_fixture(&["cargo", "test"], 0, "ok");
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
        sha256(b"rd:n-dog-1"),
    );
    match outcome {
        DogfoodOutcome::BlockedBeforeApply(failed) => {
            assert!(failed.iter().any(|v| v.gate() == "TaskProposalGate"));
        }
        other => panic!("erwartet BlockedBeforeApply, war {other:?}"),
    }
    assert!(
        gw.records.is_empty(),
        "kein Werkzeug darf beruehrt worden sein"
    );
}

/// N-DOG-2: Zielpfad in der Schutzzone ⇒ `protected_path_violation`,
/// reject VOR jedem Apply.
#[test]
fn n_dog_2_target_in_protected_zone_is_rejected_before_apply() {
    let mut proposal = whitelisted_proposal();
    proposal.target_scope = vec!["cce/crates/cce-core/src/gate.rs".to_string()];
    let run = approved_run(proposal.clone(), "dogfood/p3-n-dog-2");
    let base = base_snapshot();
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
    let mut gw = gateway_with_locks();
    let mut fs_write_tool = FsWriteTool::default();
    let fs_write_manifest = tool_manifest("fs_write");
    let build_manifest = tool_manifest("build");
    let build_tool = BuildTool::default();
    let test_manifest = tool_manifest("test");
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
        sha256(b"rd:n-dog-2"),
    );
    match outcome {
        DogfoodOutcome::BlockedBeforeApply(failed) => {
            let v = failed
                .iter()
                .find(|v| v.gate() == "ProtectedPathGate")
                .expect("ProtectedPathGate haelt");
            assert!(v.residue().unwrap().id.contains("protected_path_violation"));
        }
        other => panic!("erwartet BlockedBeforeApply, war {other:?}"),
    }
    assert!(
        gw.records.is_empty(),
        "kein Werkzeug darf beruehrt worden sein"
    );
}

/// N-DOG-3: Schreibversuch auf `main` ⇒ `BranchIsolationGate` reject,
/// keine Ausnahme.
#[test]
fn n_dog_3_write_attempt_on_main_is_rejected() {
    let proposal = whitelisted_proposal();
    let run = approved_run(proposal, "main");
    let base = base_snapshot();
    let diff = DiffCandidate {
        base_snapshot_root: base.snapshot_root(),
        hunks: vec![DiffHunk {
            path: "cce/crates/cce-swe/README.md".to_string(),
            unified_diff: FIX_DIFF.to_string(),
        }],
        rationale: "sollte nie ankommen".to_string(),
        produced_by: ProducedBy::Operator {
            operator: "op".to_string(),
        },
    };
    let mut gw = gateway_with_locks();
    let mut fs_write_tool =
        FsWriteTool::with_files(&[("cce/crates/cce-swe/README.md", ORIGINAL_DOC.as_bytes())]);
    let fs_write_manifest = tool_manifest("fs_write");
    let build_manifest = tool_manifest("build");
    let build_tool = BuildTool::with_fixture(&["cargo", "build"], 0, "ok");
    let test_manifest = tool_manifest("test");
    let test_tool = TestTool::with_fixture(&["cargo", "test"], 0, "ok");
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
        sha256(b"rd:n-dog-3"),
    );
    match outcome {
        DogfoodOutcome::BlockedBeforeApply(failed) => {
            let v = failed
                .iter()
                .find(|v| v.gate() == "BranchIsolationGate")
                .expect("BranchIsolationGate haelt");
            assert!(v
                .residue()
                .unwrap()
                .id
                .contains("branch_isolation_violation"));
        }
        other => panic!("erwartet BlockedBeforeApply, war {other:?}"),
    }
    assert!(
        gw.records.is_empty(),
        "kein Werkzeug darf beruehrt worden sein"
    );
}

/// N-DOG-4: Commit ohne aufgezeichnete Auftraggeber-Bestaetigung ⇒
/// reject (P2-Disziplin, hier fuer den Dogfood-Pfad erneut bewiesen --
/// `ToolGateway::run_git` ist exakt derselbe Code wie in P2, unveraendert).
#[test]
fn n_dog_4_commit_without_confirmation_is_rejected() {
    let mut gw = ToolGateway::new();
    gw.open_lock("git", "op", "l1");
    let mut tool = GitTool::default();
    let manifest = tool_manifest("git");
    let err = gw
        .run_git(
            &manifest,
            &mut tool,
            &GitOperation::Commit("P3-Demo-Commit".to_string()),
            None,
        )
        .unwrap_err();
    assert!(err.id.contains("tool_commit_without_confirmation"));
    assert!(
        tool.commits.is_empty(),
        "ohne Bestaetigung darf nichts committet werden"
    );

    // Mit Bestaetigung geht es durch -- derselbe Pfad, der R-DOG-1 real
    // erfolgreich commitete.
    gw.run_git(
        &manifest,
        &mut tool,
        &GitOperation::Commit("P3-Demo-Commit".to_string()),
        Some("operator:auftraggeber"),
    )
    .expect("mit Bestaetigung gelingt der Commit");
    assert_eq!(tool.commits, vec!["P3-Demo-Commit".to_string()]);
}
