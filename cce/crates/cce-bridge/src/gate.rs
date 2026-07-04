//! BridgeGate (S-E5 §3): die Promotionspruefung, sechs benannte Stufen,
//! fail-closed, Verdikt `allow | hold | reject`. Ein NormCandidate wird
//! NIEMALS automatisch zur Norm — nur ein `Allow` hier erlaubt den
//! Norm-Workbody-Bau (`workbody::seal_norm`).

use crate::types::{residue, BridgeNorm, BridgeVerdict, NormCandidate};
use cce_core::replay::HitlDecision;

/// §3(3): der HITL-Pfad ist ein AUFGEZEICHNETER Eingang (wie S5-A5) —
/// dieselbe `HitlDecision`, die schon `RunDescriptor.decisions` traegt,
/// keine neue Interaktionsform.
pub const COUNTEREXAMPLE_CONFIRMATION_GATE: &str = "counterexample-confirmation";

/// §4: Fundament-Schutz — ein Pattern, das eines dieser Stichworte
/// enthaelt, beansprucht per Konstruktion eine Wirkung auf Egress,
/// Capability-Locks, Verdikt-Schreibwege, Score-Semantik oder
/// Gate-Schwellen NACH UNTEN — genau das schliesst §4 aus. String-
/// Scan statt Semantikanalyse ist v1 bewusst konservativ (lieber ein
/// falsches Reject als ein durchgerutschtes Lockerungs-Pattern).
const FORBIDDEN_PATTERN_MARKERS: &[&str] = &[
    "egress",
    "capability_lock entfernen",
    "capability-lock entfernen",
    "verdikt aendern",
    "verdikt ändern",
    "score-schwelle senken",
    "schwelle senken",
    "gate deaktivieren",
    "gate aussetzen",
    "invariante aufheben",
    "invariante lockern",
    "verbot aufheben",
];

fn violates_scope_whitelist(pattern: &str) -> bool {
    let lower = pattern.to_lowercase();
    FORBIDDEN_PATTERN_MARKERS
        .iter()
        .any(|marker| lower.contains(marker))
}

pub struct BridgeGateContext<'a> {
    pub kappa_min: usize,
    /// Beobachtete Domaenen des ProvenanceSet (aus `provenance::resolve_provenance`).
    pub domains: &'a [String],
    pub min_diversity_domains: usize,
    /// HITL-Entscheidungen des promovierenden Laufs (S5-A5-Muster).
    pub hitl_decisions: &'a [HitlDecision],
    /// Bereits aktive Normen — fuer die ConflictGate-Pruefung.
    pub existing_active_norms: &'a [BridgeNorm],
    /// DistillationReplayGate (§3 Stufe 6): eine zweite, unabhaengig
    /// reproduzierte Destillation ueber denselben Registry-Snapshot +
    /// dasselbe RD. `None` = kein Replay verlangt (Erstlauf).
    pub replay_reproduction: Option<&'a NormCandidate>,
}

impl<'a> BridgeGateContext<'a> {
    pub fn new(domains: &'a [String], hitl_decisions: &'a [HitlDecision]) -> Self {
        Self {
            kappa_min: 3,
            domains,
            min_diversity_domains: 2,
            hitl_decisions,
            existing_active_norms: &[],
            replay_reproduction: None,
        }
    }
}

/// PROD-INV-21 (S-E5 §5): "keine Promotion außer durch BridgeGate mit
/// ProvenanceSet" ist hier STRUKTURELL erzwungen, nicht nur konventionell
/// — die Felder sind PRIVAT und die einzigen Konstruktoren
/// (`allow`/`hold`/`reject`) sind privat und nur von `bridge_gate()`
/// erreichbar. Kein Aufrufer (auch nicht `workbody::seal_norm`) kann
/// sich ein `Allow`-Verdikt ohne einen echten Gate-Durchlauf verschaffen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeGateReport {
    verdict: BridgeVerdict,
    /// Name der Stufe, die zuerst nicht bestanden hat (`None` bei `Allow`).
    failed_stage: Option<&'static str>,
    residue: Option<&'static str>,
}

impl BridgeGateReport {
    pub fn verdict(&self) -> BridgeVerdict {
        self.verdict
    }

    pub fn failed_stage(&self) -> Option<&'static str> {
        self.failed_stage
    }

    pub fn residue(&self) -> Option<&'static str> {
        self.residue
    }

    fn allow() -> Self {
        Self {
            verdict: BridgeVerdict::Allow,
            failed_stage: None,
            residue: None,
        }
    }

    fn hold(stage: &'static str, residue: &'static str) -> Self {
        Self {
            verdict: BridgeVerdict::Hold,
            failed_stage: Some(stage),
            residue: Some(residue),
        }
    }

    fn reject(stage: &'static str, residue: &'static str) -> Self {
        Self {
            verdict: BridgeVerdict::Reject,
            failed_stage: Some(stage),
            residue: Some(residue),
        }
    }
}

/// §3: die sechs Stufen, in genau dieser Reihenfolge, fail-closed
/// (erste nicht bestandene Stufe entscheidet das Verdikt).
pub fn bridge_gate(candidate: &NormCandidate, ctx: &BridgeGateContext) -> BridgeGateReport {
    // (1) ProvenanceGate.
    if candidate.provenance_set.len() < ctx.kappa_min {
        return BridgeGateReport::hold("ProvenanceGate", residue::INSUFFICIENT_PROVENANCE);
    }

    // (2) DiversityGate: >=2 Domaenen ODER >=2 Erzeugungslaeufe (Letzteres
    // ist ab kappa_min>=2 durch (1) bereits strukturell erfuellt, s.
    // reports/X3_bericht.md fuer die Begruendung).
    let diverse =
        ctx.domains.len() >= ctx.min_diversity_domains || candidate.provenance_set.len() >= 2;
    if !diverse {
        return BridgeGateReport::hold("DiversityGate", residue::DIVERSITY_UNMET);
    }

    // (3) CounterexampleGate: Gegenbeispiele sind strukturell nie
    // verschwiegen (NormCandidate::n_counter = known_counterexamples.len(),
    // s. distill.rs) — aber ohne aufgezeichnete Operator-Bestaetigung ist
    // eine Promotion trotz n_counter > 0 unzulaessig.
    if candidate.n_counter > 0 {
        let confirmed = ctx
            .hitl_decisions
            .iter()
            .any(|d| d.gate == COUNTEREXAMPLE_CONFIRMATION_GATE && d.decision == "accept");
        if !confirmed {
            return BridgeGateReport::reject(
                "CounterexampleGate",
                residue::COUNTEREXAMPLE_UNRESOLVED,
            );
        }
    }

    // (4) ConflictGate: kein Widerspruch zu einer aktiven Norm desselben
    // Scopes (anderer nexus_class ODER identisches Pattern ist kein
    // Widerspruch).
    let conflict = ctx.existing_active_norms.iter().any(|n| {
        n.scope == candidate.scope
            && n.nexus_class == candidate.nexus_class
            && n.pattern != candidate.pattern
    });
    if conflict {
        return BridgeGateReport::hold("ConflictGate", residue::NORM_CONFLICT);
    }

    // (5) ScopeGate (Fundament-Schutz).
    if violates_scope_whitelist(&candidate.pattern) {
        return BridgeGateReport::reject("ScopeGate", residue::NORM_SCOPE_VIOLATION);
    }

    // (6) DistillationReplayGate: gleicher Snapshot + gleiches RD muss
    // denselben Kandidaten ergeben.
    if let Some(reproduced) = ctx.replay_reproduction {
        if reproduced != candidate {
            return BridgeGateReport::reject(
                "DistillationReplayGate",
                residue::DISTILLATION_REPLAY_MISMATCH,
            );
        }
    }

    BridgeGateReport::allow()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{CounterExample, NexusClass, ProvenanceSet, Scope};

    fn base_candidate() -> NormCandidate {
        NormCandidate {
            pattern: "wiederkehrende Relation-Regel ueber Familien".to_string(),
            nexus_class: NexusClass::StructuralRule,
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
    fn full_candidate_is_allowed() {
        let candidate = base_candidate();
        let domains = vec!["dom:a".to_string(), "dom:b".to_string()];
        let ctx = BridgeGateContext::new(&domains, &[]);
        let report = bridge_gate(&candidate, &ctx);
        assert_eq!(report.verdict(), BridgeVerdict::Allow);
    }

    /// N-NRM-1: kappa < kappa_min ⇒ Hold.
    #[test]
    fn below_kappa_min_holds() {
        let mut candidate = base_candidate();
        candidate.provenance_set = ProvenanceSet::new(vec!["a1".repeat(34)]);
        let domains = vec!["dom:a".to_string()];
        let ctx = BridgeGateContext::new(&domains, &[]);
        let report = bridge_gate(&candidate, &ctx);
        assert_eq!(report.verdict(), BridgeVerdict::Hold);
        assert_eq!(report.failed_stage(), Some("ProvenanceGate"));
        assert_eq!(report.residue(), Some(residue::INSUFFICIENT_PROVENANCE));
    }

    /// N-NRM-4: Gegenbeispiele vorhanden, aber keine HITL-Bestaetigung
    /// aufgezeichnet ⇒ reject.
    #[test]
    fn unconfirmed_counterexamples_are_rejected() {
        let mut candidate = base_candidate();
        candidate.n_counter = 1;
        candidate.known_counterexamples = vec![CounterExample {
            core_root_hex: "ff".repeat(34),
            reason: "Abweichung".to_string(),
        }];
        let domains = vec!["dom:a".to_string(), "dom:b".to_string()];
        let ctx = BridgeGateContext::new(&domains, &[]);
        let report = bridge_gate(&candidate, &ctx);
        assert_eq!(report.verdict(), BridgeVerdict::Reject);
        assert_eq!(report.failed_stage(), Some("CounterexampleGate"));
    }

    #[test]
    fn confirmed_counterexamples_pass() {
        let mut candidate = base_candidate();
        candidate.n_counter = 1;
        candidate.known_counterexamples = vec![CounterExample {
            core_root_hex: "ff".repeat(34),
            reason: "Abweichung".to_string(),
        }];
        let domains = vec!["dom:a".to_string(), "dom:b".to_string()];
        let decisions = vec![HitlDecision {
            gate: COUNTEREXAMPLE_CONFIRMATION_GATE.to_string(),
            decision: "accept".to_string(),
            operator: "op-1".to_string(),
            rd_ref: None,
            evidence_ref: None,
        }];
        let ctx = BridgeGateContext::new(&domains, &decisions);
        let report = bridge_gate(&candidate, &ctx);
        assert_eq!(report.verdict(), BridgeVerdict::Allow);
    }

    /// N-NRM-5: Widerspruch zu aktiver Norm gleichen Scopes ⇒ beide Hold.
    #[test]
    fn conflicting_active_norm_holds() {
        let candidate = base_candidate();
        let domains = vec!["dom:a".to_string(), "dom:b".to_string()];
        let existing = vec![BridgeNorm {
            norm_id: "norm:other".to_string(),
            nexus_class: NexusClass::StructuralRule,
            pattern: "eine ANDERE Regel, gleicher Scope".to_string(),
            provenance_set: ProvenanceSet::new(vec!["b1".repeat(34)]),
            known_counterexamples: vec![],
            scope: Scope::Global,
            status: crate::types::NormStatus::Active,
            promotion_evidence: "e".to_string(),
            supersedes: None,
        }];
        let mut ctx = BridgeGateContext::new(&domains, &[]);
        ctx.existing_active_norms = &existing;
        let report = bridge_gate(&candidate, &ctx);
        assert_eq!(report.verdict(), BridgeVerdict::Hold);
        assert_eq!(report.failed_stage(), Some("ConflictGate"));
    }

    /// N-NRM-3: ein Pattern, das ein Gate/eine Invariante lockern will,
    /// ist per Konstruktion ausgeschlossen ⇒ reject.
    #[test]
    fn scope_violating_pattern_is_rejected() {
        let mut candidate = base_candidate();
        candidate.pattern = "Gate deaktivieren fuer PL2-Laeufe".to_string();
        let domains = vec!["dom:a".to_string(), "dom:b".to_string()];
        let ctx = BridgeGateContext::new(&domains, &[]);
        let report = bridge_gate(&candidate, &ctx);
        assert_eq!(report.verdict(), BridgeVerdict::Reject);
        assert_eq!(report.failed_stage(), Some("ScopeGate"));
    }

    /// N-NRM-8: eine unabhaengige Reproduktion, die vom Original abweicht
    /// (anderes Pattern trotz gleicher Eingabe simuliert) ⇒ rot/reject.
    #[test]
    fn distillation_replay_mismatch_is_rejected() {
        let candidate = base_candidate();
        let mut reproduced = base_candidate();
        reproduced.pattern = "abweichendes Pattern".to_string();
        let domains = vec!["dom:a".to_string(), "dom:b".to_string()];
        let mut ctx = BridgeGateContext::new(&domains, &[]);
        ctx.replay_reproduction = Some(&reproduced);
        let report = bridge_gate(&candidate, &ctx);
        assert_eq!(report.verdict(), BridgeVerdict::Reject);
        assert_eq!(report.failed_stage(), Some("DistillationReplayGate"));
    }

    #[test]
    fn matching_replay_reproduction_passes() {
        let candidate = base_candidate();
        let reproduced = base_candidate();
        let domains = vec!["dom:a".to_string(), "dom:b".to_string()];
        let mut ctx = BridgeGateContext::new(&domains, &[]);
        ctx.replay_reproduction = Some(&reproduced);
        let report = bridge_gate(&candidate, &ctx);
        assert_eq!(report.verdict(), BridgeVerdict::Allow);
    }
}
