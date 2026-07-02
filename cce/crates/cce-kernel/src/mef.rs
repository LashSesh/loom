//! MEF-Block: Evidence-/Proof-Payload (Strukturpause, Rebase §1.1).
//! R-6 (MEF-Feinformat) wird HIER geschlossen — deklariertes Byte-Encoding:
//!
//! ```text
//! MEF-1  :=  "MEF1" (4 Byte Magic)
//!         ‖  version   (1 Byte, 0x01)
//!         ‖  kind_len  (u16 BE) ‖ kind (UTF-8)
//!         ‖  payload_len (u64 BE) ‖ payload (kanonische CanonValue-Bytes)
//!         ‖  payload_digest (32 Byte SHA-256 ueber payload)
//! ```
//!
//! Deterministisch (kanonische Payload), content-adressiert, floatfrei.

use cce_core::signature::{sha256, Digest};
use cce_core::value::CanonValue;

pub const MEF_MAGIC: &[u8; 4] = b"MEF1";
pub const MEF_VERSION: u8 = 0x01;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MefBlock {
    pub kind: String,
    pub payload: CanonValue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MefError {
    BadMagic,
    BadVersion(u8),
    Truncated,
    DigestMismatch,
    BadPayload,
}

impl MefBlock {
    pub fn new(kind: &str, payload: CanonValue) -> Self {
        Self {
            kind: kind.to_string(),
            payload: payload.normalize(),
        }
    }

    pub fn digest(&self) -> Digest {
        sha256(&self.payload.encode())
    }

    /// Serialisierung nach MEF-1 (deklariertes Byte-Encoding).
    pub fn to_bytes(&self) -> Vec<u8> {
        let payload_bytes = self.payload.encode();
        let mut out = Vec::new();
        out.extend_from_slice(MEF_MAGIC);
        out.push(MEF_VERSION);
        out.extend_from_slice(&(self.kind.len() as u16).to_be_bytes());
        out.extend_from_slice(self.kind.as_bytes());
        out.extend_from_slice(&(payload_bytes.len() as u64).to_be_bytes());
        out.extend_from_slice(&payload_bytes);
        out.extend_from_slice(&sha256(&payload_bytes).0);
        out
    }

    /// Deserialisierung mit Digest-Pruefung VOR semantischer Nutzung.
    pub fn from_bytes(data: &[u8]) -> Result<MefBlock, MefError> {
        if data.len() < 4 || &data[0..4] != MEF_MAGIC {
            return Err(MefError::BadMagic);
        }
        if data.len() < 5 {
            return Err(MefError::Truncated);
        }
        if data[4] != MEF_VERSION {
            return Err(MefError::BadVersion(data[4]));
        }
        let mut pos = 5usize;
        let need = |pos: usize, n: usize| {
            if pos + n > data.len() {
                Err(MefError::Truncated)
            } else {
                Ok(())
            }
        };
        need(pos, 2)?;
        let kind_len = u16::from_be_bytes([data[pos], data[pos + 1]]) as usize;
        pos += 2;
        need(pos, kind_len)?;
        let kind = String::from_utf8(data[pos..pos + kind_len].to_vec())
            .map_err(|_| MefError::BadPayload)?;
        pos += kind_len;
        need(pos, 8)?;
        let plen = u64::from_be_bytes(data[pos..pos + 8].try_into().expect("8")) as usize;
        pos += 8;
        need(pos, plen)?;
        let payload_bytes = &data[pos..pos + plen];
        pos += plen;
        need(pos, 32)?;
        let declared = &data[pos..pos + 32];
        if sha256(payload_bytes).0 != *declared {
            return Err(MefError::DigestMismatch);
        }
        let payload = decode_canon(payload_bytes).ok_or(MefError::BadPayload)?;
        Ok(MefBlock { kind, payload })
    }
}

/// Dekodierung der kanonischen CanonValue-Bytes (Umkehrung von encode()).
pub fn decode_canon(data: &[u8]) -> Option<CanonValue> {
    let (v, rest) = decode_at(data)?;
    if !rest.is_empty() {
        return None;
    }
    Some(v)
}

fn decode_at(data: &[u8]) -> Option<(CanonValue, &[u8])> {
    let (&tag, rest) = data.split_first()?;
    match tag {
        0x00 => Some((CanonValue::Null, rest)),
        0x01 => {
            let (&b, rest) = rest.split_first()?;
            Some((CanonValue::Bool(b != 0), rest))
        }
        0x02 => {
            let (bytes, rest) = split_n(rest, 8)?;
            Some((
                CanonValue::Int(i64::from_be_bytes(bytes.try_into().ok()?)),
                rest,
            ))
        }
        0x03 => {
            let (m, rest) = split_n(rest, 8)?;
            let (e, rest) = split_n(rest, 4)?;
            Some((
                CanonValue::Decimal {
                    mantissa: i64::from_be_bytes(m.try_into().ok()?),
                    exponent: i32::from_be_bytes(e.try_into().ok()?),
                },
                rest,
            ))
        }
        0x04 => {
            let (len, rest) = read_len(rest)?;
            let (s, rest) = split_n(rest, len)?;
            Some((CanonValue::Text(String::from_utf8(s.to_vec()).ok()?), rest))
        }
        0x05 => {
            let (len, rest) = read_len(rest)?;
            let (b, rest) = split_n(rest, len)?;
            Some((CanonValue::Bytes(b.to_vec()), rest))
        }
        0x06 => {
            let (len, mut rest) = read_len(rest)?;
            let mut items = Vec::with_capacity(len.min(1024));
            for _ in 0..len {
                let (v, r) = decode_at(rest)?;
                items.push(v);
                rest = r;
            }
            Some((CanonValue::List(items), rest))
        }
        0x07 => {
            let (len, mut rest) = read_len(rest)?;
            let mut map = std::collections::BTreeMap::new();
            for _ in 0..len {
                let (klen, r) = read_len(rest)?;
                let (k, r) = split_n(r, klen)?;
                let key = String::from_utf8(k.to_vec()).ok()?;
                let (v, r) = decode_at(r)?;
                map.insert(key, v);
                rest = r;
            }
            Some((CanonValue::Map(map), rest))
        }
        _ => None,
    }
}

fn split_n(data: &[u8], n: usize) -> Option<(&[u8], &[u8])> {
    if data.len() < n {
        None
    } else {
        Some(data.split_at(n))
    }
}

fn read_len(data: &[u8]) -> Option<(usize, &[u8])> {
    let (bytes, rest) = split_n(data, 8)?;
    Some((u64::from_be_bytes(bytes.try_into().ok()?) as usize, rest))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// R-6-Schliessung: MEF-1 round-trip byte-deterministisch.
    #[test]
    fn mef_roundtrip() {
        let block = MefBlock::new(
            "gate_report",
            CanonValue::map([
                ("gate", CanonValue::text("G5-Replay")),
                ("verdict", CanonValue::text("pass")),
            ]),
        );
        let bytes = block.to_bytes();
        let back = MefBlock::from_bytes(&bytes).expect("roundtrip");
        assert_eq!(back, block);
        assert_eq!(back.to_bytes(), bytes, "gleiche Semantik ⇒ gleiche Bytes");
    }

    /// Manipulierte Payload wird VOR Deserialisierung erkannt.
    #[test]
    fn mef_detects_tamper() {
        let block = MefBlock::new("evidence", CanonValue::Int(7));
        let mut bytes = block.to_bytes();
        let n = bytes.len();
        bytes[n - 40] ^= 0xff; // Payload-Byte kippen
        assert!(matches!(
            MefBlock::from_bytes(&bytes),
            Err(MefError::DigestMismatch) | Err(MefError::BadPayload)
        ));
    }
}
