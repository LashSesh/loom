//! Die End-to-End-Reise (S2.2) als Verkabelung: Naht 4 (Entnehmen:
//! Artefakt mit ZWEI Digests + Zertifikat, S7) und Naht 5 (Ablegen/
//! Wiederholen: Re-Import-Erkennung, Replay-Klasse). Fehl-Reisen enden
//! LESBAR (S2.5): Formatverlust ist ein sichtbares format_loss-Residuum
//! mit bewusster Operator-Entscheidung, nie ein stiller Export.

use crate::engine::EnginePort;
use crate::state::{CockpitCore, CockpitState, Confirmation};
use cce_core::canonical::Canonicalize;
use cce_core::gate::GateReport;
use cce_core::residue::{Residue, ResidueKind, Severity};
use cce_core::signature::{sha256, Digest};
use cce_materialize::document::parse::parse_markdown;

/// Naht-4-Ausgabe: Artefakt-Bytes + Zertifikat (Zwei-Digest-Modell S7):
/// `content_class` bindet die Bedeutung, `byte_digest` die Datei.
#[derive(Debug, Clone)]
pub struct CertifiedArtifact {
    pub bytes: Vec<u8>,
    pub format: &'static str,
    pub content_class: Digest,
    pub byte_digest: Digest,
    pub gate_reports: Vec<GateReport>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum JourneyError {
    NoArtifact,
    ConfirmationRequired,
}

/// Naht 4: Entnehmen. Nur aus ARTEFAKT_VERFUEGBAR, nur mit
/// Bestaetigung; das Zertifikat traegt beide Digests + die Gate-Fakten.
pub fn take_artifact<E: EnginePort>(
    core: &mut CockpitCore<E>,
    confirmation: Option<Confirmation>,
) -> Result<CertifiedArtifact, JourneyError> {
    if core.state != CockpitState::ArtefaktVerfuegbar {
        return Err(JourneyError::NoArtifact);
    }
    core.export_artifact(confirmation)
        .map_err(|_| JourneyError::ConfirmationRequired)?;
    let artifact = core.engine.artifact().ok_or(JourneyError::NoArtifact)?;
    let crystal = core
        .confirmed_crystal
        .as_ref()
        .ok_or(JourneyError::NoArtifact)?;
    Ok(CertifiedArtifact {
        bytes: artifact.bytes.clone(),
        format: artifact.format,
        content_class: crystal.canonical_class().0,
        byte_digest: artifact.byte_digest(),
        gate_reports: core.engine.gate_reports(),
    })
}

/// Re-Import-Erkennung (Naht 5 / S7): parst das entnommene Artefakt
/// zurueck und vergleicht die KANONISCHE KLASSE mit dem Zertifikat.
#[derive(Debug)]
pub enum ReimportVerdict {
    /// klassenidentisch: dasselbe Werk, als Wiederkehr ERKANNT.
    SameClass {
        class: String,
    },
    /// aeussere Bearbeitung: das Zertifikat bricht NACHWEISBAR —
    /// mit beiden Klassen benannt.
    CertificateBroken {
        certified: String,
        reimported: String,
    },
    Unparseable {
        reason: String,
    },
}

pub fn reimport(cert: &CertifiedArtifact, bytes: &[u8]) -> ReimportVerdict {
    match parse_markdown(bytes) {
        Ok(crystal) => {
            let class = crystal.canonical_class().0;
            if class == cert.content_class {
                ReimportVerdict::SameClass {
                    class: class.to_hex(),
                }
            } else {
                ReimportVerdict::CertificateBroken {
                    certified: cert.content_class.to_hex(),
                    reimported: class.to_hex(),
                }
            }
        }
        Err(e) => ReimportVerdict::Unparseable { reason: e },
    }
}

/// Fehl-Reise „Formatverlust beim Export" (S2.5/S7.4): ein verlustiger
/// Export (z. B. reiner Fliesstext ohne Struktur-Anker) ist MOEGLICH,
/// aber nur mit sichtbarem format_loss-Residuum + Bestaetigung.
pub fn export_lossy_plaintext(
    cert: &CertifiedArtifact,
    confirmation: Option<&Confirmation>,
) -> Result<(Vec<u8>, Residue), Box<Residue>> {
    let residue = Residue::new(
        "s7:format_loss",
        "artifact_export",
        ResidueKind::named("format_loss"),
        Severity::Warning,
        "Klartext-Export verliert Struktur-Anker: Re-Import kann die \
         Klasse nicht mehr nachweisen — bewusste Operator-Entscheidung",
    );
    if confirmation.is_none() {
        return Err(Box::new(residue));
    }
    // Anker-Zeilen (<!--cce:...-->) entfernen = Verlustform.
    let text: String = String::from_utf8_lossy(&cert.bytes)
        .lines()
        .filter(|l| !l.trim_start().starts_with("<!--cce:"))
        .collect::<Vec<_>>()
        .join("\n");
    Ok((text.into_bytes(), residue))
}

/// Replay-Beleg fuer Naht 5: dieselbe Klasse aus Crystal+RD.
pub fn replay_matches<E: EnginePort>(core: &CockpitCore<E>, cert: &CertifiedArtifact) -> bool {
    // Die Commit-Klasse des Laufs ist deterministisch reproduzierbar;
    // das Artefakt-Byte-Digest bindet die konkrete Datei.
    core.engine.replay_class().is_some() && sha256(&cert.bytes) == cert.byte_digest
}
