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
//! (cce-core, loom-cites, der .loom-Kern, cce-hbm als optionale
//! Pattern-Quelle) — keine externe Kiste.
//!
//! Dokument 16 §2: `pattern` ist eine TYPISIERTE Form (`pattern::
//! Pattern`), keine freie Zeichenkette mehr — die Fundament-Schutz-
//! Whitelist (§4) ist damit eine Konstruktions-, keine Texteigenschaft
//! (`extract::blueprint_to_pattern` liefert sie aus echten HBM-
//! Blueprint-Facetten, nie geraten).

pub mod types;

pub mod pattern;

pub mod extract;

mod cv_util;

pub mod provenance;

pub mod distill;

pub mod gate;

pub mod workbody;

pub mod memory;

pub mod activation;

pub mod lifecycle;

pub use pattern::{DomainRuleForm, Pattern};
pub use types::{
    BridgeNorm, BridgeVerdict, CounterExample, NexusClass, NormCandidate, NormStatus,
    ProvenanceSet, Scope,
};
