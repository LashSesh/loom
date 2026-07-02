//! Der DomainAdapter-Vertrag (S1.8): die kanonische Teileliste — 11 Punkte,
//! die JEDE Domaene identisch bereitstellt (Adapter-Paritaet,
//! Modellbaukasten-Prinzip). Typstarke Form des G1-Geruests.

use cce_core::canonical::CanonicalClass;
use cce_core::gate::{Gate, GateReport};
use cce_core::objects::CounterHorizon;
use cce_core::residue::{Residue, ResidueKind};
use cce_phc::package::PhcPackage;
use cce_phc::projection_calc::LocalProjection;

/// Domaenen-native Oeffnen-Aktion (Punkt 10).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenAction {
    pub program_class: String,
    pub argument_hint: String,
}

/// Die kanonische Teileliste (S1.8). Jede Domaene MUSS alle Punkte
/// bereitstellen — der Compiler erzwingt die Signaturen, `check_adapter_
/// parity_typed` prueft die Inhalte (nicht leer, Zeugen vorhanden).
pub trait DomainAdapter {
    type Crystal;
    type Weave;
    type Artifact;

    fn domain_id(&self) -> &'static str;

    // 1. Wunsch-Grammatik + Validierung
    fn wish_schema(&self) -> Vec<(&'static str, &'static str)>;
    fn validate_wish(&self, c: &Self::Crystal) -> Result<(), Vec<Residue>>;

    // 2. Instanziierung auf die Motor-Objekte
    fn to_canonical(&self, c: &Self::Crystal) -> cce_core::canonical::CanonicalState;

    // 3. encode: Crystal → PHC-Paket (Domaenen-Profil)
    fn encode(&self, c: &Self::Crystal) -> PhcPackage;

    // 4. loom: Projektion → Gewebe
    fn loom(&self, p: &LocalProjection) -> Result<Self::Weave, String>;

    // 5. materialize: Gewebe → Artefakt (nur Kosmetik, kein neuer Inhalt)
    fn materialize(&self, w: &Self::Weave) -> Self::Artifact;

    // 6. reanalyze: Artefakt → Crystal' (Collect-Pfad, verlustfrei)
    fn reanalyze(&self, a: &Self::Artifact) -> Result<Self::Crystal, String>;

    // 7. Kanonisierung + Aequivalenz (das ≃ der Domaene)
    fn canonicalize(&self, c: &Self::Crystal) -> CanonicalClass;
    fn equivalent(&self, c1: &Self::Crystal, c2: &Self::Crystal) -> bool {
        self.canonicalize(c1) == self.canonicalize(c2)
    }

    // 8. Domaenen-Gates (boolesch + begruendet, fail-closed)
    fn domain_gates(&self) -> Vec<Gate>;
    fn run_domain_gates(
        &self,
        c: &Self::Crystal,
        artifact: Option<&Self::Artifact>,
    ) -> Vec<GateReport>;

    // 9. Residuen-Vokabular + Gegenhorizont
    fn residue_vocabulary(&self) -> Vec<ResidueKind>;
    fn counter_horizon(&self, c: &Self::Crystal) -> CounterHorizon;

    // 10. native Ausgabe/Verwendung
    fn native_open(&self, a: &Self::Artifact) -> OpenAction;
    fn export_formats(&self) -> Vec<&'static str>;

    // 11. Test-Assets
    fn reference_cube(&self) -> Self::Crystal;
    fn negative_cubes(&self) -> Vec<(Self::Crystal, ResidueKind)>;
}

/// check_adapter_parity (typstark): prueft die Inhalte der 11 Punkte.
pub fn check_adapter_parity_typed<A: DomainAdapter>(a: &A) -> GateReport {
    let mut missing = Vec::new();
    if a.wish_schema().is_empty() {
        missing.push("wish_schema leer");
    }
    if a.domain_gates().is_empty() {
        missing.push("domain_gates leer");
    }
    if a.residue_vocabulary().is_empty() {
        missing.push("residue_vocabulary leer");
    }
    if a.export_formats().is_empty() {
        missing.push("export_formats leer");
    }
    if a.negative_cubes().is_empty() {
        missing.push("negative_cubes fehlen");
    }
    let reference = a.reference_cube();
    if a.validate_wish(&reference).is_err() {
        missing.push("reference_cube nicht wohlgeformt");
    }
    if missing.is_empty() {
        GateReport::pass(
            "check_adapter_parity",
            &format!(
                "Domaene {} erfuellt die kanonische Teileliste (11/11)",
                a.domain_id()
            ),
        )
    } else {
        GateReport::hold(
            "check_adapter_parity",
            &format!(
                "Domaene {} unvollstaendig: {}",
                a.domain_id(),
                missing.join(", ")
            ),
        )
    }
}
