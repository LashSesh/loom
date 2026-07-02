//! Die 16 ModelResidues (C.9) — jedes sichtbar, keines still.

use cce_core::residue::{Residue, ResidueKind, Severity};

pub const ALL_MODEL_RESIDUES: [&str; 16] = [
    "provider_unavailable",
    "provider_manifest_missing",
    "model_context_overflow",
    "model_privacy_block",
    "model_budget_exceeded",
    "model_rate_limited",
    "model_terms_unknown",
    "model_retention_incompatible",
    "model_output_schema_invalid",
    "model_refusal",
    "model_replay_weak",
    "model_trace_missing",
    "model_attempted_direct_commit",
    "model_attempted_gate_override",
    "prompt_context_boundary_violation",
    "sensitive_context_egress_blocked",
];

/// Erzeugt ein benanntes Modell-Residuum. `model_refusal` ist ein
/// REGULAERER sichtbarer Zustand (kein stiller Retry, C.6);
/// `model_replay_weak` ist die sichtbare Nichtdeterminismus-Anzeige
/// (IG-A3). Beide sind Warning; Angriffe auf Commit/Gate-Pfad und
/// Boundary-Verletzungen sind Blocking.
pub fn model_residue(kind: &str, detail: &str) -> Residue {
    debug_assert!(
        ALL_MODEL_RESIDUES.contains(&kind),
        "unbekanntes Modell-Residuum"
    );
    let severity = match kind {
        "model_refusal" | "model_replay_weak" => Severity::Warning,
        _ => Severity::Blocking,
    };
    Residue::new(
        &format!("inf:{kind}"),
        "inference",
        ResidueKind::named(kind),
        severity,
        detail,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sixteen_residues_all_constructible() {
        for k in ALL_MODEL_RESIDUES {
            let r = model_residue(k, "test");
            assert!(r.id.contains(k));
        }
    }
}
