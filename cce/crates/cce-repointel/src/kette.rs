//! Die P5-Kette: EIN Aufruf, fail-closed an jeder Naht.
//!
//!   ingest_repo (CSA, Gates unveraendert) → distill_structure (HBM +
//!   Grounding) → seal_source_workbody → seal_blueprint_workbody
//!   (citet die Quelle) → Replay-Selbstpruefung (zweiter Lauf der
//!   deterministischen Kette ⇒ identische Bauplan-Klasse).

use crate::distill::{distill_structure, DistilledRepo};
use crate::ingest::{ingest_repo, RepoIntelInput};
use crate::residues::repointel_residue;
use crate::workbody::{
    blueprint_class_digest, seal_blueprint_workbody, seal_source_workbody, WorkbodyError,
};
use cce_core::residue::Residue;
use cce_swe::grounding::GroundingPacket;
use loom_codec::Sealed;

/// Ergebnis der vollen Kette: beide versiegelten Koerper + das
/// GroundingPacket + die Replay-Klasse + die sichtbaren Residuen.
pub struct RepoIntelOutcome {
    pub source_sealed: Sealed,
    pub blueprint_sealed: Sealed,
    pub packet: GroundingPacket,
    pub blueprint_class_hex: String,
    /// Sichtbare offene Punkte (decision_left_open, Herabstufungen) —
    /// Warnings, blockieren die Siegelung nicht.
    pub visible_residues: Vec<Residue>,
    pub distilled: DistilledRepo,
}

#[derive(Debug)]
pub enum RepoIntelError {
    Residue(Box<Residue>),
    Workbody(WorkbodyError),
}

impl From<Box<Residue>> for RepoIntelError {
    fn from(r: Box<Residue>) -> Self {
        RepoIntelError::Residue(r)
    }
}
impl From<WorkbodyError> for RepoIntelError {
    fn from(w: WorkbodyError) -> Self {
        RepoIntelError::Workbody(w)
    }
}

/// Der volle Lauf. Verifikation (`loom_verify`) bleibt bewusst beim
/// Aufrufer/Zeugen — dieselbe Trennung wie bei cce-swe/cce-benchmark.
pub fn run_repo_intelligence(input: &RepoIntelInput) -> Result<RepoIntelOutcome, RepoIntelError> {
    // 1. Einzug (CSA-Kette, fail-closed).
    let ingested = ingest_repo(input)?;
    // 2. Destillation (HBM + Grounding, fail-closed).
    let distilled = distill_structure(input, &ingested)?;
    // 3. Quell-Container siegeln.
    let source_sealed = seal_source_workbody(input, &ingested)?;
    let source_root_hex = hex(&source_sealed.core_root);
    // 4. Bauplan siegeln (citet die Quelle).
    let blueprint_sealed = seal_blueprint_workbody(input, &ingested, &distilled, &source_root_hex)?;
    let class_hex = blueprint_class_digest(
        &distilled.grounding.packet,
        &distilled.outcome.certified,
        &source_root_hex,
    )
    .to_hex();

    // 5. Replay-Selbstpruefung: die Kette ist deterministisch — ein
    //    ZWEITER voller Lauf derselben Eingabe MUSS dieselbe
    //    Bauplan-Klasse ergeben (replay-identisch, Dokument 23 Track B).
    let ingested2 = ingest_repo(input)?;
    let distilled2 = distill_structure(input, &ingested2)?;
    let source2 = seal_source_workbody(input, &ingested2)?;
    let class2 = blueprint_class_digest(
        &distilled2.grounding.packet,
        &distilled2.outcome.certified,
        &hex(&source2.core_root),
    )
    .to_hex();
    if class2 != class_hex {
        return Err(RepoIntelError::Residue(Box::new(repointel_residue(
            "repointel_replay_divergent",
            &format!("Replay-Klasse {class2} != Erstlauf {class_hex}"),
        ))));
    }

    // Sichtbare Residuen einsammeln (Warnings).
    let mut visible = Vec::new();
    for d in distilled
        .grounding
        .packet
        .open_decisions
        .iter()
        .filter(|d| {
            matches!(
                d.status,
                cce_swe::grounding::DecisionStatus::Open
                    | cce_swe::grounding::DecisionStatus::Proposed
            )
        })
    {
        visible.push(repointel_residue(
            "repointel_decision_left_open",
            &format!("{}: {}", d.decision_id, d.question),
        ));
    }
    visible.extend(distilled.grounding.downgrades.iter().cloned());

    Ok(RepoIntelOutcome {
        source_sealed,
        blueprint_sealed,
        packet: distilled.grounding.packet.clone(),
        blueprint_class_hex: class_hex,
        visible_residues: visible,
        distilled,
    })
}

fn hex(root: &[u8; 34]) -> String {
    root.iter().map(|b| format!("{b:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
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
    fn full_chain_seals_both_bodies_and_replays_identically() {
        let out = run_repo_intelligence(&numkit_input()).expect("Kette gruen");
        assert!(!out.source_sealed.bytes.is_empty());
        assert!(!out.blueprint_sealed.bytes.is_empty());
        assert!(!out.packet.rules.is_empty());
        assert!(!out.blueprint_class_hex.is_empty());
        // numkit ohne LICENSE ⇒ sichtbares decision_left_open.
        assert!(out
            .visible_residues
            .iter()
            .any(|r| r.id.contains("repointel_decision_left_open")));
    }

    #[test]
    fn two_full_runs_yield_identical_blueprint_class() {
        let a = run_repo_intelligence(&numkit_input()).expect("Lauf 1");
        let b = run_repo_intelligence(&numkit_input()).expect("Lauf 2");
        assert_eq!(a.blueprint_class_hex, b.blueprint_class_hex);
        assert_eq!(a.packet.digest_hex(), b.packet.digest_hex());
    }
}
