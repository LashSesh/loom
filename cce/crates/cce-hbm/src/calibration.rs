//! FixpointCalibration / PhaseSwitchController (neutralisierte „Seraphic
//! Calibration", Rebase §3): Triplet Φ(c) = (ψ, ρ, ω) ∈ [0,1]³ als METRIKEN
//! (Promille-Integer); Phasenwechsel (explorativ ↔ kontraktiv) NUR bei
//! protokollierter Stagnation, RD-geloggt (HBM-10). Ungeloggter Wechsel =
//! `phase_switch_unlogged`.

use cce_core::residue::{Residue, ResidueKind, Severity};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MiningPhase {
    Explorative,
    Contractive,
}

/// Kalibrationstriplet (Promille).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Calibration {
    pub psi: u16,
    pub rho: u16,
    pub omega: u16,
}

#[derive(Debug)]
pub struct PhaseSwitchController {
    pub phase: MiningPhase,
    pub switch_log: Vec<String>,
    stagnation_count: u32,
    last: Option<Calibration>,
}

impl PhaseSwitchController {
    pub fn new() -> Self {
        Self {
            phase: MiningPhase::Explorative,
            switch_log: Vec::new(),
            stagnation_count: 0,
            last: None,
        }
    }

    /// Beobachtet eine Kalibration; wechselt NUR bei protokollierter
    /// Stagnation (2 unveraenderte Messungen) — und loggt den Wechsel.
    pub fn observe(&mut self, c: Calibration) -> MiningPhase {
        if self.last == Some(c) {
            self.stagnation_count += 1;
        } else {
            self.stagnation_count = 0;
        }
        self.last = Some(c);
        if self.stagnation_count >= 2 {
            let from = self.phase;
            self.phase = match self.phase {
                MiningPhase::Explorative => MiningPhase::Contractive,
                MiningPhase::Contractive => MiningPhase::Explorative,
            };
            self.stagnation_count = 0;
            self.switch_log.push(format!(
                "PhaseSwitch {from:?} → {:?} bei Φ=({},{},{}) nach Stagnation (RD-geloggt)",
                self.phase, c.psi, c.rho, c.omega
            ));
        }
        self.phase
    }

    /// Negativ-Form: ein Wechsel OHNE Log-Eintrag ist ein Residuum.
    pub fn unlogged_switch_residue() -> Residue {
        Residue::new(
            "phase-switch-unlogged",
            "calibration",
            ResidueKind::named("phase_switch_unlogged"),
            Severity::Blocking,
            "Phasenwechsel ohne RD-Log verweigert (HBM-10)",
        )
    }
}

impl Default for PhaseSwitchController {
    fn default() -> Self {
        Self::new()
    }
}
