//! EmbeddedSmallModelProvider (C.3): eingebettetes Kleinmodell,
//! kein Egress — hier ein deterministischer Klassifikations-Stub.

use crate::manifest::{ModelManifest, ProviderClass, ReplayPolicy};
use crate::request::InferenceRequest;
use crate::response::{InferenceResponse, ResponseOutcome};

use super::ModelProvider;

pub struct EmbeddedSmallModelProvider;

impl ModelProvider for EmbeddedSmallModelProvider {
    fn manifest(&self) -> ModelManifest {
        let mut m = ModelManifest::complete("embedded", ProviderClass::EmbeddedSmallModel, "tiny");
        m.context_window = 1024;
        m.supported_ops = vec![
            "classify".to_string(),
            "complete".to_string(),
            "draft".to_string(),
        ];
        m.replay_policy = ReplayPolicy::Strict;
        m
    }

    fn infer(&self, request: &InferenceRequest) -> InferenceResponse {
        let label = if request.context_tokens() > 64 {
            "lang"
        } else {
            "kurz"
        };
        InferenceResponse {
            response_id: format!("resp:embedded:{}", request.request_id),
            request_id: request.request_id.clone(),
            outcome: ResponseOutcome::Output(format!("klasse: {label}")),
            provider_metadata: "embedded".to_string(),
            model_metadata: "tiny-1.0".to_string(),
            token_usage: 8,
            latency_ms: 0,
            trace: vec!["embedded classify".to_string()],
            replay_notes: "strict: reine Funktion des Kontexts".to_string(),
        }
    }
}
