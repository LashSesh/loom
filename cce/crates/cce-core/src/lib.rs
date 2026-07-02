//! cce-core — GRUNDSTOCK: BCIK-Kern (Bauverfassung Teil 3/4/6.1).
//!
//! Traegt die kanonischen Objekte (CanonicalState, TripolarFiber, Gate,
//! GateReport, Residue, RunDescriptor, Ledger, Certificate, Artifact,
//! Wunsch-Normalform W) und die Fundamentaloperatoren
//! `canonicalize / signature / reflect / closed? / residue / gate / commit / replay`.
//!
//! Nicht verhandelbar (hier baulich erzwungen):
//! - kein Float in signaturrelevanten Strukturen (CanonValue kennt keinen Float),
//! - Gates boolesch, begruendet, fail-closed (V1/V7),
//! - Residuen stets sichtbar (P4/V2),
//! - Ledger append-only mit Hash-Kette (INV-12),
//! - kein Wall-Clock / keine ungeseedete Zufaelligkeit (P9) — es existiert
//!   schlicht kein Zeit-/Random-Pfad in diesem Crate.

pub mod adapter_parity;
pub mod bcik;
pub mod canonical;
pub mod closure;
pub mod gate;
pub mod hf_import;
pub mod ledger;
pub mod machine;
pub mod objects;
pub mod pathinv;
pub mod qsna;
pub mod reflection;
pub mod replay;
pub mod residue;
pub mod signature;
pub mod value;

pub use canonical::{CanonicalClass, CanonicalState, Canonicalize};
pub use closure::{is_closed, ClosureCertificate};
pub use gate::{Gate, GateChain, GateKind, GateReport, GateVerdict};
pub use ledger::{Ledger, LedgerEvent, LedgerEventKind};
pub use objects::{
    Artifact, BoundaryContract, Certificate, CounterHorizon, Evidence, Horizon, Marker, NullAnchor,
    Response, ScaleTarget, Seam, SeamDirection, Wish,
};
pub use replay::{HitlDecision, RunDescriptor};
pub use residue::{Residue, ResidueKind, ResidueStatus, Severity};
pub use signature::{sha256, ContentAddress, Digest};
pub use value::CanonValue;
