//! nexus-adapter — der SourceAdapter-Portvertrag (CSA.3): EIN Vertrag,
//! 8 Methoden; jede Quelle braucht die 9 Manifest-Pflichtfelder.
//! Neue Quellenarten aendern den Kern NICHT (`check_source_adapter_parity`).
//! Anmerkung: `acquire` ist synchron implementiert (deterministisch,
//! kein Async-Laufzeitzwang) — sichtbare Abweichung, siehe G08-Bericht.

pub mod manifest;
pub mod port;

pub use manifest::{AdapterManifest, ManifestError};
pub use port::{check_source_adapter_parity, ExtractedRecord, FetchPlan, SourceAdapter};
