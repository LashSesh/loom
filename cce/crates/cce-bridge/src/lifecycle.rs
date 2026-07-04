//! S-E5 §6/§10(h): Erosion, Widerruf, Lineage.
//!
//! Erosion (automatisch, sichtbar): ein invalid/widerrufenes/
//! quarantaenisiertes ProvenanceSet-Mitglied laesst die Norm bei der
//! naechsten Registry-Pflege auf `deprecated` fallen — nie stiller
//! Fortbestand. Widerruf (gegateter Lauf): `revoke()` erzeugt einen
//! eigenen Widerrufs-Workbody, der die Norm + Gegenbelege zitiert.
//! Klassenstabilitaet: ein RD, der eine spaeter widerrufene Norm
//! aktiviert hatte, haelt seine damalige Klasse — nur die REANALYSE
//! zeigt `norm_since_revoked`.

use crate::provenance::check_member;
use crate::types::{BridgeNorm, CounterExample, NormStatus};
use loom_canon::Cv;
use loom_cites::{CitationResolver, CiteEntry, CiteKind};
use loom_codec::{seal_canonical, PackError, Sealed, Segment};
use loom_format::{KIND_CANON_DESC, KIND_CL_SUBSTRATE, KIND_MANIFEST, KIND_RESIDUE};

fn seg(kind: u16, v: &Cv) -> Segment {
    Segment::canonical(kind, v).expect("kanonisches Segment")
}

fn canon_desc_segment() -> Segment {
    Segment {
        kind: KIND_CANON_DESC,
        seg_flags: 0,
        payload: Cv::Text(loom_canon::CANON_RULES_TEXT.into())
            .encode()
            .unwrap(),
        deps: vec![],
    }
}

/// R-NRM-4/§6: prueft, ob ein ProvenanceSet-Mitglied inzwischen
/// ineligibel geworden ist (invalid/widerrufen/quarantaenisiert/nicht
/// mehr geschlossen). `None`, wenn keine Erosion vorliegt oder die Norm
/// ohnehin nicht mehr `Active` ist (Erosion greift nur einmal, aus
/// `Active` heraus — ein bereits `Deprecated`/`Revoked` Zustand wird
/// nicht ueberschrieben).
pub fn check_erosion(norm: &BridgeNorm, resolver: &dyn CitationResolver) -> Option<BridgeNorm> {
    if norm.status != NormStatus::Active {
        return None;
    }
    let eroded = norm
        .provenance_set
        .members
        .iter()
        .any(|root| !check_member(root, resolver).eligible);
    if !eroded {
        return None;
    }
    Some(BridgeNorm {
        status: NormStatus::Deprecated,
        ..norm.clone()
    })
}

/// §6: die Beleglage eines Widerrufs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RevocationRecord {
    pub norm_core_root_hex: String,
    pub reason: String,
    pub counter_evidence: Vec<CounterExample>,
}

fn revocation_manifest_cv(rec: &RevocationRecord) -> Cv {
    let mut external_citations: Vec<String> = rec
        .counter_evidence
        .iter()
        .map(|ce| ce.core_root_hex.clone())
        .chain(std::iter::once(rec.norm_core_root_hex.clone()))
        .collect();
    external_citations.sort();
    external_citations.dedup();
    Cv::map(vec![
        (
            "title",
            Cv::Text(format!("Widerruf der Norm {}", rec.norm_core_root_hex)),
        ),
        ("container_class", Cv::Text("inspection".into())),
        ("domain_refs", Cv::Array(vec![])),
        ("scale", Cv::Uint(1)),
        ("pl_level", Cv::Text("PL1".into())),
        // Kein Abschlussbeweis-LEDGER hier (reines Inspect-Protokoll) —
        // dieselbe Disziplin wie R1/R6: informativ, nicht "geschlossen"
        // im Bauverfassungssinn (das waere ein claim_over_evidence-Reject).
        ("claims", Cv::map(vec![("closed", Cv::Bool(false))])),
        (
            "origin",
            Cv::map(vec![("tool", Cv::Text("cce-bridge-0.1".into()))]),
        ),
        (
            "profiles_required",
            Cv::Array(vec![Cv::Text("inspection".into())]),
        ),
        ("profiles_optional", Cv::Array(vec![])),
        (
            "residue_summary",
            Cv::map(vec![
                ("count", Cv::Uint(1)),
                ("kinds", Cv::Array(vec![Cv::Text("norm_revoked".into())])),
            ]),
        ),
        (
            "capability_declarations",
            Cv::Array(vec![Cv::Text("read_segment".into())]),
        ),
        ("license_summary", Cv::Text("cc0".into())),
        (
            "created",
            Cv::Tag(0, Box::new(Cv::Text("2026-01-01T00:00:00Z".into()))),
        ),
        (
            "external_citations",
            Cv::Array(external_citations.into_iter().map(Cv::Text).collect()),
        ),
    ])
}

fn revocation_cl_cv(rec: &RevocationRecord) -> Cv {
    let mut cites = vec![CiteEntry {
        unit_id: "revocation".to_string(),
        target_core_root_hex: rec.norm_core_root_hex.clone(),
        target_unit_ref: None,
        // Refers, nicht supports/derives: ein Widerruf begruendet sich
        // NICHT auf die fortdauernde Gueltigkeit der Norm — informativ,
        // nie closure-relevant (§6 verlangt keine Naht-Semantik hier).
        cite_kind: CiteKind::Refers,
    }];
    cites.extend(
        rec.counter_evidence
            .iter()
            .enumerate()
            .map(|(i, ce)| CiteEntry {
                unit_id: format!("counter-evidence-{i}"),
                target_core_root_hex: ce.core_root_hex.clone(),
                target_unit_ref: None,
                cite_kind: CiteKind::Refers,
            }),
    );
    Cv::map(vec![
        ("cubes", Cv::Array(vec![])),
        ("constraints", Cv::Array(vec![])),
        ("cites", loom_cites::cites_field(&cites)),
        ("revocation_reason", Cv::Text(rec.reason.clone())),
    ])
}

fn revocation_residue_cv(rec: &RevocationRecord) -> Cv {
    Cv::map(vec![(
        "residues",
        Cv::Array(vec![Cv::map(vec![
            ("kind", Cv::Text("norm_revoked".into())),
            ("detail", Cv::Text(rec.reason.clone())),
        ])]),
    )])
}

/// Siegelt den Widerrufs-Workbody (zitiert die Norm + Gegenbelege,
/// §6). Rueckgabe zusaetzlich die Registry-seitige `Revoked`-Kopie der
/// Norm, damit der Aufrufer beides in einem Schritt anwenden kann.
pub fn revoke(
    norm: &BridgeNorm,
    norm_core_root_hex: &str,
    reason: &str,
    counter_evidence: Vec<CounterExample>,
) -> Result<(BridgeNorm, Sealed), PackError> {
    let record = RevocationRecord {
        norm_core_root_hex: norm_core_root_hex.to_string(),
        reason: reason.to_string(),
        counter_evidence,
    };
    let manifest = revocation_manifest_cv(&record);
    let cl = revocation_cl_cv(&record);
    let residues = revocation_residue_cv(&record);
    let sealed = seal_canonical(
        "inspection",
        &["inspection"],
        &[
            seg(KIND_MANIFEST, &manifest),
            canon_desc_segment(),
            seg(KIND_CL_SUBSTRATE, &cl),
            seg(KIND_RESIDUE, &residues),
        ],
    )?;
    let revoked_norm = BridgeNorm {
        status: NormStatus::Revoked,
        ..norm.clone()
    };
    Ok((revoked_norm, sealed))
}

/// §6: eine Ersatznorm fuehrt `supersedes` auf die Vorgaengerin.
pub fn supersede(mut new_norm: BridgeNorm, old_norm_id: String) -> BridgeNorm {
    new_norm.supersedes = Some(old_norm_id);
    new_norm
}

/// §6: die Lineage-Kette rueckwaerts ab `norm_id` (neueste zuerst).
pub fn lineage_chain<'a>(norms: &'a [BridgeNorm], norm_id: &str) -> Vec<&'a BridgeNorm> {
    let mut chain = Vec::new();
    let mut current = norms.iter().find(|n| n.norm_id == norm_id);
    while let Some(n) = current {
        chain.push(n);
        current = n
            .supersedes
            .as_ref()
            .and_then(|prev_id| norms.iter().find(|m| &m.norm_id == prev_id));
    }
    chain
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{NexusClass, ProvenanceSet, Scope};
    use loom_canon::Cv as CvT;
    use loom_cites::VerifiedTarget;
    use loom_verify::Verdict;
    use std::collections::BTreeMap;

    struct MockResolver {
        targets: BTreeMap<String, VerifiedTarget>,
    }

    impl CitationResolver for MockResolver {
        fn resolve(&self, root: &str) -> Option<VerifiedTarget> {
            self.targets.get(root).cloned()
        }
    }

    fn closed_target(root: &str) -> VerifiedTarget {
        VerifiedTarget {
            core_root_hex: root.to_string(),
            verdict: Verdict::Valid,
            manifest: CvT::map(vec![(
                "claims",
                CvT::map(vec![("closed", CvT::Bool(true))]),
            )]),
            unit_ids: Default::default(),
            cites: vec![],
        }
    }

    fn base_norm() -> BridgeNorm {
        BridgeNorm {
            norm_id: "norm:1".to_string(),
            nexus_class: NexusClass::StructuralRule,
            pattern: "p".to_string(),
            provenance_set: ProvenanceSet::new(vec!["a1".repeat(34), "a2".repeat(34)]),
            known_counterexamples: vec![],
            scope: Scope::Global,
            status: NormStatus::Active,
            promotion_evidence: "e".to_string(),
            supersedes: None,
        }
    }

    /// R-NRM-4: ein invalid gewordenes Mitglied loest automatische
    /// Erosion aus.
    #[test]
    fn eroded_member_deprecates_the_norm() {
        let norm = base_norm();
        let mut targets = BTreeMap::new();
        targets.insert(
            norm.provenance_set.members[0].clone(),
            closed_target(&norm.provenance_set.members[0]),
        );
        // members[1] bleibt unaufloesbar -> Erosion.
        let resolver = MockResolver { targets };
        let deprecated = check_erosion(&norm, &resolver).expect("Erosion erkannt");
        assert_eq!(deprecated.status, NormStatus::Deprecated);
    }

    #[test]
    fn intact_provenance_has_no_erosion() {
        let norm = base_norm();
        let mut targets = BTreeMap::new();
        for m in &norm.provenance_set.members {
            targets.insert(m.clone(), closed_target(m));
        }
        let resolver = MockResolver { targets };
        assert!(check_erosion(&norm, &resolver).is_none());
    }

    /// R-NRM-3: Widerruf erzeugt einen echten Workbody + Status revoked.
    #[test]
    fn revoke_seals_a_real_container_and_marks_revoked() {
        let norm = base_norm();
        let (revoked, sealed) = revoke(
            &norm,
            &"ff".repeat(34),
            "Gegenbeispiel in Familie X entdeckt",
            vec![CounterExample {
                core_root_hex: "ee".repeat(34),
                reason: "Abweichung".to_string(),
            }],
        )
        .expect("Widerruf siegelt");
        assert_eq!(revoked.status, NormStatus::Revoked);
        let verification = loom_verify::verify(&sealed.bytes);
        assert_eq!(
            verification.verdict,
            Verdict::ValidWithResidues,
            "{:?}",
            verification.diagnoses
        );
    }

    #[test]
    fn lineage_chain_walks_supersedes_backwards() {
        let old = BridgeNorm {
            norm_id: "norm:old".to_string(),
            ..base_norm()
        };
        let new = supersede(
            BridgeNorm {
                norm_id: "norm:new".to_string(),
                ..base_norm()
            },
            "norm:old".to_string(),
        );
        let norms = vec![old, new];
        let chain = lineage_chain(&norms, "norm:new");
        assert_eq!(chain.len(), 2);
        assert_eq!(chain[0].norm_id, "norm:new");
        assert_eq!(chain[1].norm_id, "norm:old");
    }
}
