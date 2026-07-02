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
}
