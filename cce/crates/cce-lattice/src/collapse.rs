//! Collapse-Zertifikat (F-14, CL §7.4): `(Γ, H, Vol, Gate, Ev, ReplayHash, mode)`
//! — Beleg eines akzeptierten Kollapszustands. Nur unter Gate+Evidence.

use crate::propagation::{propagate_hull, Constraint, DomainState};
use cce_core::canonical::Canonicalize;
use cce_core::gate::{GateChain, GateReport};
use cce_core::signature::Digest;
use cce_core::value::CanonValue;

#[derive(Debug, Clone)]
pub struct CollapseCertificate {
    pub hull_class: Digest,
    pub horizon: String,
    pub volume: usize,
    pub gates: Vec<GateReport>,
    pub evidence_ref: String,
    pub replay_hash: Digest,
    pub mode: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CollapseError {
    GateHold(String),
    MissingEvidence,
    Infeasible,
}

fn state_class(state: &DomainState) -> Digest {
    let v = CanonValue::Map(
        state
            .iter()
            .map(|(k, vals)| {
                (
                    k.clone(),
                    CanonValue::List(vals.iter().map(CanonValue::text).collect()),
                )
            })
            .collect(),
    );
    v.canonical_class().0
}

/// Kollaps: Huelle bilden, Gates pruefen, Zertifikat NUR bei Pass+Evidence.
pub fn collapse(
    state: DomainState,
    constraints: &[Constraint],
    gates: &GateChain,
    evidence_ref: Option<&str>,
) -> Result<CollapseCertificate, CollapseError> {
    let hull = propagate_hull(state, constraints);
    if hull.infeasible {
        return Err(CollapseError::Infeasible);
    }
    if !gates.all_pass() {
        return Err(CollapseError::GateHold(
            gates
                .first_hold()
                .map(|h| h.reason.clone())
                .unwrap_or_else(|| "Gate-Kette leer".to_string()),
        ));
    }
    let ev = evidence_ref.ok_or(CollapseError::MissingEvidence)?;
    let class = state_class(&hull.state);
    Ok(CollapseCertificate {
        hull_class: class,
        horizon: "kollabiert".to_string(),
        volume: hull.state.values().map(Vec::len).sum(),
        gates: gates.reports.clone(),
        evidence_ref: ev.to_string(),
        replay_hash: class,
        mode: "lfp".to_string(),
    })
}
