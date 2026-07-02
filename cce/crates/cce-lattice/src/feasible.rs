//! Feasible Set ℱ / Kandidaten (F-09): Down-Set, monoton schrumpfend
//! (CL Satz 10.1: F_{t+1} ⊆ F_t). Enumeration deterministisch.

use crate::propagation::{Constraint, DomainState};
use std::collections::BTreeMap;

pub type Assignment = BTreeMap<String, String>;

fn satisfies(assign: &Assignment, c: &Constraint) -> bool {
    match c {
        Constraint::AllowedValues { dim, values } => {
            assign.get(dim).map(|v| values.contains(v)).unwrap_or(true)
        }
        Constraint::AllowedPairs { a, b, pairs } => match (assign.get(a), assign.get(b)) {
            (Some(x), Some(y)) => pairs.iter().any(|(p, q)| p == x && q == y),
            _ => true,
        },
    }
}

/// Enumeriert ℱ deterministisch (Dimensionen und Werte in fester Ordnung).
pub fn enumerate_feasible(state: &DomainState, constraints: &[Constraint]) -> Vec<Assignment> {
    let dims: Vec<&String> = state.keys().collect();
    let mut out = Vec::new();
    let mut current = Assignment::new();
    fn rec(
        dims: &[&String],
        idx: usize,
        state: &DomainState,
        constraints: &[Constraint],
        current: &mut Assignment,
        out: &mut Vec<Assignment>,
    ) {
        if idx == dims.len() {
            out.push(current.clone());
            return;
        }
        let d = dims[idx];
        for v in &state[d] {
            current.insert(d.clone(), v.clone());
            if constraints.iter().all(|c| satisfies(current, c)) {
                rec(dims, idx + 1, state, constraints, current, out);
            }
            current.remove(d);
        }
    }
    rec(&dims, 0, state, constraints, &mut current, &mut out);
    out
}
