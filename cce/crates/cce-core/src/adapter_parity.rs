//! `check_adapter_parity`-Geruest (Systemlandkarte §5, S1.8):
//! Jede Domaene stellt die IDENTISCHE, vollstaendige Teileliste (11 Punkte)
//! bereit. Fehlt ein Teil ⇒ rot (Produkt unfertig). In G1 als Geruest ueber
//! einem Deskriptor; die typstarke Trait-Form folgt in G3 (cce-materialize).

use crate::gate::GateReport;

/// Die 11 Vertragspunkte (S1.8), maschinenlesbar.
pub const ADAPTER_CONTRACT_POINTS: [&str; 11] = [
    "wish_schema+validate_wish",
    "to_canonical",
    "encode",
    "loom",
    "materialize",
    "reanalyze",
    "canonicalize+equivalent",
    "domain_gates",
    "residue_vocabulary+counter_horizon",
    "native_open+export_formats",
    "reference_cube+negative_cubes",
];

/// Deskriptor eines Domaenen-Adapters: welche Punkte er bereitstellt und
/// mit welchem Inhaltsumfang.
#[derive(Debug, Clone, Default)]
pub struct AdapterParityDescriptor {
    pub domain_id: String,
    pub provided_points: Vec<String>,
    pub domain_gates: Vec<String>,
    pub residue_vocabulary: Vec<String>,
    pub export_formats: Vec<String>,
    pub has_reference_cube: bool,
    pub negative_cube_count: usize,
}

/// check_adapter_parity: boolesch, begruendet, fail-closed.
pub fn check_adapter_parity(d: &AdapterParityDescriptor) -> GateReport {
    let mut missing: Vec<String> = ADAPTER_CONTRACT_POINTS
        .iter()
        .filter(|p| !d.provided_points.iter().any(|q| q == *p))
        .map(|p| (*p).to_string())
        .collect();
    if d.domain_gates.is_empty() {
        missing.push("domain_gates: leer".into());
    }
    if d.residue_vocabulary.is_empty() {
        missing.push("residue_vocabulary: leer".into());
    }
    if d.export_formats.is_empty() {
        missing.push("export_formats: leer".into());
    }
    if !d.has_reference_cube {
        missing.push("reference_cube: fehlt".into());
    }
    if d.negative_cube_count == 0 {
        missing.push("negative_cubes: fehlen".into());
    }
    if missing.is_empty() {
        GateReport::pass(
            "check_adapter_parity",
            &format!(
                "Domaene {} erfuellt die kanonische Teileliste (11/11)",
                d.domain_id
            ),
        )
    } else {
        GateReport::hold(
            "check_adapter_parity",
            &format!(
                "Domaene {} unvollstaendig: {}",
                d.domain_id,
                missing.join(", ")
            ),
        )
    }
}

/// Vollstaendiger Deskriptor als Testhilfe.
pub fn full_descriptor(domain_id: &str) -> AdapterParityDescriptor {
    AdapterParityDescriptor {
        domain_id: domain_id.to_string(),
        provided_points: ADAPTER_CONTRACT_POINTS
            .iter()
            .map(|s| s.to_string())
            .collect(),
        domain_gates: vec!["DocG-RoundTrip".into()],
        residue_vocabulary: vec!["semantic_loss".into()],
        export_formats: vec![".md".into()],
        has_reference_cube: true,
        negative_cube_count: 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// G1-Gate: check_adapter_parity-Geruest vorhanden und fail-closed.
    #[test]
    fn parity_full_is_green_missing_is_red() {
        let full = full_descriptor("document");
        assert!(check_adapter_parity(&full).is_pass());

        let mut incomplete = full_descriptor("graph");
        incomplete.provided_points.retain(|p| p != "reanalyze");
        let r = check_adapter_parity(&incomplete);
        assert!(!r.is_pass());
        assert!(r.reason.contains("reanalyze"));
    }
}
