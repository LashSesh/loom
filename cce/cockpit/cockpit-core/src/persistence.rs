//! PersistenceAdapter (S3.7): lokal, content-adressiert, sync-faehig
//! ausgelegt — Port ueber cce-store.

use cce_store::cas::{Cas, MemoryCas, ObjectKind};

pub trait PersistenceAdapter {
    /// Legt ein Artefakt content-adressiert im Workspace ab; liefert
    /// die Adresse (Ledger-verankerbar).
    fn store_artifact(&mut self, bytes: &[u8]) -> String;
}

#[derive(Default)]
pub struct LocalWorkspace {
    pub cas: MemoryCas,
}

impl PersistenceAdapter for LocalWorkspace {
    fn store_artifact(&mut self, bytes: &[u8]) -> String {
        self.cas.put(ObjectKind::Artifact, bytes).to_hex()
    }
}
