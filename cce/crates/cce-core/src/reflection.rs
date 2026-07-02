//! Reflexion & tripolare Faser (F-03/F-04/F-05):
//! `R = R*, R² = I`, `B = ½(I+R)`, `B⁻ = ½(I−R)`, `B + B⁻ = I`, `B·B⁻ = 0`.
//! Operativ auf typisierten Item-Mengen: R klassifiziert jedes Item als
//! akzeptiert oder residual; B/B⁻ sind die zugehoerigen Projektoren.

use crate::residue::ResidueField;
use crate::value::CanonValue;

/// Ein Zustandsitem mit Akzeptanz-Polaritaet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolarItem {
    pub value: CanonValue,
    pub accepted: bool,
}

/// Tripolare Faser Γ(x) = (p⁺, p⁻, m): akzeptierter Pol, Residualpol, Naht.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TripolarFiber {
    pub accepted: Vec<CanonValue>,
    pub residual: Vec<CanonValue>,
    pub seam: String,
}

impl TripolarFiber {
    /// Closure-Gesetz: `B⁻x = 0` — Residualpol leer, UND ausgewiesen.
    pub fn is_closed(&self) -> bool {
        self.residual.is_empty()
    }
}

/// Involution R auf einem Zustand (Menge polarer Items).
/// `reflect` erzeugt die tripolare Faser; zweifache Anwendung ist Identitaet.
pub fn reflect(items: &[PolarItem], seam: &str) -> TripolarFiber {
    TripolarFiber {
        accepted: items
            .iter()
            .filter(|i| i.accepted)
            .map(|i| i.value.clone())
            .collect(),
        residual: items
            .iter()
            .filter(|i| !i.accepted)
            .map(|i| i.value.clone())
            .collect(),
        seam: seam.to_string(),
    }
}

/// R als Involution auf Items: Polaritaetstausch. `R∘R = I`.
pub fn involute(items: &[PolarItem]) -> Vec<PolarItem> {
    items
        .iter()
        .map(|i| PolarItem {
            value: i.value.clone(),
            accepted: !i.accepted,
        })
        .collect()
}

/// B-Projektor: akzeptierter Anteil.
pub fn project_accepted(items: &[PolarItem]) -> Vec<PolarItem> {
    items.iter().filter(|i| i.accepted).cloned().collect()
}

/// B⁻-Projektor: Residualanteil.
pub fn project_residual(items: &[PolarItem]) -> Vec<PolarItem> {
    items.iter().filter(|i| !i.accepted).cloned().collect()
}

/// Residuenfeld aus der Faser (stets ausgewiesen, auch leer).
pub fn residue_of(fiber: &TripolarFiber) -> ResidueField {
    use crate::residue::{Residue, ResidueKind, Severity};
    let mut f = ResidueField::new();
    for (i, r) in fiber.residual.iter().enumerate() {
        f.push(Residue::new(
            &format!("fiber-residual-{i}"),
            &fiber.seam,
            ResidueKind::UnresolvedConstraint,
            Severity::Blocking,
            &format!("{r:?}"),
        ));
    }
    f
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Vec<PolarItem> {
        vec![
            PolarItem {
                value: CanonValue::text("a"),
                accepted: true,
            },
            PolarItem {
                value: CanonValue::text("b"),
                accepted: false,
            },
            PolarItem {
                value: CanonValue::text("c"),
                accepted: true,
            },
        ]
    }

    /// F-03: R² = I.
    #[test]
    fn reflection_is_involution() {
        let s = sample();
        assert_eq!(involute(&involute(&s)), s);
    }

    /// F-04: B + B⁻ = I und B·B⁻ = 0.
    #[test]
    fn projectors_partition_state() {
        let s = sample();
        let b = project_accepted(&s);
        let bm = project_residual(&s);
        assert_eq!(b.len() + bm.len(), s.len());
        // B·B⁻ = 0: der Residualanteil des akzeptierten Anteils ist leer.
        assert!(project_residual(&b).is_empty());
        // B idempotent.
        assert_eq!(project_accepted(&b), b);
    }

    /// INV-3-Anteil: die Faser weist ihren Residualpol immer aus.
    #[test]
    fn fiber_exposes_residual() {
        let f = reflect(&sample(), "seam:test");
        assert!(!f.is_closed());
        assert_eq!(f.residual.len(), 1);
        assert!(!residue_of(&f).is_closed_empty());
    }
}
