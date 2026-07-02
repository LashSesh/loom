//! Kristall-Protokoll (CCC §18, F-16-Umfeld):
//! `Kristall ⟺ Closed ∧ QSR-Stabil ∧ Gate ∧ Replay ∧ ResidueVisible`.
//! Kein Konjunkt ist verhandelbar; das Praedikat ist fail-closed.

use cce_core::gate::GateChain;
use cce_core::residue::ResidueField;

/// Kandidat fuer das Kristall-Praedikat.
#[derive(Debug, Clone)]
pub struct CrystalCandidate {
    pub closed: bool,
    pub qsr_stable: bool,
    pub gates: GateChain,
    pub replay_ok: bool,
    /// Das Residuenfeld MUSS existieren (auch leer) — sonst kein Kristall.
    pub residue_field: Option<ResidueField>,
}

/// Das Zentralpraedikat. Liefert bei Ablehnung die benannten Gruende.
pub fn is_crystal(c: &CrystalCandidate) -> Result<(), Vec<String>> {
    let mut missing = Vec::new();
    if !c.closed {
        missing.push("Closed = 0 (B⁻x ≠ 0)".to_string());
    }
    if !c.qsr_stable {
        missing.push("QSR-Stabilitaet fehlt".to_string());
    }
    if !c.gates.all_pass() {
        missing.push("Gate-Kette nicht Pass (oder leer — fail-closed)".to_string());
    }
    if !c.replay_ok {
        missing.push("Replay nicht nachgewiesen".to_string());
    }
    match &c.residue_field {
        None => missing
            .push("Residuenfeld NICHT ausgewiesen (V2: auch ∅ muss sichtbar sein)".to_string()),
        Some(f) => {
            if f.has_blocking() {
                missing.push("blocking Residuum offen".to_string());
            }
        }
    }
    if missing.is_empty() {
        Ok(())
    } else {
        Err(missing)
    }
}
