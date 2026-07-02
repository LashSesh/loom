//! PhaseBlock: das 10-Tupel (Strukturpause via Rebase §1.1).
//! Felder: (id, scale, phase, inputs, payload_digest, gate_reports,
//! evidence_refs, residue_field, rd_ref, parent_refs) — Interpretations-
//! zuordnung dokumentiert in reports/G02_bericht.md (R-Agent-3).

use cce_core::canonical::Canonicalize;
use cce_core::gate::GateReport;
use cce_core::replay::HitlDecision;
use cce_core::residue::ResidueField;
use cce_core::signature::Digest;
use cce_core::value::CanonValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockStatus {
    /// Kandidat: inhaltsadressiert, wiederaufnahmefaehig — NICHT verbucht.
    Candidate,
    /// Hold: sichtbar abgelehnt/wartend mit Residuum.
    Hold,
    /// Accepted: irreversibler Abschluss (nur via Accept-8).
    Accepted,
}

/// Das 10-Tupel.
#[derive(Debug, Clone)]
pub struct PhaseBlock {
    /// 1. Kennung (content-adressiert nach Payload).
    pub id: String,
    /// 2. Skala (SCALE-n).
    pub scale: u8,
    /// 3. Phase innerhalb der Skala.
    pub phase: String,
    /// 4. Inputs (inkl. aufgezeichneter HITL-Entscheidungen, S5-A2).
    pub inputs: Vec<HitlDecision>,
    /// 5. Payload-Digest (kanonische Klasse des Blockinhalts).
    pub payload_digest: Digest,
    /// 6. Gate-Reports (boolesch, begruendet).
    pub gate_reports: Vec<GateReport>,
    /// 7. Evidence-Referenzen (MEF-Digests).
    pub evidence_refs: Vec<Digest>,
    /// 8. Residuenfeld (STETS ausgewiesen).
    pub residue_field: ResidueField,
    /// 9. RD-Referenz (Replay-Vertrag).
    pub rd_ref: Digest,
    /// 10. Eltern-Referenzen (kausale Ordnung).
    pub parent_refs: Vec<String>,
    pub status: BlockStatus,
}

impl PhaseBlock {
    /// Erzeugt einen KANDIDATEN (nie direkt einen akzeptierten Block).
    #[allow(clippy::too_many_arguments)]
    pub fn candidate(
        scale: u8,
        phase: &str,
        payload: &CanonValue,
        inputs: Vec<HitlDecision>,
        gate_reports: Vec<GateReport>,
        evidence_refs: Vec<Digest>,
        residue_field: ResidueField,
        rd_ref: Digest,
        parent_refs: Vec<String>,
    ) -> Self {
        let payload_digest = payload.canonical_class().0;
        Self {
            id: format!("pb:{payload_digest}"),
            scale,
            phase: phase.to_string(),
            inputs,
            payload_digest,
            gate_reports,
            evidence_refs,
            residue_field,
            rd_ref,
            parent_refs,
            status: BlockStatus::Candidate,
        }
    }
}
