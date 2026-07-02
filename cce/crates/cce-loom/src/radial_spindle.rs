//! Radiale Spindel (G-01): Rad = (Fin, Fout, Z0, Σ, M, Replay) —
//! 4π-Orientierungsrahmen. Der Nullanker wird MARKIERT, nie durchlaufen
//! (P7/V3); jede Nullnaehe erzeugt BoundaryTrace + Mandorla-Naht-Bindung.

use cce_core::objects::{BoundaryTrace, NullAnchor, Seam};

#[derive(Debug, Clone)]
pub struct RadialSpindle {
    pub zero_anchor: NullAnchor,
    pub orientation: &'static str,
    pub mandorla_seam: Seam,
    pub boundary_traces: Vec<BoundaryTrace>,
}

impl RadialSpindle {
    pub fn new(id: &str) -> Self {
        Self {
            zero_anchor: NullAnchor::new(&format!("Z0:{id}")),
            orientation: "4pi",
            mandorla_seam: Seam::new(
                &format!("mandorla:{id}"),
                &["collect", "distribute"],
                "same_crystal_class_under_declared_quotient",
                "G2-Boundary",
            ),
            boundary_traces: Vec::new(),
        }
    }

    /// Nullnaehe: erzeugt Trace, NIE Durchgang. Der Rueckgabewert ist die
    /// Markierung — es existiert kein Pfad "durch" den Anker.
    pub fn approach_zero(&mut self, note: &str) -> &BoundaryTrace {
        let t = self.zero_anchor.approach(note);
        self.boundary_traces.push(t);
        self.boundary_traces.last().expect("just pushed")
    }

    /// Boundary-Regularisierung: nullpoint_traversal ist KONSTANT verboten.
    pub const NULLPOINT_TRAVERSAL: &'static str = "forbidden";
    pub const BOUNDARY_TRACE_REQUIRED: bool = true;
}
