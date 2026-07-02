//! cce-phaseblock — L4 PhaseBlock-HyperDAG (Rebase §1.1/§4.2, S15.5):
//! PhaseBlock (10-Tupel) mit Accept-8-Kriterien, HyperDAG
//! `H = (V, E_dep, E_seam, E_phase, E_scale, E_commit)`, Frontiers mit
//! Sync-Pflicht, CrystalConsensus, `Ledger = CommitProjection(H)` +
//! `verify_hdag_projection`. Kandidat ≠ Commit: nichts wird Block ohne
//! Gate + Evidence + Replay + sichtbares Residuum (Driftverbot 6).

pub mod accept;
pub mod consensus;
pub mod frontier;
pub mod hyperdag;
pub mod phaseblock;
pub mod projection;

pub use accept::{accept_block, AcceptOutcome};
pub use consensus::crystal_consensus;
pub use frontier::{sync_frontiers, Frontier, FrontierSync};
pub use hyperdag::{EdgeKind, HyperDag};
pub use phaseblock::{BlockStatus, PhaseBlock};
pub use projection::{commit_projection, verify_hdag_projection, ProjectionError};
