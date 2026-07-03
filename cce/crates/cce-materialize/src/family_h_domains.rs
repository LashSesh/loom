//! Familie H — Security/Infra/Ops (OPS01–15) über dem Familien-Kern
//! `family_a`. Safety-by-abstraction (S13-A3): geprüft wird die
//! Struktur (Abwehr-/Fix-/Grenz-Nähte vorhanden), NICHT Ausführung von
//! Angriffen. 12 Relation + 1 Azyklik (OPS09 Patch-Abhängigkeiten) +
//! 2 Ketten (OPS03 Incident-Phasen, OPS15 Deployment-Rollback-Kette).

use crate::document::UnitType;
use crate::family_a::{DocProfile, DomainRule};
use crate::family_a_domains::{
    acyclic_domain, acyclic_negatives, acyclic_reference, chain_negatives, chain_reference,
    chained_domain, relation_domain, relation_negatives, relation_reference,
};

relation_domain!(
    ops01,
    "OPS01-threatmodel",
    "Threat-Model",
    "mitigates",
    "unmitigated_threat",
    "Bedrohung A mit Abwehr",
    "Bedrohung B mit Abwehr",
    "Abwehrmassnahmen",
    &[".md"]
);
relation_domain!(
    ops02,
    "OPS02-runbook",
    "Runbook",
    "branches",
    "undefined_branch",
    "Schritt A mit Bedingung",
    "Schritt B mit Bedingung",
    "Entscheidungsbaum",
    &[".md"]
);
relation_domain!(
    ops04,
    "OPS04-hardening",
    "Haertungs-Checkliste",
    "hardens",
    "unhardened_asset",
    "Massnahme A auf Asset",
    "Massnahme B auf Asset",
    "Asset-Inventar",
    &[".md"]
);
relation_domain!(
    ops05,
    "OPS05-segmentation",
    "Netzwerk-Segmentierung",
    "bounds",
    "flat_network",
    "Zone A grenzgebunden",
    "Zone B grenzgebunden",
    "Grenzdefinition",
    &[".md"]
);
relation_domain!(
    ops06,
    "OPS06-rbac",
    "Zugriffskontrolle",
    "grants",
    "excess_privilege",
    "Rolle A rechtescharf",
    "Rolle B rechtescharf",
    "Rechtekatalog",
    &[".md"]
);
relation_domain!(
    ops07,
    "OPS07-monitoring",
    "Monitoring-Regeln",
    "thresholds",
    "blind_spot",
    "Signal A mit Schwelle",
    "Signal B mit Schwelle",
    "Schwellenwerte",
    &[".md"]
);
relation_domain!(
    ops08,
    "OPS08-dr",
    "Disaster-Recovery",
    "targets",
    "untested_recovery",
    "Schritt A mit RPO/RTO",
    "Schritt B mit RPO/RTO",
    "Wiederherstellungsziele",
    &[".md"]
);
relation_domain!(
    ops10,
    "OPS10-vulnreport",
    "Vulnerability-Bericht",
    "fixes",
    "unremediated_vuln",
    "Schwachstelle A mit Fix",
    "Schwachstelle B mit Fix",
    "Fix-Plan",
    &[".md"]
);
relation_domain!(
    ops11,
    "OPS11-slo",
    "SLO/SLA",
    "measures",
    "unmeasurable_slo",
    "Ziel A mit Messung",
    "Ziel B mit Messung",
    "Messgroessen",
    &[".md"]
);
relation_domain!(
    ops12,
    "OPS12-scaling",
    "Skalierungsplan",
    "handles",
    "scaling_gap",
    "Ressource A lastgebunden",
    "Ressource B lastgebunden",
    "Lastprofil",
    &[".md"]
);
relation_domain!(
    ops13,
    "OPS13-backup",
    "Backup-Strategie",
    "retains",
    "unbacked_data",
    "Datensatz A mit Aufbewahrung",
    "Datensatz B mit Aufbewahrung",
    "Aufbewahrungsplan",
    &[".md"]
);
relation_domain!(
    ops14,
    "OPS14-secrets",
    "Secrets-Management",
    "rotates",
    "plaintext_secret",
    "Secret A mit Rotation",
    "Secret B mit Rotation",
    "Rotationsplan",
    &[".md"]
);

acyclic_domain!(
    ops09,
    "OPS09-patchplan",
    "Patch-Plan",
    "depends",
    "unpatched_cve",
    "Patch A abhaengig",
    "Patch B abhaengig",
    "Basispatch",
    &[".md"]
);
chained_domain!(
    ops03,
    "OPS03-incident",
    "Incident-Response",
    "handover",
    "missing_containment",
    "Phase Erkennung",
    "Phase Eindaemmung",
    "Phase Wiederherstellung",
    &[".md"]
);
chained_domain!(
    ops15,
    "OPS15-deployment",
    "Deployment-Plan",
    "rollback",
    "no_rollback",
    "Schritt Vorbereitung",
    "Schritt Ausrollung",
    "Schritt Rollback-Punkt",
    &[".md"]
);

/// Alle 15 Profile der Familie H (OPS01–15).
pub fn all_profiles() -> Vec<DocProfile> {
    vec![
        ops01(),
        ops02(),
        ops03(),
        ops04(),
        ops05(),
        ops06(),
        ops07(),
        ops08(),
        ops09(),
        ops10(),
        ops11(),
        ops12(),
        ops13(),
        ops14(),
        ops15(),
    ]
}
