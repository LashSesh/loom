//! `Can`-Kanonisierung (F-01): idempotent, deterministisch; `Can∘Can = Can`.
//! Gleichheit ist IMMER Gleichheit kanonischer Klassen, nie Byte-Gleichheit
//! roher Formen (P2).

use crate::signature::{sha256, Digest};
use crate::value::CanonValue;

/// Kanonisierter Traeger `(raw, canon, signature)` (Bauverfassung Teil 3.1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalState {
    pub raw: CanonValue,
    pub canon: CanonValue,
    pub signature: Digest,
    pub provenance: Option<String>,
}

impl CanonicalState {
    pub fn new(raw: CanonValue) -> Self {
        let canon = raw.normalize();
        let signature = sha256(&canon.encode());
        Self {
            raw,
            canon,
            signature,
            provenance: None,
        }
    }
}

/// Kanonische Klasse `[Can(c)]_σ` — identifiziert Bedeutung, nie Bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CanonicalClass(pub Digest);

impl std::fmt::Display for CanonicalClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "class:{}", self.0)
    }
}

/// Der Kanonisierungsvertrag jedes Typs: `canonical_value` MUSS deterministisch
/// sein und unter erneuter Kanonisierung stabil (INV-1, Property-Test je Typ).
pub trait Canonicalize {
    /// Kanonische Darstellung als floatfreier Wertebaum.
    fn canonical_value(&self) -> CanonValue;

    /// `Can(x)` als normalisierter Wert.
    fn canon(&self) -> CanonValue {
        self.canonical_value().normalize()
    }

    /// Kanonische Klasse (Inhaltsklassen-Digest).
    fn canonical_class(&self) -> CanonicalClass {
        CanonicalClass(sha256(&self.canon().encode()))
    }

    /// ≃ — Gleichheit kanonischer Klassen (das eine Gleichheitsmass, P2).
    fn equivalent(&self, other: &Self) -> bool
    where
        Self: Sized,
    {
        self.canonical_class() == other.canonical_class()
    }
}

impl Canonicalize for CanonValue {
    fn canonical_value(&self) -> CanonValue {
        self.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// INV-1 / VC1: Can∘Can = Can, ueber einer Wertefamilie.
    #[test]
    fn can_idempotent() {
        let samples = [
            CanonValue::Null,
            CanonValue::Int(-7),
            CanonValue::Decimal {
                mantissa: 12300,
                exponent: -4,
            },
            CanonValue::text("naht"),
            CanonValue::List(vec![
                CanonValue::Decimal {
                    mantissa: 10,
                    exponent: 0,
                },
                CanonValue::Bool(true),
            ]),
            CanonValue::map([(
                "fiber",
                CanonValue::map([("residual", CanonValue::List(vec![]))]),
            )]),
        ];
        for s in samples {
            let c1 = s.canon();
            let c2 = c1.canon();
            assert_eq!(c1, c2, "Can∘Can ≠ Can fuer {s:?}");
        }
    }

    /// P2: Klassen-Gleichheit trotz verschiedener Rohformen.
    #[test]
    fn class_equality_is_semantic_not_byte() {
        let a = CanonValue::Decimal {
            mantissa: 2500,
            exponent: -2,
        };
        let b = CanonValue::Decimal {
            mantissa: 25,
            exponent: 0,
        };
        assert_ne!(a, b);
        assert!(a.equivalent(&b));
    }
}
