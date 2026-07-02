//! cce-hbm — L7 HBM Mining-Chassis (REBASE §3, S15.8): chirurgisch
//! neutralisiert, VORGELAGERT zur Closure-Kette. Pipeline-Phasen 0–9:
//! Ingest → Facets → Gate_A → Cube/HDAG → Skeleton/JT → Kandidaten C1–C6 →
//! ExclusionGate → LayerExpansion → CrystalFinalization → Registry/Replay.
//! A7 ueberall: Score ordnet (RD-Gewichte), Gate entscheidet; θ_D ist
//! Vorauswahl, NIE Abnahme. Keine freie Agentenintelligenz
//! (ProjectionPacket-Vertrag); Safety-by-abstraction (HBM 13.2).
//! Klonung: implementiert, aber FAIL-CLOSED DEAKTIVIERT (R-13).

pub mod calibration;
pub mod candidate;
pub mod cells;
pub mod facet;
pub mod pipeline;
pub mod score;
pub mod specialization;

pub use candidate::BlueprintCandidate;
pub use facet::{gate_a, Facet};
pub use pipeline::{run_pipeline, MiningInput, PipelineOutcome};
pub use score::{rank, ScoreWeights};
