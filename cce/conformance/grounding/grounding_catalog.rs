//! Agent-Grounding-Zeugenkatalog (Dokument 21, P2-Ext) — Ablage
//! `conformance/grounding/`, dauerhaft im einen Waechter. Alle Zeugen
//! hermetisch (kein Netz, kein echter Subprozess), gruen/rot in der
//! normalen CI. R-GND-1..4 + N-GND-1..2 + der DoD-Wertbeweis.

use cce_core::replay::HitlDecision;
use cce_core::residue::ResidueField;
use cce_core::signature::sha256;
use cce_core::value::CanonValue;
use cce_phaseblock::phaseblock::PhaseBlock;
use cce_swe::gates::{
    context_budget_gate, delta_budget_gate, diff_delta_lines, rule_compliance_gate,
};
use cce_swe::grounded::{run_grounded_swe_task, GroundedOutcome};
use cce_swe::grounding::{
    compile_grounding, DecisionSlot, DecisionStatus, GroundingPacket, RuleAtom, RuleSeverity,
};
use cce_swe::kette::{run_swe_task, SweOutcome};
use cce_swe::model::{CodeUnitRole, DiffCandidate, DiffHunk, ProducedBy, RepoSnapshot};
use cce_toolgateway::gateway::{BuildTool, FsWriteTool, TestTool, ToolGateway};
use cce_toolgateway::manifest::{Egress, ToolManifest};

fn blocking_no_unwrap_rule(evidence: Option<&str>) -> RuleAtom {
    RuleAtom {
        rule_id: "R-no-unwrap-src".to_string(),
        scope: "src/".to_string(),
        trigger: ".unwrap()".to_string(),
        prescription: "kein .unwrap() in src/ — fehlerbehandelnd schreiben".to_string(),
        severity: RuleSeverity::Blocking,
        evidence_ref: evidence.map(str::to_string),
        gate_ref: None,
        decay: None,
    }
}

fn packet_with_rule(rule: RuleAtom) -> GroundingPacket {
    compile_grounding(
        "grounding-witness",
        vec![rule],
        vec![],
        vec!["cargo build".to_string(), "cargo test".to_string()],
    )
    .packet
}

/// Ein Diff, der `.unwrap()` in src/lib.rs HINZUFUEGT, aber sonst sauber
/// anwendbar ist (und mit Build/Test-Fixtures gruen durch die P2-Kette
/// liefe).
fn unwrap_diff() -> DiffCandidate {
    DiffCandidate {
        base_snapshot_root: sha256(b"base"),
        hunks: vec![DiffHunk {
            path: "src/lib.rs".to_string(),
            unified_diff: "@@ -1,1 +1,2 @@\n fn f() {}\n+    let y = a.unwrap();\n".to_string(),
        }],
        rationale: "fuegt unwrap hinzu".to_string(),
        produced_by: ProducedBy::Operator {
            operator: "op".to_string(),
        },
    }
}

fn clean_diff() -> DiffCandidate {
    DiffCandidate {
        base_snapshot_root: sha256(b"base"),
        hunks: vec![DiffHunk {
            path: "src/lib.rs".to_string(),
            unified_diff: "@@ -1,1 +1,2 @@\n fn f() {}\n+    let y = a.ok();\n".to_string(),
        }],
        rationale: "sauber".to_string(),
        produced_by: ProducedBy::Operator {
            operator: "op".to_string(),
        },
    }
}

fn tool_manifest(class: &str) -> ToolManifest {
    ToolManifest {
        tool_id: format!("{class}-1"),
        tool_class: class.to_string(),
        scope: vec!["src/".to_string()],
        side_effects: true,
        egress: Egress::None,
        budget_calls: 10,
        replay_strategy: "recorded".to_string(),
        lock_ref: format!("lock:{class}"),
    }
}

fn base_snapshot() -> RepoSnapshot {
    RepoSnapshot::from_files(
        &[("src/lib.rs", "rust", CodeUnitRole::Source, b"fn f() {}\n")],
        "rustc-1.0",
        None,
    )
}

fn gateway_with_locks() -> ToolGateway {
    let mut gw = ToolGateway::new();
    gw.open_lock("fs_write", "op", "l");
    gw.open_lock("build", "op", "l");
    gw.open_lock("test", "op", "l");
    gw
}

// ---------- Referenzen R-GND-1..4 ----------

/// R-GND-1: RuleAtom ohne Beleg wird automatisch zu `advisory`
/// herabgestuft (sichtbar, nicht stillschweigend als blocking gefuehrt).
#[test]
fn r_gnd_1_rule_without_evidence_downgraded_visibly() {
    let compiled = compile_grounding("pkg", vec![blocking_no_unwrap_rule(None)], vec![], vec![]);
    assert_eq!(compiled.packet.rules[0].severity, RuleSeverity::Advisory);
    assert_eq!(compiled.downgrades.len(), 1);
    assert!(compiled.downgrades[0].id.contains("rule_missing_evidence"));
    // Mit Beleg bleibt blocking.
    let kept = compile_grounding(
        "pkg",
        vec![blocking_no_unwrap_rule(Some("repo:src/lib.rs:1"))],
        vec![],
        vec![],
    );
    assert_eq!(kept.packet.rules[0].severity, RuleSeverity::Blocking);
    assert!(kept.downgrades.is_empty());
}

/// R-GND-2: DiffCandidate verletzt eine `blocking`-Regel ⇒
/// RuleComplianceGate reject, unabhaengig vom sonstigen Gate-Ergebnis.
#[test]
fn r_gnd_2_diff_violating_blocking_rule_is_rejected() {
    let packet = packet_with_rule(blocking_no_unwrap_rule(Some("e1")));
    let v = rule_compliance_gate(&unwrap_diff(), &packet);
    assert!(!v.allows());
    assert!(v.residue().unwrap().id.contains("rule_violation"));
    // Ein sauberer Diff besteht.
    assert!(rule_compliance_gate(&clean_diff(), &packet).allows());
}

/// R-GND-3: GroundingPacket-Digest ist deterministisch (zwei
/// Kompilierlaeufe, identisch, ordnungsunabhaengig).
#[test]
fn r_gnd_3_packet_digest_deterministic_order_independent() {
    let a = compile_grounding(
        "pkg",
        vec![
            blocking_no_unwrap_rule(Some("e1")),
            RuleAtom {
                rule_id: "R-2".to_string(),
                scope: "tests/".to_string(),
                trigger: "todo!()".to_string(),
                prescription: "kein todo in tests".to_string(),
                severity: RuleSeverity::Required,
                evidence_ref: Some("e2".to_string()),
                gate_ref: None,
                decay: None,
            },
        ],
        vec![DecisionSlot::open("D1", "?", &["a", "b"])],
        vec!["cargo test".to_string(), "cargo build".to_string()],
    )
    .packet;
    let b = compile_grounding(
        "pkg",
        vec![
            RuleAtom {
                rule_id: "R-2".to_string(),
                scope: "tests/".to_string(),
                trigger: "todo!()".to_string(),
                prescription: "kein todo in tests".to_string(),
                severity: RuleSeverity::Required,
                evidence_ref: Some("e2".to_string()),
                gate_ref: None,
                decay: None,
            },
            blocking_no_unwrap_rule(Some("e1")),
        ],
        vec![DecisionSlot::open("D1", "?", &["a", "b"])],
        vec!["cargo build".to_string(), "cargo test".to_string()],
    )
    .packet;
    assert_eq!(a.packet_digest(), b.packet_digest());
}

/// R-GND-4: eine offene DecisionSlot wird durch einen PhaseBlock
/// aufgeloest, aufgezeichnet mit RD-Ref (S5-A5-Muster).
#[test]
fn r_gnd_4_decision_resolved_by_phaseblock_with_rd_ref() {
    let slot = DecisionSlot::open("D-log", "Welche Log-Bibliothek?", &["tracing", "log"]);
    assert_eq!(slot.status, DecisionStatus::Open);

    let rd = "rd:grounding-decide-1";
    let resolved = slot
        .resolve("tracing", rd, "ev:auftraggeber-direktive")
        .expect("Aufloesung in domain gelingt");
    assert_eq!(resolved.status, DecisionStatus::Resolved);
    assert_eq!(resolved.resolved_as.as_deref(), Some("tracing"));

    // Die Aufloesung ist als HITL-Entscheidung in einem PhaseBlock
    // aufgezeichnet (dieselbe Disziplin wie jede HITL-Entscheidung).
    let hitl = HitlDecision {
        gate: "DecisionResolveGate".to_string(),
        decision: format!("{}=tracing", resolved.decision_id),
        operator: "operator:auftraggeber".to_string(),
        rd_ref: Some(rd.to_string()),
        evidence_ref: resolved.evidence_ref.clone(),
    };
    let block = PhaseBlock::candidate(
        1,
        "grounding:decision-resolve",
        &CanonValue::text(format!("resolved:{}", resolved.decision_id)),
        vec![hitl],
        vec![],
        vec![],
        ResidueField::new(),
        sha256(rd.as_bytes()),
        vec![],
    );
    assert_eq!(block.inputs.len(), 1);
    assert_eq!(block.inputs[0].rd_ref.as_deref(), Some(rd));
    // Eine Aufloesung ausserhalb der domain ist unzulaessig.
    assert!(DecisionSlot::open("D2", "?", &["x"])
        .resolve("y", rd, "e")
        .is_err());
}

// ---------- Negative N-GND-1..2 ----------

/// N-GND-1: Diff ueber Budget ohne Erhoehungs-Bestaetigung ⇒ Hold;
/// mit aufgezeichneter Erhoehung ⇒ allow.
#[test]
fn n_gnd_1_diff_over_delta_budget_holds_without_confirmation() {
    let diff = unwrap_diff();
    let lines = diff_delta_lines(&diff); // 1 hinzugefuegte Zeile (Kontext zaehlt nicht)
                                         // Budget 0 → ueberschritten.
    let v = delta_budget_gate(lines, 0, None);
    assert!(!v.allows());
    assert!(v.residue().unwrap().id.contains("delta_budget_exceeded"));
    // Mit aufgezeichneter Erhoehung geht es durch.
    assert!(delta_budget_gate(lines, 0, Some("operator:sk;ledger:e1")).allows());
    // Innerhalb des Budgets ohnehin allow.
    assert!(delta_budget_gate(lines, 10, None).allows());
}

/// N-GND-2: Packet ueber Kontextbudget ⇒ Hold, kein Kuerzen.
#[test]
fn n_gnd_2_packet_over_context_budget_holds() {
    let packet = packet_with_rule(blocking_no_unwrap_rule(Some("e1")));
    let big = packet.context_size();
    // Grenze knapp unter der tatsaechlichen Groesse.
    let v = context_budget_gate(&packet, big - 1);
    assert!(!v.allows());
    assert!(v.residue().unwrap().id.contains("context_budget_exceeded"));
    // Ausreichende Grenze → allow.
    assert!(context_budget_gate(&packet, big).allows());
}

// ---------- DoD-Wertbeweis ----------

/// Der eigentliche Wertbeweis (Dokument 21 §7): ein Diff, der die
/// bestehende P2-Gate-Familie GRUEN durchlaeuft (Build/Test-Fixtures exit
/// 0 → Candidate), wird vom GroundingPacket dennoch gefangen — die
/// bisherigen Gates allein haetten die Regelverletzung NICHT gefangen.
#[test]
fn value_proof_grounding_catches_what_p2_gates_alone_would_pass() {
    let base = base_snapshot();
    let diff = unwrap_diff();
    let build_ok = BuildTool::with_fixture(&["cargo", "build"], 0, "ok");
    let test_ok = TestTool::with_fixture(&["cargo", "test"], 0, "ok");

    // 1. Die UNVERAENDERTE P2-Kette akzeptiert den Diff (Build/Test gruen):
    let mut gw = gateway_with_locks();
    let mut fsw = FsWriteTool::with_files(&[("src/lib.rs", b"fn f() {}\n")]);
    let outcome_p2 = run_swe_task(
        &diff,
        &base,
        &mut gw,
        &tool_manifest("fs_write"),
        &mut fsw,
        &tool_manifest("build"),
        &build_ok,
        &tool_manifest("test"),
        &test_ok,
        false,
        sha256(b"rd:value-proof"),
    );
    assert!(
        matches!(outcome_p2, SweOutcome::Candidate(_)),
        "P2-Gates allein lassen den unwrap-Diff durch (build+test gruen)"
    );

    // 2. Der GROUNDED Einstieg mit einer blocking-Regel faengt ihn — VOR
    //    jeder Werkzeug-Wirkung:
    let packet = packet_with_rule(blocking_no_unwrap_rule(Some("repo:coding-guideline")));
    let mut gw2 = gateway_with_locks();
    let mut fsw2 = FsWriteTool::with_files(&[("src/lib.rs", b"fn f() {}\n")]);
    let outcome_grounded = run_grounded_swe_task(
        &packet,
        100_000,
        1000,
        None,
        &diff,
        &base,
        &mut gw2,
        &tool_manifest("fs_write"),
        &mut fsw2,
        &tool_manifest("build"),
        &build_ok,
        &tool_manifest("test"),
        &test_ok,
        false,
        sha256(b"rd:value-proof"),
    );
    match outcome_grounded {
        GroundedOutcome::BlockedByGrounding(failed) => {
            assert!(failed.iter().any(|v| v.gate() == "RuleComplianceGate"));
        }
        other => panic!("erwartet BlockedByGrounding (RuleComplianceGate), war {other:?}"),
    }
    // Kein Werkzeug wurde beruehrt (Grounding haelt VOR der Kette).
    assert!(gw2.records.is_empty());
}
