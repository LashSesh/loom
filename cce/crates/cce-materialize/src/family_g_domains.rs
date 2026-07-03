//! Familie G — Governance/Compliance/Audit (GOV01–12) als reine Profil-
//! Spezialisierung über dem Familien-Kern `family_a`. Alle 12 sind
//! Relation-Regeln (Subjekt·Nachweis-/Beleg-/Kontroll-Naht → Residuum
//! aus S1 K.4). GOV05 „kein Score-Gate" ist über das geerbte
//! no_score-Gate bereits erfüllt.

use crate::document::UnitType;
use crate::family_a::DocProfile;
use crate::family_a::DomainRule;
use crate::family_a_domains::{relation_domain, relation_negatives, relation_reference};

relation_domain!(
    gov01,
    "GOV01-compliance",
    "Compliance-Checkliste",
    "evidences",
    "unevidenced_requirement",
    "Anforderung A mit Nachweis",
    "Anforderung B mit Nachweis",
    "Nachweisregister",
    &[".md", ".docx"]
);
relation_domain!(
    gov02,
    "GOV02-audit",
    "Audit-Bericht",
    "backs",
    "unbacked_finding",
    "Feststellung A belegt",
    "Feststellung B belegt",
    "Belegsammlung",
    &[".docx", ".pdf"]
);
relation_domain!(
    gov03,
    "GOV03-controlmatrix",
    "Kontroll-Matrix",
    "covers",
    "uncovered_risk",
    "Kontrolle A deckt Risiko",
    "Kontrolle B deckt Risiko",
    "Risikoregister",
    &[".md"]
);
relation_domain!(
    gov04,
    "GOV04-controlmapping",
    "Control-Mapping",
    "maps",
    "unmapped_control",
    "Anforderung A auf Kontrolle",
    "Anforderung B auf Kontrolle",
    "Kontrollkatalog",
    &[".md"]
);
relation_domain!(
    gov05,
    "GOV05-riskassessment",
    "Risikobewertung",
    "rates",
    "inconsistent_rating",
    "Risiko A bewertet",
    "Risiko B bewertet",
    "Bewertungsmethode",
    &[".md", ".docx"]
);
relation_domain!(
    gov06,
    "GOV06-dpia",
    "DPIA",
    "protects",
    "unmitigated_processing",
    "Verarbeitung A mit Schutz",
    "Verarbeitung B mit Schutz",
    "Schutzmassnahmen",
    &[".docx", ".pdf"]
);
relation_domain!(
    gov07,
    "GOV07-evidence",
    "Evidence-Paket",
    "traces",
    "orphan_evidence",
    "Nachweis A auf Anforderung",
    "Nachweis B auf Anforderung",
    "Anforderungsbezug",
    &[".md"]
);
relation_domain!(
    gov08,
    "GOV08-gap",
    "Gap-Analyse",
    "compares",
    "unaddressed_gap",
    "Soll A mit Ist-Bezug",
    "Soll B mit Ist-Bezug",
    "Ist-Aufnahme",
    &[".md", ".docx"]
);
relation_domain!(
    gov09,
    "GOV09-attestation",
    "Attestierung",
    "backs",
    "unsupported_attestation",
    "Aussage A belegt",
    "Aussage B belegt",
    "Belegblock",
    &[".pdf"]
);
relation_domain!(
    gov10,
    "GOV10-ruleconformance",
    "Regelwerk-Konformitaet",
    "implements",
    "unimplemented_rule",
    "Regel A umgesetzt",
    "Regel B umgesetzt",
    "Umsetzungsnachweis",
    &[".md"]
);
relation_domain!(
    gov11,
    "GOV11-incident",
    "Incident-Report",
    "causes",
    "rootless_incident",
    "Ereignis A mit Ursache",
    "Ereignis B mit Ursache",
    "Ursachenanalyse",
    &[".md", ".docx"]
);
relation_domain!(
    gov12,
    "GOV12-certprep",
    "Zertifizierungs-Vorbereitung",
    "covers",
    "uncovered_criterion",
    "Kriterium A mit Nachweis",
    "Kriterium B mit Nachweis",
    "Nachweisdeckung",
    &[".md", ".docx"]
);

/// Alle 12 Profile der Familie G (GOV01–12).
pub fn all_profiles() -> Vec<DocProfile> {
    vec![
        gov01(),
        gov02(),
        gov03(),
        gov04(),
        gov05(),
        gov06(),
        gov07(),
        gov08(),
        gov09(),
        gov10(),
        gov11(),
        gov12(),
    ]
}
