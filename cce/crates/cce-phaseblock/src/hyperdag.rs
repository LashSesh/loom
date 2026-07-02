//! HyperDAG `H = (V, E_dep, E_seam, E_phase, E_scale, E_commit)`
//! (Rebase §1.1/§4.2 Formel, S15.5). Kantentypen gemaess der normativen
//! Formel; zur Zaehlweise „6 Kantentypen" siehe R-Agent-4 (residuen.md).

use crate::phaseblock::{BlockStatus, PhaseBlock};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EdgeKind {
    Dep,
    Seam,
    Phase,
    Scale,
    Commit,
}

#[derive(Debug, Default)]
pub struct HyperDag {
    pub blocks: BTreeMap<String, PhaseBlock>,
    /// (from, to, kind) — deterministisch geordnet.
    pub edges: Vec<(String, String, EdgeKind)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DagError {
    UnknownBlock(String),
    CycleDetected,
}

impl HyperDag {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert_block(&mut self, b: PhaseBlock) -> String {
        let id = b.id.clone();
        self.blocks.insert(id.clone(), b);
        id
    }

    pub fn add_edge(&mut self, from: &str, to: &str, kind: EdgeKind) -> Result<(), DagError> {
        if !self.blocks.contains_key(from) {
            return Err(DagError::UnknownBlock(from.to_string()));
        }
        if !self.blocks.contains_key(to) {
            return Err(DagError::UnknownBlock(to.to_string()));
        }
        self.edges.push((from.to_string(), to.to_string(), kind));
        if self.topological_order().is_err() {
            self.edges.pop();
            return Err(DagError::CycleDetected);
        }
        Ok(())
    }

    /// Deterministische topologische Ordnung ueber ALLE Kanten.
    pub fn topological_order(&self) -> Result<Vec<String>, DagError> {
        let mut indeg: BTreeMap<&str, usize> =
            self.blocks.keys().map(|k| (k.as_str(), 0)).collect();
        for (_, to, _) in &self.edges {
            *indeg.get_mut(to.as_str()).expect("known") += 1;
        }
        let mut order = Vec::new();
        let mut done: Vec<&str> = Vec::new();
        while order.len() < self.blocks.len() {
            let next = indeg
                .iter()
                .filter(|(id, deg)| **deg == 0 && !done.contains(*id))
                .map(|(id, _)| *id)
                .min();
            let Some(next) = next else {
                return Err(DagError::CycleDetected);
            };
            done.push(next);
            order.push(next.to_string());
            for (from, to, _) in &self.edges {
                if from == next {
                    *indeg.get_mut(to.as_str()).expect("known") -= 1;
                }
            }
        }
        Ok(order)
    }

    /// Akzeptierte Bloecke in topologischer Ordnung (Grundlage der
    /// Commit-Projektion).
    pub fn accepted_in_order(&self) -> Result<Vec<&PhaseBlock>, DagError> {
        Ok(self
            .topological_order()?
            .iter()
            .filter_map(|id| self.blocks.get(id))
            .filter(|b| b.status == BlockStatus::Accepted)
            .collect())
    }
}
