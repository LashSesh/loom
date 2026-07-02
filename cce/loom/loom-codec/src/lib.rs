//! loom-codec — pack/seal (LOOM-Standard Teil 3/4, Bauordnung B3/B5):
//! Segmente sammeln → SEGTAB (eintragslos, sortiert nach (kind,digest),
//! Header-Pseudo-Eintrag 0x0000) → Merkle-core_root (Domain-Separation
//! 0x00/0x01, promote-odd) → Footer. `seal --canonical` erzeugt das
//! Byte-Konformanzprofil canonical-stored (Golden-File-Grundlage).

use loom_canon::Cv;
use loom_format::sha256::sha256;
use loom_format::{
    Footer, Frame, Preamble, FLAG_SEALED, FORMAT_MAJOR, FORMAT_MINOR, KIND_HEADER, KIND_SEGTAB,
    MULTIHASH_SHA256, SEG_FLAG_NON_CORE,
};

/// Ein zu packendes Segment (logisch).
#[derive(Debug, Clone)]
pub struct Segment {
    pub kind: u16,
    pub seg_flags: u16,
    pub payload: Vec<u8>,
    /// Abhaengigkeiten als Digests (loom:seg:sha256:<hex>) — nie Offsets.
    pub deps: Vec<[u8; 34]>,
}

impl Segment {
    pub fn canonical(kind: u16, value: &Cv) -> Result<Segment, loom_canon::CanonError> {
        Ok(Segment { kind, seg_flags: 0, payload: value.encode()?, deps: vec![] })
    }

    pub fn multihash(&self) -> [u8; 34] {
        let mut mh = [0u8; 34];
        mh[..2].copy_from_slice(&MULTIHASH_SHA256);
        mh[2..].copy_from_slice(&sha256(&self.payload));
        mh
    }
}

/// Header-Segment (dCBOR-Map, Teil 2.3).
pub fn build_header(container_class: &str, profiles_required: &[&str]) -> Cv {
    Cv::map(vec![
        ("format_profile", Cv::Text("LBC-1".into())),
        ("digest_alg", Cv::Uint(0x12)),
        ("canon_id", Cv::Text("loom-canon-1".into())),
        (
            "canon_rules_digest",
            Cv::Bytes(sha256(loom_canon::CANON_RULES_TEXT.as_bytes()).to_vec()),
        ),
        ("container_class", Cv::Text(container_class.into())),
        (
            "profiles_required",
            Cv::Array(profiles_required.iter().map(|p| Cv::Text((*p).into())).collect()),
        ),
        ("profiles_optional", Cv::Array(vec![])),
        (
            "min_reader_version",
            Cv::map(vec![
                ("major", Cv::Uint(u64::from(FORMAT_MAJOR))),
                ("minor", Cv::Uint(u64::from(FORMAT_MINOR))),
            ]),
        ),
    ])
}

/// Merkle-Blatt: H(0x00 ‖ dCBOR(kind, digest, uncompressed_len)).
fn leaf_hash(kind: u16, digest: &[u8; 34], unc_len: u64) -> [u8; 32] {
    let entry = Cv::Array(vec![
        Cv::Uint(u64::from(kind)),
        Cv::Bytes(digest.to_vec()),
        Cv::Uint(unc_len),
    ]);
    let enc = entry.encode().expect("kanonischer Blatt-Eintrag");
    let mut buf = Vec::with_capacity(1 + enc.len());
    buf.push(0x00);
    buf.extend_from_slice(&enc);
    sha256(&buf)
}

/// core_root ueber alle non_core=0-Eintraege der (bereits sortierten)
/// Tabelle; SEGTAB selbst ist EINTRAGSLOS (Teil 3.2/4.2).
pub fn merkle_core_root(leaves: &[[u8; 32]]) -> [u8; 32] {
    if leaves.is_empty() {
        return sha256(&[0x00]);
    }
    let mut level: Vec<[u8; 32]> = leaves.to_vec();
    while level.len() > 1 {
        let mut next = Vec::with_capacity(level.len().div_ceil(2));
        let mut i = 0;
        while i + 1 < level.len() {
            let mut buf = Vec::with_capacity(65);
            buf.push(0x01);
            buf.extend_from_slice(&level[i]);
            buf.extend_from_slice(&level[i + 1]);
            next.push(sha256(&buf));
            i += 2;
        }
        if i < level.len() {
            // promote odd: unveraendert hochziehen (keine Duplikation).
            next.push(level[i]);
        }
        level = next;
    }
    level[0]
}

/// SEGTAB-Eintrag (logisch).
#[derive(Debug, Clone)]
pub struct SegtabEntry {
    pub kind: u16,
    pub digest: [u8; 34],
    pub uncompressed_len: u64,
    pub offset: u64,
    pub stored_len: u64,
    pub seg_flags: u16,
    pub deps: Vec<[u8; 34]>,
}

impl SegtabEntry {
    pub fn to_cv(&self) -> Cv {
        Cv::Array(vec![
            Cv::Uint(u64::from(self.kind)),
            Cv::Bytes(self.digest.to_vec()),
            Cv::Uint(self.uncompressed_len),
            Cv::Uint(self.offset),
            Cv::Uint(self.stored_len),
            Cv::Uint(u64::from(self.seg_flags)),
            Cv::Array(self.deps.iter().map(|d| Cv::Bytes(d.to_vec())).collect()),
        ])
    }

    pub fn from_cv(v: &Cv) -> Option<SegtabEntry> {
        let Cv::Array(items) = v else { return None };
        if items.len() != 7 {
            return None;
        }
        let u = |i: usize| match &items[i] {
            Cv::Uint(n) => Some(*n),
            _ => None,
        };
        let Cv::Bytes(d) = &items[1] else { return None };
        if d.len() != 34 {
            return None;
        }
        let mut digest = [0u8; 34];
        digest.copy_from_slice(d);
        let Cv::Array(dep_items) = &items[6] else { return None };
        let mut deps = Vec::new();
        for di in dep_items {
            let Cv::Bytes(db) = di else { return None };
            if db.len() != 34 {
                return None;
            }
            let mut dd = [0u8; 34];
            dd.copy_from_slice(db);
            deps.push(dd);
        }
        Some(SegtabEntry {
            kind: u(0)? as u16,
            digest,
            uncompressed_len: u(2)?,
            offset: u(3)?,
            stored_len: u(4)?,
            seg_flags: u(5)? as u16,
            deps,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackError {
    Canon(String),
    DuplicateSegtab,
}

/// Ein gepackter (versiegelter) Container im canonical-stored-Profil:
/// alle Segmente unkomprimiert, physisch in Tabellenordnung.
pub struct Sealed {
    pub bytes: Vec<u8>,
    pub core_root: [u8; 34],
    pub segtab: Vec<SegtabEntry>,
}

/// pack+seal (canonical-stored): deterministisch aus den logischen
/// Segmenten — gleiche Segmente ⇒ gleiche Bytes (C0-Grundlage).
pub fn seal_canonical(
    container_class: &str,
    profiles_required: &[&str],
    segments: &[Segment],
) -> Result<Sealed, PackError> {
    if segments.iter().any(|s| s.kind == KIND_SEGTAB) {
        return Err(PackError::DuplicateSegtab);
    }
    let header_cv = build_header(container_class, profiles_required);
    let header_bytes = header_cv.encode().map_err(|e| PackError::Canon(format!("{e:?}")))?;
    let header_seg = Segment { kind: KIND_HEADER, seg_flags: 0, payload: header_bytes.clone(), deps: vec![] };

    // Dedupe-Pflicht: gleicher Digest genau einmal physisch.
    let mut logical: Vec<Segment> = Vec::new();
    for s in segments {
        if !logical.iter().any(|l| l.kind == s.kind && l.multihash() == s.multihash()) {
            logical.push(s.clone());
        }
    }
    // Tabellenordnung: (kind, digest) — Header-Pseudo-Eintrag inklusive.
    logical.sort_by_key(|s| (s.kind, s.multihash()));

    // Physisches Layout planen: Praeambel + Header-Frame + Segmente in
    // Tabellenordnung + SEGTAB + Footer.
    let preamble = Preamble {
        major: FORMAT_MAJOR,
        minor: FORMAT_MINOR,
        flags: FLAG_SEALED,
        header_len: header_seg.encode_len() as u32,
    };
    let mut offset = loom_format::PREAMBLE_LEN as u64;
    let header_frame = Frame { kind: KIND_HEADER, seg_flags: 0, payload: header_seg.payload.clone() };
    let header_offset = offset;
    offset += header_frame.encode().len() as u64;

    let mut entries: Vec<SegtabEntry> = Vec::new();
    entries.push(SegtabEntry {
        kind: KIND_HEADER,
        digest: header_seg.multihash(),
        uncompressed_len: header_seg.payload.len() as u64,
        offset: header_offset,
        stored_len: header_seg.payload.len() as u64,
        seg_flags: 0,
        deps: vec![],
    });
    let mut frames: Vec<Frame> = Vec::new();
    for s in &logical {
        let frame = Frame { kind: s.kind, seg_flags: s.seg_flags, payload: s.payload.clone() };
        entries.push(SegtabEntry {
            kind: s.kind,
            digest: s.multihash(),
            uncompressed_len: s.payload.len() as u64,
            offset,
            stored_len: s.payload.len() as u64,
            seg_flags: s.seg_flags,
            deps: s.deps.clone(),
        });
        offset += frame.encode().len() as u64;
        frames.push(frame);
    }
    entries.sort_by_key(|e| (e.kind, e.digest));

    // core_root: alle Eintraege mit non_core=0.
    let leaves: Vec<[u8; 32]> = entries
        .iter()
        .filter(|e| e.seg_flags & SEG_FLAG_NON_CORE == 0)
        .map(|e| leaf_hash(e.kind, &e.digest, e.uncompressed_len))
        .collect();
    let root32 = merkle_core_root(&leaves);
    let mut core_root = [0u8; 34];
    core_root[..2].copy_from_slice(&MULTIHASH_SHA256);
    core_root[2..].copy_from_slice(&root32);

    // SEGTAB-Frame (eintragslos: kein Selbsteintrag).
    let segtab_cv = Cv::Array(entries.iter().map(|e| e.to_cv()).collect());
    let segtab_payload = segtab_cv.encode().map_err(|e| PackError::Canon(format!("{e:?}")))?;
    let segtab_frame = Frame { kind: KIND_SEGTAB, seg_flags: 0, payload: segtab_payload };
    let segtab_offset = offset;
    let segtab_encoded = segtab_frame.encode();

    let footer = Footer {
        segtab_offset,
        segtab_stored_len: segtab_encoded.len() as u64,
        core_root,
    };

    let mut bytes = Vec::new();
    bytes.extend_from_slice(&preamble.encode());
    bytes.extend_from_slice(&header_frame.encode());
    for f in &frames {
        bytes.extend_from_slice(&f.encode());
    }
    bytes.extend_from_slice(&segtab_encoded);
    bytes.extend_from_slice(&footer.encode());

    Ok(Sealed { bytes, core_root, segtab: entries })
}

impl Segment {
    fn encode_len(&self) -> usize {
        loom_format::FRAME_HEADER_LEN + self.payload.len()
    }
}

/// Dekodiert einen versiegelten Container in (Preamble, Footer, SEGTAB,
/// Frames) — Digest-Pruefung je Frame VOR jeder Semantik.
pub struct Decoded {
    pub preamble: Preamble,
    pub footer: Footer,
    pub segtab: Vec<SegtabEntry>,
    pub frames: Vec<(SegtabEntry, Frame)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeError {
    Format(loom_format::FormatError),
    Canon(String),
    SegtabNotTable,
    SegtabEntryMalformed { index: usize },
    SegtabUnsorted { index: usize },
    SegtabSelfEntry,
    FrameNotAtDeclaredOffset { kind: u16 },
    FrameDigestNotInTable { kind: u16 },
    NotSealed,
}

pub fn decode_sealed(bytes: &[u8]) -> Result<Decoded, DecodeError> {
    let preamble = Preamble::decode(bytes).map_err(DecodeError::Format)?;
    if !preamble.sealed() {
        return Err(DecodeError::NotSealed);
    }
    let footer = Footer::decode(bytes).map_err(DecodeError::Format)?;
    let (segtab_frame, _end) = Frame::decode(bytes, footer.segtab_offset as usize)
        .map_err(DecodeError::Format)?;
    if segtab_frame.kind != KIND_SEGTAB {
        return Err(DecodeError::SegtabNotTable);
    }
    let segtab_cv =
        loom_canon::decode(&segtab_frame.payload).map_err(|e| DecodeError::Canon(format!("{e:?}")))?;
    let Cv::Array(items) = segtab_cv else {
        return Err(DecodeError::SegtabNotTable);
    };
    let mut entries = Vec::new();
    for (i, item) in items.iter().enumerate() {
        let e = SegtabEntry::from_cv(item).ok_or(DecodeError::SegtabEntryMalformed { index: i })?;
        if e.kind == KIND_SEGTAB {
            return Err(DecodeError::SegtabSelfEntry);
        }
        entries.push(e);
    }
    for i in 1..entries.len() {
        if (entries[i - 1].kind, entries[i - 1].digest) > (entries[i].kind, entries[i].digest) {
            return Err(DecodeError::SegtabUnsorted { index: i });
        }
    }
    let mut frames = Vec::new();
    for e in &entries {
        let (frame, _) = Frame::decode(bytes, e.offset as usize).map_err(DecodeError::Format)?;
        if frame.kind != e.kind {
            return Err(DecodeError::FrameNotAtDeclaredOffset { kind: e.kind });
        }
        let mut mh = [0u8; 34];
        mh[..2].copy_from_slice(&MULTIHASH_SHA256);
        mh[2..].copy_from_slice(&sha256(&frame.payload));
        if mh != e.digest {
            return Err(DecodeError::FrameDigestNotInTable { kind: e.kind });
        }
        frames.push((e.clone(), frame));
    }
    Ok(Decoded { preamble, footer, segtab: entries, frames })
}

/// core_root aus der dekodierten Tabelle neu berechnen (Verify-Baustein).
pub fn recompute_core_root(entries: &[SegtabEntry]) -> [u8; 34] {
    let leaves: Vec<[u8; 32]> = entries
        .iter()
        .filter(|e| e.seg_flags & SEG_FLAG_NON_CORE == 0)
        .map(|e| leaf_hash(e.kind, &e.digest, e.uncompressed_len))
        .collect();
    let root32 = merkle_core_root(&leaves);
    let mut mh = [0u8; 34];
    mh[..2].copy_from_slice(&MULTIHASH_SHA256);
    mh[2..].copy_from_slice(&root32);
    mh
}

#[cfg(test)]
mod tests {
    use super::*;
    use loom_format::{KIND_CANON_DESC, KIND_MANIFEST};

    fn manifest_cv() -> Cv {
        Cv::map(vec![
            ("title", Cv::Text("test".into())),
            ("container_class", Cv::Text("inspection".into())),
        ])
    }

    fn canon_desc() -> Segment {
        Segment {
            kind: KIND_CANON_DESC,
            seg_flags: 0,
            payload: Cv::Text(loom_canon::CANON_RULES_TEXT.into()).encode().unwrap(),
            deps: vec![],
        }
    }

    #[test]
    fn seal_decode_roundtrip_and_root_stable() {
        let m = Segment::canonical(KIND_MANIFEST, &manifest_cv()).unwrap();
        let sealed1 = seal_canonical("inspection", &["inspection"], &[m.clone(), canon_desc()]).unwrap();
        // Reihenfolge der Eingabe-Segmente ist egal (C1-Kern):
        let sealed2 = seal_canonical("inspection", &["inspection"], &[canon_desc(), m]).unwrap();
        assert_eq!(sealed1.bytes, sealed2.bytes, "canonical-stored ist byte-deterministisch");
        assert_eq!(sealed1.core_root, sealed2.core_root);
        let dec = decode_sealed(&sealed1.bytes).unwrap();
        assert_eq!(recompute_core_root(&dec.segtab), dec.footer.core_root);
        // Header-Pseudo-Eintrag vorhanden, SEGTAB eintragslos:
        assert!(dec.segtab.iter().any(|e| e.kind == KIND_HEADER));
        assert!(!dec.segtab.iter().any(|e| e.kind == KIND_SEGTAB));
    }

    #[test]
    fn tamper_changes_root() {
        let m = Segment::canonical(KIND_MANIFEST, &manifest_cv()).unwrap();
        let sealed = seal_canonical("inspection", &["inspection"], &[m, canon_desc()]).unwrap();
        let dec = decode_sealed(&sealed.bytes).unwrap();
        let mut entries = dec.segtab.clone();
        // Tabellenmanipulation: ein Digest-Byte kippen ⇒ Wurzel aendert sich.
        entries[1].digest[10] ^= 0xff;
        assert_ne!(recompute_core_root(&entries), dec.footer.core_root);
    }

    #[test]
    fn dedupe_same_digest_once() {
        let m = Segment::canonical(KIND_MANIFEST, &manifest_cv()).unwrap();
        let sealed =
            seal_canonical("inspection", &["inspection"], &[m.clone(), m.clone(), canon_desc()])
                .unwrap();
        let dec = decode_sealed(&sealed.bytes).unwrap();
        let manifests = dec.segtab.iter().filter(|e| e.kind == KIND_MANIFEST).count();
        assert_eq!(manifests, 1, "Dedupe-Pflicht beim Seal");
    }
}
