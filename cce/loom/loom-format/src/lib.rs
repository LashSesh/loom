//! loom-format — LBC-1 Byte-Rahmen (LOOM-Standard Teil 2/3):
//! Praeambel (16 B), Frames, Footer (64 B), Kind-Registry.
//! Alle Mehrbyte-Zahlen im RAHMEN sind little-endian; Segment-PAYLOADS
//! sind dCBOR (big-endian-Konventionen dort sind CBOR-Sache).

pub mod sha256;

use sha256::sha256;

/// PNG-artiges Magic: \x89 "LOOM" CR LF SUB LF.
pub const MAGIC: [u8; 9] = [0x89, 0x4C, 0x4F, 0x4F, 0x4D, 0x0D, 0x0A, 0x1A, 0x0A];
pub const END_MAGIC: [u8; 8] = *b"LOOM_END";
pub const FORMAT_MAJOR: u8 = 0x01;
pub const FORMAT_MINOR: u8 = 0x00;
pub const PREAMBLE_LEN: usize = 16;
pub const FOOTER_LEN: usize = 64;
/// Multihash-Praefix sha2-256: Code 0x12, Laenge 0x20.
pub const MULTIHASH_SHA256: [u8; 2] = [0x12, 0x20];

// Kind-Registry (Teil 3.3 + Overlay 05 Teil E, minor-additiv).
pub const KIND_HEADER: u16 = 0x0000;
pub const KIND_MANIFEST: u16 = 0x0001;
pub const KIND_SEGTAB: u16 = 0x0002;
pub const KIND_CANON_DESC: u16 = 0x0003;
pub const KIND_TYPE_REGISTRY: u16 = 0x0004;
pub const KIND_CL_SUBSTRATE: u16 = 0x0010;
pub const KIND_PHC: u16 = 0x0011;
pub const KIND_SPIRAL_RATCHET: u16 = 0x0012;
pub const KIND_LEDGER: u16 = 0x0013;
pub const KIND_RESIDUE: u16 = 0x0014;
pub const KIND_GATE_REPORTS: u16 = 0x0015;
pub const KIND_EVIDENCE: u16 = 0x0016;
pub const KIND_REPLAY_MANIFEST: u16 = 0x0017;
pub const KIND_RUNTIME_PROFILE: u16 = 0x0018;
pub const KIND_CSA_NSB: u16 = 0x0020;
pub const KIND_HBM: u16 = 0x0021;
pub const KIND_ARTIFACT: u16 = 0x0030;
pub const KIND_CAS_BLOB: u16 = 0x0031;
pub const KIND_DOC: u16 = 0x0032;
pub const KIND_LIBRARY_WITNESS: u16 = 0x0033;
pub const KIND_SIGNATURE: u16 = 0x0050;
// Overlay 05 Teil E (L9c, minor-additiv):
pub const KIND_PROVIDER_MANIFEST: u16 = 0x0060;
pub const KIND_INFERENCE_PROFILE: u16 = 0x0061;
pub const KIND_INFERENCE_TRACE: u16 = 0x0062;
pub const KIND_CANDIDATE_OUTPUTS: u16 = 0x0063;
pub const KIND_TOOL_PROFILE: u16 = 0x0064;
pub const KIND_EXT_MIN: u16 = 0x7000;
pub const KIND_EXT_MAX: u16 = 0x7FFF;

pub const SEG_FLAG_COMPRESSED: u16 = 1 << 0;
pub const SEG_FLAG_NON_CORE: u16 = 1 << 1;
pub const SEG_FLAG_REQUIRED_UNDERSTAND: u16 = 1 << 2;

pub const FLAG_SEALED: u8 = 1 << 0;

/// Container-Klassen/Profile (Teil 2.5). S-E5 §2/§10(e) fuegt additiv
/// "norm" hinzu: die Speicherform des L9b-Normic-Memory-Workbody (eine
/// Norm IST ein .loom-Workbody, kein Verweis darauf). Dokument 18 §6
/// (P2) fuegt additiv "repo" hinzu: die Speicherform des
/// SWE-RepoWorkbody (ein zertifizierter Bauauftrag IST ein
/// .loom-Workbody). Dokument 20 §5 (P4) fuegt additiv "benchmark" hinzu:
/// die Speicherform eines Vergleichslaufs (die Ueberlegenheitsbehauptung
/// wird selbst ein zertifiziertes Artefakt).
pub const PROFILES: [&str; 9] = [
    "inspection",
    "workcell",
    "source",
    "hbm",
    "runtime",
    "full",
    "norm",
    "repo",
    "benchmark",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormatError {
    BadMagic,
    BadVersion { major: u8, minor: u8 },
    ReservedFlagSet,
    Truncated { need: usize, have: usize },
    BadEndMagic,
    BadFooterReserved,
    FrameDigestMismatch { kind: u16 },
    StoredExceedsDeclared { kind: u16 },
    CompressionUnsupported { kind: u16 },
}

/// Praeambel (Bytes 0..16).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Preamble {
    pub major: u8,
    pub minor: u8,
    pub flags: u8,
    pub header_len: u32,
}

impl Preamble {
    pub fn encode(&self) -> [u8; PREAMBLE_LEN] {
        let mut out = [0u8; PREAMBLE_LEN];
        out[..9].copy_from_slice(&MAGIC);
        out[9] = self.major;
        out[10] = self.minor;
        out[11] = self.flags;
        out[12..16].copy_from_slice(&self.header_len.to_le_bytes());
        out
    }

    pub fn decode(buf: &[u8]) -> Result<Self, FormatError> {
        if buf.len() < PREAMBLE_LEN {
            return Err(FormatError::Truncated {
                need: PREAMBLE_LEN,
                have: buf.len(),
            });
        }
        if buf[..9] != MAGIC {
            return Err(FormatError::BadMagic);
        }
        let (major, minor) = (buf[9], buf[10]);
        if major != FORMAT_MAJOR {
            return Err(FormatError::BadVersion { major, minor });
        }
        let flags = buf[11];
        if flags & 0b1111_1100 != 0 {
            return Err(FormatError::ReservedFlagSet);
        }
        Ok(Self {
            major,
            minor,
            flags,
            header_len: u32::from_le_bytes(buf[12..16].try_into().unwrap()),
        })
    }

    pub fn sealed(&self) -> bool {
        self.flags & FLAG_SEALED != 0
    }
}

/// Ein Segment-Frame (Teil 3.1). Kompression ist im Bau nicht aktiviert
/// (bit0 wird als CompressionUnsupported abgewiesen — zstd waere eine
/// externe Abhaengigkeit; sichtbare Schranke im G09-Bericht).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frame {
    pub kind: u16,
    pub seg_flags: u16,
    pub payload: Vec<u8>,
}

pub const FRAME_HEADER_LEN: usize = 54;

impl Frame {
    pub fn payload_digest(&self) -> [u8; 32] {
        sha256(&self.payload)
    }

    /// Multihash (34 B) ueber die unkomprimierte kanonische Payload.
    pub fn multihash(&self) -> [u8; 34] {
        let mut mh = [0u8; 34];
        mh[..2].copy_from_slice(&MULTIHASH_SHA256);
        mh[2..].copy_from_slice(&self.payload_digest());
        mh
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(FRAME_HEADER_LEN + self.payload.len());
        out.extend_from_slice(&self.kind.to_le_bytes());
        out.extend_from_slice(&self.seg_flags.to_le_bytes());
        out.extend_from_slice(&(self.payload.len() as u64).to_le_bytes());
        out.extend_from_slice(&(self.payload.len() as u64).to_le_bytes());
        out.extend_from_slice(&self.multihash());
        out.extend_from_slice(&self.payload);
        out
    }

    /// Dekodiert einen Frame ab `at`; prueft Digest VOR jeder weiteren
    /// Deserialisierung (Teil 9.2) und stored ≤ uncompressed als
    /// Bomben-Schranke (N13).
    pub fn decode(buf: &[u8], at: usize) -> Result<(Self, usize), FormatError> {
        let need = at + FRAME_HEADER_LEN;
        if buf.len() < need {
            return Err(FormatError::Truncated {
                need,
                have: buf.len(),
            });
        }
        let kind = u16::from_le_bytes(buf[at..at + 2].try_into().unwrap());
        let seg_flags = u16::from_le_bytes(buf[at + 2..at + 4].try_into().unwrap());
        let unc_len = u64::from_le_bytes(buf[at + 4..at + 12].try_into().unwrap());
        let stored_len = u64::from_le_bytes(buf[at + 12..at + 20].try_into().unwrap());
        let mut digest = [0u8; 34];
        digest.copy_from_slice(&buf[at + 20..at + 54]);
        if seg_flags & SEG_FLAG_COMPRESSED != 0 {
            return Err(FormatError::CompressionUnsupported { kind });
        }
        // Unkomprimiert MUSS stored == uncompressed sein; eine Deklaration
        // stored ≪ uncompressed waere die Bomben-Signatur (N13).
        if stored_len != unc_len {
            return Err(FormatError::StoredExceedsDeclared { kind });
        }
        let end = at + FRAME_HEADER_LEN + stored_len as usize;
        if buf.len() < end {
            return Err(FormatError::Truncated {
                need: end,
                have: buf.len(),
            });
        }
        let payload = buf[at + FRAME_HEADER_LEN..end].to_vec();
        let frame = Frame {
            kind,
            seg_flags,
            payload,
        };
        if frame.multihash() != digest {
            return Err(FormatError::FrameDigestMismatch { kind });
        }
        Ok((frame, end))
    }
}

/// Footer (letzte 64 Bytes, nur sealed).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Footer {
    pub segtab_offset: u64,
    pub segtab_stored_len: u64,
    pub core_root: [u8; 34],
}

impl Footer {
    pub fn encode(&self) -> [u8; FOOTER_LEN] {
        let mut out = [0u8; FOOTER_LEN];
        out[..8].copy_from_slice(&self.segtab_offset.to_le_bytes());
        out[8..16].copy_from_slice(&self.segtab_stored_len.to_le_bytes());
        out[16..50].copy_from_slice(&self.core_root);
        // [50..56) reserved = 0
        out[56..64].copy_from_slice(&END_MAGIC);
        out
    }

    pub fn decode(file: &[u8]) -> Result<Self, FormatError> {
        if file.len() < FOOTER_LEN {
            return Err(FormatError::Truncated {
                need: FOOTER_LEN,
                have: file.len(),
            });
        }
        let f = &file[file.len() - FOOTER_LEN..];
        if f[56..64] != END_MAGIC {
            return Err(FormatError::BadEndMagic);
        }
        if f[50..56].iter().any(|&b| b != 0) {
            return Err(FormatError::BadFooterReserved);
        }
        let mut core_root = [0u8; 34];
        core_root.copy_from_slice(&f[16..50]);
        Ok(Self {
            segtab_offset: u64::from_le_bytes(f[..8].try_into().unwrap()),
            segtab_stored_len: u64::from_le_bytes(f[8..16].try_into().unwrap()),
            core_root,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preamble_roundtrip_and_negatives() {
        let p = Preamble {
            major: 1,
            minor: 0,
            flags: FLAG_SEALED,
            header_len: 42,
        };
        let b = p.encode();
        assert_eq!(Preamble::decode(&b).unwrap(), p);
        let mut bad = b;
        bad[0] = 0x50; // "P..."
        assert_eq!(Preamble::decode(&bad), Err(FormatError::BadMagic));
        let mut v2 = p.encode();
        v2[9] = 0x02;
        assert!(matches!(
            Preamble::decode(&v2),
            Err(FormatError::BadVersion { .. })
        ));
        let mut rf = p.encode();
        rf[11] = 0b0000_0100;
        assert_eq!(Preamble::decode(&rf), Err(FormatError::ReservedFlagSet));
    }

    #[test]
    fn frame_roundtrip_digest_checked_before_use() {
        let f = Frame {
            kind: KIND_MANIFEST,
            seg_flags: 0,
            payload: vec![1, 2, 3],
        };
        let enc = f.encode();
        let (back, end) = Frame::decode(&enc, 0).unwrap();
        assert_eq!(back, f);
        assert_eq!(end, enc.len());
        // Payload-Manipulation faellt am Digest, nicht erst in der Semantik.
        let mut tampered = enc.clone();
        let last = tampered.len() - 1;
        tampered[last] ^= 0xff;
        assert!(matches!(
            Frame::decode(&tampered, 0),
            Err(FormatError::FrameDigestMismatch { .. })
        ));
        // Bomben-Signatur: stored_len < uncompressed_len
        let mut bomb = enc;
        bomb[4..12].copy_from_slice(&100u64.to_le_bytes()); // unc=100, stored=3
        assert!(matches!(
            Frame::decode(&bomb, 0),
            Err(FormatError::StoredExceedsDeclared { .. })
        ));
    }

    #[test]
    fn footer_roundtrip() {
        let mut root = [0u8; 34];
        root[..2].copy_from_slice(&MULTIHASH_SHA256);
        let f = Footer {
            segtab_offset: 512,
            segtab_stored_len: 99,
            core_root: root,
        };
        let mut file = vec![0u8; 100];
        file.extend_from_slice(&f.encode());
        assert_eq!(Footer::decode(&file).unwrap(), f);
    }
}
