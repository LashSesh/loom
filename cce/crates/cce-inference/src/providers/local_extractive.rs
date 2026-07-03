//! LocalExtractiveModel (P4, Track C) — ein ECHTES lokales Modell, kein
//! Mock: ein deterministisches extraktives Sprachmodell, das den
//! allowed_context WIRKLICH verarbeitet (Term-Frequenz-Gewichtung, Satz-
//! Ranking) und daraus einen Entwurf erzeugt. Provider-Klasse
//! LocalModel: KEIN Egress, replay_policy=recorded (deterministisch ⇒
//! Aufzeichnung und Einspielung sind klassenidentisch). Es liegt
//! ausschliesslich unter providers/ und laeuft hinter dem unveraenderten
//! Gateway/Gates.
//!
//! EHRLICH: dies ist ein leichtgewichtiges, regelbasiertes Lokalmodell
//! (extraktiv/heuristisch), KEIN neuronales LLM. Die Anbindung eines
//! GGUF-/llama.cpp-Modells ist ein Folgeschritt (Modell-Artefakt +
//! Host-Ressourcen), im Bericht als Residuum gefuehrt.

use crate::manifest::{ModelManifest, ProviderClass, ReplayPolicy};
use crate::request::InferenceRequest;
use crate::response::{InferenceResponse, ResponseOutcome};

use super::ModelProvider;

pub struct LocalExtractiveModel {
    pub model_name: String,
}

impl LocalExtractiveModel {
    pub fn new(model_name: &str) -> Self {
        Self {
            model_name: model_name.to_string(),
        }
    }

    /// Der eigentliche Modellkern: reine Funktion des Kontexts.
    /// Schritte: Kontext in Saetze zerlegen; Term-Frequenzen (ohne
    /// triviale Stoppwoerter) zaehlen; jeden Satz nach Summe seiner
    /// Term-Gewichte ranken; die staerksten Saetze als Entwurf ausgeben.
    fn draft(&self, context: &str) -> String {
        let sentences: Vec<&str> = context
            .split(['.', '\n', ';'])
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();
        if sentences.is_empty() {
            return "(leerer Kontext)".to_string();
        }
        // Term-Frequenz ueber den gesamten Kontext.
        let mut freq: std::collections::BTreeMap<String, u32> = std::collections::BTreeMap::new();
        for w in tokenize(context) {
            *freq.entry(w).or_insert(0) += 1;
        }
        // Satz-Score = Summe der Term-Gewichte (deterministisch).
        let mut scored: Vec<(u32, usize, &str)> = sentences
            .iter()
            .enumerate()
            .map(|(i, s)| {
                let score: u32 = tokenize(s)
                    .iter()
                    .map(|w| freq.get(w).copied().unwrap_or(0))
                    .sum();
                (score, i, *s)
            })
            .collect();
        // stabile Ordnung: Score absteigend, bei Gleichstand nach Position.
        scored.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
        let top = scored.len().min(2);
        let picked: Vec<&str> = scored[..top].iter().map(|(_, _, s)| *s).collect();
        format!("ENTWURF (extraktiv, lokal): {}", picked.join(". "))
    }
}

/// Einfache, deterministische Tokenisierung ohne triviale Stoppwoerter.
fn tokenize(s: &str) -> Vec<String> {
    const STOP: &[&str] = &[
        "der", "die", "das", "und", "oder", "ein", "eine", "ist", "im", "in", "zu", "mit", "den",
        "dem", "auf", "fuer", "von", "the", "a", "an", "of", "to", "and", "is", "in",
    ];
    s.split(|c: char| !c.is_alphanumeric())
        .map(|w| w.to_lowercase())
        .filter(|w| w.len() > 2 && !STOP.contains(&w.as_str()))
        .collect()
}

impl ModelProvider for LocalExtractiveModel {
    fn manifest(&self) -> ModelManifest {
        let mut m = ModelManifest::complete(
            &format!("local-extractive:{}", self.model_name),
            ProviderClass::LocalModel,
            &self.model_name,
        );
        // recorded (Default): die Aufzeichnung ist die Replay-Wahrheit.
        m.replay_policy = ReplayPolicy::Recorded;
        m.modality = "text".to_string();
        m.supported_ops = vec![
            "draft".to_string(),
            "summarize".to_string(),
            "complete".to_string(),
        ];
        m
    }

    fn infer(&self, request: &InferenceRequest) -> InferenceResponse {
        // Modell verarbeitet NUR den (bereits gegateten) Kontext.
        let context: String = request
            .context
            .iter()
            .map(|s| s.content.as_str())
            .collect::<Vec<_>>()
            .join("\n");
        let content = self.draft(&context);
        let tokens = tokenize(&context).len() as u32;
        InferenceResponse {
            response_id: format!(
                "resp:local-extractive:{}:{}",
                self.model_name, request.request_id
            ),
            request_id: request.request_id.clone(),
            outcome: ResponseOutcome::Output(content),
            provider_metadata: format!("local-extractive:{} (kein Egress)", self.model_name),
            model_metadata: "extractive-tf-1.0 (regelbasiert, lokal)".to_string(),
            token_usage: tokens + 8,
            latency_ms: 1,
            trace: vec![format!("tf-ranking ueber {tokens} Terme")],
            replay_notes: "recorded: gleiche Eingabe ⇒ identische Ausgabe (deterministisch)"
                .to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{ContextSlice, InferenceRequest};

    fn req_with(context: &str) -> InferenceRequest {
        let mut r = InferenceRequest::example("extractive-test");
        r.context = vec![ContextSlice {
            name: "projektion".to_string(),
            content: context.to_string(),
        }];
        r
    }

    #[test]
    fn model_is_real_function_of_context_not_canned() {
        let m = LocalExtractiveModel::new("kernmodell");
        let a = m.infer(&req_with(
            "Serverausfall gefaehrdet den Betrieb. Redundanz senkt das Ausfallrisiko deutlich. Redundanz ist zentral.",
        ));
        let b = m.infer(&req_with(
            "Ein voellig anderer Text ueber Lieferketten und Puffer.",
        ));
        let (ta, tb) = (text(&a), text(&b));
        // Ausgabe haengt WIRKLICH vom Kontext ab (kein fester String).
        assert_ne!(ta, tb);
        // Das haeufigste Konzept ("Redundanz") wird extrahiert.
        assert!(ta.to_lowercase().contains("redundanz"), "extrahiert: {ta}");
    }

    #[test]
    fn deterministic_recorded_replay() {
        let m = LocalExtractiveModel::new("kernmodell");
        let ctx = "Naht traegt Garantie. Garantie ueberquert die Naht. Naht ist Separator.";
        let a = m.infer(&req_with(ctx));
        let b = m.infer(&req_with(ctx));
        assert_eq!(a.digest().to_hex(), b.digest().to_hex());
    }

    fn text(r: &InferenceResponse) -> String {
        match &r.outcome {
            ResponseOutcome::Output(s) => s.clone(),
            _ => panic!("kein Output"),
        }
    }
}
