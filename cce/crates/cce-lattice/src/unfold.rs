//! Entfaltungsgraph U (F-10, CL §4): der gewurzelte Junction Tree als
//! azyklische Materialisierungsordnung; Traversierungsplan deterministisch.

use crate::graph::JunctionTree;

/// Traversierungsplan: Cliquen-Indizes in deterministischer topologischer
/// Ordnung (Wurzel zuerst, Kinder lexikographisch nach Index).
pub fn traversal_plan(jt: &JunctionTree) -> Vec<usize> {
    let n = jt.cliques.len();
    let mut adj = vec![Vec::new(); n];
    for (i, j, _) in &jt.edges {
        adj[*i].push(*j);
        adj[*j].push(*i);
    }
    for a in &mut adj {
        a.sort_unstable();
    }
    let mut plan = Vec::new();
    let mut visited = vec![false; n];
    let mut stack = vec![jt.root];
    while let Some(x) = stack.pop() {
        if visited[x] {
            continue;
        }
        visited[x] = true;
        plan.push(x);
        for &y in adj[x].iter().rev() {
            if !visited[y] {
                stack.push(y);
            }
        }
    }
    // isolierte Cliquen anhaengen (deterministisch)
    for (i, seen) in visited.iter().enumerate() {
        if !seen {
            plan.push(i);
        }
    }
    plan
}
