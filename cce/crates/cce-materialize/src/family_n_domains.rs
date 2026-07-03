//! Familie N — Kommunikation/CRM/Organisation (COM01–12) über dem
//! Familien-Kern `family_a`. 10 Relation-Regeln + 2 Ketten (COM05
//! Newsletter-Fluss, COM09 Onboarding-Zeit). Kern-Residuen aus S1 K.4.

use crate::document::UnitType;
use crate::family_a::{DocProfile, DomainRule};
use crate::family_a_domains::{
    chain_negatives, chain_reference, chained_domain, relation_domain, relation_negatives,
    relation_reference,
};

relation_domain!(
    com01,
    "COM01-message",
    "Nachricht",
    "purpose",
    "purposeless_message",
    "Absatz A mit Zweck",
    "Absatz B mit Zweck",
    "Zweckangabe",
    &[".md", ".docx"]
);
relation_domain!(
    com02,
    "COM02-agenda",
    "Agenda",
    "aims",
    "goalless_item",
    "Punkt A mit Ziel",
    "Punkt B mit Ziel",
    "Sitzungsziel",
    &[".md"]
);
relation_domain!(
    com03,
    "COM03-crm",
    "CRM-Kontakt",
    "relates",
    "orphan_contact",
    "Kontakt A verknuepft",
    "Kontakt B verknuepft",
    "Beziehungsknoten",
    &[".md"]
);
relation_domain!(
    com04,
    "COM04-commsplan",
    "Kommunikationsplan",
    "targets",
    "unreached_audience",
    "Botschaft A mit Zielgruppe",
    "Botschaft B mit Zielgruppe",
    "Zielgruppenliste",
    &[".md", ".docx"]
);
relation_domain!(
    com06,
    "COM06-stakeholdermap",
    "Stakeholder-Map",
    "interests",
    "unmapped_stakeholder",
    "Stakeholder A mit Interesse",
    "Stakeholder B mit Interesse",
    "Interessenlage",
    &[".md"]
);
relation_domain!(
    com07,
    "COM07-replytemplate",
    "Antwort-Vorlage",
    "covers",
    "uncovered_case",
    "Baustein A deckt Fall",
    "Baustein B deckt Fall",
    "Fallkatalog",
    &[".md"]
);
relation_domain!(
    com08,
    "COM08-escalation",
    "Eskalations-Kommunikation",
    "notifies",
    "missed_recipient",
    "Stufe A mit Empfaenger",
    "Stufe B mit Empfaenger",
    "Empfaengerkreis",
    &[".md"]
);
relation_domain!(
    com10,
    "COM10-feedback",
    "Kundenfeedback-Synthese",
    "themes",
    "lost_signal",
    "Feedback A thematisiert",
    "Feedback B thematisiert",
    "Themencluster",
    &[".md"]
);
relation_domain!(
    com11,
    "COM11-announcement",
    "Interne Ankuendigung",
    "refers",
    "ambiguous_notice",
    "Aussage A mit Bezug",
    "Aussage B mit Bezug",
    "Bezugsrahmen",
    &[".md", ".docx"]
);
relation_domain!(
    com12,
    "COM12-orgstructure",
    "Verteiler-/Org-Struktur",
    "assigns",
    "orphan_unit",
    "Einheit A zugeordnet",
    "Einheit B zugeordnet",
    "Zuordnungsknoten",
    &[".md"]
);

chained_domain!(
    com05,
    "COM05-newsletter",
    "Newsletter",
    "flow",
    "missing_cta",
    "Aufmacher-Sektion",
    "Inhalts-Sektion",
    "Call-to-Action-Sektion",
    &[".md"]
);
chained_domain!(
    com09,
    "COM09-onboarding",
    "Onboarding",
    "after",
    "onboarding_gap",
    "Willkommen Tag 1",
    "Einrichtung Tag 2",
    "Erste Aufgabe Tag 3",
    &[".md", ".docx"]
);

/// Alle 12 Profile der Familie N (COM01–12).
pub fn all_profiles() -> Vec<DocProfile> {
    vec![
        com01(),
        com02(),
        com03(),
        com04(),
        com05(),
        com06(),
        com07(),
        com08(),
        com09(),
        com10(),
        com11(),
        com12(),
    ]
}
