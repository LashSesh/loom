//! Die sechs Werkzeug-Gates (Dokument 18 §4): boolesch, fail-closed,
//! begruendet — dieselbe `InfVerdict`-Form wie die 14 Inference-Gates
//! (C.8). ToolEgressGate/HumanConfirmationGate/NoDirectCommitGate sind
//! bereits in `cce_inference::gates` gebaut (P1-Aera) und werden hier
//! unveraendert wiederverwendet, statt sie zu duplizieren; ToolCapability/
//! ToolScope/BuildEvidence/TestEvidence/Regression sind neu (P2).

use crate::grounding::{GroundingPacket, RuleSeverity};
use crate::model::{BuildRun, DiffCandidate, TestRun};
use crate::residues::swe_residue;

pub use cce_inference::gates::{
    human_confirmation_gate, no_direct_commit_gate, tool_egress_gate, InfVerdict,
};

/// Die sechs Werkzeug-Gate-Namen (§4) — die letzte Bullet fasst zwei
/// Gates (ToolEgress + HumanConfirmation) zusammen, macht sieben
/// tatsaechliche Funktionen aus sechs benannten Gate-Konzepten.
pub const ALL_SWE_GATES: [&str; 7] = [
    "ToolCapabilityGate",
    "ToolScopeGate",
    "BuildEvidenceGate",
    "TestEvidenceGate",
    "RegressionGate",
    "ToolEgressGate",
    "HumanConfirmationGate",
];

/// 1/6 ToolCapabilityGate: angeforderte Klasse hat offenen Lock im
/// Scope — sonst `tool_capability_denied` (Dokument 18 §4 woertlich).
/// `locked` kommt aus `ToolGateway::is_locked(tool_class)` — das
/// Gateway haelt den eigentlichen `CapabilityLock` privat je Klasse
/// (keine Sammel-Freigabe), dieses Gate komponiert nur das Ergebnis in
/// die GateReport-Kette fuer den PhaseBlock.
pub fn tool_capability_gate(tool_class: &str, locked: bool) -> InfVerdict {
    if locked {
        InfVerdict::Allow {
            gate: "ToolCapabilityGate".into(),
            reason: format!("Lock fuer Tool-Klasse {tool_class} offen"),
        }
    } else {
        InfVerdict::Hold {
            gate: "ToolCapabilityGate".into(),
            residue: Box::new(swe_residue(
                "tool_capability_denied",
                &format!("Tool-Klasse {tool_class} ohne offenen ToolCapabilityLock"),
            )),
        }
    }
}

/// 2/6 ToolScopeGate: fs_write/git wirken nur im deklarierten
/// Arbeitsbereich — Pfad ausserhalb ⇒ `tool_scope_violation`, reject.
pub fn tool_scope_gate(path: &str, scope: &[String]) -> InfVerdict {
    if scope.iter().any(|s| path.starts_with(s.as_str())) {
        InfVerdict::Allow {
            gate: "ToolScopeGate".into(),
            reason: format!("{path} im deklarierten Arbeitsbereich"),
        }
    } else {
        InfVerdict::Reject {
            gate: "ToolScopeGate".into(),
            residue: Box::new(swe_residue(
                "tool_scope_violation",
                &format!("{path} ausserhalb des deklarierten Arbeitsbereichs"),
            )),
        }
    }
}

/// 3/6 BuildEvidenceGate: ein DiffCandidate wird nur commit-faehig, wenn
/// ein BuildRun mit `exit_code == 0` als Evidence vorliegt — sonst
/// `build_unverified`, Hold (kein Modell-Selbstauskunft-Ersatz,
/// PROD-INV-20/N-SWE-7).
pub fn build_evidence_gate(build: Option<&BuildRun>) -> InfVerdict {
    match build {
        Some(b) if b.exit_code == 0 => InfVerdict::Allow {
            gate: "BuildEvidenceGate".into(),
            reason: format!("BuildRun {} exit 0", b.tool_id),
        },
        Some(b) => InfVerdict::Hold {
            gate: "BuildEvidenceGate".into(),
            residue: Box::new(swe_residue(
                "build_unverified",
                &format!(
                    "BuildRun {} exit {} — kein gruener Beweis",
                    b.tool_id, b.exit_code
                ),
            )),
        },
        None => InfVerdict::Hold {
            gate: "BuildEvidenceGate".into(),
            residue: Box::new(swe_residue(
                "build_unverified",
                "kein BuildRun als Evidence vorhanden",
            )),
        },
    }
}

/// 4/6 TestEvidenceGate: analog — kein TestRun ⇒ `tests_unverified`;
/// TestRun vorhanden aber rot ⇒ `tests_red`. Beides Hold.
pub fn test_evidence_gate(test: Option<&TestRun>) -> InfVerdict {
    match test {
        Some(t) if t.exit_code == 0 => InfVerdict::Allow {
            gate: "TestEvidenceGate".into(),
            reason: format!("TestRun {} exit 0", t.tool_id),
        },
        Some(t) => InfVerdict::Hold {
            gate: "TestEvidenceGate".into(),
            residue: Box::new(swe_residue(
                "tests_red",
                &format!("TestRun {} exit {} — Test(e) rot", t.tool_id, t.exit_code),
            )),
        },
        None => InfVerdict::Hold {
            gate: "TestEvidenceGate".into(),
            residue: Box::new(swe_residue(
                "tests_unverified",
                "kein TestRun als Evidence vorhanden",
            )),
        },
    }
}

/// 5/6 RegressionGate: der EINE Waechter — kein DiffCandidate wird
/// PhaseBlock, der einen bestehenden Zeugen kippt (Selbstschutz-Regel
/// gilt fuer Code-Arbeit woertlich).
pub fn regression_gate(breaks_existing_witness: bool) -> InfVerdict {
    if breaks_existing_witness {
        InfVerdict::Reject {
            gate: "RegressionGate".into(),
            residue: Box::new(swe_residue(
                "regression_detected",
                "DiffCandidate kippt einen bestehenden Zeugen — Reject (Selbstschutz-Regel)",
            )),
        }
    } else {
        InfVerdict::Allow {
            gate: "RegressionGate".into(),
            reason: "kein bestehender Zeuge gekippt".into(),
        }
    }
}

// ---------- Agent-Grounding-Gates (Dokument 21 §3/§4, additiv) ----------

/// Zaehlt die geaenderten Zeilen eines DiffCandidate (`+`/`-`-Zeilen ueber
/// alle Hunks) — die Aenderungsgroesse fuer das DeltaBudgetGate.
pub fn diff_delta_lines(diff: &DiffCandidate) -> usize {
    diff.hunks
        .iter()
        .flat_map(|h| h.unified_diff.lines())
        .filter(|l| {
            (l.starts_with('+') || l.starts_with('-'))
                && !l.starts_with("+++")
                && !l.starts_with("---")
        })
        .count()
}

/// DeltaBudgetGate (§3): ein DiffCandidate, dessen Zeilenumfang das im
/// Auftrag deklarierte Budget ueberschreitet, HAELT — ausser bei explizit
/// aufgezeichneter Erhoehung des Budgets durch den Auftraggeber
/// (materielle Aktion, HumanConfirmationGate). Verhindert unbemerkt
/// riesige Aenderungen.
pub fn delta_budget_gate(
    delta_lines: usize,
    budget: usize,
    increase_confirmation: Option<&str>,
) -> InfVerdict {
    if delta_lines <= budget {
        return InfVerdict::Allow {
            gate: "DeltaBudgetGate".into(),
            reason: format!("{delta_lines} ≤ Budget {budget}"),
        };
    }
    match increase_confirmation {
        Some(r) if !r.is_empty() => InfVerdict::Allow {
            gate: "DeltaBudgetGate".into(),
            reason: format!("{delta_lines} > Budget {budget}, aber aufgezeichnete Erhoehung: {r}"),
        },
        _ => InfVerdict::Hold {
            gate: "DeltaBudgetGate".into(),
            residue: Box::new(swe_residue(
                "delta_budget_exceeded",
                &format!(
                    "Diff aendert {delta_lines} Zeilen > Budget {budget} ohne aufgezeichnete \
                     Erhoehung — Hold"
                ),
            )),
        },
    }
}

/// ContextBudgetGate (§3): ein GroundingPacket, das die deklarierte
/// Zeichen-Obergrenze ueberschreitet, HAELT mit `context_budget_exceeded`
/// — kein stilles Kuerzen sensibler Regeln.
pub fn context_budget_gate(packet: &GroundingPacket, char_limit: usize) -> InfVerdict {
    let size = packet.context_size();
    if size <= char_limit {
        InfVerdict::Allow {
            gate: "ContextBudgetGate".into(),
            reason: format!("{size} ≤ Kontextbudget {char_limit}"),
        }
    } else {
        InfVerdict::Hold {
            gate: "ContextBudgetGate".into(),
            residue: Box::new(swe_residue(
                "context_budget_exceeded",
                &format!(
                    "GroundingPacket {size} Zeichen > Budget {char_limit} — Hold, kein stilles \
                     Kuerzen"
                ),
            )),
        }
    }
}

/// RuleComplianceGate (§4): jeder DiffCandidate wird gegen die
/// `blocking`-RuleAtoms des GroundingPacket geprueft — reject bei
/// Verstoss, UNABHAENGIG vom Modell-Output oder den Werkzeug-Gates. Eine
/// Regel loest aus, wenn ihr `trigger` in einer HINZUGEFUEGTEN (`+`)
/// Zeile eines Hunks auftaucht, dessen Pfad unter dem `scope` der Regel
/// liegt.
pub fn rule_compliance_gate(diff: &DiffCandidate, packet: &GroundingPacket) -> InfVerdict {
    for rule in &packet.rules {
        if rule.severity != RuleSeverity::Blocking {
            continue;
        }
        for hunk in &diff.hunks {
            if !hunk.path.starts_with(rule.scope.as_str()) {
                continue;
            }
            for line in hunk.unified_diff.lines() {
                if let Some(added) = line.strip_prefix('+') {
                    if !line.starts_with("+++") && added.contains(rule.trigger.as_str()) {
                        return InfVerdict::Reject {
                            gate: "RuleComplianceGate".into(),
                            residue: Box::new(swe_residue(
                                "rule_violation",
                                &format!(
                                    "blocking-Regel '{}' verletzt (Trigger '{}' in {}): {}",
                                    rule.rule_id, rule.trigger, hunk.path, rule.prescription
                                ),
                            )),
                        };
                    }
                }
            }
        }
    }
    InfVerdict::Allow {
        gate: "RuleComplianceGate".into(),
        reason: "kein blocking-Regel-Trigger in hinzugefuegten Zeilen".into(),
    }
}
