//! CloudModelProvider — im Bau NUR als MOCK (F.1 e/F.3: reale
//! Cloud-Anbieter = Betriebsschritt). Der Mock steht hinter denselben
//! Gates wie ein echter Anbieter; sein „Egress" ist eine Fixture-Map.

use crate::manifest::{ModelManifest, ProviderClass};
use crate::request::InferenceRequest;
use crate::response::{InferenceResponse, ResponseOutcome};
use std::collections::BTreeMap;

use super::ModelProvider;

pub struct CloudModelProviderMock {
    /// Fixture: request_id → Antworttext (deterministisch).
    pub fixtures: BTreeMap<String, String>,
    /// Zaehlt „Egress"-Aufrufe — Beweismittel fuer die Zeugen (N-INF-2:
    /// struktureller Reject VOR Socket ⇒ Zaehler bleibt 0).
    pub egress_calls: std::cell::Cell<u32>,
}

impl CloudModelProviderMock {
    pub fn new(fixtures: &[(&str, &str)]) -> Self {
        Self {
            fixtures: fixtures
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
            egress_calls: std::cell::Cell::new(0),
        }
    }
}

impl ModelProvider for CloudModelProviderMock {
    fn manifest(&self) -> ModelManifest {
        ModelManifest::complete("cloud-mock", ProviderClass::CloudModel, "mock-xl")
    }

    fn infer(&self, request: &InferenceRequest) -> InferenceResponse {
        self.egress_calls.set(self.egress_calls.get() + 1);
        let outcome = match self.fixtures.get(&request.request_id) {
            Some(text) => ResponseOutcome::Output(text.clone()),
            None => ResponseOutcome::Refusal(
                "model_refusal: keine Fixture — regulaerer sichtbarer Zustand".to_string(),
            ),
        };
        InferenceResponse {
            response_id: format!("resp:cloud-mock:{}", request.request_id),
            request_id: request.request_id.clone(),
            outcome,
            provider_metadata: "cloud-mock (kein realer Anbieter, F.3)".to_string(),
            model_metadata: "mock-xl-1.0".to_string(),
            token_usage: request.context_tokens() + 64,
            latency_ms: 5,
            trace: vec!["cloud-mock fixture lookup".to_string()],
            replay_notes: "recorded: Antwort wird aufgezeichnet und eingespielt".to_string(),
        }
    }
}
