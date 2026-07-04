//! Objektmodell (Dokument 20 §3): BenchmarkTaskPackage, RawRunResult,
//! CceRunResult, ComparisonMatrix (die D1–D6-Zeilen je Aufgabenklasse).
//! `evidence_present` ist eine strukturelle Tatsachenfeststellung, keine
//! Wertung (§3): der ungegatete Arm hat schlicht keinen Gate-/Replay-/
//! Zertifizierungsmechanismus, den man abfragen koennte.

use cce_core::signature::{sha256, Digest};
use cce_swe::grounding::GroundingPacket;

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

    /// Dokument 22 §3: der `packet_digest` des GroundingPacket wird TEIL
    /// des `task_package_digest` — ein Regel-Unterschied zwischen den
    /// Armen ist damit strukturell ausgeschlossen, nicht nur behauptet.
    /// Der bestehende `task_package_digest()` (P4, zwei Arme ohne
    /// Grounding) bleibt WOERTLICH unveraendert; dies ist additiv die
    /// geerdete Fassung fuer den Drei-Arm-Vergleich (P4-Ext). Beide
    /// Bestandteile werden mit dem Untertrenner 0x1e verbunden, damit die
    /// Faltung eindeutig und reihenfolgestabil ist.
    pub fn grounded_task_package_digest(&self, packet: &GroundingPacket) -> Digest {
        let base = self.task_package_digest();
        let mut buf = Vec::new();
        buf.extend_from_slice(&base.0);
        buf.push(0x1e);
        buf.extend_from_slice(packet.digest_hex().as_bytes());
        sha256(&buf)
    }

    pub fn grounded_digest_hex(&self, packet: &GroundingPacket) -> String {
        self.grounded_task_package_digest(packet).to_hex()
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

/// ExternalToolResult (Dokument 22 §2): das BEOBACHTBARE Ergebnis eines
/// Fremdwerkzeugs (Cursor / GitHub Copilot / Bolt), das CCE weder
/// kontrolliert noch dessen Inneres beobachten kann. CCE stellt keine
/// Behauptung ueber das Werkzeug auf — nur ueber die Tatsachen, die es
/// selbst gesehen hat: Enddatei-Inhalt, Wanduhrzeit, Zahl der noetigen
/// menschlichen Nachbesserungen, ob die `success_criteria` mechanisch
/// erfuellt sind. `tool_name` ist ein freies Textfeld (kein Enum).
/// `evidence_present` ist strukturell stets `false` (wie der Raw-Arm):
/// es gibt dort keinen von CCE beobachtbaren Gate-/Replay-/
/// Zertifizierungsmechanismus.
///
/// Die drei Felder `task_package_digest`, `packet_digest` und
/// `submission_order` sind nicht Teil des §2-Tupels, sondern die
/// Protokoll-Bindung aus §3 (identischer Ausgangszustand, identisches
/// GroundingPacket, Fremdarm zuerst und isoliert) — dieselbe additive
/// Disziplin wie `submission_order`/`task_package_digest` beim Raw-Arm.
#[derive(Debug, Clone)]
pub struct ExternalToolResult {
    pub package_id: String,
    /// Der GEERDETE task_package_digest (§3, packet_digest eingefaltet) —
    /// muss mit dem der anderen Arme identisch sein.
    pub task_package_digest: String,
    /// Der packet_digest des GroundingPacket, das dieser Arm tatsaechlich
    /// erhielt (als abgeleitete Exportdatei, §1). Wird zusaetzlich explizit
    /// auf Gleichheit geprueft (§3).
    pub packet_digest: String,
    pub tool_name: String,
    pub tool_version: Option<String>,
    pub output_content: Vec<u8>,
    pub wall_time_ms: u64,
    pub human_interventions_count: u32,
    pub criteria_met: bool,
    pub evidence_present: bool,
    pub observer_note: String,
    /// Reihenfolge-Marker: der Fremdarm gibt VOR dem CCE-Arm ab (§3).
    pub submission_order: u64,
}

impl ExternalToolResult {
    #[allow(clippy::too_many_arguments)]
    pub fn observed(
        package_id: &str,
        task_package_digest: &str,
        packet_digest: &str,
        tool_name: &str,
        tool_version: Option<&str>,
        output_content: Vec<u8>,
        wall_time_ms: u64,
        human_interventions_count: u32,
        criteria_met: bool,
        observer_note: &str,
        submission_order: u64,
    ) -> Self {
        Self {
            package_id: package_id.to_string(),
            task_package_digest: task_package_digest.to_string(),
            packet_digest: packet_digest.to_string(),
            tool_name: tool_name.to_string(),
            tool_version: tool_version.map(str::to_string),
            output_content,
            wall_time_ms,
            human_interventions_count,
            criteria_met,
            // Strukturell: das Fremdwerkzeug traegt keine von CCE
            // beobachtbare Evidence.
            evidence_present: false,
            observer_note: observer_note.to_string(),
            submission_order,
        }
    }

    /// Digest ueber den beobachteten Enddatei-Inhalt (fuer die
    /// Evidence-Aufzeichnung im Workbody).
    pub fn output_digest(&self) -> String {
        sha256(&self.output_content).to_hex()
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

    fn packet(pkg_id: &str) -> GroundingPacket {
        use cce_swe::grounding::{compile_grounding, RuleAtom, RuleSeverity};
        compile_grounding(
            pkg_id,
            vec![RuleAtom {
                rule_id: "no-unwrap".to_string(),
                scope: "src/".to_string(),
                trigger: ".unwrap()".to_string(),
                prescription: "Fehler propagieren".to_string(),
                severity: RuleSeverity::Blocking,
                evidence_ref: Some("CLAUDE.md#errors".to_string()),
                gate_ref: None,
                decay: None,
            }],
            vec![],
            vec!["fs_write".to_string()],
        )
        .packet
    }

    #[test]
    fn grounded_digest_folds_in_packet_and_changes_with_it() {
        let a = pkg();
        // Grounded weicht vom ungegroundeten Digest ab (packet eingefaltet).
        let p1 = packet("pkt-1");
        assert_ne!(
            a.grounded_digest_hex(&p1),
            a.digest_hex(),
            "packet_digest muss den task_package_digest veraendern"
        );
        // Anderes Packet ⇒ anderer geerdeter Digest (Regel-Unterschied
        // strukturell sichtbar, §3).
        let p2 = packet("pkt-2");
        assert_ne!(a.grounded_digest_hex(&p1), a.grounded_digest_hex(&p2));
        // Gleiches Packet ⇒ deterministisch gleicher geerdeter Digest.
        assert_eq!(
            a.grounded_digest_hex(&p1),
            a.grounded_digest_hex(&packet("pkt-1"))
        );
    }

    #[test]
    fn external_tool_result_is_evidence_free_and_free_text_named() {
        let ext = ExternalToolResult::observed(
            "p",
            "grounded-digest",
            "packet-digest",
            "Cursor",
            Some("0.42"),
            b"pub fn add(a:i32,b:i32)->i32{a+b}".to_vec(),
            60_000,
            2,
            true,
            "manuell in Cursor ausgefuehrt, zwei Nachbesserungen",
            1,
        );
        assert!(!ext.evidence_present);
        assert_eq!(ext.tool_name, "Cursor");
        assert_eq!(ext.tool_version.as_deref(), Some("0.42"));
        assert!(ext.criteria_met);
        assert!(!ext.output_digest().is_empty());
    }
}
