//! Objektmodell (Dokument 20 §3): BenchmarkTaskPackage, RawRunResult,
//! CceRunResult, ComparisonMatrix (die D1–D6-Zeilen je Aufgabenklasse).
//! `evidence_present` ist eine strukturelle Tatsachenfeststellung, keine
//! Wertung (§3): der ungegatete Arm hat schlicht keinen Gate-/Replay-/
//! Zertifizierungsmechanismus, den man abfragen koennte.

use cce_core::signature::{sha256, Digest};

/// Die zwei Aufgabenklassen aus Dokument 20 §1 (Coding = CCEs neue
/// Staerke P2/P3; Document = CCEs urspruengliche Staerke, 213 Domaenen).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskClass {
    Coding,
    Document,
}

impl TaskClass {
    pub fn as_str(self) -> &'static str {
        match self {
            TaskClass::Coding => "coding",
            TaskClass::Document => "document",
        }
    }
}

/// `(package_id, task_text, starting_files, success_criteria,
/// task_package_digest)` — beide Arme erhalten EXAKT dies, gehasht vor
/// Beginn (§2). `build_command`/`test_command` sind typisierte
/// Programm-Argv (nie Shell), ueber die der Erfolg maschinell geprueft
/// wird — dieselbe Disziplin wie P2 (kein `sh -c`). `target_path` ist
/// die eine Datei, die ein Arm erzeugt/aendert.
#[derive(Debug, Clone)]
pub struct BenchmarkTaskPackage {
    pub package_id: String,
    pub task_class: TaskClass,
    pub task_text: String,
    /// Sortiert-deterministische Ausgangslage: (Pfad, Inhalt).
    pub starting_files: Vec<(String, Vec<u8>)>,
    pub success_criteria: String,
    pub build_command: Vec<String>,
    pub test_command: Vec<String>,
    pub target_path: String,
}

impl BenchmarkTaskPackage {
    /// Deterministischer Digest ueber ALLE Felder (sortierte
    /// starting_files) — der `task_package_digest` aus §2/§3, vor Beginn
    /// beider Arme fixiert. Gleiche Semantik ⇒ gleicher Digest,
    /// unabhaengig von der Eingabereihenfolge der Dateien.
    pub fn task_package_digest(&self) -> Digest {
        let mut files = self.starting_files.clone();
        files.sort_by(|a, b| a.0.cmp(&b.0));
        let mut buf = Vec::new();
        buf.extend_from_slice(self.package_id.as_bytes());
        buf.push(0x1f);
        buf.extend_from_slice(self.task_class.as_str().as_bytes());
        buf.push(0x1f);
        buf.extend_from_slice(self.task_text.as_bytes());
        buf.push(0x1f);
        buf.extend_from_slice(self.success_criteria.as_bytes());
        buf.push(0x1f);
        buf.extend_from_slice(self.build_command.join("\u{1f}").as_bytes());
        buf.push(0x1f);
        buf.extend_from_slice(self.test_command.join("\u{1f}").as_bytes());
        buf.push(0x1f);
        buf.extend_from_slice(self.target_path.as_bytes());
        buf.push(0x1e);
        for (path, content) in &files {
            buf.extend_from_slice(path.as_bytes());
            buf.push(0x1f);
            buf.extend_from_slice(&sha256(content).0);
            buf.push(0x1e);
        }
        sha256(&buf)
    }

    pub fn digest_hex(&self) -> String {
        self.task_package_digest().to_hex()
    }
}

/// Ergebnis des UNGEGATETEN Arms (§3). `evidence_present` ist stets
/// `false` — strukturelle Tatsachenfeststellung: es existiert dort kein
/// Gate-/Replay-/Zertifizierungsmechanismus.
#[derive(Debug, Clone)]
pub struct RawRunResult {
    pub package_id: String,
    pub task_package_digest: String,
    pub output_digest: String,
    pub wall_time_ms: u64,
    pub build_pass: Option<bool>,
    pub test_pass: Option<bool>,
    pub human_interventions_count: u32,
    pub evidence_present: bool,
    /// Reihenfolge-Marker: der Raw-Arm gibt VOR dem CCE-Arm ab (§2).
    pub submission_order: u64,
}

impl RawRunResult {
    #[allow(clippy::too_many_arguments)]
    pub fn observed(
        package_id: &str,
        task_package_digest: &str,
        output_digest: &str,
        wall_time_ms: u64,
        build_pass: Option<bool>,
        test_pass: Option<bool>,
        human_interventions_count: u32,
        submission_order: u64,
    ) -> Self {
        Self {
            package_id: package_id.to_string(),
            task_package_digest: task_package_digest.to_string(),
            output_digest: output_digest.to_string(),
            wall_time_ms,
            build_pass,
            test_pass,
            human_interventions_count,
            // Strukturell: der ungegatete Arm HAT keine Evidence.
            evidence_present: false,
            submission_order,
        }
    }

    /// Erfolg gegen die `success_criteria` (§8 D4): fuer Coding
    /// build ∧ test; fuer Document (kein Compile) genuegt test.
    pub fn criteria_met(&self) -> bool {
        match (self.build_pass, self.test_pass) {
            (Some(b), Some(t)) => b && t,
            (None, Some(t)) => t,
            _ => false,
        }
    }
}

/// Ergebnis des CCE-Arms (§3): dieselben Felder + `repo_workbody_ref`
/// (P2/P3-Kette, generalisiert auf ein beliebiges Zielpaket) + die
/// Evidence-/Gate-Kennzahlen. `evidence_present` ist stets `true`.
#[derive(Debug, Clone)]
pub struct CceRunResult {
    pub package_id: String,
    pub task_package_digest: String,
    pub output_digest: String,
    pub wall_time_ms: u64,
    pub build_pass: Option<bool>,
    pub test_pass: Option<bool>,
    pub human_interventions_count: u32,
    pub evidence_present: bool,
    pub submission_order: u64,
    /// core_root (hex) des versiegelten "repo"-Workbody dieses Arms.
    pub repo_workbody_ref: String,
    pub gate_report_count: u32,
    pub replay_confirmed: bool,
}

impl CceRunResult {
    #[allow(clippy::too_many_arguments)]
    pub fn certified(
        package_id: &str,
        task_package_digest: &str,
        output_digest: &str,
        wall_time_ms: u64,
        build_pass: Option<bool>,
        test_pass: Option<bool>,
        human_interventions_count: u32,
        submission_order: u64,
        repo_workbody_ref: &str,
        gate_report_count: u32,
        replay_confirmed: bool,
    ) -> Self {
        Self {
            package_id: package_id.to_string(),
            task_package_digest: task_package_digest.to_string(),
            output_digest: output_digest.to_string(),
            wall_time_ms,
            build_pass,
            test_pass,
            human_interventions_count,
            // Strukturell: der CCE-Arm TRAEGT Evidence.
            evidence_present: true,
            submission_order,
            repo_workbody_ref: repo_workbody_ref.to_string(),
            gate_report_count,
            replay_confirmed,
        }
    }

    pub fn criteria_met(&self) -> bool {
        match (self.build_pass, self.test_pass) {
            (Some(b), Some(t)) => b && t,
            (None, Some(t)) => t,
            _ => false,
        }
    }
}

/// Eine Zelle der Vergleichsmatrix. `verdict` ist die Kategorie,
/// `beleg` der Verweis (Digest / Testname / "strukturell nicht
/// vorhanden") — §3: jede Zelle traegt einen Belegverweis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatrixCell {
    pub verdict: String,
    pub beleg: String,
}

impl MatrixCell {
    pub fn present(beleg: &str) -> Self {
        Self {
            verdict: "present".to_string(),
            beleg: beleg.to_string(),
        }
    }
    /// Der Raw-Arm bei D1/D2/D3/D6: strukturell nicht vorhanden
    /// (Tatsachenfeststellung, kein Werturteil, §8).
    pub fn structurally_absent() -> Self {
        Self {
            verdict: "structurally_absent".to_string(),
            beleg: "kein Gate-/Replay-/Zertifizierungsmechanismus vorhanden".to_string(),
        }
    }
    pub fn criteria(met: bool, beleg: &str) -> Self {
        Self {
            verdict: if met {
                "criteria_met".to_string()
            } else {
                "criteria_failed".to_string()
            },
            beleg: beleg.to_string(),
        }
    }
}

/// Eine D-Zeile: Dimension + Raw-Zelle + CCE-Zelle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatrixRow {
    pub dimension: String,
    pub raw: MatrixCell,
    pub cce: MatrixCell,
}

/// Die D1–D6-Matrix je Aufgabenklasse (§3).
#[derive(Debug, Clone)]
pub struct ComparisonMatrix {
    pub package_id: String,
    pub task_class: TaskClass,
    pub rows: Vec<MatrixRow>,
}

impl ComparisonMatrix {
    pub fn is_complete(&self) -> bool {
        self.rows.len() == 6
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pkg() -> BenchmarkTaskPackage {
        BenchmarkTaskPackage {
            package_id: "bench-x".to_string(),
            task_class: TaskClass::Coding,
            task_text: "fix the bug".to_string(),
            starting_files: vec![
                ("src/lib.rs".to_string(), b"fn f(){}".to_vec()),
                ("tests/t.rs".to_string(), b"assert!(true);".to_vec()),
            ],
            success_criteria: "cargo test gruen".to_string(),
            build_command: vec!["cargo".to_string(), "build".to_string()],
            test_command: vec!["cargo".to_string(), "test".to_string()],
            target_path: "src/lib.rs".to_string(),
        }
    }

    #[test]
    fn package_digest_is_order_independent() {
        let a = pkg();
        let mut b = pkg();
        b.starting_files.reverse();
        assert_eq!(a.task_package_digest(), b.task_package_digest());
    }

    #[test]
    fn package_digest_changes_with_content() {
        let a = pkg();
        let mut b = pkg();
        b.task_text = "anderer Auftrag".to_string();
        assert_ne!(a.task_package_digest(), b.task_package_digest());
    }

    #[test]
    fn raw_result_is_always_evidence_free_cce_always_evidence_bearing() {
        let raw = RawRunResult::observed("p", "d", "o", 10, Some(true), Some(true), 3, 1);
        assert!(!raw.evidence_present);
        assert!(raw.criteria_met());
        let cce = CceRunResult::certified(
            "p",
            "d",
            "o",
            10,
            Some(true),
            Some(true),
            0,
            2,
            "root",
            12,
            true,
        );
        assert!(cce.evidence_present);
        assert!(cce.criteria_met());
    }

    #[test]
    fn document_criteria_needs_only_test() {
        let raw = RawRunResult::observed("p", "d", "o", 10, None, Some(true), 0, 1);
        assert!(raw.criteria_met());
        let raw_fail = RawRunResult::observed("p", "d", "o", 10, None, Some(false), 0, 1);
        assert!(!raw_fail.criteria_met());
    }
}
