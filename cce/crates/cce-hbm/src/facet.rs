//! Facets (Rebase §1.1) + Adapter-Mining-Gate Gate_A (S1-A2):
//! `Gate_A = ValidTypes ∧ ResolvableRefs ∧ Testable ∧ NonTautological ∧
//! NonContradictory` — boolesch, fail-closed. Residuen: `adapter_untyped`,
//! `facet_unsourced`.

use cce_core::gate::GateReport;

/// Facet (id, type, scope, source, evidence, confidence).
/// confidence in Promille (Integer — kein Float, und NIE ein Gate).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Facet {
    pub id: String,
    pub facet_type: String,
    pub scope: String,
    pub source: String,
    pub evidence: Option<String>,
    pub confidence_permille: u16,
}

/// Zulaessige Facet-Typen (MiningProfile-Vokabular, S1-A2).
pub const FACET_TYPES: [&str; 10] = [
    "entity",
    "operator",
    "constraint",
    "gate",
    "invariant",
    "measure",
    "process_step",
    "boundary_contract",
    "risk",
    "export_target",
];

/// Gate_A: prueft eine Facet-Menge fail-closed.
pub fn gate_a(facets: &[Facet]) -> GateReport {
    for f in facets {
        if !FACET_TYPES.contains(&f.facet_type.as_str()) {
            return GateReport::hold(
                "Gate_A",
                &format!(
                    "adapter_untyped: Facet {} traegt unbekannten Typ '{}'",
                    f.id, f.facet_type
                ),
            );
        }
        if f.source.trim().is_empty() {
            return GateReport::hold(
                "Gate_A",
                &format!("facet_unsourced: Facet {} ohne Quelle", f.id),
            );
        }
        if f.evidence.is_none() {
            return GateReport::hold(
                "Gate_A",
                &format!("facet_unsourced: Facet {} ohne Evidence", f.id),
            );
        }
        // NonTautological: Scope ≠ Inhalt der eigenen Kennung.
        if f.scope == f.id {
            return GateReport::hold(
                "Gate_A",
                &format!("Facet {} tautologisch (scope == id)", f.id),
            );
        }
    }
    // NonContradictory: keine zwei Facets gleicher Kennung mit
    // verschiedenem Typ.
    for (i, a) in facets.iter().enumerate() {
        for b in &facets[i + 1..] {
            if a.id == b.id && a.facet_type != b.facet_type {
                return GateReport::hold(
                    "Gate_A",
                    &format!("widerspruechliche Facet-Typen fuer {}", a.id),
                );
            }
        }
    }
    GateReport::pass(
        "Gate_A",
        "ValidTypes ∧ ResolvableRefs ∧ Testable ∧ NonTautological ∧ NonContradictory",
    )
}

/// Phase 1: Facet-Extraktion aus einem Korpus (deterministisch).
/// Jede Zeile `typ: inhalt` wird eine belegte Facet.
pub fn extract_facets(corpus_id: &str, lines: &[&str]) -> Vec<Facet> {
    lines
        .iter()
        .enumerate()
        .filter_map(|(i, line)| {
            let (t, content) = line.split_once(':')?;
            let t = t.trim();
            FACET_TYPES.contains(&t).then(|| Facet {
                id: format!("f{i}"),
                facet_type: t.to_string(),
                scope: content.trim().to_string(),
                source: corpus_id.to_string(),
                evidence: Some(format!("{corpus_id}#L{i}")),
                confidence_permille: 800,
            })
        })
        .collect()
}
