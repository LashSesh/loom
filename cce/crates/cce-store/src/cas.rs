//! Unveraenderlicher, inhaltsadressierter Speicher (S9.1): Objekte werden
//! nach ihrem Digest abgelegt, NIE mutiert (Aenderung = neues Objekt),
//! dedupliziert, integritaetsgeprueft (Digest-Pruefung beim Lesen).

use cce_core::signature::{sha256, Digest};
use std::collections::BTreeMap;
use std::path::PathBuf;

/// Adressierte Objektarten (S9-A1/S9-A4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ObjectKind {
    Crystal,
    PhaseBlock,
    Mef,
    Frontier,
    ReplayPack,
    ResidueReport,
    Artifact,
    LibraryAsset,
    Other,
}

impl ObjectKind {
    pub fn as_str(self) -> &'static str {
        match self {
            ObjectKind::Crystal => "crystal",
            ObjectKind::PhaseBlock => "phaseblock",
            ObjectKind::Mef => "mef",
            ObjectKind::Frontier => "frontier",
            ObjectKind::ReplayPack => "replay_pack",
            ObjectKind::ResidueReport => "residue_report",
            ObjectKind::Artifact => "artifact",
            ObjectKind::LibraryAsset => "library_asset",
            ObjectKind::Other => "other",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CasError {
    NotFound(Digest),
    /// Beschaedigtes Objekt: gespeicherte Bytes passen nicht zum Digest.
    IntegrityViolation(Digest),
    Io(String),
}

/// Der CAS-Vertrag: put ist idempotent (Dedup), get prueft Integritaet.
pub trait Cas {
    fn put(&mut self, kind: ObjectKind, bytes: &[u8]) -> Digest;
    fn get(&self, digest: Digest) -> Result<Vec<u8>, CasError>;
    fn contains(&self, digest: Digest) -> bool;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// In-Memory-CAS (Tests, fluechtige Laeufe).
#[derive(Debug, Default)]
pub struct MemoryCas {
    objects: BTreeMap<Digest, (ObjectKind, Vec<u8>)>,
}

impl MemoryCas {
    pub fn new() -> Self {
        Self::default()
    }

    /// Nur fuer Negativ-Tests: beschaedigt ein Objekt in einer KOPIE.
    pub fn corrupted_copy_for_tests(&self, digest: Digest) -> MemoryCas {
        let mut copy = MemoryCas {
            objects: self.objects.clone(),
        };
        if let Some((_, bytes)) = copy.objects.get_mut(&digest) {
            if let Some(b) = bytes.first_mut() {
                *b ^= 0xff;
            }
        }
        copy
    }
}

impl Cas for MemoryCas {
    fn put(&mut self, kind: ObjectKind, bytes: &[u8]) -> Digest {
        let d = sha256(bytes);
        self.objects.entry(d).or_insert((kind, bytes.to_vec()));
        d
    }

    fn get(&self, digest: Digest) -> Result<Vec<u8>, CasError> {
        let (_, bytes) = self
            .objects
            .get(&digest)
            .ok_or(CasError::NotFound(digest))?;
        if sha256(bytes) != digest {
            return Err(CasError::IntegrityViolation(digest));
        }
        Ok(bytes.clone())
    }

    fn contains(&self, digest: Digest) -> bool {
        self.objects.contains_key(&digest)
    }

    fn len(&self) -> usize {
        self.objects.len()
    }
}

/// Dateisystem-CAS: `<root>/objects/<kind>/<hex[0..2]>/<hex>` —
/// unveraenderlich (write-once), dedupliziert, selbst-pruefend.
#[derive(Debug)]
pub struct FsCas {
    root: PathBuf,
}

impl FsCas {
    pub fn open(root: impl Into<PathBuf>) -> Result<FsCas, CasError> {
        let root = root.into();
        std::fs::create_dir_all(root.join("objects")).map_err(|e| CasError::Io(e.to_string()))?;
        Ok(FsCas { root })
    }

    fn path_for(&self, digest: Digest) -> PathBuf {
        let hex = digest.to_hex();
        self.root.join("objects").join(&hex[0..2]).join(hex)
    }
}

impl Cas for FsCas {
    fn put(&mut self, _kind: ObjectKind, bytes: &[u8]) -> Digest {
        let d = sha256(bytes);
        let path = self.path_for(d);
        if !path.exists() {
            if let Some(parent) = path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            let _ = std::fs::write(&path, bytes);
        }
        d
    }

    fn get(&self, digest: Digest) -> Result<Vec<u8>, CasError> {
        let bytes = std::fs::read(self.path_for(digest)).map_err(|_| CasError::NotFound(digest))?;
        if sha256(&bytes) != digest {
            return Err(CasError::IntegrityViolation(digest));
        }
        Ok(bytes)
    }

    fn contains(&self, digest: Digest) -> bool {
        self.path_for(digest).exists()
    }

    fn len(&self) -> usize {
        walk_count(&self.root.join("objects"))
    }
}

fn walk_count(dir: &std::path::Path) -> usize {
    std::fs::read_dir(dir)
        .map(|entries| {
            entries
                .flatten()
                .map(|e| {
                    if e.path().is_dir() {
                        walk_count(&e.path())
                    } else {
                        1
                    }
                })
                .sum()
        })
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// S9.1: put idempotent (Dedup), get integritaetsgeprueft.
    #[test]
    fn cas_roundtrip_and_dedup() {
        let mut cas = MemoryCas::new();
        let d1 = cas.put(ObjectKind::Crystal, b"inhalt");
        let d2 = cas.put(ObjectKind::Crystal, b"inhalt");
        assert_eq!(d1, d2);
        assert_eq!(cas.len(), 1);
        assert_eq!(cas.get(d1).unwrap(), b"inhalt");
    }

    /// S9.7: beschaedigtes Objekt faellt bei der Digest-Pruefung durch.
    #[test]
    fn corruption_is_detected() {
        let mut cas = MemoryCas::new();
        let d = cas.put(ObjectKind::Mef, b"payload");
        let bad = cas.corrupted_copy_for_tests(d);
        assert_eq!(bad.get(d), Err(CasError::IntegrityViolation(d)));
    }
}
