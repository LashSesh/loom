//! Chameleon-Projektion (F-07/G-14, CL §5): lokale, adaequate Sicht auf eine
//! Zelle — No-Horizon-Leakage: die Projektion enthaelt AUSSCHLIESSLICH die
//! Dimensionen der Zelle plus deklarierte Separator-Information, nie den
//! globalen Horizont.

use crate::propagation::{Constraint, DomainState};
use std::collections::BTreeSet;

/// Lokale Projektion π_c.
#[derive(Debug, Clone)]
pub struct Projection {
    pub cell_dims: Vec<String>,
    pub state: DomainState,
    pub constraints: Vec<Constraint>,
    pub separator_info: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectionError {
    /// Zelle referenziert unbekannte Dimension — fail-closed.
    UnknownDimension(String),
}

/// project_collect (Teil 4.3): Zustand ↓ Zelle.
pub fn project(
    global: &DomainState,
    constraints: &[Constraint],
    cell_dims: &[String],
    separator_info: &[String],
) -> Result<Projection, ProjectionError> {
    let cell: BTreeSet<&String> = cell_dims.iter().collect();
    for d in cell_dims {
        if !global.contains_key(d) {
            return Err(ProjectionError::UnknownDimension(d.clone()));
        }
    }
    let state: DomainState = global
        .iter()
        .filter(|(k, _)| cell.contains(k))
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    let local_constraints = constraints
        .iter()
        .filter(|c| match c {
            Constraint::AllowedValues { dim, .. } => cell.contains(dim),
            Constraint::AllowedPairs { a, b, .. } => cell.contains(a) && cell.contains(b),
        })
        .cloned()
        .collect();
    Ok(Projection {
        cell_dims: cell_dims.to_vec(),
        state,
        constraints: local_constraints,
        separator_info: separator_info.to_vec(),
    })
}

impl Projection {
    /// No-Horizon-Leakage-Nachweis: kein Zustandsschluessel ausserhalb der Zelle.
    pub fn leaks_horizon(&self) -> bool {
        self.state.keys().any(|k| !self.cell_dims.contains(k))
    }
}
