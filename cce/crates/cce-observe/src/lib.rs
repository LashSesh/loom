//! cce-observe — BEOBACHTEND (Collect/Reanalyze): O-01..O-18 als
//! Rollen-Importe, NICHT als getrennte Subsysteme (Teil 9 Phase G).
//! Collect-Normalform: Obs = MatrixCrystal ∘ Gate ∘ Triangulate ∘ Horizon
//! ∘ Respond ∘ Mark ∘ Embed. Teilt das Radfenster (cce-core::WheelWindow)
//! mit cce-loom (Identitaet I-7) und die Skelett-Struktur mit cce-ccc.
//! TAT-Abnahme P1–P7 in tests/tat_catalog.rs.

pub mod optics;
pub mod qlogic;
pub mod reanalyze;
pub mod substrate;
pub mod tat;

pub use reanalyze::observe;
pub use tat::{collect, CollectInput};
