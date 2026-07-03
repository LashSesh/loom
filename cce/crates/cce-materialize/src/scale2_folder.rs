//! SCALE-2 „Dokumentenmappe" (P8, Track E): die erste höhere Skala über
//! SCALE-1. Zellen = D01-Workbodies (Memos); Nähte = Verweis-/
//! Reihenfolge-Konsistenz zwischen Memos; Materialisierung = Mappen-
//! Index (+ konzeptuell gebündelte .loom). STRUKTUR real; PL bleibt
//! sichtbar geführt (SCALE-2-Reife = S15-R1, review-/nutzungsgebunden).
//! KEINE Änderung an SCALE-1-Pfaden (der Wächter beweist es).

use crate::document::DocCrystal;
use cce_core::canonical::Canonicalize;
use cce_core::value::CanonValue;

/// Eine Mappe-Einheit: ein D01-Memo, referenziert über seine
/// Inhaltsklasse (Zwei-Digest-Denken auf Skala 2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FolderEntry {
    pub id: String,
    pub content_class: String,
}

/// Die Dokumentenmappe (SCALE-2-Kristall).
#[derive(Debug, Clone)]
pub struct DocFolder {
    pub title: String,
    pub entries: Vec<FolderEntry>,
    /// (from_id, kind, to_id) — kind ∈ {references, precedes}.
    pub seams: Vec<(String, String, String)>,
}

impl DocFolder {
    /// Baut die Mappe aus D01-Memos + Nähten (Verweis/Reihenfolge).
    pub fn from_memos(
        title: &str,
        memos: &[(&str, &DocCrystal)],
        seams: &[(&str, &str, &str)],
    ) -> Self {
        let entries = memos
            .iter()
            .map(|(id, m)| FolderEntry {
                id: (*id).to_string(),
                content_class: m.canonical_class().0.to_hex(),
            })
            .collect();
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
        self.entries.iter().any(|e| e.id == id)
    }
}

impl Canonicalize for DocFolder {
    fn canonical_value(&self) -> CanonValue {
        let mut entries = self.entries.clone();
        entries.sort_by(|a, b| a.id.cmp(&b.id));
        let mut seams = self.seams.clone();
        seams.sort();
        CanonValue::map([
            ("scale", CanonValue::Int(2)),
            ("title", CanonValue::text(&self.title)),
            (
                "entries",
                CanonValue::List(
                    entries
                        .iter()
                        .map(|e| {
                            CanonValue::map([
                                ("id", CanonValue::text(&e.id)),
                                ("content_class", CanonValue::text(&e.content_class)),
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

/// materialize: Mappen-Index als Markdown mit verlustfreien Ankern.
pub fn materialize_index(folder: &DocFolder) -> String {
    let mut out = format!("# Mappe: {}\n\n<!--cce:folder scale=2-->\n\n", folder.title);
    let mut entries = folder.entries.clone();
    entries.sort_by(|a, b| a.id.cmp(&b.id));
    for e in &entries {
        out.push_str(&format!("- {} · Klasse {}\n", e.id, e.content_class));
        out.push_str(&format!(
            "<!--cce:folder-entry id={};class={}-->\n",
            e.id, e.content_class
        ));
    }
    let mut seams = folder.seams.clone();
    seams.sort();
    for (f, k, t) in &seams {
        out.push_str(&format!("<!--cce:folder-seam {f};{k};{t}-->\n"));
    }
    out
}

/// reanalyze: Mappen-Index → DocFolder (verlustfrei auf Klassenebene).
pub fn parse_index(text: &str) -> Result<DocFolder, String> {
    let mut title = String::new();
    let mut entries = Vec::new();
    let mut seams = Vec::new();
    for line in text.lines() {
        let l = line.trim();
        if let Some(rest) = l.strip_prefix("# Mappe: ") {
            title = rest.to_string();
        } else if let Some(rest) = l.strip_prefix("<!--cce:folder-entry ") {
            let body = rest.trim_end_matches("-->");
            let mut id = String::new();
            let mut class = String::new();
            for kv in body.split(';') {
                if let Some(v) = kv.strip_prefix("id=") {
                    id = v.to_string();
                } else if let Some(v) = kv.strip_prefix("class=") {
                    class = v.to_string();
                }
            }
            entries.push(FolderEntry {
                id,
                content_class: class,
            });
        } else if let Some(rest) = l.strip_prefix("<!--cce:folder-seam ") {
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
        return Err("kein Mappen-Titel".to_string());
    }
    Ok(DocFolder {
        title,
        entries,
        seams,
    })
}

pub fn folder_equivalent(a: &DocFolder, b: &DocFolder) -> bool {
    a.canonical_class() == b.canonical_class()
}

/// Naht-Konsistenz der Mappe (Verweis-/Reihenfolge): jede Naht zeigt auf
/// existierende Einträge; der `precedes`-Graph ist azyklisch.
pub fn folder_seams_valid(folder: &DocFolder) -> bool {
    for (f, _k, t) in &folder.seams {
        if !folder.exists(f) || !folder.exists(t) {
            return false;
        }
    }
    !precedes_cycle(folder)
}

fn precedes_cycle(folder: &DocFolder) -> bool {
    use std::collections::{BTreeMap, BTreeSet};
    let mut adj: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for (f, k, t) in &folder.seams {
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
    use crate::document::assets::three_risks_memo;

    fn folder3() -> DocFolder {
        let m = three_risks_memo();
        // drei Memo-Workbodies (hier klassengleich; ids unterscheiden sie)
        DocFolder::from_memos(
            "Projektmappe Q1",
            &[("memo_a", &m), ("memo_b", &m), ("memo_c", &m)],
            &[
                ("memo_a", "precedes", "memo_b"),
                ("memo_b", "precedes", "memo_c"),
                ("memo_c", "references", "memo_a"),
            ],
        )
    }

    #[test]
    fn red2_kerntest_reanalyze_class_identical() {
        let f = folder3();
        assert_eq!(f.entries.len(), 3);
        assert!(folder_seams_valid(&f), "Mappe-Naehte konsistent");
        let index = materialize_index(&f);
        let back = parse_index(&index).expect("Reanalyse der Mappe");
        assert!(folder_equivalent(&back, &f), "Mappe: Reanalyze ≄ id");
    }

    #[test]
    fn folder_seam_gap_and_cycle_detected() {
        let m = three_risks_memo();
        // Naht auf nicht existierenden Eintrag
        let gap = DocFolder::from_memos("m", &[("a", &m)], &[("a", "references", "ghost")]);
        assert!(!folder_seams_valid(&gap));
        // Zyklus im precedes-Graph
        let cyc = DocFolder::from_memos(
            "m",
            &[("a", &m), ("b", &m)],
            &[("a", "precedes", "b"), ("b", "precedes", "a")],
        );
        assert!(!folder_seams_valid(&cyc));
    }
}
