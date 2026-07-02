//! Gesamtumlauf (G-22, MERKABA §96): F = Commit∘C∘G∘Δ∘I∘Q∘Θ∘P∘Can.
//! Emission monolithisch ⟺ E ∈ L ∧ Replay(E)=1 ∧ Pass(G_E)=1 ∧ Res(E) ≤ ε.

use crate::organ::{compose, Organ};
use cce_core::canonical::Canonicalize;
use cce_core::gate::GateChain;
use cce_core::ledger::{Ledger, LedgerEventKind};
use cce_core::residue::ResidueField;
use cce_core::value::CanonValue;

#[derive(Debug)]
pub enum Emission {
    /// Monolithische Emission E mit Ledger-Anker.
    Emitted { value: CanonValue, ledger_seq: u64 },
    /// Verweigert mit benannten Gruenden (fail-closed).
    Refused(Vec<String>),
}

/// Der eine Lauf E = F(x): Organkette, dann Emissionsbedingung, dann Commit.
pub fn circulate(
    organs: &[Organ],
    x: &CanonValue,
    gates: &GateChain,
    residue: &ResidueField,
    replay_ok: bool,
    ledger: &mut Ledger,
) -> Emission {
    let e = compose(organs, x);
    let mut refusals = Vec::new();
    if !gates.all_pass() {
        refusals.push("Pass(G_E) = 0".to_string());
    }
    if residue.has_blocking() {
        refusals.push("Res(E) > ε (blocking Residuum)".to_string());
    }
    if !replay_ok {
        refusals.push("Replay(E) = 0".to_string());
    }
    if !refusals.is_empty() {
        return Emission::Refused(refusals);
    }
    let event = ledger.append(LedgerEventKind::Commit, e.canonical_class().0);
    Emission::Emitted {
        value: e,
        ledger_seq: event.seq,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cce_core::gate::GateReport;

    fn organs() -> Vec<Organ> {
        // P: Projektion — Θ: Fensterung — Q: Spektrallogik — I: Instanz —
        // Δ: Differenz — G: Gate-Vorbereitung — C: Kondensation.
        // Hier als klassenerhaltende Identitaets-Organe (der eigentliche
        // Inhalt lebt in den Fach-Crates; MERKABA orchestriert).
        fn ident(x: &CanonValue) -> CanonValue {
            x.clone()
        }
        ["P", "Theta", "Q", "I", "Delta", "G", "C"]
            .iter()
            .map(|id| Organ { id, alpha: ident })
            .collect()
    }

    /// Master-Normalform: der Umlauf ist deterministisch und
    /// klassenerhaltend; Emission nur unter allen Bedingungen.
    #[test]
    fn circulation_emits_only_under_full_conditions() {
        let mut gates = GateChain::new();
        for g in cce_core::gate::mandatory_gates() {
            gates.push(GateReport::pass(&g.id, "ok"));
        }
        let x = CanonValue::map([("kern", CanonValue::Int(1))]);
        let mut ledger = Ledger::new();
        match circulate(
            &organs(),
            &x,
            &gates,
            &ResidueField::new(),
            true,
            &mut ledger,
        ) {
            Emission::Emitted { value, ledger_seq } => {
                assert_eq!(value.canonical_class(), x.canonical_class());
                assert_eq!(ledger_seq, 0);
            }
            Emission::Refused(r) => panic!("Emission verweigert: {r:?}"),
        }
        // ohne Replay: verweigert.
        match circulate(
            &organs(),
            &x,
            &gates,
            &ResidueField::new(),
            false,
            &mut ledger,
        ) {
            Emission::Refused(reasons) => {
                assert!(reasons.iter().any(|r| r.contains("Replay")));
            }
            Emission::Emitted { .. } => panic!("Emission ohne Replay (verboten)"),
        }
    }

    /// Konvergenz der Projektionsflaechen: verschiedene Organ-Ordnungen
    /// derselben klassenerhaltenden Familie ⇒ EINE Signaturklasse.
    #[test]
    fn all_projection_faces_converge_to_one_class() {
        let x = CanonValue::map([("kern", CanonValue::Int(2))]);
        let mut o1 = organs();
        let o2 = {
            let mut v = organs();
            v.reverse();
            v
        };
        let a = compose(&o1, &x).canonical_class();
        let b = compose(&o2, &x).canonical_class();
        assert_eq!(a, b);
        o1.truncate(3);
        assert_eq!(compose(&o1, &x).canonical_class(), a);
    }
}
