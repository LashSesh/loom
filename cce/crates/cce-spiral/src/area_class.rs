//! AreaClass (Formel 4): `[E^k(X)]_{µ_s} = [X]_{µ_s}` — Flaechengleichheit
//! ist Support-/Budgetklassen-Invarianz UNTER WICKLUNG, keine naive
//! Geometrie.

use crate::expansion::SpiralState;
use cce_core::signature::{sha256, Digest};
use cce_core::value::CanonValue;

/// Die Flaechenklasse eines Zustands: Digest ueber (Boundary-Support ∩
/// Zustand, Budgetklasse) — µ_s-Klassentraeger.
pub fn area_class(z: &SpiralState, boundary: &std::collections::BTreeSet<String>) -> Digest {
    let bounded: Vec<CanonValue> = z
        .support
        .iter()
        .filter(|s| boundary.contains(*s))
        .map(CanonValue::text)
        .collect();
    let v = CanonValue::map([
        ("support", CanonValue::List(bounded)),
        ("budget_class", CanonValue::Int(i64::from(z.budget_class))),
    ]);
    sha256(&v.normalize().encode())
}
