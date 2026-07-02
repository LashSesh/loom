//! Expansionsoperator E_{λ,α} (Formel 3): intrinsische Ausdehnung eines
//! Spiralzustands — Drift INNEN erlaubt (F6). Der Zustand traegt Support,
//! Budgetklasse und Inhaltsklasse.

use crate::address::SpiralAddress;
use cce_core::canonical::Canonicalize;
use cce_core::value::CanonValue;
use std::collections::BTreeSet;

/// Spiralzustand z.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpiralState {
    pub address: SpiralAddress,
    /// Support: die belegten Traeger-Kennungen.
    pub support: BTreeSet<String>,
    /// Budgetklasse (diskret).
    pub budget_class: u32,
    /// Inhaltskern (Klassentraeger).
    pub content: CanonValue,
    /// Innen-Drift-Puffer: parallele Komponente d∥ (zulaessig).
    pub inner_drift: Vec<String>,
}

impl SpiralState {
    pub fn new(address: SpiralAddress, support: &[&str], content: CanonValue) -> Self {
        Self {
            address,
            support: support.iter().map(|s| s.to_string()).collect(),
            budget_class: 1,
            content,
            inner_drift: Vec::new(),
        }
    }

    pub fn content_class(&self) -> cce_core::canonical::CanonicalClass {
        self.content.canonical_class()
    }
}

/// E_{λ,α}: expandiert den Zustand intrinsisch — k steigt, θ rotiert frei,
/// Support darf WACHSEN (auch ueber den Boundary hinaus: das faengt erst
/// die Wicklung W_s ein), Inhaltsklasse bleibt.
pub fn expand(z: &SpiralState, growth: &[&str], delta_theta: u32) -> SpiralState {
    let mut out = z.clone();
    out.address.k += 1;
    out.address = out.address.rotate(delta_theta);
    for g in growth {
        out.support.insert(g.to_string());
        out.inner_drift.push(format!("d∥:{g}"));
    }
    out
}
