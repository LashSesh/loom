//! Closure-Gesetz (F-06): `closed?(x) ⟺ B⁻x = 0` — mit ausgewiesenem
//! Residuenfeld. Closure-Zertifikat als pruefbarer Beleg.

use crate::gate::GateChain;
use crate::reflection::TripolarFiber;
use crate::residue::ResidueField;
use crate::signature::Digest;

/// `closed?` — Zentralpraedikat.
pub fn is_closed(fiber: &TripolarFiber, residue: &ResidueField) -> bool {
    fiber.is_closed() && !residue.has_blocking()
}

/// Closure-Zertifikat (F-14, vereinfachter Traeger; Vollform in objects::Certificate):
/// Nachweis eines akzeptierten Kollapszustands.
#[derive(Debug, Clone)]
pub struct ClosureCertificate {
    pub subject: String,
    pub closed: bool,
    pub gate_chain_pass: bool,
    pub residue_summary: String,
    pub replay_hash: Option<Digest>,
}

impl ClosureCertificate {
    /// Zertifikat entsteht NUR aus tatsaechlicher Pruefung (kein Konstruktor
    /// fuer ein unbelegtes "closed=true").
    pub fn issue(
        subject: &str,
        fiber: &TripolarFiber,
        residue: &ResidueField,
        gates: &GateChain,
        replay_hash: Option<Digest>,
    ) -> Self {
        Self {
            subject: subject.to_string(),
            closed: is_closed(fiber, residue) && gates.all_pass(),
            gate_chain_pass: gates.all_pass(),
            residue_summary: residue.render_summary(),
            replay_hash,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gate::GateReport;
    use crate::reflection::{reflect, PolarItem};
    use crate::value::CanonValue;

    /// INV-4-Anteil: Materialize-Vorbedingung `B⁻x = 0` mit ausgewiesenem Feld.
    #[test]
    fn closed_requires_empty_residual_pole() {
        let open_fiber = reflect(
            &[PolarItem {
                value: CanonValue::text("offen"),
                accepted: false,
            }],
            "s",
        );
        let closed_fiber = reflect(
            &[PolarItem {
                value: CanonValue::text("fertig"),
                accepted: true,
            }],
            "s",
        );
        let empty = ResidueField::new();
        assert!(!is_closed(&open_fiber, &empty));
        assert!(is_closed(&closed_fiber, &empty));
    }

    /// VC10-Grundlage: Zertifikat nur mit Gate-Kette UND Closure wahrhaftig.
    #[test]
    fn certificate_reflects_actual_state() {
        let fiber = reflect(
            &[PolarItem {
                value: CanonValue::text("x"),
                accepted: true,
            }],
            "s",
        );
        let residue = ResidueField::new();
        let mut gates = GateChain::new();
        gates.push(GateReport::hold("G5-Replay", "RD unvollstaendig"));
        let cert = ClosureCertificate::issue("t", &fiber, &residue, &gates, None);
        assert!(!cert.closed, "Zertifikat darf Hold nicht ueberdecken");
        assert_eq!(cert.residue_summary, "geschlossen (∅)");
    }
}
