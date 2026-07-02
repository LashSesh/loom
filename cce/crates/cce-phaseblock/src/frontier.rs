//! Frontiers (Rebase §1.1): HDAG-Frontier und Ratchet-Frontier mit
//! SYNC-PFLICHT — `Sync(Frontier_HDAG, Frontier_Ratchet) = 1`;
//! `frontier_desync` ist BLOCKING (S6-A1/S3-A2).

use crate::hyperdag::HyperDag;
use crate::phaseblock::BlockStatus;
use cce_core::residue::{Residue, ResidueKind, Severity};
use std::collections::BTreeSet;

/// Eine Frontier: die Menge der aktuell "vordersten" akzeptierten Bloecke.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frontier {
    pub kind: String,
    pub block_ids: BTreeSet<String>,
}

/// HDAG-Frontier: akzeptierte Bloecke ohne akzeptierte Nachfolger.
pub fn hdag_frontier(h: &HyperDag) -> Frontier {
    let accepted: BTreeSet<&String> = h
        .blocks
        .iter()
        .filter(|(_, b)| b.status == BlockStatus::Accepted)
        .map(|(id, _)| id)
        .collect();
    let has_accepted_successor: BTreeSet<&String> = h
        .edges
        .iter()
        .filter(|(from, to, _)| accepted.contains(from) && accepted.contains(to))
        .map(|(from, _, _)| from)
        .collect();
    Frontier {
        kind: "hdag".to_string(),
        block_ids: accepted
            .into_iter()
            .filter(|id| !has_accepted_successor.contains(*id))
            .cloned()
            .collect(),
    }
}

#[derive(Debug, Clone)]
pub enum FrontierSync {
    InSync,
    /// Desync ist blocking — mit sichtbarem Residuum.
    Desync(Residue),
}

/// Sync-Pflicht: beide Frontiers muessen dieselbe Blockmenge sehen.
pub fn sync_frontiers(hdag: &Frontier, ratchet: &Frontier) -> FrontierSync {
    if hdag.block_ids == ratchet.block_ids {
        FrontierSync::InSync
    } else {
        let only_h: Vec<&String> = hdag.block_ids.difference(&ratchet.block_ids).collect();
        let only_r: Vec<&String> = ratchet.block_ids.difference(&hdag.block_ids).collect();
        FrontierSync::Desync(Residue::new(
            "frontier-desync",
            "frontier-sync",
            ResidueKind::named("frontier_desync"),
            Severity::Blocking,
            &format!("nur HDAG: {only_h:?}; nur Ratchet: {only_r:?}"),
        ))
    }
}
