//! CitationGate (I.3): die fail-closed Pruefkette je `cites`-Naht.
//! Fuenf Schritte, jeder benannt: (1) Ziel aufloesbar, (2) aufgeloester
//! Container Valid (L0–L2), (3) `core_root` byte-identisch, (4) falls
//! `target_unit_ref`: Einheit existiert im Ziel, (5) der
//! `supports`/`derives`-Teilgraph ueber Workbody-Grenzen ist azyklisch.
//!
//! Verdikt-Semantik (I.3): ein Container mit unaufgeloesten `cites`
//! bleibt TRANSPORTIERBAR — das ist Sache von `loom-verify` (hermetisch,
//! meldet nie mehr als `valid_with_residues` fuer fehlende Ziele, s.
//! `manifest_citation_mismatch`). CitationGate hier ist die
//! RESOLVER-ABHAENGIGE Pruefung fuer Closure/Commit:
//! `Close(x)=1 ⟺ CitationGate(x)=Pass` fuer JEDE `supports`/`derives`-Naht.

use crate::{hex34, parse_cites_from_cl_substrate, CitationResolver, CiteEntry};
use loom_format::KIND_CL_SUBSTRATE;
use loom_verify::Verdict;
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CitationOutcome {
    Ok,
    UnresolvedCitation,
    CitationTargetInvalid,
    CitationClassMismatch,
    CitationUnitMissing,
    CircularSupportCitation,
}

#[derive(Debug, Clone)]
pub struct CitationVerdict {
    pub entry: CiteEntry,
    pub outcome: CitationOutcome,
}

#[derive(Debug, Clone)]
pub struct CitationGateReport {
    pub entries: Vec<CitationVerdict>,
    /// `Close(x)=1 ⟺` ALLE `supports`/`derives`-Eintraege sind `Ok`.
    /// `refers`-Eintraege sind informativ und blockieren Closure nie.
    pub closure_pass: bool,
}

fn own_cites(bytes: &[u8]) -> Vec<CiteEntry> {
    let Ok(dec) = loom_codec::decode_sealed(bytes) else {
        return Vec::new();
    };
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

fn own_core_root_hex(bytes: &[u8]) -> Option<String> {
    let dec = loom_codec::decode_sealed(bytes).ok()?;
    Some(hex34(&loom_codec::recompute_core_root(&dec.segtab)))
}

/// DFS ueber `supports`/`derives`-Kanten, transitiv per Resolver
/// aufgeloest, mit Zyklus-Erkennung via Besuchsmenge (dieselbe
/// Kahn-/DFS-Grundidee wie `cce_phaseblock::hyperdag`, hier auf
/// `core_root`-Hex statt PhaseBlock-IDs).
fn reaches(
    start: &str,
    origin_root: &str,
    resolver: &dyn CitationResolver,
    visited: &mut BTreeSet<String>,
) -> bool {
    if start == origin_root {
        return true;
    }
    if !visited.insert(start.to_string()) {
        return false;
    }
    let Some(target) = resolver.resolve(start) else {
        return false;
    };
    for c in &target.cites {
        if c.cite_kind.is_closure_relevant()
            && reaches(&c.target_core_root_hex, origin_root, resolver, visited)
        {
            return true;
        }
    }
    false
}

pub fn citation_gate(bytes: &[u8], resolver: &dyn CitationResolver) -> CitationGateReport {
    let cites = own_cites(bytes);
    let origin_root = own_core_root_hex(bytes);

    let mut entries: Vec<CitationVerdict> = cites
        .into_iter()
        .map(|c| {
            let outcome = match resolver.resolve(&c.target_core_root_hex) {
                None => CitationOutcome::UnresolvedCitation,
                Some(vt) => {
                    if vt.core_root_hex != c.target_core_root_hex {
                        // Verteidigung in der Tiefe (X1a-Muster): dem
                        // Resolver nie blind vertrauen, immer gegenpruefen.
                        CitationOutcome::CitationClassMismatch
                    } else if !matches!(vt.verdict, Verdict::Valid | Verdict::ValidWithResidues) {
                        CitationOutcome::CitationTargetInvalid
                    } else if let Some(u) = &c.target_unit_ref {
                        if vt.unit_ids.contains(u) {
                            CitationOutcome::Ok
                        } else {
                            CitationOutcome::CitationUnitMissing
                        }
                    } else {
                        CitationOutcome::Ok
                    }
                }
            };
            CitationVerdict { entry: c, outcome }
        })
        .collect();

    // Schritt 5: Zyklus ueber Workbody-Grenzen — nur fuer supports/derives,
    // nur wenn der bisherige Schritt Ok war (ein bereits kaputtes Ziel
    // braucht keine Zyklenpruefung mehr).
    if let Some(origin) = &origin_root {
        for v in entries.iter_mut() {
            if v.outcome == CitationOutcome::Ok && v.entry.cite_kind.is_closure_relevant() {
                let mut visited = BTreeSet::new();
                if reaches(
                    &v.entry.target_core_root_hex,
                    origin,
                    resolver,
                    &mut visited,
                ) {
                    v.outcome = CitationOutcome::CircularSupportCitation;
                }
            }
        }
    }

    let closure_pass = entries
        .iter()
        .all(|v| !v.entry.cite_kind.is_closure_relevant() || v.outcome == CitationOutcome::Ok);

    CitationGateReport {
        entries,
        closure_pass,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CiteKind, VerifiedTarget};
    use std::collections::BTreeMap;

    /// Kontrollierter Test-Resolver: exakt vorgegebene Ziele, keine
    /// Dateisystem-Abhaengigkeit — deckt jeden der fuenf Gate-Schritte
    /// isoliert und deterministisch ab (N-CIT-1..4 auf Einheiten-Ebene;
    /// die realen Container-Gegenstuecke leben in den
    /// loom-conformance-Zeugen R-CIT-1..3).
    struct MockResolver {
        targets: BTreeMap<String, VerifiedTarget>,
    }

    impl CitationResolver for MockResolver {
        fn resolve(&self, root: &str) -> Option<VerifiedTarget> {
            self.targets.get(root).cloned()
        }
    }

    fn valid_target(root: &str, unit_ids: &[&str], cites: Vec<CiteEntry>) -> VerifiedTarget {
        VerifiedTarget {
            core_root_hex: root.to_string(),
            verdict: Verdict::Valid,
            manifest: loom_canon::Cv::map(vec![]),
            unit_ids: unit_ids.iter().map(|s| s.to_string()).collect(),
            cites,
        }
    }

    fn cite(
        unit_id: &str,
        target: &str,
        kind: CiteKind,
        target_unit_ref: Option<&str>,
    ) -> CiteEntry {
        CiteEntry {
            unit_id: unit_id.to_string(),
            target_core_root_hex: target.to_string(),
            target_unit_ref: target_unit_ref.map(String::from),
            cite_kind: kind,
        }
    }

    // Baut einen minimalen, echten .loom mit genau den gegebenen
    // cites-Eintraegen im CL_SUBSTRATE, damit citation_gate() etwas
    // Reales dekodieren kann (`own_cites`/`own_core_root_hex`).
    fn container_with_cites(cites: &[CiteEntry]) -> Vec<u8> {
        use loom_canon::Cv;
        use loom_codec::{seal_canonical, Segment};
        use loom_format::{KIND_CANON_DESC, KIND_CL_SUBSTRATE, KIND_MANIFEST};

        let manifest = Cv::map(vec![
            ("title", Cv::Text("gate-test".into())),
            ("container_class", Cv::Text("inspection".into())),
            ("domain_refs", Cv::Array(vec![])),
            ("scale", Cv::Uint(1)),
            ("pl_level", Cv::Text("PL0".into())),
            ("claims", Cv::map(vec![("closed", Cv::Bool(false))])),
            ("origin", Cv::map(vec![])),
            (
                "profiles_required",
                Cv::Array(vec![Cv::Text("inspection".into())]),
            ),
            ("profiles_optional", Cv::Array(vec![])),
            (
                "residue_summary",
                Cv::map(vec![("count", Cv::Uint(0)), ("kinds", Cv::Array(vec![]))]),
            ),
            ("capability_declarations", Cv::Array(vec![])),
            ("license_summary", Cv::Text("cc0".into())),
            (
                "created",
                Cv::Tag(0, Box::new(Cv::Text("2026-01-01T00:00:00Z".into()))),
            ),
        ]);
        let cl = Cv::map(vec![
            ("cubes", Cv::Array(vec![])),
            ("constraints", Cv::Array(vec![])),
            ("cites", crate::cites_field(cites)),
        ]);
        let canon_desc = Segment::canonical(KIND_CANON_DESC, &Cv::Text("rules".into())).unwrap();
        seal_canonical(
            "inspection",
            &["inspection"],
            &[
                Segment::canonical(KIND_MANIFEST, &manifest).unwrap(),
                canon_desc,
                Segment::canonical(KIND_CL_SUBSTRATE, &cl).unwrap(),
            ],
        )
        .unwrap()
        .bytes
    }

    #[test]
    fn unresolved_target_is_reported_without_reject() {
        let citing =
            container_with_cites(&[cite("a1", &"11".repeat(34), CiteKind::Supports, None)]);
        let resolver = MockResolver {
            targets: BTreeMap::new(),
        };
        let report = citation_gate(&citing, &resolver);
        assert_eq!(report.entries.len(), 1);
        assert_eq!(
            report.entries[0].outcome,
            CitationOutcome::UnresolvedCitation
        );
        assert!(
            !report.closure_pass,
            "N-CIT-1: unresolved blockiert Closure"
        );
    }

    #[test]
    fn invalid_target_is_citation_target_invalid() {
        let target_root = "22".repeat(34);
        let citing = container_with_cites(&[cite("a1", &target_root, CiteKind::Supports, None)]);
        let mut targets = BTreeMap::new();
        targets.insert(
            target_root.clone(),
            VerifiedTarget {
                core_root_hex: target_root.clone(),
                verdict: Verdict::Reject,
                manifest: loom_canon::Cv::map(vec![]),
                unit_ids: BTreeSet::new(),
                cites: vec![],
            },
        );
        let resolver = MockResolver { targets };
        let report = citation_gate(&citing, &resolver);
        assert_eq!(
            report.entries[0].outcome,
            CitationOutcome::CitationTargetInvalid
        );
        assert!(!report.closure_pass);
    }

    /// N-CIT-2: der Resolver "luegt" (liefert ein Ziel mit ANDEREM
    /// core_root als angefragt) — CitationGate darf ihm nicht blind
    /// vertrauen (Verteidigung in der Tiefe, dieselbe Disziplin wie
    /// `loom_mount::extract_artifact`).
    #[test]
    fn mismatched_resolved_root_is_citation_class_mismatch() {
        let requested_root = "33".repeat(34);
        let decoy_root = "44".repeat(34);
        let citing = container_with_cites(&[cite("a1", &requested_root, CiteKind::Supports, None)]);
        let mut targets = BTreeMap::new();
        targets.insert(
            requested_root.clone(),
            valid_target(&decoy_root, &[], vec![]),
        );
        let resolver = MockResolver { targets };
        let report = citation_gate(&citing, &resolver);
        assert_eq!(
            report.entries[0].outcome,
            CitationOutcome::CitationClassMismatch
        );
        assert!(!report.closure_pass);
    }

    #[test]
    fn missing_target_unit_ref_is_citation_unit_missing() {
        let target_root = "55".repeat(34);
        let citing =
            container_with_cites(&[cite("a1", &target_root, CiteKind::Supports, Some("d1"))]);
        let mut targets = BTreeMap::new();
        targets.insert(
            target_root.clone(),
            valid_target(&target_root, &["s1"], vec![]),
        );
        let resolver = MockResolver { targets };
        let report = citation_gate(&citing, &resolver);
        assert_eq!(
            report.entries[0].outcome,
            CitationOutcome::CitationUnitMissing
        );
        assert!(!report.closure_pass);
    }

    #[test]
    fn present_target_unit_ref_passes() {
        let target_root = "66".repeat(34);
        let citing =
            container_with_cites(&[cite("a1", &target_root, CiteKind::Supports, Some("d1"))]);
        let mut targets = BTreeMap::new();
        targets.insert(
            target_root.clone(),
            valid_target(&target_root, &["d1"], vec![]),
        );
        let resolver = MockResolver { targets };
        let report = citation_gate(&citing, &resolver);
        assert_eq!(report.entries[0].outcome, CitationOutcome::Ok);
        assert!(report.closure_pass);
    }

    /// N-CIT-3: eine zirkulaere supports-Kette ueber zwei fremde
    /// Container (B cites zurueck auf den ZITIERENDEN Container A) ⇒ rot.
    #[test]
    fn circular_support_chain_across_workbodies_is_rejected() {
        let root_b = "88".repeat(34);

        // A (dieser Container) zitiert (supports) B.
        let citing = container_with_cites(&[cite("a1", &root_b, CiteKind::Supports, None)]);
        // Der ECHTE core_root von `citing` (origin_root wird intern genau
        // so berechnet) — B muss GENAU darauf zurueckzitieren, damit die
        // Zyklenerkennung greift.
        let origin = loom_codec::decode_sealed(&citing).unwrap();
        let origin_hex = crate::hex34(&loom_codec::recompute_core_root(&origin.segtab));
        let b_cites = vec![cite("b1", &origin_hex, CiteKind::Supports, None)];

        let mut targets = BTreeMap::new();
        targets.insert(root_b.clone(), valid_target(&root_b, &[], b_cites));
        let resolver = MockResolver { targets };

        let report = citation_gate(&citing, &resolver);
        assert_eq!(
            report.entries[0].outcome,
            CitationOutcome::CircularSupportCitation
        );
        assert!(!report.closure_pass, "N-CIT-3: Zyklus blockiert Closure");
    }

    #[test]
    fn refers_never_blocks_closure_even_when_unresolved() {
        let citing = container_with_cites(&[cite("a1", &"99".repeat(34), CiteKind::Refers, None)]);
        let resolver = MockResolver {
            targets: BTreeMap::new(),
        };
        let report = citation_gate(&citing, &resolver);
        assert_eq!(
            report.entries[0].outcome,
            CitationOutcome::UnresolvedCitation
        );
        assert!(
            report.closure_pass,
            "refers ist informativ — blockiert Closure nie, auch unaufgeloest"
        );
    }
}
