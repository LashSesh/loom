//! AssemblyGraph AG = (N, E, Σ, Γ, O, Q, R) ↦ PhaseBlockHDAG (Rebase §1.1):
//! die Abbildung des Montagegraphen auf die PhaseBlock-Ordnung — nach unten
//! typisiert, keine eigene Theorie.

use crate::scheduler::TaskNode;

/// Montagegraph (reduzierte pruefbare Form).
#[derive(Debug, Clone)]
pub struct AssemblyGraph {
    pub nodes: Vec<TaskNode>,
    pub state_tag: String,
}

/// Abbildungsergebnis: geordnete PhaseBlock-Kandidaten-Kennungen
/// (die eigentlichen Bloecke entstehen in cce-phaseblock unter Accept-8).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockPlan {
    pub ordered_candidates: Vec<String>,
}

pub fn map_to_block_plan(
    ag: &AssemblyGraph,
    registry: &crate::scheduler::OperatorRegistry,
) -> Result<BlockPlan, crate::scheduler::ScheduleError> {
    Ok(BlockPlan {
        ordered_candidates: crate::scheduler::schedule(&ag.nodes, registry)?,
    })
}
