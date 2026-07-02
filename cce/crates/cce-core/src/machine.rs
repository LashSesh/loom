//! BCIK-Maschine (I-11, BCIK §36): Kleinschritt-Semantik mit den Regeln
//! Canon → ExCal → Seam → Sediment → Assim → Commit | Hold.
//! Metatheoreme als Tests: Erhaltung (Subject Reduction) und Fortschritt
//! (INV-8), Determinismus mod Can.

use crate::gate::GateChain;
use crate::residue::{Residue, ResidueField, ResidueKind, Severity};
use crate::value::CanonValue;

/// Phasen der Maschine (Lebenszyklus-Achse raw→…→closed, Teil 3.8).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Phase {
    Raw,
    Canonized,
    Excavated,
    Seamed,
    Sedimented,
    Assimilated,
    Committed,
    Held,
}

/// Konfiguration (x, Σ, ℱ, ℓ, g).
#[derive(Debug, Clone)]
pub struct Conf {
    pub state: CanonValue,
    pub phase: Phase,
    pub residue: ResidueField,
    pub gates: GateChain,
    pub trace: Vec<&'static str>,
}

impl Conf {
    pub fn new(state: CanonValue) -> Self {
        Self {
            state,
            phase: Phase::Raw,
            residue: ResidueField::new(),
            gates: GateChain::new(),
            trace: Vec::new(),
        }
    }

    pub fn is_terminal(&self) -> bool {
        matches!(self.phase, Phase::Committed | Phase::Held)
    }
}

/// Ein Kleinschritt. Deterministisch: je Phase genau eine anwendbare Regel.
pub fn step(mut c: Conf) -> Conf {
    match c.phase {
        Phase::Raw => {
            c.state = c.state.normalize();
            c.phase = Phase::Canonized;
            c.trace.push("Canon");
        }
        Phase::Canonized => {
            c.phase = Phase::Excavated;
            c.trace.push("ExCal");
        }
        Phase::Excavated => {
            c.phase = Phase::Seamed;
            c.trace.push("Seam");
        }
        Phase::Seamed => {
            c.phase = Phase::Sedimented;
            c.trace.push("Sediment");
        }
        Phase::Sedimented => {
            c.phase = Phase::Assimilated;
            c.trace.push("Assim");
        }
        Phase::Assimilated => {
            // Commit nur bei Gate-Pass und ohne blocking Residuum; sonst Hold.
            if c.gates.all_pass() && !c.residue.has_blocking() {
                c.phase = Phase::Committed;
                c.trace.push("Commit");
            } else {
                if c.gates.reports.is_empty() {
                    // fehlende Pruefung wird nicht durchgelassen (V7):
                    c.residue.push(Residue::new(
                        "machine-no-gates",
                        "machine",
                        ResidueKind::FailedGate,
                        Severity::Blocking,
                        "Commit ohne Gate-Kette verweigert (fail-closed)",
                    ));
                }
                c.phase = Phase::Held;
                c.trace.push("Hold");
            }
        }
        Phase::Committed | Phase::Held => {}
    }
    c
}

/// Laufe bis zum Terminalzustand (Fortschritt garantiert Terminierung).
pub fn run(mut c: Conf) -> Conf {
    let mut fuel = 64;
    while !c.is_terminal() && fuel > 0 {
        c = step(c);
        fuel -= 1;
    }
    c
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canonical::Canonicalize;
    use crate::gate::GateReport;

    /// INV-8 Fortschritt: jeder wohlgeformte, nicht-terminale Zustand
    /// hat einen naechsten Schritt; die Maschine terminiert.
    #[test]
    fn progress_terminates() {
        let c = run(Conf::new(CanonValue::text("x")));
        assert!(c.is_terminal());
    }

    /// INV-8 Erhaltung: die kanonische Klasse des Zustands bleibt unter
    /// allen Maschinenregeln erhalten (Signatur-Erhalt).
    #[test]
    fn subject_reduction_preserves_class() {
        let start = CanonValue::map([(
            "v",
            CanonValue::Decimal {
                mantissa: 100,
                exponent: -2,
            },
        )]);
        let class_before = start.normalize().canonical_class();
        let mut c = Conf::new(start);
        while !c.is_terminal() {
            c = step(c);
            assert_eq!(
                c.state.canonical_class(),
                class_before,
                "Regel {} hat die Klasse veraendert",
                c.trace.last().unwrap()
            );
        }
    }

    /// Fail-closed: ohne Gate-Kette endet die Maschine in Hold, nie Commit —
    /// mit sichtbarem Residuum.
    #[test]
    fn commit_requires_gates() {
        let c = run(Conf::new(CanonValue::text("x")));
        assert_eq!(c.phase, Phase::Held);
        assert!(c.residue.has_blocking());
    }

    /// Mit vollstaendiger Gate-Kette: Commit.
    #[test]
    fn commit_with_full_gate_chain() {
        let mut c = Conf::new(CanonValue::text("x"));
        for g in crate::gate::mandatory_gates() {
            c.gates.push(GateReport::pass(&g.id, "ok"));
        }
        let c = run(c);
        assert_eq!(c.phase, Phase::Committed);
        assert_eq!(
            c.trace,
            vec!["Canon", "ExCal", "Seam", "Sediment", "Assim", "Commit"]
        );
    }
}
