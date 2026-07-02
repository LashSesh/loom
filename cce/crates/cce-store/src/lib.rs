//! cce-store — Substanz: unveraenderlicher CAS + veraenderliche Refs
//! (S9 + S9-A). Adressiert Crystals, PhaseBlocks, MEF, Frontiers,
//! Replay-Packs, ResidueReports. Zwei-Speicher-Modell: Garantien im
//! unveraenderlichen Kern, Namen/Versionen in der Ref-Schicht;
//! Ref-Konflikte sind SICHTBAR und operator-aufgeloest (nie still, S9.6).
//! `verify_ledger` (cce-core) und `verify_hdag_projection` (cce-phaseblock)
//! laufen gegen die persistierten Objekte.

pub mod cas;
pub mod refs;

pub use cas::{Cas, FsCas, MemoryCas, ObjectKind};
pub use refs::{RefConflict, RefStore};

// Re-Exporte der Pruefer (S9-A1: ein Mechanismus, nicht drei).
pub use cce_core::ledger::verify_ledger;
pub use cce_phaseblock::projection::verify_hdag_projection;
