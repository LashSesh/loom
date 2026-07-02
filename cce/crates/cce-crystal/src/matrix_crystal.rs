//! MatrixCrystal ⊂ Crystal (Teil 3.4, O-10): die geschlossene Horizontkarte
//! eines Collect-Laufs — der Typ, den `Reanalyze` erzeugt.

use crate::crystal::Crystal;
use cce_core::gate::GateReport;
use cce_core::objects::{CounterHorizon, Horizon};
use cce_core::residue::Residue;
use cce_core::signature::Digest;

#[derive(Debug, Clone)]
pub struct MatrixCrystal {
    pub crystal: Crystal,
    pub attractor_map: Vec<String>,
    pub horizons: Horizon,
    pub counter_horizon: CounterHorizon,
    pub triangulation: Vec<(String, String)>,
    pub gates: Vec<GateReport>,
    pub residues: Vec<Residue>,
    pub null_models: Vec<String>,
    pub trace: Vec<String>,
    pub replay_hash: Digest,
}

impl MatrixCrystal {
    /// MatrixCrystal ist ein Crystal mit Horizontfeldern (Rollenkonflikt-
    /// Aufloesung 3, Teil 2.3): die Klasse ist die des Traeger-Kristalls.
    pub fn class(&self) -> cce_core::canonical::CanonicalClass {
        self.crystal.class()
    }
}
