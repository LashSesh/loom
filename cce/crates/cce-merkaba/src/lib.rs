//! cce-merkaba — MONOLITH: Gesamtumlauf (G-21..G-24).
//! Gesamtgleichung F = Commit ∘ C ∘ G ∘ Δ ∘ I ∘ Q ∘ Θ ∘ P ∘ Can als
//! deterministische Organ-Komposition; Emission ⟺ E∈L ∧ Replay ∧
//! Pass(G_E) ∧ Res ≤ ε. Jedes Crate ist ein Organ O=(X,Y,α,β,σ,γ);
//! Kopplung nur ueber kanonisierte Typen (∘_Can).

pub mod circulate;
pub mod organ;

pub use circulate::{circulate, Emission};
pub use organ::Organ;
