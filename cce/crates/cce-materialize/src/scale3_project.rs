//! SCALE-3 „Projektraum" (S-E2a I.6): der Verbund aus SCALE-2-Mappen,
//! Quellen-Workbodies (CSA-getragen) und Blueprint-Kristallen als EIN
//! Red(3)-Koerper. Zellen sind REFERENZEN auf bereits versiegelte
//! Container (`core_root`, wie bei `cites`) — anders als SCALE-2, wo die
//! Mappe unversiegelte `DocCrystal`s per `content_class` referenziert.
//! Nähte: `contains` (Mappe/Quelle gehoert zum Projekt — Quelle ist
//! IMMER das synthetische Projekt-Wurzelelement `"__project__"`, s.
//! `exists`), `cites` (I.1, Gate-Pass wird ausserhalb geprueft — das
//! braucht einen `CitationResolver`, den cce-materialize nicht kennen
//! darf, INV-11), `precedes` (Reihenfolge, azyklisch wie bei SCALE-2).
//! STRUKTUR real; die Schliessungs-PRUEFUNG (Resolver-abhaengig) lebt
//! im Aufrufer (loom-conformance/loom-cites), keine Aenderung an
//! SCALE-1/2-Pfaden.

use cce_core::canonical::Canonicalize;
use cce_core::value::CanonValue;

/// Der synthetische Wurzelknoten fuer `contains`-Nähte — das Projekt
/// selbst „enthaelt" seine Zellen; es gibt keinen zweiten Projekt-Eintrag
/// dafuer noetig.
pub const PROJECT_ROOT: &str = "__project__";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellKind {
    /// SCALE-2-Mappe.
    Folder,
    /// Quellen-Workbody (CSA-getragen).
    Source,
    /// Blueprint-Kristall (HBM).
    Blueprint,
}

impl CellKind {
    pub fn as_str(self) -> &'static str {
        match self {
            CellKind::Folder => "folder",
            CellKind::Source => "source",
            CellKind::Blueprint => "blueprint",
        }
    }

    pub fn parse(s: &str) -> Option<CellKind> {
        match s {
            "folder" => Some(CellKind::Folder),
            "source" => Some(CellKind::Source),
            "blueprint" => Some(CellKind::Blueprint),
            _ => None,
        }
    }
}

/// Eine Projekt-Zelle: Referenz auf einen bereits versiegelten Container.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectEntry {
    pub id: String,
    pub cell_kind: CellKind,
    pub core_root_hex: String,
}

/// Das SCALE-3-Projekt (Red(3)-Koerper).
#[derive(Debug, Clone)]
pub struct Scale3Project {
    pub title: String,
    pub entries: Vec<ProjectEntry>,
    /// (from_id, kind, to_id) — kind ∈ {contains, cites, precedes}.
    pub seams: Vec<(String, String, String)>,
}

impl Scale3Project {
    pub fn new(title: &str, entries: Vec<ProjectEntry>, seams: &[(&str, &str, &str)]) -> Self {
        Self {
            title: title.to_string(),
            entries,
            seams: seams
                .iter()
                .map(|(f, k, t)| ((*f).to_string(), (*k).to_string(), (*t).to_string()))
                .collect(),
        }
    }

    fn exists(&self, id: &str) -> bool {
        id == PROJECT_ROOT || self.entries.iter().any(|e| e.id == id)
    }
}

impl Canonicalize for Scale3Project {
    fn canonical_value(&self) -> CanonValue {
        let mut entries = self.entries.clone();
        entries.sort_by(|a, b| a.id.cmp(&b.id));
        let mut seams = self.seams.clone();
        seams.sort();
        CanonValue::map([
            ("scale", CanonValue::Int(3)),
            ("title", CanonValue::text(&self.title)),
            (
                "entries",
                CanonValue::List(
                    entries
                        .iter()
                        .map(|e| {
                            CanonValue::map([
                                ("id", CanonValue::text(&e.id)),
                                ("cell_kind", CanonValue::text(e.cell_kind.as_str())),
                                ("core_root", CanonValue::text(&e.core_root_hex)),
                            ])
                        })
                        .collect(),
                ),
            ),
            (
                "seams",
                CanonValue::List(
                    seams
                        .iter()
                        .map(|(f, k, t)| {
                            CanonValue::List(vec![
                                CanonValue::text(f),
                                CanonValue::text(k),
                                CanonValue::text(t),
                            ])
                        })
                        .collect(),
                ),
            ),
        ])
    }
}

/// materialize: Projekt-Index als Markdown mit verlustfreien Ankern
/// (dasselbe Muster wie `scale2_folder::materialize_index`).
pub fn materialize_index(p: &Scale3Project) -> String {
    let mut out = format!(
        "# Projektraum: {}\n\n<!--cce:project scale=3-->\n\n",
        p.title
    );
    let mut entries = p.entries.clone();
    entries.sort_by(|a, b| a.id.cmp(&b.id));
    for e in &entries {
        out.push_str(&format!(
            "- {} · {} · {}\n",
            e.id,
            e.cell_kind.as_str(),
            e.core_root_hex
        ));
        out.push_str(&format!(
            "<!--cce:project-entry id={};kind={};core_root={}-->\n",
            e.id,
            e.cell_kind.as_str(),
            e.core_root_hex
        ));
    }
    let mut seams = p.seams.clone();
    seams.sort();
    for (f, k, t) in &seams {
        out.push_str(&format!("<!--cce:project-seam {f};{k};{t}-->\n"));
    }
    out
}

/// reanalyze: Projekt-Index → Scale3Project (verlustfrei auf Klassenebene).
pub fn parse_index(text: &str) -> Result<Scale3Project, String> {
    let mut title = String::new();
    let mut entries = Vec::new();
    let mut seams = Vec::new();
    for line in text.lines() {
        let l = line.trim();
        if let Some(rest) = l.strip_prefix("# Projektraum: ") {
            title = rest.to_string();
        } else if let Some(rest) = l.strip_prefix("<!--cce:project-entry ") {
            let body = rest.trim_end_matches("-->");
            let mut id = String::new();
            let mut kind = String::new();
            let mut core_root = String::new();
            for kv in body.split(';') {
                if let Some(v) = kv.strip_prefix("id=") {
                    id = v.to_string();
                } else if let Some(v) = kv.strip_prefix("kind=") {
                    kind = v.to_string();
                } else if let Some(v) = kv.strip_prefix("core_root=") {
                    core_root = v.to_string();
                }
            }
            let cell_kind =
                CellKind::parse(&kind).ok_or_else(|| format!("unbekannte cell_kind: {kind}"))?;
            entries.push(ProjectEntry {
                id,
                cell_kind,
                core_root_hex: core_root,
            });
        } else if let Some(rest) = l.strip_prefix("<!--cce:project-seam ") {
            let body = rest.trim_end_matches("-->");
            let parts: Vec<&str> = body.split(';').collect();
            if parts.len() == 3 {
                seams.push((
                    parts[0].to_string(),
                    parts[1].to_string(),
                    parts[2].to_string(),
                ));
            }
        }
    }
    if title.is_empty() {
        return Err("kein Projekt-Titel".to_string());
    }
    Ok(Scale3Project {
        title,
        entries,
        seams,
    })
}

pub fn project_equivalent(a: &Scale3Project, b: &Scale3Project) -> bool {
    a.canonical_class() == b.canonical_class()
}

/// Naht-Konsistenz des Projekts: jede Naht zeigt auf existierende
/// Eintraege (oder den synthetischen `PROJECT_ROOT` fuer `contains`);
/// der `precedes`-Graph ist azyklisch (dieselbe Pruefung wie
/// `scale2_folder::folder_seams_valid`, hier ueber Projekt-Zellen).
pub fn project_seams_valid(p: &Scale3Project) -> bool {
    for (f, k, t) in &p.seams {
        if !["contains", "cites", "precedes"].contains(&k.as_str()) {
            return false;
        }
        if !p.exists(f) || !p.exists(t) {
            return false;
        }
    }
    !precedes_cycle(p)
}

fn precedes_cycle(p: &Scale3Project) -> bool {
    use std::collections::{BTreeMap, BTreeSet};
    let mut adj: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for (f, k, t) in &p.seams {
        if k == "precedes" {
            adj.entry(f.as_str()).or_default().push(t.as_str());
        }
    }
    let mut color: BTreeMap<&str, u8> = BTreeMap::new();
    fn dfs<'a>(
        n: &'a str,
        adj: &BTreeMap<&'a str, Vec<&'a str>>,
        color: &mut BTreeMap<&'a str, u8>,
    ) -> bool {
        color.insert(n, 1);
        if let Some(ns) = adj.get(n) {
            for &m in ns {
                match color.get(m).copied().unwrap_or(0) {
                    1 => return true,
                    0 => {
                        if dfs(m, adj, color) {
                            return true;
                        }
                    }
                    _ => {}
                }
            }
        }
        color.insert(n, 2);
        false
    }
    let nodes: BTreeSet<&str> = adj.keys().copied().collect();
    for n in nodes {
        if color.get(n).copied().unwrap_or(0) == 0 && dfs(n, &adj, &mut color) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn project() -> Scale3Project {
        Scale3Project::new(
            "Referenzprojekt",
            vec![
                ProjectEntry {
                    id: "mappe_a".to_string(),
                    cell_kind: CellKind::Folder,
                    core_root_hex: "aa".repeat(34),
                },
                ProjectEntry {
                    id: "mappe_b".to_string(),
                    cell_kind: CellKind::Folder,
                    core_root_hex: "bb".repeat(34),
                },
                ProjectEntry {
                    id: "quelle".to_string(),
                    cell_kind: CellKind::Source,
                    core_root_hex: "cc".repeat(34),
                },
                ProjectEntry {
                    id: "blueprint".to_string(),
                    cell_kind: CellKind::Blueprint,
                    core_root_hex: "dd".repeat(34),
                },
            ],
            &[
                (PROJECT_ROOT, "contains", "mappe_a"),
                (PROJECT_ROOT, "contains", "mappe_b"),
                (PROJECT_ROOT, "contains", "quelle"),
                (PROJECT_ROOT, "contains", "blueprint"),
                ("mappe_a", "precedes", "mappe_b"),
                ("mappe_b", "cites", "quelle"),
            ],
        )
    }

    #[test]
    fn red3_kerntest_reanalyze_class_identical() {
        let p = project();
        assert!(project_seams_valid(&p));
        let index = materialize_index(&p);
        let back = parse_index(&index).expect("Reanalyse des Projekts");
        assert!(project_equivalent(&back, &p), "Projekt: Reanalyze ≄ id");
    }

    #[test]
    fn seam_gap_and_precedes_cycle_are_detected() {
        let m = project();
        let mut gap = m.clone();
        gap.seams.push((
            "mappe_a".to_string(),
            "cites".to_string(),
            "ghost".to_string(),
        ));
        assert!(!project_seams_valid(&gap));

        let mut cyc = m.clone();
        cyc.seams.push((
            "mappe_b".to_string(),
            "precedes".to_string(),
            "mappe_a".to_string(),
        ));
        assert!(
            !project_seams_valid(&cyc),
            "mappe_a->mappe_b->mappe_a ist ein Zyklus"
        );
    }

    #[test]
    fn unknown_seam_kind_is_rejected() {
        let mut p = project();
        p.seams.push((
            "mappe_a".to_string(),
            "unknown".to_string(),
            "mappe_b".to_string(),
        ));
        assert!(!project_seams_valid(&p));
    }
}
