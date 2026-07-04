//! S-E5 §10(c): der Destillationslauf — RD-gebunden, NIE ein
//! Hintergrundprozess (N-A6). Liefert einen `NormCandidate`; Kandidat ≠
//! Norm, das Kandidaten-Commit-Verbot gilt unveraendert (die eigentliche
//! Promotion laeuft ausschliesslich durchs BridgeGate, §3).

use crate::pattern::Pattern;
use crate::provenance::{resolve_provenance, ProvenanceError};
use crate::types::{CounterExample, NormCandidate, Scope};
use cce_core::canonical::Canonicalize;
use cce_core::replay::{RdError, RunDescriptor};
use loom_cites::CitationResolver;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DistillError {
    /// N-NRM-7: kein Lauf ohne vollstaendigen RD (S5.9-Disziplin) — eine
    /// Destillation ohne echten, validierten RD ist verboten/reject,
    /// nicht "einfach ohne RD-Feld weiterlaufen".
    IncompleteRunDescriptor(&'static str),
    Provenance(ProvenanceError),
}

/// Eingabe eines Destillationslaufs — `pattern` wird vom Aufrufer
/// geliefert (HBM-Blueprints via `extract::blueprint_to_pattern` und/
/// oder Registry-Bestand als Pattern-Lieferant, §7); L9b selbst
/// gatet/promoviert, es mined nicht. Dokument 16 §2b: `pattern` ist die
/// typisierte Form — die `NexusClass` folgt daraus (`pattern.class()`),
/// kein eigenes Feld mehr.
pub struct DistillationInput {
    pub pattern: Pattern,
    pub candidate_roots: Vec<String>,
    pub n_support: u64,
    /// Pflichtfeld: ein Kandidat, der Gegenbeispiele verschweigt, ist
    /// ungueltig (§2). `known_counterexamples.len()` ist die einzige
    /// Quelle der Wahrheit fuer `n_counter`.
    pub known_counterexamples: Vec<CounterExample>,
    pub scope: Scope,
    pub kappa_min: usize,
}

pub fn distill(
    input: DistillationInput,
    resolver: &dyn CitationResolver,
    rd: &RunDescriptor,
) -> Result<NormCandidate, DistillError> {
    rd.validate().map_err(|e| {
        let RdError::ReplayDescriptorIncomplete(field) = e;
        DistillError::IncompleteRunDescriptor(field)
    })?;
    let (provenance_set, _domains) =
        resolve_provenance(&input.candidate_roots, resolver, input.kappa_min)
            .map_err(DistillError::Provenance)?;
    let distillation_rd_class_hex = rd.canonical_class().0.to_hex();
    Ok(NormCandidate {
        pattern: input.pattern,
        provenance_set,
        n_support: input.n_support,
        n_counter: input.known_counterexamples.len() as u64,
        known_counterexamples: input.known_counterexamples,
        scope: input.scope,
        distillation_rd_class_hex,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cce_core::signature::sha256;
    use loom_canon::Cv;
    use loom_cites::VerifiedTarget;
    use loom_verify::Verdict;
    use std::collections::BTreeMap;

    struct MockResolver {
        targets: BTreeMap<String, VerifiedTarget>,
    }

    impl CitationResolver for MockResolver {
        fn resolve(&self, root: &str) -> Option<VerifiedTarget> {
            self.targets.get(root).cloned()
        }
    }

    fn closed_target(root: &str) -> VerifiedTarget {
        VerifiedTarget {
            core_root_hex: root.to_string(),
            verdict: Verdict::Valid,
            manifest: Cv::map(vec![
                ("claims", Cv::map(vec![("closed", Cv::Bool(true))])),
                (
                    "domain_refs",
                    Cv::Array(vec![Cv::Text("dom:family_a".to_string())]),
                ),
            ]),
            unit_ids: Default::default(),
            cites: vec![],
        }
    }

    fn three_members() -> (Vec<String>, BTreeMap<String, VerifiedTarget>) {
        let roots: Vec<String> = ["a1", "a2", "a3"].iter().map(|s| s.repeat(34)).collect();
        let mut targets = BTreeMap::new();
        for r in &roots {
            targets.insert(r.clone(), closed_target(r));
        }
        (roots, targets)
    }

    /// N-NRM-7: ein unvollstaendiger RD (leere domain) macht die
    /// Destillation verboten — reject, nicht raten.
    #[test]
    fn incomplete_rd_is_rejected() {
        let (roots, targets) = three_members();
        let resolver = MockResolver { targets };
        let mut rd = RunDescriptor::new(sha256(b"c"), "document", 1);
        rd.domain = String::new();
        let input = DistillationInput {
            pattern: Pattern::StructuralRule(crate::pattern::DomainRuleForm::UniqueSubjects),
            candidate_roots: roots,
            n_support: 3,
            known_counterexamples: vec![],
            scope: Scope::Global,
            kappa_min: 3,
        };
        let err = distill(input, &resolver, &rd).unwrap_err();
        assert_eq!(err, DistillError::IncompleteRunDescriptor("domain"));
    }

    #[test]
    fn valid_rd_and_provenance_yields_candidate() {
        let (roots, targets) = three_members();
        let resolver = MockResolver { targets };
        let rd = RunDescriptor::new(sha256(b"c"), "document", 1);
        let input = DistillationInput {
            pattern: Pattern::StructuralRule(crate::pattern::DomainRuleForm::Relation {
                seam: "refers".to_string(),
            }),
            candidate_roots: roots,
            n_support: 3,
            known_counterexamples: vec![],
            scope: Scope::Global,
            kappa_min: 3,
        };
        let candidate = distill(input, &resolver, &rd).expect("Destillation gelingt");
        assert_eq!(candidate.provenance_set.len(), 3);
        assert_eq!(candidate.n_counter, 0);
        assert!(!candidate.distillation_rd_class_hex.is_empty());
    }

    /// N-NRM-4 (Vorstufe): `n_counter` ist NIE vom Aufrufer frei
    /// wählbar — es ist strukturell die Laenge der mitgefuehrten
    /// Gegenbeispiele, ein Verschweigen ist unmoeglich.
    #[test]
    fn n_counter_is_derived_from_known_counterexamples() {
        let (roots, targets) = three_members();
        let resolver = MockResolver { targets };
        let rd = RunDescriptor::new(sha256(b"c"), "document", 1);
        let input = DistillationInput {
            pattern: Pattern::StructuralRule(crate::pattern::DomainRuleForm::UniqueSubjects),
            candidate_roots: roots,
            n_support: 3,
            known_counterexamples: vec![CounterExample {
                core_root_hex: "ff".repeat(34),
                reason: "Abweichung in Familie X".to_string(),
            }],
            scope: Scope::Global,
            kappa_min: 3,
        };
        let candidate = distill(input, &resolver, &rd).unwrap();
        assert_eq!(candidate.n_counter, 1);
    }

    #[test]
    fn insufficient_provenance_propagates() {
        let (mut roots, targets) = three_members();
        roots.truncate(1);
        let resolver = MockResolver { targets };
        let rd = RunDescriptor::new(sha256(b"c"), "document", 1);
        let input = DistillationInput {
            pattern: Pattern::StructuralRule(crate::pattern::DomainRuleForm::UniqueSubjects),
            candidate_roots: roots,
            n_support: 1,
            known_counterexamples: vec![],
            scope: Scope::Global,
            kappa_min: 3,
        };
        assert!(matches!(
            distill(input, &resolver, &rd),
            Err(DistillError::Provenance(_))
        ));
    }
}
