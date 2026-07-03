//! Familie E — Mathematik/Formale Struktur (MATH01–15) über dem
//! Familien-Kern `family_a`. CLAIM-SCHRANKE (V10/INV-14) besonders
//! wachsam: geprüft wird die STRUKTUR (Ableit-/Kopplungs-/Prämissen-
//! Nähte vorhanden, azyklisch, lückenlos) — NIE die mathematische
//! Wahrheit eines Inhalts. Ein grünes Gate heißt „strukturell
//! wohlgeformt", nicht „bewiesen". 11 Relation + 2 Azyklik (MATH05
//! Definitionen, MATH10 Regelprämissen) + 2 Ketten (MATH01 Beweis-
//! Schritte, MATH04 Kontrollfluss).

use crate::document::UnitType;
use crate::family_a::{DocProfile, DomainRule};
use crate::family_a_domains::{
    acyclic_domain, acyclic_negatives, acyclic_reference, chain_negatives, chain_reference,
    chained_domain, relation_domain, relation_negatives, relation_reference,
};

relation_domain!(
    math02,
    "MATH02-equations",
    "Gleichungssystem",
    "couples",
    "inconsistent_system",
    "Gleichung A gekoppelt",
    "Gleichung B gekoppelt",
    "Variablenraum",
    &[".md"]
);
relation_domain!(
    math03,
    "MATH03-optimization",
    "Optimierungsmodell",
    "constrains",
    "infeasible",
    "Variable A mit Constraint",
    "Variable B mit Constraint",
    "Zulaessigkeitsbereich",
    &[".md"]
);
relation_domain!(
    math06,
    "MATH06-theorem",
    "Theorem-Kette",
    "uses",
    "unused_hypothesis",
    "Satz A nutzt Hypothese",
    "Satz B nutzt Hypothese",
    "Hypothesenblock",
    &[".md"]
);
relation_domain!(
    math07,
    "MATH07-combinatorics",
    "Kombinatorik",
    "incident",
    "double_counting",
    "Objekt A inzidenzscharf",
    "Objekt B inzidenzscharf",
    "Zaehlbasis",
    &[".md"]
);
relation_domain!(
    math08,
    "MATH08-probability",
    "Wahrscheinlichkeitsmodell",
    "depends",
    "unnormalized",
    "Ereignis A abhaengigkeitsklar",
    "Ereignis B abhaengigkeitsklar",
    "Ereignisraum",
    &[".md"]
);
relation_domain!(
    math09,
    "MATH09-sat",
    "Logische Formel",
    "binds",
    "contradiction",
    "Klausel A variablengebunden",
    "Klausel B variablengebunden",
    "Variablenmenge",
    &[".md"]
);
relation_domain!(
    math11,
    "MATH11-category",
    "Kategorielle Konstruktion",
    "composes",
    "noncommuting_diagram",
    "Morphismus A komponierbar",
    "Morphismus B komponierbar",
    "Zielobjekt",
    &[".md"]
);
relation_domain!(
    math12,
    "MATH12-ode",
    "Differentialgleichung",
    "bounds",
    "ill_posed",
    "Term A mit Randbedingung",
    "Term B mit Randbedingung",
    "Randdaten",
    &[".md"]
);
relation_domain!(
    math13,
    "MATH13-group",
    "Gruppenstruktur",
    "operates",
    "not_closed",
    "Element A operationsgebunden",
    "Element B operationsgebunden",
    "Traegermenge",
    &[".md"]
);
relation_domain!(
    math14,
    "MATH14-numerics",
    "Numerisches Verfahren",
    "errors",
    "divergence",
    "Schritt A mit Fehlerschranke",
    "Schritt B mit Fehlerschranke",
    "Fehlermass",
    &[".md"]
);
relation_domain!(
    math15,
    "MATH15-proofsketch",
    "Beweis-Skizze",
    "gaps",
    "hidden_gap",
    "Kernidee A mit Lueckenausweis",
    "Kernidee B mit Lueckenausweis",
    "Lueckenregister",
    &[".md"]
);

acyclic_domain!(
    math05,
    "MATH05-definitions",
    "Formale Definitionen",
    "defines",
    "circular_definition",
    "Term A definiert ueber",
    "Term B definiert ueber",
    "Grundterm",
    &[".md"]
);
acyclic_domain!(
    math10,
    "MATH10-typesystem",
    "Typsystem",
    "premises",
    "broken_preservation",
    "Regel A mit Praemissen",
    "Regel B mit Praemissen",
    "Basisregel",
    &[".md"]
);

chained_domain!(
    math01,
    "MATH01-proof",
    "Beweis",
    "derives",
    "gap_in_proof",
    "Annahme",
    "Zwischenschritt",
    "Folgerung",
    &[".md"]
);
chained_domain!(
    math04,
    "MATH04-algorithm",
    "Algorithmus",
    "controlflow",
    "nontermination",
    "Initialisierung",
    "Schleifenschritt",
    "Terminierung",
    &[".md"]
);

/// Alle 15 Profile der Familie E (MATH01–15).
pub fn all_profiles() -> Vec<DocProfile> {
    vec![
        math01(),
        math02(),
        math03(),
        math04(),
        math05(),
        math06(),
        math07(),
        math08(),
        math09(),
        math10(),
        math11(),
        math12(),
        math13(),
        math14(),
        math15(),
    ]
}
