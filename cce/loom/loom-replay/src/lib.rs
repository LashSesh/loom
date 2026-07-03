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

/// S-E2a I.5: additive Erweiterung von `replay_manifest_segment` um die
/// zitierten `target_core_root`s (Replay-Inputs) — bestehende Aufrufer
/// bleiben unveraendert, dieselbe Disziplin wie
/// `manifest_declare_hash_profiles` (X1c).
pub fn replay_manifest_segment_with_inputs(
    rd_digest_hex: &str,
    seed: u64,
    commit_class_hex: &str,
    input_digests_hex: &[&str],
) -> Cv {
    let Cv::Map(mut entries) = replay_manifest_segment(rd_digest_hex, seed, commit_class_hex)
    else {
        unreachable!("replay_manifest_segment liefert immer eine Map")
    };
    entries.push((
        Cv::Text("input_digests".to_string()),
        Cv::Array(
            input_digests_hex
                .iter()
                .map(|d| Cv::Text((*d).to_string()))
                .collect(),
        ),
    ));
    Cv::Map(entries)
}

/// S-E2a I.5: prueft, dass ALLE erwarteten Ziel-Digests im deklarierten
/// REPLAY_MANIFEST auftauchen — Replay verlangt dieselben Ziele (der
/// Resolver darf ein anderer sein, die Klassen nicht). Rein additive
/// Pruefung, aendert `check_replay` selbst nicht.
pub fn check_replay_inputs(
    manifest: &Cv,
    expected_input_digests_hex: &[&str],
) -> Result<(), Vec<String>> {
    let declared: Vec<&str> = match get(manifest, "input_digests") {
        Some(Cv::Array(items)) => items
            .iter()
            .filter_map(|v| match v {
                Cv::Text(s) => Some(s.as_str()),
                _ => None,
            })
            .collect(),
        _ => vec![],
    };
    let missing: Vec<String> = expected_input_digests_hex
        .iter()
        .filter(|d| !declared.contains(d))
        .map(|d| (*d).to_string())
        .collect();
    if missing.is_empty() {
        Ok(())
    } else {
        Err(missing)
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_inputs_is_additive_and_old_form_unchanged() {
        let old = replay_manifest_segment("sha256:rd", 7, "sha256:class");
        assert!(get(&old, "input_digests").is_none());
        let new = replay_manifest_segment_with_inputs(
            "sha256:rd",
            7,
            "sha256:class",
            &["sha256:target-a", "sha256:target-b"],
        );
        assert_eq!(get(&new, "rd_digest"), get(&old, "rd_digest"));
        assert_eq!(get(&new, "seed"), get(&old, "seed"));
        assert_eq!(get(&new, "commit_class"), get(&old, "commit_class"));
        match get(&new, "input_digests") {
            Some(Cv::Array(items)) => assert_eq!(items.len(), 2),
            other => panic!("input_digests fehlt/falscher Typ: {other:?}"),
        }
    }

    #[test]
    fn check_replay_inputs_finds_missing_targets() {
        let m = replay_manifest_segment_with_inputs(
            "sha256:rd",
            7,
            "sha256:class",
            &["sha256:target-a"],
        );
        assert_eq!(check_replay_inputs(&m, &["sha256:target-a"]), Ok(()));
        assert_eq!(
            check_replay_inputs(&m, &["sha256:target-a", "sha256:target-b"]),
            Err(vec!["sha256:target-b".to_string()])
        );
    }
}
