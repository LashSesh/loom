//! DisabledProvider / DegradedMode (C.3): KEIN Provider. Kern und
//! Cockpit bleiben voll funktionsfaehig; Kanzel-Funktionen sind
//! SICHTBAR degradiert (kein stilles Weiterlaufen).

use crate::manifest::{ModelManifest, ProviderClass};
use crate::request::InferenceRequest;
use crate::response::{InferenceResponse, ResponseOutcome};

use super::ModelProvider;

pub struct DisabledProvider;

impl ModelProvider for DisabledProvider {
    fn manifest(&self) -> ModelManifest {
        let mut m = ModelManifest::complete("disabled", ProviderClass::Disabled, "none");
        m.supported_ops = vec!["none".to_string()];
        m
    }

    /// Antwortet IMMER mit sichtbarer Degradation — nie mit Inhalt.
    fn infer(&self, request: &InferenceRequest) -> InferenceResponse {
        InferenceResponse {
            response_id: format!("resp:disabled:{}", request.request_id),
            request_id: request.request_id.clone(),
            outcome: ResponseOutcome::Error(
                "provider_unavailable: Kanzel degradiert, Kern voll funktionsfaehig".to_string(),
            ),
            provider_metadata: "disabled".to_string(),
            model_metadata: "none".to_string(),
            token_usage: 0,
            latency_ms: 0,
            trace: vec!["disabled: kein Egress, keine Inferenz".to_string()],
            replay_notes: "deterministisch: immer degradiert".to_string(),
        }
    }
}
