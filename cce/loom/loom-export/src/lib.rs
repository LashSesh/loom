//! loom-export — ExportBundle mit ExportGate: Lizenz-/Attribution-
//! Transport (LOOM Teil 5.4; Port: cce-materialize). Quarantaene hat
//! KEINEN Exportpfad (Teil 9.4).

use loom_verify::{verify, Verdict};

#[derive(Debug)]
pub struct ExportBundle {
    pub profile: String,
    pub core_root_hex: String,
    pub attribution: Vec<String>,
    pub license_summary: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ExportError {
    QuarantineNoExport,
    RejectNoExport,
    AttributionMissing,
}

/// export(handle, sink_profile): nur valid/valid_with_residues;
/// cc-by-* verlangt transportierte Attribution (PROD-INV-16).
pub fn export(
    bytes: &[u8],
    profile: &str,
    core_root_hex: &str,
    license_summary: &str,
    attribution: Vec<String>,
) -> Result<ExportBundle, ExportError> {
    let report = verify(bytes);
    match report.verdict {
        Verdict::Reject => return Err(ExportError::RejectNoExport),
        Verdict::Quarantine => return Err(ExportError::QuarantineNoExport),
        _ => {}
    }
    if license_summary.starts_with("cc-by") && attribution.is_empty() {
        return Err(ExportError::AttributionMissing);
    }
    Ok(ExportBundle {
        profile: profile.to_string(),
        core_root_hex: core_root_hex.to_string(),
        attribution,
        license_summary: license_summary.to_string(),
    })
}
