//! Die deterministische Struktur-Beobachtung `structural_kv_v1`:
//! Repo-Dateien (Bytes) → EIN `kv_lines`-Dokument beobachteter
//! Tatsachen. Das ist die Transport-Ansicht, die der UNVERAENDERTE
//! `GitRepositoryAdapter` (decode_kv_lines, CSA.4) einzieht — kein
//! Adapter, kein Decoder, kein Gate wird angefasst.
//!
//! Ehrlichkeit: jede Datei-Beobachtung traegt den SHA-256 der ROHEN
//! Bytes — die Projektion ist damit an den tatsaechlichen Inhalt am
//! fixen Commit gebunden (Replay-Achse des Adapters). Die Zaehlungen
//! sind konservative, sichtbare Zeichenketten-Beobachtungen (dieselbe
//! dokumentierte Disziplin wie das RuleComplianceGate aus Dokument 21:
//! Stichwort-Zaehlung, keine Semantikanalyse).

use cce_core::signature::sha256;

/// Eine Datei des Ziel-Repos, wie eingelesen (Pfad + rohe Bytes).
#[derive(Debug, Clone)]
pub struct RepoFile {
    pub path: String,
    pub bytes: Vec<u8>,
}

impl RepoFile {
    pub fn new(path: &str, bytes: &[u8]) -> Self {
        Self {
            path: path.to_string(),
            bytes: bytes.to_vec(),
        }
    }
}

/// Zaehlt nicht-ueberlappende Vorkommen von `needle` in `text`.
fn count(text: &str, needle: &str) -> usize {
    text.matches(needle).count()
}

/// Beobachtete Tatsachen EINER Datei (rein mechanisch).
fn observe_file(out: &mut String, idx: usize, file: &RepoFile) {
    let text = String::from_utf8_lossy(&file.bytes);
    let is_rust = file.path.ends_with(".rs");
    out.push_str(&format!("file_{idx}_path: {}\n", file.path));
    out.push_str(&format!(
        "file_{idx}_sha256: {}\n",
        sha256(&file.bytes).to_hex()
    ));
    out.push_str(&format!("file_{idx}_lines: {}\n", text.lines().count()));
    if is_rust {
        // Konservative Stichwort-Beobachtungen (keine Semantik).
        out.push_str(&format!(
            "file_{idx}_pub_fns: {}\n",
            count(&text, "pub fn ")
        ));
        out.push_str(&format!(
            "file_{idx}_test_fns: {}\n",
            count(&text, "#[test]")
        ));
        out.push_str(&format!(
            "file_{idx}_unsafe_blocks: {}\n",
            count(&text, "unsafe ")
        ));
        out.push_str(&format!(
            "file_{idx}_unwrap_calls: {}\n",
            count(&text, ".unwrap()")
        ));
        out.push_str(&format!(
            "file_{idx}_panic_macros: {}\n",
            count(&text, "panic!(")
        ));
    }
    if file.path == "Cargo.toml" {
        // Deklarierte Edition, falls beobachtbar.
        let edition = text
            .lines()
            .find_map(|l| {
                let l = l.trim();
                l.strip_prefix("edition")
                    .and_then(|rest| rest.split('"').nth(1))
            })
            .unwrap_or("undeklariert");
        out.push_str(&format!("file_{idx}_edition: {edition}\n"));
    }
}

/// `structural_kv_v1`: das eine, deterministische kv_lines-Dokument
/// des Repo-Zustands am fixen Commit. Dateien werden nach Pfad
/// sortiert — Eingabereihenfolge aendert NICHTS (Replay-Disziplin).
pub fn observe_repo_kv(repo_id: &str, commit_sha: &str, files: &[RepoFile]) -> Vec<u8> {
    let mut sorted: Vec<&RepoFile> = files.iter().collect();
    sorted.sort_by(|a, b| a.path.cmp(&b.path));

    let mut out = String::new();
    out.push_str("observation: structural_kv_v1\n");
    out.push_str(&format!("repo: {repo_id}\n"));
    out.push_str(&format!("commit: {commit_sha}\n"));
    out.push_str(&format!("file_count: {}\n", sorted.len()));
    let has_license = sorted
        .iter()
        .any(|f| f.path == "LICENSE" || f.path == "LICENSE.md" || f.path == "NOTICE");
    out.push_str(&format!("license_file_present: {has_license}\n"));
    for (i, f) in sorted.iter().enumerate() {
        observe_file(&mut out, i, f);
    }
    out.into_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn numkit_like() -> Vec<RepoFile> {
        vec![
            RepoFile::new("src/lib.rs", b"pub fn max_of(v: &[i64]) -> i64 { v[0] }\n"),
            RepoFile::new(
                "Cargo.toml",
                b"[package]\nname = \"numkit\"\nedition = \"2021\"\n",
            ),
            RepoFile::new("tests/it.rs", b"#[test]\nfn t() { assert!(true); }\n"),
        ]
    }

    #[test]
    fn observation_is_order_independent_and_deterministic() {
        let a = observe_repo_kv("numkit", "abc1234", &numkit_like());
        let mut rev = numkit_like();
        rev.reverse();
        let b = observe_repo_kv("numkit", "abc1234", &rev);
        assert_eq!(a, b, "Eingabereihenfolge darf nichts aendern");
    }

    #[test]
    fn observation_binds_raw_bytes_via_sha256() {
        let text = String::from_utf8(observe_repo_kv("numkit", "abc1234", &numkit_like())).unwrap();
        let lib_digest = sha256(b"pub fn max_of(v: &[i64]) -> i64 { v[0] }\n").to_hex();
        assert!(
            text.contains(&lib_digest),
            "Rohbyte-Digest muss enthalten sein"
        );
        // Sortierte Dateifolge: Cargo.toml=0, src/lib.rs=1, tests/it.rs=2.
        assert!(text.contains("file_1_pub_fns: 1"));
        assert!(text.contains("file_0_edition: 2021"));
        assert!(text.contains("license_file_present: false"));
    }

    #[test]
    fn every_line_is_strict_kv() {
        let bytes = observe_repo_kv("numkit", "abc1234", &numkit_like());
        let text = String::from_utf8(bytes).unwrap();
        for line in text.lines().filter(|l| !l.trim().is_empty()) {
            let (k, _) = line.split_once(':').expect("kv-Zeile");
            assert!(
                k.trim()
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-'),
                "Schluessel {k} muss decode_kv_lines-konform sein"
            );
        }
    }
}
