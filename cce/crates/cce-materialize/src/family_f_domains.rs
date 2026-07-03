//! Familie F — Projekt/Prozess/Workflow (PM01–15) über dem Familien-
//! Kern `family_a`. 12 Relation + 1 Azyklik (PM01 Abhängigkeit) +
//! 2 Ketten (PM02 Prozessfluss, PM13 Eskalationspfad).

use crate::document::UnitType;
use crate::family_a::{DocProfile, DomainRule};
use crate::family_a_domains::{
    acyclic_domain, acyclic_negatives, acyclic_reference, chain_negatives, chain_reference,
    chained_domain, relation_domain, relation_negatives, relation_reference,
};

acyclic_domain!(
    pm01,
    "PM01-gantt",
    "Projektplan",
    "depends",
    "dependency_cycle",
    "Aufgabe A mit Abhaengigkeit",
    "Aufgabe B mit Abhaengigkeit",
    "Startaufgabe",
    &[".md", ".pdf"]
);
chained_domain!(
    pm02,
    "PM02-process",
    "Prozess",
    "flow",
    "no_end_event",
    "Start-Schritt",
    "Zwischen-Schritt",
    "End-Ereignis",
    &[".md"]
);
relation_domain!(
    pm03,
    "PM03-riskregister",
    "Risikoregister",
    "mitigates",
    "unmitigated_risk",
    "Risiko A mit Massnahme",
    "Risiko B mit Massnahme",
    "Massnahmenplan",
    &[".md"]
);
relation_domain!(
    pm04,
    "PM04-milestones",
    "Meilenstein-Plan",
    "depends",
    "orphan_milestone",
    "Meilenstein A verknuepft",
    "Meilenstein B verknuepft",
    "Basismeilenstein",
    &[".md"]
);
relation_domain!(
    pm05,
    "PM05-backlog",
    "Sprint-Backlog",
    "estimates",
    "unestimated_story",
    "Story A geschaetzt",
    "Story B geschaetzt",
    "Schaetzbasis",
    &[".md"]
);
relation_domain!(
    pm06,
    "PM06-raci",
    "RACI-Matrix",
    "accountable",
    "missing_accountable",
    "Aufgabe A mit Verantwortlichem",
    "Aufgabe B mit Verantwortlichem",
    "Rollenmodell",
    &[".md"]
);
relation_domain!(
    pm07,
    "PM07-resourceplan",
    "Ressourcenplan",
    "assigns",
    "overallocation",
    "Ressource A zugewiesen",
    "Ressource B zugewiesen",
    "Kapazitaetsbasis",
    &[".md"]
);
relation_domain!(
    pm08,
    "PM08-statusreport",
    "Statusbericht",
    "backs",
    "unbacked_status",
    "Kennzahl A belegt",
    "Kennzahl B belegt",
    "Datengrundlage",
    &[".md", ".docx"]
);
relation_domain!(
    pm09,
    "PM09-retro",
    "Retrospektive",
    "acts",
    "actionless_finding",
    "Befund A mit Aktion",
    "Befund B mit Aktion",
    "Massnahmenliste",
    &[".md"]
);
relation_domain!(
    pm10,
    "PM10-decision",
    "Entscheidungsvorlage",
    "decides",
    "undecided_criterion",
    "Option A mit Kriterium",
    "Option B mit Kriterium",
    "Kriterienkatalog",
    &[".md", ".docx"]
);
relation_domain!(
    pm11,
    "PM11-changerequest",
    "Change-Request",
    "assesses",
    "unassessed_impact",
    "Aenderung A mit Impact",
    "Aenderung B mit Impact",
    "Impact-Analyse",
    &[".md"]
);
relation_domain!(
    pm12,
    "PM12-dod",
    "Abnahmekriterien",
    "tests",
    "untestable_criterion",
    "Kriterium A pruefbar",
    "Kriterium B pruefbar",
    "Pruefverfahren",
    &[".md"]
);
chained_domain!(
    pm13,
    "PM13-escalationpath",
    "Eskalationspfad",
    "handover",
    "dead_end_escalation",
    "Stufe 1 Team",
    "Stufe 2 Lead",
    "Stufe 3 Management",
    &[".md"]
);
relation_domain!(
    pm14,
    "PM14-capacityplan",
    "Kapazitaetsplan",
    "demands",
    "capacity_gap",
    "Einheit A mit Bedarf",
    "Einheit B mit Bedarf",
    "Bedarfsbasis",
    &[".md"]
);
relation_domain!(
    pm15,
    "PM15-roadmap",
    "Roadmap",
    "depends",
    "orphan_initiative",
    "Initiative A verknuepft",
    "Initiative B verknuepft",
    "Basisinitiative",
    &[".md", ".pdf"]
);

/// Alle 15 Profile der Familie F (PM01–15).
pub fn all_profiles() -> Vec<DocProfile> {
    vec![
        pm01(),
        pm02(),
        pm03(),
        pm04(),
        pm05(),
        pm06(),
        pm07(),
        pm08(),
        pm09(),
        pm10(),
        pm11(),
        pm12(),
        pm13(),
        pm14(),
        pm15(),
    ]
}
