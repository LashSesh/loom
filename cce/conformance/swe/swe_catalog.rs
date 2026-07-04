//! SWE-Zeugenkatalog (Dokument 18, P2) — Ablage `conformance/swe/`,
//! dauerhaft im einen Waechter. 5 Referenzen (gruen) + 8 Negative (rot).

use cce_core::capability::CapabilityLock;
use cce_core::signature::sha256;
use cce_inference::request::InferenceRequest;
use cce_phaseblock::accept::{accept_block, AcceptContext, AcceptOutcome};
use cce_swe::gates::{
    build_evidence_gate, no_direct_commit_gate, regression_gate, tool_capability_gate,
    tool_egress_gate, tool_scope_gate,
};
use cce_swe::kette::{run_swe_task, SweOutcome};
use cce_swe::model::{CodeUnitRole, DiffCandidate, DiffHunk, ProducedBy, RepoSnapshot, TaskLedger};
use cce_swe::provider_diff::{
    provider_diff_candidate, ProviderDiffError, RecordedOpenAiDiffProvider,
};
use cce_swe::workbody::seal_repo_workbody;
use cce_toolgateway::gateway::{
    BuildTool, FsWriteTool, GitOperation, GitTool, TestTool, ToolGateway,
};
use cce_toolgateway::manifest::{Egress, ToolManifest};

const ORIGINAL_LIB: &str = "fn add(a: i32, b: i32) -> i32 {\n    a - b\n}\n";
const FIXED_LIB: &str = "fn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n";
const FIX_DIFF: &str =
    "@@ -1,3 +1,3 @@\n fn add(a: i32, b: i32) -> i32 {\n-    a - b\n+    a + b\n }\n";

fn tool_manifest(class: &str) -> ToolManifest {
    ToolManifest {
        tool_id: format!("{class}-1"),
        tool_class: class.to_string(),
        scope: vec!["repo/".to_string()],
        side_effects: true,
        egress: Egress::None,
        budget_calls: 10,
        replay_strategy: "recorded".to_string(),
        lock_ref: format!("lock:{class}"),
    }
}

fn base_snapshot() -> RepoSnapshot {
    RepoSnapshot::from_files(
        &[(
            "repo/src/lib.rs",
            "rust",
            CodeUnitRole::Source,
            ORIGINAL_LIB.as_bytes(),
        )],
        "rustc-1.0",
        Some("deadbeef"),
    )
}

fn fix_diff_candidate(base: &RepoSnapshot) -> DiffCandidate {
    DiffCandidate {
        base_snapshot_root: base.snapshot_root(),
        hunks: vec![DiffHunk {
            path: "repo/src/lib.rs".to_string(),
            unified_diff: FIX_DIFF.to_string(),
        }],
        rationale: "behebt den Vorzeichenfehler in add()".to_string(),
        produced_by: ProducedBy::Operator {
            operator: "operator:sk".to_string(),
        },
    }
}

/// Gateway mit fs_write/build/test-Locks offen (Standardaufbau fuer die
/// gruenen Referenzen).
fn gateway_with_locks() -> ToolGateway {
    let mut gw = ToolGateway::new();
    gw.open_lock("fs_write", "op", "l1");
    gw.open_lock("build", "op", "l1");
    gw.open_lock("test", "op", "l1");
    gw
}

// ---------- Referenzen R-SWE-1..5 (gruen) ----------

/// R-SWE-1: RepoSnapshot eines kleinen Referenz-Repos — snapshot_root
/// deterministisch ueber zwei Laeufe, unabhaengig von der
/// Eingabereihenfolge der Dateien.
#[test]
fn r_swe_1_repo_snapshot_deterministic_across_two_runs() {
    let a = RepoSnapshot::from_files(
        &[
            (
                "src/lib.rs",
                "rust",
                CodeUnitRole::Source,
                ORIGINAL_LIB.as_bytes(),
            ),
            (
                "tests/it.rs",
                "rust",
                CodeUnitRole::Test,
                b"assert_eq!(add(2,2),4);",
            ),
        ],
        "rustc-1.0",
        Some("deadbeef"),
    );
    let b = RepoSnapshot::from_files(
        &[
            (
                "tests/it.rs",
                "rust",
                CodeUnitRole::Test,
                b"assert_eq!(add(2,2),4);",
            ),
            (
                "src/lib.rs",
                "rust",
                CodeUnitRole::Source,
                ORIGINAL_LIB.as_bytes(),
            ),
        ],
        "rustc-1.0",
        Some("deadbeef"),
    );
    assert_eq!(a.snapshot_root(), b.snapshot_root());
}

/// R-SWE-2: DiffCandidate behebt den failing test → apply(dry) →
/// BuildRun exit 0 → TestRun gruen → alle Gates → PhaseBlock; der
/// TaskLedger ist ein zertifizierter RepoWorkbody (verify Valid).
#[test]
fn r_swe_2_diff_fixes_failing_test_and_seals_valid_repo_workbody() {
    let base = base_snapshot();
    let diff = fix_diff_candidate(&base);
    let mut gw = gateway_with_locks();
    let mut fs_write_tool =
        FsWriteTool::with_files(&[("repo/src/lib.rs", ORIGINAL_LIB.as_bytes())]);
    let fs_write_manifest = tool_manifest("fs_write");
    let build_manifest = tool_manifest("build");
    let build_tool = BuildTool::with_fixture(&["cargo", "build"], 0, "Compiling ok");
    let test_manifest = tool_manifest("test");
    let test_tool = TestTool::with_fixture(&["cargo", "test"], 0, "test result: ok. 1 passed");
    let rd_ref = sha256(b"rd:r-swe-2");

    let outcome = run_swe_task(
        &diff,
        &base,
        &mut gw,
        &fs_write_manifest,
        &mut fs_write_tool,
        &build_manifest,
        &build_tool,
        &test_manifest,
        &test_tool,
        false,
        rd_ref,
    );
    let (mut block, new_snapshot) = match outcome {
        SweOutcome::Candidate(boxed) => *boxed,
        other => panic!("erwartet Candidate, war {other:?}"),
    };
    assert_eq!(
        fs_write_tool.files.get("repo/src/lib.rs").unwrap(),
        FIXED_LIB.as_bytes()
    );

    assert_eq!(
        accept_block(&mut block, &AcceptContext::all_true()),
        AcceptOutcome::Accepted
    );
    let mut ledger = TaskLedger::new(rd_ref);
    ledger.push(block);
    assert!(ledger.all_accepted());

    let tool_manifests = [&fs_write_manifest, &build_manifest, &test_manifest];
    let sealed = seal_repo_workbody(
        "task:r-swe-2",
        &new_snapshot,
        &fs_write_tool.files,
        &ledger,
        &diff,
        &tool_manifests,
    )
    .expect("Siegelung gelingt");

    let verification = loom_verify::verify(&sealed.bytes);
    assert_eq!(
        verification.verdict,
        loom_verify::Verdict::Valid,
        "{:?}",
        verification.diagnoses
    );
}

/// R-SWE-3: derselbe Auftrag, zweiter Lauf — Ergebnisklasse identisch
/// (gleicher base_snapshot_root + gleiche RD ⇒ gleiche
/// DiffCandidate-/BuildRun-/TestRun-Klasse, §5).
#[test]
fn r_swe_3_same_task_twice_yields_identical_result_class() {
    fn run_once() -> (cce_core::signature::Digest, cce_core::signature::Digest) {
        let base = base_snapshot();
        let diff = fix_diff_candidate(&base);
        let mut gw = gateway_with_locks();
        let mut fs_write_tool =
            FsWriteTool::with_files(&[("repo/src/lib.rs", ORIGINAL_LIB.as_bytes())]);
        let fs_write_manifest = tool_manifest("fs_write");
        let build_manifest = tool_manifest("build");
        let build_tool = BuildTool::with_fixture(&["cargo", "build"], 0, "Compiling ok");
        let test_manifest = tool_manifest("test");
        let test_tool = TestTool::with_fixture(&["cargo", "test"], 0, "test result: ok. 1 passed");
        let rd_ref = sha256(b"rd:r-swe-3");
        let outcome = run_swe_task(
            &diff,
            &base,
            &mut gw,
            &fs_write_manifest,
            &mut fs_write_tool,
            &build_manifest,
            &build_tool,
            &test_manifest,
            &test_tool,
            false,
            rd_ref,
        );
        match outcome {
            SweOutcome::Candidate(boxed) => {
                let (block, snapshot) = *boxed;
                (block.payload_digest, snapshot.snapshot_root())
            }
            other => panic!("erwartet Candidate, war {other:?}"),
        }
    }
    let (p1, r1) = run_once();
    let (p2, r2) = run_once();
    assert_eq!(
        p1, p2,
        "gleicher Auftrag ⇒ gleiche PhaseBlock-Payload-Klasse"
    );
    assert_eq!(r1, r2, "gleicher Auftrag ⇒ gleicher Snapshot-Root");
}

/// R-SWE-4: git commit als materielle Aktion ⇒ HumanConfirmationGate-
/// Bestaetigung aufgezeichnet (ohne Bestaetigung: reject; mit
/// Bestaetigung: geht durch und wird im GitTool-Zustand sichtbar).
#[test]
fn r_swe_4_git_commit_records_human_confirmation() {
    let mut gw = ToolGateway::new();
    gw.open_lock("git", "op", "l1");
    let mut tool = GitTool::default();
    let m = tool_manifest("git");

    let err = gw
        .run_git(
            &m,
            &mut tool,
            &GitOperation::Commit("fix add()".into()),
            None,
        )
        .unwrap_err();
    assert!(err.id.contains("tool_commit_without_confirmation"));

    gw.run_git(
        &m,
        &mut tool,
        &GitOperation::Commit("fix add()".into()),
        Some("operator:sk;ledger:e1"),
    )
    .expect("Commit mit aufgezeichneter Bestaetigung gelingt");
    assert_eq!(tool.commits, vec!["fix add()".to_string()]);
}

/// R-SWE-5: eine DiffCandidate, die einen Provider nutzt (OpenAI
/// hinter dem Gateway, recorded) — die modellerzeugte Diff durchlaeuft
/// dieselbe Kette (Gate-Kette + Kern-Kette bis PhaseBlock).
#[test]
fn r_swe_5_provider_generated_diff_runs_full_chain_to_phaseblock() {
    let base = base_snapshot();
    let provider = RecordedOpenAiDiffProvider {
        model_id: "gpt-4o-mini".to_string(),
        recorded_diff: FIX_DIFF.to_string(),
    };
    let req = InferenceRequest::example("r-swe-5-catalog");
    let mut lock = CapabilityLock::closed("model_egress:cloud-openai");
    lock.open("operator:sk", "ledger:e1");
    let mut rec = cce_inference::gateway::InferenceRecorder::default();

    let diff = provider_diff_candidate(
        &provider,
        &req,
        "repo/src/lib.rs",
        "OpenAI-erzeugte Korrektur",
        base.snapshot_root(),
        &lock,
        &mut rec,
    )
    .expect("Provider-Diff gelingt");
    assert!(matches!(diff.produced_by, ProducedBy::Provider { .. }));

    let mut gw = gateway_with_locks();
    let mut fs_write_tool =
        FsWriteTool::with_files(&[("repo/src/lib.rs", ORIGINAL_LIB.as_bytes())]);
    let fs_write_manifest = tool_manifest("fs_write");
    let build_manifest = tool_manifest("build");
    let build_tool = BuildTool::with_fixture(&["cargo", "build"], 0, "Compiling ok");
    let test_manifest = tool_manifest("test");
    let test_tool = TestTool::with_fixture(&["cargo", "test"], 0, "test result: ok. 1 passed");
    let outcome = run_swe_task(
        &diff,
        &base,
        &mut gw,
        &fs_write_manifest,
        &mut fs_write_tool,
        &build_manifest,
        &build_tool,
        &test_manifest,
        &test_tool,
        false,
        sha256(b"rd:r-swe-5"),
    );
    match outcome {
        SweOutcome::Candidate(_) => {}
        other => panic!("erwartet Candidate, war {other:?}"),
    }
}

// ---------- Negative N-SWE-1..8 (rot) ----------

/// N-SWE-1: DiffCandidate ohne gruenen BuildRun ⇒ `build_unverified`,
/// kein PhaseBlock (build-Lock bleibt zu — kein BuildRun als Evidence).
#[test]
fn n_swe_1_diff_without_green_build_is_build_unverified() {
    let base = base_snapshot();
    let diff = fix_diff_candidate(&base);
    let mut gw = ToolGateway::new();
    gw.open_lock("fs_write", "op", "l1");
    // "build" und "test" bewusst NICHT gelockt: run_build/run_test
    // schlagen fehl ⇒ kein BuildRun/TestRun als Evidence.
    let mut fs_write_tool =
        FsWriteTool::with_files(&[("repo/src/lib.rs", ORIGINAL_LIB.as_bytes())]);
    let fs_write_manifest = tool_manifest("fs_write");
    let build_manifest = tool_manifest("build");
    let build_tool = BuildTool::with_fixture(&["cargo", "build"], 0, "ok");
    let test_manifest = tool_manifest("test");
    let test_tool = TestTool::with_fixture(&["cargo", "test"], 0, "ok");

    // Ohne offene build/test-Locks haelt bereits die Vor-Wirkungs-
    // Gate-Kette (ToolCapabilityGate) VOR jedem Apply.
    let outcome = run_swe_task(
        &diff,
        &base,
        &mut gw,
        &fs_write_manifest,
        &mut fs_write_tool,
        &build_manifest,
        &build_tool,
        &test_manifest,
        &test_tool,
        false,
        sha256(b"rd:n-swe-1"),
    );
    match outcome {
        SweOutcome::BlockedBeforeApply(failed) => {
            assert!(failed.iter().any(|v| v.gate() == "ToolCapabilityGate"));
        }
        other => panic!("erwartet Gate-Halt, war {other:?}"),
    }

    // Direkt am Gate: ein DiffCandidate ohne jedes BuildRun ist NIE
    // commit-faehig — `build_unverified`, unabhaengig vom Aufrufer.
    let verdict = build_evidence_gate(None);
    assert!(!verdict.allows());
    assert!(verdict.residue().unwrap().id.contains("build_unverified"));
}

/// N-SWE-2: Diff mit rotem Test ⇒ `tests_red`, Hold — Build war gruen,
/// der Test bleibt rot.
#[test]
fn n_swe_2_diff_with_red_test_holds_as_tests_red() {
    let base = base_snapshot();
    let diff = fix_diff_candidate(&base);
    let mut gw = gateway_with_locks();
    let mut fs_write_tool =
        FsWriteTool::with_files(&[("repo/src/lib.rs", ORIGINAL_LIB.as_bytes())]);
    let fs_write_manifest = tool_manifest("fs_write");
    let build_manifest = tool_manifest("build");
    let build_tool = BuildTool::with_fixture(&["cargo", "build"], 0, "Compiling ok");
    let test_manifest = tool_manifest("test");
    let test_tool = TestTool::with_fixture(&["cargo", "test"], 101, "1 failed");

    let outcome = run_swe_task(
        &diff,
        &base,
        &mut gw,
        &fs_write_manifest,
        &mut fs_write_tool,
        &build_manifest,
        &build_tool,
        &test_manifest,
        &test_tool,
        false,
        sha256(b"rd:n-swe-2"),
    );
    match outcome {
        SweOutcome::Hold(failed) => {
            let v = failed
                .iter()
                .find(|v| v.gate() == "TestEvidenceGate")
                .expect("TestEvidenceGate haelt");
            assert!(v.residue().unwrap().id.contains("tests_red"));
        }
        other => panic!("erwartet Hold, war {other:?}"),
    }
}

/// N-SWE-3: fs_write ausserhalb des Arbeitsbereichs ⇒
/// `tool_scope_violation`, reject — VOR jedem Apply.
#[test]
fn n_swe_3_fs_write_outside_workspace_is_scope_violation() {
    let base = base_snapshot();
    let mut diff = fix_diff_candidate(&base);
    diff.hunks[0].path = "ausserhalb/geheim.rs".to_string();
    let mut gw = gateway_with_locks();
    let mut fs_write_tool = FsWriteTool::default();
    let fs_write_manifest = tool_manifest("fs_write");
    let build_manifest = tool_manifest("build");
    let build_tool = BuildTool::with_fixture(&["cargo", "build"], 0, "ok");
    let test_manifest = tool_manifest("test");
    let test_tool = TestTool::with_fixture(&["cargo", "test"], 0, "ok");

    let outcome = run_swe_task(
        &diff,
        &base,
        &mut gw,
        &fs_write_manifest,
        &mut fs_write_tool,
        &build_manifest,
        &build_tool,
        &test_manifest,
        &test_tool,
        false,
        sha256(b"rd:n-swe-3"),
    );
    match outcome {
        SweOutcome::BlockedBeforeApply(failed) => {
            let v = failed
                .iter()
                .find(|v| v.gate() == "ToolScopeGate")
                .expect("ToolScopeGate haelt");
            assert!(v.residue().unwrap().id.contains("tool_scope_violation"));
        }
        other => panic!("erwartet Gate-Halt, war {other:?}"),
    }

    // Direkt am Gate + am ToolGateway: derselbe Pfad haelt.
    assert!(!tool_scope_gate("ausserhalb/geheim.rs", &fs_write_manifest.scope).allows());
}

/// N-SWE-4: git ohne Lock ⇒ `tool_capability_denied`, reject (Dokument
/// 18 §4 woertlich).
#[test]
fn n_swe_4_git_without_lock_is_tool_capability_denied() {
    let verdict = tool_capability_gate("git", false);
    assert!(!verdict.allows());
    assert!(verdict
        .residue()
        .unwrap()
        .id
        .contains("tool_capability_denied"));

    // Strukturell auch am ToolGateway selbst: kein Lock ⇒ kein Commit.
    let mut gw = ToolGateway::new();
    let mut tool = GitTool::default();
    let m = tool_manifest("git");
    let err = gw
        .run_git(
            &m,
            &mut tool,
            &GitOperation::Commit("x".into()),
            Some("operator:sk"),
        )
        .unwrap_err();
    assert!(err.id.contains("tool_egress_without_tool_capability"));
}

/// N-SWE-5: ein DiffCandidate, der einen bestehenden Zeugen kippt ⇒
/// RegressionGate reject — der EINE Waechter gilt fuer Code-Arbeit
/// woertlich wie ueberall sonst.
#[test]
fn n_swe_5_diff_breaking_existing_witness_is_regression_rejected() {
    let base = base_snapshot();
    let diff = fix_diff_candidate(&base);
    let mut gw = gateway_with_locks();
    let mut fs_write_tool =
        FsWriteTool::with_files(&[("repo/src/lib.rs", ORIGINAL_LIB.as_bytes())]);
    let fs_write_manifest = tool_manifest("fs_write");
    let build_manifest = tool_manifest("build");
    let build_tool = BuildTool::with_fixture(&["cargo", "build"], 0, "Compiling ok");
    let test_manifest = tool_manifest("test");
    let test_tool = TestTool::with_fixture(&["cargo", "test"], 0, "test result: ok. 1 passed");

    let outcome = run_swe_task(
        &diff,
        &base,
        &mut gw,
        &fs_write_manifest,
        &mut fs_write_tool,
        &build_manifest,
        &build_tool,
        &test_manifest,
        &test_tool,
        true, // breaks_existing_witness
        sha256(b"rd:n-swe-5"),
    );
    match outcome {
        SweOutcome::Hold(failed) => {
            let v = failed
                .iter()
                .find(|v| v.gate() == "RegressionGate")
                .expect("RegressionGate haelt");
            assert!(v.residue().unwrap().id.contains("regression_detected"));
        }
        other => panic!("erwartet Hold, war {other:?}"),
    }
    assert!(!regression_gate(true).allows());
}

/// N-SWE-6: push ohne ToolEgressGate (kein Remote-Manifest, keine
/// explizite Erlaubnis) ⇒ reject.
#[test]
fn n_swe_6_push_without_tool_egress_gate_is_rejected() {
    let mut gw = ToolGateway::new();
    gw.open_lock("git", "op", "l1");
    let mut tool = GitTool::default();
    let local_manifest = tool_manifest("git"); // egress: None
    let err = gw
        .run_git(
            &local_manifest,
            &mut tool,
            &GitOperation::Push,
            Some("operator:sk"),
        )
        .unwrap_err();
    assert!(err.id.contains("tool_egress_blocked"));

    // Auch am generischen ToolEgressGate: remote ohne explizite
    // Erlaubnis/Budget/Aufzeichnung haelt.
    let v = tool_egress_gate(true, false, true, true);
    assert!(!v.allows());
}

/// N-SWE-7: Modell-Confidence als Ersatz fuer BuildEvidenceGate ⇒
/// reject (PROD-INV-20 woertlich) — es gibt strukturell KEIN
/// Confidence-Feld, das `build_evidence_gate` akzeptiert; nur ein
/// echter `BuildRun` mit `exit_code == 0` genuegt.
#[test]
fn n_swe_7_model_confidence_cannot_substitute_build_evidence() {
    // Kein BuildRun vorhanden — eine (hier simulierte) hohe
    // Modell-Selbsteinschaetzung aendert nichts am Ergebnis, weil die
    // Gate-Signatur gar kein Konfidenz-Argument entgegennimmt.
    let confidence_claim_permille: u16 = 999; // "99.9% sicher" -- irrelevant
    let verdict = build_evidence_gate(None);
    assert!(
        !verdict.allows(),
        "Confidence {confidence_claim_permille} darf niemals ein Gate ersetzen"
    );
    assert!(verdict.residue().unwrap().id.contains("build_unverified"));
}

/// N-SWE-8: direkter Commit eines DiffCandidate am Gate vorbei ⇒
/// `model_attempted_direct_commit`/reject (wiederverwendetes Gate aus
/// cce-inference, dieselbe Garantie fuer Code-Arbeit).
#[test]
fn n_swe_8_direct_commit_attempt_is_rejected() {
    let verdict = no_direct_commit_gate(true);
    assert!(!verdict.allows());
    assert!(verdict
        .residue()
        .unwrap()
        .id
        .contains("model_attempted_direct_commit"));
}

/// Zusatz: ProviderDiffError ist erreichbar (Typ-Sichtbarkeitscheck,
/// verhindert stille API-Regressionen bei Refactorings).
#[test]
fn provider_diff_error_type_is_reachable() {
    fn assert_debug<T: std::fmt::Debug>() {}
    assert_debug::<ProviderDiffError>();
}
