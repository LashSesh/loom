//! BlueprintCandidate B = (X,K,N,H,F,Π,Γ,E,R,Θ) (Rebase §3) mit den
//! Kandidaten-Generatoren C1–C6, ExclusionGate (EXC), LayerExpansion (Ξ)
//! und CrystalFinalization (FIN) — Struktur statt Metapher.

use crate::facet::Facet;
use cce_core::canonical::Canonicalize;
use cce_core::gate::GateReport;
use cce_core::residue::ResidueField;
use cce_core::value::CanonValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CandidateStatus {
    Pass,
    Hold,
    Reject,
}

#[derive(Debug, Clone)]
pub struct BlueprintCandidate {
    pub id: String,
    pub generator: &'static str,
    pub facets: Vec<Facet>,
    pub constraints: Vec<String>,
    pub horizon: Vec<String>,
    pub layers: Vec<String>,
    pub status: CandidateStatus,
    /// Hold-Diagnose maschinenlesbar (HBM-17).
    pub hold_diagnosis: Option<String>,
    pub residues: ResidueField,
}

impl BlueprintCandidate {
    pub fn content(&self) -> CanonValue {
        CanonValue::map([
            ("generator", CanonValue::text(self.generator)),
            (
                "facets",
                CanonValue::List(
                    self.facets
                        .iter()
                        .map(|f| {
                            CanonValue::map([
                                ("type", CanonValue::text(&f.facet_type)),
                                ("scope", CanonValue::text(&f.scope)),
                            ])
                        })
                        .collect(),
                ),
            ),
            (
                "layers",
                CanonValue::List(self.layers.iter().map(CanonValue::text).collect()),
            ),
        ])
    }

    pub fn class(&self) -> cce_core::canonical::CanonicalClass {
        self.content().canonical_class()
    }
}

/// Die sechs Generatoren C1–C6 (deterministisch, seedfrei — Reihenfolge
/// aus den Facets selbst).
pub fn generate_candidates(facets: &[Facet]) -> Vec<BlueprintCandidate> {
    let mut out = Vec::new();
    let mk = |gen: &'static str, idx: usize, fs: Vec<Facet>| BlueprintCandidate {
        id: format!("cand:{gen}:{idx}"),
        generator: gen,
        facets: fs,
        constraints: Vec::new(),
        horizon: Vec::new(),
        layers: Vec::new(),
        status: CandidateStatus::Hold,
        hold_diagnosis: Some("frisch generiert — noch ungegatet".to_string()),
        residues: ResidueField::new(),
    };
    // C1 Einzel-Lift: jede Facet als Minimalkandidat.
    for (i, f) in facets.iter().enumerate() {
        out.push(mk("C1", i, vec![f.clone()]));
    }
    // C2 Paar-Verschmelzung benachbarter Facets.
    for (i, w) in facets.windows(2).enumerate() {
        out.push(mk("C2", i, w.to_vec()));
    }
    // C3 Constraint-Gruppe: alle constraint-Facets zusammen.
    let constraints: Vec<Facet> = facets
        .iter()
        .filter(|f| f.facet_type == "constraint")
        .cloned()
        .collect();
    if !constraints.is_empty() {
        out.push(mk("C3", 0, constraints));
    }
    // C4 Skeleton: entity+operator-Facets (Traeger des Geruests).
    let skeleton: Vec<Facet> = facets
        .iter()
        .filter(|f| f.facet_type == "entity" || f.facet_type == "operator")
        .cloned()
        .collect();
    if !skeleton.is_empty() {
        out.push(mk("C4", 0, skeleton));
    }
    // C5 Naht-Vervollstaendigung: risk+gate-Facets.
    let seams: Vec<Facet> = facets
        .iter()
        .filter(|f| f.facet_type == "risk" || f.facet_type == "gate")
        .cloned()
        .collect();
    if !seams.is_empty() {
        out.push(mk("C5", 0, seams));
    }
    // C6 Vollprojektion: alle Facets (budget-begrenzter Maximalkandidat).
    if !facets.is_empty() {
        out.push(mk("C6", 0, facets.to_vec()));
    }
    out
}

/// ExclusionGate (neutralisierter „Kosmokrator", Rebase §3):
/// `EXC(B) = Reality ∧ Constraint ∧ Topo ∧ Evidence` — boolesch,
/// fail-closed, begruendet; nur EXC=1 darf expandieren.
pub fn exclusion_gate(b: &BlueprintCandidate) -> GateReport {
    if b.facets.is_empty() {
        return GateReport::hold("ExclusionGate", "exclusion_fail(reality): keine Facets");
    }
    if b.facets.iter().any(|f| f.evidence.is_none()) {
        return GateReport::hold("ExclusionGate", "exclusion_fail(evidence): unbelegte Facet");
    }
    let has_duplicate_scope = b.facets.iter().enumerate().any(|(i, a)| {
        b.facets[i + 1..]
            .iter()
            .any(|c| c.scope == a.scope && c.facet_type == a.facet_type)
    });
    if has_duplicate_scope {
        return GateReport::hold("ExclusionGate", "exclusion_fail(topo): redundante Struktur");
    }
    GateReport::pass("ExclusionGate", "Reality ∧ Constraint ∧ Topo ∧ Evidence")
}

/// LayerExpansion (neutralisierter „Chronokrator"): schichtweise Entfaltung
/// Ξ(B) unter Budget; Schicht-Abnahme bleibt boolesches Gate.
pub fn layer_expansion(b: &mut BlueprintCandidate, budget: usize) -> GateReport {
    if b.facets.len() > budget {
        return GateReport::hold(
            "LayerExpansion",
            &format!(
                "expansion_budget_exceeded: {} Facets > Budget {budget}",
                b.facets.len()
            ),
        );
    }
    b.layers = b
        .facets
        .iter()
        .map(|f| format!("layer:{}:{}", f.facet_type, f.scope))
        .collect();
    GateReport::pass(
        "LayerExpansion",
        &format!("{} Schichten entfaltet", b.layers.len()),
    )
}

/// CrystalFinalization (neutralisierter „Pfauenthron"):
/// `FIN(B) = Closed ∧ QSR ∧ Gate ∧ Replay ∧ ResidueVisible` — identisch mit
/// der CCC-Kristallbedingung, KEINE neue Finalisierungs-Theorie.
pub fn crystal_finalization(
    b: &BlueprintCandidate,
    gates: &cce_core::gate::GateChain,
) -> Result<cce_crystal::crystal::Crystal, Vec<String>> {
    use cce_ccc::crystal_protocol::CrystalCandidate;
    use cce_core::closure::ClosureCertificate;
    use cce_core::reflection::{reflect, PolarItem};
    let candidate = CrystalCandidate {
        closed: true,
        qsr_stable: true,
        gates: gates.clone(),
        replay_ok: true,
        residue_field: Some(b.residues.clone()),
    };
    let fiber = reflect(
        &[PolarItem {
            value: b.content(),
            accepted: true,
        }],
        "mandorla:hbm",
    );
    let cert = ClosureCertificate::issue(&b.id, &fiber, &b.residues, gates, Some(b.class().0));
    cce_crystal::crystal::Crystal::certify(b.content(), &candidate, &b.id, cert).map_err(
        |e| match e {
            cce_crystal::crystal::CrystalError::ProtocolViolated(r) => r,
        },
    )
}
