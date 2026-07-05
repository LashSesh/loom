//! Die Destillation: beobachtete Repo-Tatsachen (das eine CSU der
//! CSA-Kette) → Facetten-Zeilen → UNVERAENDERTE HBM-Kette
//! (`run_pipeline`) → zertifizierte Blueprint-Kandidaten; daneben
//! mechanisch abgeleitete RuleAtoms + DecisionSlots → `compile_grounding`
//! (Dokument 21). Jede Regel traegt ihren Beleg (`evidence_ref` zeigt
//! auf das beobachtete CSU-Feld); eine Regel ohne Beleg entsteht hier
//! strukturell nie — und wuerde von `compile_grounding` ohnehin
//! sichtbar herabgestuft.

use crate::ingest::{IngestedRepo, RepoIntelInput};
use crate::residues::repointel_residue;
use cce_core::residue::Residue;
use cce_core::value::CanonValue;
use cce_hbm::facet::{extract_facets, Facet};
use cce_hbm::pipeline::{run_pipeline, MiningInput, PipelineOutcome};
use cce_hbm::score::ScoreWeights;
use cce_swe::grounding::{
    compile_grounding, CompiledGrounding, DecisionSlot, RuleAtom, RuleSeverity,
};

/// Ergebnis der Destillation: die Facetten-Zeilen (deterministisch),
/// die extrahierten Facetten, der HBM-Lauf und das kompilierte
/// Grounding.
pub struct DistilledRepo {
    pub corpus_id: String,
    pub lines: Vec<String>,
    pub facets: Vec<Facet>,
    pub outcome: PipelineOutcome,
    pub grounding: CompiledGrounding,
}

/// Liest ein Textfeld aus dem CSU-Payload (kv_lines → Map).
fn field<'a>(payload: &'a CanonValue, key: &str) -> Option<&'a str> {
    match payload {
        CanonValue::Map(m) => match m.get(key) {
            Some(CanonValue::Text(t)) => Some(t.as_str()),
            _ => None,
        },
        _ => None,
    }
}

fn field_usize(payload: &CanonValue, key: &str) -> Option<usize> {
    field(payload, key).and_then(|t| t.trim().parse().ok())
}

/// Beobachtete Tatsachen einer Datei i (aus dem CSU-Payload).
struct FileFacts {
    path: String,
    lines: usize,
    pub_fns: Option<usize>,
    test_fns: Option<usize>,
    unsafe_blocks: Option<usize>,
    unwrap_calls: Option<usize>,
    panic_macros: Option<usize>,
    edition: Option<String>,
}

fn file_facts(payload: &CanonValue) -> Vec<FileFacts> {
    let count: usize = field_usize(payload, "file_count").unwrap_or(0);
    (0..count)
        .filter_map(|i| {
            Some(FileFacts {
                path: field(payload, &format!("file_{i}_path"))?.to_string(),
                lines: field_usize(payload, &format!("file_{i}_lines")).unwrap_or(0),
                pub_fns: field_usize(payload, &format!("file_{i}_pub_fns")),
                test_fns: field_usize(payload, &format!("file_{i}_test_fns")),
                unsafe_blocks: field_usize(payload, &format!("file_{i}_unsafe_blocks")),
                unwrap_calls: field_usize(payload, &format!("file_{i}_unwrap_calls")),
                panic_macros: field_usize(payload, &format!("file_{i}_panic_macros")),
                edition: field(payload, &format!("file_{i}_edition")).map(str::to_string),
            })
        })
        .collect()
}

/// Facetten-Zeilen (MiningProfile-Vokabular, S1-A2) aus den
/// beobachteten Tatsachen — rein mechanisch, sortierte Dateifolge
/// (kommt bereits sortiert aus `structural_kv_v1`).
fn facet_lines(input: &RepoIntelInput, facts: &[FileFacts], license_file: bool) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "boundary_contract: repo {} @ commit {} (structural_kv_v1)",
        input.repo_id, input.commit_sha
    ));
    for f in facts {
        lines.push(format!("entity: datei {}", f.path));
        lines.push(format!("measure: {} umfasst {} zeilen", f.path, f.lines));
        if let Some(n) = f.pub_fns {
            if n > 0 {
                lines.push(format!(
                    "boundary_contract: {} exportiert {} pub-fn",
                    f.path, n
                ));
            }
        }
        if let Some(n) = f.test_fns {
            if n > 0 {
                lines.push(format!("gate: {} traegt {} test-fn", f.path, n));
            }
        }
        if let Some(n) = f.unwrap_calls {
            if n > 0 {
                lines.push(format!("risk: {} enthaelt {} unwrap-aufrufe", f.path, n));
            }
        }
        if let Some(e) = &f.edition {
            lines.push(format!("constraint: edition {e} (Cargo.toml)"));
        }
    }
    // Repo-weite, am Commit BEOBACHTETE Invarianten (Zaehlung == 0
    // ueber alle .rs-Dateien) — die destillierbaren Strukturmuster.
    let rs: Vec<&FileFacts> = facts.iter().filter(|f| f.unsafe_blocks.is_some()).collect();
    if !rs.is_empty() {
        if rs.iter().all(|f| f.unsafe_blocks == Some(0)) {
            lines.push(format!(
                "invariant: unsafe-frei (alle rs-dateien, commit {})",
                input.commit_sha
            ));
        }
        if rs
            .iter()
            .filter(|f| f.path.starts_with("src/"))
            .all(|f| f.unwrap_calls == Some(0))
        {
            lines.push(format!(
                "invariant: unwrap-frei in src (commit {})",
                input.commit_sha
            ));
        }
        if rs
            .iter()
            .filter(|f| f.path.starts_with("src/"))
            .all(|f| f.panic_macros == Some(0))
        {
            lines.push(format!(
                "invariant: panic-frei in src (commit {})",
                input.commit_sha
            ));
        }
    }
    if !license_file {
        lines.push("risk: keine lizenzdatei im repo beobachtet".to_string());
    }
    lines
}

/// Mechanische Regel-Ableitung: NUR aus repo-weit beobachteten
/// Invarianten, jede mit Beleg auf das CSU-Feld. Blocking mit
/// `gate_ref: RuleComplianceGate` — ein Diff, der die am Commit
/// nachgewiesene Invariante bricht, verletzt den Bauplan.
fn derive_rules(csu_uid: &str, facts: &[FileFacts], commit: &str) -> Vec<RuleAtom> {
    let mut rules = Vec::new();
    let rs: Vec<&FileFacts> = facts.iter().filter(|f| f.unsafe_blocks.is_some()).collect();
    if rs.is_empty() {
        return rules;
    }
    let src: Vec<&&FileFacts> = rs.iter().filter(|f| f.path.starts_with("src/")).collect();
    if rs.iter().all(|f| f.unsafe_blocks == Some(0)) {
        rules.push(RuleAtom {
            rule_id: "keep-unsafe-free".to_string(),
            scope: "src/".to_string(),
            trigger: "unsafe ".to_string(),
            prescription: format!(
                "Repo ist am Commit {commit} nachweislich unsafe-frei — kein unsafe einfuehren"
            ),
            severity: RuleSeverity::Blocking,
            evidence_ref: Some(format!("{csu_uid}#unsafe_blocks=0")),
            gate_ref: Some("RuleComplianceGate".to_string()),
            decay: None,
        });
    }
    if !src.is_empty() && src.iter().all(|f| f.unwrap_calls == Some(0)) {
        rules.push(RuleAtom {
            rule_id: "keep-unwrap-free-src".to_string(),
            scope: "src/".to_string(),
            trigger: ".unwrap()".to_string(),
            prescription: format!(
                "src/ ist am Commit {commit} nachweislich unwrap-frei — Fehler propagieren"
            ),
            severity: RuleSeverity::Blocking,
            evidence_ref: Some(format!("{csu_uid}#src_unwrap_calls=0")),
            gate_ref: Some("RuleComplianceGate".to_string()),
            decay: None,
        });
    }
    if !src.is_empty() && src.iter().all(|f| f.panic_macros == Some(0)) {
        rules.push(RuleAtom {
            rule_id: "keep-panic-free-src".to_string(),
            scope: "src/".to_string(),
            trigger: "panic!(".to_string(),
            prescription: format!(
                "src/ ist am Commit {commit} nachweislich panic-frei — keine panic!-Makros"
            ),
            severity: RuleSeverity::Blocking,
            evidence_ref: Some(format!("{csu_uid}#src_panic_macros=0")),
            gate_ref: Some("RuleComplianceGate".to_string()),
            decay: None,
        });
    }
    rules
}

/// Mechanische Entscheidungs-Ableitung: was am Repo OFFEN ist, wird
/// offener DecisionSlot — sichtbare Wahrheit, kein stilles Raten.
fn derive_decisions(
    input: &RepoIntelInput,
    facts: &[FileFacts],
    license_file: bool,
) -> Vec<DecisionSlot> {
    let mut ds = Vec::new();
    if !license_file {
        ds.push(DecisionSlot::open(
            "license-clarification",
            &format!(
                "Keine LICENSE-Datei im Repo beobachtet (deklariert war '{}'). Unter welcher \
                 Lizenz steht {} formell?",
                input.declared_license, input.repo_id
            ),
            &[
                "LICENSE-Datei ergaenzen",
                "Deklaration im Manifest belassen",
                "Eigentuemer klaeren",
            ],
        ));
    }
    let unwrap_in_src: usize = facts
        .iter()
        .filter(|f| f.path.starts_with("src/"))
        .filter_map(|f| f.unwrap_calls)
        .sum();
    if unwrap_in_src > 0 {
        ds.push(DecisionSlot::open(
            "unwrap-policy",
            &format!(
                "src/ enthaelt {unwrap_in_src} beobachtete .unwrap()-Aufrufe — ist das \
                 Konvention oder Altlast?"
            ),
            &[
                "als Konvention dokumentieren",
                "abbauen (Fehler propagieren)",
            ],
        ));
    }
    ds
}

/// Die volle Destillation. Fail-closed: zertifiziert die HBM-Kette
/// keinen einzigen Blueprint, gibt es keinen Bauplan.
pub fn distill_structure(
    input: &RepoIntelInput,
    ingested: &IngestedRepo,
) -> Result<DistilledRepo, Box<Residue>> {
    let csu = &ingested.nsb.csu_set[0];
    let facts = file_facts(&csu.payload);
    if facts.is_empty() {
        return Err(Box::new(repointel_residue(
            "repointel_empty_ingest",
            "CSU traegt keine Datei-Beobachtungen",
        )));
    }
    let license_file = field(&csu.payload, "license_file_present") == Some("true");

    let corpus_id = format!("repo:{}@{}", input.repo_id, input.commit_sha);
    let lines = facet_lines(input, &facts, license_file);
    let line_refs: Vec<&str> = lines.iter().map(String::as_str).collect();
    let facets = extract_facets(&corpus_id, &line_refs);

    let mining = MiningInput {
        corpus_id: corpus_id.clone(),
        lines: lines.clone(),
        weights: ScoreWeights::default(),
        theta_d: 0,
        expansion_budget: lines.len() + 8,
    };
    let outcome = run_pipeline(&mining).map_err(|e| {
        Box::new(repointel_residue(
            "repointel_no_certified_blueprint",
            &format!("HBM-Pipeline hielt: {e:?}"),
        ))
    })?;
    if outcome.certified.is_empty() {
        return Err(Box::new(repointel_residue(
            "repointel_no_certified_blueprint",
            "HBM zertifizierte keinen einzigen Blueprint-Kandidaten",
        )));
    }

    let rules = derive_rules(&csu.uid, &facts, &input.commit_sha);
    let decisions = derive_decisions(input, &facts, license_file);
    let grounding = compile_grounding(
        &format!("blueprint:{corpus_id}"),
        rules,
        decisions,
        vec![
            "csa_ingest".to_string(),
            "hbm_distill".to_string(),
            "loom_seal".to_string(),
        ],
    );

    Ok(DistilledRepo {
        corpus_id,
        lines,
        facets,
        outcome,
        grounding,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ingest::ingest_repo;
    use crate::observe::RepoFile;

    fn numkit_input() -> RepoIntelInput {
        RepoIntelInput {
            repo_id: "beispiel/numkit".to_string(),
            commit_sha: "abc1234def".to_string(),
            declared_license: "mit".to_string(),
            files: vec![
                RepoFile::new(
                    "src/lib.rs",
                    b"pub fn max_of(v: &[i64]) -> i64 {\n    let mut m = v[0];\n    m\n}\n",
                ),
                RepoFile::new(
                    "Cargo.toml",
                    b"[package]\nname = \"numkit\"\nedition = \"2021\"\n",
                ),
                RepoFile::new(
                    "tests/it.rs",
                    b"#[test]\nfn finds_maximum() { assert!(true); }\n",
                ),
            ],
        }
    }

    #[test]
    fn distill_certifies_blueprint_and_derives_evidence_backed_rules() {
        let input = numkit_input();
        let ing = ingest_repo(&input).expect("Einzug gruen");
        let d = distill_structure(&input, &ing).expect("Destillation gruen");
        assert!(!d.outcome.certified.is_empty(), "Blueprint zertifiziert");
        // Drei repo-weit beobachtete Invarianten ⇒ drei belegte Regeln,
        // KEINE Herabstufung (jede traegt evidence_ref).
        assert_eq!(d.grounding.packet.rules.len(), 3);
        assert!(d.grounding.downgrades.is_empty());
        assert!(d
            .grounding
            .packet
            .rules
            .iter()
            .all(|r| r.evidence_ref.is_some()));
        // numkit ohne LICENSE ⇒ offene Entscheidung, sichtbar.
        assert!(d.grounding.packet.has_open_decisions());
    }

    #[test]
    fn distillation_is_deterministic_same_packet_digest() {
        let input = numkit_input();
        let ing1 = ingest_repo(&input).expect("Einzug 1");
        let ing2 = ingest_repo(&input).expect("Einzug 2");
        let d1 = distill_structure(&input, &ing1).expect("Destillation 1");
        let d2 = distill_structure(&input, &ing2).expect("Destillation 2");
        assert_eq!(
            d1.grounding.packet.digest_hex(),
            d2.grounding.packet.digest_hex(),
            "packet_digest muss replay-identisch sein"
        );
        assert_eq!(d1.outcome.certified, d2.outcome.certified);
    }
}
