//! R-SWE-5 (Dokument 18 §8): eine vom Provider erzeugte Diff — "OpenAI
//! hinter dem unveraenderten Gateway, recorded". Dieselbe
//! `ModelProvider`-Schnittstelle, derselbe unveraenderte
//! `cce_inference::gateway::run_inference()` wie jeder andere Provider
//! (P1-Disziplin unveraendert: alle zehn Vor-Egress-Gates, Aufzeichnung
//! vor Typisierung). `RecordedOpenAiDiffProvider` ist eine hermetische,
//! aufgezeichnete Fixture — der reale `CloudModelProviderOpenAI` aus P1
//! (`crates/cce-inference/src/providers/openai.rs`, Feature `http`)
//! bleibt fuer den echten Betriebsfall vollstaendig unangetastet; diese
//! Fixture traegt bewusst denselben `provider_id = "cloud-openai"` und
//! dieselbe `ProviderClass::CloudModel`, um strukturell identisch durch
//! die Gate-Kette zu laufen.

use crate::model::{DiffCandidate, DiffHunk, ProducedBy};
use cce_core::capability::CapabilityLock;
use cce_core::signature::Digest;
use cce_inference::gateway::{run_inference, GatewayOutcome, InferenceRecorder};
use cce_inference::manifest::{ModelManifest, ProviderClass};
use cce_inference::providers::ModelProvider;
use cce_inference::request::InferenceRequest;
use cce_inference::response::{InferenceResponse, ResponseOutcome};

/// Hermetische, aufgezeichnete Stellvertreter-Antwort fuer "OpenAI
/// hinter dem Gateway" (Bau-Default bleibt netzfrei — kein Feature
/// `http`, kein Socket).
pub struct RecordedOpenAiDiffProvider {
    pub model_id: String,
    pub recorded_diff: String,
}

impl ModelProvider for RecordedOpenAiDiffProvider {
    fn manifest(&self) -> ModelManifest {
        ModelManifest::complete("cloud-openai", ProviderClass::CloudModel, &self.model_id)
    }

    fn infer(&self, request: &InferenceRequest) -> InferenceResponse {
        InferenceResponse {
            response_id: format!("resp:cloud-openai:recorded:{}", request.request_id),
            request_id: request.request_id.clone(),
            outcome: ResponseOutcome::Output(self.recorded_diff.clone()),
            provider_metadata: "cloud-openai (recorded fixture, kein Live-Egress im Bau-Default)"
                .to_string(),
            model_metadata: self.model_id.clone(),
            token_usage: request.context_tokens() + 32,
            latency_ms: 0,
            trace: vec![
                "cloud-openai: aufgezeichnete Antwort eingespielt, kein Socket".to_string(),
            ],
            replay_notes: "recorded: Antwort wird aufgezeichnet und eingespielt".to_string(),
        }
    }
}

/// Fehler beim Erzeugen eines DiffCandidate aus einem Provider-Lauf.
#[derive(Debug)]
pub enum ProviderDiffError {
    /// Gate-Halt VOR Egress oder Provider-Fehlschlag NACH Egress.
    Gateway(Box<GatewayOutcome>),
}

/// Fuehrt EINEN Provider-Lauf durch das unveraenderte Gateway und
/// bildet das Ergebnis (der Modell-Content ist der unified diff) als
/// DiffCandidate ab. `produced_by` traegt den echten `provider_id` aus
/// dem Manifest — nie geraten.
#[allow(clippy::too_many_arguments)]
pub fn provider_diff_candidate(
    provider: &dyn ModelProvider,
    request: &InferenceRequest,
    path: &str,
    rationale: &str,
    base_snapshot_root: Digest,
    egress_lock: &CapabilityLock,
    recorder: &mut InferenceRecorder,
) -> Result<DiffCandidate, ProviderDiffError> {
    let boundary = vec!["projektion".to_string(), "wunsch".to_string()];
    let out = run_inference(
        provider,
        request,
        &boundary,
        "no_pii",
        Some(egress_lock),
        0,
        "draft",
        recorder,
    );
    match out {
        GatewayOutcome::Candidate(boxed) => {
            let (candidate, _evidence) = *boxed;
            Ok(DiffCandidate {
                base_snapshot_root,
                hunks: vec![DiffHunk {
                    path: path.to_string(),
                    unified_diff: candidate.content,
                }],
                rationale: rationale.to_string(),
                produced_by: ProducedBy::Provider {
                    provider_id: provider.manifest().provider_id,
                    manifest_ref: provider.manifest().model_id,
                },
            })
        }
        other => Err(ProviderDiffError::Gateway(Box::new(other))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provider_diff_runs_full_gate_chain_and_is_recorded() {
        let provider = RecordedOpenAiDiffProvider {
            model_id: "gpt-4o-mini".to_string(),
            recorded_diff:
                "@@ -1,3 +1,3 @@\n fn add(a: i32, b: i32) -> i32 {\n-    a - b\n+    a + b\n }\n"
                    .to_string(),
        };
        let req = InferenceRequest::example("r-swe-5");
        let mut lock = CapabilityLock::closed("model_egress:cloud-openai");
        lock.open("operator:sk", "ledger:e1");
        let mut rec = InferenceRecorder::default();

        let diff = provider_diff_candidate(
            &provider,
            &req,
            "repo/src/lib.rs",
            "OpenAI-erzeugte Korrektur des Vorzeichenfehlers",
            Digest([0u8; 32]),
            &lock,
            &mut rec,
        )
        .expect("Provider-Diff gelingt");

        assert_eq!(diff.hunks.len(), 1);
        assert!(diff.hunks[0].unified_diff.contains("a + b"));
        match &diff.produced_by {
            ProducedBy::Provider { provider_id, .. } => assert_eq!(provider_id, "cloud-openai"),
            ProducedBy::Operator { .. } => panic!("erwartet Provider-Herkunft"),
        }
        // Recorded: die Antwort ist im Recorder wiederauffindbar (der
        // eigentliche Replay-Zeuge R-SWE-3 nutzt genau diesen Recorder).
        assert!(rec.lookup("r-swe-5").is_some());
    }

    /// Ohne offenen model_egress-Lock haelt ModelCapabilityGate VOR
    /// jedem Providerkontakt — dieselbe Garantie wie bei jedem anderen
    /// CloudModel-Provider (P1/R-INF-3-Disziplin), jetzt am
    /// Provider-Diff-Pfad.
    #[test]
    fn provider_diff_blocked_before_egress_without_lock() {
        let provider = RecordedOpenAiDiffProvider {
            model_id: "gpt-4o-mini".to_string(),
            recorded_diff: "irrelevant".to_string(),
        };
        let req = InferenceRequest::example("r-swe-5-nolock");
        let closed_lock = CapabilityLock::closed("model_egress:cloud-openai");
        let mut rec = InferenceRecorder::default();
        let err = provider_diff_candidate(
            &provider,
            &req,
            "repo/src/lib.rs",
            "sollte nie ankommen",
            Digest([0u8; 32]),
            &closed_lock,
            &mut rec,
        )
        .unwrap_err();
        match err {
            ProviderDiffError::Gateway(boxed) => match *boxed {
                GatewayOutcome::BlockedBeforeEgress(failed) => {
                    assert!(failed.iter().any(|v| v.gate() == "ModelCapabilityGate"));
                }
                other => panic!("erwartet Gate-Halt VOR Egress, war {other:?}"),
            },
        }
    }

    /// R-SWE-3 (fuer den Provider-Pfad): die Aufzeichnung IST die
    /// Antwort — ein zweiter "Lauf" liest sie ueber `replay_response`
    /// ein, OHNE `infer()` ein zweites Mal aufzurufen (S-A5/A6: kein
    /// Live-Re-Call im Replay-Pfad).
    #[test]
    fn recorded_provider_response_replays_without_second_live_call() {
        use cce_inference::replay::replay_response;

        let provider = RecordedOpenAiDiffProvider {
            model_id: "gpt-4o-mini".to_string(),
            recorded_diff: "@@ -1,3 +1,3 @@\n a\n-b\n+c\n d\n".to_string(),
        };
        let req = InferenceRequest::example("r-swe-3-provider");
        let mut lock = CapabilityLock::closed("model_egress:cloud-openai");
        lock.open("operator:sk", "ledger:e1");
        let mut rec = InferenceRecorder::default();

        let live = provider_diff_candidate(
            &provider,
            &req,
            "repo/src/lib.rs",
            "erster Lauf",
            Digest([0u8; 32]),
            &lock,
            &mut rec,
        )
        .expect("erster Lauf gelingt");

        // Zweiter "Lauf": KEIN erneuter provider_diff_candidate()-Aufruf
        // (der wieder infer() riefe) — stattdessen die Aufzeichnung
        // direkt eingespielt.
        let replayed =
            replay_response(&rec, "r-swe-3-provider").expect("Replay findet die Aufzeichnung");
        match replayed.outcome {
            cce_inference::response::ResponseOutcome::Output(content) => {
                assert_eq!(content, live.hunks[0].unified_diff);
            }
            other => panic!("erwartet Output, war {other:?}"),
        }
    }
}
