//! Fibriertes Closure-System (CCC §3/§12): an jedem Skelettknoten eine
//! tripolare Faser mit lokalem Closure-Gesetz; Restriktionen ρ_ij auf
//! Separatoren.

use cce_lattice::propagation::DomainState;
use std::collections::BTreeSet;

/// Lokale Faser eines Cliquen-Knotens: Zustandsausschnitt + Residualmarker.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeFiber {
    pub vars: BTreeSet<String>,
    pub state: DomainState,
}

impl NodeFiber {
    pub fn new(vars: BTreeSet<String>, state: DomainState) -> Self {
        Self { vars, state }
    }

    /// Restriktion ρ auf eine Separator-Menge (F-11).
    pub fn restrict(&self, sep: &BTreeSet<String>) -> DomainState {
        self.state
            .iter()
            .filter(|(k, _)| sep.contains(*k))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    /// Lokale Closure: keine leere Wertemenge (B⁻ leer auf diesem Knoten).
    pub fn locally_closed(&self) -> bool {
        !self.state.is_empty() && self.state.values().all(|d| !d.is_empty())
    }
}

/// Separator-Konsistenz zweier Fasern (Verklebungs-Vorbedingung).
pub fn separator_consistent(a: &NodeFiber, b: &NodeFiber, sep: &BTreeSet<String>) -> bool {
    a.restrict(sep) == b.restrict(sep)
}
