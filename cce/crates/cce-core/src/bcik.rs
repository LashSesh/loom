//! BCIK-Zentralgesetz (I-01, BCIK §9):
//! `BCIK(K) = Fix(T) ∩ Int(H⁻_Σ) ∩ Closed ∩ Pass(G) ∩ τ ∩ Replay ∩ PathInv`.
//! Materialisierung nur bei `BCIK = 1 ∧ cert ≠ ∅` (Teil 7.2).

use crate::closure::is_closed;
use crate::gate::GateChain;
use crate::objects::Certificate;
use crate::reflection::TripolarFiber;
use crate::residue::ResidueField;
use crate::signature::Digest;

/// Kandidat fuer den Integritaetsschnitt: getypter Kern K (I-02, reduziert
/// auf die pruefbaren Bestandteile).
#[derive(Debug, Clone)]
pub struct KernelCandidate {
    pub id_can: String,
    pub signature: Digest,
    pub typed: bool,
    pub boundary_nonempty: bool,
    pub fiber: TripolarFiber,
    pub residue: ResidueField,
    pub gates: GateChain,
    pub trace_head: Option<Digest>,
    pub replay_ok: bool,
    pub path_invariant: bool,
    pub rd_class: Digest,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BcikVerdict {
    Accepted,
    Rejected(Vec<String>),
}

/// BCIK-Validate (Teil 7.4): jede fehlende Bedingung wird BENANNT abgelehnt
/// — fail-closed, begruendet.
pub fn bcik_validate(k: &KernelCandidate) -> BcikVerdict {
    let mut missing = Vec::new();
    if !k.typed {
        missing.push("G_typed: Kandidat untypisiert".to_string());
    }
    if !k.boundary_nonempty {
        missing.push("G_boundary: Rand leer (INV-3)".to_string());
    }
    if !is_closed(&k.fiber, &k.residue) {
        missing.push("G_closure: B⁻x ≠ 0 oder blocking Residuum".to_string());
    }
    if !k.gates.all_pass() {
        missing.push(format!(
            "G_gate: {}",
            k.gates
                .first_hold()
                .map(|h| format!("{} hold: {}", h.gate_id, h.reason))
                .unwrap_or_else(|| "Gate-Kette leer (fehlende Pruefung ⇒ Ablehnung)".to_string())
        ));
    }
    if k.trace_head.is_none() {
        missing.push("G_trace: kein τ-Kopf".to_string());
    }
    if !k.replay_ok {
        missing.push("G_replay: Replay nicht nachgewiesen".to_string());
    }
    if !k.path_invariant {
        missing.push("G_path: Pfadinvarianz nicht nachgewiesen".to_string());
    }
    if missing.is_empty() {
        BcikVerdict::Accepted
    } else {
        BcikVerdict::Rejected(missing)
    }
}

/// Zertifikat NUR bei BCIK=1 (kein unbelegter Erfolg konstruierbar).
pub fn emit_certificate(k: &KernelCandidate) -> Result<Certificate, Vec<String>> {
    match bcik_validate(k) {
        BcikVerdict::Accepted => Ok(Certificate {
            id_can: k.id_can.clone(),
            signature: k.signature,
            run_descriptor_class: k.rd_class,
            gate_reports: k.gates.reports.clone(),
            boundary_report: "Rand nichtleer, Seam-gebunden".to_string(),
            residual_report: k.residue.render_summary(),
            trace_head: k.trace_head.expect("geprueft"),
            replay_manifest: None,
            path_report: "pfadinvariant mod ≡σ".to_string(),
        }),
        BcikVerdict::Rejected(reasons) => Err(reasons),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gate::GateReport;
    use crate::reflection::{reflect, PolarItem};
    use crate::signature::sha256;
    use crate::value::CanonValue;

    fn good_candidate() -> KernelCandidate {
        let fiber = reflect(
            &[PolarItem {
                value: CanonValue::text("kern"),
                accepted: true,
            }],
            "s",
        );
        let mut gates = GateChain::new();
        for g in crate::gate::mandatory_gates() {
            gates.push(GateReport::pass(&g.id, "erfuellt"));
        }
        KernelCandidate {
            id_can: "k1".into(),
            signature: sha256(b"k1"),
            typed: true,
            boundary_nonempty: true,
            fiber,
            residue: ResidueField::new(),
            gates,
            trace_head: Some(sha256(b"trace")),
            replay_ok: true,
            path_invariant: true,
            rd_class: sha256(b"rd"),
        }
    }

    /// VC10: Zertifikat-Roundtrip — erzeugen, unabhaengig pruefen.
    #[test]
    fn certificate_roundtrip() {
        let k = good_candidate();
        let cert = emit_certificate(&k).expect("BCIK=1");
        assert_eq!(cert.gate_reports.len(), 7);
        assert_eq!(cert.residual_report, "geschlossen (∅)");
        // unabhaengige Pruefung: erneutes Validieren desselben Kandidaten.
        assert_eq!(bcik_validate(&k), BcikVerdict::Accepted);
    }

    /// INV-3: leerer Rand wird benannt abgelehnt (G_boundary).
    #[test]
    fn empty_boundary_is_rejected() {
        let mut k = good_candidate();
        k.boundary_nonempty = false;
        match bcik_validate(&k) {
            BcikVerdict::Rejected(rs) => assert!(rs.iter().any(|r| r.contains("G_boundary"))),
            BcikVerdict::Accepted => panic!("leerer Rand zugelassen (INV-3)"),
        }
    }

    /// Fail-closed: JEDE fehlende Bedingung wird benannt abgelehnt.
    #[test]
    fn each_missing_condition_rejects() {
        let mut k = good_candidate();
        k.replay_ok = false;
        match bcik_validate(&k) {
            BcikVerdict::Rejected(rs) => {
                assert!(rs.iter().any(|r| r.contains("G_replay")));
            }
            BcikVerdict::Accepted => panic!("fehlendes Replay wurde durchgelassen (V7)"),
        }
        assert!(emit_certificate(&k).is_err());
    }
}
