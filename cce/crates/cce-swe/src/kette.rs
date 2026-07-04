//! Die Kern-Kette (Dokument 18 §4): `DiffCandidate -> apply(dry-run) ->
//! BuildRun -> TestRun -> BuildEvidenceGate ∧ TestEvidenceGate ∧
//! RegressionGate -> PhaseBlock | ResidueReport`. Analog zu
//! `cce_inference::gateway::run_inference`: die Vor-Gate-Kette
//! (ToolCapability/ToolScope je Werkzeugklasse) laeuft VOR jeder
//! Werkzeug-Wirkung; apply/build/test geschehen erst danach.
//!
//! `git commit`/`push` sind bewusst NICHT Teil dieser Funktion — sie
//! sind eine separate, spaetere Aktion des Aufrufers ueber
//! `ToolGateway::run_git` (das die HumanConfirmationGate-Aufzeichnung
//! bereits selbst erzwingt, s. `cce-toolgateway`).

use crate::gates::{
    build_evidence_gate, regression_gate, test_evidence_gate, tool_capability_gate,
    tool_scope_gate, InfVerdict,
};
use crate::model::{
    apply_unified_diff, BuildRun, CodeUnitRole, DiffCandidate, RepoSnapshot, TestRun,
};
use crate::residues::swe_residue;
use cce_core::gate::GateReport;
use cce_core::replay::HitlDecision;
use cce_core::residue::{Residue, ResidueField};
use cce_core::signature::{sha256, Digest};
use cce_core::value::CanonValue;
use cce_phaseblock::phaseblock::PhaseBlock;
use cce_toolgateway::gateway::{BuildTool, FsWriteTool, TestTool, ToolGateway};
use cce_toolgateway::manifest::ToolManifest;

/// Ergebnis eines Kern-Kette-Laufs.
#[derive(Debug)]
pub enum SweOutcome {
    /// Kandidat + neuer Snapshot — der EINZIGE Erfolgspfad.
    Candidate(Box<(PhaseBlock, RepoSnapshot)>),
    /// Apply gelang, aber BuildEvidenceGate/TestEvidenceGate/
    /// RegressionGate hielten NACH dem Lauf.
    Hold(Vec<InfVerdict>),
    /// Gate-Halt VOR jeder Werkzeug-Wirkung (kein Apply versucht).
    BlockedBeforeApply(Vec<InfVerdict>),
    /// `apply()` selbst schlug fehl (Diff passt nicht auf die
    /// Arbeitskopie) oder das Schreiben wurde vom ToolGateway abgelehnt.
    ApplyFailed(Box<Residue>),
}

fn to_gate_report(v: &InfVerdict) -> GateReport {
    match v {
        InfVerdict::Allow { gate, reason } => GateReport::pass(gate, reason),
        InfVerdict::Hold { gate, residue }
        | InfVerdict::Reject { gate, residue }
        | InfVerdict::Quarantine { gate, residue } => GateReport::hold(gate, &residue.content),
    }
}

/// Der Kern-Kette-Lauf. `breaks_existing_witness` ist das Ergebnis der
/// RegressionGate-Pruefung durch den Aufrufer (S13-A7/Selbstschutz-Regel:
/// ob dieser Diff einen ausserhalb dieser Funktion bereits gruenen
/// Zeugen kippen wuerde).
#[allow(clippy::too_many_arguments)]
pub fn run_swe_task(
    diff: &DiffCandidate,
    base: &RepoSnapshot,
    gw: &mut ToolGateway,
    fs_write_manifest: &ToolManifest,
    fs_write_tool: &mut FsWriteTool,
    build_manifest: &ToolManifest,
    build_tool: &BuildTool,
    test_manifest: &ToolManifest,
    test_tool: &TestTool,
    breaks_existing_witness: bool,
    rd_ref: Digest,
) -> SweOutcome {
    // 1. Vor-Wirkungs-Gate-Kette: ToolCapabilityGate je benoetigter
    //    Klasse + ToolScopeGate je Hunk — VOR jedem Apply/Build/Test.
    let mut pre_verdicts: Vec<InfVerdict> = vec![
        tool_capability_gate("fs_write", gw.is_locked("fs_write")),
        tool_capability_gate("build", gw.is_locked("build")),
        tool_capability_gate("test", gw.is_locked("test")),
    ];
    for hunk in &diff.hunks {
        pre_verdicts.push(tool_scope_gate(&hunk.path, &fs_write_manifest.scope));
    }
    let pre_failed: Vec<InfVerdict> = pre_verdicts
        .iter()
        .filter(|v| !v.allows())
        .cloned()
        .collect();
    if !pre_failed.is_empty() {
        return SweOutcome::BlockedBeforeApply(pre_failed);
    }

    // 2. apply(dry-run in Arbeitskopie): das Original kommt aus der
    //    bereits gespiegelten Arbeitskopie (`fs_write_tool.files`); das
    //    Schreiben des Patch-Ergebnisses laeuft ausschliesslich ueber
    //    das ToolGateway (kein Pfad daran vorbei).
    let mut new_snapshot = base.clone();
    for hunk in &diff.hunks {
        let original = fs_write_tool
            .files
            .get(&hunk.path)
            .cloned()
            .unwrap_or_default();
        let original_text = String::from_utf8_lossy(&original).into_owned();
        let patched = match apply_unified_diff(&original_text, &hunk.unified_diff) {
            Ok(p) => p,
            Err(e) => {
                return SweOutcome::ApplyFailed(Box::new(swe_residue(
                    "diff_apply_conflict",
                    &format!("{}: {e}", hunk.path),
                )))
            }
        };
        if let Err(residue) = gw.run_fs_write(
            fs_write_manifest,
            fs_write_tool,
            &hunk.path,
            patched.as_bytes(),
        ) {
            return SweOutcome::ApplyFailed(residue);
        }
        let (language, role) = base
            .get(&hunk.path)
            .map(|u| (u.language.clone(), u.role))
            .unwrap_or_else(|| ("text".to_string(), CodeUnitRole::Source));
        new_snapshot =
            new_snapshot.with_unit_content(&hunk.path, &language, role, patched.as_bytes());
    }

    // 3. BuildRun + TestRun — ERST jetzt, nach dem vollstaendigen Apply.
    let build_run = gw
        .run_build(build_manifest, build_tool)
        .ok()
        .map(|o| BuildRun {
            snapshot_root_after_apply: new_snapshot.snapshot_root(),
            tool_id: build_manifest.tool_id.clone(),
            command: build_tool.argv.join(" "),
            exit_code: o.exit_code,
            log_digest: sha256(o.log.as_bytes()),
            duration_ms: 0,
            artifacts: vec![],
        });
    let test_run = gw.run_test(test_manifest, test_tool).ok().map(|o| TestRun {
        snapshot_root_after_apply: new_snapshot.snapshot_root(),
        tool_id: test_manifest.tool_id.clone(),
        command: test_tool.argv.join(" "),
        exit_code: o.exit_code,
        log_digest: sha256(o.log.as_bytes()),
        duration_ms: 0,
        artifacts: vec![],
    });

    // 4. Die drei Kern-Gates (§4).
    let verdicts = [
        build_evidence_gate(build_run.as_ref()),
        test_evidence_gate(test_run.as_ref()),
        regression_gate(breaks_existing_witness),
    ];
    let failed: Vec<InfVerdict> = verdicts.iter().filter(|v| !v.allows()).cloned().collect();
    if !failed.is_empty() {
        return SweOutcome::Hold(failed);
    }

    // 5. PhaseBlock — der einzige Erfolgspfad. build_run/test_run sind
    //    an dieser Stelle Some (sonst haetten die Gates oben gehalten).
    let build_run = build_run.expect("BuildEvidenceGate allow ⇒ BuildRun vorhanden");
    let test_run = test_run.expect("TestEvidenceGate allow ⇒ TestRun vorhanden");

    let mut all_verdicts = pre_verdicts;
    all_verdicts.extend(verdicts.iter().cloned());
    let gate_reports: Vec<GateReport> = all_verdicts.iter().map(to_gate_report).collect();

    let payload = CanonValue::map([
        (
            "base_snapshot_root",
            CanonValue::text(diff.base_snapshot_root.to_hex()),
        ),
        (
            "new_snapshot_root",
            CanonValue::text(new_snapshot.snapshot_root().to_hex()),
        ),
        ("rationale", CanonValue::text(diff.rationale.clone())),
    ]);
    let evidence_refs = vec![build_run.log_digest, test_run.log_digest];
    let block = PhaseBlock::candidate(
        1,
        "swe:diff-apply",
        &payload,
        Vec::<HitlDecision>::new(),
        gate_reports,
        evidence_refs,
        ResidueField::new(),
        rd_ref,
        vec![],
    );
    SweOutcome::Candidate(Box::new((block, new_snapshot)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{DiffHunk, ProducedBy};
    use crate::workbody::seal_repo_workbody;
    use cce_phaseblock::accept::{accept_block, AcceptContext};
    use cce_toolgateway::manifest::Egress;

    const ORIGINAL_LIB: &str = "fn add(a: i32, b: i32) -> i32 {\n    a - b\n}\n";
    const FIXED_LIB: &str = "fn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n";
    const DIFF: &str =
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

    /// R-SWE-2-Referenzlauf (Betriebsdisziplin: der Zeugenkatalog selbst
    /// lebt in `conformance/swe/`; dieser Test ist ein interner
    /// Integrations-Smoke-Test der Kern-Kette + Container-Siegelung).
    #[test]
    fn full_pipeline_fix_failing_test_seals_valid_repo_workbody() {
        let base = RepoSnapshot::from_files(
            &[(
                "repo/src/lib.rs",
                "rust",
                CodeUnitRole::Source,
                ORIGINAL_LIB.as_bytes(),
            )],
            "rustc-1.0",
            Some("deadbeef"),
        );
        let diff = DiffCandidate {
            base_snapshot_root: base.snapshot_root(),
            hunks: vec![DiffHunk {
                path: "repo/src/lib.rs".to_string(),
                unified_diff: DIFF.to_string(),
            }],
            rationale: "behebt den Vorzeichenfehler in add()".to_string(),
            produced_by: ProducedBy::Operator {
                operator: "operator:sk".to_string(),
            },
        };

        let mut gw = ToolGateway::new();
        gw.open_lock("fs_write", "op", "l1");
        gw.open_lock("build", "op", "l1");
        gw.open_lock("test", "op", "l1");
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

        let accept = accept_block(&mut block, &AcceptContext::all_true());
        assert_eq!(accept, cce_phaseblock::accept::AcceptOutcome::Accepted);

        let mut ledger = crate::model::TaskLedger::new(rd_ref);
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

    /// R-SWE-3: derselbe Auftrag, zweiter Lauf — Ergebnisklasse
    /// identisch (`base_snapshot_root` + gleiche RD ⇒ gleiche
    /// DiffCandidate-/BuildRun-/TestRun-Klasse, §5). Zwei vollstaendig
    /// unabhaengige `ToolGateway`/`FsWriteTool`-Instanzen, derselbe
    /// Auftrag — keine geteilte Zustandsvariable zwischen den Laeufen.
    #[test]
    fn same_task_twice_yields_identical_result_class() {
        fn run_once() -> (Digest, Digest, i32, i32) {
            let base = RepoSnapshot::from_files(
                &[(
                    "repo/src/lib.rs",
                    "rust",
                    CodeUnitRole::Source,
                    ORIGINAL_LIB.as_bytes(),
                )],
                "rustc-1.0",
                Some("deadbeef"),
            );
            let diff = DiffCandidate {
                base_snapshot_root: base.snapshot_root(),
                hunks: vec![DiffHunk {
                    path: "repo/src/lib.rs".to_string(),
                    unified_diff: DIFF.to_string(),
                }],
                rationale: "behebt den Vorzeichenfehler in add()".to_string(),
                produced_by: ProducedBy::Operator {
                    operator: "operator:sk".to_string(),
                },
            };
            let mut gw = ToolGateway::new();
            gw.open_lock("fs_write", "op", "l1");
            gw.open_lock("build", "op", "l1");
            gw.open_lock("test", "op", "l1");
            let mut fs_write_tool =
                FsWriteTool::with_files(&[("repo/src/lib.rs", ORIGINAL_LIB.as_bytes())]);
            let fs_write_manifest = tool_manifest("fs_write");
            let build_manifest = tool_manifest("build");
            let build_tool = BuildTool::with_fixture(&["cargo", "build"], 0, "Compiling ok");
            let test_manifest = tool_manifest("test");
            let test_tool =
                TestTool::with_fixture(&["cargo", "test"], 0, "test result: ok. 1 passed");
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
                    (block.payload_digest, snapshot.snapshot_root(), 0, 0)
                }
                other => panic!("erwartet Candidate, war {other:?}"),
            }
        }

        let (payload_1, root_1, _, _) = run_once();
        let (payload_2, root_2, _, _) = run_once();
        assert_eq!(
            payload_1, payload_2,
            "gleicher Auftrag ⇒ gleiche PhaseBlock-Payload-Klasse (Replay-Basis, §5)"
        );
        assert_eq!(
            root_1, root_2,
            "gleicher Auftrag ⇒ gleicher Snapshot-Root nach Apply"
        );
    }
}
