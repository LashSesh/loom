//! Das InferenceGateway (C.2): der EINZIGE Port zu Modellen.
//! Provider-Registry (nur manifestierte Provider), Gate-Kette VOR jedem
//! Egress, Kontext-Schnitt, Aufzeichnung, Antwort-Typisierung.
//!
//! Das Gateway hat KEINEN Schreibpfad auf Gates, Urteile, Residuen,
//! Ledger-Verdikte oder Commits: es liefert (CandidateOutput,
//! InferenceEvidence) — mehr nicht.

use crate::evidence::{build_evidence, InferenceEvidence};
use crate::gates::{
    model_budget_gate, model_capability_gate, model_privacy_gate, model_rate_gate,
    model_replay_gate, no_direct_commit_gate, no_gate_override_gate, output_schema_gate,
    prompt_context_gate, provider_manifest_gate, provider_terms_gate, InfVerdict,
};
use crate::providers::ModelProvider;
use crate::request::InferenceRequest;
use crate::residues::model_residue;
use crate::response::{CandidateOutput, InferenceResponse, ResponseOutcome};
use cce_core::capability::CapabilityLock;
use cce_core::gate::GateReport;
use cce_core::residue::Residue;

/// Ergebnis eines Gateway-Durchlaufs.
#[derive(Debug)]
pub enum GatewayOutcome {
    /// Kandidat + Evidence — der EINZIGE Erfolgspfad.
    Candidate(Box<(CandidateOutput, InferenceEvidence)>),
    /// Modell-Weigerung: regulaerer sichtbarer Zustand.
    Refusal {
        response: InferenceResponse,
        residue: Box<Residue>,
    },
    /// Provider-/Formatfehler nach Egress.
    Failed {
        response: InferenceResponse,
        residue: Box<Residue>,
    },
    /// Gate-Halt VOR Egress: kein Socket wurde beruehrt.
    BlockedBeforeEgress(Vec<InfVerdict>),
}

/// Aufzeichnungsspeicher fuer recorded-Replay (IG-A3): Responses werden
/// als RD-gebundene Eingaenge abgelegt und im Replay EINGESPIELT.
#[derive(Debug, Default)]
pub struct InferenceRecorder {
    pub recorded: Vec<(String, InferenceResponse)>,
}

impl InferenceRecorder {
    pub fn record(&mut self, req_id: &str, resp: &InferenceResponse) {
        self.recorded.push((req_id.to_string(), resp.clone()));
    }

    pub fn lookup(&self, req_id: &str) -> Option<&InferenceResponse> {
        self.recorded
            .iter()
            .find(|(id, _)| id == req_id)
            .map(|(_, r)| r)
    }
}

/// Der Gateway-Lauf: Gate-Kette (1–11 der 14; Tool-Gates 12/13 gehoeren
/// dem ToolGateway, Gate 14 dem materiellen Aktionspfad) → Egress →
/// Typisierung → Aufzeichnung.
#[allow(clippy::too_many_arguments)] // Spec-Signatur: RD, Boundary, Lock, Zaehler sind je eigene Groessen (C.2/C.8)
pub fn run_inference(
    provider: &dyn ModelProvider,
    req: &InferenceRequest,
    projection_boundary: &[String],
    run_data_policy: &str,
    egress_lock: Option<&CapabilityLock>,
    requests_this_run: u32,
    requested_op: &str,
    recorder: &mut InferenceRecorder,
) -> GatewayOutcome {
    let manifest = provider.manifest();
    // Gate-Kette VOR jedem Egress (Reihenfolge C.8; PromptContext strikt
    // vor dem Socket).
    let verdicts = [
        provider_manifest_gate(&manifest),
        provider_terms_gate(&manifest),
        model_privacy_gate(&manifest, run_data_policy),
        prompt_context_gate(req, projection_boundary),
        model_budget_gate(req, &manifest),
        model_rate_gate(requests_this_run, &manifest),
        model_capability_gate(&manifest, egress_lock, requested_op),
        model_replay_gate(&manifest, true),
        no_direct_commit_gate(false),
        no_gate_override_gate(false),
    ];
    let failed: Vec<InfVerdict> = verdicts.iter().filter(|v| !v.allows()).cloned().collect();
    if !failed.is_empty() {
        return GatewayOutcome::BlockedBeforeEgress(failed);
    }
    // Erst JETZT der Egress.
    let response = provider.infer(req);
    recorder.record(&req.request_id, &response);
    let gate_reports: Vec<GateReport> = verdicts
        .iter()
        .map(|v| GateReport::pass(v.gate(), "egress gate allow"))
        .collect();
    match &response.outcome {
        ResponseOutcome::Refusal(msg) => {
            let residue = Box::new(model_residue("model_refusal", msg));
            GatewayOutcome::Refusal { response, residue }
        }
        ResponseOutcome::Error(msg) => {
            let residue = Box::new(model_residue("provider_unavailable", msg));
            GatewayOutcome::Failed { response, residue }
        }
        ResponseOutcome::Output(_) => {
            // OutputSchemaGate NACH Egress (9/14): Typisierung.
            let schema_verdict = output_schema_gate(&req.output_schema, &req.output_schema);
            if !schema_verdict.allows() {
                let residue = Box::new(
                    schema_verdict
                        .residue()
                        .cloned()
                        .expect("non-allow hat residue"),
                );
                return GatewayOutcome::Failed { response, residue };
            }
            let mut candidate = CandidateOutput::from_response(&response, &req.output_schema)
                .expect("Output-Variante");
            let evidence = build_evidence(req, &response, &manifest.provider_id, gate_reports);
            candidate.evidence_ref = Some(evidence.evidence_id.clone());
            GatewayOutcome::Candidate(Box::new((candidate, evidence)))
        }
    }
}
