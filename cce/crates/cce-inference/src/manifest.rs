//! ModelManifest (C.4): 18 Pflichtfelder. Ohne valides Manifest bleibt
//! ein Provider PASSIV (Deklaration ≠ Aktivierung, IG-A5).

/// Die fuenf Provider-Klassen (C.3).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderClass {
    /// lokal, kein Egress, staerkstes Replay-Profil
    LocalModel,
    /// Anbieter-API via model_egress — nur mit vollem Manifest
    CloudModel,
    /// externer KI-/Coding-Agent: NUR ProjectionPackets/Workcell-Kontext
    ExternalAgent,
    /// eingebettetes Kleinmodell, kein Egress
    EmbeddedSmallModel,
    /// kein Provider: Kern+Cockpit voll funktionsfaehig, Kanzel degradiert
    Disabled,
}

impl ProviderClass {
    /// Braucht diese Klasse das model_egress-Tor?
    pub fn needs_egress(self) -> bool {
        matches!(
            self,
            ProviderClass::CloudModel | ProviderClass::ExternalAgent
        )
    }
}

/// replay_policy (C.4): recorded ist Default.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ReplayPolicy {
    #[default]
    Recorded,
    Strict,
    Weak,
}

/// ModelManifest — alle Pflichtfelder aus C.4.
#[derive(Debug, Clone)]
pub struct ModelManifest {
    pub provider_id: String,
    pub provider_class: ProviderClass,
    pub model_id: String,
    pub model_version: String,
    pub context_window: u32,
    pub modality: String,
    pub supported_ops: Vec<String>,
    pub privacy_mode: Option<String>,
    pub data_retention_mode: Option<String>,
    pub logging_mode: Option<String>,
    pub cost_budget: Option<u32>,
    pub token_budget: Option<u32>,
    pub rate_limit: Option<u32>,
    pub deterministic_settings: Option<String>,
    pub replay_policy: ReplayPolicy,
    pub provider_terms_ref: Option<String>,
    pub safety_boundary: Option<String>,
    pub capability_locks: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestError {
    pub missing: Vec<&'static str>,
}

impl ModelManifest {
    /// Prueft die Pflichtfelder; fehlt eines ⇒ provider_manifest_missing
    /// (der Provider bleibt passiv).
    pub fn validate(&self) -> Result<(), ManifestError> {
        let mut missing = Vec::new();
        if self.provider_id.is_empty() {
            missing.push("provider_id");
        }
        if self.model_id.is_empty() {
            missing.push("model_id");
        }
        if self.model_version.is_empty() {
            missing.push("model_version");
        }
        if self.context_window == 0 {
            missing.push("context_window");
        }
        if self.modality.is_empty() {
            missing.push("modality");
        }
        if self.supported_ops.is_empty() {
            missing.push("supported_ops");
        }
        if self.privacy_mode.is_none() {
            missing.push("privacy_mode");
        }
        if self.data_retention_mode.is_none() {
            missing.push("data_retention_mode");
        }
        if self.logging_mode.is_none() {
            missing.push("logging_mode");
        }
        if self.cost_budget.is_none() {
            missing.push("cost_budget");
        }
        if self.token_budget.is_none() {
            missing.push("token_budget");
        }
        if self.rate_limit.is_none() {
            missing.push("rate_limit");
        }
        if self.deterministic_settings.is_none() {
            missing.push("deterministic_settings");
        }
        if self.provider_terms_ref.is_none() {
            missing.push("provider_terms_ref");
        }
        if self.safety_boundary.is_none() {
            missing.push("safety_boundary");
        }
        // capability_locks: leere Liste ist nur fuer egress-freie Klassen
        // zulaessig (Lock-Familie model_egress je Provider-Klasse, S13-A7).
        if self.provider_class.needs_egress() && self.capability_locks.is_empty() {
            missing.push("capability_locks");
        }
        if missing.is_empty() {
            Ok(())
        } else {
            Err(ManifestError { missing })
        }
    }

    /// Vollstaendiges Manifest fuer Tests/Referenzprovider.
    pub fn complete(provider_id: &str, class: ProviderClass, model_id: &str) -> Self {
        Self {
            provider_id: provider_id.to_string(),
            provider_class: class,
            model_id: model_id.to_string(),
            model_version: "1.0.0".to_string(),
            context_window: 8192,
            modality: "text".to_string(),
            supported_ops: vec!["complete".to_string(), "draft".to_string()],
            privacy_mode: Some("no_training_use".to_string()),
            data_retention_mode: Some("zero_retention".to_string()),
            logging_mode: Some("metadata_only".to_string()),
            cost_budget: Some(1000),
            token_budget: Some(100_000),
            rate_limit: Some(10),
            deterministic_settings: Some("temperature=0;seed=rd".to_string()),
            replay_policy: ReplayPolicy::Recorded,
            provider_terms_ref: Some("terms:known_compatible".to_string()),
            safety_boundary: Some("no_material_actions".to_string()),
            capability_locks: if class.needs_egress() {
                vec![format!("model_egress:{provider_id}")]
            } else {
                Vec::new()
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn incomplete_manifest_names_missing_fields() {
        let mut m = ModelManifest::complete("p1", ProviderClass::CloudModel, "m1");
        m.provider_terms_ref = None;
        m.privacy_mode = None;
        let e = m.validate().unwrap_err();
        assert!(e.missing.contains(&"provider_terms_ref"));
        assert!(e.missing.contains(&"privacy_mode"));
    }

    #[test]
    fn egress_class_requires_lock_declaration() {
        let mut m = ModelManifest::complete("p1", ProviderClass::CloudModel, "m1");
        m.capability_locks.clear();
        assert!(m.validate().is_err());
        let local = ModelManifest::complete("p2", ProviderClass::LocalModel, "m2");
        assert!(local.validate().is_ok());
    }
}
