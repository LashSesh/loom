//! loom-mount — Mount-Vertrag (LOOM-Standard Teil 5.3):
//! inspect · mount-ro · run · workbench · repair/quarantine.
//! Oeffnen ist reine Verifikation + Deserialisierung — KEIN Hook,
//! KEIN Autostart, KEIN Modell-/Tool-Egress (Oeffnungs-Haertung,
//! Overlay 05 Teil E).

use loom_codec::Decoded;
use loom_verify::{verify, verify_l0, Verdict, VerificationReport};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MountMode {
    Inspect,
    MountRo,
    Run,
    Workbench,
    RepairQuarantine,
}

/// M1–M2: open — Praeambel/Footer/Index, KEINE Ausfuehrung, keine
/// Seiteneffekte (der Handle haelt nur geparste Strukturen).
pub struct LoomHandle {
    pub decoded: Decoded,
    pub bytes_len: usize,
}

#[derive(Debug)]
pub enum OpenError {
    Structure(Vec<String>),
}

pub fn open(bytes: &[u8]) -> Result<LoomHandle, OpenError> {
    let decoded = verify_l0(bytes).map_err(|ds| {
        OpenError::Structure(
            ds.into_iter()
                .map(|d| format!("{}: {}", d.point, d.detail))
                .collect(),
        )
    })?;
    Ok(LoomHandle {
        decoded,
        bytes_len: bytes.len(),
    })
}

/// InspectionReport (M5-Sicht) — read-only Zusammenfassung.
#[derive(Debug)]
pub struct InspectionReport {
    pub segment_kinds: Vec<u16>,
    pub segment_count: usize,
    pub core_root_hex: String,
    pub verdict: Verdict,
    pub residue_count: u64,
    pub diagnoses: Vec<String>,
}

pub fn inspect(bytes: &[u8], handle: &LoomHandle) -> InspectionReport {
    let report: VerificationReport = verify(bytes);
    InspectionReport {
        segment_kinds: handle.decoded.segtab.iter().map(|e| e.kind).collect(),
        segment_count: handle.decoded.segtab.len(),
        core_root_hex: handle
            .decoded
            .footer
            .core_root
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect(),
        verdict: report.verdict,
        residue_count: report.residue_count,
        diagnoses: report
            .diagnoses
            .iter()
            .map(|d| format!("[{}] {}: {}", d.level, d.point, d.detail))
            .collect(),
    }
}

/// Gemounteter Arbeitskoerper: traegt Modus + Verdikt; Quarantaene ist
/// inspizierbar, aber nicht export-/run-/importfaehig (Teil 9.4).
pub struct MountedWorkbody {
    pub mode: MountMode,
    pub verdict: Verdict,
}

#[derive(Debug, PartialEq, Eq)]
pub enum MountError {
    QuarantinedNoRun,
    RejectedNoMount,
}

pub fn mount(bytes: &[u8], mode: MountMode) -> Result<MountedWorkbody, MountError> {
    let report = verify(bytes);
    match report.verdict {
        Verdict::Reject => Err(MountError::RejectedNoMount),
        Verdict::Quarantine => {
            if matches!(mode, MountMode::Inspect | MountMode::RepairQuarantine) {
                Ok(MountedWorkbody {
                    mode,
                    verdict: report.verdict,
                })
            } else {
                Err(MountError::QuarantinedNoRun)
            }
        }
        v => Ok(MountedWorkbody { mode, verdict: v }),
    }
}
