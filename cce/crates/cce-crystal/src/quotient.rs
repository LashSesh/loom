//! Quotient q und ≃ (Teil 0.1/4.7): Gleichheit modulo deklariertem
//! Quotienten auf Kristallklassen — `q(Obs(A)) = q(C)`.

use crate::crystal::Crystal;
use cce_core::canonical::CanonicalClass;

/// q: Crystal → Klasse (der deklarierte Quotient: kanonische Inhaltsklasse).
pub fn quotient_class(c: &Crystal) -> CanonicalClass {
    c.class()
}

/// ≃ — das eine Gleichheitsmass des Kerntests.
pub fn equivalent(a: &Crystal, b: &Crystal) -> bool {
    quotient_class(a) == quotient_class(b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cce_ccc::crystal_protocol::CrystalCandidate;
    use cce_core::closure::ClosureCertificate;
    use cce_core::gate::{GateChain, GateReport};
    use cce_core::reflection::{reflect, PolarItem};
    use cce_core::residue::ResidueField;
    use cce_core::value::CanonValue;

    fn make(content: CanonValue) -> Crystal {
        let mut gates = GateChain::new();
        for g in cce_core::gate::mandatory_gates() {
            gates.push(GateReport::pass(&g.id, "ok"));
        }
        let candidate = CrystalCandidate {
            closed: true,
            qsr_stable: true,
            gates: gates.clone(),
            replay_ok: true,
            residue_field: Some(ResidueField::new()),
        };
        let fiber = reflect(
            &[PolarItem {
                value: CanonValue::text("t"),
                accepted: true,
            }],
            "s",
        );
        let cert = ClosureCertificate::issue("t", &fiber, &ResidueField::new(), &gates, None);
        Crystal::certify(content, &candidate, "test", cert).expect("protocol ok")
    }

    /// Zwei-Digest-Grundlage (S7.2): verschiedene Rohform, gleiche Klasse.
    #[test]
    fn equivalence_is_class_equality() {
        let a = make(CanonValue::Decimal {
            mantissa: 100,
            exponent: -2,
        });
        let b = make(CanonValue::Int(1));
        assert!(equivalent(&a, &b), "1.00 ≃ 1 (kanonische Klasse)");
        let c = make(CanonValue::Int(2));
        assert!(!equivalent(&a, &c));
    }

    /// Crystal ist Klasse, nie Repraesentant: id = crystal:<klassen-hash>.
    #[test]
    fn crystal_id_is_content_addressed_class() {
        let a = make(CanonValue::Int(7));
        assert_eq!(a.id.namespace, "crystal");
        assert_eq!(a.id.digest, a.class().0);
        assert_eq!(a.residue_state, "geschlossen (∅)");
    }

    /// Kein Crystal unter Protokollverletzung (fail-closed).
    #[test]
    fn certify_rejects_protocol_violation() {
        let candidate = CrystalCandidate {
            closed: true,
            qsr_stable: true,
            gates: GateChain::new(), // leer ⇒ fail-closed
            replay_ok: true,
            residue_field: Some(ResidueField::new()),
        };
        let fiber = reflect(
            &[PolarItem {
                value: CanonValue::text("t"),
                accepted: true,
            }],
            "s",
        );
        let cert =
            ClosureCertificate::issue("t", &fiber, &ResidueField::new(), &GateChain::new(), None);
        assert!(Crystal::certify(CanonValue::Int(1), &candidate, "t", cert).is_err());
    }

    /// Monolith: Commit erzeugt append-only Ledger-Ereignis.
    #[test]
    fn monolith_commit_is_ledgered() {
        use crate::monolith::commit_monolith;
        let a = make(CanonValue::Int(3));
        let mut ledger = cce_core::ledger::Ledger::new();
        let m = commit_monolith(&mut ledger, a.class().0);
        assert_eq!(m.ledger_seq, 0);
        assert!(cce_core::ledger::verify_ledger(&ledger).is_ok());
    }
}
