//! SeedResolver (I.2): die erste Bau-Implementierung von
//! `CitationResolver` — durchsucht konfigurierte Verzeichnisse
//! (typischerweise `library/seed/`) nach `.loom`-Dateien und verifiziert
//! jeden Kandidaten L0–L2, BEVOR er akzeptiert wird (Digest-Vergleich
//! gegen den angefragten `target_core_root` ist Pflicht — kein blindes
//! Vertrauen auf Dateiname/Reihenfolge, dieselbe Disziplin wie
//! `loom_mount::extract_artifact`, X1a).

use crate::{hex34, parse_cites_from_cl_substrate, CitationResolver, VerifiedTarget};
use loom_canon::Cv;
use loom_format::{KIND_CL_SUBSTRATE, KIND_MANIFEST};
use std::collections::BTreeSet;
use std::path::PathBuf;

pub struct SeedResolver {
    pub search_dirs: Vec<PathBuf>,
}

impl SeedResolver {
    pub fn new(search_dirs: Vec<PathBuf>) -> Self {
        Self { search_dirs }
    }

    fn manifest_of(dec: &loom_codec::Decoded) -> Option<Cv> {
        dec.frames.iter().find_map(|(e, f)| {
            if e.kind == KIND_MANIFEST {
                loom_canon::decode(&f.payload).ok()
            } else {
                None
            }
        })
    }

    fn cites_of(dec: &loom_codec::Decoded) -> Vec<crate::CiteEntry> {
        let mut out = Vec::new();
        for (e, f) in &dec.frames {
            if e.kind == KIND_CL_SUBSTRATE {
                if let Ok(cl) = loom_canon::decode(&f.payload) {
                    out.extend(parse_cites_from_cl_substrate(&cl));
                }
            }
        }
        out
    }

    /// Bestmoegliche Rekonstruktion des Einheiten-Index: nutzt die in
    /// X1(a) eingefuehrte Artefakt-Extraktion + den echten
    /// Markdown-Reanalyse-Parser (cce-materialize). Scheitert das
    /// (kein Artefakt, kein Markdown) bleibt der Index leer — ein
    /// `target_unit_ref` gegen dieses Ziel wird dann korrekt als
    /// `citation_unit_missing` abgelehnt (fail-closed), statt zu raten.
    fn unit_ids_of(bytes: &[u8]) -> BTreeSet<String> {
        let Ok(handle) = loom_mount::open(bytes) else {
            return BTreeSet::new();
        };
        let Ok(artifact_bytes) = loom_mount::extract_artifact(&handle) else {
            return BTreeSet::new();
        };
        match cce_materialize::document::parse::parse_markdown(&artifact_bytes) {
            Ok(crystal) => crystal.units.into_iter().map(|u| u.id).collect(),
            Err(_) => BTreeSet::new(),
        }
    }
}

impl CitationResolver for SeedResolver {
    fn resolve(&self, target_core_root_hex: &str) -> Option<VerifiedTarget> {
        for dir in &self.search_dirs {
            let Ok(read_dir) = std::fs::read_dir(dir) else {
                continue;
            };
            for entry in read_dir.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) != Some("loom") {
                    continue;
                }
                let Ok(bytes) = std::fs::read(&path) else {
                    continue;
                };
                let Ok(dec) = loom_codec::decode_sealed(&bytes) else {
                    continue;
                };
                if hex34(&dec.footer.core_root) != target_core_root_hex {
                    continue;
                }
                // Digest-Vergleich (oben) ist Pflicht VOR jeder weiteren
                // Annahme — erst danach L0-L2 verifizieren.
                let report = loom_verify::verify(&bytes);
                let Some(manifest) = Self::manifest_of(&dec) else {
                    continue;
                };
                return Some(VerifiedTarget {
                    core_root_hex: target_core_root_hex.to_string(),
                    verdict: report.verdict,
                    manifest,
                    unit_ids: Self::unit_ids_of(&bytes),
                    cites: Self::cites_of(&dec),
                });
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_root_resolves_to_none() {
        let resolver = SeedResolver::new(vec![PathBuf::from(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../library/seed"
        ))]);
        assert!(resolver.resolve(&"ff".repeat(34)).is_none());
    }

    #[test]
    fn missing_search_dir_is_skipped_not_fatal() {
        let resolver = SeedResolver::new(vec![PathBuf::from("/nonexistent/does-not-exist")]);
        assert!(resolver.resolve(&"00".repeat(34)).is_none());
    }

    #[test]
    fn real_seed_file_resolves_valid_with_matching_root() {
        let dir = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../../library/seed"));
        let bytes = std::fs::read(dir.join("kristall_wikimedia_workbody.loom"))
            .expect("Seed-Datei muss vorhanden sein");
        let dec = loom_codec::decode_sealed(&bytes).expect("dekodieren");
        let root_hex = hex34(&dec.footer.core_root);

        let resolver = SeedResolver::new(vec![dir]);
        let target = resolver
            .resolve(&root_hex)
            .expect("das reale Seed-File muss auflösbar sein");
        assert_eq!(target.core_root_hex, root_hex);
        assert!(matches!(
            target.verdict,
            loom_verify::Verdict::Valid | loom_verify::Verdict::ValidWithResidues
        ));
        // Der echte Welt-Kristall traegt d1 (Definition) — vgl.
        // kristall_memo_from_wikimedia in loom-conformance.
        assert!(target.unit_ids.contains("d1"));
    }
}
