//! cce-loom — GENERATIV: Generative Inversion (G-01..G-08), der
//! Distribute-Sweep. Radiale Spindel mit Nullanker Z0 (markiert, NIE
//! durchlaufen — P7/V3), Mandorla-Regularisierung, Weave/Workcell,
//! Nadelapertur = Radfenster (EIN Typ aus cce-core, Identitaet I-7),
//! `loom_generate`-Referenzalgorithmus mit repair_or_reweave.
//! LOOM-Abnahmekatalog (10 Tests) in tests/loom_catalog.rs.

pub mod distribute;
pub mod radial_spindle;
pub mod weave;
pub mod workcell;

pub use distribute::{loom_generate, LoomOutcome, WorkcellRuntime};
pub use radial_spindle::RadialSpindle;
pub use weave::{Weave, WeaveBlock};
pub use workcell::{LoomWorkcell, WorkcellStatus};
