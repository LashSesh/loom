//! InferenceResponse (C.6) und CandidateOutput (C.7).
//!
//! Der CandidateOutput ist STRUKTURELL kein Commit: er besitzt kein
//! Status-, kein Verdikt- und kein Ledger-Feld — der einzige Weg zur
//! Wirkung fuehrt durch die Motor-Gates (NoDirectCommitGate, C.8).

use cce_core::signature::{sha256, Digest};

/// Modell-Weigerung ist ein REGULAERER sichtbarer Zustand (C.6) —
/// nie ein stiller Retry mit aufgeweichtem Kontext.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResponseOutcome {
    Output(String),
    Refusal(String),
    Error(String),
}

/// InferenceResponse — Pflichtfelder aus C.6.
#[derive(Debug, Clone)]
pub struct InferenceResponse {
    pub response_id: String,
    pub request_id: String,
    pub outcome: ResponseOutcome,
    pub provider_metadata: String,
    pub model_metadata: String,
    pub token_usage: u32,
    pub latency_ms: u32,
    pub trace: Vec<String>,
    pub replay_notes: String,
}

impl InferenceResponse {
    pub fn digest(&self) -> Digest {
        let body = match &self.outcome {
            ResponseOutcome::Output(s) => format!("out\u{1f}{s}"),
            ResponseOutcome::Refusal(s) => format!("ref\u{1f}{s}"),
            ResponseOutcome::Error(s) => format!("err\u{1f}{s}"),
        };
        sha256(
            format!(
                "{}\u{1f}{}\u{1f}{}",
                self.response_id, self.request_id, body
            )
            .as_bytes(),
        )
    }
}

/// CandidateOutput (C.7): niemals Commit, niemals Gate-Urteil, niemals
/// Ledger-Schreibzugriff. Traegt optional eine Modell-Einschaetzung —
/// als ANZEIGE (COCK-INV-8: „Einschaetzung, kein Urteil"), die kein
/// Gate ersetzen kann (getestet in N-INF-12).
#[derive(Debug, Clone)]
pub struct CandidateOutput {
    pub candidate_id: String,
    pub response_ref: String,
    pub content: String,
    pub output_schema: String,
    /// Einschaetzung in Promille — reine Anzeige, nie Verdikt (IG-A4).
    pub self_assessment_permille: Option<u16>,
    pub evidence_ref: Option<String>,
}

impl CandidateOutput {
    pub fn from_response(resp: &InferenceResponse, output_schema: &str) -> Option<Self> {
        match &resp.outcome {
            ResponseOutcome::Output(content) => Some(Self {
                candidate_id: format!("cand:{}", resp.digest().to_hex()),
                response_ref: resp.response_id.clone(),
                content: content.clone(),
                output_schema: output_schema.to_string(),
                self_assessment_permille: None,
                evidence_ref: None,
            }),
            _ => None,
        }
    }
}
