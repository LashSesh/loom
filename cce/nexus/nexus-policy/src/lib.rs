//! nexus-policy — die Policy-Gate-Kette (CSA.7) und der ARCHITEKTONISCHE
//! NoFetchBeforePolicyGate (PROD-INV-13): `ApprovedFetchPlan` ist ein
//! VERSIEGELTER Typ — sein einziger Konstruktor ist `approve_fetch()`,
//! das ALLE Policy-Gates (SourceHorizon, SourcePolicy, Access, RobotsTerms,
//! License, Privacy) fail-closed durchlaeuft. Der Netzpfad (nexus-fetch)
//! akzeptiert AUSSCHLIESSLICH diesen Typ: es EXISTIERT kein Codepfad
//! „hole URL ohne Horizon/Manifest/Gates" (CSA.14).

use cce_core::gate::GateReport;
use nexus_adapter::manifest::AdapterManifest;
use nexus_adapter::port::FetchPlan;
use nexus_core::objects::{SourceHorizon, TaskSpec, DISALLOWED_ACTIONS};
use nexus_core::residues::csa_residue;
use nexus_core::verdict::Verdict;

/// Der versiegelte Beleg: kann NUR von `approve_fetch` erzeugt werden
/// (privates Feld verhindert Literal-Konstruktion ausserhalb des Crates).
// Bewusst ein privates Sentinel-Feld statt #[non_exhaustive]: das Feld
// macht die Versiegelung im Code SICHTBAR (Konstruktion nur via
// approve_fetch) — das ist die architektonische Aussage von PROD-INV-13.
#[allow(clippy::manual_non_exhaustive)]
#[derive(Debug)]
pub struct ApprovedFetchPlan {
    pub plan: FetchPlan,
    pub policy_snapshot: String,
    pub gate_reports: Vec<GateReport>,
    _sealed: (),
}

/// Deklaration einer Quelle fuer die Policy-Pruefung.
#[derive(Debug, Clone)]
pub struct SourceDeclaration {
    pub adapter_class: String,
    pub access_method: String,
    pub robots_or_terms: Option<String>,
    pub license: Option<String>,
    pub contains_pii: bool,
    /// Angeforderte Aktionen (muessen disjunkt zu DISALLOWED_ACTIONS sein).
    pub requested_actions: Vec<String>,
}

/// 1/15 SourceHorizonGate: Auftrag liegt im deklarierten Quellenhorizont.
pub fn source_horizon_gate(hs: &SourceHorizon, decl: &SourceDeclaration) -> Verdict {
    if hs.contains_class(&decl.adapter_class) {
        Verdict::Allow {
            gate: "SourceHorizonGate".into(),
            reason: format!("Klasse {} im Horizont", decl.adapter_class),
        }
    } else {
        Verdict::Hold {
            gate: "SourceHorizonGate".into(),
            residue: Box::new(csa_residue(
                "source_unknown",
                &format!(
                    "exploration_out_of_horizon: Klasse {} nicht in HS deklariert",
                    decl.adapter_class
                ),
            )),
        }
    }
}

/// 2/15 SourcePolicyGate: Manifest vollstaendig + zulaessige Klasse.
pub fn source_policy_gate(manifest: &AdapterManifest) -> Verdict {
    match manifest.validate() {
        Ok(()) => Verdict::Allow {
            gate: "SourcePolicyGate".into(),
            reason: "Manifest vollstaendig (9/9)".into(),
        },
        Err(e) => Verdict::Hold {
            gate: "SourcePolicyGate".into(),
            residue: Box::new(csa_residue(
                "manifest_missing",
                &format!(
                    "Pflichtfelder fehlen: {:?} — Quelle bleibt passiv",
                    e.missing
                ),
            )),
        },
    }
}

/// 3/15 AccessGate: Auth-Status geklaert; KEINE Umgehung — jede
/// angeforderte disallowed_action ist ein Reject-ENDZUSTAND.
pub fn access_gate(decl: &SourceDeclaration) -> Verdict {
    for a in &decl.requested_actions {
        if DISALLOWED_ACTIONS.contains(&a.as_str()) {
            return Verdict::Reject {
                gate: "AccessGate".into(),
                residue: Box::new(csa_residue(
                    "access_blocked",
                    &format!(
                        "disallowed_action '{a}' angefordert — Endzustand, keine Umgehung (PROD-INV-14)"
                    ),
                )),
            };
        }
    }
    if decl.access_method == "blocked" {
        return Verdict::Hold {
            gate: "AccessGate".into(),
            residue: Box::new(csa_residue(
                "access_blocked",
                "Zugriff blockiert (Auth/Paywall/Captcha) — Endzustand mit Residue",
            )),
        };
    }
    Verdict::Allow {
        gate: "AccessGate".into(),
        reason: format!("Zugriffsmethode '{}' geklaert", decl.access_method),
    }
}

/// 4/15 RobotsTermsGate.
pub fn robots_terms_gate(decl: &SourceDeclaration) -> Verdict {
    match decl.robots_or_terms.as_deref() {
        Some("permitted") | Some("not_applicable") => Verdict::Allow {
            gate: "RobotsTermsGate".into(),
            reason: "robots/Terms stehen nicht entgegen".into(),
        },
        Some("blocked") => Verdict::Hold {
            gate: "RobotsTermsGate".into(),
            residue: Box::new(csa_residue(
                "robots_blocked",
                "robots.txt/Terms untersagen — Endzustand",
            )),
        },
        _ => Verdict::Hold {
            gate: "RobotsTermsGate".into(),
            residue: Box::new(csa_residue("terms_unknown", "Nutzungsstatus ungeklaert")),
        },
    }
}

/// 5/15 LicenseGate: Lizenz kompatibel; Attribution wird als
/// Transportpflicht gefuehrt (PROD-INV-16).
pub fn license_gate(decl: &SourceDeclaration) -> Verdict {
    match decl.license.as_deref() {
        Some("proprietary_no_reuse") => Verdict::Hold {
            gate: "LicenseGate".into(),
            residue: Box::new(csa_residue(
                "license_incompatible",
                "Weiterverwendung untersagt",
            )),
        },
        Some(l) if l.starts_with("cc-by") => Verdict::Allow {
            gate: "LicenseGate".into(),
            reason: format!("{l}: kompatibel; Attribution wird transportiert (PROD-INV-16)"),
        },
        Some(l) => Verdict::Allow {
            gate: "LicenseGate".into(),
            reason: format!("Lizenz {l} kompatibel"),
        },
        None => Verdict::Hold {
            gate: "LicenseGate".into(),
            residue: Box::new(csa_residue("license_incompatible", "keine Lizenzangabe")),
        },
    }
}

/// 6/15 PrivacyGate: PII-Modus + Datenminimierung.
pub fn privacy_gate(decl: &SourceDeclaration, pii_mode: &str) -> Verdict {
    if decl.contains_pii && pii_mode == "no_pii" {
        Verdict::Hold {
            gate: "PrivacyGate".into(),
            residue: Box::new(csa_residue(
                "privacy_risk",
                "Quelle enthaelt PII, Lauf-Datenpolicy schliesst PII aus",
            )),
        }
    } else {
        Verdict::Allow {
            gate: "PrivacyGate".into(),
            reason: format!("pii_mode={pii_mode} eingehalten"),
        }
    }
}

/// DIE Versiegelung: alle fuenf Quell-Policy-Gates + Horizont muessen
/// `allow` liefern, sonst gibt es KEINEN ApprovedFetchPlan — und damit
/// architektonisch keinen Netzabruf (PROD-INV-13).
pub fn approve_fetch(
    hs: &SourceHorizon,
    task: &TaskSpec,
    manifest: &AdapterManifest,
    decl: &SourceDeclaration,
    plan: FetchPlan,
) -> Result<ApprovedFetchPlan, Vec<Verdict>> {
    let verdicts = [
        source_horizon_gate(hs, decl),
        source_policy_gate(manifest),
        access_gate(decl),
        robots_terms_gate(decl),
        license_gate(decl),
        privacy_gate(decl, &task.data_policy),
    ];
    let failed: Vec<Verdict> = verdicts.iter().filter(|v| !v.allows()).cloned().collect();
    if !failed.is_empty() {
        return Err(failed);
    }
    let gate_reports = verdicts
        .iter()
        .map(|v| GateReport::pass(v.gate(), "policy allow"))
        .collect();
    Ok(ApprovedFetchPlan {
        plan,
        policy_snapshot: "policy-1".to_string(),
        gate_reports,
        _sealed: (),
    })
}
