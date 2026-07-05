//! R-RIG-1 — Betriebsverifikation (Dokument 23 Track B): der EINE
//! reale RepoIntelligence-Lauf gegen das FREIGEGEBENE Ziel-Repo.
//! LAEUFT NIE in `cargo test --workspace` (`#[ignore]`); folgt erst
//! nach der STOPP-Freigabe (Repo-Wahl durch den Auftraggeber).
//!
//! Konvention:
//!   cargo test -p cce-repointel --test r_rig_betriebsverifikation -- \
//!       --ignored --nocapture
//!
//! Ziel-Repo per Umgebung (Default: der projekteigene numkit-Testling,
//! materialisiert nach /tmp — laut Dokument 23 als Erstziel zulaessig
//! und lizenzfrei unbedenklich):
//!   CCE_RIG_TARGET_DIR  Verzeichnis des Ziel-Repos (echte Datei-Reads)
//!   CCE_RIG_REPO_ID     Repo-Name (frei, z. B. "beispiel/numkit")
//!   CCE_RIG_LICENSE     deklarierte Lizenz (Default "cc0")
//!
//! Commit-Achse: fuer ein git-Repo der echte HEAD-SHA; fuer den
//! numkit-Testling (kein git) der deterministische Baum-Digest
//! (sha256 ueber sortierte Datei-Digests) — EHRLICH als
//! `tree:<hex>` gekennzeichnet, dieselbe Replay-Garantie (fixer
//! Stand), kein vorgetaeuschter git-Commit.
//!
//! Schreibt NUR nach /tmp (nie ins Repo): source.loom + blueprint.loom
//! + einen Klartext-Abdruck der Digests.

use cce_core::signature::sha256;
use cce_repointel::ingest::RepoIntelInput;
use cce_repointel::kette::run_repo_intelligence;
use cce_repointel::observe::RepoFile;
use std::path::{Path, PathBuf};

const NUMKIT_FILES: [(&str, &str); 3] = [
    (
        "Cargo.toml",
        "[package]\nname = \"numkit\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    ),
    (
        "src/lib.rs",
        "pub fn max_of(v: &[i64]) -> i64 {\n    let mut m = v[0];\n    for &x in v {\n        if x > m {\n            m = x;\n        }\n    }\n    m\n}\n",
    ),
    (
        "tests/it.rs",
        "use numkit::max_of;\n\n#[test]\nfn finds_maximum() {\n    assert_eq!(max_of(&[1, 9, 3]), 9);\n}\n",
    ),
];

/// Materialisiert den numkit-Testling nach /tmp (Default-Ziel).
fn materialize_numkit() -> PathBuf {
    let dir = std::env::temp_dir().join("cce-rig-1-numkit");
    let _ = std::fs::remove_dir_all(&dir);
    for (path, content) in NUMKIT_FILES {
        let full = dir.join(path);
        if let Some(parent) = full.parent() {
            std::fs::create_dir_all(parent).expect("mkdir");
        }
        std::fs::write(full, content).expect("write");
    }
    dir
}

/// Liest ALLE Dateien eines Verzeichnisses (rekursiv, sortiert) —
/// echte fs-Reads, kein Fixture-Bypass.
fn read_repo(dir: &Path) -> Vec<RepoFile> {
    fn walk(base: &Path, dir: &Path, out: &mut Vec<RepoFile>) {
        let mut entries: Vec<_> = std::fs::read_dir(dir)
            .expect("read_dir")
            .map(|e| e.expect("entry").path())
            .collect();
        entries.sort();
        for p in entries {
            if p.is_dir() {
                if p.file_name().is_some_and(|n| n == ".git" || n == "target") {
                    continue;
                }
                walk(base, &p, out);
            } else {
                let rel = p
                    .strip_prefix(base)
                    .expect("rel")
                    .to_string_lossy()
                    .replace('\\', "/");
                let bytes = std::fs::read(&p).expect("read");
                out.push(RepoFile::new(&rel, &bytes));
            }
        }
    }
    let mut files = Vec::new();
    walk(dir, dir, &mut files);
    files
}

/// Deterministischer Baum-Digest als Commit-Achse fuer Nicht-git-Ziele.
fn tree_commit(files: &[RepoFile]) -> String {
    let mut sorted: Vec<&RepoFile> = files.iter().collect();
    sorted.sort_by(|a, b| a.path.cmp(&b.path));
    let mut buf = Vec::new();
    for f in sorted {
        buf.extend_from_slice(f.path.as_bytes());
        buf.push(0x1f);
        buf.extend_from_slice(&sha256(&f.bytes).0);
        buf.push(0x1e);
    }
    format!("tree:{}", sha256(&buf).to_hex())
}

#[test]
#[ignore = "R-RIG-1: realer Lauf, erst nach STOPP-Freigabe (Repo-Wahl)"]
fn r_rig_1_real_repo_intelligence_run() {
    // Ziel bestimmen (Default: numkit-Testling).
    let target_dir = std::env::var("CCE_RIG_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| materialize_numkit());
    let repo_id =
        std::env::var("CCE_RIG_REPO_ID").unwrap_or_else(|_| "beispiel/numkit".to_string());
    let license = std::env::var("CCE_RIG_LICENSE").unwrap_or_else(|_| "cc0".to_string());

    let files = read_repo(&target_dir);
    assert!(!files.is_empty(), "Ziel-Repo ist leer: {target_dir:?}");
    let commit = tree_commit(&files);

    println!("=== R-RIG-1 RepoIntelligence — realer Lauf ===");
    println!(
        "ziel        : {} ({} Dateien)",
        target_dir.display(),
        files.len()
    );
    println!("repo_id     : {repo_id}");
    println!("commit-achse: {commit}");
    println!("lizenz      : {license}");

    let input = RepoIntelInput {
        repo_id,
        commit_sha: commit,
        declared_license: license,
        files,
    };
    let out = run_repo_intelligence(&input).expect("P5-Kette gruen");

    // Beide Koerper verifizieren.
    let vs = loom_verify::verify(&out.source_sealed.bytes);
    let vb = loom_verify::verify(&out.blueprint_sealed.bytes);
    println!("source  verdict: {:?}", vs.verdict);
    println!("bauplan verdict: {:?}", vb.verdict);
    assert_eq!(
        vs.verdict,
        loom_verify::Verdict::Valid,
        "{:?}",
        vs.diagnoses
    );
    assert!(
        matches!(
            vb.verdict,
            loom_verify::Verdict::Valid | loom_verify::Verdict::ValidWithResidues
        ),
        "{:?}",
        vb.diagnoses
    );

    // Artefakte NUR nach /tmp.
    let out_dir = std::env::temp_dir().join("cce-rig-1-out");
    std::fs::create_dir_all(&out_dir).expect("mkdir out");
    std::fs::write(out_dir.join("source.loom"), &out.source_sealed.bytes).expect("write source");
    std::fs::write(out_dir.join("blueprint.loom"), &out.blueprint_sealed.bytes)
        .expect("write blueprint");

    println!("packet_digest    : {}", out.packet.digest_hex());
    println!("blueprint_class  : {}", out.blueprint_class_hex);
    println!(
        "regeln           : {} (alle belegt), offene entscheidungen: {}",
        out.packet.rules.len(),
        out.packet.open_decisions.len()
    );
    println!(
        "zertifizierte blueprints: {}",
        out.distilled.outcome.certified.len()
    );
    for r in &out.visible_residues {
        println!("sichtbares residuum: {} — {}", r.id, r.content);
    }
    println!("artefakte: {}", out_dir.display());
    println!("=== Ende R-RIG-1 ===");
}
