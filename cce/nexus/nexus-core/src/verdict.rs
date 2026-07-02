//! Vier-Wege-Verdikt (CSA.7): allow | hold | reject | quarantine —
//! boolesch-begruendet, ledgerpflichtig; nie stilles Teilergebnis.

use cce_core::residue::Residue;

#[derive(Debug, Clone)]
pub enum Verdict {
    Allow {
        gate: String,
        reason: String,
    },
    Hold {
        gate: String,
        residue: Box<Residue>,
    },
    Reject {
        gate: String,
        residue: Box<Residue>,
    },
    /// Quarantaene: bereits geholte Beobachtungen isoliert, ohne
    /// Export-/Importpfad, bis ein Gate-Neuentscheid vorliegt.
    Quarantine {
        gate: String,
        residue: Box<Residue>,
    },
}

impl Verdict {
    pub fn allows(&self) -> bool {
        matches!(self, Verdict::Allow { .. })
    }

    pub fn gate(&self) -> &str {
        match self {
            Verdict::Allow { gate, .. }
            | Verdict::Hold { gate, .. }
            | Verdict::Reject { gate, .. }
            | Verdict::Quarantine { gate, .. } => gate,
        }
    }
}
