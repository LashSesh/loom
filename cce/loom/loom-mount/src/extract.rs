//! extract — CAS_BLOB-Rueckgewinnung (Oekosystem-Karte §2/E1, Etappe
//! X1a/b): Artefakt-/Kind-Workbody-Bytes byte-identisch zurueckgeben.
//! Reine Dekodierung + Digest-Gegenprobe (Reader-Prinzip) — kein
//! Motor-Zugriff, kein Schreibpfad.

use crate::LoomHandle;
use loom_canon::Cv;
use loom_format::sha256::sha256;
use loom_format::{KIND_ARTIFACT, KIND_CAS_BLOB};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExtractError {
    /// Kein ARTIFACT-Segment vorhanden.
    NoArtifact,
    /// Kein CAS_BLOB-Segment vorhanden — aeltere/illustrative Container
    /// (z. B. R1–R8) tragen nur Digest-Metadaten; das ist ein Fakt ueber
    /// den Container, kein Fehler des Extraktors.
    NoCasBlob,
    /// CV liess sich nicht wie erwartet dekodieren.
    Malformed(String),
    /// Kein CAS_BLOB passt zum deklarierten byte_digest — sichtbar
    /// gemeldet, nie stillschweigend der erstbeste zurueckgegeben.
    DigestMismatch { declared: String },
}

fn cv_get<'a>(v: &'a Cv, key: &str) -> Option<&'a Cv> {
    match v {
        Cv::Map(entries) => entries
            .iter()
            .find(|(k, _)| matches!(k, Cv::Text(s) if s == key))
            .map(|(_, v)| v),
        _ => None,
    }
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// Alle CAS_BLOB-Frames, dekodiert zu ihren rohen Bytes (Segtab-
/// Reihenfolge, digest-sortiert) — Grundlage fuer den Einzel-Artefakt-
/// Fall (X1a) UND die Mappen-Extraktion (X1b, mehrere Kinder).
pub fn all_cas_blobs(handle: &LoomHandle) -> Result<Vec<Vec<u8>>, ExtractError> {
    let mut out = Vec::new();
    for (entry, frame) in &handle.decoded.frames {
        if entry.kind == KIND_CAS_BLOB {
            let v = loom_canon::decode(&frame.payload)
                .map_err(|e| ExtractError::Malformed(format!("{e:?}")))?;
            match v {
                Cv::Bytes(b) => out.push(b),
                other => {
                    return Err(ExtractError::Malformed(format!(
                        "CAS_BLOB-Payload ist kein Bytes-Wert: {other:?}"
                    )))
                }
            }
        }
    }
    if out.is_empty() {
        return Err(ExtractError::NoCasBlob);
    }
    Ok(out)
}

/// X1a: das EINE materialisierte Artefakt. Liest `byte_digest` aus dem
/// ARTIFACT-Segment, sucht den CAS_BLOB mit genau diesem sha256, prueft
/// GEGEN bevor die Bytes zurueckgehen (kein blindes Vertrauen auf
/// Positions-/Reihenfolge-Zufall — der Digest entscheidet, nichts sonst).
pub fn extract_artifact(handle: &LoomHandle) -> Result<Vec<u8>, ExtractError> {
    let artifact_frame = handle
        .decoded
        .frames
        .iter()
        .find(|(e, _)| e.kind == KIND_ARTIFACT)
        .ok_or(ExtractError::NoArtifact)?
        .1
        .clone();
    let artifact_cv = loom_canon::decode(&artifact_frame.payload)
        .map_err(|e| ExtractError::Malformed(format!("{e:?}")))?;
    let declared = cv_get(&artifact_cv, "two_digest")
        .and_then(|td| cv_get(td, "byte_digest"))
        .and_then(|v| match v {
            Cv::Text(s) => Some(s.clone()),
            _ => None,
        })
        .ok_or_else(|| ExtractError::Malformed("kein two_digest.byte_digest".to_string()))?;

    for bytes in all_cas_blobs(handle)? {
        if hex(&sha256(&bytes)) == declared {
            return Ok(bytes);
        }
    }
    Err(ExtractError::DigestMismatch { declared })
}
