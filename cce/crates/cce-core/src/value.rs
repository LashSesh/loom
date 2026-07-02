//! Kanonisches Wertemodell. Signaturrelevante Strukturen kennen KEINEN Float
//! (01_MASTER_BUILD G1-Verbot; LOOM-CANON-1 K3 sinngemaess): Zahlen sind
//! Integer oder Dezimalbruch mit Integer-Mantisse.

use std::collections::BTreeMap;

/// Deterministischer, floatfreier Wertebaum. Map-Schluessel sind Text und
/// durch `BTreeMap` bytewise sortiert — gleiche Semantik ⇒ gleiche Bytes.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CanonValue {
    Null,
    Bool(bool),
    Int(i64),
    /// Dezimalbruch: mantissa * 10^exponent — Ersatz fuer Gleitkomma (K3).
    Decimal {
        mantissa: i64,
        exponent: i32,
    },
    Text(String),
    Bytes(Vec<u8>),
    List(Vec<CanonValue>),
    Map(BTreeMap<String, CanonValue>),
}

impl CanonValue {
    pub fn map(entries: impl IntoIterator<Item = (&'static str, CanonValue)>) -> Self {
        CanonValue::Map(
            entries
                .into_iter()
                .map(|(k, v)| (k.to_string(), v))
                .collect(),
        )
    }

    pub fn text(s: impl Into<String>) -> Self {
        CanonValue::Text(s.into())
    }

    pub fn get(&self, key: &str) -> Option<&CanonValue> {
        match self {
            CanonValue::Map(m) => m.get(key),
            _ => None,
        }
    }

    /// Deterministische Byte-Kodierung (tag ‖ laenge ‖ payload, laengen big-endian).
    /// Grundlage jeder Signatur (σ = sha256 ∘ encode ∘ Can).
    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::new();
        self.encode_into(&mut out);
        out
    }

    fn encode_into(&self, out: &mut Vec<u8>) {
        match self {
            CanonValue::Null => out.push(0x00),
            CanonValue::Bool(b) => {
                out.push(0x01);
                out.push(u8::from(*b));
            }
            CanonValue::Int(i) => {
                out.push(0x02);
                out.extend_from_slice(&i.to_be_bytes());
            }
            CanonValue::Decimal { mantissa, exponent } => {
                out.push(0x03);
                out.extend_from_slice(&mantissa.to_be_bytes());
                out.extend_from_slice(&exponent.to_be_bytes());
            }
            CanonValue::Text(t) => {
                out.push(0x04);
                out.extend_from_slice(&(t.len() as u64).to_be_bytes());
                out.extend_from_slice(t.as_bytes());
            }
            CanonValue::Bytes(b) => {
                out.push(0x05);
                out.extend_from_slice(&(b.len() as u64).to_be_bytes());
                out.extend_from_slice(b);
            }
            CanonValue::List(items) => {
                out.push(0x06);
                out.extend_from_slice(&(items.len() as u64).to_be_bytes());
                for i in items {
                    i.encode_into(out);
                }
            }
            CanonValue::Map(m) => {
                out.push(0x07);
                out.extend_from_slice(&(m.len() as u64).to_be_bytes());
                for (k, v) in m {
                    out.extend_from_slice(&(k.len() as u64).to_be_bytes());
                    out.extend_from_slice(k.as_bytes());
                    v.encode_into(out);
                }
            }
        }
    }

    /// Normalisierung: Dezimalbruch-Mantissen von Nachnullen befreit
    /// (gleiche Zahl ⇒ gleiche Form), rekursiv. Idempotent per Konstruktion.
    pub fn normalize(&self) -> CanonValue {
        match self {
            CanonValue::Decimal { mantissa, exponent } => {
                let (mut m, mut e) = (*mantissa, *exponent);
                if m == 0 {
                    return CanonValue::Decimal {
                        mantissa: 0,
                        exponent: 0,
                    };
                }
                while m % 10 == 0 {
                    m /= 10;
                    e += 1;
                }
                if e == 0 {
                    CanonValue::Int(m)
                } else {
                    CanonValue::Decimal {
                        mantissa: m,
                        exponent: e,
                    }
                }
            }
            CanonValue::List(items) => {
                CanonValue::List(items.iter().map(Self::normalize).collect())
            }
            CanonValue::Map(m) => {
                CanonValue::Map(m.iter().map(|(k, v)| (k.clone(), v.normalize())).collect())
            }
            other => other.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_is_idempotent() {
        let v = CanonValue::map([
            (
                "x",
                CanonValue::Decimal {
                    mantissa: 1500,
                    exponent: -2,
                },
            ),
            ("y", CanonValue::List(vec![CanonValue::Int(1)])),
        ]);
        let once = v.normalize();
        assert_eq!(once, once.normalize());
    }

    #[test]
    fn encoding_is_deterministic_under_key_order() {
        let mut a = BTreeMap::new();
        a.insert("b".to_string(), CanonValue::Int(2));
        a.insert("a".to_string(), CanonValue::Int(1));
        let mut b = BTreeMap::new();
        b.insert("a".to_string(), CanonValue::Int(1));
        b.insert("b".to_string(), CanonValue::Int(2));
        assert_eq!(CanonValue::Map(a).encode(), CanonValue::Map(b).encode());
    }
}
