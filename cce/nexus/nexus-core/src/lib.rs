//! nexus-core — CSA-Objektmodell (CSA.2/CSA.3): SourceHorizon HS, TaskSpec,
//! CandidateSource, RawObservation, CSU, EvidencePack, NexusSourceBundle,
//! Source-RunDescriptor, Vier-Wege-Verdikt, die 18 Residuen (CSA.8).
//! `disallowed_actions` sind RD-VERBINDLICH und hart verdrahtet
//! (PROD-INV-14): keine Umgehung von Auth, Captchas, Paywalls, Bot-Schutz,
//! Rate-Limits, Terms.

pub mod objects;
pub mod residues;
pub mod verdict;

pub use objects::{
    CandidateSource, Csu, EvidencePack, NexusSourceBundle, RawObservation, SourceHorizon,
    SourceRunDescriptor, TaskSpec, DISALLOWED_ACTIONS,
};
pub use residues::{csa_residue, ALL_CSA_RESIDUES};
pub use verdict::Verdict;
