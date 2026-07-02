//! LocalModelProvider (C.3): lokal, KEIN Egress, staerkstes
//! Replay-Profil. Echtpfad als deterministischer Stub (F.1 d zulaessig):
//! seed-deterministische Ableitung des Entwurfs aus dem Kontext —
//! replay_policy=strict ist damit ehrlich erfuellbar.

use crate::manifest::{ModelManifest, ProviderClass, ReplayPolicy};
use crate::request::InferenceRequest;
use crate::response::{InferenceResponse, ResponseOutcome};

use super::ModelProvider;

pub struct LocalModelProvider {
    pub model_name: String,
}

impl LocalModelProvider {
    pub fn new(model_name: &str) -> Self {
        Self {
            model_name: model_name.to_string(),
        }
    }
}

impl ModelProvider for LocalModelProvider {
    fn manifest(&self) -> ModelManifest {
        let mut m = ModelManifest::complete(
            &format!("local:{}", self.model_name),
            ProviderClass::LocalModel,
            &self.model_name,
        );
        m.replay_policy = ReplayPolicy::Strict;
        m
    }

    fn infer(&self, request: &InferenceRequest) -> InferenceResponse {
        // Deterministisch: Digest des Kontexts + RD-Seed → Entwurf.
        let ctx = request.context_digest().to_hex();
        // splitmix64-Schritt direkt auf dem RD-Seed (deterministisch).
        let mixed = request
            .seed
            .wrapping_add(0x9e37_79b9_7f4a_7c15)
            .wrapping_mul(0xbf58_476d_1ce4_e5b9);
        let flavor = mixed % 3;
        let content = format!(
            "ENTWURF (kandidat, kein urteil) — schema {} — kontext {} — variante {}",
            request.output_schema,
            &ctx[..16],
            flavor
        );
        InferenceResponse {
            response_id: format!("resp:local:{}:{}", self.model_name, request.request_id),
            request_id: request.request_id.clone(),
            outcome: ResponseOutcome::Output(content),
            provider_metadata: format!("local:{}", self.model_name),
            model_metadata: "stub-1.0 deterministisch".to_string(),
            token_usage: request.context_tokens() + 32,
            latency_ms: 1,
            trace: vec![format!("seed={} ctx={}", request.seed, &ctx[..16])],
            replay_notes: "strict: gleicher Seed+Kontext ⇒ identische Antwort".to_string(),
        }
    }
}
