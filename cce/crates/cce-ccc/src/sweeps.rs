//! Zwei-Sweep-Schema (CCC §14/§17, Satz 19.4): Collect (zentripetal, Bx)
//! und Distribute (zentrifugal, ClosedMarginal) teilen sich EINE
//! Junction-Tree-Struktur (P6/INV-7) — keine getrennten Subsysteme.

use crate::fiber::NodeFiber;
use cce_lattice::graph::JunctionTree;
use cce_lattice::propagation::{propagate_hull, Constraint};
use std::collections::BTreeSet;

/// Der Cluster-Baum: EINE Struktur fuer beide Sweeps.
#[derive(Debug, Clone)]
pub struct ClusterTree {
    pub jt: JunctionTree,
    pub fibers: Vec<NodeFiber>,
    pub constraints: Vec<Constraint>,
}

impl ClusterTree {
    /// Kanten als (parent, child, sep) relativ zur Wurzel, deterministisch.
    fn oriented_edges(&self) -> Vec<(usize, usize, BTreeSet<String>)> {
        let n = self.jt.cliques.len();
        let mut adj: Vec<Vec<(usize, BTreeSet<String>)>> = vec![Vec::new(); n];
        for (i, j, sep) in &self.jt.edges {
            adj[*i].push((*j, sep.clone()));
            adj[*j].push((*i, sep.clone()));
        }
        let mut out = Vec::new();
        let mut visited = vec![false; n];
        let mut stack = vec![self.jt.root];
        visited[self.jt.root] = true;
        while let Some(x) = stack.pop() {
            let mut kids = adj[x].clone();
            kids.sort_by_key(|(j, _)| *j);
            for (j, sep) in kids {
                if !visited[j] {
                    visited[j] = true;
                    out.push((x, j, sep));
                    stack.push(j);
                }
            }
        }
        out
    }

    fn local_constraints(&self, node: usize) -> Vec<Constraint> {
        let vars = &self.fibers[node].vars;
        self.constraints
            .iter()
            .filter(|c| match c {
                Constraint::AllowedValues { dim, .. } => vars.contains(dim),
                Constraint::AllowedPairs { a, b, .. } => vars.contains(a) && vars.contains(b),
            })
            .cloned()
            .collect()
    }
}

fn intersect_into(fiber: &mut NodeFiber, sep_state: &cce_lattice::propagation::DomainState) {
    for (k, allowed) in sep_state {
        if let Some(dom) = fiber.state.get_mut(k) {
            dom.retain(|v| allowed.contains(v));
        }
    }
}

/// Collect-Sweep (aufwickelnde Spule): Blaetter → Wurzel.
/// Jeder Knoten kollabiert lokal (Huelle) und reicht Separator-Marginale hoch.
pub fn collect_sweep(tree: &mut ClusterTree) {
    let edges = tree.oriented_edges();
    // Kinder vor Eltern: umgekehrte Orientierungsreihenfolge.
    for (parent, child, sep) in edges.iter().rev() {
        let lc = tree.local_constraints(*child);
        let hull = propagate_hull(tree.fibers[*child].state.clone(), &lc);
        tree.fibers[*child].state = hull.state;
        let msg = tree.fibers[*child].restrict(sep);
        intersect_into(&mut tree.fibers[*parent], &msg);
    }
    let rc = tree.local_constraints(tree.jt.root);
    let hull = propagate_hull(tree.fibers[tree.jt.root].state.clone(), &rc);
    tree.fibers[tree.jt.root].state = hull.state;
}

/// Distribute-Sweep (abwickelnde Spule): Wurzel → Blaetter.
/// Schliesst jede lokale Faser auf die global konsistente Marginale.
pub fn distribute_sweep(tree: &mut ClusterTree) {
    let edges = tree.oriented_edges();
    for (parent, child, sep) in &edges {
        let msg = tree.fibers[*parent].restrict(sep);
        intersect_into(&mut tree.fibers[*child], &msg);
        let lc = tree.local_constraints(*child);
        let hull = propagate_hull(tree.fibers[*child].state.clone(), &lc);
        tree.fibers[*child].state = hull.state;
    }
}

/// Der geschlossene Rundlauf: Collect ∘ Distribute auf EINER Struktur.
pub fn two_sweep(tree: &mut ClusterTree) {
    collect_sweep(tree);
    distribute_sweep(tree);
}

/// Nach beiden Sweeps: Separator-Konsistenz an jeder Kante (Kalibrierung).
pub fn is_calibrated(tree: &ClusterTree) -> bool {
    tree.jt.edges.iter().all(|(i, j, sep)| {
        crate::fiber::separator_consistent(&tree.fibers[*i], &tree.fibers[*j], sep)
    })
}
