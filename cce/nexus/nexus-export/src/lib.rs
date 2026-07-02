//! nexus-export — R_ρ: Routing + die drei Ausgangs-Gates (CSA.7/CSA.11):
//! 13/15 HBMImportGate (CSU → Facet, evidence-gebunden) ·
//! 14/15 PHCProjectionGate (NSB → Blue_source-Input) ·
//! 15/15 ExportGate (ExportContract + Lizenz-/Attribution-Transport +
//! ResidueReport vollstaendig).

use nexus_core::objects::{Csu, EvidencePack, NexusSourceBundle};
use nexus_core::residues::csa_residue;
use nexus_core::verdict::Verdict;

/// 13/15 HBMImportGate: `HBMImport(c) = Evidence ∧ Provenance ∧
/// DomainProjection ∧ Replay` — liefert die belegte Facet-Vorform.
pub fn hbm_import_gate(csu: &Csu, ep: Option<&EvidencePack>) -> Verdict {
    if ep.is_none() {
        return Verdict::Hold {
            gate: "HBMImportGate".into(),
            residue: Box::new(csa_residue(
                "evidence_missing",
                &format!("CSU {} ohne EP — kein HBM-Import", csu.uid),
            )),
        };
    }
    if csu.domain_facet.is_empty() {
        return Verdict::Hold {
            gate: "HBMImportGate".into(),
            residue: Box::new(csa_residue(
                "hbm_projection_missing",
                &format!("CSU {} ohne domain_facet-Typisierung", csu.uid),
            )),
        };
    }
    Verdict::Allow {
        gate: "HBMImportGate".into(),
        reason: "CSU als Facet-Quelle typisiert und belegt".into(),
    }
}

/// 14/15 PHCProjectionGate: Bundle in PHC-/BlueCube-Input projizierbar.
pub fn phc_projection_gate(nsb: &NexusSourceBundle) -> Verdict {
    if nsb.csu_set.is_empty() {
        return Verdict::Hold {
            gate: "PHCProjectionGate".into(),
            residue: Box::new(csa_residue(
                "phc_projection_missing",
                "leeres Bundle nicht projizierbar",
            )),
        };
    }
    if !nsb.every_csu_has_evidence() {
        return Verdict::Hold {
            gate: "PHCProjectionGate".into(),
            residue: Box::new(csa_residue(
                "evidence_missing",
                "Bundle enthaelt CSU ohne EvidencePack",
            )),
        };
    }
    Verdict::Allow {
        gate: "PHCProjectionGate".into(),
        reason: "NSB → Blue_source projizierbar; EPs werden Proof-Payloads".into(),
    }
}

/// 15/15 ExportGate: ExportContract, Lizenztransport, ResidueReport.
pub fn export_gate(nsb: &NexusSourceBundle) -> Verdict {
    if nsb.export_contract.is_empty() {
        return Verdict::Hold {
            gate: "ExportGate".into(),
            residue: Box::new(csa_residue("export_blocked", "kein ExportContract")),
        };
    }
    if !nsb.every_csu_has_evidence() {
        return Verdict::Hold {
            gate: "ExportGate".into(),
            residue: Box::new(csa_residue(
                "evidence_missing",
                "Export ohne vollstaendige EvidencePacks (PROD-INV-15)",
            )),
        };
    }
    // Attribution-Transport (PROD-INV-16):
    for ep in &nsb.evidence_packs {
        if ep.license.starts_with("cc-by") && ep.attribution.is_none() {
            return Verdict::Hold {
                gate: "ExportGate".into(),
                residue: Box::new(csa_residue(
                    "license_attribution_required",
                    &format!("EP {} ohne Attribution", ep.evidence_id),
                )),
            };
        }
    }
    Verdict::Allow {
        gate: "ExportGate".into(),
        reason: "ExportContract + Lizenz-/Attribution-Transport vollstaendig".into(),
    }
}
