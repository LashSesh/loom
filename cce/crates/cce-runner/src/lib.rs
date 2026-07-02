//! cce-runner — ADAPTER/Orchestrierung (Bauverfassung Phase F/H, S5+S5-A,
//! Rebase-Handoff §8.2): deterministische Ausfuehrung (a) des PhaseBlock-
//! HyperDAG, (b) der Multi-Ratchet-Kaskade, (c) generischer Pipeline-Laeufe
//! (RD-gebunden). Checkpoint = KANDIDAT (wiederaufnahmefaehig);
//! PhaseBlock = akzeptierter, irreversibler Abschluss (S5-A1).
//! HITL-Entscheidungen sind PhaseBlock-Inputs; Replay spielt sie ab (S5-A2).
//! Harte Gates pausieren NIE (S5.4).

pub mod checkpoint;
pub mod closure_report;
pub mod exec;
pub mod journey;
pub mod runner;

pub use runner::{DecisionProvider, Run, RunError, RunStatus, Stage};
