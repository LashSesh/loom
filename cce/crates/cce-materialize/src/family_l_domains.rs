//! Familie L — Kreativ/Medien/Design (CRE01–15) über dem Familien-Kern
//! `family_a`. 10 Relation + 1 Azyklik (FIN-artige Zirkelfreiheit nicht
//! noetig) — konkret: 10 Relation + 5 Ketten (CRE02 Beats, CRE04
//! UX-Flow, CRE08 Kalender, CRE10 Storyboard, CRE13 Skript-Segmente:
//! alles Fluss-/Reihenfolge-Naehte). Kern-Residuen aus S1 K.4.

use crate::document::UnitType;
use crate::family_a::{DocProfile, DomainRule};
use crate::family_a_domains::{
    chain_negatives, chain_reference, chained_domain, relation_domain, relation_negatives,
    relation_reference,
};

relation_domain!(
    cre01,
    "CRE01-story",
    "Story",
    "causes",
    "plot_hole",
    "Szene A kausal gebunden",
    "Szene B kausal gebunden",
    "Ausloeser-Szene",
    &[".md"]
);
relation_domain!(
    cre03,
    "CRE03-designsystem",
    "Design-System",
    "uses",
    "inconsistent_token",
    "Token A genutzt",
    "Token B genutzt",
    "Basistoken",
    &[".md"]
);
relation_domain!(
    cre05,
    "CRE05-campaign",
    "Kampagne",
    "channels",
    "off_message",
    "Botschaft A kanalgebunden",
    "Botschaft B kanalgebunden",
    "Kernbotschaft",
    &[".md"]
);
relation_domain!(
    cre06,
    "CRE06-moodboard",
    "Moodboard",
    "coheres",
    "incoherent_mood",
    "Motiv A zusammenhaengend",
    "Motiv B zusammenhaengend",
    "Stimmungskern",
    &[".md"]
);
relation_domain!(
    cre07,
    "CRE07-worldbuilding",
    "World-Building",
    "consistent",
    "contradiction",
    "Entitaet A konsistenzgebunden",
    "Entitaet B konsistenzgebunden",
    "Weltregeln",
    &[".md"]
);
relation_domain!(
    cre09,
    "CRE09-naming",
    "Naming-Set",
    "criteria",
    "trademark_clash",
    "Kandidat A kriteriengeprueft",
    "Kandidat B kriteriengeprueft",
    "Pruefkriterien",
    &[".md"]
);
relation_domain!(
    cre11,
    "CRE11-lyrics",
    "Songtext",
    "flows",
    "reproduction",
    "Abschnitt A fluss-/reimgebunden",
    "Abschnitt B fluss-/reimgebunden",
    "Reimschema",
    &[".md"]
);
relation_domain!(
    cre12,
    "CRE12-poem",
    "Gedicht",
    "meters",
    "broken_meter",
    "Zeile A metrisch gebunden",
    "Zeile B metrisch gebunden",
    "Metrumschema",
    &[".md"]
);
relation_domain!(
    cre14,
    "CRE14-brand",
    "Marken-Identitaet",
    "consistent",
    "off_brand",
    "Element A markenkonsistent",
    "Element B markenkonsistent",
    "Markenkern",
    &[".md"]
);
relation_domain!(
    cre15,
    "CRE15-layout",
    "Layout",
    "aligns",
    "visual_imbalance",
    "Element A ausgerichtet",
    "Element B ausgerichtet",
    "Raster",
    &[".md"]
);

chained_domain!(
    cre02,
    "CRE02-script",
    "Drehbuch",
    "flow",
    "dangling_beat",
    "Opening-Beat",
    "Mittel-Beat",
    "Aufloesungs-Beat",
    &[".md"]
);
chained_domain!(
    cre04,
    "CRE04-uxflow",
    "UX-Flow",
    "transition",
    "dead_end_screen",
    "Start-Screen",
    "Eingabe-Screen",
    "Abschluss-Screen",
    &[".md"]
);
chained_domain!(
    cre08,
    "CRE08-contentcalendar",
    "Content-Kalender",
    "schedule",
    "scheduling_gap",
    "Beitrag Woche 1",
    "Beitrag Woche 2",
    "Beitrag Woche 3",
    &[".md"]
);
chained_domain!(
    cre10,
    "CRE10-storyboard",
    "Storyboard",
    "flow",
    "continuity_break",
    "Panel Anfang",
    "Panel Mitte",
    "Panel Ende",
    &[".md"]
);
chained_domain!(
    cre13,
    "CRE13-podcast",
    "Podcast-Skript",
    "transition",
    "abrupt_transition",
    "Intro-Segment",
    "Haupt-Segment",
    "Outro-Segment",
    &[".md"]
);

/// Alle 15 Profile der Familie L (CRE01–15).
pub fn all_profiles() -> Vec<DocProfile> {
    vec![
        cre01(),
        cre02(),
        cre03(),
        cre04(),
        cre05(),
        cre06(),
        cre07(),
        cre08(),
        cre09(),
        cre10(),
        cre11(),
        cre12(),
        cre13(),
        cre14(),
        cre15(),
    ]
}
