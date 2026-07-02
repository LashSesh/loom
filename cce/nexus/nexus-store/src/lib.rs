//! nexus-store — Ablage der Akquisition: RawObservations, CSUs und
//! EvidencePacks landen content-adressiert im CAS (cce-store); der
//! Snapshot-Index macht Replays adressierbar (fixer Snapshot ⇒
//! gleiche IDs/Hashes, CSA.13).

use cce_core::canonical::Canonicalize;
use cce_store::cas::{Cas, MemoryCas, ObjectKind};
use nexus_core::objects::{Csu, EvidencePack, RawObservation};

/// Akquisitions-Store: CAS + Snapshot-Verzeichnis.
#[derive(Default)]
pub struct NexusStore {
    pub cas: MemoryCas,
    /// snapshot_id → Liste der Roh-Digests (Replay-Index).
    pub snapshots: std::collections::BTreeMap<String, Vec<String>>,
}

impl NexusStore {
    pub fn put_raw(&mut self, raw: &RawObservation) -> String {
        let digest = self.cas.put(ObjectKind::Other, &raw.bytes);
        self.snapshots
            .entry(raw.snapshot_id.clone())
            .or_default()
            .push(digest.to_hex());
        digest.to_hex()
    }

    pub fn put_csu(&mut self, csu: &Csu) -> String {
        let bytes = csu.canonical_value().encode();
        self.cas.put(ObjectKind::Other, &bytes).to_hex()
    }

    /// EvidencePacks werden ueber ihre stabilen Kennfelder adressiert
    /// (evidence_id + raw_hash + csu_class).
    pub fn put_evidence(&mut self, pack: &EvidencePack) -> String {
        let bytes = format!(
            "{}|{}|{}",
            pack.evidence_id,
            pack.raw_hash.to_hex(),
            pack.csu_class.to_hex()
        );
        self.cas.put(ObjectKind::Other, bytes.as_bytes()).to_hex()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot_index_is_replay_stable() {
        let raw = RawObservation {
            locator: "l://1".to_string(),
            bytes: b"inhalt".to_vec(),
            fetched_via: "fixture".to_string(),
            snapshot_id: "snap-1".to_string(),
        };
        let mut s1 = NexusStore::default();
        let mut s2 = NexusStore::default();
        assert_eq!(s1.put_raw(&raw), s2.put_raw(&raw));
        assert_eq!(s1.snapshots, s2.snapshots);
    }
}
