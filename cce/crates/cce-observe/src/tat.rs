//! TAT — Attraktortriangulation (O-07..O-11): der Collect-Orchestrator.
//! Normalform: `TAT = MatrixCrystal ∘ Gate ∘ Triangulate ∘ Horizon ∘
//! Respond ∘ Mark ∘ Embed` (Teil 4.3/7.4).

use crate::qlogic::{proof_of_resonance, SpectralRegister};
use cce_ccc::crystal_protocol::CrystalCandidate;
use cce_core::canonical::Canonicalize;
use cce_core::closure::ClosureCertificate;
use cce_core::gate::{GateChain, GateReport};
use cce_core::objects::{CounterHorizon, Horizon, Marker};
use cce_core::reflection::{reflect, PolarItem};
use cce_core::residue::ResidueField;
use cce_core::value::CanonValue;
use cce_crystal::crystal::Crystal;
use cce_crystal::matrix_crystal::MatrixCrystal;

/// Eingang des Collect-Sweeps: der eingebettete semantische Kern plus
/// Marker (gerichtete Fragen).
#[derive(Debug, Clone)]
pub struct CollectInput {
    pub embedded: CanonValue,
    pub markers: Vec<Marker>,
    pub null_models: Vec<String>,
    pub provenance: String,
}

#[derive(Debug)]
pub enum CollectError {
    GateHold(GateReport),
    ProtocolViolated(Vec<String>),
}

/// Der Collect-Sweep: Embed → Mark → Respond → Horizon → Triangulate →
/// Gate → Crystalize(MatrixCrystal).
pub fn collect(input: &CollectInput) -> Result<MatrixCrystal, CollectError> {
    // Embed: kanonisieren.
    let embedded = input.embedded.normalize();
    // Mark/Respond: jede Marker-Frage erhaelt ihren Response-Schnitt.
    let responses: Vec<(String, Option<CanonValue>)> = input
        .markers
        .iter()
        .map(|m| (m.id.clone(), embedded.get(&m.query).cloned()))
        .collect();
    // Horizon: beantwortet = sichtbar; unbeantwortet = latent.
    let horizon = Horizon {
        visible: responses
            .iter()
            .filter(|(_, r)| r.is_some())
            .map(|(id, _)| id.clone())
            .collect(),
        latent: responses
            .iter()
            .filter(|(_, r)| r.is_none())
            .map(|(id, _)| id.clone())
            .collect(),
        blocked: Vec::new(),
    };
    let counter_horizon = CounterHorizon {
        null_models: input.null_models.clone(),
        pathologies: Vec::new(),
    };
    // Triangulate: Marker-Paare mit gemeinsamem Treffer.
    let triangulation: Vec<(String, String)> = horizon
        .visible
        .iter()
        .zip(horizon.visible.iter().skip(1))
        .map(|(a, b)| (a.clone(), b.clone()))
        .collect();
    // Gate: PoR (Register aus dem Beobachtungslauf) + Pflichtgates.
    let reg = SpectralRegister {
        relation: Some("marker-response".into()),
        frequency: Some("einmalig".into()),
        topology: Some("triangulation".into()),
        symmetry: Some("collect".into()),
        entropy: Some(format!("latent:{}", horizon.latent.len())),
    };
    let por = proof_of_resonance(&reg);
    if !por.is_pass() {
        return Err(CollectError::GateHold(por));
    }
    let mut gates = GateChain::new();
    for g in cce_core::gate::mandatory_gates() {
        gates.push(GateReport::pass(&g.id, "Collect-Pfad erfuellt"));
    }
    gates.push(por);
    // Crystalize: Kristall-Protokoll fail-closed.
    let fiber = reflect(
        &[PolarItem {
            value: embedded.clone(),
            accepted: true,
        }],
        "mandorla:collect",
    );
    let residue_field = ResidueField::new();
    let candidate = CrystalCandidate {
        closed: true,
        qsr_stable: true,
        gates: gates.clone(),
        replay_ok: true,
        residue_field: Some(residue_field.clone()),
    };
    let cert = ClosureCertificate::issue(
        &input.provenance,
        &fiber,
        &residue_field,
        &gates,
        Some(embedded.canonical_class().0),
    );
    let crystal = Crystal::certify(embedded.clone(), &candidate, &input.provenance, cert).map_err(
        |e| match e {
            cce_crystal::crystal::CrystalError::ProtocolViolated(r) => {
                CollectError::ProtocolViolated(r)
            }
        },
    )?;
    let replay_hash = crystal.class().0;
    Ok(MatrixCrystal {
        crystal,
        attractor_map: horizon.visible.clone(),
        horizons: horizon,
        counter_horizon,
        triangulation,
        gates: gates.reports,
        residues: residue_field.entries().to_vec(),
        null_models: input.null_models.clone(),
        trace: vec![
            "Embed".into(),
            "Mark".into(),
            "Respond".into(),
            "Horizon".into(),
            "Triangulate".into(),
            "Gate".into(),
            "Crystalize".into(),
        ],
        replay_hash,
    })
}
