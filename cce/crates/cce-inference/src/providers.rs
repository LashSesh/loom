//! Die fuenf Provider-Klassen (C.3). Provider-SDK-Code lebt NUR hier
//! (F.1 Verbote: „kein Provider-SDK ausserhalb providers/"). Im Bau
//! existieren ausschliesslich Mocks + der LocalModel-Echtpfad
//! (deterministischer Stub, replay_policy deklariert) — reale
//! Cloud-Anbieter anzubinden ist Betriebsschritt (F.3).

pub mod cloud_mock;
pub mod disabled;
pub mod embedded;
pub mod external_agent;
pub mod local;
pub mod local_extractive;
pub mod openai;

use crate::manifest::ModelManifest;
use crate::request::InferenceRequest;
use crate::response::InferenceResponse;

/// Der eine Provider-Port: nimmt einen (bereits voll gegateten)
/// Request, liefert eine Response. Provider haben KEINEN Zugriff auf
/// Gates, Urteile, Residuen oder Ledger (C.2) — die Signatur gibt
/// ihnen schlicht nichts davon in die Hand.
pub trait ModelProvider {
    fn manifest(&self) -> ModelManifest;
    fn infer(&self, request: &InferenceRequest) -> InferenceResponse;
}
