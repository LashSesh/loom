//! cce-ccc — GRUNDSTOCK: Crystalline Closure Calculus (F-05, F-10..F-15).
//! Die Klammer: Faser an jedem Knoten + chordales Skelett (Junction Tree,
//! aus cce-lattice — EINE Struktur, keine Duplikation) + ZWEI Sweeps
//! (Collect/Distribute) + Kristall-Protokoll + Bi-Temporalitaet.
//! Abnahmekatalog CCC C1–C14 in tests/c_catalog.rs.

pub mod bitemporal;
pub mod crystal_protocol;
pub mod fiber;
pub mod glue;
pub mod sweeps;

pub use crystal_protocol::{is_crystal, CrystalCandidate};
pub use sweeps::{collect_sweep, distribute_sweep, two_sweep, ClusterTree};
