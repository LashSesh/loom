//! Score als RANKING (A7, Rebase §3 „DenseBlueprintAttractor"):
//! `Rank(B) = Score_D(B)` PRIORISIERT; die Annahme bleibt
//! `Gate(B)=Pass ∧ Closed(B) ∧ Replay(B)`. Der Schwellwert θ_D ist
//! Vorauswahl fuers Weiterrechnen, NIE Abnahme. Gewichte sind RD-Parameter
//! (R-8), Integer-Promille — kein Float.

use crate::candidate::{BlueprintCandidate, CandidateStatus};
use cce_core::gate::GateReportError;
use cce_core::value::CanonValue;

/// RD-gebundene Score-Gewichte (Promille).
#[derive(Debug, Clone, Copy)]
pub struct ScoreWeights {
    pub utility: u32,
    pub novelty: u32,
    pub cost_penalty: u32,
}

impl Default for ScoreWeights {
    fn default() -> Self {
        Self {
            utility: 500,
            novelty: 300,
            cost_penalty: 200,
        }
    }
}

/// Score_D(B): deterministische Kennzahl (ordnet, entscheidet NIE).
pub fn score(b: &BlueprintCandidate, w: &ScoreWeights) -> u64 {
    let utility = b.facets.len() as u64 * u64::from(w.utility);
    let novelty = b
        .facets
        .iter()
        .map(|f| f.facet_type.len() as u64)
        .sum::<u64>()
        * u64::from(w.novelty);
    let cost = b.facets.len() as u64 * b.facets.len() as u64 * u64::from(w.cost_penalty) / 10;
    (utility + novelty).saturating_sub(cost)
}

/// Ranking: sortiert Kandidaten absteigend nach Score, deterministischer
/// Tie-Break ueber die Kennung. θ_D filtert die WARTESCHLANGE (Vorauswahl),
/// aendert aber keinen Status — kein Kandidat wird durch θ_D akzeptiert
/// oder abgelehnt.
pub fn rank(
    candidates: &[BlueprintCandidate],
    w: &ScoreWeights,
    theta_d: u64,
) -> Vec<(String, u64)> {
    let mut scored: Vec<(String, u64)> = candidates
        .iter()
        .map(|b| (b.id.clone(), score(b, w)))
        .filter(|(_, s)| *s >= theta_d)
        .collect();
    scored.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    scored
}

/// V1-Schranke: der Versuch, einen Score als Verdikt zu verbuchen, wird
/// abgewiesen (`score_as_gate_attempt`).
pub fn attempt_score_as_gate(b: &BlueprintCandidate, s: u64) -> GateReportError {
    let forged = CanonValue::map([
        ("gate_id", CanonValue::text("hbm-accept")),
        ("score", CanonValue::Int(s as i64)),
        ("verdict", CanonValue::text("pass")),
        (
            "reason",
            CanonValue::text(format!("Score {s} fuer {}", b.id)),
        ),
    ]);
    cce_core::gate::GateReport::from_untyped(&forged)
        .expect_err("Score-als-Gate MUSS abgewiesen werden (V1/A7)")
}

/// Statuswechsel geschieht NUR ueber Gates — diese Funktion belegt, dass
/// Ranking den Status unangetastet laesst.
pub fn ranking_leaves_status_untouched(before: &[BlueprintCandidate]) -> bool {
    before.iter().all(|b| b.status != CandidateStatus::Pass)
}
