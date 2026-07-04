//! Die neuen P3-Gates (Dokument 19 §5), zusaetzlich zu ALLEN
//! P1/P2-Gates, die unveraendert gelten (importiert, nicht dupliziert):
//! `TaskProposalGate` · `ProtectedPathGate` · `BranchIsolationGate` ·
//! `MergeExclusionGate`. Dieselbe `InfVerdict`-Form wie ueberall sonst.

use crate::fence::first_forbidden;
use crate::proposal::{TaskProposal, TASK_CLASS_WHITELIST};
use crate::residues::dogfood_residue;
pub use cce_swe::gates::InfVerdict;

/// TaskProposalGate: Task-Klasse ∈ Whitelist ∧ target_scope ⊆ erlaubt
/// ∧ Budget gesetzt (§4). Komponiert `ProtectedPathGate` NICHT selbst
/// noch einmal (das ist ein eigenes Gate, s. unten) — prueft hier nur
/// Klasse + Budget + dass ueberhaupt ein Scope deklariert ist.
pub fn task_proposal_gate(proposal: &TaskProposal) -> InfVerdict {
    if !TASK_CLASS_WHITELIST.contains(&proposal.task_class.as_str()) {
        return InfVerdict::Reject {
            gate: "TaskProposalGate".into(),
            residue: Box::new(dogfood_residue(
                "task_class_not_whitelisted",
                &format!(
                    "Task-Klasse '{}' nicht in der Whitelist {:?}",
                    proposal.task_class, TASK_CLASS_WHITELIST
                ),
            )),
        };
    }
    if proposal.target_scope.is_empty() {
        return InfVerdict::Reject {
            gate: "TaskProposalGate".into(),
            residue: Box::new(dogfood_residue(
                "task_scope_empty",
                "target_scope ist leer — keine implizite Vollspiegelung",
            )),
        };
    }
    if proposal.estimated_cost_budget == 0 {
        return InfVerdict::Reject {
            gate: "TaskProposalGate".into(),
            residue: Box::new(dogfood_residue(
                "task_budget_missing",
                "estimated_cost_budget ist 0 — kein Budget deklariert",
            )),
        };
    }
    InfVerdict::Allow {
        gate: "TaskProposalGate".into(),
        reason: format!(
            "Klasse '{}' gewhitelisted, Scope+Budget deklariert",
            proposal.task_class
        ),
    }
}

/// ProtectedPathGate: jeder Pfad im Scope gegen die Schutzzone (§2)
/// geprüft — der erste Treffer außerhalb ⇒ `protected_path_violation`,
/// reject VOR jedem Apply.
pub fn protected_path_gate(scope: &[String]) -> InfVerdict {
    match first_forbidden(scope) {
        None => InfVerdict::Allow {
            gate: "ProtectedPathGate".into(),
            reason: format!("{} Pfad(e) ausserhalb der Schutzzone", scope.len()),
        },
        Some(bad) => InfVerdict::Reject {
            gate: "ProtectedPathGate".into(),
            residue: Box::new(dogfood_residue(
                "protected_path_violation",
                &format!("'{bad}' liegt in der Schutzzone (Dokument 19 §2)"),
            )),
        },
    }
}

/// BranchIsolationGate: jede Schreib-/Commit-Operation NUR auf
/// `dogfood/p3-*` — ein Versuch auf `main` (oder irgendeinem anderen
/// Branch) ⇒ harter Reject, keine Ausnahme.
pub fn branch_isolation_gate(branch_name: &str) -> InfVerdict {
    if branch_name != "main" && branch_name.starts_with("dogfood/p3-") {
        InfVerdict::Allow {
            gate: "BranchIsolationGate".into(),
            reason: format!("Branch '{branch_name}' ist ein isolierter Dogfood-Branch"),
        }
    } else {
        InfVerdict::Reject {
            gate: "BranchIsolationGate".into(),
            residue: Box::new(dogfood_residue(
                "branch_isolation_violation",
                &format!("Branch '{branch_name}' ist kein isolierter dogfood/p3-*-Branch"),
            )),
        }
    }
}

/// MergeExclusionGate: strukturell — kein CCE-Pfad enthaelt einen
/// Merge-in-main-Aufruf; diese Aktion existiert im System schlicht
/// nicht. `attempts_merge_to_main` ist IMMER `false` in jedem echten
/// Aufrufpfad dieses Crates (es gibt keine Funktion, die einen Merge
/// ausfuehrt) — dieses Gate faengt nur den DEKLARATIVEN Versuch, exakt
/// wie `no_direct_commit_gate`/`no_gate_override_gate` in
/// `cce_inference::gates`.
pub fn merge_exclusion_gate(attempts_merge_to_main: bool) -> InfVerdict {
    if attempts_merge_to_main {
        InfVerdict::Reject {
            gate: "MergeExclusionGate".into(),
            residue: Box::new(dogfood_residue(
                "merge_to_main_attempted",
                "Merge-in-main-Versuch — diese Aktion existiert im System nicht (Reject)",
            )),
        }
    } else {
        InfVerdict::Allow {
            gate: "MergeExclusionGate".into(),
            reason: "kein Merge-Pfad existiert; kein Versuch".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn proposal() -> TaskProposal {
        TaskProposal {
            proposal_id: "p3-demo".to_string(),
            wish_text: "Beispiel".to_string(),
            task_class: "doc".to_string(),
            target_scope: vec!["cce/crates/cce-swe/README.md".to_string()],
            estimated_cost_budget: 1,
            rationale: "Test".to_string(),
        }
    }

    #[test]
    fn task_proposal_gate_allows_whitelisted_class() {
        assert!(task_proposal_gate(&proposal()).allows());
    }

    #[test]
    fn task_proposal_gate_rejects_unlisted_class() {
        let mut p = proposal();
        p.task_class = "refactor".to_string();
        let v = task_proposal_gate(&p);
        assert!(!v.allows());
        assert!(v
            .residue()
            .unwrap()
            .id
            .contains("task_class_not_whitelisted"));
    }

    #[test]
    fn protected_path_gate_rejects_kern_crate() {
        let v = protected_path_gate(&["cce/crates/cce-core/src/gate.rs".to_string()]);
        assert!(!v.allows());
        assert!(v.residue().unwrap().id.contains("protected_path_violation"));
    }

    #[test]
    fn branch_isolation_gate_rejects_main() {
        let v = branch_isolation_gate("main");
        assert!(!v.allows());
        assert!(v
            .residue()
            .unwrap()
            .id
            .contains("branch_isolation_violation"));
    }

    #[test]
    fn branch_isolation_gate_allows_dogfood_branch() {
        assert!(branch_isolation_gate("dogfood/p3-demo").allows());
    }

    #[test]
    fn merge_exclusion_gate_rejects_attempt() {
        assert!(!merge_exclusion_gate(true).allows());
        assert!(merge_exclusion_gate(false).allows());
    }
}
