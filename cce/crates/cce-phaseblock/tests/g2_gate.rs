//! G2-Ausgangs-Gate (01_MASTER_BUILD):
//! - Accept-Kriterien einzeln getestet (jede fehlende Bedingung ⇒ Hold, nie Commit)
//! - Frontier-Desync wird erkannt (blocking)
//! - Ledger-Projektion round-trip-getestet
//! - Negativ: Kandidaten-Commit ohne Evidence abgewiesen (Driftverbot 6)

use cce_core::gate::GateReport;
use cce_core::residue::{Residue, ResidueField, ResidueKind, Severity};
use cce_core::signature::sha256;
use cce_core::value::CanonValue;
use cce_phaseblock::accept::{accept_block, AcceptContext, AcceptOutcome};
use cce_phaseblock::consensus::{crystal_consensus, ConsensusVerdict};
use cce_phaseblock::frontier::{hdag_frontier, sync_frontiers, Frontier, FrontierSync};
use cce_phaseblock::hyperdag::{EdgeKind, HyperDag};
use cce_phaseblock::phaseblock::{BlockStatus, PhaseBlock};
use cce_phaseblock::projection::{commit_projection, verify_hdag_projection, ProjectionError};
use std::collections::BTreeSet;

fn good_candidate(tag: &str) -> PhaseBlock {
    let gates: Vec<GateReport> = cce_core::gate::mandatory_gates()
        .iter()
        .map(|g| GateReport::pass(&g.id, "ok"))
        .collect();
    PhaseBlock::candidate(
        1,
        "p1",
        &CanonValue::text(tag),
        vec![],
        gates,
        vec![sha256(tag.as_bytes())],
        ResidueField::new(),
        sha256(b"rd"),
        vec![],
    )
}

/// Accept-8: JEDES der acht Kriterien fuehrt einzeln zu Hold, nie Commit.
#[test]
fn each_accept_criterion_individually_holds() {
    // 1..3, 7, 8: Kontext-Kriterien
    let ctx_variants: Vec<(&str, AcceptContext)> = vec![
        (
            "Typed",
            AcceptContext {
                typed: false,
                ..AcceptContext::all_true()
            },
        ),
        (
            "BoundaryValid",
            AcceptContext {
                boundary_valid: false,
                ..AcceptContext::all_true()
            },
        ),
        (
            "SeamConsistent",
            AcceptContext {
                seam_consistent: false,
                ..AcceptContext::all_true()
            },
        ),
        (
            "Replayable",
            AcceptContext {
                replayable: false,
                ..AcceptContext::all_true()
            },
        ),
        (
            "ReanalysisCompatible",
            AcceptContext {
                reanalysis_compatible: false,
                ..AcceptContext::all_true()
            },
        ),
    ];
    for (name, ctx) in ctx_variants {
        let mut b = good_candidate("x");
        match accept_block(&mut b, &ctx) {
            AcceptOutcome::Hold(reasons) => {
                assert!(
                    reasons.iter().any(|r| r.contains(name)),
                    "{name} fehlt in {reasons:?}"
                );
                assert_eq!(b.status, BlockStatus::Hold);
            }
            AcceptOutcome::Accepted => panic!("{name}=0 wurde committed (Accept-8 verletzt)"),
        }
    }
    // 4: Gate=Pass
    let mut b = good_candidate("g");
    b.gate_reports = vec![GateReport::hold("G4-Residue", "offen")];
    assert!(matches!(
        accept_block(&mut b, &AcceptContext::all_true()),
        AcceptOutcome::Hold(_)
    ));
    // 5: EvidenceComplete (Driftverbot 6) — siehe eigener Test unten.
    // 6: ResidueVisible/blocking
    let mut b = good_candidate("r");
    let mut rf = ResidueField::new();
    rf.push(Residue::new(
        "r1",
        "test",
        ResidueKind::OpenQuestion,
        Severity::Blocking,
        "offen",
    ));
    b.residue_field = rf;
    assert!(matches!(
        accept_block(&mut b, &AcceptContext::all_true()),
        AcceptOutcome::Hold(_)
    ));
    // 8/8 erfuellt ⇒ Accepted
    let mut b = good_candidate("ok");
    assert_eq!(
        accept_block(&mut b, &AcceptContext::all_true()),
        AcceptOutcome::Accepted
    );
}

/// Driftverbot 6 (Negativ): Kandidaten-Commit ohne Evidence wird abgewiesen.
#[test]
fn candidate_commit_without_evidence_is_rejected() {
    let mut b = good_candidate("no-ev");
    b.evidence_refs.clear();
    match accept_block(&mut b, &AcceptContext::all_true()) {
        AcceptOutcome::Hold(reasons) => {
            assert!(reasons.iter().any(|r| r.contains("EvidenceComplete")));
            assert_eq!(b.status, BlockStatus::Hold, "Hold, nie Commit");
        }
        AcceptOutcome::Accepted => panic!("Commit ohne Evidence (Driftverbot 6 verletzt)"),
    }
}

/// Frontier-Desync wird erkannt und ist blocking.
#[test]
fn frontier_desync_is_detected_blocking() {
    let mut h = HyperDag::new();
    let mut b1 = good_candidate("f1");
    accept_block(&mut b1, &AcceptContext::all_true());
    let id1 = h.insert_block(b1);
    let mut b2 = good_candidate("f2");
    accept_block(&mut b2, &AcceptContext::all_true());
    let id2 = h.insert_block(b2);
    h.add_edge(&id1, &id2, EdgeKind::Dep).unwrap();

    let fh = hdag_frontier(&h);
    assert_eq!(fh.block_ids, BTreeSet::from([id2.clone()]));

    // Ratchet-Frontier synchron:
    let fr_ok = Frontier {
        kind: "ratchet".into(),
        block_ids: BTreeSet::from([id2]),
    };
    assert!(matches!(sync_frontiers(&fh, &fr_ok), FrontierSync::InSync));

    // Ratchet-Frontier hinkt hinterher ⇒ Desync, blocking Residuum:
    let fr_bad = Frontier {
        kind: "ratchet".into(),
        block_ids: BTreeSet::from([id1]),
    };
    match sync_frontiers(&fh, &fr_bad) {
        FrontierSync::Desync(res) => {
            assert_eq!(res.kind.as_str(), "frontier_desync");
            assert_eq!(res.severity, Severity::Blocking);
        }
        FrontierSync::InSync => panic!("Desync NICHT erkannt (blocking Pflicht)"),
    }
}

/// Ledger = CommitProjection(H): Round-Trip + Mismatch-Erkennung.
#[test]
fn ledger_projection_roundtrip() {
    let mut h = HyperDag::new();
    let mut ids = Vec::new();
    for tag in ["a", "b", "c"] {
        let mut b = good_candidate(tag);
        accept_block(&mut b, &AcceptContext::all_true());
        ids.push(h.insert_block(b));
    }
    h.add_edge(&ids[0], &ids[1], EdgeKind::Phase).unwrap();
    h.add_edge(&ids[1], &ids[2], EdgeKind::Commit).unwrap();

    // Projektion und Verifikation: Round-Trip gruen.
    let ledger = commit_projection(&h).unwrap();
    assert_eq!(ledger.events().len(), 3);
    assert!(verify_hdag_projection(&h, &ledger).is_ok());

    // Manipulierter Ledger ⇒ ledger_hdag_mismatch (blocking).
    let tampered = ledger.tampered_copy_for_tests(1, sha256(b"anders"));
    match verify_hdag_projection(&h, &tampered) {
        Err(ProjectionError::Mismatch(res)) => {
            assert_eq!(res.kind.as_str(), "ledger_hdag_mismatch");
            assert_eq!(res.severity, Severity::Blocking);
            let _boxed: Box<cce_core::residue::Residue> = res;
        }
        other => panic!("Mismatch nicht erkannt: {other:?}"),
    }

    // Kandidat (nicht akzeptiert) erscheint NIE in der Projektion.
    let candidate = good_candidate("nur-kandidat");
    let cid = h.insert_block(candidate);
    let ledger2 = commit_projection(&h).unwrap();
    assert_eq!(ledger2.events().len(), 3, "Kandidat {cid} nicht verbucht");
}

/// CrystalConsensus: Accept ∧ ParentClosure ∧ FrontierCompatible ∧ LedgerAppendable.
#[test]
fn crystal_consensus_requires_parent_closure() {
    let mut h = HyperDag::new();
    let parent = good_candidate("eltern"); // bleibt Kandidat!
    let pid = h.insert_block(parent);
    let mut child = good_candidate("kind");
    child.parent_refs = vec![pid.clone()];
    accept_block(&mut child, &AcceptContext::all_true());
    match crystal_consensus(&h, &child) {
        ConsensusVerdict::Refused(reasons) => {
            assert!(reasons.iter().any(|r| r.contains("ParentClosure")));
        }
        ConsensusVerdict::Reached => panic!("Konsens ohne Eltern-Schluss (verboten)"),
    }
}

/// Zyklen im HyperDAG werden abgewiesen (azyklische Commit-Ordnung).
#[test]
fn hyperdag_rejects_cycles() {
    let mut h = HyperDag::new();
    let a = h.insert_block(good_candidate("za"));
    let b = h.insert_block(good_candidate("zb"));
    h.add_edge(&a, &b, EdgeKind::Dep).unwrap();
    assert!(h.add_edge(&b, &a, EdgeKind::Seam).is_err());
}
