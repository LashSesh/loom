//! Familie J — Wissen/Forschung/Quellen (KNOW01–15) als reine Profil-
//! Spezialisierung über DEMSELBEN Familien-Kern `family_a` (Text/Naht-
//! Grammatik). Höchste Kern-Wiederverwendung (Wellenplan §2.5-b):
//! 14 Relation-Regeln + 1 Ketten-Regel (KNOW09 PRISMA-Fluss). Nur die
//! Kern-Naht + das Kern-Residuum (S1 K.4) unterscheiden die Domänen.

use crate::document::UnitType;
use crate::family_a::{DocProfile, DomainRule};
use crate::family_a_domains::{
    crystal, relation_domain, relation_negatives, relation_reference, section, subject,
};

relation_domain!(
    know01,
    "KNOW01-litreview",
    "Literaturuebersicht",
    "sources",
    "uncited_claim",
    "Aussage A quellenbelegt",
    "Aussage B quellenbelegt",
    "Quelle Studie",
    &[".md", ".docx"]
);
relation_domain!(
    know02,
    "KNOW02-grant",
    "Forschungsantrag",
    "method",
    "methodless_aim",
    "Ziel A mit Methode",
    "Ziel B mit Methode",
    "Methodenteil",
    &[".docx", ".pdf"]
);
relation_domain!(
    know03,
    "KNOW03-citenet",
    "Zitat-Netz",
    "cites",
    "dangling_citation",
    "Quelle A zitiert",
    "Quelle B zitiert",
    "Referenzliste",
    &[".md"]
);
relation_domain!(
    know04,
    "KNOW04-hypotheses",
    "Hypothesen",
    "tests",
    "untestable_hypothesis",
    "Hypothese A testbar",
    "Hypothese B testbar",
    "Testplan",
    &[".md"]
);
relation_domain!(
    know05,
    "KNOW05-experiment",
    "Experiment-Design",
    "controls",
    "confounded_design",
    "Faktor A kontrolliert",
    "Faktor B kontrolliert",
    "Kontrollgruppe",
    &[".md", ".docx"]
);
relation_domain!(
    know06,
    "KNOW06-metaanalysis",
    "Meta-Analyse",
    "weights",
    "selection_bias",
    "Studie A gewichtet",
    "Studie B gewichtet",
    "Ein-Ausschluss-Kriterien",
    &[".md"]
);
relation_domain!(
    know07,
    "KNOW07-annobib",
    "Annotierte Bibliographie",
    "annotates",
    "unannotated_source",
    "Quelle A annotiert",
    "Quelle B annotiert",
    "Annotationsschema",
    &[".md"]
);
relation_domain!(
    know08,
    "KNOW08-kbarticle",
    "Wissensbasis-Artikel",
    "supports",
    "unsupported_statement",
    "Aussage A belegt",
    "Aussage B belegt",
    "Belegsammlung",
    &[".md"]
);
relation_domain!(
    know10,
    "KNOW10-factcheck",
    "Faktencheck-Dossier",
    "evidence",
    "one_sided_check",
    "Behauptung A mit Beleg/Gegenbeleg",
    "Behauptung B mit Beleg/Gegenbeleg",
    "Evidenzblock",
    &[".md"]
);
relation_domain!(
    know11,
    "KNOW11-conceptmap",
    "Begriffs-Landkarte",
    "relates",
    "isolated_concept",
    "Begriff A verknuepft",
    "Begriff B verknuepft",
    "Bezugsbegriff",
    &[".md"]
);
relation_domain!(
    know12,
    "KNOW12-evidsynth",
    "Evidenz-Synthese",
    "sources",
    "hidden_conflict",
    "Befund A quellenbelegt",
    "Befund B quellenbelegt",
    "Quellenlage",
    &[".md", ".docx"]
);
relation_domain!(
    know13,
    "KNOW13-argmap",
    "Argumentationskarte",
    "premises",
    "enthymeme_gap",
    "These A mit Praemissen",
    "These B mit Praemissen",
    "Praemissenblock",
    &[".md"]
);
relation_domain!(
    know14,
    "KNOW14-datasources",
    "Datenquellen-Verzeichnis",
    "provenance",
    "unknown_provenance",
    "Quelle A mit Herkunft",
    "Quelle B mit Herkunft",
    "Herkunftsregister",
    &[".md"]
);
relation_domain!(
    know15,
    "KNOW15-researchreport",
    "Forschungsbericht",
    "method",
    "unsupported_result",
    "Ergebnis A methodengestuetzt",
    "Ergebnis B methodengestuetzt",
    "Methodenteil",
    &[".pdf", ".md"]
);

/// KNOW09 Systematic Review (PRISMA) — Studie·Fluss → unaccounted_exclusion.
/// PRISMA-Fluss = lückenlose Kette (Identifikation→Screening→Eignung→
/// Einschluss); eine Studie ohne Ketteneinordnung ist unaccounted.
pub fn know09() -> DocProfile {
    DocProfile {
        id: "KNOW09-prisma",
        subject: UnitType::Definition,
        rule: DomainRule::ChainedRelation { seam: "flow" },
        core_residue: "unaccounted_exclusion",
        reference: || {
            crystal(
                "KNOW09-prisma",
                "PRISMA-Fluss",
                vec![
                    section("s0", "PRISMA-Fluss"),
                    subject("f1", "Identifikation 120 Treffer", Some(("notes", "s0"))),
                    subject("f2", "Screening 80 nach Titel", Some(("flow", "f1"))),
                    subject("f3", "Eignung 30 Volltext", Some(("flow", "f2"))),
                    subject("f4", "Einschluss 12 Studien", Some(("flow", "f3"))),
                ],
            )
        },
        negatives: || {
            vec![
                (
                    crystal(
                        "neg-two-roots",
                        "PRISMA-Fluss",
                        vec![
                            section("s0", "PRISMA-Fluss"),
                            subject("f1", "Stufe A", Some(("notes", "s0"))),
                            subject("f2", "Stufe B", Some(("notes", "s0"))),
                        ],
                    ),
                    "unaccounted_exclusion",
                ),
                (
                    crystal(
                        "neg-detached",
                        "PRISMA-Fluss",
                        vec![
                            section("s0", "PRISMA-Fluss"),
                            subject("f1", "Stufe A", Some(("flow", "f2"))),
                            subject("f2", "Stufe B", Some(("notes", "s0"))),
                            subject("f3", "unverbuchte Ausschluss-Stufe", Some(("notes", "s0"))),
                        ],
                    ),
                    "unaccounted_exclusion",
                ),
            ]
        },
        export_formats: &[".md", ".pdf"],
    }
}

/// Alle 15 Profile der Familie J (KNOW01–15).
pub fn all_profiles() -> Vec<DocProfile> {
    vec![
        know01(),
        know02(),
        know03(),
        know04(),
        know05(),
        know06(),
        know07(),
        know08(),
        know09(),
        know10(),
        know11(),
        know12(),
        know13(),
        know14(),
        know15(),
    ]
}
