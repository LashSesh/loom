//! TaskProposal (Dokument 19 §3): `(proposal_id, wish_text, task_class,
//! target_scope, estimated_cost_budget, rationale)` — das Ergebnis der
//! Wunschstrecke/Kanzel, BEVOR irgendein Werkzeug läuft. `task_class`
//! ist bewusst ein String + Laufzeit-Whitelist (nicht ein Enum,
//! dieselbe Konvention wie `cce_toolgateway::manifest::ToolManifest
//! ::tool_class`) — nur so kann N-DOG-1 (eine Klasse ausserhalb der
//! Whitelist) als ECHTER Laufzeit-Reject bewiesen werden statt durch
//! Typkonstruktion unmoeglich zu sein.

use cce_swe::model::TaskLedger;

/// Die drei erlaubten Aufgabenklassen fuer den ERSTEN (und bis auf
/// Weiteres einzigen) Dogfooding-Lauf (§2: Task-Klassen-Whitelist,
/// niedrigstes Risiko).
pub const TASK_CLASS_WHITELIST: [&str; 3] = ["doc", "additive_test", "lint_fix"];

/// Ein TaskProposal — traegt NIE einen Ausfuehrungs-Marker; er entsteht
/// vor jeder Werkzeug-Beruehrung.
#[derive(Debug, Clone)]
pub struct TaskProposal {
    pub proposal_id: String,
    pub wish_text: String,
    pub task_class: String,
    pub target_scope: Vec<String>,
    pub estimated_cost_budget: u32,
    pub rationale: String,
}

/// Die Auftraggeber-Entscheidung ueber EINEN konkreten Vorschlag —
/// § 4: "AUFTRAGGEBER-FREIGABE DES VORSCHLAGS (expliziter Stopp — kein
/// Autolauf ab hier)". `Approved` traegt eine aufgezeichnete
/// Bestaetigungsreferenz (dieselbe Disziplin wie
/// `ToolGateway::run_git`s `confirmation_ref`); `Rejected` haelt den
/// Lauf sauber an, VOR jeder Werkzeug-Ausfuehrung (R-DOG-2).
#[derive(Debug, Clone)]
pub enum OperatorDecision {
    Approved { confirmation_ref: String },
    Rejected { reason: String },
}

/// Bindet TaskProposal + TaskLedger (aus P2) + Branch-Name + die
/// Auftraggeber-Freigabe des Vorschlags selbst.
#[derive(Debug, Clone)]
pub struct DogfoodRun {
    pub proposal: TaskProposal,
    pub branch_name: String,
    pub decision: OperatorDecision,
    pub ledger: TaskLedger,
}

impl DogfoodRun {
    pub fn new(proposal: TaskProposal, branch_name: &str, decision: OperatorDecision) -> Self {
        Self {
            proposal,
            branch_name: branch_name.to_string(),
            decision,
            ledger: TaskLedger::default(),
        }
    }

    pub fn is_approved(&self) -> bool {
        matches!(self.decision, OperatorDecision::Approved { .. })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejected_decision_never_reaches_approved() {
        let proposal = TaskProposal {
            proposal_id: "p3-demo".to_string(),
            wish_text: "Beispiel".to_string(),
            task_class: "doc".to_string(),
            target_scope: vec!["cce/crates/cce-swe/README.md".to_string()],
            estimated_cost_budget: 1,
            rationale: "Test".to_string(),
        };
        let run = DogfoodRun::new(
            proposal,
            "dogfood/p3-demo",
            OperatorDecision::Rejected {
                reason: "nicht jetzt".to_string(),
            },
        );
        assert!(!run.is_approved());
    }
}
