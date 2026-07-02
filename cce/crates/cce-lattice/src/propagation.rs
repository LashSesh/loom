//! Propagationshuelle Γ_𝒦 (F-08, CL §3.2): extensiv, monoton, idempotent —
//! Kollaps zum kleinsten Fixpunkt (Knaster–Tarski). Implementiert als
//! Bogenkonsistenz ueber endlichen Wertebereichen. Jeder entfernte Wert ist
//! ein SICHTBARES Residuum (CL K12) — nie stiller Verlust.

use cce_core::residue::{Residue, ResidueKind, Severity};
use std::collections::BTreeMap;

/// Constraint-Formen 𝒦.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Constraint {
    /// Unaer: Dimension darf nur diese Werte tragen.
    AllowedValues { dim: String, values: Vec<String> },
    /// Binaer: erlaubte Wertepaare (a-Wert, b-Wert).
    AllowedPairs {
        a: String,
        b: String,
        pairs: Vec<(String, String)>,
    },
}

/// Determinationszustand: je Dimension die verbliebene Wertemenge.
/// Informationsordnung: kleinere Mengen = mehr Information.
pub type DomainState = BTreeMap<String, Vec<String>>;

/// Ergebnis der Huelle: Fixpunkt + sichtbare Residuen der entfernten Werte.
#[derive(Debug, Clone)]
pub struct HullResult {
    pub state: DomainState,
    pub removed: Vec<Residue>,
    pub infeasible: bool,
}

/// Γ_𝒦: kleinster Fixpunkt der Konsistenzregeln.
/// Extensiv (in Informationsordnung), monoton, idempotent — als Tests belegt.
pub fn propagate_hull(mut state: DomainState, constraints: &[Constraint]) -> HullResult {
    let mut removed = Vec::new();
    let mut changed = true;
    while changed {
        changed = false;
        for c in constraints {
            match c {
                Constraint::AllowedValues { dim, values } => {
                    if let Some(dom) = state.get_mut(dim) {
                        let before = dom.len();
                        dom.retain(|v| {
                            let keep = values.contains(v);
                            if !keep {
                                removed.push(removed_residue(dim, v, "unary"));
                            }
                            keep
                        });
                        if dom.len() != before {
                            changed = true;
                        }
                    }
                }
                Constraint::AllowedPairs { a, b, pairs } => {
                    // a-Werte ohne Stuetze in b entfernen, und umgekehrt.
                    for (x, y) in [(a, b), (b, a)] {
                        let other: Vec<String> = state.get(y).cloned().unwrap_or_default();
                        if let Some(dom) = state.get_mut(x) {
                            let before = dom.len();
                            dom.retain(|v| {
                                let supported = other.iter().any(|w| {
                                    if x == a {
                                        pairs.iter().any(|(p, q)| p == v && q == w)
                                    } else {
                                        pairs.iter().any(|(p, q)| q == v && p == w)
                                    }
                                });
                                if !supported {
                                    removed.push(removed_residue(x, v, "arc"));
                                }
                                supported
                            });
                            if dom.len() != before {
                                changed = true;
                            }
                        }
                    }
                }
            }
        }
    }
    let infeasible = state.values().any(|d| d.is_empty());
    HullResult {
        state,
        removed,
        infeasible,
    }
}

fn removed_residue(dim: &str, value: &str, rule: &str) -> Residue {
    Residue::new(
        &format!("hull-removed:{dim}={value}"),
        &format!("propagation:{rule}"),
        ResidueKind::ExcludedMaterial,
        Severity::Info,
        &format!("Wert '{value}' aus Dimension '{dim}' propagiert (sichtbar, CL K12)"),
    )
}

/// Informationsordnung: `a ⊑ b` ⟺ b ist ueberall Teilmenge von a
/// (b traegt mindestens so viel Information).
pub fn refines(coarser: &DomainState, finer: &DomainState) -> bool {
    coarser.keys().all(|k| {
        let cd = &coarser[k];
        finer
            .get(k)
            .map(|fd| fd.iter().all(|v| cd.contains(v)))
            .unwrap_or(false)
    })
}
