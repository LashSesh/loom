//! Familie A — die 14 Leaf-Domänen D02–D15 als reine Profil-
//! Spezialisierung über dem Familien-Kern `family_a`. D01 bleibt der
//! eigenständige PL4-Anker (document::DocumentAdapter). Je Domäne:
//! Kern-Regel + Kern-Residuum (aus S1_DOMAENENKATALOG K.4),
//! 1 Referenz-Cube (schließt), ≥2 Negativ-Cubes (erwartetes Residuum).

use crate::document::{DocCrystal, DocUnit, UnitType};
use crate::family_a::{DocProfile, DomainRule};

// ---- Cube-Baukasten (geteilt) --------------------------------------

pub fn section(id: &str, text: &str) -> DocUnit {
    DocUnit::new(id, UnitType::Section, text)
}
/// Subjekt-Einheit (neutraler Typ ohne Stütz-Pflicht) mit optionaler
/// Kern-Naht.
pub fn subject(id: &str, text: &str, seam: Option<(&str, &str)>) -> DocUnit {
    let u = DocUnit::new(id, UnitType::Definition, text);
    match seam {
        Some((k, t)) => u.with_seam(k, t),
        None => u,
    }
}
pub fn object(id: &str, text: &str) -> DocUnit {
    DocUnit::new(id, UnitType::Support, text)
}
pub fn crystal(title: &str, section_name: &str, units: Vec<DocUnit>) -> DocCrystal {
    DocCrystal {
        title: title.to_string(),
        units,
        covers: vec![],
        required_sections: vec![section_name.to_string()],
        no_score_fields: true,
        ordering: "neutral".to_string(),
    }
}

/// Standard-Referenz für Relation-Domänen: 1 Section, 2 Subjekte mit
/// Kern-Naht auf einen gemeinsamen Anker.
pub fn relation_reference(
    title: &str,
    sec: &str,
    seam: &str,
    s1: &str,
    s2: &str,
    anchor: &str,
) -> DocCrystal {
    crystal(
        title,
        sec,
        vec![
            section("s0", sec),
            subject("a1", s1, Some((seam, "o1"))),
            subject("a2", s2, Some((seam, "o1"))),
            object("o1", anchor),
        ],
    )
}

/// Standard-Negative für Relation-Domänen: Subjekt ohne Kern-Naht
/// (aber via Neben-Naht verbunden, damit NUR die Kern-Regel bricht).
pub fn relation_negatives(sec: &str, seam: &str) -> Vec<(DocCrystal, &'static str)> {
    vec![
        // (1) Subjekt ohne Kern-Naht — via "notes" verbunden.
        (
            crystal(
                "neg-missing",
                sec,
                vec![
                    section("s0", sec),
                    subject("a1", "Subjekt ohne Kern-Naht", Some(("notes", "s0"))),
                    object("o1", "Anker"),
                ],
            ),
            "MISS",
        ),
        // (2) zweites Subjekt fehlt die Kern-Naht.
        (
            crystal(
                "neg-partial",
                sec,
                vec![
                    section("s0", sec),
                    subject("a1", "erstes Subjekt", Some((seam, "o1"))),
                    subject("a2", "zweites ohne Naht", Some(("notes", "s0"))),
                    object("o1", "Anker"),
                ],
            ),
            "MISS",
        ),
    ]
}

// ---- Die 14 Profile ------------------------------------------------

macro_rules! relation_domain {
    ($fn:ident, $id:literal, $sec:literal, $seam:literal, $res:literal, $s1:literal, $s2:literal, $anchor:literal, $fmts:expr) => {
        pub fn $fn() -> DocProfile {
            DocProfile {
                id: $id,
                subject: UnitType::Definition,
                rule: DomainRule::Relation { seam: $seam },
                core_residue: $res,
                reference: || relation_reference($id, $sec, $seam, $s1, $s2, $anchor),
                negatives: || {
                    relation_negatives($sec, $seam)
                        .into_iter()
                        .map(|(c, _)| (c, $res))
                        .collect()
                },
                export_formats: $fmts,
            }
        }
    };
}
pub(crate) use relation_domain;

relation_domain!(
    d02,
    "D02-contract",
    "Vertrag",
    "refers",
    "dangling_clause",
    "Klausel A verweist auf Praeambel",
    "Klausel B verweist auf Praeambel",
    "Praeambel",
    &[".docx", ".pdf"]
);
relation_domain!(
    d03,
    "D03-spec",
    "Anforderungen",
    "derives",
    "untestable_req",
    "Anforderung A ableitbar aus Ziel",
    "Anforderung B ableitbar aus Ziel",
    "Systemziel",
    &[".md", ".docx"]
);
relation_domain!(
    d06,
    "D06-proposal",
    "Angebot",
    "priced",
    "orphan_line_item",
    "Position Lizenz mit Preisbezug",
    "Position Support mit Preisbezug",
    "Preisliste",
    &[".docx", ".pdf"]
);
relation_domain!(
    d07,
    "D07-policy",
    "Richtlinie",
    "scopes",
    "unscoped_rule",
    "Regel Zugriff mit Geltungsbereich",
    "Regel Aufbewahrung mit Geltungsbereich",
    "Geltungsbereich IT",
    &[".md", ".docx"]
);
relation_domain!(
    d08,
    "D08-minutes",
    "Protokoll",
    "resolves",
    "actionless_item",
    "Punkt Budget mit Beschluss",
    "Punkt Termin mit Beschluss",
    "Beschlusslage",
    &[".md"]
);
relation_domain!(
    d09,
    "D09-abstract",
    "Zusammenfassung",
    "sources",
    "invented_semantic",
    "Kernaussage A quellenbelegt",
    "Kernaussage B quellenbelegt",
    "Quelle Studie",
    &[".md"]
);
relation_domain!(
    d10,
    "D10-press",
    "Pressemitteilung",
    "supports",
    "unsupported_claim",
    "Botschaft Wachstum belegt",
    "Botschaft Qualitaet belegt",
    "Faktenlage",
    &[".docx"]
);
relation_domain!(
    d12,
    "D12-whitepaper",
    "Whitepaper",
    "argues",
    "broken_argument",
    "These A mit Argumentkette",
    "These B mit Argumentkette",
    "Praemisse",
    &[".pdf", ".md"]
);
relation_domain!(
    d13,
    "D13-faq",
    "FAQ",
    "answers",
    "unanswered_question",
    "Frage zur Einrichtung",
    "Frage zur Abrechnung",
    "Antwortblock",
    &[".md"]
);

/// D04 Handbuch/Manual — Schritt·Reihenfolge → gap_in_procedure.
pub fn d04() -> DocProfile {
    DocProfile {
        id: "D04-manual",
        subject: UnitType::Step,
        rule: DomainRule::OrderedSteps { seam: "next" },
        core_residue: "gap_in_procedure",
        reference: || {
            crystal(
                "D04-manual",
                "Ablauf",
                vec![
                    section("s0", "Ablauf"),
                    DocUnit::new("p1", UnitType::Step, "Geraet einschalten")
                        .with_seam("next", "p2"),
                    DocUnit::new("p2", UnitType::Step, "Anmeldung durchfuehren")
                        .with_seam("next", "p3"),
                    DocUnit::new("p3", UnitType::Step, "Vorgang bestaetigen")
                        .with_seam("notes", "s0"),
                ],
            )
        },
        negatives: || {
            vec![
                (
                    crystal(
                        "neg-two-roots",
                        "Ablauf",
                        vec![
                            section("s0", "Ablauf"),
                            DocUnit::new("p1", UnitType::Step, "Schritt A")
                                .with_seam("notes", "s0"),
                            DocUnit::new("p2", UnitType::Step, "Schritt B")
                                .with_seam("notes", "s0"),
                        ],
                    ),
                    "gap_in_procedure",
                ),
                (
                    crystal(
                        "neg-broken",
                        "Ablauf",
                        vec![
                            section("s0", "Ablauf"),
                            DocUnit::new("p1", UnitType::Step, "Schritt A").with_seam("next", "p2"),
                            DocUnit::new("p2", UnitType::Step, "Schritt B")
                                .with_seam("notes", "s0"),
                            DocUnit::new("p3", UnitType::Step, "verwaister Schritt")
                                .with_seam("notes", "s0"),
                        ],
                    ),
                    "gap_in_procedure",
                ),
            ]
        },
        export_formats: &[".md", ".pdf"],
    }
}

/// D05 Brief/Korrespondenz — Anrede/Schluss → missing_salutation.
pub fn d05() -> DocProfile {
    DocProfile {
        id: "D05-letter",
        subject: UnitType::Section,
        rule: DomainRule::StructuralPresence {
            markers: &["Anrede", "Schluss"],
        },
        core_residue: "missing_salutation",
        reference: || {
            crystal(
                "D05-letter",
                "Brief",
                vec![
                    section("s0", "Brief"),
                    section("s1", "Anrede: Sehr geehrte Damen und Herren"),
                    DocUnit::new("b1", UnitType::Definition, "Hauptteil des Briefs")
                        .with_seam("notes", "s0"),
                    section("s2", "Schluss: Mit freundlichen Gruessen"),
                ],
            )
        },
        negatives: || {
            vec![
                (
                    crystal(
                        "neg-no-salutation",
                        "Brief",
                        vec![
                            section("s0", "Brief"),
                            DocUnit::new("b1", UnitType::Definition, "Hauptteil")
                                .with_seam("notes", "s0"),
                            section("s2", "Schluss: Gruss"),
                        ],
                    ),
                    "missing_salutation",
                ),
                (
                    crystal(
                        "neg-no-closing",
                        "Brief",
                        vec![
                            section("s0", "Brief"),
                            section("s1", "Anrede: Hallo"),
                            DocUnit::new("b1", UnitType::Definition, "Hauptteil")
                                .with_seam("notes", "s0"),
                        ],
                    ),
                    "missing_salutation",
                ),
            ]
        },
        export_formats: &[".docx"],
    }
}

/// D11 Lebenslauf/CV — Station·Zeit → timeline_gap.
pub fn d11() -> DocProfile {
    DocProfile {
        id: "D11-cv",
        subject: UnitType::Definition,
        rule: DomainRule::ChainedRelation { seam: "after" },
        core_residue: "timeline_gap",
        reference: || {
            crystal(
                "D11-cv",
                "Lebenslauf",
                vec![
                    section("s0", "Lebenslauf"),
                    subject("st1", "Studium 2016-2020", Some(("notes", "s0"))),
                    subject("st2", "Erste Stelle 2020-2023", Some(("after", "st1"))),
                    subject("st3", "Zweite Stelle seit 2023", Some(("after", "st2"))),
                ],
            )
        },
        negatives: || {
            vec![
                (
                    crystal(
                        "neg-two-roots",
                        "Lebenslauf",
                        vec![
                            section("s0", "Lebenslauf"),
                            subject("st1", "Station A", Some(("notes", "s0"))),
                            subject("st2", "Station B", Some(("notes", "s0"))),
                        ],
                    ),
                    "timeline_gap",
                ),
                (
                    crystal(
                        "neg-detached",
                        "Lebenslauf",
                        vec![
                            section("s0", "Lebenslauf"),
                            subject("st1", "Station A", Some(("after", "st2"))),
                            subject("st2", "Station B", Some(("notes", "s0"))),
                            subject("st3", "losgeloeste Station", Some(("notes", "s0"))),
                        ],
                    ),
                    "timeline_gap",
                ),
            ]
        },
        export_formats: &[".docx", ".pdf"],
    }
}

/// D14 Checkliste — Prüfpunkt·Ordnung → ambiguous_item.
pub fn d14() -> DocProfile {
    DocProfile {
        id: "D14-checklist",
        subject: UnitType::Definition,
        rule: DomainRule::UniqueSubjects,
        core_residue: "ambiguous_item",
        reference: || {
            crystal(
                "D14-checklist",
                "Checkliste",
                vec![
                    section("s0", "Checkliste"),
                    subject("c1", "Sicherung geprueft", Some(("notes", "s0"))),
                    subject("c2", "Zugriff getestet", Some(("notes", "s0"))),
                    subject("c3", "Protokoll erstellt", Some(("notes", "s0"))),
                ],
            )
        },
        negatives: || {
            vec![
                (
                    crystal(
                        "neg-dup",
                        "Checkliste",
                        vec![
                            section("s0", "Checkliste"),
                            subject("c1", "Sicherung geprueft", Some(("notes", "s0"))),
                            subject("c2", "Sicherung geprueft", Some(("notes", "s0"))),
                        ],
                    ),
                    "ambiguous_item",
                ),
                (
                    crystal(
                        "neg-dup2",
                        "Checkliste",
                        vec![
                            section("s0", "Checkliste"),
                            subject("c1", "Zugriff getestet", Some(("notes", "s0"))),
                            subject("c2", "Protokoll erstellt", Some(("notes", "s0"))),
                            subject("c3", "Zugriff getestet", Some(("notes", "s0"))),
                        ],
                    ),
                    "ambiguous_item",
                ),
            ]
        },
        export_formats: &[".md"],
    }
}

/// D15 Glossar — Begriff·Definition → undefined_term (+ Zirkel).
pub fn d15() -> DocProfile {
    DocProfile {
        id: "D15-glossary",
        subject: UnitType::Definition,
        rule: DomainRule::AcyclicRelation { seam: "defines" },
        core_residue: "undefined_term",
        reference: || {
            crystal(
                "D15-glossary",
                "Glossar",
                vec![
                    section("s0", "Glossar"),
                    subject("t1", "Begriff Kristall", Some(("defines", "o1"))),
                    subject("t2", "Begriff Naht", Some(("defines", "o1"))),
                    object("o1", "Grundbegriff Struktur"),
                ],
            )
        },
        negatives: || {
            vec![
                (
                    crystal(
                        "neg-undefined",
                        "Glossar",
                        vec![
                            section("s0", "Glossar"),
                            subject("t1", "Begriff ohne Definition", Some(("notes", "s0"))),
                            object("o1", "Grundbegriff"),
                        ],
                    ),
                    "undefined_term",
                ),
                (
                    crystal(
                        "neg-cycle",
                        "Glossar",
                        vec![
                            section("s0", "Glossar"),
                            subject("t1", "Begriff A", Some(("defines", "t2"))),
                            subject("t2", "Begriff B", Some(("defines", "t1"))),
                        ],
                    ),
                    "undefined_term",
                ),
            ]
        },
        export_formats: &[".md"],
    }
}

/// Standard-Referenz fuer Ketten-Domaenen: 1 Section + 3 Subjekte,
/// per `seam` lueckenlos verkettet (eine Wurzel).
pub fn chain_reference(
    title: &str,
    sec: &str,
    seam: &str,
    s1: &str,
    s2: &str,
    s3: &str,
) -> DocCrystal {
    crystal(
        title,
        sec,
        vec![
            section("s0", sec),
            subject("k1", s1, Some(("notes", "s0"))),
            subject("k2", s2, Some((seam, "k1"))),
            subject("k3", s3, Some((seam, "k2"))),
        ],
    )
}

/// Standard-Negative fuer Ketten-Domaenen: (1) zwei Wurzeln, (2) losgeloest.
pub fn chain_negatives(sec: &str, seam: &str) -> Vec<(DocCrystal, &'static str)> {
    vec![
        (
            crystal(
                "neg-two-roots",
                sec,
                vec![
                    section("s0", sec),
                    subject("k1", "Stufe A", Some(("notes", "s0"))),
                    subject("k2", "Stufe B", Some(("notes", "s0"))),
                ],
            ),
            "CHAIN",
        ),
        (
            crystal(
                "neg-detached",
                sec,
                vec![
                    section("s0", sec),
                    subject("k1", "Stufe A", Some((seam, "k2"))),
                    subject("k2", "Stufe B", Some(("notes", "s0"))),
                    subject("k3", "losgeloeste Stufe", Some(("notes", "s0"))),
                ],
            ),
            "CHAIN",
        ),
    ]
}

/// Makro fuer Ketten-Domaenen (ChainedRelation).
macro_rules! chained_domain {
    ($fn:ident, $id:literal, $sec:literal, $seam:literal, $res:literal, $s1:literal, $s2:literal, $s3:literal, $fmts:expr) => {
        pub fn $fn() -> DocProfile {
            DocProfile {
                id: $id,
                subject: UnitType::Definition,
                rule: DomainRule::ChainedRelation { seam: $seam },
                core_residue: $res,
                reference: || chain_reference($id, $sec, $seam, $s1, $s2, $s3),
                negatives: || {
                    chain_negatives($sec, $seam)
                        .into_iter()
                        .map(|(c, _)| (c, $res))
                        .collect()
                },
                export_formats: $fmts,
            }
        }
    };
}
pub(crate) use chained_domain;

/// Alle 14 Profile der Familie A (D02–D15).
pub fn all_profiles() -> Vec<DocProfile> {
    vec![
        d02(),
        d03(),
        d04(),
        d05(),
        d06(),
        d07(),
        d08(),
        d09(),
        d10(),
        d11(),
        d12(),
        d13(),
        d14(),
        d15(),
    ]
}
