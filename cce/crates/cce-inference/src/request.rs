//! InferenceRequest (C.5) — mit Kontext-Schnitt: Egress ⊆ allowed_context,
//! ∩ forbidden_context = ∅ (Pruefvorrang fuer forbidden).

use cce_core::signature::{sha256, Digest};

/// Der Kontext-Schnitt eines Requests: benannte Kontextstuecke.
#[derive(Debug, Clone)]
pub struct ContextSlice {
    pub name: String,
    pub content: String,
}

impl ContextSlice {
    pub fn digest(&self) -> Digest {
        sha256(format!("{}\u{1f}{}", self.name, self.content).as_bytes())
    }
}

/// InferenceRequest — Pflichtfelder aus C.5.
#[derive(Debug, Clone)]
pub struct InferenceRequest {
    pub request_id: String,
    pub rd_ref: String,
    pub projection_id: String,
    pub workcell_id: String,
    /// Nur diese Kontextstuecke DUERFEN gesendet werden.
    pub allowed_context: Vec<String>,
    /// Pruefvorrang: nichts hiervon darf je das Tor passieren.
    pub forbidden_context: Vec<String>,
    /// Der tatsaechlich zu sendende Kontext.
    pub context: Vec<ContextSlice>,
    pub output_schema: String,
    pub system_contract: String,
    pub tool_policy: String,
    pub privacy_profile: String,
    pub budget_tokens: u32,
    pub sampling_policy: String,
    pub seed: u64,
    pub trace_id: String,
}

impl InferenceRequest {
    /// Digest ueber den GESENDETEN Kontext — wird in der Evidence fixiert
    /// (C.5: „der gesendete Kontext-Digest wird in der Evidence fixiert").
    pub fn context_digest(&self) -> Digest {
        let mut buf = Vec::new();
        for s in &self.context {
            buf.extend_from_slice(&s.digest().0);
        }
        sha256(&buf)
    }

    /// Grobe Token-Schaetzung (deterministisch): Bytes/4, kein Float.
    pub fn context_tokens(&self) -> u32 {
        let bytes: usize = self.context.iter().map(|s| s.content.len()).sum();
        (bytes / 4) as u32
    }

    /// Referenz-Request fuer Zeugen.
    pub fn example(request_id: &str) -> Self {
        Self {
            request_id: request_id.to_string(),
            rd_ref: "rd:run-1".to_string(),
            projection_id: "proj:p1".to_string(),
            workcell_id: "w:cell-1".to_string(),
            allowed_context: vec!["projektion".to_string(), "wunsch".to_string()],
            forbidden_context: vec!["geheimnis".to_string()],
            context: vec![ContextSlice {
                name: "projektion".to_string(),
                content: "ProjectionPacket: drei Risiken".to_string(),
            }],
            output_schema: "text/markdown-entwurf".to_string(),
            system_contract: "kandidat, kein urteil".to_string(),
            tool_policy: "none".to_string(),
            privacy_profile: "no_pii".to_string(),
            budget_tokens: 4096,
            sampling_policy: "greedy".to_string(),
            seed: 7,
            trace_id: "trace:1".to_string(),
        }
    }
}
