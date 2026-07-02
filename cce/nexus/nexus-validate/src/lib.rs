//! nexus-validate — V_φ: Validierung, Deduplikation, Qualitaetssignatur
//! (CSA.10): σ(c) = (ψ, ρ, ω) ∈ [0,1]³ (Promille-Integer). Ranking
//! D(c) ordnet die WARTESCHLANGE; die GATE-Zulassung verlangt
//! ACHSEN-MINDESTWERTE — der Score entscheidet nie allein (CSA-A3).
//! 8/15 SchemaGate · 9/15 QualityGate · 10/15 ProvenanceGate.

use cce_core::canonical::Canonicalize;
use nexus_core::objects::Csu;
use nexus_core::residues::csa_residue;
use nexus_core::verdict::Verdict;

/// RD-gebundene Schwellen (Promille; CSA-R2/R-15: Kalibrierung sichtbar).
#[derive(Debug, Clone, Copy)]
pub struct QualityThresholds {
    pub tau_d: u32,
    pub tau_psi: u16,
    pub tau_rho: u16,
    pub tau_omega: u16,
}

impl Default for QualityThresholds {
    fn default() -> Self {
        Self {
            tau_d: 200,
            tau_psi: 300,
            tau_rho: 300,
            tau_omega: 100,
        }
    }
}

/// 8/15 SchemaGate: erwartete Struktur parsebar (payload nicht Null).
pub fn schema_gate(c: &Csu) -> Verdict {
    if matches!(c.payload, cce_core::value::CanonValue::Null) {
        Verdict::Hold {
            gate: "SchemaGate".into(),
            residue: Box::new(csa_residue(
                "schema_unparseable",
                &format!("CSU {} ohne parsebares Payload", c.uid),
            )),
        }
    } else {
        Verdict::Allow {
            gate: "SchemaGate".into(),
            reason: "Struktur parsebar".into(),
        }
    }
}

/// Ranking D(c): gewichtetes Produkt (Integer-Approximation des
/// geometrischen Mittels) — ORDNET die Warteschlange.
pub fn rank_d(c: &Csu) -> u32 {
    let (psi, rho, omega) = c.quality;
    // kubische Wurzel des Produkts, integer-approximiert:
    let product = u64::from(psi) * u64::from(rho) * u64::from(omega);
    let mut lo = 0u64;
    let mut hi = 1000;
    while lo < hi {
        let mid = (lo + hi).div_ceil(2);
        if mid * mid * mid <= product {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    lo as u32
}

/// 9/15 QualityGate: Achsen-Mindestwerte + D(c) ≥ τ_D — das GATE
/// entscheidet; fehlt eine Achse: `quality_axis_missing`.
pub fn quality_gate(c: &Csu, t: &QualityThresholds) -> Verdict {
    let (psi, rho, omega) = c.quality;
    if psi == 0 || rho == 0 || omega == 0 {
        return Verdict::Hold {
            gate: "QualityGate".into(),
            residue: Box::new(csa_residue(
                "quality_axis_missing",
                &format!("CSU {}: unbelegte Qualitaetsachse", c.uid),
            )),
        };
    }
    let d = rank_d(c);
    if d >= t.tau_d && psi >= t.tau_psi && rho >= t.tau_rho && omega >= t.tau_omega {
        Verdict::Allow {
            gate: "QualityGate".into(),
            reason: format!("D={d} ≥ τ_D={} und alle Achsen ueber Mindestwert", t.tau_d),
        }
    } else {
        Verdict::Hold {
            gate: "QualityGate".into(),
            residue: Box::new(csa_residue(
                "quality_axis_missing",
                &format!("Achsen-Mindestwert verfehlt: ψ={psi}, ρ={rho}, ω={omega}, D={d}"),
            )),
        }
    }
}

/// 10/15 ProvenanceGate: Locator, Hash, Lizenzstatus vorhanden.
pub fn provenance_gate(c: &Csu) -> Verdict {
    if c.provenance.is_empty() || c.license.is_empty() {
        Verdict::Hold {
            gate: "ProvenanceGate".into(),
            residue: Box::new(csa_residue(
                "provenance_gap",
                &format!("CSU {}: Provenienz/Lizenz unvollstaendig", c.uid),
            )),
        }
    } else {
        Verdict::Allow {
            gate: "ProvenanceGate".into(),
            reason: "Locator + Hash + Lizenzstatus vorhanden".into(),
        }
    }
}

/// Deduplikation ueber die kanonische Klasse.
pub fn dedup(csus: Vec<Csu>) -> Vec<Csu> {
    let mut seen = std::collections::BTreeSet::new();
    csus.into_iter()
        .filter(|c| seen.insert(c.canonical_class()))
        .collect()
}
