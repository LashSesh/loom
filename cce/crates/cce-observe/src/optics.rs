//! Panoptische Instrumentenoptik (O-15..O-18, PIO): optische Normalform
//! X0 → Apt → Lens → Split → Track → Dump → Wave → Pulse → Gate → XC.
//! Axiome: Apertur vor Fokus · Ueberschuss sichtbar · Nachfuehrung erhaelt
//! Fixpunkt. Die Apertur IST das Radfenster (cce-core::WheelWindow, I-7).

use cce_core::residue::{Residue, ResidueKind, Severity};
use cce_core::value::CanonValue;
use cce_core::wheel_window::WheelWindow;

/// Ergebnis des optischen Durchlaufs: fokussierter Strahl + SICHTBARER
/// Ueberschuss (nie still verworfen — PIO Axiom 2.2).
#[derive(Debug, Clone)]
pub struct OpticalPass {
    pub focused: Vec<CanonValue>,
    pub excess: Vec<Residue>,
}

/// Apertur vor Fokus: das Radfenster filtert, der Ueberschuss wird Residuum.
pub fn optical_pass(window: &WheelWindow, raw: &[(String, CanonValue)]) -> OpticalPass {
    let (passed, excess_scopes) = window.aperture(raw);
    OpticalPass {
        focused: passed.into_iter().cloned().collect(),
        excess: excess_scopes
            .iter()
            .map(|scope| {
                Residue::new(
                    &format!("optics-excess:{scope}"),
                    "optics:aperture",
                    ResidueKind::ExcludedMaterial,
                    Severity::Info,
                    &format!("Rohmasse ausserhalb der Apertur ({scope}) — sichtbar gefuehrt"),
                )
            })
            .collect(),
    }
}
