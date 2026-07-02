//! cce-phc — GENERATIV: Projektiver Hypercube-Codec (G-09..G-20).
//! PHC = (M, T, A, C, W, P, G, R, L, E): der Transportkoerper des Kristalls.
//! PHC-CANON-0.1, Zelladressen phc://…, Projektionskalkuel (No-Horizon-
//! Leakage), Gate-Matrix-Bindung G1–G7, Loader V0–V9, Profile
//! CORE/LOOM/MERKABA.

pub mod loader;
pub mod package;
pub mod profiles;
pub mod projection_calc;

pub use loader::{load_phc, LoaderError, ValidationPhase};
pub use package::{Cell, PhcManifest, PhcPackage, Projection, Workcell};
pub use profiles::Profile;
pub use projection_calc::project;
