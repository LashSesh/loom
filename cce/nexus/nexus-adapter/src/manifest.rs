//! AdapterManifest (CSA.3): ALLE NEUN Pflichtfelder — fehlt eines
//! ⇒ `manifest_missing`, die Quelle bleibt passiv.

#[derive(Debug, Clone)]
pub struct AdapterManifest {
    pub adapter_id: String,
    pub class: String,
    pub policy: Option<String>,
    pub license: Option<String>,
    pub privacy: Option<String>,
    pub budget_requests: Option<u32>,
    pub rate_limit_per_run: Option<u32>,
    pub provenance_strategy: Option<String>,
    pub replay_strategy: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestError {
    pub missing: Vec<&'static str>,
}

impl AdapterManifest {
    /// Prueft die 9 Pflichtfelder (Identitaet, Klasse, Policy, License,
    /// Privacy, Budget, RateLimit, Provenance, Replay).
    pub fn validate(&self) -> Result<(), ManifestError> {
        let mut missing = Vec::new();
        if self.adapter_id.is_empty() {
            missing.push("adapter_id");
        }
        if self.class.is_empty() {
            missing.push("class");
        }
        if self.policy.is_none() {
            missing.push("policy");
        }
        if self.license.is_none() {
            missing.push("license");
        }
        if self.privacy.is_none() {
            missing.push("privacy");
        }
        if self.budget_requests.is_none() {
            missing.push("budget");
        }
        if self.rate_limit_per_run.is_none() {
            missing.push("rate_limit");
        }
        if self.provenance_strategy.is_none() {
            missing.push("provenance");
        }
        if self.replay_strategy.is_none() {
            missing.push("replay");
        }
        if missing.is_empty() {
            Ok(())
        } else {
            Err(ManifestError { missing })
        }
    }

    pub fn complete(adapter_id: &str, class: &str, license: &str) -> Self {
        Self {
            adapter_id: adapter_id.to_string(),
            class: class.to_string(),
            policy: Some("official_use_permitted".to_string()),
            license: Some(license.to_string()),
            privacy: Some("no_pii".to_string()),
            budget_requests: Some(16),
            rate_limit_per_run: Some(4),
            provenance_strategy: Some("locator+hash+snapshot".to_string()),
            replay_strategy: Some("fixed_snapshot".to_string()),
        }
    }
}
