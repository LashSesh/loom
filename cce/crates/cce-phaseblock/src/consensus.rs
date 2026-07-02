//! CrystalConsensus (Rebase §4.2):
//! `CrystalConsensus(b) = Accept(b) ∧ ParentClosure(b) ∧ FrontierCompatible(b)
//!  ∧ LedgerAppendable(b)` — struktureller Konsens, KEIN Blockchain-Import
//! (Driftverbot 2): keine Miner, keine Token, kein Konsensnetz.

use crate::hyperdag::HyperDag;
use crate::phaseblock::{BlockStatus, PhaseBlock};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConsensusVerdict {
    Reached,
    Refused(Vec<String>),
}

pub fn crystal_consensus(h: &HyperDag, block: &PhaseBlock) -> ConsensusVerdict {
    let mut missing = Vec::new();
    if block.status != BlockStatus::Accepted {
        missing.push("Accept(b) = 0".to_string());
    }
    // ParentClosure: alle Eltern akzeptiert.
    for p in &block.parent_refs {
        match h.blocks.get(p) {
            Some(pb) if pb.status == BlockStatus::Accepted => {}
            Some(_) => missing.push(format!("ParentClosure: Elternblock {p} nicht akzeptiert")),
            None => missing.push(format!("ParentClosure: Elternblock {p} unbekannt")),
        }
    }
    // FrontierCompatible: alle Eltern liegen im HDAG (kausal anschliessbar).
    // LedgerAppendable: der Block traegt RD-Referenz und Payload-Digest.
    if block.payload_digest.0 == [0u8; 32] {
        missing.push("LedgerAppendable: leerer Payload-Digest".to_string());
    }
    if missing.is_empty() {
        ConsensusVerdict::Reached
    } else {
        ConsensusVerdict::Refused(missing)
    }
}
