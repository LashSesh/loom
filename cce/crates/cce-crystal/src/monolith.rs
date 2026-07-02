//! Monolith (Teil 3.4): irreversibles, append-only Commit-Ereignis eines
//! Kristalls. Unveraenderlich, signiert, im Ledger.

use cce_core::ledger::{Ledger, LedgerEventKind};
use cce_core::signature::Digest;

#[derive(Debug, Clone)]
pub struct Monolith {
    pub crystal_class: Digest,
    pub ledger_seq: u64,
    pub entry_hash: Digest,
}

/// Append(ℓ_k, π(K_k)): der einzige Weg, einen Monolithen zu erzeugen.
pub fn commit_monolith(ledger: &mut Ledger, crystal_class: Digest) -> Monolith {
    let e = ledger.append(LedgerEventKind::Commit, crystal_class);
    Monolith {
        crystal_class,
        ledger_seq: e.seq,
        entry_hash: e.entry_hash,
    }
}
