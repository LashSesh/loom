//! CloudModelProviderOpenAI (P1, Dokument 17 §3 — Overlay-Klausel: der
//! erste echte CloudModelProvider ist OpenAI, nicht Anthropic) hinter dem
//! UNVERAENDERTEN Gateway: dasselbe `ModelProvider`-Trait, dieselbe
//! Vor-Egress-Gate-Kette (`run_inference`), derselbe strukturelle
//! Nicht-Zugriff auf Gates/Urteile/Residuen/Ledger (C.2) wie jeder andere
//! Provider.
//!
//! Schluessel-Disziplin (S13, Auftraggeber-Weisung): `OPENAI_API_KEY`
//! kommt AUSSCHLIESSLICH aus der Prozess-Umgebung, wird NIE in einem
//! Feld gespeichert (kein struct-Member, keine Datei, kein Log, kein
//! Debug/Display), sondern ausschliesslich lokal innerhalb von `infer()`
//! gelesen und sofort fuer den einen Aufruf verwendet.
//!
//! Netzwerk-Disziplin: der reale HTTP-Pfad existiert NUR unter dem
//! opt-in-Feature `http` (Standard AUS) — dieselbe Disziplin wie
//! `nexus_fetch::HttpTransport`. Ohne das Feature ODER ohne gesetzten
//! Schluessel bleibt der Provider sichtbar degradiert
//! (`ResponseOutcome::Error`, `provider_unavailable`), OHNE je einen
//! Socket zu beruehren — dieselbe Disziplin wie `DisabledProvider`.

use crate::manifest::{ModelManifest, ProviderClass};
use crate::request::InferenceRequest;
use crate::response::{InferenceResponse, ResponseOutcome};

use super::ModelProvider;

pub struct CloudModelProviderOpenAI {
    pub model_id: String,
}

impl CloudModelProviderOpenAI {
    pub fn new(model_id: &str) -> Self {
        Self {
            model_id: model_id.to_string(),
        }
    }
}

impl ModelProvider for CloudModelProviderOpenAI {
    fn manifest(&self) -> ModelManifest {
        let mut m =
            ModelManifest::complete("cloud-openai", ProviderClass::CloudModel, &self.model_id);
        m.provider_terms_ref = Some("terms:known_compatible:openai".to_string());
        m
    }

    /// Egress geschieht AUSSCHLIESSLICH hier — und `run_inference` ruft
    /// `infer()` erst NACH der vollen Vor-Egress-Gate-Kette auf (Manifest/
    /// Terms/Privacy/PromptContext/Budget/Rate/Capability/Replay/
    /// NoDirectCommit/NoGateOverride). Ohne Feature `http` oder ohne
    /// gesetzten `OPENAI_API_KEY`: sichtbarer Fehler, kein Socket-Versuch.
    fn infer(&self, request: &InferenceRequest) -> InferenceResponse {
        #[cfg(feature = "http")]
        {
            if let Ok(key) = std::env::var("OPENAI_API_KEY") {
                if !key.is_empty() {
                    return self.call_openai(request, &key);
                }
            }
            unavailable_response(
                request,
                "OPENAI_API_KEY nicht gesetzt — Provider bleibt sichtbar degradiert",
            )
        }
        #[cfg(not(feature = "http"))]
        {
            unavailable_response(
                request,
                "Feature `http` nicht aktiv (Bau-Default: netzfrei) — Provider bleibt sichtbar degradiert",
            )
        }
    }
}

/// Sichtbare Degradation ohne Egress-Versuch — dieselbe Disziplin wie
/// `DisabledProvider`: nie stilles Weiterlaufen, nie ein Fallback-Inhalt.
fn unavailable_response(request: &InferenceRequest, detail: &str) -> InferenceResponse {
    InferenceResponse {
        response_id: format!("resp:cloud-openai:unavailable:{}", request.request_id),
        request_id: request.request_id.clone(),
        outcome: ResponseOutcome::Error(format!("provider_unavailable: {detail}")),
        provider_metadata: "cloud-openai (kein Egress erfolgt)".to_string(),
        model_metadata: "none".to_string(),
        token_usage: 0,
        latency_ms: 0,
        trace: vec!["cloud-openai: kein Socket-Versuch".to_string()],
        replay_notes: "deterministisch: unavailable ist stabil replay-faehig".to_string(),
    }
}

#[cfg(feature = "http")]
fn error_response(request: &InferenceRequest, detail: &str) -> InferenceResponse {
    InferenceResponse {
        response_id: format!("resp:cloud-openai:error:{}", request.request_id),
        request_id: request.request_id.clone(),
        outcome: ResponseOutcome::Error(format!("provider_unavailable: {detail}")),
        provider_metadata: "cloud-openai".to_string(),
        model_metadata: "none".to_string(),
        token_usage: 0,
        latency_ms: 0,
        trace: vec!["cloud-openai: Egress versucht, fehlgeschlagen".to_string()],
        replay_notes: "recorded: der Fehlerzustand wird aufgezeichnet".to_string(),
    }
}

#[cfg(feature = "http")]
impl CloudModelProviderOpenAI {
    /// Der einzige Egress-Aufruf. `api_key` ist ein lokaler, kurzlebiger
    /// Parameter — er wird weder zurueckgegeben noch geloggt noch in
    /// `provider_metadata`/`trace`/`replay_notes` referenziert.
    fn call_openai(&self, request: &InferenceRequest, api_key: &str) -> InferenceResponse {
        let prompt: String = request
            .context
            .iter()
            .map(|s| s.content.as_str())
            .collect::<Vec<_>>()
            .join("\n");
        let body = serde_json::json!({
            "model": self.model_id,
            "messages": [
                {"role": "system", "content": request.system_contract},
                {"role": "user", "content": prompt},
            ],
        });
        let result = ureq::post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", &format!("Bearer {api_key}"))
            .send_json(body);
        match result {
            Ok(mut resp) => match resp.body_mut().read_to_string() {
                Ok(text) => self.parse_response(request, &text),
                Err(e) => error_response(request, &format!("Antwort-Lesefehler: {e}")),
            },
            Err(e) => error_response(request, &format!("HTTP-Fehler: {e}")),
        }
    }

    fn parse_response(&self, request: &InferenceRequest, text: &str) -> InferenceResponse {
        let parsed: Result<serde_json::Value, _> = serde_json::from_str(text);
        let Ok(v) = parsed else {
            return error_response(request, "Antwort kein valides JSON — kein stilles Umformen");
        };
        let content = v["choices"][0]["message"]["content"].as_str();
        let Some(content) = content else {
            return error_response(
                request,
                "Antwort ohne choices[0].message.content — kein stilles Erfinden",
            );
        };
        let tokens = v["usage"]["total_tokens"].as_u64().unwrap_or(0) as u32;
        InferenceResponse {
            response_id: format!("resp:cloud-openai:{}:{}", self.model_id, request.request_id),
            request_id: request.request_id.clone(),
            outcome: ResponseOutcome::Output(content.to_string()),
            provider_metadata: "cloud-openai".to_string(),
            model_metadata: self.model_id.clone(),
            token_usage: tokens,
            latency_ms: 0,
            trace: vec!["cloud-openai: Egress nach voller Vor-Egress-Gate-Kette".to_string()],
            replay_notes: "recorded: Antwort wird aufgezeichnet und eingespielt".to_string(),
        }
    }
}
