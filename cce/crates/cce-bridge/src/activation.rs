//! S-E5 §5/§10(g): Aktivierung — der Aktivierungs-Hook, den `cce-runner`
//! aufruft (Abhaengigkeitsrichtung `cce-runner -> cce-bridge`, kein
//! Zyklus). Eine Norm wirkt NUR, wenn ein Auftrag sie ausdruecklich per
//! `norm_profile` aktiviert; jede norm-geformte Hilfe traegt die
//! Markierung `norm-geformt:<norm_id>`; Anwendung ohne Aktivierung ist
//! `norm_not_activated` (PROD-INV-22).

use crate::types::{residue, BridgeNorm, NormStatus};

/// §5: die explizite Aktivierungsliste eines Laufs/einer Capsule.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct NormProfile {
    pub activated_norm_ids: Vec<String>,
}

impl NormProfile {
    pub fn new(activated_norm_ids: Vec<String>) -> Self {
        Self { activated_norm_ids }
    }

    pub fn is_activated(&self, norm_id: &str) -> bool {
        self.activated_norm_ids.iter().any(|id| id == norm_id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActivationError {
    /// N-NRM-6/PROD-INV-22: Anwendung ohne explizite Aktivierung.
    NotActivated,
    /// Eine Norm, die nicht (mehr) `Active` ist, darf nicht wirken —
    /// selbst wenn sie im Profil steht (Erosion/Widerruf gehen vor).
    NormNotActive(NormStatus),
}

impl ActivationError {
    pub fn residue(&self) -> &'static str {
        match self {
            ActivationError::NotActivated => residue::NORM_NOT_ACTIVATED,
            ActivationError::NormNotActive(NormStatus::Revoked) => residue::NORM_SINCE_REVOKED,
            ActivationError::NormNotActive(_) => residue::PROVENANCE_EROSION,
        }
    }
}

/// §5: markiert eine Kanzel-/Workbench-Hilfe als `norm-geformt:<norm_id>`.
pub fn norm_shaped_marker(norm_id: &str) -> String {
    format!("norm-geformt:{norm_id}")
}

/// Der eigentliche Aktivierungs-Hook (`cce-runner` ruft dies vor jeder
/// norm-geformten Wirkung eines Laufs auf): fail-closed — nur eine im
/// `norm_profile` gelistete UND `Active` Norm darf wirken.
pub fn activate(norm: &BridgeNorm, profile: &NormProfile) -> Result<String, ActivationError> {
    if !profile.is_activated(&norm.norm_id) {
        return Err(ActivationError::NotActivated);
    }
    if norm.status != NormStatus::Active {
        return Err(ActivationError::NormNotActive(norm.status));
    }
    Ok(norm_shaped_marker(&norm.norm_id))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ProvenanceSet, Scope};

    fn active_norm() -> BridgeNorm {
        BridgeNorm {
            norm_id: "norm:x".to_string(),
            pattern: crate::pattern::Pattern::StructuralRule(
                crate::pattern::DomainRuleForm::UniqueSubjects,
            ),
            provenance_set: ProvenanceSet::new(vec!["aa".repeat(34)]),
            known_counterexamples: vec![],
            scope: Scope::Global,
            status: NormStatus::Active,
            promotion_evidence: "e".to_string(),
            supersedes: None,
        }
    }

    /// R-NRM-2: Aktivierung mit passendem Profil wirkt.
    #[test]
    fn activated_norm_produces_marker() {
        let norm = active_norm();
        let profile = NormProfile::new(vec![norm.norm_id.clone()]);
        let marker = activate(&norm, &profile).expect("aktiviert");
        assert_eq!(marker, "norm-geformt:norm:x");
    }

    /// N-NRM-6/PROD-INV-22: ohne Aktivierung ⇒ norm_not_activated.
    #[test]
    fn unactivated_norm_is_refused() {
        let norm = active_norm();
        let profile = NormProfile::default();
        let err = activate(&norm, &profile).unwrap_err();
        assert_eq!(err, ActivationError::NotActivated);
        assert_eq!(err.residue(), residue::NORM_NOT_ACTIVATED);
    }

    #[test]
    fn revoked_norm_in_profile_is_still_refused() {
        let mut norm = active_norm();
        norm.status = NormStatus::Revoked;
        let profile = NormProfile::new(vec![norm.norm_id.clone()]);
        let err = activate(&norm, &profile).unwrap_err();
        assert_eq!(err, ActivationError::NormNotActive(NormStatus::Revoked));
        assert_eq!(err.residue(), residue::NORM_SINCE_REVOKED);
    }
}
