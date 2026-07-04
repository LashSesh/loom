//! cce-bridge — S-E5: L9b Normic Memory (Ring E5, Etappe X3).
//!
//! Ein Gedaechtnis, das ausschliesslich aus Geschlossenem besteht:
//! wiederkehrende, bewaehrte Regelmaessigkeiten ueber viele zertifizierte
//! Arbeitskoerper werden zu Normen destilliert — herkunftsgebunden
//! (ProvenanceSet ueber den bestehenden CitationResolver-Port),
//! gate-promoviert (BridgeGate, sechs Stufen), revidierbar, sichtbar
//! erodierend, und in der Anwendung strikt opt-in (`norm_profile`).
//!
//! Kein Egress: "Bridge" ist die Bruecke zwischen Arbeiten und
//! Gedaechtnis, nicht zum Netz. Alle Abhaengigkeiten sind interne Ports
//! (cce-core, loom-cites, der .loom-Kern) — keine externe Kiste.

pub mod types;

mod cv_util;

pub mod provenance;

pub mod distill;

pub mod gate;

pub mod workbody;

pub mod memory;

pub mod activation;

pub mod lifecycle;

pub use types::{
    BridgeNorm, BridgeVerdict, CounterExample, NexusClass, NormCandidate, NormStatus,
    ProvenanceSet, Scope,
};
