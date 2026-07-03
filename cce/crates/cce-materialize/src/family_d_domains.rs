//! Familie D — Graph/Netzwerk/Struktur (GRA01–12) über dem Familien-
//! Kern `family_a` — die Naht-Grammatik IST hier die Domäne selbst.
//! 8 Relation + 2 Azyklik (GRA02 is-a-Hierarchie, GRA04 Dependencies) +
//! 2 Ketten (GRA06 Workflow-DAG-Fluss, GRA03 FSM-Erreichbarkeit als
//! lückenlose Übergangskette vom Start).

use crate::document::UnitType;
use crate::family_a::{DocProfile, DomainRule};
use crate::family_a_domains::{
    acyclic_domain, acyclic_negatives, acyclic_reference, chain_negatives, chain_reference,
    chained_domain, relation_domain, relation_negatives, relation_reference,
};

relation_domain!(
    gra01,
    "GRA01-graph",
    "Graph",
    "edge",
    "dangling_edge",
    "Knoten A mit aufloesbarer Kante",
    "Knoten B mit aufloesbarer Kante",
    "Zielknoten",
    &[".md"]
);
relation_domain!(
    gra05,
    "GRA05-ermodel",
    "ER-Modell",
    "cardinality",
    "undefined_cardinality",
    "Entitaet A mit Kardinalitaet",
    "Entitaet B mit Kardinalitaet",
    "Relationsdefinition",
    &[".md"]
);
relation_domain!(
    gra07,
    "GRA07-mindmap",
    "Konzeptnetz",
    "associates",
    "isolated_node",
    "Konzept A assoziiert",
    "Konzept B assoziiert",
    "Zentralkonzept",
    &[".md"]
);
relation_domain!(
    gra08,
    "GRA08-topology",
    "Netzwerk-Topologie",
    "connects",
    "single_point_of_failure",
    "Knoten A redundant verbunden",
    "Knoten B redundant verbunden",
    "Redundanzpfad",
    &[".md"]
);
relation_domain!(
    gra09,
    "GRA09-tree",
    "Baum/Hierarchie",
    "parent",
    "multiple_parents",
    "Knoten A mit Eltern-Naht",
    "Knoten B mit Eltern-Naht",
    "Wurzelknoten",
    &[".md"]
);
relation_domain!(
    gra10,
    "GRA10-petrinet",
    "Petri-Netz",
    "fires",
    "deadlock",
    "Transition A feuerbar",
    "Transition B feuerbar",
    "Markierung",
    &[".md"]
);
relation_domain!(
    gra11,
    "GRA11-causal",
    "Kausaldiagramm",
    "causes",
    "unmeasured_confounder",
    "Faktor A kausal gebunden",
    "Faktor B kausal gebunden",
    "Messgroesse",
    &[".md"]
);
relation_domain!(
    gra12,
    "GRA12-routing",
    "Routing",
    "weighted",
    "unreachable_target",
    "Knoten A mit gewichteter Kante",
    "Knoten B mit gewichteter Kante",
    "Zielknoten",
    &[".md"]
);

acyclic_domain!(
    gra02,
    "GRA02-ontology",
    "Ontologie",
    "isa",
    "cycle_in_hierarchy",
    "Begriff A is-a",
    "Begriff B is-a",
    "Oberbegriff",
    &[".md"]
);
acyclic_domain!(
    gra04,
    "GRA04-depgraph",
    "Abhaengigkeitsgraph",
    "depends",
    "dependency_cycle",
    "Einheit A abhaengig",
    "Einheit B abhaengig",
    "Basiseinheit",
    &[".md"]
);

chained_domain!(
    gra03,
    "GRA03-fsm",
    "Zustandsautomat",
    "transition",
    "unreachable_state",
    "Startzustand",
    "Arbeitszustand",
    "Endzustand",
    &[".md"]
);
chained_domain!(
    gra06,
    "GRA06-workflow",
    "Workflow-DAG",
    "flow",
    "orphan_step",
    "Eingang",
    "Verarbeitung",
    "Abschluss",
    &[".md"]
);

/// Alle 12 Profile der Familie D (GRA01–12).
pub fn all_profiles() -> Vec<DocProfile> {
    vec![
        gra01(),
        gra02(),
        gra03(),
        gra04(),
        gra05(),
        gra06(),
        gra07(),
        gra08(),
        gra09(),
        gra10(),
        gra11(),
        gra12(),
    ]
}
