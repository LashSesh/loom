//! cce-inference — L9c: das InferenceGateway (Overlay 05, Teil C).
//!
//! Egress-Vierteilung (Teil B, normativ):
//! > Kein Source-Fetch ausser CSA. Kein Modell-Egress ausser
//! > InferenceGateway. Kein Tool-Egress ausser ToolGateway. Kein Commit
//! > aus Source-, Model- oder Tool-Egress ohne Gate + Evidence + Replay
//! > + sichtbares Residuum.
//!
//! IG-A2: Modell-/Agentenausgaben sind CandidateOutputs — Eingaben fuer
//! Gates, nie Urteile/Commits/Ledger-Schreiber. IG-A4: Confidence ist
//! nie ein Gate. IG-A5: Deklaration ≠ Aktivierung.

pub mod contracts;
pub mod evidence;
pub mod gates;
pub mod gateway;
pub mod kanzel;
pub mod manifest;
pub mod providers;
pub mod replay;
pub mod request;
pub mod residues;
pub mod response;
