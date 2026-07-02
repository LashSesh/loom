//! H/F-Import-Gate (I-08, Teil 4.6): `import_F_to_H = Typed ∧ BoundaryDefined
//! ∧ Closed ∧ Pass(G) ∧ τ ∧ Replay ∧ NoOverclaim`. Der Admission-Controller
//! an jeder Systemgrenze — kein Rohimport (V6), Claim-Schranke (V10/INV-14).

use crate::bcik::{bcik_validate, BcikVerdict, KernelCandidate};
use crate::residue::{Residue, ResidueKind, Severity};

/// Verbotene Ueberreichweiten-Behauptungen (BCIK A8, INV-14):
/// Physik-, RH-, π/ζ-als-Operator-Claims.
const OVERCLAIM_MARKERS: &[&str] = &[
    "riemann-hypothese",
    "riemann hypothesis",
    "hilbert-polya",
    "hilbert-pólya",
    "physikalisches gesetz",
    "naturgesetz",
    "pi-als-operator",
    "zeta-als-operator",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImportVerdict {
    /// H-Artefakt: zugelassen.
    Admitted,
    /// ⊥ mit Begruendung + sichtbarem Residuum.
    Rejected(Vec<String>),
}

/// Prueft eine Behauptungsmenge gegen die Claim-Schranke.
pub fn overclaims(claims: &[String]) -> Vec<String> {
    claims
        .iter()
        .filter(|c| {
            let lc = c.to_lowercase();
            OVERCLAIM_MARKERS.iter().any(|m| lc.contains(m))
        })
        .cloned()
        .collect()
}

/// import_F_to_H: Admission-Controller. Reflektor L ist auf H Identitaet
/// (idempotent): ein bereits zugelassener Kandidat passiert unveraendert.
pub fn import_f_to_h(candidate: &KernelCandidate, claims: &[String]) -> ImportVerdict {
    let mut reasons = Vec::new();
    if let BcikVerdict::Rejected(rs) = bcik_validate(candidate) {
        reasons.extend(rs);
    }
    for oc in overclaims(claims) {
        reasons.push(format!("NoOverclaim verletzt (V10/INV-14): \"{oc}\""));
    }
    if reasons.is_empty() {
        ImportVerdict::Admitted
    } else {
        ImportVerdict::Rejected(reasons)
    }
}

/// Rohimport-Residuum (V6): jeder abgewiesene Import ist sichtbar.
pub fn rejection_residue(reasons: &[String]) -> Residue {
    Residue::new(
        "import-rejected",
        "hf_import",
        ResidueKind::ExcludedMaterial,
        Severity::Blocking,
        &reasons.join("; "),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gate::{GateChain, GateReport};
    use crate::reflection::{reflect, PolarItem};
    use crate::residue::ResidueField;
    use crate::signature::sha256;
    use crate::value::CanonValue;

    fn admitted_candidate() -> KernelCandidate {
        let mut gates = GateChain::new();
        for g in crate::gate::mandatory_gates() {
            gates.push(GateReport::pass(&g.id, "ok"));
        }
        KernelCandidate {
            id_can: "f-obj".into(),
            signature: sha256(b"f"),
            typed: true,
            boundary_nonempty: true,
            fiber: reflect(
                &[PolarItem {
                    value: CanonValue::text("ok"),
                    accepted: true,
                }],
                "s",
            ),
            residue: ResidueField::new(),
            gates,
            trace_head: Some(sha256(b"t")),
            replay_ok: true,
            path_invariant: true,
            rd_class: sha256(b"rd"),
        }
    }

    /// VC9: Import nur durch Gate; Rohimport (untypisiert) wird abgewiesen.
    #[test]
    fn raw_import_is_rejected() {
        let mut k = admitted_candidate();
        k.typed = false;
        match import_f_to_h(&k, &[]) {
            ImportVerdict::Rejected(rs) => assert!(rs.iter().any(|r| r.contains("G_typed"))),
            ImportVerdict::Admitted => panic!("Rohimport zugelassen (V6)"),
        }
    }

    /// V10/INV-14: Ueberreichweiten-Claim wird abgewiesen.
    #[test]
    fn overclaim_is_rejected() {
        let k = admitted_candidate();
        let claims = vec!["beweist die Riemann-Hypothese".to_string()];
        match import_f_to_h(&k, &claims) {
            ImportVerdict::Rejected(rs) => {
                assert!(rs.iter().any(|r| r.contains("NoOverclaim")));
                let res = rejection_residue(&rs);
                assert_eq!(res.severity, Severity::Blocking);
            }
            ImportVerdict::Admitted => panic!("Overclaim zugelassen (V10)"),
        }
    }

    /// Reflektor-Idempotenz: zugelassener Kandidat passiert erneut.
    #[test]
    fn reflector_is_idempotent_on_h() {
        let k = admitted_candidate();
        assert_eq!(import_f_to_h(&k, &[]), ImportVerdict::Admitted);
        assert_eq!(import_f_to_h(&k, &[]), ImportVerdict::Admitted);
    }
}
