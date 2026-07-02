//! nexus-evidence — P_π: Provenienzbindung + EvidencePack (CSA.3/CSA.4).
//! 11/15 EvidenceGate: KEIN Export, kein BlueprintCandidate, kein Crystal
//! ohne EvidencePack (PROD-INV-15); Attribution wird transportiert, wo die
//! Lizenz sie fordert (PROD-INV-16).

use cce_core::canonical::Canonicalize;
use nexus_core::objects::{Csu, EvidencePack, RawObservation};
use nexus_core::residues::csa_residue;
use nexus_core::verdict::Verdict;

/// Baut das EvidencePack einer CSU aus ihrem Abruf.
pub fn build_pack(
    csu: &Csu,
    raw: &RawObservation,
    transform_path: &[&str],
    attribution: Option<&str>,
) -> EvidencePack {
    EvidencePack {
        evidence_id: format!("ep:{}", csu.uid),
        record_id: csu.uid.clone(),
        source: raw.fetched_via.clone(),
        locator: raw.locator.clone(),
        transform_path: transform_path.iter().map(|s| s.to_string()).collect(),
        policy_snapshot: "policy-1".to_string(),
        license: csu.license.clone(),
        attribution: attribution.map(|s| s.to_string()),
        raw_hash: raw.digest(),
        csu_class: csu.canonical_class().0,
    }
}

/// 11/15 EvidenceGate: jede exportierte CSU besitzt ein EP; Attribution-
/// Pflicht wird durchgesetzt (PROD-INV-16).
pub fn evidence_gate(csu: &Csu, pack: Option<&EvidencePack>) -> Verdict {
    match pack {
        None => Verdict::Hold {
            gate: "EvidenceGate".into(),
            residue: Box::new(csa_residue(
                "evidence_missing",
                &format!(
                    "CSU {} ohne EvidencePack — kein Export (PROD-INV-15)",
                    csu.uid
                ),
            )),
        },
        Some(ep) => {
            if csu.license.starts_with("cc-by") && ep.attribution.is_none() {
                return Verdict::Hold {
                    gate: "EvidenceGate".into(),
                    residue: Box::new(csa_residue(
                        "license_attribution_required",
                        &format!(
                            "CSU {}: {} verlangt Attribution im Evidence-/Exportpfad",
                            csu.uid, csu.license
                        ),
                    )),
                };
            }
            Verdict::Allow {
                gate: "EvidenceGate".into(),
                reason: "EvidencePack vollstaendig, Attribution transportiert".into(),
            }
        }
    }
}
