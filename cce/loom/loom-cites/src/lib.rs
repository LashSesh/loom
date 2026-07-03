//! loom-cites — S-E2a Teil I: die `cites`-Naht (Cross-Workbody-Referenz).
//!
//! Eine Einheit eines Workbody stuetzt sich beweisbar auf einen ANDEREN
//! zertifizierten Container — adressiert ueber dessen Inhaltsklasse
//! (`core_root`, ein Multihash), nie ueber Pfade. `cites`-Eintraege leben
//! als Seam-Daten im CL_SUBSTRATE (Feld `"cites"`, additiv neben den
//! bestehenden `"cubes"`/`"constraints"`-Feldern); das MANIFEST traegt
//! zusaetzlich `external_citations[]` (I.4) — deren Konsistenz prueft
//! bereits `loom-verify` hermetisch (`manifest_citation_mismatch`,
//! N-CIT-5). Die eigentliche AUFLOESUNG externer Ziele (I.2/I.3) braucht
//! Dateisystemzugriff und lebt bewusst HIER, nicht in loom-verify (das
//! bleibt motorfrei-hermetisch, LOOM Teil 6).

use loom_canon::Cv;
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CiteKind {
    /// Stuetzungsanspruch — closure-relevant, Replay-Input (I.1).
    Supports,
    /// Bezug — informativ, nicht closure-relevant.
    Refers,
    /// Ableitung aus der Zieleinheit — closure-relevant, Replay-Input.
    Derives,
}

impl CiteKind {
    pub fn as_str(self) -> &'static str {
        match self {
            CiteKind::Supports => "supports",
            CiteKind::Refers => "refers",
            CiteKind::Derives => "derives",
        }
    }

    pub fn parse(s: &str) -> Option<CiteKind> {
        match s {
            "supports" => Some(CiteKind::Supports),
            "refers" => Some(CiteKind::Refers),
            "derives" => Some(CiteKind::Derives),
            _ => None,
        }
    }

    /// I.3: der `supports`/`derives`-Teilgraph ist closure-relevant und
    /// muss azyklisch sein; `refers` darf frei zyklisch sein (kein
    /// Stuetzungszirkel).
    pub fn is_closure_relevant(self) -> bool {
        matches!(self, CiteKind::Supports | CiteKind::Derives)
    }
}

/// Ein `cites`-Anker: `cites(unit_id -> target)` (I.1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CiteEntry {
    pub unit_id: String,
    /// Multihash-Hexdarstellung des Ziel-`core_root` (34 Byte -> 68 Hex-Zeichen).
    pub target_core_root_hex: String,
    pub target_unit_ref: Option<String>,
    pub cite_kind: CiteKind,
}

impl CiteEntry {
    pub fn to_cv(&self) -> Cv {
        Cv::map(vec![
            ("unit_id", Cv::Text(self.unit_id.clone())),
            (
                "target_core_root",
                Cv::Text(self.target_core_root_hex.clone()),
            ),
            (
                "target_unit_ref",
                match &self.target_unit_ref {
                    Some(u) => Cv::Text(u.clone()),
                    None => Cv::Null,
                },
            ),
            ("cite_kind", Cv::Text(self.cite_kind.as_str().to_string())),
        ])
    }

    fn from_cv(v: &Cv) -> Option<CiteEntry> {
        let Cv::Map(fields) = v else { return None };
        let get = |k: &str| {
            fields.iter().find_map(|(key, val)| match key {
                Cv::Text(s) if s == k => Some(val.clone()),
                _ => None,
            })
        };
        let unit_id = match get("unit_id") {
            Some(Cv::Text(s)) => s,
            _ => return None,
        };
        let target_core_root_hex = match get("target_core_root") {
            Some(Cv::Text(s)) => s,
            _ => return None,
        };
        let target_unit_ref = match get("target_unit_ref") {
            Some(Cv::Text(s)) => Some(s),
            _ => None,
        };
        let cite_kind = match get("cite_kind") {
            Some(Cv::Text(s)) => CiteKind::parse(&s)?,
            _ => return None,
        };
        Some(CiteEntry {
            unit_id,
            target_core_root_hex,
            target_unit_ref,
            cite_kind,
        })
    }
}

/// Das `"cites"`-Feld eines CL_SUBSTRATE-Segments — additiv neben
/// `"cubes"`/`"constraints"` in den bestehenden CL_SUBSTRATE-Cv einzufuegen.
pub fn cites_field(cites: &[CiteEntry]) -> Cv {
    Cv::Array(cites.iter().map(CiteEntry::to_cv).collect())
}

/// Liest die `cites`-Eintraege aus einem bereits dekodierten CL_SUBSTRATE-Cv
/// (fehlt das Feld, ist das Ergebnis leer — additiv/read-kompatibel).
pub fn parse_cites_from_cl_substrate(cl_substrate: &Cv) -> Vec<CiteEntry> {
    let Cv::Map(fields) = cl_substrate else {
        return Vec::new();
    };
    for (k, v) in fields {
        if matches!(k, Cv::Text(s) if s == "cites") {
            if let Cv::Array(items) = v {
                return items.iter().filter_map(CiteEntry::from_cv).collect();
            }
        }
    }
    Vec::new()
}

/// I.4: die deduplizierte, sortierte Liste aller `target_core_root`s —
/// exakt das, was `manifest_declare_external_citations` ins MANIFEST
/// schreiben muss, damit `loom-verify`s `manifest_citation_mismatch`
/// gruen bleibt.
pub fn external_citations_hex(cites: &[CiteEntry]) -> Vec<String> {
    let mut roots: Vec<String> = cites
        .iter()
        .map(|c| c.target_core_root_hex.clone())
        .collect();
    roots.sort();
    roots.dedup();
    roots
}

pub fn hex34(root: &[u8; 34]) -> String {
    root.iter().map(|b| format!("{b:02x}")).collect()
}

/// Ein aufgeloestes Zitat-Ziel (I.2): Verdikt, Manifest-Kern,
/// Einheiten-Index (fuer `target_unit_ref`-Pruefung) UND die eigenen
/// ausgehenden `cites` (fuer die Zyklenerkennung ueber Workbody-Grenzen,
/// I.3 Punkt 5).
#[derive(Debug, Clone)]
pub struct VerifiedTarget {
    pub core_root_hex: String,
    pub verdict: loom_verify::Verdict,
    pub manifest: Cv,
    pub unit_ids: BTreeSet<String>,
    pub cites: Vec<CiteEntry>,
}

/// I.2: entkoppelt E2 von E4 — die Klassen-Registry (E4c) wird spaeter
/// eine ZWEITE Implementierung desselben Ports.
pub trait CitationResolver {
    fn resolve(&self, target_core_root_hex: &str) -> Option<VerifiedTarget>;
}

mod resolver;
pub use resolver::SeedResolver;

mod gate;
pub use gate::{citation_gate, CitationGateReport, CitationOutcome, CitationVerdict};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cite_entry_roundtrips_through_cv() {
        let e = CiteEntry {
            unit_id: "a1".to_string(),
            target_core_root_hex: "ab".repeat(34),
            target_unit_ref: Some("d1".to_string()),
            cite_kind: CiteKind::Supports,
        };
        let back = CiteEntry::from_cv(&e.to_cv()).expect("roundtrip");
        assert_eq!(back, e);
    }

    #[test]
    fn cite_entry_without_unit_ref_roundtrips_as_none() {
        let e = CiteEntry {
            unit_id: "a1".to_string(),
            target_core_root_hex: "cd".repeat(34),
            target_unit_ref: None,
            cite_kind: CiteKind::Refers,
        };
        let back = CiteEntry::from_cv(&e.to_cv()).expect("roundtrip");
        assert_eq!(back.target_unit_ref, None);
    }

    #[test]
    fn cl_substrate_without_cites_field_parses_as_empty() {
        let cl = Cv::map(vec![("cubes", Cv::Array(vec![]))]);
        assert!(parse_cites_from_cl_substrate(&cl).is_empty());
    }

    #[test]
    fn external_citations_are_deduped_and_sorted() {
        let cites = vec![
            CiteEntry {
                unit_id: "a1".to_string(),
                target_core_root_hex: "bb".repeat(34),
                target_unit_ref: None,
                cite_kind: CiteKind::Supports,
            },
            CiteEntry {
                unit_id: "a2".to_string(),
                target_core_root_hex: "aa".repeat(34),
                target_unit_ref: None,
                cite_kind: CiteKind::Refers,
            },
            CiteEntry {
                unit_id: "a3".to_string(),
                target_core_root_hex: "bb".repeat(34),
                target_unit_ref: Some("x".to_string()),
                cite_kind: CiteKind::Derives,
            },
        ];
        assert_eq!(
            external_citations_hex(&cites),
            vec!["aa".repeat(34), "bb".repeat(34)]
        );
    }

    #[test]
    fn closure_relevance_matches_spec() {
        assert!(CiteKind::Supports.is_closure_relevant());
        assert!(CiteKind::Derives.is_closure_relevant());
        assert!(!CiteKind::Refers.is_closure_relevant());
    }
}
