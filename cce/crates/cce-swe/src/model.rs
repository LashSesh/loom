//! Objektmodell (Dokument 18 §2): RepoWorkbody-Inhalte diesseits der
//! Container-Serialisierung (`workbody.rs`). CodeUnit/RepoSnapshot/
//! DiffCandidate/BuildRun/TestRun/TaskLedger.

use cce_core::signature::{sha256, Digest};
use cce_phaseblock::phaseblock::{BlockStatus, PhaseBlock};

/// Rolle einer CodeUnit (Dokument 18 §2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CodeUnitRole {
    Source,
    Test,
    Config,
    Doc,
}

impl CodeUnitRole {
    pub fn as_str(self) -> &'static str {
        match self {
            CodeUnitRole::Source => "source",
            CodeUnitRole::Test => "test",
            CodeUnitRole::Config => "config",
            CodeUnitRole::Doc => "doc",
        }
    }
}

/// `(path, language, content_digest, role)` — Inhalt selbst wandert als
/// CAS_BLOB (E1 wirkt weiter, s. `workbody.rs`). Der TYPE_REGISTRY-
/// Eintrag `unit:code` (Dokument 18 §6, additiv/minor) IST dieser Typ +
/// seine `as_str()`-Abdeckung — dieselbe Disziplin wie `unit:table`
/// (`cce_materialize::document::UnitType`): kein separates
/// Registry-Objekt, die Match-Arm-Abdeckung registriert bereits
/// vollstaendig.
#[derive(Debug, Clone)]
pub struct CodeUnit {
    pub path: String,
    pub language: String,
    pub content_digest: Digest,
    pub role: CodeUnitRole,
}

/// Deduplizierte, sortierte Menge von CodeUnits + Metadaten — die
/// adressierte Ausgangslage eines Bauauftrags (SWE-A4).
#[derive(Debug, Clone, Default)]
pub struct RepoSnapshot {
    pub units: Vec<CodeUnit>,
    pub toolchain_pin: String,
    pub commit_ref: Option<String>,
}

impl RepoSnapshot {
    /// Baut einen Snapshot aus `(path, language, role, content)`-Tupeln:
    /// dedupliziert nach Pfad (letzter Eintrag gewinnt), sortiert nach
    /// Pfad — deterministisch unabhaengig von der Eingabereihenfolge.
    pub fn from_files(
        files: &[(&str, &str, CodeUnitRole, &[u8])],
        toolchain_pin: &str,
        commit_ref: Option<&str>,
    ) -> Self {
        use std::collections::BTreeMap;
        let mut by_path: BTreeMap<String, CodeUnit> = BTreeMap::new();
        for (path, language, role, content) in files {
            by_path.insert(
                path.to_string(),
                CodeUnit {
                    path: path.to_string(),
                    language: language.to_string(),
                    content_digest: sha256(content),
                    role: *role,
                },
            );
        }
        Self {
            units: by_path.into_values().collect(),
            toolchain_pin: toolchain_pin.to_string(),
            commit_ref: commit_ref.map(str::to_string),
        }
    }

    /// Merkle-Wurzel ueber die (bereits sortierten) CodeUnits +
    /// Toolchain-Pin — Toolchain-Pin ist Teil des Snapshots (§5: andere
    /// Compiler-Version = anderer Snapshot). Deterministisch: gleicher
    /// Inhalt ⇒ gleiche Wurzel, unabhaengig von Bau-Reihenfolge.
    pub fn snapshot_root(&self) -> Digest {
        let mut buf = Vec::new();
        buf.extend_from_slice(self.toolchain_pin.as_bytes());
        buf.push(0x1f);
        for u in &self.units {
            buf.extend_from_slice(u.path.as_bytes());
            buf.push(0x1f);
            buf.extend_from_slice(u.language.as_bytes());
            buf.push(0x1f);
            buf.extend_from_slice(u.role.as_str().as_bytes());
            buf.push(0x1f);
            buf.extend_from_slice(&u.content_digest.0);
            buf.push(0x1e);
        }
        sha256(&buf)
    }

    pub fn get(&self, path: &str) -> Option<&CodeUnit> {
        self.units.iter().find(|u| u.path == path)
    }

    /// Wendet einen bereits verifizierten `DiffHunk` an und liefert den
    /// resultierenden Snapshot (angewandte Arbeitskopie) — der Aufrufer
    /// (die Kern-Kette) haelt daneben die tatsaechlichen Byte-Inhalte in
    /// einem `FsWriteTool`; dieser Snapshot spiegelt nur die
    /// content-adressierte Sicht danach.
    pub fn with_unit_content(
        &self,
        path: &str,
        language: &str,
        role: CodeUnitRole,
        content: &[u8],
    ) -> Self {
        let mut units: Vec<CodeUnit> = self
            .units
            .iter()
            .filter(|u| u.path != path)
            .cloned()
            .collect();
        units.push(CodeUnit {
            path: path.to_string(),
            language: language.to_string(),
            content_digest: sha256(content),
            role,
        });
        units.sort_by(|a, b| a.path.cmp(&b.path));
        Self {
            units,
            toolchain_pin: self.toolchain_pin.clone(),
            commit_ref: self.commit_ref.clone(),
        }
    }
}

/// Ein einzelner Hunk `(path, unified_diff)` — DiffCandidate traegt
/// mehrere Hunks (i. d. R. einer je betroffener Datei).
#[derive(Debug, Clone)]
pub struct DiffHunk {
    pub path: String,
    pub unified_diff: String,
}

/// Herkunft eines DiffCandidate (SWE-A1: Kandidat, nie Commit).
#[derive(Debug, Clone)]
pub enum ProducedBy {
    Provider {
        provider_id: String,
        manifest_ref: String,
    },
    Operator {
        operator: String,
    },
}

/// `(base_snapshot_root, hunks, rationale, produced_by)` — ein
/// CandidateOutput; traegt NIE einen Commit-Marker (SWE-A1).
#[derive(Debug, Clone)]
pub struct DiffCandidate {
    pub base_snapshot_root: Digest,
    pub hunks: Vec<DiffHunk>,
    pub rationale: String,
    pub produced_by: ProducedBy,
}

/// ToolEvidence eines Werkzeuglaufs — Faktum, nie Interpretation
/// (SWE-A2). `BuildRun`/`TestRun` sind strukturell identisch, bleiben
/// aber eigene Typen (unterschiedliche Rolle in der Kern-Kette/den
/// Gates, §4).
#[derive(Debug, Clone)]
pub struct BuildRun {
    pub snapshot_root_after_apply: Digest,
    pub tool_id: String,
    pub command: String,
    pub exit_code: i32,
    pub log_digest: Digest,
    pub duration_ms: u64,
    pub artifacts: Vec<String>,
}

/// Siehe `BuildRun`.
#[derive(Debug, Clone)]
pub struct TestRun {
    pub snapshot_root_after_apply: Digest,
    pub tool_id: String,
    pub command: String,
    pub exit_code: i32,
    pub log_digest: Digest,
    pub duration_ms: u64,
    pub artifacts: Vec<String>,
}

/// Die PhaseBlock-Kette EINES Bauauftrags, an eine RD gebunden
/// (Ledger=CommitProjection wie ueberall, hier auf einen Task
/// verengt statt auf den gesamten HyperDAG).
#[derive(Debug, Clone, Default)]
pub struct TaskLedger {
    pub rd_ref: Option<Digest>,
    pub blocks: Vec<PhaseBlock>,
}

impl TaskLedger {
    pub fn new(rd_ref: Digest) -> Self {
        Self {
            rd_ref: Some(rd_ref),
            blocks: Vec::new(),
        }
    }

    pub fn push(&mut self, block: PhaseBlock) {
        self.blocks.push(block);
    }

    /// Alle Bloecke des Tasks sind akzeptiert (Accept-8 erfuellt) —
    /// Voraussetzung fuer die "verify Valid"-Zertifizierung des
    /// RepoWorkbody.
    pub fn all_accepted(&self) -> bool {
        !self.blocks.is_empty()
            && self
                .blocks
                .iter()
                .all(|b| b.status == BlockStatus::Accepted)
    }
}

/// Wendet einen unified-diff-Hunk (ein oder mehrere `@@`-Bloecke) real
/// an — kein Fuzzy-Match: jede Kontext-/Entfernungszeile muss exakt zum
/// Original passen, sonst `Err` (fail-closed, kein stilles Umformen).
pub fn apply_unified_diff(original: &str, unified_diff: &str) -> Result<String, String> {
    let trailing_newline = original.ends_with('\n');
    let mut orig_lines: Vec<&str> = original.split('\n').collect();
    if trailing_newline {
        orig_lines.pop();
    }
    let mut out: Vec<String> = Vec::new();
    let mut cursor: usize = 0;

    for line in unified_diff.lines() {
        if let Some(rest) = line.strip_prefix("@@ -") {
            let old_part = rest.split(' ').next().unwrap_or("");
            let old_start: usize = old_part
                .split(',')
                .next()
                .unwrap_or("1")
                .parse()
                .map_err(|_| format!("ungueltiger Hunk-Header: {line}"))?;
            let hunk_start_idx = old_start.saturating_sub(1);
            if hunk_start_idx < cursor {
                return Err(format!(
                    "Hunk-Header {line} liegt vor bereits verarbeitetem Bereich"
                ));
            }
            while cursor < hunk_start_idx {
                out.push(
                    orig_lines
                        .get(cursor)
                        .ok_or_else(|| "Original zu kurz vor Hunk-Beginn".to_string())?
                        .to_string(),
                );
                cursor += 1;
            }
            continue;
        }
        if let Some(content) = line.strip_prefix(' ') {
            let actual = orig_lines
                .get(cursor)
                .ok_or_else(|| format!("Original zu kurz bei Kontextzeile '{content}'"))?;
            if *actual != content {
                return Err(format!(
                    "Kontextzeile stimmt nicht ueberein: erwartet '{content}', vorhanden '{actual}'"
                ));
            }
            out.push(content.to_string());
            cursor += 1;
        } else if let Some(content) = line.strip_prefix('-') {
            let actual = orig_lines
                .get(cursor)
                .ok_or_else(|| format!("Original zu kurz bei Entfernungszeile '{content}'"))?;
            if *actual != content {
                return Err(format!(
                    "zu entfernende Zeile stimmt nicht ueberein: erwartet '{content}', vorhanden '{actual}'"
                ));
            }
            cursor += 1;
        } else if let Some(content) = line.strip_prefix('+') {
            out.push(content.to_string());
        } else if line.is_empty() {
            continue;
        } else {
            return Err(format!("unbekanntes Diff-Zeilenpraefix: '{line}'"));
        }
    }
    while cursor < orig_lines.len() {
        out.push(orig_lines[cursor].to_string());
        cursor += 1;
    }
    let mut result = out.join("\n");
    if trailing_newline {
        result.push('\n');
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot_root_is_order_independent_and_deterministic() {
        let a = RepoSnapshot::from_files(
            &[
                (
                    "src/lib.rs",
                    "rust",
                    CodeUnitRole::Source,
                    b"fn add(a:i32,b:i32)->i32{a-b}",
                ),
                (
                    "tests/it.rs",
                    "rust",
                    CodeUnitRole::Test,
                    b"assert_eq!(add(2,2),4);",
                ),
            ],
            "rustc-1.0",
            Some("deadbeef"),
        );
        let b = RepoSnapshot::from_files(
            &[
                (
                    "tests/it.rs",
                    "rust",
                    CodeUnitRole::Test,
                    b"assert_eq!(add(2,2),4);",
                ),
                (
                    "src/lib.rs",
                    "rust",
                    CodeUnitRole::Source,
                    b"fn add(a:i32,b:i32)->i32{a-b}",
                ),
            ],
            "rustc-1.0",
            Some("deadbeef"),
        );
        assert_eq!(a.snapshot_root(), b.snapshot_root());
        assert_eq!(a.units.len(), 2);
        assert_eq!(a.units[0].path, "src/lib.rs");
    }

    #[test]
    fn different_toolchain_pin_changes_snapshot_root() {
        let a = RepoSnapshot::from_files(
            &[("x", "rust", CodeUnitRole::Source, b"1")],
            "rustc-1.0",
            None,
        );
        let b = RepoSnapshot::from_files(
            &[("x", "rust", CodeUnitRole::Source, b"1")],
            "rustc-2.0",
            None,
        );
        assert_ne!(a.snapshot_root(), b.snapshot_root());
    }

    #[test]
    fn apply_unified_diff_fixes_a_bug() {
        let original = "fn add(a: i32, b: i32) -> i32 {\n    a - b\n}\n";
        let diff =
            "@@ -1,3 +1,3 @@\n fn add(a: i32, b: i32) -> i32 {\n-    a - b\n+    a + b\n }\n";
        let patched = apply_unified_diff(original, diff).expect("Diff wendet an");
        assert_eq!(patched, "fn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n");
    }

    #[test]
    fn apply_unified_diff_rejects_mismatched_context() {
        let original = "a\nb\nc\n";
        let diff = "@@ -1,3 +1,3 @@\n a\n-X\n+y\n c\n";
        assert!(apply_unified_diff(original, diff).is_err());
    }
}
