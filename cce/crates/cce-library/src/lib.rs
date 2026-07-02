//! cce-library — die Bibliothek als LEBENDER BEWEIS (S8 + S8-A):
//! Referenz-Cubes (bleiben gruen) und Negativ-Cubes (bleiben rot),
//! Eintritts-Selbstvalidierung (S8.2), Registry mit Provenienz,
//! und der REGRESSIONSWAECHTER (S8.3): EIN Waechter, der Domaenen,
//! Updates, CoreExtensions, CSA- und .loom-Zeugen schuetzt.

pub mod guard;
pub mod registry;

pub use guard::{run_guard, GuardOutcome};
pub use registry::{AssetKind, LibraryAsset, Registry, RegistryError};
