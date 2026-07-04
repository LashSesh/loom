//! S-E5 §2/§4 + Dokument 16 §2b (R-Agent-14, ScopeGate v2): die
//! typisierte Pattern-Repräsentation. Die Whitelist (§4) ist ab hier
//! eine KONSTRUKTIONS-Eigenschaft, keine Text-Eigenschaft mehr — jede
//! Variante kann strukturell nur ausdrücken, was ihre `NexusClass`
//! erlaubt; keine Variante trägt ein Feld, mit dem sich ein Gate/eine
//! Invariante/ein Capability-Lock/Egress/Verdikt-Schreibweg lockern
//! ließe. `gate.rs`s ScopeGate prüft nur noch die eine verbleibende,
//! genuin gefährliche Form (`ClosureProfile`: darf keinen bestehenden
//! Fundament-Gate-Namen "kapern").

use crate::types::NexusClass;
use loom_canon::Cv;

/// Die sechs bestehenden Regeltypen — spiegelbildlich zu
/// `cce_materialize::family_a::DomainRule`, hier aber OWNED (`String`/
/// `Vec<String>` statt `&'static str`): eine Destillation entsteht zur
/// LAUFZEIT aus echten Blueprint-Facetten, nicht aus Compile-Zeit-
/// Konstanten — `cce-bridge` bleibt deshalb bewusst unabhängig von
/// `cce-materialize` (kein neuer Layering-Zyklus, keine
/// `'static`-Reibung).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DomainRuleForm {
    Relation { seam: String },
    AcyclicRelation { seam: String },
    ChainedRelation { seam: String },
    UniqueSubjects,
    OrderedSteps { seam: String },
    StructuralPresence { markers: Vec<String> },
}

impl DomainRuleForm {
    /// Vollstaendige, injektive Kurzform (anders als `kind_tag` allein
    /// — die trueg bei zwei verschiedenen Nahtnamen dieselbe Zeichenkette).
    fn describe(&self) -> String {
        match self {
            DomainRuleForm::Relation { seam } => format!("relation:{seam}"),
            DomainRuleForm::AcyclicRelation { seam } => format!("acyclic_relation:{seam}"),
            DomainRuleForm::ChainedRelation { seam } => format!("chained_relation:{seam}"),
            DomainRuleForm::UniqueSubjects => "unique_subjects".to_string(),
            DomainRuleForm::OrderedSteps { seam } => format!("ordered_steps:{seam}"),
            DomainRuleForm::StructuralPresence { markers } => {
                format!("structural_presence:{}", markers.join(","))
            }
        }
    }

    fn kind_tag(&self) -> &'static str {
        match self {
            DomainRuleForm::Relation { .. } => "relation",
            DomainRuleForm::AcyclicRelation { .. } => "acyclic_relation",
            DomainRuleForm::ChainedRelation { .. } => "chained_relation",
            DomainRuleForm::UniqueSubjects => "unique_subjects",
            DomainRuleForm::OrderedSteps { .. } => "ordered_steps",
            DomainRuleForm::StructuralPresence { .. } => "structural_presence",
        }
    }

    fn to_cv(&self) -> Cv {
        let mut fields = vec![("kind", Cv::Text(self.kind_tag().into()))];
        match self {
            DomainRuleForm::Relation { seam }
            | DomainRuleForm::AcyclicRelation { seam }
            | DomainRuleForm::ChainedRelation { seam }
            | DomainRuleForm::OrderedSteps { seam } => {
                fields.push(("seam", Cv::Text(seam.clone())));
            }
            DomainRuleForm::UniqueSubjects => {}
            DomainRuleForm::StructuralPresence { markers } => {
                fields.push((
                    "markers",
                    Cv::Array(markers.iter().map(|m| Cv::Text(m.clone())).collect()),
                ));
            }
        }
        Cv::map(fields)
    }

    fn from_cv(v: &Cv) -> Option<DomainRuleForm> {
        let get = |k: &str| cv_get(v, k);
        let seam = || match get("seam") {
            Some(Cv::Text(s)) => Some(s.clone()),
            _ => None,
        };
        match get("kind") {
            Some(Cv::Text(k)) => match k.as_str() {
                "relation" => Some(DomainRuleForm::Relation { seam: seam()? }),
                "acyclic_relation" => Some(DomainRuleForm::AcyclicRelation { seam: seam()? }),
                "chained_relation" => Some(DomainRuleForm::ChainedRelation { seam: seam()? }),
                "ordered_steps" => Some(DomainRuleForm::OrderedSteps { seam: seam()? }),
                "unique_subjects" => Some(DomainRuleForm::UniqueSubjects),
                "structural_presence" => match get("markers") {
                    Some(Cv::Array(items)) => Some(DomainRuleForm::StructuralPresence {
                        markers: items
                            .iter()
                            .filter_map(|i| match i {
                                Cv::Text(s) => Some(s.clone()),
                                _ => None,
                            })
                            .collect(),
                    }),
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        }
    }
}

fn cv_get<'a>(map: &'a Cv, key: &str) -> Option<&'a Cv> {
    if let Cv::Map(entries) = map {
        entries.iter().find_map(|(k, v)| match k {
            Cv::Text(s) if s == key => Some(v),
            _ => None,
        })
    } else {
        None
    }
}

/// §2/§4: die fünf whitelisted Pattern-Formen. Jede Variante ist per
/// Konstruktion additiv/nicht-lockernd (s. Modul-Doku).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pattern {
    /// DomainRule-Instanz eines der sechs bestehenden Regeltypen.
    StructuralRule(DomainRuleForm),
    /// Naht-Muster-Empfehlung — informativ, nie enforced.
    SeamPattern {
        seam: String,
        recommendation: String,
    },
    /// Zusatz-Gate-Profil, das NUR verschärft: eine Liste NEUER
    /// Gate-IDs. `gate.rs`s ScopeGate v2 weist jede ID zurück, die
    /// einen bestehenden Fundament-Gate-Namen trägt (Kaperungsversuch
    /// statt echter Neuheit).
    ClosureProfile { added_gate_ids: Vec<String> },
    /// Terminologie-/Benennungsnorm.
    VocabularyNorm { term: String, definition: String },
    /// Reihenfolge-Muster (PhaseLadder-artig) — eine empfohlene Folge.
    ProcessNorm { sequence: Vec<String> },
}

impl Pattern {
    pub fn class(&self) -> NexusClass {
        match self {
            Pattern::StructuralRule(_) => NexusClass::StructuralRule,
            Pattern::SeamPattern { .. } => NexusClass::SeamPattern,
            Pattern::ClosureProfile { .. } => NexusClass::ClosureProfile,
            Pattern::VocabularyNorm { .. } => NexusClass::VocabularyNorm,
            Pattern::ProcessNorm { .. } => NexusClass::ProcessNorm,
        }
    }

    /// Menschenlesbare Kurzbeschreibung (Titel/Log-Zwecke) — NIE
    /// Grundlage einer Gate-Entscheidung (die laeuft auf der
    /// typisierten Form selbst).
    pub fn describe(&self) -> String {
        match self {
            Pattern::StructuralRule(form) => format!("structural_rule:{}", form.describe()),
            Pattern::SeamPattern { seam, .. } => format!("seam_pattern:{seam}"),
            Pattern::ClosureProfile { added_gate_ids } => {
                format!("closure_profile:+{}", added_gate_ids.join(","))
            }
            Pattern::VocabularyNorm { term, .. } => format!("vocabulary_norm:{term}"),
            Pattern::ProcessNorm { sequence } => format!("process_norm:{}", sequence.join("->")),
        }
    }

    pub fn to_cv(&self) -> Cv {
        match self {
            Pattern::StructuralRule(form) => Cv::map(vec![
                ("kind", Cv::Text("structural_rule".into())),
                ("rule", form.to_cv()),
            ]),
            Pattern::SeamPattern {
                seam,
                recommendation,
            } => Cv::map(vec![
                ("kind", Cv::Text("seam_pattern".into())),
                ("seam", Cv::Text(seam.clone())),
                ("recommendation", Cv::Text(recommendation.clone())),
            ]),
            Pattern::ClosureProfile { added_gate_ids } => Cv::map(vec![
                ("kind", Cv::Text("closure_profile".into())),
                (
                    "added_gate_ids",
                    Cv::Array(added_gate_ids.iter().map(|g| Cv::Text(g.clone())).collect()),
                ),
            ]),
            Pattern::VocabularyNorm { term, definition } => Cv::map(vec![
                ("kind", Cv::Text("vocabulary_norm".into())),
                ("term", Cv::Text(term.clone())),
                ("definition", Cv::Text(definition.clone())),
            ]),
            Pattern::ProcessNorm { sequence } => Cv::map(vec![
                ("kind", Cv::Text("process_norm".into())),
                (
                    "sequence",
                    Cv::Array(sequence.iter().map(|s| Cv::Text(s.clone())).collect()),
                ),
            ]),
        }
    }

    pub fn from_cv(v: &Cv) -> Option<Pattern> {
        match cv_get(v, "kind") {
            Some(Cv::Text(k)) => match k.as_str() {
                "structural_rule" => {
                    let rule = cv_get(v, "rule")?;
                    Some(Pattern::StructuralRule(DomainRuleForm::from_cv(rule)?))
                }
                "seam_pattern" => {
                    let seam = match cv_get(v, "seam") {
                        Some(Cv::Text(s)) => s.clone(),
                        _ => return None,
                    };
                    let recommendation = match cv_get(v, "recommendation") {
                        Some(Cv::Text(s)) => s.clone(),
                        _ => return None,
                    };
                    Some(Pattern::SeamPattern {
                        seam,
                        recommendation,
                    })
                }
                "closure_profile" => {
                    let added_gate_ids = match cv_get(v, "added_gate_ids") {
                        Some(Cv::Array(items)) => items
                            .iter()
                            .filter_map(|i| match i {
                                Cv::Text(s) => Some(s.clone()),
                                _ => None,
                            })
                            .collect(),
                        _ => return None,
                    };
                    Some(Pattern::ClosureProfile { added_gate_ids })
                }
                "vocabulary_norm" => {
                    let term = match cv_get(v, "term") {
                        Some(Cv::Text(s)) => s.clone(),
                        _ => return None,
                    };
                    let definition = match cv_get(v, "definition") {
                        Some(Cv::Text(s)) => s.clone(),
                        _ => return None,
                    };
                    Some(Pattern::VocabularyNorm { term, definition })
                }
                "process_norm" => {
                    let sequence = match cv_get(v, "sequence") {
                        Some(Cv::Array(items)) => items
                            .iter()
                            .filter_map(|i| match i {
                                Cv::Text(s) => Some(s.clone()),
                                _ => None,
                            })
                            .collect(),
                        _ => return None,
                    };
                    Some(Pattern::ProcessNorm { sequence })
                }
                _ => None,
            },
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn samples() -> Vec<Pattern> {
        vec![
            Pattern::StructuralRule(DomainRuleForm::Relation {
                seam: "refers".to_string(),
            }),
            Pattern::StructuralRule(DomainRuleForm::UniqueSubjects),
            Pattern::StructuralRule(DomainRuleForm::StructuralPresence {
                markers: vec!["Anrede".to_string(), "Schluss".to_string()],
            }),
            Pattern::SeamPattern {
                seam: "supports".to_string(),
                recommendation: "Gegenmassnahmen sollten Risiken stuetzen".to_string(),
            },
            Pattern::ClosureProfile {
                added_gate_ids: vec!["G-Extra-Check".to_string()],
            },
            Pattern::VocabularyNorm {
                term: "Residuum".to_string(),
                definition: "sichtbarer, unvollstaendiger Rest".to_string(),
            },
            Pattern::ProcessNorm {
                sequence: vec!["encode".to_string(), "loom".to_string()],
            },
        ]
    }

    #[test]
    fn all_patterns_roundtrip_through_cv() {
        for p in samples() {
            let back = Pattern::from_cv(&p.to_cv()).expect("roundtrip");
            assert_eq!(back, p);
        }
    }

    #[test]
    fn class_matches_nexus_class_one_to_one() {
        assert_eq!(
            Pattern::StructuralRule(DomainRuleForm::UniqueSubjects).class(),
            NexusClass::StructuralRule
        );
        assert_eq!(
            Pattern::SeamPattern {
                seam: "s".into(),
                recommendation: "r".into()
            }
            .class(),
            NexusClass::SeamPattern
        );
        assert_eq!(
            Pattern::ClosureProfile {
                added_gate_ids: vec![]
            }
            .class(),
            NexusClass::ClosureProfile
        );
        assert_eq!(
            Pattern::VocabularyNorm {
                term: "t".into(),
                definition: "d".into()
            }
            .class(),
            NexusClass::VocabularyNorm
        );
        assert_eq!(
            Pattern::ProcessNorm { sequence: vec![] }.class(),
            NexusClass::ProcessNorm
        );
    }
}
