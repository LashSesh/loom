//! Organ-Kalkuel (G-23, MERKABA §100): O = (X_O, Y_O, α_O, β_O, σ_O, γ_O).
//! Organische Komposition `O_j ∘_Can O_i = α_j ∘ Can_ij ∘ α_i` — Module
//! koppeln NUR ueber kanonisierte Werte.

use cce_core::canonical::Canonicalize;
use cce_core::value::CanonValue;

/// Ein Organ: benannter, deterministischer Transformator ueber dem
/// kanonischen Wertemodell.
pub struct Organ {
    pub id: &'static str,
    pub alpha: fn(&CanonValue) -> CanonValue,
}

impl Organ {
    pub fn apply(&self, x: &CanonValue) -> CanonValue {
        // Kopplungsregel: Eingang wird IMMER erst kanonisiert (∘_Can).
        (self.alpha)(&x.canon())
    }
}

/// Organische Komposition: kanonisierende Verkettung.
pub fn compose(organs: &[Organ], x: &CanonValue) -> CanonValue {
    organs.iter().fold(x.canon(), |acc, o| o.apply(&acc))
}
