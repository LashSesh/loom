//! Checkpointing (S5.2): content-adressierte KANDIDATEN-Zustaende an den
//! Stufengrenzen — gleiche Lage ⇒ gleicher Checkpoint-Digest; Wiederaufnahme
//! ist deterministische Fortsetzung, kein Neustart.

use cce_core::canonical::Canonicalize;
use cce_core::replay::HitlDecision;
use cce_core::signature::Digest;
use cce_core::value::CanonValue;

#[derive(Debug, Clone)]
pub struct Checkpoint {
    pub stage_index: usize,
    pub state_class: Digest,
    pub decisions_so_far: Vec<HitlDecision>,
}

impl Checkpoint {
    pub fn capture(stage_index: usize, state: &CanonValue, decisions: &[HitlDecision]) -> Self {
        let v = CanonValue::map([
            ("stage", CanonValue::Int(stage_index as i64)),
            ("state", state.clone()),
            (
                "decisions",
                CanonValue::List(decisions.iter().map(|d| d.canonical_value()).collect()),
            ),
        ]);
        Checkpoint {
            stage_index,
            state_class: v.canonical_class().0,
            decisions_so_far: decisions.to_vec(),
        }
    }
}
