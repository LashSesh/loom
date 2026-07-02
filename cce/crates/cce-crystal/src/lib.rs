//! cce-crystal — der gemeinsame Kristalltyp (Bauverfassung Teil 3.4):
//! Crystal als closure-zertifizierte KLASSE [Can(c)]_σ (nie Repraesentant),
//! MatrixCrystal (Collect-Karte), Monolith (append-only Commit-Ereignis),
//! Quotient q / equivalent (das ≃) — Zwei-Digest-Grundlage
//! (Inhaltsklassen-Digest, S7.2).

pub mod crystal;
pub mod matrix_crystal;
pub mod monolith;
pub mod quotient;

pub use crystal::Crystal;
pub use matrix_crystal::MatrixCrystal;
pub use monolith::Monolith;
pub use quotient::{equivalent, quotient_class};
