//! S-E5 §2/§10(f): Normic Memory — die `norms/`-Sektion der Klassen-
//! Registry (E4c-Katalog-Workbody), additiv erweitert. Dieselbe additive
//! Cv-Erweiterungsdisziplin wie `manifest_declare_external_citations`
//! (S-E2a I.4)/`manifest_declare_hash_profiles` (X1c): eine bestehende
//! Map bekommt ein neues Feld, alte Leser bleiben unveraendert
//! funktionsfaehig (fehlt `norms`, ist das Ergebnis leer).

use crate::cv_util::cv_get;
use crate::types::{BridgeNorm, CounterExample, NexusClass, NormStatus, ProvenanceSet, Scope};
use loom_canon::Cv;

fn norm_to_cv(norm: &BridgeNorm) -> Cv {
    Cv::map(vec![
        ("norm_id", Cv::Text(norm.norm_id.clone())),
        ("nexus_class", Cv::Text(norm.nexus_class.as_str().into())),
        ("pattern", Cv::Text(norm.pattern.clone())),
        (
            "provenance_set",
            Cv::Array(
                norm.provenance_set
                    .members
                    .iter()
                    .cloned()
                    .map(Cv::Text)
                    .collect(),
            ),
        ),
        (
            "known_counterexamples",
            Cv::Array(
                norm.known_counterexamples
                    .iter()
                    .map(|c| {
                        Cv::map(vec![
                            ("core_root", Cv::Text(c.core_root_hex.clone())),
                            ("reason", Cv::Text(c.reason.clone())),
                        ])
                    })
                    .collect(),
            ),
        ),
        ("scope", Cv::Text(norm.scope.as_string())),
        ("status", Cv::Text(norm.status.as_str().into())),
        (
            "promotion_evidence",
            Cv::Text(norm.promotion_evidence.clone()),
        ),
        (
            "supersedes",
            match &norm.supersedes {
                Some(s) => Cv::Text(s.clone()),
                None => Cv::Null,
            },
        ),
    ])
}

fn norm_from_cv(v: &Cv) -> Option<BridgeNorm> {
    let norm_id = match cv_get(v, "norm_id") {
        Some(Cv::Text(s)) => s.clone(),
        _ => return None,
    };
    let nexus_class = match cv_get(v, "nexus_class") {
        Some(Cv::Text(s)) => NexusClass::parse(s)?,
        _ => return None,
    };
    let pattern = match cv_get(v, "pattern") {
        Some(Cv::Text(s)) => s.clone(),
        _ => return None,
    };
    let provenance_set = match cv_get(v, "provenance_set") {
        Some(Cv::Array(items)) => ProvenanceSet::new(
            items
                .iter()
                .filter_map(|i| match i {
                    Cv::Text(s) => Some(s.clone()),
                    _ => None,
                })
                .collect(),
        ),
        _ => return None,
    };
    let known_counterexamples = match cv_get(v, "known_counterexamples") {
        Some(Cv::Array(items)) => items
            .iter()
            .filter_map(|i| {
                let core_root_hex = match cv_get(i, "core_root") {
                    Some(Cv::Text(s)) => s.clone(),
                    _ => return None,
                };
                let reason = match cv_get(i, "reason") {
                    Some(Cv::Text(s)) => s.clone(),
                    _ => return None,
                };
                Some(CounterExample {
                    core_root_hex,
                    reason,
                })
            })
            .collect(),
        _ => vec![],
    };
    let scope = match cv_get(v, "scope") {
        Some(Cv::Text(s)) => Scope::parse(s)?,
        _ => return None,
    };
    let status = match cv_get(v, "status") {
        Some(Cv::Text(s)) => NormStatus::parse(s)?,
        _ => return None,
    };
    let promotion_evidence = match cv_get(v, "promotion_evidence") {
        Some(Cv::Text(s)) => s.clone(),
        _ => String::new(),
    };
    let supersedes = match cv_get(v, "supersedes") {
        Some(Cv::Text(s)) => Some(s.clone()),
        _ => None,
    };
    Some(BridgeNorm {
        norm_id,
        nexus_class,
        pattern,
        provenance_set,
        known_counterexamples,
        scope,
        status,
        promotion_evidence,
        supersedes,
    })
}

pub fn norms_section_field(norms: &[BridgeNorm]) -> Cv {
    Cv::Array(norms.iter().map(norm_to_cv).collect())
}

/// Haengt additiv eine `norms`-Sektion an einen bereits gebauten
/// Klassen-Registry-`KIND_DOC`-Cv an (`loom_cites::class_registry_field`).
pub fn attach_norms_section(registry_doc: Cv, norms: &[BridgeNorm]) -> Cv {
    let Cv::Map(mut entries) = registry_doc else {
        panic!("class_registry_field liefert immer eine Map")
    };
    entries.push((Cv::Text("norms".to_string()), norms_section_field(norms)));
    Cv::Map(entries)
}

/// Liest die `norms`-Sektion zurueck — leer, wenn additiv fehlend
/// (read-kompatibel mit jeder aelteren Klassen-Registry).
pub fn parse_norms_section(registry_doc: &Cv) -> Vec<BridgeNorm> {
    match cv_get(registry_doc, "norms") {
        Some(Cv::Array(items)) => items.iter().filter_map(norm_from_cv).collect(),
        _ => vec![],
    }
}

/// §2: die lesende API des Memory — Index nach Scope x NexusClass.
/// `class = None` liefert alle Klassen des Scopes.
pub fn query<'a>(
    norms: &'a [BridgeNorm],
    scope: &Scope,
    class: Option<NexusClass>,
) -> Vec<&'a BridgeNorm> {
    norms
        .iter()
        .filter(|n| &n.scope == scope && class.map(|c| c == n.nexus_class).unwrap_or(true))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_norm(scope: Scope, class: NexusClass) -> BridgeNorm {
        BridgeNorm {
            norm_id: "norm:1".to_string(),
            nexus_class: class,
            pattern: "p".to_string(),
            provenance_set: ProvenanceSet::new(vec!["aa".repeat(34)]),
            known_counterexamples: vec![],
            scope,
            status: NormStatus::Active,
            promotion_evidence: "e".to_string(),
            supersedes: Some("norm:0".to_string()),
        }
    }

    #[test]
    fn norms_section_roundtrips() {
        let norms = vec![sample_norm(Scope::Global, NexusClass::StructuralRule)];
        let registry_doc = loom_cites::class_registry_field(&[]);
        let extended = attach_norms_section(registry_doc, &norms);
        let back = parse_norms_section(&extended);
        assert_eq!(back, norms);
        // Additiv: die urspruengliche `entries`-Sektion bleibt lesbar.
        assert!(loom_cites::parse_class_registry(&extended)
            .expect("weiterhin als Klassen-Registry lesbar")
            .is_empty());
    }

    #[test]
    fn missing_norms_section_parses_as_empty() {
        let registry_doc = loom_cites::class_registry_field(&[]);
        assert!(parse_norms_section(&registry_doc).is_empty());
    }

    #[test]
    fn query_filters_by_scope_and_class() {
        let norms = vec![
            sample_norm(Scope::Domain("d06".to_string()), NexusClass::StructuralRule),
            sample_norm(Scope::Domain("d06".to_string()), NexusClass::SeamPattern),
            sample_norm(Scope::Global, NexusClass::StructuralRule),
        ];
        let hits = query(
            &norms,
            &Scope::Domain("d06".to_string()),
            Some(NexusClass::StructuralRule),
        );
        assert_eq!(hits.len(), 1);
        let all_in_scope = query(&norms, &Scope::Domain("d06".to_string()), None);
        assert_eq!(all_in_scope.len(), 2);
    }
}
