//! Verklebung (CCC Lemma 12.2 / Satz 13.1): lokale Closure jeder Faser +
//! Separator-Konsistenz ⇒ globale Closure; die Verklebung ist EINDEUTIG.

use crate::fiber::{separator_consistent, NodeFiber};
use cce_lattice::propagation::DomainState;
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GlueError {
    /// Separator-Inkonsistenz — Verklebung verweigert (V4: Naht vor Sprung).
    SeamInconsistent { seam: Vec<String> },
    /// Eine Faser ist lokal nicht geschlossen.
    NotLocallyClosed { node: usize },
}

/// Verklebt eine Familie lokal geschlossener Fasern ueber ihre Separatoren
/// zu EINEM globalen Zustand. Deterministisch und eindeutig (Satz 13.1).
pub fn glue(
    fibers: &[NodeFiber],
    seams: &[(usize, usize, BTreeSet<String>)],
) -> Result<DomainState, GlueError> {
    for (i, f) in fibers.iter().enumerate() {
        if !f.locally_closed() {
            return Err(GlueError::NotLocallyClosed { node: i });
        }
    }
    for (i, j, sep) in seams {
        if !separator_consistent(&fibers[*i], &fibers[*j], sep) {
            return Err(GlueError::SeamInconsistent {
                seam: sep.iter().cloned().collect(),
            });
        }
    }
    let mut global = DomainState::new();
    for f in fibers {
        for (k, v) in &f.state {
            match global.get(k) {
                None => {
                    global.insert(k.clone(), v.clone());
                }
                Some(existing) => {
                    // Separator-Konsistenz garantiert Gleichheit auf Schnitten;
                    // Schnittbildung haelt die Verklebung eindeutig.
                    let merged: Vec<String> =
                        existing.iter().filter(|x| v.contains(x)).cloned().collect();
                    global.insert(k.clone(), merged);
                }
            }
        }
    }
    Ok(global)
}
