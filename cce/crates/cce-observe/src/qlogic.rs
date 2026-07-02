//! QLOGIC-Rollenimport (O-04..O-06): Spektralregister (R,F,T,S,E),
//! Q-Zustand, Proof-of-Resonance als GATE-Praedikat („Gate vor Emission")
//! — boolesch, fail-closed, NIE Konsens (Neutralisierung, Rebase §1.2).

use cce_core::gate::GateReport;

/// Spektralregister (Relation, Frequenz, Topologie, Symmetrie, Entropie) —
/// als Vollstaendigkeitsmarker der Beobachtung, keine Physik.
#[derive(Debug, Clone, Default)]
pub struct SpectralRegister {
    pub relation: Option<String>,
    pub frequency: Option<String>,
    pub topology: Option<String>,
    pub symmetry: Option<String>,
    pub entropy: Option<String>,
}

/// PoR-Gate: Emission nur bei vollstaendigem Register — fail-closed (Hold).
pub fn proof_of_resonance(reg: &SpectralRegister) -> GateReport {
    let missing: Vec<&str> = [
        ("relation", reg.relation.is_none()),
        ("frequency", reg.frequency.is_none()),
        ("topology", reg.topology.is_none()),
        ("symmetry", reg.symmetry.is_none()),
        ("entropy", reg.entropy.is_none()),
    ]
    .iter()
    .filter(|(_, m)| *m)
    .map(|(n, _)| *n)
    .collect();
    if missing.is_empty() {
        GateReport::pass("G-PoR", "Spektralregister vollstaendig belegt")
    } else {
        GateReport::hold(
            "G-PoR",
            &format!("Register unvollstaendig: {}", missing.join(", ")),
        )
    }
}
