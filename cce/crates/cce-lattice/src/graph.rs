//! Kopplungsgraph, Chordalitaet (F-12), PEO, Triangulierung (min-fill),
//! Junction Tree mit Running-Intersection-Property (F-10/F-11, INV-6).
//! Adaequatheit = Chordalitaet (CCC Satz 19.2 / CL §5.2).

use std::collections::{BTreeMap, BTreeSet};

/// Ungerichteter Graph ueber String-Knoten, deterministisch geordnet.
#[derive(Debug, Clone, Default)]
pub struct Graph {
    pub adj: BTreeMap<String, BTreeSet<String>>,
}

impl Graph {
    pub fn new(nodes: &[&str]) -> Self {
        let mut g = Graph::default();
        for n in nodes {
            g.adj.entry(n.to_string()).or_default();
        }
        g
    }

    pub fn add_edge(&mut self, a: &str, b: &str) {
        if a == b {
            return;
        }
        self.adj
            .entry(a.to_string())
            .or_default()
            .insert(b.to_string());
        self.adj
            .entry(b.to_string())
            .or_default()
            .insert(a.to_string());
    }

    pub fn neighbors(&self, n: &str) -> BTreeSet<String> {
        self.adj.get(n).cloned().unwrap_or_default()
    }

    /// Maximum Cardinality Search: liefert eine Ordnung; der Graph ist
    /// chordal gdw. die Umkehrung eine PEO ist.
    pub fn mcs_order(&self) -> Vec<String> {
        let mut weight: BTreeMap<&String, usize> = self.adj.keys().map(|k| (k, 0usize)).collect();
        let mut order = Vec::new();
        let mut visited: BTreeSet<&String> = BTreeSet::new();
        while visited.len() < self.adj.len() {
            // deterministischer Tie-Break: lexikographisch kleinster Knoten
            let next = weight
                .iter()
                .filter(|(k, _)| !visited.contains(*k))
                .max_by(|(k1, w1), (k2, w2)| w1.cmp(w2).then(k2.cmp(k1)))
                .map(|(k, _)| (*k).clone())
                .expect("nonempty");
            visited.insert(self.adj.keys().find(|k| **k == next).expect("known"));
            for nb in self.neighbors(&next) {
                if let Some(w) = weight.get_mut(&nb) {
                    if !visited.iter().any(|v| **v == nb) {
                        *w += 1;
                    }
                }
            }
            order.push(next);
        }
        order
    }

    /// Prueft, ob `order` (rueckwaerts eliminiert) eine perfekte
    /// Eliminationsordnung ist.
    pub fn is_peo(&self, order: &[String]) -> bool {
        let pos: BTreeMap<&String, usize> = order.iter().enumerate().map(|(i, n)| (n, i)).collect();
        for (i, v) in order.iter().enumerate() {
            // spaetere Nachbarn muessen eine Clique bilden
            let later: Vec<&String> = self
                .neighbors(v)
                .iter()
                .filter(|n| pos.get(n).map(|p| *p > i).unwrap_or(false))
                .map(|n| order.iter().find(|o| *o == n).expect("known"))
                .collect();
            for (j, a) in later.iter().enumerate() {
                for b in &later[j + 1..] {
                    if !self.neighbors(a).contains(*b) {
                        return false;
                    }
                }
            }
        }
        true
    }

    /// Chordalitaet: MCS-Ordnung (umgekehrt) ist PEO gdw. chordal.
    pub fn is_chordal(&self) -> bool {
        let mut order = self.mcs_order();
        order.reverse();
        self.is_peo(&order)
    }

    /// Min-Fill-Triangulierung: fuellt Kanten, bis chordal; liefert Fill-in.
    pub fn triangulate(&self) -> (Graph, Vec<(String, String)>) {
        let mut g = self.clone();
        let mut fill = Vec::new();
        let mut remaining: BTreeSet<String> = g.adj.keys().cloned().collect();
        let mut work = g.clone();
        while !remaining.is_empty() {
            // Knoten mit minimalem Fill-in (deterministischer Tie-Break)
            let v = remaining
                .iter()
                .min_by_key(|v| {
                    let nbs: Vec<String> = work
                        .neighbors(v)
                        .intersection(&remaining)
                        .cloned()
                        .collect();
                    let mut cnt = 0usize;
                    for (i, a) in nbs.iter().enumerate() {
                        for b in &nbs[i + 1..] {
                            if !work.neighbors(a).contains(b) {
                                cnt += 1;
                            }
                        }
                    }
                    (cnt, (*v).clone())
                })
                .cloned()
                .expect("nonempty");
            let nbs: Vec<String> = work
                .neighbors(&v)
                .intersection(&remaining)
                .cloned()
                .collect();
            for (i, a) in nbs.iter().enumerate() {
                for b in &nbs[i + 1..] {
                    if !work.neighbors(a).contains(b) {
                        work.add_edge(a, b);
                        g.add_edge(a, b);
                        fill.push((a.clone(), b.clone()));
                    }
                }
            }
            remaining.remove(&v);
        }
        (g, fill)
    }

    /// Maximale Cliquen entlang einer PEO (nur fuer chordale Graphen korrekt).
    pub fn max_cliques(&self) -> Vec<BTreeSet<String>> {
        let mut order = self.mcs_order();
        order.reverse();
        let pos: BTreeMap<&String, usize> = order.iter().enumerate().map(|(i, n)| (n, i)).collect();
        let mut cliques: Vec<BTreeSet<String>> = Vec::new();
        for (i, v) in order.iter().enumerate() {
            let mut c: BTreeSet<String> = self
                .neighbors(v)
                .into_iter()
                .filter(|n| pos.get(n).map(|p| *p > i).unwrap_or(false))
                .collect();
            c.insert(v.clone());
            if !cliques.iter().any(|d| c.is_subset(d)) {
                cliques.retain(|d| !d.is_subset(&c));
                cliques.push(c);
            }
        }
        cliques
    }
}

/// Perfekte Eliminationsordnung.
pub type Peo = Vec<String>;

/// Junction Tree (F-10): Cliquenbaum mit RIP; Kanten tragen Separatoren.
#[derive(Debug, Clone)]
pub struct JunctionTree {
    pub cliques: Vec<BTreeSet<String>>,
    /// (i, j, Separator = C_i ∩ C_j)
    pub edges: Vec<(usize, usize, BTreeSet<String>)>,
    pub root: usize,
}

impl JunctionTree {
    /// Baut den Junction Tree eines chordalen Graphen
    /// (maximaler Spannbaum ueber Separatorgroessen, deterministisch).
    pub fn build(g: &Graph) -> Option<JunctionTree> {
        if !g.is_chordal() {
            return None;
        }
        let cliques = g.max_cliques();
        if cliques.is_empty() {
            return None;
        }
        // Kruskal auf Separator-Gewichten (max), deterministischer Tie-Break.
        let mut candidates = Vec::new();
        for i in 0..cliques.len() {
            for j in i + 1..cliques.len() {
                let sep: BTreeSet<String> = cliques[i].intersection(&cliques[j]).cloned().collect();
                candidates.push((i, j, sep));
            }
        }
        candidates.sort_by(|a, b| {
            b.2.len()
                .cmp(&a.2.len())
                .then(a.0.cmp(&b.0))
                .then(a.1.cmp(&b.1))
        });
        let mut parent: Vec<usize> = (0..cliques.len()).collect();
        fn find(p: &mut Vec<usize>, i: usize) -> usize {
            if p[i] != i {
                p[i] = find(p, p[i]);
            }
            p[i]
        }
        let mut edges = Vec::new();
        for (i, j, sep) in candidates {
            let (ri, rj) = (find(&mut parent, i), find(&mut parent, j));
            if ri != rj {
                parent[ri] = rj;
                edges.push((i, j, sep));
            }
        }
        Some(JunctionTree {
            cliques,
            edges,
            root: 0,
        })
    }

    /// Running-Intersection-Property (INV-6): fuer jedes Knotenpaar liegt
    /// der Schnitt auf dem Verbindungspfad in jedem Baum-Knoten.
    pub fn has_rip(&self) -> bool {
        let n = self.cliques.len();
        // Adjazenz des Baums
        let mut adj = vec![Vec::new(); n];
        for (i, j, _) in &self.edges {
            adj[*i].push(*j);
            adj[*j].push(*i);
        }
        // Pfad zwischen allen Paaren (klein: BFS)
        for a in 0..n {
            for b in a + 1..n {
                let inter: BTreeSet<String> = self.cliques[a]
                    .intersection(&self.cliques[b])
                    .cloned()
                    .collect();
                if inter.is_empty() {
                    continue;
                }
                // BFS-Pfad a→b
                let mut prev = vec![usize::MAX; n];
                let mut queue = std::collections::VecDeque::from([a]);
                prev[a] = a;
                while let Some(x) = queue.pop_front() {
                    for &y in &adj[x] {
                        if prev[y] == usize::MAX {
                            prev[y] = x;
                            queue.push_back(y);
                        }
                    }
                }
                if prev[b] == usize::MAX {
                    return false; // Baum nicht zusammenhaengend
                }
                let mut cur = b;
                while cur != a {
                    if !inter.is_subset(&self.cliques[cur]) {
                        return false;
                    }
                    cur = prev[cur];
                }
                if !inter.is_subset(&self.cliques[a]) {
                    return false;
                }
            }
        }
        true
    }

    /// Baumweite: groesste Clique − 1 (F-15).
    pub fn treewidth(&self) -> usize {
        self.cliques.iter().map(|c| c.len()).max().unwrap_or(1) - 1
    }
}
