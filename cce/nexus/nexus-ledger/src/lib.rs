//! nexus-ledger — der Source-Run-Ledger: `Ledger = CommitProjection
//! (SourceRunHyperDAG)` (S9-A4, kein Blockchain-Import). 12/15 ReplayGate:
//! fixer Snapshot ⇒ gleiche IDs, Hashes, Ledgerpfade.

use cce_core::canonical::Canonicalize;
use cce_core::ledger::{Ledger, LedgerEventKind};
use cce_core::signature::Digest;
use nexus_core::objects::NexusSourceBundle;
use nexus_core::residues::csa_residue;
use nexus_core::verdict::Verdict;

/// Verbucht einen Akquisitionslauf deterministisch.
pub fn ledger_for_bundle(nsb: &NexusSourceBundle) -> Ledger {
    let mut l = Ledger::new();
    for csu in &nsb.csu_set {
        l.append(LedgerEventKind::Execute, csu.canonical_class().0);
    }
    for ep in &nsb.evidence_packs {
        l.append(LedgerEventKind::Gate, ep.csu_class);
    }
    l.append(
        LedgerEventKind::Commit,
        cce_core::signature::sha256(nsb.bundle_id.as_bytes()),
    );
    l
}

/// 12/15 ReplayGate: zwei Laeufe desselben Snapshots muessen identische
/// Ledgerpfade liefern.
pub fn replay_gate(first_head: Digest, second_head: Digest) -> Verdict {
    if first_head == second_head {
        Verdict::Allow {
            gate: "ReplayGate".into(),
            reason: "fixer Snapshot ⇒ gleiche IDs/Hashes/Ledgerpfade".into(),
        }
    } else {
        Verdict::Reject {
            gate: "ReplayGate".into(),
            residue: Box::new(csa_residue(
                "replay_drift",
                "Snapshot-Wiederholung erzeugt abweichende Ledgerpfade",
            )),
        }
    }
}
