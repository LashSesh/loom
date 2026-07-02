//! cce-materialize — ADAPTER: Export-/Materialisierungsprofile (Materialize)
//! sowie der typstarke DomainAdapter-Vertrag (S1.8, 11 Punkte) und die
//! Dokument-Referenzimplementierung (S1, erste geschlossene Domaene).
//! Materialisierung NUR nach Gate/Evidence/Trace/Replay; reobserve-Pflicht.

pub mod adapter;
pub mod catalog;
pub mod document;

pub use adapter::{check_adapter_parity_typed, DomainAdapter, OpenAction};
pub use document::{DocCrystal, DocUnit, DocumentAdapter, UnitType};
