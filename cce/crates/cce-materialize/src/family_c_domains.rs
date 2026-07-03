//! Familie C — Daten/Analytics (DATA01–12) über dem Familien-Kern
//! `family_a`. 11 Relation-Regeln + 1 Kette (DATA03 ETL-Lineage
//! lückenlos). Kern-Residuen aus S1 K.4. Kennzahlen ordnen, Gates
//! entscheiden (V1 geerbt) — auch fuer KPI/Statistik-Domaenen.

use crate::document::UnitType;
use crate::family_a::{DocProfile, DomainRule};
use crate::family_a_domains::{
    chain_negatives, chain_reference, chained_domain, relation_domain, relation_negatives,
    relation_reference,
};

relation_domain!(
    data01,
    "DATA01-cleaning",
    "Datenbereinigung",
    "rules",
    "unhandled_null",
    "Feld A mit Null-Regel",
    "Feld B mit Null-Regel",
    "Regelwerk",
    &[".md"]
);
relation_domain!(
    data02,
    "DATA02-sql",
    "SQL-Query",
    "joins",
    "cartesian_blowup",
    "Klausel A mit Join-Bedingung",
    "Klausel B mit Join-Bedingung",
    "Join-Schluessel",
    &[".md"]
);
relation_domain!(
    data04,
    "DATA04-dashboard",
    "Dashboard-Spec",
    "metrics",
    "undefined_metric",
    "Widget A mit Metrikbezug",
    "Widget B mit Metrikbezug",
    "Metrikdefinition",
    &[".md"]
);
relation_domain!(
    data05,
    "DATA05-datamodel",
    "Datenmodell",
    "relates",
    "orphan_entity",
    "Entitaet A verknuepft",
    "Entitaet B verknuepft",
    "Beziehungsknoten",
    &[".md"]
);
relation_domain!(
    data06,
    "DATA06-statistics",
    "Statistische Analyse",
    "assumes",
    "violated_assumption",
    "Test A mit Annahmenpruefung",
    "Test B mit Annahmenpruefung",
    "Annahmenkatalog",
    &[".md"]
);
relation_domain!(
    data07,
    "DATA07-features",
    "Feature-Engineering",
    "sources",
    "data_leakage",
    "Feature A quellensauber",
    "Feature B quellensauber",
    "Trainingsquellen",
    &[".md"]
);
relation_domain!(
    data08,
    "DATA08-dqrules",
    "Datenqualitaets-Regelwerk",
    "checks",
    "unchecked_field",
    "Regel A prueft Feld",
    "Regel B prueft Feld",
    "Feldkatalog",
    &[".md"]
);
relation_domain!(
    data09,
    "DATA09-chart",
    "Visualisierung",
    "encodes",
    "misleading_scale",
    "Kanal A datengebunden",
    "Kanal B datengebunden",
    "Datenbasis",
    &[".md"]
);
relation_domain!(
    data10,
    "DATA10-kpi",
    "KPI-Definition",
    "formulates",
    "ambiguous_kpi",
    "KPI A mit Formel",
    "KPI B mit Formel",
    "Formelregister",
    &[".md"]
);
relation_domain!(
    data11,
    "DATA11-catalog",
    "Datenkatalog",
    "provenance",
    "undocumented_source",
    "Asset A mit Herkunft",
    "Asset B mit Herkunft",
    "Herkunftsregister",
    &[".md"]
);
relation_domain!(
    data12,
    "DATA12-sampling",
    "Sampling/Kohorte",
    "belongs",
    "selection_bias",
    "Kriterium A zugehoerigkeitsscharf",
    "Kriterium B zugehoerigkeitsscharf",
    "Kohortendefinition",
    &[".md"]
);

chained_domain!(
    data03,
    "DATA03-etl",
    "ETL-Pipeline",
    "lineage",
    "broken_lineage",
    "Stufe Extract",
    "Stufe Transform",
    "Stufe Load",
    &[".md"]
);

/// Alle 12 Profile der Familie C (DATA01–12).
pub fn all_profiles() -> Vec<DocProfile> {
    vec![
        data01(),
        data02(),
        data03(),
        data04(),
        data05(),
        data06(),
        data07(),
        data08(),
        data09(),
        data10(),
        data11(),
        data12(),
    ]
}
