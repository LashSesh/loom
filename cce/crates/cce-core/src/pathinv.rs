//! Pfadinvarianz (I-07/INV-9): divergierende Reduktionsreihenfolgen
//! konvergieren in der kanonischen Klasse `[x] ∈ X/≡σ`.

use crate::canonical::{CanonicalClass, Canonicalize};

/// Prueft Konfluenz mod ≡σ: zwei Ableitungspfade (als Funktionsfolgen)
/// auf demselben Eingang muessen in derselben kanonischen Klasse enden.
pub fn confluent_mod_sigma<T, U>(
    input: &T,
    path_a: impl Fn(&T) -> U,
    path_b: impl Fn(&T) -> U,
) -> bool
where
    U: Canonicalize,
{
    path_a(input).canonical_class() == path_b(input).canonical_class()
}

/// PathReport fuer Zertifikate (VC8).
pub fn path_report(class_a: CanonicalClass, class_b: CanonicalClass) -> String {
    if class_a == class_b {
        format!("pfadinvariant: beide Ordnungen ⇒ {class_a}")
    } else {
        format!("PFADDIVERGENZ: {class_a} ≠ {class_b}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::CanonValue;
    use std::collections::BTreeMap;

    /// INV-9: Einfuege-Reihenfolge in eine Map ist klassenneutral.
    #[test]
    fn insertion_order_is_confluent() {
        let items = vec![("a", 1i64), ("b", 2), ("c", 3)];
        let build = |order: Vec<usize>| {
            move |xs: &Vec<(&str, i64)>| {
                let mut m = BTreeMap::new();
                for &i in &order {
                    let (k, v) = xs[i];
                    m.insert(k.to_string(), CanonValue::Int(v));
                }
                CanonValue::Map(m)
            }
        };
        assert!(confluent_mod_sigma(
            &items,
            build(vec![0, 1, 2]),
            build(vec![2, 0, 1])
        ));
    }
}
