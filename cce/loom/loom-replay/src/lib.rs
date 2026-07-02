//! loom-replay — L3/C4: Klassen-Replay (LOOM Teil 5.2/10.3). Replay
//! erzeugt aus dem REPLAY_MANIFEST dieselbe Commit-Klasse — Vergleich
//! modulo kanonischer Klasse, nie Bytes (Port: cce-core).

use loom_canon::Cv;

/// REPLAY_MANIFEST-Payload: RD, Seed, Versionen, Snapshot-Refs.
pub fn replay_manifest_segment(rd_digest_hex: &str, seed: u64, commit_class_hex: &str) -> Cv {
    Cv::map(vec![
        ("rd_digest", Cv::Text(rd_digest_hex.into())),
        ("seed", Cv::Uint(seed)),
        ("commit_class", Cv::Text(commit_class_hex.into())),
        ("engine_version", Cv::Text("cce-0.1".into())),
    ])
}

#[derive(Debug, PartialEq, Eq)]
pub enum ReplayVerdict {
    /// Reproduzierte Klasse == deklarierte Klasse.
    Pass,
    /// Abweichung — mit beiden Klassen benannt.
    ClassMismatch {
        declared: String,
        reproduced: String,
    },
    ManifestIncomplete {
        missing: &'static str,
    },
}

fn get<'a>(map: &'a Cv, key: &str) -> Option<&'a Cv> {
    match map {
        Cv::Map(entries) => entries.iter().find_map(|(k, v)| match k {
            Cv::Text(s) if s == key => Some(v),
            _ => None,
        }),
        _ => None,
    }
}

/// replay(handle, rd): fuehrt die (vom Aufrufer via Motor-Port
/// reproduzierte) Klasse gegen das Manifest. Der Motor-Lauf selbst
/// bleibt beim Aufrufer — loom-replay prueft den Vertrag (N7: ein
/// unvollstaendiges Manifest scheitert HIER, benannt).
pub fn check_replay(manifest: &Cv, reproduced_class_hex: &str) -> ReplayVerdict {
    let Some(Cv::Text(declared)) = get(manifest, "commit_class") else {
        return ReplayVerdict::ManifestIncomplete {
            missing: "commit_class",
        };
    };
    if get(manifest, "rd_digest").is_none() {
        return ReplayVerdict::ManifestIncomplete {
            missing: "rd_digest",
        };
    }
    if get(manifest, "seed").is_none() {
        return ReplayVerdict::ManifestIncomplete { missing: "seed" };
    }
    if declared == reproduced_class_hex {
        ReplayVerdict::Pass
    } else {
        ReplayVerdict::ClassMismatch {
            declared: declared.clone(),
            reproduced: reproduced_class_hex.to_string(),
        }
    }
}
