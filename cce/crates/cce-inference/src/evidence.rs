//! InferenceEvidence (C.7): ohne sie ist ein Kandidat nicht gate-faehig
//! (`model_trace_missing`).

use crate::request::InferenceRequest;
use crate::response::InferenceResponse;
use cce_core::gate::GateReport;
use cce_core::signature::Digest;

#[derive(Debug, Clone)]
pub struct InferenceEvidence {
    pub evidence_id: String,
    /// Request-Digest inkl. Kontext-Schnitt.
    pub request_digest: Digest,
    pub context_digest: Digest,
    pub manifest_ref: String,
    pub projection_ref: String,
    pub rd_ref: String,
    pub response_digest: Digest,
    pub egress_gate_reports: Vec<GateReport>,
    pub cost_tokens: u32,
    pub latency_ms: u32,
}

pub fn build_evidence(
    req: &InferenceRequest,
    resp: &InferenceResponse,
    manifest_ref: &str,
    gate_reports: Vec<GateReport>,
) -> InferenceEvidence {
    InferenceEvidence {
        evidence_id: format!("iev:{}", resp.digest().to_hex()),
        request_digest: cce_core::signature::sha256(req.request_id.as_bytes()),
        context_digest: req.context_digest(),
        manifest_ref: manifest_ref.to_string(),
        projection_ref: req.projection_id.clone(),
        rd_ref: req.rd_ref.clone(),
        response_digest: resp.digest(),
        egress_gate_reports: gate_reports,
        cost_tokens: resp.token_usage,
        latency_ms: resp.latency_ms,
    }
}
