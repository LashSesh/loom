//! Familie I — Produkt/Business/Markt (BUS01–15) über dem Familien-
//! Kern `family_a`. Alle 15 sind Relation-Regeln (Subjekt·Begründungs-/
//! Evidenz-/Wert-Naht → Kern-Residuum aus S1 K.4).

use crate::document::UnitType;
use crate::family_a::{DocProfile, DomainRule};
use crate::family_a_domains::{relation_domain, relation_negatives, relation_reference};

relation_domain!(
    bus01,
    "BUS01-businesscase",
    "Business-Case",
    "justifies",
    "unbacked_assumption",
    "Annahme A begruendet",
    "Annahme B begruendet",
    "Begruendungsbasis",
    &[".md", ".docx"]
);
relation_domain!(
    bus02,
    "BUS02-marketanalysis",
    "Marktanalyse",
    "evidences",
    "unsourced_claim",
    "Segment A mit Evidenz",
    "Segment B mit Evidenz",
    "Datenquelle",
    &[".md", ".docx"]
);
relation_domain!(
    bus03,
    "BUS03-competitive",
    "Wettbewerbsanalyse",
    "compares",
    "apples_oranges",
    "Wettbewerber A vergleichbar",
    "Wettbewerber B vergleichbar",
    "Vergleichsrahmen",
    &[".md"]
);
relation_domain!(
    bus04,
    "BUS04-prd",
    "Produktanforderungen",
    "benefits",
    "unjustified_feature",
    "Anforderung A mit Nutzen",
    "Anforderung B mit Nutzen",
    "Nutzenmodell",
    &[".md"]
);
relation_domain!(
    bus05,
    "BUS05-gtm",
    "Go-to-Market",
    "channels",
    "untargeted_segment",
    "Massnahme A mit Kanal",
    "Massnahme B mit Kanal",
    "Kanalplan",
    &[".md"]
);
relation_domain!(
    bus06,
    "BUS06-pricing",
    "Preismodell",
    "values",
    "negative_margin",
    "Stufe A wertgedeckt",
    "Stufe B wertgedeckt",
    "Wertbasis",
    &[".md"]
);
relation_domain!(
    bus07,
    "BUS07-canvas",
    "Geschaeftsmodell-Canvas",
    "refers",
    "missing_revenue_stream",
    "Baustein A verknuepft",
    "Baustein B verknuepft",
    "Ertragsstrom",
    &[".md"]
);
relation_domain!(
    bus08,
    "BUS08-swot",
    "SWOT-Analyse",
    "derives",
    "actionless_factor",
    "Faktor A mit Ableitung",
    "Faktor B mit Ableitung",
    "Massnahmenblock",
    &[".md"]
);
relation_domain!(
    bus09,
    "BUS09-okr",
    "OKR-Set",
    "measures",
    "unmeasurable_kr",
    "Objective A mit KR",
    "Objective B mit KR",
    "Messgroessen",
    &[".md"]
);
relation_domain!(
    bus10,
    "BUS10-pitch",
    "Investoren-Pitch",
    "backs",
    "unbacked_claim",
    "Aussage A belegt",
    "Aussage B belegt",
    "Beleglage",
    &[".pdf"]
);
relation_domain!(
    bus11,
    "BUS11-valueprop",
    "Value Proposition",
    "needs",
    "value_gap",
    "Nutzen A mit Beduerfnisbezug",
    "Nutzen B mit Beduerfnisbezug",
    "Beduerfnisanalyse",
    &[".md"]
);
relation_domain!(
    bus12,
    "BUS12-segmentation",
    "Kundensegmentierung",
    "criteria",
    "overlapping_segments",
    "Segment A kriterienscharf",
    "Segment B kriterienscharf",
    "Kriterienset",
    &[".md"]
);
relation_domain!(
    bus13,
    "BUS13-forecast",
    "Umsatzprognose",
    "formulates",
    "unfounded_projection",
    "Treiber A mit Formelbezug",
    "Treiber B mit Formelbezug",
    "Prognosemodell",
    &[".md"]
);
relation_domain!(
    bus14,
    "BUS14-partnership",
    "Partnerschafts-Vorschlag",
    "reciprocates",
    "one_sided_deal",
    "Beitrag A mit Gegenleistung",
    "Beitrag B mit Gegenleistung",
    "Gegenleistungsrahmen",
    &[".md", ".docx"]
);
relation_domain!(
    bus15,
    "BUS15-roadmap",
    "Produkt-Roadmap",
    "values",
    "valueless_initiative",
    "Initiative A wertgebunden",
    "Initiative B wertgebunden",
    "Wertmodell",
    &[".md"]
);

/// Alle 15 Profile der Familie I (BUS01–15).
pub fn all_profiles() -> Vec<DocProfile> {
    vec![
        bus01(),
        bus02(),
        bus03(),
        bus04(),
        bus05(),
        bus06(),
        bus07(),
        bus08(),
        bus09(),
        bus10(),
        bus11(),
        bus12(),
        bus13(),
        bus14(),
        bus15(),
    ]
}
