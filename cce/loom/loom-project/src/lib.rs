//! loom-project — Workcell-Projektionen aus dem PHC-Segment
//! (Ports: cce-phc / cce-lattice). Mount liefert getypte, LOKALE,
//! gate-faehige Projektionen — nie den globalen Horizont als Vollprompt.

use cce_core::canonical::Canonicalize;
use cce_phc::projection_calc::LocalProjection;
use loom_canon::Cv;

/// Baut das PHC-Segment-Payload aus lokalen Projektionen (deren
/// kanonische Klasse wird transportiert, nicht der Horizont).
pub fn phc_segment(projections: &[LocalProjection]) -> Cv {
    Cv::map(vec![(
        "projections",
        Cv::Array(
            projections
                .iter()
                .map(|p| {
                    Cv::map(vec![
                        ("cell_id", Cv::Text(p.cell_id.clone())),
                        ("class", Cv::Text(p.payload.canonical_class().to_string())),
                    ])
                })
                .collect(),
        ),
    )])
}

/// project(mount, workcell_id): liefert die EINE lokale Projektion.
pub fn project<'a>(
    projections: &'a [LocalProjection],
    workcell_id: &str,
) -> Option<&'a LocalProjection> {
    projections.iter().find(|p| p.cell_id == workcell_id)
}
