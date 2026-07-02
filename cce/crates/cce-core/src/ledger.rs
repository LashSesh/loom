//! Ledger (Teil 3.6/7.7): append-only Hash-Kette. `verify_ledger` erkennt
//! jede nachtraegliche Aenderung (INV-12). Der Ledger ist eine PROJEKTION —
//! ab G2 praezisiert als `Ledger = CommitProjection(HyperDAG)` (S9-A1).

use crate::signature::{sha256, Digest};
use crate::value::CanonValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LedgerEventKind {
    Decode,
    Project,
    Execute,
    Gate,
    Residue,
    Commit,
    Materialize,
    Decision,
    Replay,
}

impl LedgerEventKind {
    pub fn as_str(self) -> &'static str {
        match self {
            LedgerEventKind::Decode => "decode",
            LedgerEventKind::Project => "project",
            LedgerEventKind::Execute => "execute",
            LedgerEventKind::Gate => "gate",
            LedgerEventKind::Residue => "residue",
            LedgerEventKind::Commit => "commit",
            LedgerEventKind::Materialize => "materialize",
            LedgerEventKind::Decision => "decision",
            LedgerEventKind::Replay => "replay",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LedgerEvent {
    pub seq: u64,
    pub kind: LedgerEventKind,
    pub payload_digest: Digest,
    pub prev_hash: Digest,
    pub entry_hash: Digest,
}

impl LedgerEvent {
    fn compute_hash(seq: u64, kind: LedgerEventKind, payload: Digest, prev: Digest) -> Digest {
        let v = CanonValue::map([
            ("seq", CanonValue::Int(seq as i64)),
            ("kind", CanonValue::text(kind.as_str())),
            ("payload", CanonValue::Bytes(payload.0.to_vec())),
            ("prev", CanonValue::Bytes(prev.0.to_vec())),
        ]);
        sha256(&v.encode())
    }
}

/// Append-only Ledger. Es existiert KEIN Mutations-/Loesch-Pfad —
/// Unveraenderlichkeit ist baulich, nicht konventionell.
#[derive(Debug, Clone, Default)]
pub struct Ledger {
    events: Vec<LedgerEvent>,
}

pub const GENESIS: Digest = Digest([0u8; 32]);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LedgerError {
    ChainBroken { at_seq: u64, reason: String },
}

impl Ledger {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn append(&mut self, kind: LedgerEventKind, payload_digest: Digest) -> &LedgerEvent {
        let seq = self.events.len() as u64;
        let prev = self.events.last().map(|e| e.entry_hash).unwrap_or(GENESIS);
        let entry_hash = LedgerEvent::compute_hash(seq, kind, payload_digest, prev);
        self.events.push(LedgerEvent {
            seq,
            kind,
            payload_digest,
            prev_hash: prev,
            entry_hash,
        });
        self.events.last().expect("just pushed")
    }

    pub fn events(&self) -> &[LedgerEvent] {
        &self.events
    }

    pub fn head(&self) -> Digest {
        self.events.last().map(|e| e.entry_hash).unwrap_or(GENESIS)
    }

    /// INV-12: prueft die gesamte Hash-Kette; jede Manipulation wird erkannt.
    pub fn verify(&self) -> Result<(), LedgerError> {
        let mut prev = GENESIS;
        for (i, e) in self.events.iter().enumerate() {
            if e.seq != i as u64 {
                return Err(LedgerError::ChainBroken {
                    at_seq: i as u64,
                    reason: "Sequenzluecke".into(),
                });
            }
            if e.prev_hash != prev {
                return Err(LedgerError::ChainBroken {
                    at_seq: e.seq,
                    reason: "prev_hash gebrochen".into(),
                });
            }
            let expect = LedgerEvent::compute_hash(e.seq, e.kind, e.payload_digest, e.prev_hash);
            if expect != e.entry_hash {
                return Err(LedgerError::ChainBroken {
                    at_seq: e.seq,
                    reason: "entry_hash manipuliert".into(),
                });
            }
            prev = e.entry_hash;
        }
        Ok(())
    }

    /// Nur fuer Negativ-Tests: liefert eine manipulierte Kopie
    /// (der echte Ledger bleibt unveraendert — append-only).
    pub fn tampered_copy_for_tests(&self, seq: usize, new_payload: Digest) -> Ledger {
        let mut copy = self.clone();
        if let Some(e) = copy.events.get_mut(seq) {
            e.payload_digest = new_payload;
        }
        copy
    }
}

/// `verify_ledger` — freie Funktion, Name gemaess Spec (S9.5, Bauverfassung 9-B).
pub fn verify_ledger(l: &Ledger) -> Result<(), LedgerError> {
    l.verify()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// VC6/INV-12: Kette gruen; Manipulation wird erkannt.
    #[test]
    fn ledger_chain_verifies_and_detects_tamper() {
        let mut l = Ledger::new();
        l.append(LedgerEventKind::Decode, sha256(b"p"));
        l.append(LedgerEventKind::Gate, sha256(b"g"));
        l.append(LedgerEventKind::Commit, sha256(b"c"));
        assert!(verify_ledger(&l).is_ok());

        let bad = l.tampered_copy_for_tests(1, sha256(b"anders"));
        match verify_ledger(&bad) {
            Err(LedgerError::ChainBroken { at_seq, .. }) => assert_eq!(at_seq, 1),
            Ok(()) => panic!("Manipulation NICHT erkannt (INV-12 verletzt)"),
        }
    }

    /// Append-only: es existiert kein oeffentlicher Mutationspfad.
    #[test]
    fn append_only_head_moves_forward() {
        let mut l = Ledger::new();
        let h0 = l.head();
        l.append(LedgerEventKind::Execute, sha256(b"x"));
        assert_ne!(l.head(), h0);
        assert_eq!(l.events().len(), 1);
    }
}
