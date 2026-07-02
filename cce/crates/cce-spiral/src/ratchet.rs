//! Multi-Ratchet (Formel 5): Cell/Phase/Ring/Scale-Ratchets (+Nexus als
//! Typ-Stub, R-1b). `Step(r) = 1 ⟺ G(r) = Pass ∧ E(r) = 1 ∧ ρ(r) sichtbar`.
//! Intrinsische Phase rotiert frei; der Commitzaehler steigt NUR unter
//! Gate/Evidence. Kaskadenregel: `Lock(R_{n+1}) = 1 ⇒ ∀R_i ≺ R_{n+1}:
//! Lock(R_i) = 1 ∨ Resid(R_i) sichtbar`.

use cce_core::gate::GateReport;
use cce_core::residue::Residue;
use cce_core::signature::Digest;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RatchetKind {
    Cell,
    Phase,
    Ring,
    Scale,
    /// Typ-Stub: spaetere Foundation (R-1b) — nie aktivierbar in diesem Bau.
    Nexus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RatchetState {
    Advance,
    Hold,
    Lock,
}

#[derive(Debug, Clone)]
pub struct Ratchet {
    pub kind: RatchetKind,
    pub id: String,
    pub state: RatchetState,
    /// Commitzaehler c — steigt NUR via step().
    pub commit_counter: u64,
    /// Intrinsische Phase (Milliturns) — rotiert frei.
    pub theta_milliturns: u32,
    pub evidence_refs: Vec<Digest>,
    pub visible_residues: Vec<Residue>,
}

impl Ratchet {
    pub fn new(kind: RatchetKind, id: &str) -> Self {
        Self {
            kind,
            id: id.to_string(),
            state: RatchetState::Advance,
            commit_counter: 0,
            theta_milliturns: 0,
            evidence_refs: Vec::new(),
            visible_residues: Vec::new(),
        }
    }

    /// Freie Rotation (F6): beruehrt NIE den Commitzaehler.
    pub fn rotate(&mut self, delta: u32) {
        self.theta_milliturns = (self.theta_milliturns + delta) % 1000;
    }

    /// Ratchet-Schritt: nur unter Gate=Pass ∧ Evidence ∧ Residuum sichtbar.
    /// Ohne Evidence ⇒ Hold + `ratchet_lock_without_evidence`.
    pub fn step(&mut self, gate: &GateReport, evidence: Option<Digest>) -> RatchetState {
        if self.state == RatchetState::Lock {
            return RatchetState::Lock; // irreversibel: kein Ruecklauf.
        }
        match (gate.is_pass(), evidence) {
            (true, Some(ev)) => {
                self.commit_counter += 1;
                self.evidence_refs.push(ev);
                self.state = RatchetState::Advance;
            }
            (true, None) => {
                self.visible_residues
                    .push(crate::residues::ratchet_lock_without_evidence(&format!(
                        "Ratchet {} ohne Evidence — Hold",
                        self.id
                    )));
                self.state = RatchetState::Hold;
            }
            (false, _) => {
                self.visible_residues
                    .push(gate.to_residue().expect("Hold-Report traegt Residuum"));
                self.state = RatchetState::Hold;
            }
        }
        self.state
    }

    /// Lock: nur aus Advance mit mindestens einem Commit + Evidence.
    pub fn lock(&mut self) -> Result<(), Box<Residue>> {
        if self.state == RatchetState::Advance
            && self.commit_counter > 0
            && !self.evidence_refs.is_empty()
        {
            self.state = RatchetState::Lock;
            Ok(())
        } else {
            let r = crate::residues::ratchet_lock_without_evidence(&format!(
                "Lock von {} verweigert (state={:?}, c={}, ev={})",
                self.id,
                self.state,
                self.commit_counter,
                self.evidence_refs.len()
            ));
            self.visible_residues.push(r.clone());
            Err(Box::new(r))
        }
    }
}

/// Kaskadenregel (Formel 5): das hoehere Ratchet lockt nur, wenn ALLE
/// untergeordneten gelockt sind ODER ihr Residuum sichtbar ist
/// (Counter-Horizon).
pub fn cascade_lock(higher: &mut Ratchet, lower: &[&Ratchet]) -> Result<(), Box<Residue>> {
    for l in lower {
        let locked = l.state == RatchetState::Lock;
        let residue_visible = !l.visible_residues.is_empty();
        if !locked && !residue_visible {
            let r = crate::residues::ratchet_lock_without_evidence(&format!(
                "Kaskade verletzt: {} weder gelockt noch residuen-sichtbar",
                l.id
            ));
            higher.visible_residues.push(r.clone());
            return Err(Box::new(r));
        }
    }
    higher.lock()
}
