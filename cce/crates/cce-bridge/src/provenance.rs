//! S-E5 §2/§10(b): ProvenanceSet-Aufloesung ueber den bestehenden
//! `CitationResolver`-Port (SeedResolver und/oder Klassen-Registry,
//! E4c) — jedes Mitglied MUSS Valid und geschlossen sein; Mitglieder
//! unter Quarantaene oder mit offenen `supports`/`derives`-Nähten sind
//! unzulaessig (Quellen-Quiescence).

use crate::cv_util::cv_get;
use crate::types::ProvenanceSet;
use loom_canon::Cv;
use loom_cites::{CitationResolver, VerifiedTarget};
use loom_verify::Verdict;

fn is_closed(manifest: &Cv) -> bool {
    match cv_get(manifest, "claims").and_then(|c| cv_get(c, "closed")) {
        Some(Cv::Bool(b)) => *b,
        _ => false,
    }
}

/// Erste Domaene aus `domain_refs` (dieselbe Konvention wie die
/// Klassen-Registry, E4c).
pub fn domain_of(vt: &VerifiedTarget) -> Option<String> {
    match cv_get(&vt.manifest, "domain_refs") {
        Some(Cv::Array(items)) => items.first().and_then(|v| match v {
            Cv::Text(s) => Some(s.clone()),
            _ => None,
        }),
        _ => None,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemberDiagnosis {
    pub core_root_hex: String,
    pub eligible: bool,
    pub reason: Option<&'static str>,
}

/// Prueft EIN ProvenanceSet-Kandidatenmitglied: aufgeloest, `core_root`
/// byte-identisch (Verteidigung in der Tiefe, X1a-Muster), Verdikt
/// EXAKT `Valid` (schliesst Quarantaene UND `ValidWithResidues` aus —
/// die woertliche Spec-Lesart "MUSS Valid und geschlossen sein"),
/// `claims.closed == true`, und keine offene eigene `supports`/
/// `derives`-Naht (Quellen-Quiescence: die Zielseite jeder eigenen
/// closure-relevanten Naht muss selbst aufloesbar und mindestens
/// `ValidWithResidues` sein).
pub fn check_member(root_hex: &str, resolver: &dyn CitationResolver) -> MemberDiagnosis {
    let diag = |eligible: bool, reason: Option<&'static str>| MemberDiagnosis {
        core_root_hex: root_hex.to_string(),
        eligible,
        reason,
    };
    let Some(vt) = resolver.resolve(root_hex) else {
        return diag(false, Some("unresolved"));
    };
    if vt.core_root_hex != root_hex {
        return diag(false, Some("class_mismatch"));
    }
    if vt.verdict != Verdict::Valid {
        return diag(false, Some("not_valid_or_quarantined"));
    }
    if !is_closed(&vt.manifest) {
        return diag(false, Some("not_closed"));
    }
    for c in &vt.cites {
        if c.cite_kind.is_closure_relevant() {
            match resolver.resolve(&c.target_core_root_hex) {
                Some(t2) if matches!(t2.verdict, Verdict::Valid | Verdict::ValidWithResidues) => {}
                _ => return diag(false, Some("open_supports_cite")),
            }
        }
    }
    diag(true, None)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProvenanceError {
    /// N-NRM-1: `|ProvenanceSet nach Filterung| < kappa_min`.
    InsufficientProvenance { resolved: usize, kappa_min: usize },
}

/// Loest eine Kandidatenliste zu einem gefilterten `ProvenanceSet` auf
/// UND liefert die Menge der beobachteten Domaenen (fuer die
/// DiversityGate, §3 Stufe 2). Nur eligible Mitglieder zaehlen fuer
/// `kappa_min`.
pub fn resolve_provenance(
    candidates: &[String],
    resolver: &dyn CitationResolver,
    kappa_min: usize,
) -> Result<(ProvenanceSet, Vec<String>), ProvenanceError> {
    let mut eligible = Vec::new();
    let mut domains = std::collections::BTreeSet::new();
    for root in candidates {
        let d = check_member(root, resolver);
        if d.eligible {
            eligible.push(root.clone());
            if let Some(vt) = resolver.resolve(root) {
                if let Some(dom) = domain_of(&vt) {
                    domains.insert(dom);
                }
            }
        }
    }
    let set = ProvenanceSet::new(eligible);
    if set.len() < kappa_min {
        return Err(ProvenanceError::InsufficientProvenance {
            resolved: set.len(),
            kappa_min,
        });
    }
    Ok((set, domains.into_iter().collect()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use loom_cites::CiteEntry;
    use std::collections::BTreeMap;

    struct MockResolver {
        targets: BTreeMap<String, VerifiedTarget>,
    }

    impl CitationResolver for MockResolver {
        fn resolve(&self, root: &str) -> Option<VerifiedTarget> {
            self.targets.get(root).cloned()
        }
    }

    fn closed_target(root: &str, domain: &str, cites: Vec<CiteEntry>) -> VerifiedTarget {
        VerifiedTarget {
            core_root_hex: root.to_string(),
            verdict: Verdict::Valid,
            manifest: Cv::map(vec![
                ("claims", Cv::map(vec![("closed", Cv::Bool(true))])),
                ("domain_refs", Cv::Array(vec![Cv::Text(domain.to_string())])),
            ]),
            unit_ids: Default::default(),
            cites,
        }
    }

    #[test]
    fn eligible_member_passes_all_checks() {
        let root = "aa".repeat(34);
        let mut targets = BTreeMap::new();
        targets.insert(root.clone(), closed_target(&root, "dom:d1", vec![]));
        let resolver = MockResolver { targets };
        let d = check_member(&root, &resolver);
        assert!(d.eligible, "{:?}", d.reason);
    }

    #[test]
    fn unresolved_member_is_ineligible() {
        let resolver = MockResolver {
            targets: BTreeMap::new(),
        };
        let d = check_member(&"bb".repeat(34), &resolver);
        assert!(!d.eligible);
        assert_eq!(d.reason, Some("unresolved"));
    }

    #[test]
    fn not_closed_member_is_ineligible() {
        let root = "cc".repeat(34);
        let mut vt = closed_target(&root, "dom:d1", vec![]);
        vt.manifest = Cv::map(vec![("claims", Cv::map(vec![("closed", Cv::Bool(false))]))]);
        let mut targets = BTreeMap::new();
        targets.insert(root.clone(), vt);
        let resolver = MockResolver { targets };
        let d = check_member(&root, &resolver);
        assert!(!d.eligible);
        assert_eq!(d.reason, Some("not_closed"));
    }

    #[test]
    fn quarantined_member_is_ineligible() {
        let root = "dd".repeat(34);
        let mut vt = closed_target(&root, "dom:d1", vec![]);
        vt.verdict = Verdict::Quarantine;
        let mut targets = BTreeMap::new();
        targets.insert(root.clone(), vt);
        let resolver = MockResolver { targets };
        let d = check_member(&root, &resolver);
        assert!(!d.eligible);
        assert_eq!(d.reason, Some("not_valid_or_quarantined"));
    }

    #[test]
    fn open_supports_cite_makes_member_ineligible() {
        let root = "ee".repeat(34);
        let dangling = "ff".repeat(34);
        let vt = closed_target(
            &root,
            "dom:d1",
            vec![CiteEntry {
                unit_id: "u1".to_string(),
                target_core_root_hex: dangling,
                target_unit_ref: None,
                cite_kind: loom_cites::CiteKind::Supports,
            }],
        );
        let mut targets = BTreeMap::new();
        targets.insert(root.clone(), vt);
        let resolver = MockResolver { targets };
        let d = check_member(&root, &resolver);
        assert!(!d.eligible);
        assert_eq!(d.reason, Some("open_supports_cite"));
    }

    /// N-NRM-1: kappa_min nicht erreicht ⇒ Hold (hier: Err).
    #[test]
    fn below_kappa_min_is_insufficient_provenance() {
        let root = "12".repeat(34);
        let mut targets = BTreeMap::new();
        targets.insert(root.clone(), closed_target(&root, "dom:d1", vec![]));
        let resolver = MockResolver { targets };
        let err = resolve_provenance(&[root], &resolver, 3).unwrap_err();
        assert_eq!(
            err,
            ProvenanceError::InsufficientProvenance {
                resolved: 1,
                kappa_min: 3
            }
        );
    }

    #[test]
    fn resolve_provenance_collects_distinct_domains() {
        let r1 = "21".repeat(34);
        let r2 = "22".repeat(34);
        let r3 = "23".repeat(34);
        let mut targets = BTreeMap::new();
        targets.insert(r1.clone(), closed_target(&r1, "dom:a", vec![]));
        targets.insert(r2.clone(), closed_target(&r2, "dom:b", vec![]));
        targets.insert(r3.clone(), closed_target(&r3, "dom:a", vec![]));
        let resolver = MockResolver { targets };
        let (set, domains) = resolve_provenance(&[r1, r2, r3], &resolver, 3).unwrap();
        assert_eq!(set.len(), 3);
        assert_eq!(domains, vec!["dom:a".to_string(), "dom:b".to_string()]);
    }
}
