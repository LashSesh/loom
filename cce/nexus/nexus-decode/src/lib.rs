//! nexus-decode — deklarierte Decoder (CSA.4): Bytes → Struktur.
//! Nur registrierte, deterministische Decoder; ein unparsbarer Input
//! ist `schema_unparseable` (sichtbar), nie ein stilles Ueberspringen.

use cce_core::residue::Residue;
use cce_core::value::CanonValue;
use nexus_core::objects::RawObservation;
use nexus_core::residues::csa_residue;

/// Zeilenbasierter Key-Value-Decoder ("k: v" pro Zeile) — der
/// deterministische Referenz-Decoder fuer Fixtures und lokale Korpora.
pub fn decode_kv_lines(raw: &RawObservation) -> Result<CanonValue, Box<Residue>> {
    let text = match core::str::from_utf8(&raw.bytes) {
        Ok(t) => t,
        Err(_) => {
            return Err(Box::new(csa_residue(
                "schema_unparseable",
                &format!("{}: kein UTF-8", raw.locator),
            )))
        }
    };
    let mut map = std::collections::BTreeMap::new();
    for (i, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        match line.split_once(':') {
            Some((k, v)) => {
                let key = k.trim();
                // Deklariertes Schema: Schluessel sind einfache Bezeichner.
                // Markup (z. B. HTML) faellt hier fail-closed heraus —
                // kein stiller Scope-Leak ueber "zufaellig parsbare" Zeilen.
                if key.is_empty()
                    || !key
                        .chars()
                        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
                {
                    return Err(Box::new(csa_residue(
                        "schema_unparseable",
                        &format!(
                            "{}: Zeile {} mit unzulaessigem Schluessel",
                            raw.locator,
                            i + 1
                        ),
                    )));
                }
                map.insert(key.to_string(), CanonValue::Text(v.trim().to_string()));
            }
            None => {
                return Err(Box::new(csa_residue(
                    "schema_unparseable",
                    &format!("{}: Zeile {} ohne ':'-Trenner", raw.locator, i + 1),
                )))
            }
        }
    }
    if map.is_empty() {
        return Err(Box::new(csa_residue(
            "schema_unparseable",
            &format!("{}: leerer Datensatz", raw.locator),
        )));
    }
    Ok(CanonValue::Map(map))
}

/// JSON-Decoder (offizielle APIs, CSA.4): Bytes → CanonValue. Kein
/// externes Parser-Crate (Bauplan: fremde Kisten bleiben im CLI-
/// Blattcrate, s. loom-cli/sign.rs) — ein kleiner, selbst gefuehrter
/// rekursiver Abstieg reicht fuer das deklarierte Zielschema. Deckt
/// Objekte/Arrays/Strings (inkl. `\uXXXX`- und Surrogatpaar-Escapes)/
/// Ganzzahlen/einfache Dezimalbrueche/Bool/Null. Exponentialschreibweise
/// und alles darueber hinaus ist `schema_unparseable` — fail-closed
/// statt eine Zahl still falsch zu interpretieren.
pub fn decode_json(raw: &RawObservation) -> Result<CanonValue, Box<Residue>> {
    let text = match core::str::from_utf8(&raw.bytes) {
        Ok(t) => t,
        Err(_) => {
            return Err(Box::new(csa_residue(
                "schema_unparseable",
                &format!("{}: kein UTF-8", raw.locator),
            )))
        }
    };
    let mut p = JsonParser {
        s: text.as_bytes(),
        pos: 0,
    };
    let value = p.parse_value().map_err(|e| {
        Box::new(csa_residue(
            "schema_unparseable",
            &format!("{}: {e}", raw.locator),
        ))
    })?;
    p.skip_ws();
    if p.pos != p.s.len() {
        return Err(Box::new(csa_residue(
            "schema_unparseable",
            &format!(
                "{}: ueberschuessige Bytes nach dem JSON-Wert (Position {})",
                raw.locator, p.pos
            ),
        )));
    }
    Ok(value)
}

struct JsonParser<'a> {
    s: &'a [u8],
    pos: usize,
}

impl<'a> JsonParser<'a> {
    fn skip_ws(&mut self) {
        while matches!(self.peek(), Some(b' ' | b'\t' | b'\n' | b'\r')) {
            self.pos += 1;
        }
    }

    fn peek(&self) -> Option<u8> {
        self.s.get(self.pos).copied()
    }

    fn expect(&mut self, b: u8) -> Result<(), String> {
        if self.peek() == Some(b) {
            self.pos += 1;
            Ok(())
        } else {
            Err(format!("erwartet '{}' an Position {}", b as char, self.pos))
        }
    }

    fn expect_lit(&mut self, lit: &str) -> Result<(), String> {
        let end = self.pos + lit.len();
        if end <= self.s.len() && &self.s[self.pos..end] == lit.as_bytes() {
            self.pos = end;
            Ok(())
        } else {
            Err(format!("erwartet '{lit}' an Position {}", self.pos))
        }
    }

    fn parse_value(&mut self) -> Result<CanonValue, String> {
        self.skip_ws();
        match self.peek() {
            Some(b'{') => self.parse_object(),
            Some(b'[') => self.parse_array(),
            Some(b'"') => Ok(CanonValue::Text(self.parse_string()?)),
            Some(b't') => {
                self.expect_lit("true")?;
                Ok(CanonValue::Bool(true))
            }
            Some(b'f') => {
                self.expect_lit("false")?;
                Ok(CanonValue::Bool(false))
            }
            Some(b'n') => {
                self.expect_lit("null")?;
                Ok(CanonValue::Null)
            }
            Some(c) if c == b'-' || c.is_ascii_digit() => self.parse_number(),
            Some(c) => Err(format!(
                "unerwartetes Zeichen '{}' an Position {}",
                c as char, self.pos
            )),
            None => Err("unerwartetes Ende der Eingabe".to_string()),
        }
    }

    fn parse_object(&mut self) -> Result<CanonValue, String> {
        self.expect(b'{')?;
        let mut map = std::collections::BTreeMap::new();
        self.skip_ws();
        if self.peek() == Some(b'}') {
            self.pos += 1;
            return Ok(CanonValue::Map(map));
        }
        loop {
            self.skip_ws();
            if self.peek() != Some(b'"') {
                return Err(format!(
                    "Objektschluessel erwartet an Position {}",
                    self.pos
                ));
            }
            let key = self.parse_string()?;
            self.skip_ws();
            self.expect(b':')?;
            let value = self.parse_value()?;
            map.insert(key, value);
            self.skip_ws();
            match self.peek() {
                Some(b',') => self.pos += 1,
                Some(b'}') => {
                    self.pos += 1;
                    break;
                }
                _ => return Err(format!("',' oder '}}' erwartet an Position {}", self.pos)),
            }
        }
        Ok(CanonValue::Map(map))
    }

    fn parse_array(&mut self) -> Result<CanonValue, String> {
        self.expect(b'[')?;
        let mut items = Vec::new();
        self.skip_ws();
        if self.peek() == Some(b']') {
            self.pos += 1;
            return Ok(CanonValue::List(items));
        }
        loop {
            items.push(self.parse_value()?);
            self.skip_ws();
            match self.peek() {
                Some(b',') => self.pos += 1,
                Some(b']') => {
                    self.pos += 1;
                    break;
                }
                _ => return Err(format!("',' oder ']' erwartet an Position {}", self.pos)),
            }
        }
        Ok(CanonValue::List(items))
    }

    fn parse_string(&mut self) -> Result<String, String> {
        self.expect(b'"')?;
        let mut out = String::new();
        loop {
            match self.peek() {
                None => return Err("unbeendeter String".to_string()),
                Some(b'"') => {
                    self.pos += 1;
                    break;
                }
                Some(b'\\') => {
                    self.pos += 1;
                    match self.peek() {
                        Some(b'"') => {
                            out.push('"');
                            self.pos += 1;
                        }
                        Some(b'\\') => {
                            out.push('\\');
                            self.pos += 1;
                        }
                        Some(b'/') => {
                            out.push('/');
                            self.pos += 1;
                        }
                        Some(b'b') => {
                            out.push('\u{8}');
                            self.pos += 1;
                        }
                        Some(b'f') => {
                            out.push('\u{c}');
                            self.pos += 1;
                        }
                        Some(b'n') => {
                            out.push('\n');
                            self.pos += 1;
                        }
                        Some(b'r') => {
                            out.push('\r');
                            self.pos += 1;
                        }
                        Some(b't') => {
                            out.push('\t');
                            self.pos += 1;
                        }
                        Some(b'u') => {
                            self.pos += 1;
                            let cp = self.parse_hex4()?;
                            if (0xD800..=0xDBFF).contains(&cp) {
                                if self.peek() == Some(b'\\')
                                    && self.s.get(self.pos + 1) == Some(&b'u')
                                {
                                    self.pos += 2;
                                    let low = self.parse_hex4()?;
                                    if !(0xDC00..=0xDFFF).contains(&low) {
                                        return Err(
                                            "ungueltiges Surrogatpaar (low ausserhalb DC00-DFFF)"
                                                .to_string(),
                                        );
                                    }
                                    let c = 0x10000 + ((cp - 0xD800) << 10) + (low - 0xDC00);
                                    out.push(
                                        char::from_u32(c).ok_or(
                                            "ungueltiger Unicode-Codepoint aus Surrogatpaar",
                                        )?,
                                    );
                                } else {
                                    return Err(
                                        "high-surrogate \\u-Escape ohne folgendes low-surrogate"
                                            .to_string(),
                                    );
                                }
                            } else {
                                out.push(
                                    char::from_u32(cp).ok_or("ungueltiger Unicode-Codepoint")?,
                                );
                            }
                        }
                        _ => return Err(format!("ungueltiges Escape an Position {}", self.pos)),
                    }
                }
                Some(_) => {
                    let rest = core::str::from_utf8(&self.s[self.pos..])
                        .map_err(|_| "ungueltiges UTF-8 im String".to_string())?;
                    let ch = rest.chars().next().ok_or("unerwartetes Stringende")?;
                    out.push(ch);
                    self.pos += ch.len_utf8();
                }
            }
        }
        Ok(out)
    }

    fn parse_hex4(&mut self) -> Result<u32, String> {
        if self.pos + 4 > self.s.len() {
            return Err("unvollstaendiges \\u-Escape".to_string());
        }
        let hex = core::str::from_utf8(&self.s[self.pos..self.pos + 4])
            .map_err(|_| "ungueltiges \\u-Escape".to_string())?;
        let v = u32::from_str_radix(hex, 16).map_err(|_| "ungueltiges \\u-Escape".to_string())?;
        self.pos += 4;
        Ok(v)
    }

    fn parse_number(&mut self) -> Result<CanonValue, String> {
        let start = self.pos;
        if self.peek() == Some(b'-') {
            self.pos += 1;
        }
        while matches!(self.peek(), Some(c) if c.is_ascii_digit()) {
            self.pos += 1;
        }
        let mut is_decimal = false;
        if self.peek() == Some(b'.') {
            is_decimal = true;
            self.pos += 1;
            while matches!(self.peek(), Some(c) if c.is_ascii_digit()) {
                self.pos += 1;
            }
        }
        if matches!(self.peek(), Some(b'e' | b'E')) {
            return Err(
                "Exponentialschreibweise nicht unterstuetzt (schema_unparseable statt stiller Rundung)"
                    .to_string(),
            );
        }
        let tok = core::str::from_utf8(&self.s[start..self.pos]).expect("nur ASCII-Ziffern/-.");
        if is_decimal {
            let (int_part, frac_part) = tok.split_once('.').expect("is_decimal ⇒ '.' vorhanden");
            let exponent = -(frac_part.len() as i32);
            let mantissa: i64 = format!("{int_part}{frac_part}")
                .parse()
                .map_err(|_| format!("ungueltige Zahl '{tok}'"))?;
            Ok(CanonValue::Decimal { mantissa, exponent })
        } else {
            let n: i64 = tok
                .parse()
                .map_err(|_| format!("ungueltige Zahl '{tok}'"))?;
            Ok(CanonValue::Int(n))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn obs(bytes: &[u8]) -> RawObservation {
        RawObservation {
            locator: "corpus://t".to_string(),
            bytes: bytes.to_vec(),
            fetched_via: "fixture".to_string(),
            snapshot_id: "snap-1".to_string(),
        }
    }

    #[test]
    fn kv_decodes_and_fails_closed() {
        assert!(decode_kv_lines(&obs(b"titel: A\nkorpus: B")).is_ok());
        let err = decode_kv_lines(&obs(b"keine trennung")).unwrap_err();
        assert!(err.id.contains("schema_unparseable"));
        assert!(decode_kv_lines(&obs(&[0xff, 0xfe])).is_err());
    }

    #[test]
    fn json_decodes_nested_objects_arrays_and_scalars() {
        let v = decode_json(&obs(
            br#"{"a": 1, "b": [true, false, null, -3], "c": {"d": "x"}, "e": 1.5}"#,
        ))
        .unwrap();
        assert_eq!(v.get("a"), Some(&CanonValue::Int(1)));
        assert_eq!(
            v.get("b"),
            Some(&CanonValue::List(vec![
                CanonValue::Bool(true),
                CanonValue::Bool(false),
                CanonValue::Null,
                CanonValue::Int(-3),
            ]))
        );
        assert_eq!(
            v.get("c").and_then(|c| c.get("d")),
            Some(&CanonValue::Text("x".to_string()))
        );
        assert_eq!(
            v.get("e"),
            Some(&CanonValue::Decimal {
                mantissa: 15,
                exponent: -1
            })
        );
    }

    #[test]
    fn json_decodes_unicode_escapes_matching_raw_utf8() {
        // JSON-Escape-Form (6 ASCII-Zeichen: Backslash u 0 0 f 6) muss
        // dasselbe Ergebnis liefern wie das rohe UTF-8 'ö' — genau die
        // Escape-Form, die echte MediaWiki-Antworten nutzen (s. Fixture).
        let escaped = decode_json(&obs("{\"t\": \"K\\u00f6rper\"}".as_bytes())).unwrap();
        let raw = decode_json(&obs("{\"t\": \"Körper\"}".as_bytes())).unwrap();
        assert_eq!(escaped, raw);
        assert_eq!(
            escaped.get("t"),
            Some(&CanonValue::Text("Körper".to_string()))
        );
    }

    #[test]
    fn json_decodes_surrogate_pair() {
        // U+1F600 als UTF-16-Surrogatpaar (Escape-Form) muss dasselbe
        // Ergebnis liefern wie das rohe 4-Byte-UTF-8-Emoji.
        let escaped = decode_json(&obs("{\"t\": \"\\uD83D\\uDE00\"}".as_bytes())).unwrap();
        let raw = decode_json(&obs("{\"t\": \"😀\"}".as_bytes())).unwrap();
        assert_eq!(escaped, raw);
        assert_eq!(escaped.get("t"), Some(&CanonValue::Text("😀".to_string())));
    }

    #[test]
    fn json_fails_closed_on_malformed_or_trailing_bytes() {
        assert!(decode_json(&obs(b"{not json")).is_err());
        assert!(decode_json(&obs(b"{\"a\":1} trailing")).is_err());
        assert!(
            decode_json(&obs(b"{\"a\": 1e10}")).is_err(),
            "Exponentialschreibweise fail-closed"
        );
        assert!(decode_json(&obs(&[0xff, 0xfe])).is_err());
    }

    /// Die tatsaechliche, eingefrorene MediaWiki-Antwort
    /// (conformance/fixtures/wikimedia_kristall.json) muss vollstaendig
    /// und ohne Informationsverlust dekodieren — das ist der reale
    /// Zielfall dieses Decoders, kein Kunstbeispiel.
    #[test]
    fn json_decodes_the_real_frozen_wikimedia_fixture() {
        let fixture = include_bytes!("../../../conformance/fixtures/wikimedia_kristall.json");
        let v = decode_json(&obs(fixture)).expect("echte Wikimedia-JSON-Antwort muss dekodieren");
        let page = v
            .get("query")
            .and_then(|q| q.get("pages"))
            .and_then(|pages| match pages {
                CanonValue::Map(m) => m.values().next(),
                _ => None,
            })
            .expect("query.pages.<id> muss vorhanden sein");
        assert_eq!(
            page.get("title"),
            Some(&CanonValue::Text("Kristall".to_string()))
        );
        assert_eq!(page.get("pageid"), Some(&CanonValue::Int(734532)));
        let extract = match page.get("extract") {
            Some(CanonValue::Text(t)) => t.clone(),
            other => panic!("extract muss Text sein: {other:?}"),
        };
        assert!(extract.contains("Kristallographie"), "{extract}");
        assert!(extract.contains("Festkörper"), "{extract}");
    }
}
