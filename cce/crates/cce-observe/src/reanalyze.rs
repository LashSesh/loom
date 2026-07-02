//! `reanalyze` (=== Obs, Teil 4.3): Artefakt → MatrixCrystal.
//! Der Collect-Sweep ueber einem materialisierten Artefakt; der Domaenen-
//! Parser liefert den semantischen Kern (verlustfrei), TAT kristallisiert.

use crate::tat::{collect, CollectError, CollectInput};
use cce_core::objects::Marker;
use cce_core::value::CanonValue;
use cce_crystal::matrix_crystal::MatrixCrystal;

#[derive(Debug)]
pub enum ObserveError {
    /// Der Parser konnte den semantischen Kern nicht zurueckgewinnen.
    ParseFailed(String),
    Collect(CollectError),
}

/// observe(Artifact) → MatrixCrystal. Der `parser` ist der domaenen-konkrete
/// Rueckleser (Adapter-Punkt 6); alles Weitere ist der EINE Collect-Pfad.
pub fn observe(
    artifact_bytes: &[u8],
    parser: impl Fn(&[u8]) -> Result<CanonValue, String>,
    provenance: &str,
) -> Result<MatrixCrystal, ObserveError> {
    let semantic_core = parser(artifact_bytes).map_err(ObserveError::ParseFailed)?;
    let markers = match &semantic_core {
        CanonValue::Map(m) => m
            .keys()
            .map(|k| Marker {
                id: format!("m:{k}"),
                marker_type: "field".to_string(),
                query: k.clone(),
                scope: None,
            })
            .collect(),
        _ => Vec::new(),
    };
    collect(&CollectInput {
        embedded: semantic_core,
        markers,
        null_models: vec!["Artefakt nennt Einheiten ohne Stuetzung (reine Nennung)".to_string()],
        provenance: provenance.to_string(),
    })
    .map_err(ObserveError::Collect)
}
