//! loom-viewer — Referenz-Viewer (CLI), L0–L2 OHNE Projektwissen und
//! OHNE Motor (LOOM Teil 6). Keine Capabilities, keine Netzaktivitaet,
//! kein Run, kein Commit. Die fuenf Pflichtansichten (Minimal-GUI-
//! Inspektionspfad) werden headless aus denselben Reportdaten gerendert.

use loom_mount::{inspect, open};

/// Rendert die fuenf Pflichtansichten als Text (headless):
/// 1 Manifest · 2 Segmentliste+Digest-Status · 3 Residuen+Verdikt ·
/// 4 Gate-Reports · 5 Ledger/PhaseBlocks.
pub fn render_views(bytes: &[u8]) -> Result<Vec<String>, String> {
    let handle = open(bytes).map_err(|e| format!("{e:?}"))?;
    let report = inspect(bytes, &handle);
    let manifest_view = format!(
        "MANIFEST: {} Segmente, core_root {}",
        report.segment_count,
        &report.core_root_hex[..16]
    );
    let seg_view = format!(
        "SEGMENTE: kinds {:?} (alle Digests am Frame geprueft)",
        report.segment_kinds
    );
    let residue_view = format!(
        "RESIDUEN: {} sichtbar · Verdikt {:?}",
        report.residue_count, report.verdict
    );
    let gates_view = format!(
        "GATES: {} Diagnosen: {:?}",
        report.diagnoses.len(),
        report.diagnoses
    );
    let ledger_view = "LEDGER: siehe Segment 0x0013 (read-only)".to_string();
    Ok(vec![
        manifest_view,
        seg_view,
        residue_view,
        gates_view,
        ledger_view,
    ])
}
