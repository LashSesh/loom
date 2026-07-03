//! loom-sdk — S-E4a Teil E4b: die schmale, stabile Fassade
//! (open/inspect/verify/extract/replay) ueber den bestehenden
//! loom-*-Kern. READER-PRINZIP (LOOM Teil 6): motorfrei, haengt an
//! KEINEM cce-*/nexus-*/cockpit-*-Crate — der wasm32-Viewer (E4b) baut
//! direkt auf dieser Fassade auf und beweist damit
//! Plattform-Unabhaengigkeit des Reader-Prinzips.
//!
//! `replay` reproduziert den Motor-Lauf NICHT selbst (das waere
//! Motor-Wissen) — es vergleicht eine vom AUFRUFER bereits reproduzierte
//! Klasse gegen das im Container deklarierte REPLAY_MANIFEST
//! (`loom_replay::check_replay`, unveraendert).

pub use loom_mount::{ExtractError, LoomHandle, OpenError};
pub use loom_replay::ReplayVerdict;
pub use loom_verify::{Diagnosis, Verdict, VerificationReport};

/// open (M1–M2): reine Verifikation + Deserialisierung, kein Hook, kein
/// Autostart, kein Egress.
pub fn open(bytes: &[u8]) -> Result<LoomHandle, OpenError> {
    loom_mount::open(bytes)
}

/// inspect: die fuenf Pflichtansichten-Rohdaten (Manifest/Segmentliste/
/// Residuen+Verdikt/Gate-Reports/Ledger-Referenz) aus L0-L2.
pub fn inspect(bytes: &[u8], handle: &LoomHandle) -> loom_mount::InspectionReport {
    loom_mount::inspect(bytes, handle)
}

/// verify: L0-L2 in einem Lauf (nie mehr als `valid_with_residues` fuer
/// unaufgeloeste externe Referenzen — s. `manifest_citation_mismatch`,
/// S-E2a).
pub fn verify(bytes: &[u8]) -> VerificationReport {
    loom_verify::verify(bytes)
}

/// extract: das materialisierte Artefakt byte-identisch aus dem
/// Container (X1a) — Digest-Gegenprobe ist Pflicht, kein blindes
/// Vertrauen auf Position/Reihenfolge.
pub fn extract(handle: &LoomHandle) -> Result<Vec<u8>, ExtractError> {
    loom_mount::extract_artifact(handle)
}

/// extract_children: alle gebuendelten CAS_BLOB-Kinder (X1b, SCALE-2+).
pub fn extract_children(handle: &LoomHandle) -> Result<Vec<Vec<u8>>, ExtractError> {
    loom_mount::all_cas_blobs(handle)
}

#[derive(Debug)]
pub enum ReplayError {
    Open(OpenError),
    NoReplayManifest,
    Decode(String),
}

/// replay: vergleicht eine vom Aufrufer reproduzierte Klasse gegen das
/// deklarierte REPLAY_MANIFEST-Segment. Die Reproduktion SELBST bleibt
/// beim Aufrufer (Motor-Port) — die Fassade bleibt motorfrei.
pub fn replay(bytes: &[u8], reproduced_class_hex: &str) -> Result<ReplayVerdict, ReplayError> {
    let dec = loom_codec::decode_sealed(bytes)
        .map_err(|e| ReplayError::Open(OpenError::Structure(vec![format!("{e:?}")])))?;
    let manifest = dec
        .frames
        .iter()
        .find(|(e, _)| e.kind == loom_format::KIND_REPLAY_MANIFEST)
        .ok_or(ReplayError::NoReplayManifest)?;
    let cv = loom_canon::decode(&manifest.1.payload)
        .map_err(|e| ReplayError::Decode(format!("{e:?}")))?;
    Ok(loom_replay::check_replay(&cv, reproduced_class_hex))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn r1_bytes() -> Vec<u8> {
        // Minimaler, echter Inspect-Container — dieselbe Struktur wie
        // R1 (loom-conformance::build_r1), hier ohne die
        // Motor-Abhaengigkeit von loom-conformance direkt gebaut, damit
        // loom-sdk motorfrei bleibt (Reader-Prinzip).
        use loom_canon::Cv;
        use loom_codec::{seal_canonical, Segment};
        use loom_format::{KIND_CANON_DESC, KIND_CL_SUBSTRATE, KIND_MANIFEST};
        let m = Cv::map(vec![
            ("title", Cv::Text("sdk-test".into())),
            ("container_class", Cv::Text("inspection".into())),
            ("domain_refs", Cv::Array(vec![])),
            ("scale", Cv::Uint(1)),
            ("pl_level", Cv::Text("PL0".into())),
            ("claims", Cv::map(vec![("closed", Cv::Bool(false))])),
            ("origin", Cv::map(vec![])),
            (
                "profiles_required",
                Cv::Array(vec![Cv::Text("inspection".into())]),
            ),
            ("profiles_optional", Cv::Array(vec![])),
            (
                "residue_summary",
                Cv::map(vec![("count", Cv::Uint(0)), ("kinds", Cv::Array(vec![]))]),
            ),
            ("capability_declarations", Cv::Array(vec![])),
            ("license_summary", Cv::Text("cc0".into())),
            (
                "created",
                Cv::Tag(0, Box::new(Cv::Text("2026-01-01T00:00:00Z".into()))),
            ),
        ]);
        let cl = Cv::map(vec![
            ("cubes", Cv::Array(vec![])),
            ("constraints", Cv::Array(vec![])),
        ]);
        seal_canonical(
            "inspection",
            &["inspection"],
            &[
                Segment::canonical(KIND_MANIFEST, &m).unwrap(),
                Segment {
                    kind: KIND_CANON_DESC,
                    seg_flags: 0,
                    payload: Cv::Text(loom_canon::CANON_RULES_TEXT.into())
                        .encode()
                        .unwrap(),
                    deps: vec![],
                },
                Segment::canonical(KIND_CL_SUBSTRATE, &cl).unwrap(),
            ],
        )
        .unwrap()
        .bytes
    }

    #[test]
    fn open_inspect_verify_extract_roundtrip() {
        let bytes = r1_bytes();
        let handle = open(&bytes).expect("open");
        let report = inspect(&bytes, &handle);
        assert!(report.segment_kinds.contains(&0x0001)); // MANIFEST
        let v = verify(&bytes);
        assert_eq!(v.verdict, Verdict::Valid);
        // R1-artige Container tragen kein ARTIFACT-Segment -> NoArtifact.
        assert!(matches!(extract(&handle), Err(ExtractError::NoArtifact)));
    }

    #[test]
    fn replay_reports_missing_manifest_when_absent() {
        let bytes = r1_bytes();
        assert!(matches!(
            replay(&bytes, "sha256:whatever"),
            Err(ReplayError::NoReplayManifest)
        ));
    }

    #[test]
    fn replay_passes_on_matching_reproduced_class() {
        use loom_codec::{seal_canonical, Segment};
        use loom_format::KIND_REPLAY_MANIFEST;

        let replay_manifest = loom_replay::replay_manifest_segment("sha256:rd", 7, "sha256:class");
        let dec = loom_codec::decode_sealed(&r1_bytes()).unwrap();
        let mut segments: Vec<Segment> = dec
            .frames
            .iter()
            .filter(|(e, _)| {
                e.kind != loom_format::KIND_HEADER && e.kind != loom_format::KIND_SEGTAB
            })
            .map(|(e, f)| Segment {
                kind: e.kind,
                seg_flags: e.seg_flags,
                payload: f.payload.clone(),
                deps: e.deps.clone(),
            })
            .collect();
        segments.push(Segment::canonical(KIND_REPLAY_MANIFEST, &replay_manifest).unwrap());
        let with_replay = seal_canonical("inspection", &["inspection"], &segments)
            .unwrap()
            .bytes;

        assert_eq!(
            replay(&with_replay, "sha256:class").unwrap(),
            ReplayVerdict::Pass
        );
        assert!(matches!(
            replay(&with_replay, "sha256:andere-klasse").unwrap(),
            ReplayVerdict::ClassMismatch { .. }
        ));
    }

    #[test]
    fn open_fails_closed_on_malformed_bytes() {
        assert!(open(b"kein loom").is_err());
    }
}
