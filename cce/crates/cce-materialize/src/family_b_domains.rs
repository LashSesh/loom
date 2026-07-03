//! Familie B — Software Engineering (SWE01–15) über dem Familien-Kern
//! `family_a`. Unit = Code-/Spec-Einheit, Seam = Aufruf-/Vertrags-/
//! Abhängigkeits-Beziehung; `equivalent` bleibt Klassenvergleich (nicht
//! Formatierung — der Kern normalisiert Whitespace). 12 Relation +
//! 1 Azyklik (SWE07 Architektur) + 2 Ketten (SWE05 Pipeline-Stufen,
//! fail-closed lückenlos).

use crate::document::UnitType;
use crate::family_a::{DocProfile, DomainRule};
use crate::family_a_domains::{
    acyclic_domain, acyclic_negatives, acyclic_reference, chain_negatives, chain_reference,
    chained_domain, relation_domain, relation_negatives, relation_reference,
};

relation_domain!(
    swe01,
    "SWE01-module",
    "Modul",
    "tested",
    "untested_path",
    "Funktion A mit Testbezug",
    "Funktion B mit Testbezug",
    "Testsuite",
    &[".md"]
);
relation_domain!(
    swe02,
    "SWE02-apispec",
    "API-Spezifikation",
    "schema",
    "undefined_response",
    "Endpoint A mit Schema",
    "Endpoint B mit Schema",
    "Schemadefinition",
    &[".md"]
);
relation_domain!(
    swe03,
    "SWE03-testsuite",
    "Test-Suite",
    "covers",
    "uncovered_requirement",
    "Testfall A deckt Anforderung",
    "Testfall B deckt Anforderung",
    "Anforderungsliste",
    &[".md"]
);
relation_domain!(
    swe04,
    "SWE04-dbschema",
    "DB-Schema",
    "references",
    "orphan_fk",
    "Tabelle A mit FK-Bezug",
    "Tabelle B mit FK-Bezug",
    "Zieltabelle",
    &[".md"]
);
relation_domain!(
    swe06,
    "SWE06-refactoring",
    "Refactoring-Plan",
    "preserves",
    "behavior_change",
    "Schritt A verhaltenswahrend",
    "Schritt B verhaltenswahrend",
    "Verhaltensinventar",
    &[".md"]
);
relation_domain!(
    swe08,
    "SWE08-bugfix",
    "Bugfix",
    "causes",
    "regression",
    "Aenderung A mit Ursachenbezug",
    "Aenderung B mit Ursachenbezug",
    "Ursachenanalyse",
    &[".md"]
);
relation_domain!(
    swe09,
    "SWE09-config",
    "Konfiguration",
    "scopes",
    "missing_key",
    "Schluessel A mit Geltung",
    "Schluessel B mit Geltung",
    "Geltungsrahmen",
    &[".md"]
);
relation_domain!(
    swe10,
    "SWE10-cli",
    "CLI-Tool",
    "documents",
    "undocumented_flag",
    "Kommando A mit Flag-Doku",
    "Kommando B mit Flag-Doku",
    "Flag-Referenz",
    &[".md"]
);
relation_domain!(
    swe11,
    "SWE11-library",
    "Library",
    "contracts",
    "breaking_change",
    "Public-API A vertragsgebunden",
    "Public-API B vertragsgebunden",
    "API-Vertrag",
    &[".md"]
);
relation_domain!(
    swe12,
    "SWE12-review",
    "Code-Review-Bericht",
    "locates",
    "unlocated_finding",
    "Befund A verortet",
    "Befund B verortet",
    "Fundstellenregister",
    &[".md"]
);
relation_domain!(
    swe13,
    "SWE13-sbom",
    "SBOM",
    "licenses",
    "unknown_license",
    "Abhaengigkeit A mit Lizenz",
    "Abhaengigkeit B mit Lizenz",
    "Lizenzregister",
    &[".md"]
);
relation_domain!(
    swe14,
    "SWE14-iac",
    "Infrastructure-as-Code",
    "refers",
    "drift",
    "Ressource A mit Bezug",
    "Ressource B mit Bezug",
    "Sollzustand",
    &[".md"]
);
relation_domain!(
    swe15,
    "SWE15-grammar",
    "Grammatik/Parser",
    "derives",
    "ambiguous_grammar",
    "Regel A eindeutig ableitbar",
    "Regel B eindeutig ableitbar",
    "Startsymbol",
    &[".md"]
);

acyclic_domain!(
    swe07,
    "SWE07-architecture",
    "Architektur",
    "interfaces",
    "dependency_cycle",
    "Komponente A mit Schnittstelle",
    "Komponente B mit Schnittstelle",
    "Schnittstellenvertrag",
    &[".md"]
);
chained_domain!(
    swe05,
    "SWE05-pipeline",
    "CI-Pipeline",
    "then",
    "missing_gate",
    "Stufe Build",
    "Stufe Test",
    "Stufe Gate/Release",
    &[".md"]
);

/// Alle 15 Profile der Familie B (SWE01–15).
pub fn all_profiles() -> Vec<DocProfile> {
    vec![
        swe01(),
        swe02(),
        swe03(),
        swe04(),
        swe05(),
        swe06(),
        swe07(),
        swe08(),
        swe09(),
        swe10(),
        swe11(),
        swe12(),
        swe13(),
        swe14(),
        swe15(),
    ]
}
