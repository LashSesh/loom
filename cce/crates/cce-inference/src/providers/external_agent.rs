//! ExternalAgentProvider (C.3): externer KI-/Coding-Agent. Er erhaelt
//! NUR ProjectionPackets/Workcell-Kontext — die Signatur des Ports
//! laesst ihm gar keinen anderen Zugriff; Repo-Freiheit gaebe es nur
//! ueber einen expliziten ToolCapabilityLock (getestet N-INF-9).

use crate::manifest::{ModelManifest, ProviderClass};
use crate::request::InferenceRequest;
use crate::response::{InferenceResponse, ResponseOutcome};

use super::ModelProvider;

pub struct ExternalAgentProviderMock {
    /// Simuliert einen Agenten, der mehr sehen WILL: fordert er
    /// Vollzugriff an, wird das als regulaerer Weigerungs-/Fehlerpfad
    /// sichtbar — nie still erfuellt.
    pub demands_full_repo: bool,
}

impl ModelProvider for ExternalAgentProviderMock {
    fn manifest(&self) -> ModelManifest {
        ModelManifest::complete(
            "external-agent-mock",
            ProviderClass::ExternalAgent,
            "agent-1",
        )
    }

    fn infer(&self, request: &InferenceRequest) -> InferenceResponse {
        if self.demands_full_repo {
            return InferenceResponse {
                response_id: format!("resp:agent:{}", request.request_id),
                request_id: request.request_id.clone(),
                outcome: ResponseOutcome::Error(
                    "agent fordert Repo-Vollzugriff — nur via expliziten ToolCapabilityLock"
                        .to_string(),
                ),
                provider_metadata: "external-agent-mock".to_string(),
                model_metadata: "agent-1".to_string(),
                token_usage: 0,
                latency_ms: 1,
                trace: vec!["full-repo demand rejected upstream".to_string()],
                replay_notes: "recorded".to_string(),
            };
        }
        InferenceResponse {
            response_id: format!("resp:agent:{}", request.request_id),
            request_id: request.request_id.clone(),
            outcome: ResponseOutcome::Output(format!(
                "AGENT-KANDIDAT auf Basis ProjectionPacket {} (kein anderer Zugriff)",
                request.projection_id
            )),
            provider_metadata: "external-agent-mock".to_string(),
            model_metadata: "agent-1".to_string(),
            token_usage: request.context_tokens() + 16,
            latency_ms: 3,
            trace: vec![format!("sicht = {}", request.projection_id)],
            replay_notes: "recorded".to_string(),
        }
    }
}
