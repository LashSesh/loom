//! Dokument 16 §2a (schließt R-Agent-13): der typisierte Extraktor
//! `blueprint_to_pattern` — die Pattern-Herkunft ist ab hier beweisbar
//! aus echter HBM-Eigenarbeit (`cce_hbm::candidate::BlueprintCandidate`),
//! nicht mehr caller-geliefert. Nur whitelist-konforme Pattern-Formen
//! (Spec 15 §4) entstehen; ein Blueprint ohne erkennbare Regel-Facette
//! liefert `None` — nie stilles Raten, der Aufrufer meldet
//! `pattern_extraction_unsupported` sichtbar.

use crate::pattern::{DomainRuleForm, Pattern};
use cce_hbm::candidate::BlueprintCandidate;

/// Parst eine `invariant`-Facette der Form, die
/// `loom_conformance::domain_rule_fact` erzeugt:
/// `"{id}|regel={Name}|naht={seam}"` · `"{id}|regel=UniqueSubjects"` ·
/// `"{id}|regel=StructuralPresence|marker={a,b,...}"`. Jede andere Form
/// (auch jede ANDERE Facette, z. B. `gate:`/`constraint:`) ist nicht
/// abbildbar und liefert `None`.
fn parse_invariant_scope(scope: &str) -> Option<DomainRuleForm> {
    let mut parts = scope.split('|');
    let _id = parts.next()?;
    let regel_name = parts.next()?.strip_prefix("regel=")?;
    match regel_name {
        "UniqueSubjects" => Some(DomainRuleForm::UniqueSubjects),
        "Relation" | "AcyclicRelation" | "ChainedRelation" | "OrderedSteps" => {
            let seam = parts.next()?.strip_prefix("naht=")?.to_string();
            Some(match regel_name {
                "Relation" => DomainRuleForm::Relation { seam },
                "AcyclicRelation" => DomainRuleForm::AcyclicRelation { seam },
                "ChainedRelation" => DomainRuleForm::ChainedRelation { seam },
                "OrderedSteps" => DomainRuleForm::OrderedSteps { seam },
                _ => unreachable!("durch den aeusseren match bereits eingeschraenkt"),
            })
        }
        "StructuralPresence" => {
            let markers = parts
                .next()?
                .strip_prefix("marker=")?
                .split(',')
                .map(str::to_string)
                .collect();
            Some(DomainRuleForm::StructuralPresence { markers })
        }
        _ => None,
    }
}

/// Dokument 16 §2a: liefert das Pattern eines Blueprint-Kandidaten —
/// die erste `invariant`-Facette in stabiler Facet-Reihenfolge, die
/// sich als eine der sechs bestehenden Regelformen erkennen laesst.
/// `None`, wenn KEINE Facette erkennbar ist (der Blueprint traegt kein
/// destillierbares Struktur-Muster) — der Aufrufer meldet
/// `pattern_extraction_unsupported`, rät nie.
pub fn blueprint_to_pattern(candidate: &BlueprintCandidate) -> Option<Pattern> {
    candidate
        .facets
        .iter()
        .filter(|f| f.facet_type == "invariant")
        .find_map(|f| parse_invariant_scope(&f.scope))
        .map(Pattern::StructuralRule)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cce_core::residue::ResidueField;
    use cce_hbm::candidate::CandidateStatus;
    use cce_hbm::facet::Facet;

    fn facet(facet_type: &str, scope: &str) -> Facet {
        Facet {
            id: format!("f-{scope}"),
            facet_type: facet_type.to_string(),
            scope: scope.to_string(),
            source: "test".to_string(),
            evidence: Some("test#L0".to_string()),
            confidence_permille: 800,
        }
    }

    fn candidate(facets: Vec<Facet>) -> BlueprintCandidate {
        BlueprintCandidate {
            id: "cand:test".to_string(),
            generator: "C6",
            facets,
            constraints: vec![],
            horizon: vec![],
            layers: vec![],
            status: CandidateStatus::Pass,
            hold_diagnosis: None,
            residues: ResidueField::new(),
        }
    }

    #[test]
    fn extracts_relation_rule_from_real_shaped_invariant_facet() {
        let b = candidate(vec![facet(
            "invariant",
            "D02-contract|regel=Relation|naht=refers",
        )]);
        let pattern = blueprint_to_pattern(&b).expect("Pattern erkannt");
        assert_eq!(
            pattern,
            Pattern::StructuralRule(DomainRuleForm::Relation {
                seam: "refers".to_string()
            })
        );
    }

    #[test]
    fn extracts_unique_subjects_without_seam() {
        let b = candidate(vec![facet(
            "invariant",
            "D14-checklist|regel=UniqueSubjects",
        )]);
        assert_eq!(
            blueprint_to_pattern(&b),
            Some(Pattern::StructuralRule(DomainRuleForm::UniqueSubjects))
        );
    }

    #[test]
    fn extracts_structural_presence_with_multiple_markers() {
        let b = candidate(vec![facet(
            "invariant",
            "D05-letter|regel=StructuralPresence|marker=Anrede,Schluss",
        )]);
        assert_eq!(
            blueprint_to_pattern(&b),
            Some(Pattern::StructuralRule(
                DomainRuleForm::StructuralPresence {
                    markers: vec!["Anrede".to_string(), "Schluss".to_string()]
                }
            ))
        );
    }

    /// Nicht-invariant-Facetten (gate:/constraint:) sind nicht die
    /// gesuchte Form — ein Blueprint, das NUR solche traegt, liefert
    /// `None` statt zu raten.
    #[test]
    fn candidate_without_a_recognizable_invariant_facet_is_unsupported() {
        let b = candidate(vec![
            facet("gate", "D02-contract|core:D02-contract"),
            facet("constraint", "D02-contract|dangling_clause"),
        ]);
        assert_eq!(blueprint_to_pattern(&b), None);
    }

    #[test]
    fn free_text_invariant_facet_is_unsupported_not_guessed() {
        let b = candidate(vec![facet(
            "invariant",
            "irgendein freier Fliesstext ohne die erkannte Form",
        )]);
        assert_eq!(blueprint_to_pattern(&b), None);
    }

    /// Bei mehreren Kandidaten-Facetten gewinnt die ERSTE erkennbare —
    /// deterministisch, an der stabilen Facet-Reihenfolge festgemacht.
    #[test]
    fn first_recognizable_invariant_facet_wins_deterministically() {
        let b = candidate(vec![
            facet("invariant", "D02-contract|regel=Relation|naht=refers"),
            facet("invariant", "D03-spec|regel=Relation|naht=derives"),
        ]);
        assert_eq!(
            blueprint_to_pattern(&b),
            Some(Pattern::StructuralRule(DomainRuleForm::Relation {
                seam: "refers".to_string()
            }))
        );
    }
}
