//! S15-Strukturtypen (DoD-Matrix f): ScaleAdapter-Vertrag, WorkbenchCapsule,
//! Operational Multicube, MultiScaleClosure — STRUKTUREN gebaut; Skalenreife
//! jenseits SCALE-1 ist PL-gefuehrtes Residuum (S15-R1/R-10), NICHT behauptet.

use crate::bluered::{close_red, RedCube, RedVerdict};
use cce_core::gate::GateReport;
use cce_core::signature::Digest;

/// ScaleAdapter<s> (S15.1): das skalenachsige Gegenstueck zum DomainAdapter.
#[derive(Debug, Clone)]
pub struct ScaleAdapter {
    pub scale: u8,
    pub cell_type: String,
    pub phase_set: Vec<String>,
    pub close_blue_rule: String,
    pub close_red_rule: String,
    pub wrap_policy_desc: String,
    pub promotion_gate: String,
    pub residue_vocab: Vec<String>,
    pub replay_contract: String,
}

/// check_scale_adapter_parity (S15.1): jede aktivierte Skala traegt den
/// VOLLEN Vertrag.
pub fn check_scale_adapter_parity(a: &ScaleAdapter) -> GateReport {
    let mut missing = Vec::new();
    if a.cell_type.is_empty() {
        missing.push("CellType");
    }
    if a.phase_set.is_empty() {
        missing.push("PhaseSet");
    }
    if a.close_blue_rule.is_empty() {
        missing.push("CloseBlue");
    }
    if a.close_red_rule.is_empty() {
        missing.push("CloseRed");
    }
    if a.wrap_policy_desc.is_empty() {
        missing.push("WrapPolicy");
    }
    if a.promotion_gate.is_empty() {
        missing.push("PromotionGate");
    }
    if a.residue_vocab.is_empty() {
        missing.push("ResidueVocab");
    }
    if a.replay_contract.is_empty() {
        missing.push("ReplayContract");
    }
    if missing.is_empty() {
        GateReport::pass(
            "check_scale_adapter_parity",
            &format!("SCALE-{} vollstaendig (8/8 Slots)", a.scale),
        )
    } else {
        GateReport::hold(
            "check_scale_adapter_parity",
            &format!("SCALE-{} unvollstaendig: {missing:?}", a.scale),
        )
    }
}

/// Der SCALE-1-Adapter (aus S1–S13 ableitbar, S15-R2).
pub fn scale1_adapter() -> ScaleAdapter {
    ScaleAdapter {
        scale: 1,
        cell_type: "Artefakt".to_string(),
        phase_set: vec![
            "wuenschen".into(),
            "bestaetigen".into(),
            "laufen".into(),
            "pruefen".into(),
            "entnehmen".into(),
            "ablegen".into(),
        ],
        close_blue_rule: "Typed ∧ BoundaryValid ∧ Gate=Pass ∧ Evidence ∧ Replay ∧ WrapStable"
            .into(),
        close_red_rule: "∀p Close(Blue) ∧ SeamsValid ∧ Replay ∧ Resid sichtbar".into(),
        wrap_policy_desc: "B_1 = Dokument-Boundary; DispersionProfile dyadic (RD)".into(),
        promotion_gate: "ScaleRatchetGate".into(),
        residue_vocab: crate::residues::ALL_SPIRAL_RESIDUES
            .iter()
            .map(|s| s.to_string())
            .collect(),
        replay_contract: "rd-1".into(),
    }
}

/// WorkbenchCapsule (S15.3): scope-gebunden, budgetiert, ttl-begrenzt,
/// dissolution-pflichtig — nur Trace/Evidence/Kristalle persistieren.
#[derive(Debug, Clone)]
pub struct Capsule {
    pub scope: String,
    pub budget: u64,
    pub projection: String,
    pub allowed_ops: Vec<String>,
    pub gates: Vec<String>,
    pub evidence_schema: String,
    pub ttl: u32,
    pub ledger_ref: String,
}

/// Dissolution: liefert NUR das Persistierungswuerdige zurueck.
#[derive(Debug, Clone)]
pub struct DissolutionResult {
    pub trace: Vec<String>,
    pub evidence_refs: Vec<Digest>,
    pub crystal_classes: Vec<Digest>,
}

impl Capsule {
    pub fn dissolve(
        self,
        trace: Vec<String>,
        evidence_refs: Vec<Digest>,
        crystal_classes: Vec<Digest>,
    ) -> DissolutionResult {
        // Die Capsule selbst verschwindet (self wird konsumiert) —
        // ephemere Suchpfade persistieren NICHT (HBM-Axiom 5.4).
        DissolutionResult {
            trace,
            evidence_refs,
            crystal_classes,
        }
    }
}

/// Operational Multicube (S15.2): die Werkbank-Sicht.
#[derive(Debug, Default)]
pub struct Multicube {
    pub reds: Vec<RedCube>,
    pub capsules_active: usize,
}

/// MultiScaleClosure(s*) (S15.13): ∀ s ≤ s*: Close(Red_s) — strukturell.
/// SCALE-1-Instanz = Produkt-Kerntest; hoehere s* PL-gefuehrt (S15-R1).
pub fn multi_scale_closure(mc: &Multicube, target: u8) -> Result<(), Vec<String>> {
    let mut open = Vec::new();
    for s in 0..=target {
        match mc.reds.iter().find(|r| r.scale == s) {
            None => open.push(format!("SCALE-{s}: kein RedCube vorhanden")),
            Some(r) => {
                if let RedVerdict::Open(rs) = close_red(r) {
                    open.push(format!(
                        "SCALE-{s}: offen ({})",
                        rs.iter()
                            .map(|x| x.kind.as_str().to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ));
                }
            }
        }
    }
    if open.is_empty() {
        Ok(())
    } else {
        Err(open)
    }
}
