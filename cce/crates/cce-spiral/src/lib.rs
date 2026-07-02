//! cce-spiral — L6 Spiral-Kinematik (REBASE §1.1/§4.2 Formeln 3–5, S15.6–7)
//! samt L5 Blue/Red-Skalengeometrie und S15-Strukturtypen (ScaleAdapter,
//! Capsule, Multicube, MultiScaleClosure).
//! `z_{k+1} = W_s(E_{λ,α}(z_k))` — Expansion intrinsisch, Wicklung
//! extrinsisch, Ratchet irreversibel. Drift innen erlaubt, aussen annulliert
//! (F6). DispersionProfile: Profil statt Dogma (R-7: Dyadic default).

pub mod address;
pub mod area_class;
pub mod bluered;
pub mod dispersion;
pub mod expansion;
pub mod gates;
pub mod ratchet;
pub mod residues;
pub mod s15;
pub mod wrap;

pub use address::SpiralAddress;
pub use bluered::{close_blue, close_red, BlueCube, RedCube};
pub use dispersion::DispersionProfile;
pub use expansion::{expand, SpiralState};
pub use ratchet::{Ratchet, RatchetKind, RatchetState};
pub use wrap::{wrap, WrapPolicy};
