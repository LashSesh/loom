//! Familie P — Regulated Advisory mit ProfessionalReviewGate (REG01–08)
//! über dem Familien-Kern `family_a`. STRUKTURELL wie andere Familien
//! gebaut (Relation: jede Aussage/Empfehlung braucht ihre Rechtsgrund-/
//! Evidenz-/Norm-Naht), ABER: das Kern-Residuum ist einheitlich
//! `unreviewed` und die Domänen sind PL4 AUSDRÜCKLICH an das
//! ProfessionalReviewGate gebunden — der Agent hebt strukturell nur bis
//! **PL3** (Adapter+Zeugen+Kerntest), NIE auf PL4. Ein grünes
//! Struktur-Gate ersetzt keine Berufszulassung (Masterplan §1).

use crate::document::UnitType;
use crate::family_a::DocProfile;
use crate::family_a::DomainRule;
use crate::family_a_domains::{relation_domain, relation_negatives, relation_reference};

relation_domain!(
    reg01,
    "REG01-legal",
    "Rechtsberatungs-Entwurf",
    "grounds",
    "unreviewed",
    "Aussage A rechtsgrundgebunden",
    "Aussage B rechtsgrundgebunden",
    "Rechtsgrundlage",
    &[".docx", ".pdf"]
);
relation_domain!(
    reg02,
    "REG02-medical",
    "Medizinische Information",
    "evidences",
    "unreviewed",
    "Aussage A evidenzgebunden",
    "Aussage B evidenzgebunden",
    "Evidenzlage",
    &[".pdf"]
);
relation_domain!(
    reg03,
    "REG03-financial",
    "Finanz-/Anlageberatung",
    "grounds",
    "unreviewed",
    "Empfehlung A grundlagengebunden",
    "Empfehlung B grundlagengebunden",
    "Anlagegrundlage",
    &[".pdf"]
);
relation_domain!(
    reg04,
    "REG04-tax",
    "Steuerberatung",
    "norms",
    "unreviewed",
    "Aussage A normgebunden",
    "Aussage B normgebunden",
    "Steuernorm",
    &[".docx", ".pdf"]
);
relation_domain!(
    reg05,
    "REG05-compliance",
    "Compliance-Gutachten",
    "norms",
    "unreviewed",
    "Befund A normgebunden",
    "Befund B normgebunden",
    "Normkatalog",
    &[".pdf"]
);
relation_domain!(
    reg06,
    "REG06-engineering",
    "Ingenieur-Abnahme",
    "evidences",
    "unreviewed",
    "Aussage A nachweisgebunden",
    "Aussage B nachweisgebunden",
    "Nachweisbasis",
    &[".pdf"]
);
relation_domain!(
    reg07,
    "REG07-safety",
    "Gefahrgut-Bewertung",
    "mitigates",
    "unreviewed",
    "Risiko A massnahmengebunden",
    "Risiko B massnahmengebunden",
    "Massnahmenkatalog",
    &[".pdf"]
);
relation_domain!(
    reg08,
    "REG08-clinical",
    "Klinische/Pharma-Doku",
    "regulates",
    "unreviewed",
    "Aussage A regulatorikgebunden",
    "Aussage B regulatorikgebunden",
    "Regulatorik",
    &[".pdf"]
);

/// Alle 8 Profile der Familie P (REG01–08). PL4 review-gebunden!
pub fn all_profiles() -> Vec<DocProfile> {
    vec![
        reg01(),
        reg02(),
        reg03(),
        reg04(),
        reg05(),
        reg06(),
        reg07(),
        reg08(),
    ]
}
