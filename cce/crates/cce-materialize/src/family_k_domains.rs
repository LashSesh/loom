//! Familie K — Bildung/Training (EDU01–12) über dem Familien-Kern
//! `family_a`. 10 Relation-Regeln + 2 Ketten (EDU04 Schwierigkeits-
//! folge lückenlos, EDU08 Tutorial-Schritte lückenlos).

use crate::document::UnitType;
use crate::family_a::{DocProfile, DomainRule};
use crate::family_a_domains::{
    chain_negatives, chain_reference, chained_domain, relation_domain, relation_negatives,
    relation_reference,
};

relation_domain!(
    edu01,
    "EDU01-curriculum",
    "Lehrplan",
    "requires",
    "prerequisite_gap",
    "Lernziel A mit Voraussetzung",
    "Lernziel B mit Voraussetzung",
    "Grundlagenmodul",
    &[".md", ".docx"]
);
relation_domain!(
    edu02,
    "EDU02-lesson",
    "Lektion",
    "activates",
    "goalless_activity",
    "Ziel A mit Aktivitaet",
    "Ziel B mit Aktivitaet",
    "Aktivitaetenpool",
    &[".md"]
);
relation_domain!(
    edu03,
    "EDU03-quiz",
    "Quiz",
    "aligns",
    "unaligned_question",
    "Frage A mit Lernzielbezug",
    "Frage B mit Lernzielbezug",
    "Lernzielkatalog",
    &[".md"]
);
relation_domain!(
    edu05,
    "EDU05-learningpath",
    "Lernpfad",
    "depends",
    "orphan_module",
    "Modul A verknuepft",
    "Modul B verknuepft",
    "Basismodul",
    &[".md"]
);
relation_domain!(
    edu06,
    "EDU06-rubric",
    "Bewertungsraster",
    "levels",
    "ambiguous_level",
    "Kriterium A mit Niveau",
    "Kriterium B mit Niveau",
    "Niveaustufen",
    &[".md"]
);
relation_domain!(
    edu07,
    "EDU07-syllabus",
    "Syllabus",
    "aims",
    "uncovered_objective",
    "Einheit A mit Ziel",
    "Einheit B mit Ziel",
    "Kursziele",
    &[".md", ".pdf"]
);
relation_domain!(
    edu09,
    "EDU09-casestudy",
    "Fallstudie",
    "teaches",
    "pointless_detail",
    "Element A mit Lernzielbezug",
    "Element B mit Lernzielbezug",
    "Lernzielrahmen",
    &[".md"]
);
relation_domain!(
    edu10,
    "EDU10-competency",
    "Kompetenzmodell",
    "derives",
    "unobservable_competency",
    "Kompetenz A beobachtbar abgeleitet",
    "Kompetenz B beobachtbar abgeleitet",
    "Verhaltensanker",
    &[".md"]
);
relation_domain!(
    edu11,
    "EDU11-flashcards",
    "Flashcards",
    "concepts",
    "ambiguous_card",
    "Karte A mit Konzeptbezug",
    "Karte B mit Konzeptbezug",
    "Konzeptregister",
    &[".md"]
);
relation_domain!(
    edu12,
    "EDU12-trainingmanual",
    "Schulungshandbuch",
    "aims",
    "uncovered_topic",
    "Modul A mit Ziel",
    "Modul B mit Ziel",
    "Themenkatalog",
    &[".md", ".pdf"]
);

chained_domain!(
    edu04,
    "EDU04-exercises",
    "Uebungsset",
    "harder",
    "difficulty_jump",
    "Aufgabe leicht",
    "Aufgabe mittel",
    "Aufgabe schwer",
    &[".md"]
);
chained_domain!(
    edu08,
    "EDU08-tutorial",
    "Tutorial",
    "next",
    "gap_in_steps",
    "Schritt Einrichtung",
    "Schritt Konfiguration",
    "Schritt Abschluss",
    &[".md"]
);

/// Alle 12 Profile der Familie K (EDU01–12).
pub fn all_profiles() -> Vec<DocProfile> {
    vec![
        edu01(),
        edu02(),
        edu03(),
        edu04(),
        edu05(),
        edu06(),
        edu07(),
        edu08(),
        edu09(),
        edu10(),
        edu11(),
        edu12(),
    ]
}
