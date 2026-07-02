//! loom-canon — LOOM-CANON-1 (LOOM-Standard Teil 4.1):
//! dCBOR nach RFC 8949 §4.2.1, verschaerft:
//!   - definite lengths only
//!   - kuerzeste Integer-Kodierung
//!   - Map-Keys nur tstr/uint, bytewise aufsteigend sortiert, keine Duplikate
//!   - KEINE Floats — Zahlen als Integer oder Decimal-Fraction (Tag 4,
//!     Integer-Mantisse) [K3]
//!   - Text NFC-normalisiert (Teilmengen-Pruefung, s. `nfc_check`)
//!   - Tag-Whitelist {0 (Zeit, nur Evidence-Felder), 4}
//!
//! Ergebnis: gleiche Semantik ⇒ gleiche kanonische Bytes [K5].

/// Das kanonische Wertemodell. Es gibt KEINEN Float-Konstruktor —
/// der Verstoss ist damit strukturell unmoeglich [K3].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cv {
    Uint(u64),
    /// negative Zahl: gespeichert als n, bedeutet -(n+1) (CBOR Major 1).
    Nint(u64),
    Bytes(Vec<u8>),
    Text(String),
    Array(Vec<Cv>),
    /// Map: Schluessel nur Text oder Uint; wird beim Encode sortiert.
    Map(Vec<(Cv, Cv)>),
    /// Tag: Whitelist {0, 4}.
    Tag(u64, Box<Cv>),
    Bool(bool),
    Null,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CanonError {
    FloatForbidden,
    IndefiniteLength,
    NonShortestInt { at: usize },
    MapKeyNotTextOrUint,
    MapKeysUnsorted { at: usize },
    MapKeyDuplicate { at: usize },
    TagNotWhitelisted(u64),
    TextNotNfc { hint: String },
    Truncated,
    TrailingBytes { extra: usize },
    MalformedHeader { at: usize },
    DepthExceeded,
}

pub const TAG_WHITELIST: [u64; 2] = [0, 4];
const MAX_DEPTH: u32 = 64;

/// NFC-Teilmengenpruefung: verworfen wird jede Sequenz Basiszeichen +
/// kombinierendes Diakritikum (U+0300–U+036F) — die in unserem Korpus
/// einzige real vorkommende Nicht-NFC-Klasse. Volle Unicode-NFC-Tabellen
/// werden nicht eingebettet (sichtbare Schranke, s. G09-Bericht).
pub fn nfc_check(s: &str) -> Result<(), CanonError> {
    for c in s.chars() {
        if ('\u{0300}'..='\u{036f}').contains(&c) {
            return Err(CanonError::TextNotNfc {
                hint: format!(
                    "kombinierendes Zeichen U+{:04X} (dekomponierte Form)",
                    c as u32
                ),
            });
        }
    }
    Ok(())
}

fn head(major: u8, arg: u64, out: &mut Vec<u8>) {
    let mt = major << 5;
    if arg < 24 {
        out.push(mt | arg as u8);
    } else if arg <= 0xff {
        out.push(mt | 24);
        out.push(arg as u8);
    } else if arg <= 0xffff {
        out.push(mt | 25);
        out.extend_from_slice(&(arg as u16).to_be_bytes());
    } else if arg <= 0xffff_ffff {
        out.push(mt | 26);
        out.extend_from_slice(&(arg as u32).to_be_bytes());
    } else {
        out.push(mt | 27);
        out.extend_from_slice(&arg.to_be_bytes());
    }
}

impl Cv {
    /// Decimal-Fraction (Tag 4) mit Integer-Mantisse — die EINZIGE
    /// zugelassene Nicht-Integer-Zahlform [K3].
    pub fn decimal(mantissa: i64, exponent: i64) -> Cv {
        let enc = |v: i64| {
            if v >= 0 {
                Cv::Uint(v as u64)
            } else {
                Cv::Nint((-(v + 1)) as u64)
            }
        };
        Cv::Tag(4, Box::new(Cv::Array(vec![enc(exponent), enc(mantissa)])))
    }

    pub fn map(entries: Vec<(&str, Cv)>) -> Cv {
        Cv::Map(
            entries
                .into_iter()
                .map(|(k, v)| (Cv::Text(k.to_string()), v))
                .collect(),
        )
    }

    /// Kanonische Kodierung. Sortiert Maps bytewise nach kodiertem
    /// Schluessel; prueft Schluesseltyp, Duplikate, Tag-Whitelist, NFC.
    pub fn encode(&self) -> Result<Vec<u8>, CanonError> {
        let mut out = Vec::new();
        self.enc(&mut out, 0)?;
        Ok(out)
    }

    fn enc(&self, out: &mut Vec<u8>, depth: u32) -> Result<(), CanonError> {
        if depth > MAX_DEPTH {
            return Err(CanonError::DepthExceeded);
        }
        match self {
            Cv::Uint(n) => head(0, *n, out),
            Cv::Nint(n) => head(1, *n, out),
            Cv::Bytes(b) => {
                head(2, b.len() as u64, out);
                out.extend_from_slice(b);
            }
            Cv::Text(s) => {
                nfc_check(s)?;
                head(3, s.len() as u64, out);
                out.extend_from_slice(s.as_bytes());
            }
            Cv::Array(items) => {
                head(4, items.len() as u64, out);
                for it in items {
                    it.enc(out, depth + 1)?;
                }
            }
            Cv::Map(entries) => {
                let mut encoded: Vec<(Vec<u8>, Vec<u8>)> = Vec::with_capacity(entries.len());
                for (k, v) in entries {
                    if !matches!(k, Cv::Text(_) | Cv::Uint(_)) {
                        return Err(CanonError::MapKeyNotTextOrUint);
                    }
                    let mut kb = Vec::new();
                    k.enc(&mut kb, depth + 1)?;
                    let mut vb = Vec::new();
                    v.enc(&mut vb, depth + 1)?;
                    encoded.push((kb, vb));
                }
                encoded.sort_by(|a, b| a.0.cmp(&b.0));
                for w in encoded.windows(2) {
                    if w[0].0 == w[1].0 {
                        return Err(CanonError::MapKeyDuplicate { at: 0 });
                    }
                }
                head(5, encoded.len() as u64, out);
                for (kb, vb) in encoded {
                    out.extend_from_slice(&kb);
                    out.extend_from_slice(&vb);
                }
            }
            Cv::Tag(t, inner) => {
                if !TAG_WHITELIST.contains(t) {
                    return Err(CanonError::TagNotWhitelisted(*t));
                }
                head(6, *t, out);
                inner.enc(out, depth + 1)?;
            }
            Cv::Bool(false) => out.push(0xf4),
            Cv::Bool(true) => out.push(0xf5),
            Cv::Null => out.push(0xf6),
        }
        Ok(())
    }
}

struct Reader<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    fn byte(&mut self) -> Result<u8, CanonError> {
        let b = *self.buf.get(self.pos).ok_or(CanonError::Truncated)?;
        self.pos += 1;
        Ok(b)
    }

    fn take(&mut self, n: usize) -> Result<&'a [u8], CanonError> {
        if self.pos + n > self.buf.len() {
            return Err(CanonError::Truncated);
        }
        let s = &self.buf[self.pos..self.pos + n];
        self.pos += n;
        Ok(s)
    }

    /// Liest einen Kopf und prueft die KUERZESTE Kodierung.
    fn head(&mut self) -> Result<(u8, u64), CanonError> {
        let at = self.pos;
        let ib = self.byte()?;
        let major = ib >> 5;
        let ai = ib & 0x1f;
        let arg = match ai {
            0..=23 => u64::from(ai),
            24 => {
                let v = u64::from(self.byte()?);
                if v < 24 {
                    return Err(CanonError::NonShortestInt { at });
                }
                v
            }
            25 => {
                let v = u64::from(u16::from_be_bytes(self.take(2)?.try_into().unwrap()));
                if v <= 0xff {
                    return Err(CanonError::NonShortestInt { at });
                }
                v
            }
            26 => {
                let v = u64::from(u32::from_be_bytes(self.take(4)?.try_into().unwrap()));
                if v <= 0xffff {
                    return Err(CanonError::NonShortestInt { at });
                }
                v
            }
            27 => {
                let v = u64::from_be_bytes(self.take(8)?.try_into().unwrap());
                if v <= 0xffff_ffff {
                    return Err(CanonError::NonShortestInt { at });
                }
                v
            }
            31 => return Err(CanonError::IndefiniteLength),
            _ => return Err(CanonError::MalformedHeader { at }),
        };
        Ok((major, arg))
    }
}

/// Strikte kanonische Dekodierung: prueft Kuerzeste-Form, Sortierung,
/// Duplikate, Float-Verbot, Tag-Whitelist, definite lengths, NFC —
/// und dass KEINE Bytes uebrig bleiben.
pub fn decode(buf: &[u8]) -> Result<Cv, CanonError> {
    let mut r = Reader { buf, pos: 0 };
    let v = decode_item(&mut r, 0)?;
    if r.pos != buf.len() {
        return Err(CanonError::TrailingBytes {
            extra: buf.len() - r.pos,
        });
    }
    Ok(v)
}

fn decode_item(r: &mut Reader, depth: u32) -> Result<Cv, CanonError> {
    if depth > MAX_DEPTH {
        return Err(CanonError::DepthExceeded);
    }
    let at = r.pos;
    // Major 7 mit ai=25/26/27 ist Float16/32/64 — VOR dem generischen
    // Head-Parser abfangen (dessen Argument-Semantik gilt nur fuer Ints).
    if let Some(&ib) = r.buf.get(r.pos) {
        if ib >> 5 == 7 && matches!(ib & 0x1f, 25..=27) {
            return Err(CanonError::FloatForbidden);
        }
    }
    let (major, arg) = r.head()?;
    match major {
        0 => Ok(Cv::Uint(arg)),
        1 => Ok(Cv::Nint(arg)),
        2 => Ok(Cv::Bytes(r.take(arg as usize)?.to_vec())),
        3 => {
            let bytes = r.take(arg as usize)?;
            let s = core::str::from_utf8(bytes)
                .map_err(|_| CanonError::MalformedHeader { at })?
                .to_string();
            nfc_check(&s)?;
            Ok(Cv::Text(s))
        }
        4 => {
            let mut items = Vec::new();
            for _ in 0..arg {
                items.push(decode_item(r, depth + 1)?);
            }
            Ok(Cv::Array(items))
        }
        5 => {
            let mut entries = Vec::new();
            let mut prev_key: Option<Vec<u8>> = None;
            for _ in 0..arg {
                let kstart = r.pos;
                let k = decode_item(r, depth + 1)?;
                if !matches!(k, Cv::Text(_) | Cv::Uint(_)) {
                    return Err(CanonError::MapKeyNotTextOrUint);
                }
                let kbytes = r.buf[kstart..r.pos].to_vec();
                if let Some(p) = &prev_key {
                    if *p == kbytes {
                        return Err(CanonError::MapKeyDuplicate { at: kstart });
                    }
                    if *p > kbytes {
                        return Err(CanonError::MapKeysUnsorted { at: kstart });
                    }
                }
                prev_key = Some(kbytes);
                let v = decode_item(r, depth + 1)?;
                entries.push((k, v));
            }
            Ok(Cv::Map(entries))
        }
        6 => {
            if !TAG_WHITELIST.contains(&arg) {
                return Err(CanonError::TagNotWhitelisted(arg));
            }
            Ok(Cv::Tag(arg, Box::new(decode_item(r, depth + 1)?)))
        }
        7 => match arg {
            20 => Ok(Cv::Bool(false)),
            21 => Ok(Cv::Bool(true)),
            22 => Ok(Cv::Null),
            // 25/26/27 = Float16/32/64 — verboten [K3].
            25..=27 => Err(CanonError::FloatForbidden),
            _ => Err(CanonError::MalformedHeader { at }),
        },
        _ => unreachable!(),
    }
}

/// Der normative Regeltext fuer das CANON_DESC-Segment (kind 0x0003).
pub const CANON_RULES_TEXT: &str = "LOOM-CANON-1: dCBOR RFC8949-4.2.1; \
definite lengths only; shortest int encoding; map keys tstr/uint bytewise \
ascending, no duplicates; no floats (integer or decimal fraction tag 4 with \
integer mantissa); text NFC; tag whitelist {0 evidence-time, 4}; no wall \
clock in signature-relevant paths; seeds/tie-breaks only in REPLAY_MANIFEST; \
randomness only seeded";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_and_sorting() {
        let v = Cv::map(vec![
            ("zz", Cv::Uint(1)),
            ("aa", Cv::Array(vec![Cv::Bool(true), Cv::Null])),
            ("mm", Cv::decimal(1234, -2)),
        ]);
        let bytes = v.encode().unwrap();
        let back = decode(&bytes).unwrap();
        // Map kommt sortiert zurueck: aa, mm, zz
        if let Cv::Map(entries) = &back {
            let keys: Vec<_> = entries
                .iter()
                .map(|(k, _)| match k {
                    Cv::Text(s) => s.clone(),
                    _ => panic!(),
                })
                .collect();
            assert_eq!(keys, vec!["aa", "mm", "zz"]);
        } else {
            panic!("Map erwartet");
        }
        // Re-Encode ist byte-identisch (Can∘Can=Can auf Formatebene).
        assert_eq!(back.encode().unwrap(), bytes);
    }

    #[test]
    fn same_semantics_same_bytes_k5() {
        let a = Cv::map(vec![("x", Cv::Uint(5)), ("y", Cv::Text("t".into()))]);
        let b = Cv::map(vec![("y", Cv::Text("t".into())), ("x", Cv::Uint(5))]);
        assert_eq!(a.encode().unwrap(), b.encode().unwrap());
    }

    #[test]
    fn violations_fail_closed() {
        // Float 64 (0xfb ...) ⇒ FloatForbidden
        let mut f = vec![0xfb];
        f.extend_from_slice(&1.5f64.to_be_bytes());
        assert_eq!(decode(&f), Err(CanonError::FloatForbidden));
        // Nicht-kuerzeste Int-Kodierung: 24 00 (0 als 1-Byte-Arg)
        assert!(matches!(
            decode(&[0x18, 0x00]),
            Err(CanonError::NonShortestInt { .. })
        ));
        // Indefinite-Length-Array (0x9f)
        assert_eq!(decode(&[0x9f, 0xff]), Err(CanonError::IndefiniteLength));
        // Unsortierte Map: {"b":1,"a":1}
        let unsorted = [0xa2, 0x61, b'b', 0x01, 0x61, b'a', 0x01];
        assert!(matches!(
            decode(&unsorted),
            Err(CanonError::MapKeysUnsorted { .. })
        ));
        // Duplikat-Schluessel
        let dup = [0xa2, 0x61, b'a', 0x01, 0x61, b'a', 0x02];
        assert!(matches!(
            decode(&dup),
            Err(CanonError::MapKeyDuplicate { .. })
        ));
        // Tag ausserhalb Whitelist (Tag 1 = Epoch)
        assert_eq!(
            Cv::Tag(1, Box::new(Cv::Uint(1))).encode(),
            Err(CanonError::TagNotWhitelisted(1))
        );
        // Dekomponiertes a + Combining Acute ⇒ TextNotNfc
        assert!(matches!(
            Cv::Text("a\u{0301}".into()).encode(),
            Err(CanonError::TextNotNfc { .. })
        ));
        // Trailing Bytes
        assert!(matches!(
            decode(&[0x01, 0x02]),
            Err(CanonError::TrailingBytes { .. })
        ));
    }

    #[test]
    fn negative_ints_and_decimal_fraction() {
        // -5 => Nint(4)
        let v = Cv::Nint(4);
        let b = v.encode().unwrap();
        assert_eq!(b, vec![0x24]);
        // decimal 12.34 = Tag4 [ -2, 1234 ]
        let d = Cv::decimal(1234, -2);
        let db = d.encode().unwrap();
        assert_eq!(decode(&db).unwrap(), d);
    }
}
