//! Klassen-Registry (S-E4a E4c, Karte §2/E4c): ein Verzeichnis
//! zertifizierter `core_root`s als eigener `.loom`-Katalog-Workbody
//! (Einträge: `core_root`, Klasse, Domäne, Signaturen, Pfad) —
//! Grundlage für E2-`cites` und jedes spätere Teilen. `RegistryResolver`
//! ist die ZWEITE Implementierung des `CitationResolver`-Ports (I.2):
//! sie loest per Katalog-Eintrag auf statt per Verzeichnis-Scan
//! (`SeedResolver`), verifiziert aber genauso streng (Digest-Gegenprobe
//! + volles L0-L2) — dem Katalog wird nie blind vertraut.

use crate::{hex34, CitationResolver, VerifiedTarget};
use loom_canon::Cv;
use std::path::{Path, PathBuf};

/// Ein Katalog-Eintrag: die deklarierte Beschreibung eines zertifizierten
/// Containers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassRegistryEntry {
    pub core_root_hex: String,
    pub class: String,
    pub domain: String,
    pub signatures: Vec<String>,
    pub path: String,
}

impl ClassRegistryEntry {
    fn to_cv(&self) -> Cv {
        Cv::map(vec![
            ("core_root", Cv::Text(self.core_root_hex.clone())),
            ("class", Cv::Text(self.class.clone())),
            ("domain", Cv::Text(self.domain.clone())),
            (
                "signatures",
                Cv::Array(
                    self.signatures
                        .iter()
                        .map(|s| Cv::Text(s.clone()))
                        .collect(),
                ),
            ),
            ("path", Cv::Text(self.path.clone())),
        ])
    }

    fn from_cv(v: &Cv) -> Option<ClassRegistryEntry> {
        let Cv::Map(fields) = v else { return None };
        let get = |k: &str| {
            fields.iter().find_map(|(key, val)| match key {
                Cv::Text(s) if s == k => Some(val.clone()),
                _ => None,
            })
        };
        let core_root_hex = match get("core_root") {
            Some(Cv::Text(s)) => s,
            _ => return None,
        };
        let class = match get("class") {
            Some(Cv::Text(s)) => s,
            _ => return None,
        };
        let domain = match get("domain") {
            Some(Cv::Text(s)) => s,
            _ => return None,
        };
        let signatures = match get("signatures") {
            Some(Cv::Array(items)) => items
                .iter()
                .filter_map(|v| match v {
                    Cv::Text(s) => Some(s.clone()),
                    _ => None,
                })
                .collect(),
            _ => Vec::new(),
        };
        let path = match get("path") {
            Some(Cv::Text(s)) => s,
            _ => return None,
        };
        Some(ClassRegistryEntry {
            core_root_hex,
            class,
            domain,
            signatures,
            path,
        })
    }
}

/// Das `KIND_DOC`-Payload des Registry-Workbody (additiv — dasselbe
/// generische Kind wie `folder_meta`/`doc_meta` andernorts, kein neues
/// Segment-Kind noetig).
pub fn class_registry_field(entries: &[ClassRegistryEntry]) -> Cv {
    Cv::map(vec![
        ("kind", Cv::Text("class_registry".to_string())),
        (
            "entries",
            Cv::Array(entries.iter().map(ClassRegistryEntry::to_cv).collect()),
        ),
    ])
}

/// Liest den Katalog aus einem dekodierten `KIND_DOC`-Cv zurueck.
/// `None`, wenn es kein Klassen-Registry-Dokument ist (additiv/
/// read-kompatibel — ein generisches `KIND_DOC` anderer Herkunft wird
/// nicht faelschlich als Registry gelesen).
pub fn parse_class_registry(doc_cv: &Cv) -> Option<Vec<ClassRegistryEntry>> {
    let Cv::Map(fields) = doc_cv else { return None };
    let is_registry = fields.iter().any(|(k, v)| {
        matches!(k, Cv::Text(s) if s == "kind") && matches!(v, Cv::Text(s) if s == "class_registry")
    });
    if !is_registry {
        return None;
    }
    let entries_cv = fields.iter().find_map(|(k, v)| match k {
        Cv::Text(s) if s == "entries" => Some(v.clone()),
        _ => None,
    })?;
    let Cv::Array(items) = entries_cv else {
        return None;
    };
    items.iter().map(ClassRegistryEntry::from_cv).collect()
}

/// Die zweite `CitationResolver`-Implementierung (I.2): loest per
/// Katalog-Eintrag auf (nicht per Verzeichnis-Scan), verifiziert aber
/// genauso streng — der Katalog liefert nur den PFAD-Hinweis, niemals
/// eine ungeprüfte Behauptung.
pub struct RegistryResolver {
    pub entries: Vec<ClassRegistryEntry>,
    pub base_dir: PathBuf,
}

impl RegistryResolver {
    pub fn new(entries: Vec<ClassRegistryEntry>, base_dir: impl AsRef<Path>) -> Self {
        Self {
            entries,
            base_dir: base_dir.as_ref().to_path_buf(),
        }
    }

    /// Baut einen Resolver DIREKT aus einem bereits geoeffneten
    /// Registry-Workbody (liest sein eigenes `KIND_DOC` aus).
    pub fn from_registry_bytes(bytes: &[u8], base_dir: impl AsRef<Path>) -> Option<Self> {
        let dec = loom_codec::decode_sealed(bytes).ok()?;
        for (e, f) in &dec.frames {
            if e.kind == loom_format::KIND_DOC {
                if let Ok(cv) = loom_canon::decode(&f.payload) {
                    if let Some(entries) = parse_class_registry(&cv) {
                        return Some(Self::new(entries, base_dir));
                    }
                }
            }
        }
        None
    }
}

impl CitationResolver for RegistryResolver {
    fn resolve(&self, target_core_root_hex: &str) -> Option<VerifiedTarget> {
        let entry = self
            .entries
            .iter()
            .find(|e| e.core_root_hex == target_core_root_hex)?;
        let bytes = std::fs::read(self.base_dir.join(&entry.path)).ok()?;
        let dec = loom_codec::decode_sealed(&bytes).ok()?;
        // Digest-Gegenprobe ist Pflicht — der Katalog-Pfad ist nur ein
        // Hinweis, niemals eine ungeprüfte Behauptung.
        if hex34(&dec.footer.core_root) != target_core_root_hex {
            return None;
        }
        let report = loom_verify::verify(&bytes);
        let manifest = dec.frames.iter().find_map(|(e, f)| {
            if e.kind == loom_format::KIND_MANIFEST {
                loom_canon::decode(&f.payload).ok()
            } else {
                None
            }
        })?;
        let cites = {
            let mut out = Vec::new();
            for (e, f) in &dec.frames {
                if e.kind == loom_format::KIND_CL_SUBSTRATE {
                    if let Ok(cl) = loom_canon::decode(&f.payload) {
                        out.extend(crate::parse_cites_from_cl_substrate(&cl));
                    }
                }
            }
            out
        };
        let unit_ids = {
            let handle = loom_mount::open(&bytes).ok()?;
            match loom_mount::extract_artifact(&handle) {
                Ok(artifact_bytes) => {
                    match cce_materialize::document::parse::parse_markdown(&artifact_bytes) {
                        Ok(crystal) => crystal.units.into_iter().map(|u| u.id).collect(),
                        Err(_) => std::collections::BTreeSet::new(),
                    }
                }
                Err(_) => std::collections::BTreeSet::new(),
            }
        };
        Some(VerifiedTarget {
            core_root_hex: target_core_root_hex.to_string(),
            verdict: report.verdict,
            manifest,
            unit_ids,
            cites,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_registry_field_roundtrips() {
        let entries = vec![ClassRegistryEntry {
            core_root_hex: "ab".repeat(34),
            class: "full".to_string(),
            domain: "dom:document".to_string(),
            signatures: vec!["author".to_string()],
            path: "library/seed/x.loom".to_string(),
        }];
        let cv = class_registry_field(&entries);
        let back = parse_class_registry(&cv).expect("Registry lesbar");
        assert_eq!(back, entries);
    }

    #[test]
    fn parse_class_registry_rejects_unrelated_doc_cv() {
        let unrelated = Cv::map(vec![("title", Cv::Text("etwas anderes".into()))]);
        assert_eq!(parse_class_registry(&unrelated), None);
    }

    #[test]
    fn registry_resolver_resolves_a_real_seeded_container() {
        let dir = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../../library/seed"));
        let bytes = std::fs::read(dir.join("kristall_wikimedia_workbody.loom"))
            .expect("Seed-Datei muss vorhanden sein");
        let dec = loom_codec::decode_sealed(&bytes).expect("dekodieren");
        let root_hex = hex34(&dec.footer.core_root);

        let entries = vec![ClassRegistryEntry {
            core_root_hex: root_hex.clone(),
            class: "full".to_string(),
            domain: "dom:document".to_string(),
            signatures: vec![],
            path: "kristall_wikimedia_workbody.loom".to_string(),
        }];
        let resolver = RegistryResolver::new(entries, dir);
        let target = resolver
            .resolve(&root_hex)
            .expect("per Katalog-Eintrag aufloesbar");
        assert_eq!(target.core_root_hex, root_hex);
        assert!(matches!(
            target.verdict,
            loom_verify::Verdict::Valid | loom_verify::Verdict::ValidWithResidues
        ));
    }

    #[test]
    fn registry_resolver_returns_none_for_unknown_root() {
        let dir = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../../library/seed"));
        let resolver = RegistryResolver::new(vec![], dir);
        assert!(resolver.resolve(&"ff".repeat(34)).is_none());
    }

    /// Verteidigung in der Tiefe: ein Katalog-Eintrag, dessen `path` auf
    /// eine ANDERE (aber real existierende) Datei zeigt, deren
    /// tatsaechlicher core_root NICHT zum behaupteten passt, darf nicht
    /// als aufgeloest durchgehen.
    #[test]
    fn registry_resolver_rejects_path_pointing_at_wrong_container() {
        let dir = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../../library/seed"));
        let claimed_root = "ff".repeat(34);
        let entries = vec![ClassRegistryEntry {
            core_root_hex: claimed_root.clone(),
            class: "full".to_string(),
            domain: "dom:document".to_string(),
            signatures: vec![],
            path: "kristall_wikimedia_workbody.loom".to_string(),
        }];
        let resolver = RegistryResolver::new(entries, dir);
        assert!(resolver.resolve(&claimed_root).is_none());
    }
}
