//! nexus-ingress — Discovery-Schicht (CSA.2): sammelt KANDIDATEN,
//! ruft NICHTS ab. Ein CandidateSource ist passiv, bis ein Adapter
//! mit vollstaendigem Manifest das SourcePolicyGate bestanden hat.

use nexus_core::objects::{CandidateSource, SourceHorizon};

/// Deklarative Quellenliste: nur Quellen, deren Klasse im Horizont
/// liegt, werden ueberhaupt als Kandidaten gefuehrt; alles andere ist
/// `exploration_out_of_horizon` und wird sichtbar verworfen.
pub fn discover(
    hs: &SourceHorizon,
    declared: &[(String, String)],
) -> (Vec<CandidateSource>, Vec<String>) {
    let mut candidates = Vec::new();
    let mut out_of_horizon = Vec::new();
    for (locator, class) in declared {
        if hs.contains_class(class) {
            candidates.push(CandidateSource {
                locator: locator.clone(),
                source_type: class.clone(),
                discovered_by: "declared_list".to_string(),
                expected_schema: "kv_lines".to_string(),
                license_hint: None,
                risk: "low".to_string(),
                confidence_permille: 1000,
            });
        } else {
            out_of_horizon.push(format!(
                "exploration_out_of_horizon: {locator} (Klasse {class})"
            ));
        }
    }
    (candidates, out_of_horizon)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn out_of_horizon_is_visible_not_silent() {
        let hs = SourceHorizon::example_local();
        let declared = vec![
            ("corpus://a".to_string(), "local_corpus".to_string()),
            ("scrape://x".to_string(), "adhoc_scrape".to_string()),
        ];
        let (cands, rejected) = discover(&hs, &declared);
        assert_eq!(cands.len(), 1);
        assert_eq!(rejected.len(), 1);
        assert!(rejected[0].contains("exploration_out_of_horizon"));
    }
}
