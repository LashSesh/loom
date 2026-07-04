//! Das ToolGateway: Ausfuehrung NUR mit (a) validem ToolManifest,
//! (b) offenem klassen-eigenem ToolCapabilityLock im Scope,
//! (c) fuer Remote zusaetzlich ToolEgressGate,
//! (d) Budget, (e) Aufzeichnung. Keine Sammel-Freischaltung: der Lock
//! wird je (Klasse, Scope) gefuehrt.

use crate::manifest::{Egress, ToolManifest};
use cce_core::capability::CapabilityLock;
use cce_core::residue::{Residue, ResidueKind, Severity};
use std::collections::BTreeMap;

fn tool_residue(kind: &str, detail: &str) -> Residue {
    Residue::new(
        &format!("tool:{kind}"),
        "toolgateway",
        ResidueKind::named(kind),
        Severity::Blocking,
        detail,
    )
}

/// Das Referenz-Tool fs_read (F.1 h): liest aus einer VORHER
/// deklarierten, eingefrorenen Sicht (Fixture-Dateibaum) — kein
/// freier Dateisystemzugriff.
#[derive(Debug, Default)]
pub struct FsReadTool {
    pub files: BTreeMap<String, Vec<u8>>,
}

impl FsReadTool {
    pub fn with_files(files: &[(&str, &[u8])]) -> Self {
        Self {
            files: files
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_vec()))
                .collect(),
        }
    }
}

/// fs_write (P2/Dokument 18 §3): schreibt NUR in eine deklarierte
/// Arbeitskopie. Bau-Default: eine In-Memory-Sicht (dieselbe
/// Fixture-Disziplin wie FsReadTool) — niemals ausserhalb des
/// deklarierten Scopes, niemals real ausserhalb dieser Struktur.
#[derive(Debug, Default)]
pub struct FsWriteTool {
    pub files: BTreeMap<String, Vec<u8>>,
}

impl FsWriteTool {
    pub fn with_files(files: &[(&str, &[u8])]) -> Self {
        Self {
            files: files
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_vec()))
                .collect(),
        }
    }
}

/// git-Operationen (P2 §3): status/diff/add/commit/branch lokal; push
/// braucht zusaetzlich Remote-Egress. `commit`/`push` sind materielle
/// Aktionen — HumanConfirmationGate-pflichtig (hier: `confirmation_ref`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GitOperation {
    Status,
    Diff,
    Add(String),
    Commit(String),
    Branch(String),
    Push,
}

/// git-Tool. Bau-Default: Fixture-Zustand (kein realer Prozess) —
/// dieselbe Hermetik-Disziplin wie FsReadTool/FsWriteTool.
#[derive(Debug, Clone, Default)]
pub struct GitTool {
    pub fixture_status: String,
    pub fixture_diff: String,
    pub staged: Vec<String>,
    pub commits: Vec<String>,
    pub branches: Vec<String>,
    pub pushed: bool,
}

/// Typisierter Programmaufruf (NIE per Shell interpretiert, §3 Verbot:
/// "kein Shell-Sammelzugriff") + Fixture-Ergebnis fuer den Bau-Default.
#[derive(Debug, Clone, Default)]
pub struct BuildTool {
    pub argv: Vec<String>,
    pub fixture_exit_code: i32,
    pub fixture_log: String,
}

impl BuildTool {
    pub fn with_fixture(argv: &[&str], exit_code: i32, log: &str) -> Self {
        Self {
            argv: argv.iter().map(|s| s.to_string()).collect(),
            fixture_exit_code: exit_code,
            fixture_log: log.to_string(),
        }
    }
}

/// Analog zu `BuildTool`, eigene Klasse `test`.
#[derive(Debug, Clone, Default)]
pub struct TestTool {
    pub argv: Vec<String>,
    pub fixture_exit_code: i32,
    pub fixture_log: String,
}

impl TestTool {
    pub fn with_fixture(argv: &[&str], exit_code: i32, log: &str) -> Self {
        Self {
            argv: argv.iter().map(|s| s.to_string()).collect(),
            fixture_exit_code: exit_code,
            fixture_log: log.to_string(),
        }
    }
}

/// Ergebnis eines build/test-Aufrufs — ToolEvidence (Fakt, keine
/// Interpretation), egal ob Fixture (Bau-Default) oder real (Feature
/// `process`).
#[derive(Debug, Clone)]
pub struct ToolExecOutput {
    pub exit_code: i32,
    pub log: String,
}

/// Aufzeichnung eines Tool-Aufrufs (Replay-Grundlage).
#[derive(Debug, Clone)]
pub struct ToolCallRecord {
    pub tool_id: String,
    pub operation: String,
    pub argument: String,
    pub result_digest: String,
}

/// Das Gateway: haelt je Tool-Klasse einen eigenen Lock.
#[derive(Default)]
pub struct ToolGateway {
    locks: BTreeMap<String, CapabilityLock>,
    pub records: Vec<ToolCallRecord>,
    pub calls_used: BTreeMap<String, u32>,
}

impl ToolGateway {
    pub fn new() -> Self {
        Self::default()
    }

    /// Lock-Oeffnung ist eine MATERIELLE Aktion: Operator + Ledger-Ref
    /// verpflichtend (S13-A7); nur fuer GENAU EINE Klasse.
    pub fn open_lock(&mut self, tool_class: &str, operator: &str, ledger_ref: &str) {
        let mut lock = CapabilityLock::closed(&format!("tool_egress:{tool_class}"));
        lock.open(operator, ledger_ref);
        self.locks.insert(tool_class.to_string(), lock);
    }

    /// Oeffentliche Lock-Abfrage (nur lesend): erlaubt Aufrufern (z. B.
    /// der SWE-Kern-Kette, P2) den Lock-Zustand in eine eigene
    /// GateReport-Kette zu komponieren, ohne den `CapabilityLock` selbst
    /// preiszugeben (der bleibt privat, keine Sammel-Freigabe).
    pub fn is_locked(&self, tool_class: &str) -> bool {
        self.lock_open(tool_class)
    }

    fn lock_open(&self, tool_class: &str) -> bool {
        self.locks
            .get(tool_class)
            .map(|l| l.is_open())
            .unwrap_or(false)
    }

    /// fs_read-Ausfuehrung durch das Gateway.
    pub fn run_fs_read(
        &mut self,
        manifest: &ToolManifest,
        tool: &FsReadTool,
        path: &str,
    ) -> Result<Vec<u8>, Box<Residue>> {
        if let Err(e) = manifest.validate() {
            return Err(Box::new(tool_residue(
                "tool_manifest_missing",
                &format!("Pflichtfelder fehlen: {:?}", e.missing),
            )));
        }
        if manifest.tool_class != "fs_read" {
            return Err(Box::new(tool_residue(
                "tool_class_mismatch",
                &format!("Manifest-Klasse {} ≠ fs_read", manifest.tool_class),
            )));
        }
        // Klassen-eigener Lock — niemals implizit frei (N-INF-10).
        if !self.lock_open("fs_read") {
            return Err(Box::new(tool_residue(
                "tool_egress_without_tool_capability",
                "fs_read ohne offenen ToolCapabilityLock — keine implizite Freigabe",
            )));
        }
        // Remote braeuchte zusaetzlich das ToolEgressGate.
        if manifest.egress == Egress::Remote {
            return Err(Box::new(tool_residue(
                "tool_egress_blocked",
                "fs_read ist lokal; Remote-Deklaration inkonsistent",
            )));
        }
        // Scope-Pruefung: Pfad muss unter einem deklarierten Scope liegen.
        if !manifest.scope.iter().any(|s| path.starts_with(s)) {
            return Err(Box::new(tool_residue(
                "tool_scope_violation",
                &format!("Pfad {path} ausserhalb des deklarierten Scopes"),
            )));
        }
        // Budget.
        let used = self.calls_used.entry(manifest.tool_id.clone()).or_insert(0);
        if *used >= manifest.budget_calls {
            return Err(Box::new(tool_residue(
                "tool_budget_exceeded",
                &format!("Budget {} Aufrufe erschoepft", manifest.budget_calls),
            )));
        }
        *used += 1;
        let bytes = tool.files.get(path).cloned().ok_or_else(|| {
            Box::new(tool_residue(
                "tool_target_missing",
                &format!("{path} nicht in der eingefrorenen Sicht"),
            ))
        })?;
        // Aufzeichnung (Replay-Grundlage).
        self.records.push(ToolCallRecord {
            tool_id: manifest.tool_id.clone(),
            operation: "fs_read".to_string(),
            argument: path.to_string(),
            result_digest: cce_core::signature::sha256(&bytes).to_hex(),
        });
        Ok(bytes)
    }

    /// fs_write-Ausfuehrung: schreibt NUR in die deklarierte Arbeitskopie,
    /// nie ausserhalb des Scopes (dieselbe Reihenfolge wie run_fs_read).
    pub fn run_fs_write(
        &mut self,
        manifest: &ToolManifest,
        tool: &mut FsWriteTool,
        path: &str,
        content: &[u8],
    ) -> Result<(), Box<Residue>> {
        if let Err(e) = manifest.validate() {
            return Err(Box::new(tool_residue(
                "tool_manifest_missing",
                &format!("Pflichtfelder fehlen: {:?}", e.missing),
            )));
        }
        if manifest.tool_class != "fs_write" {
            return Err(Box::new(tool_residue(
                "tool_class_mismatch",
                &format!("Manifest-Klasse {} ≠ fs_write", manifest.tool_class),
            )));
        }
        if !self.lock_open("fs_write") {
            return Err(Box::new(tool_residue(
                "tool_egress_without_tool_capability",
                "fs_write ohne offenen ToolCapabilityLock — keine implizite Freigabe",
            )));
        }
        if manifest.egress == Egress::Remote {
            return Err(Box::new(tool_residue(
                "tool_egress_blocked",
                "fs_write ist lokal; Remote-Deklaration inkonsistent",
            )));
        }
        if !manifest.scope.iter().any(|s| path.starts_with(s)) {
            return Err(Box::new(tool_residue(
                "tool_scope_violation",
                &format!("Pfad {path} ausserhalb des deklarierten Arbeitsbereichs"),
            )));
        }
        let used = self.calls_used.entry(manifest.tool_id.clone()).or_insert(0);
        if *used >= manifest.budget_calls {
            return Err(Box::new(tool_residue(
                "tool_budget_exceeded",
                &format!("Budget {} Aufrufe erschoepft", manifest.budget_calls),
            )));
        }
        *used += 1;
        tool.files.insert(path.to_string(), content.to_vec());
        self.records.push(ToolCallRecord {
            tool_id: manifest.tool_id.clone(),
            operation: "fs_write".to_string(),
            argument: path.to_string(),
            result_digest: cce_core::signature::sha256(content).to_hex(),
        });
        Ok(())
    }

    /// git-Ausfuehrung: `commit`/`push` verlangen zusaetzlich
    /// `confirmation_ref` (materielle Aktion, HumanConfirmationGate-
    /// Aufzeichnung); `push` verlangt zusaetzlich `Egress::Remote`.
    pub fn run_git(
        &mut self,
        manifest: &ToolManifest,
        tool: &mut GitTool,
        op: &GitOperation,
        confirmation_ref: Option<&str>,
    ) -> Result<String, Box<Residue>> {
        if let Err(e) = manifest.validate() {
            return Err(Box::new(tool_residue(
                "tool_manifest_missing",
                &format!("Pflichtfelder fehlen: {:?}", e.missing),
            )));
        }
        if manifest.tool_class != "git" {
            return Err(Box::new(tool_residue(
                "tool_class_mismatch",
                &format!("Manifest-Klasse {} ≠ git", manifest.tool_class),
            )));
        }
        if !self.lock_open("git") {
            return Err(Box::new(tool_residue(
                "tool_egress_without_tool_capability",
                "git ohne offenen ToolCapabilityLock — keine implizite Freigabe",
            )));
        }
        if matches!(op, GitOperation::Push) && manifest.egress != Egress::Remote {
            return Err(Box::new(tool_residue(
                "tool_egress_blocked",
                "push ohne Remote-Deklaration — kein Egress ohne ToolEgressGate",
            )));
        }
        if let GitOperation::Add(path) = op {
            if !manifest.scope.iter().any(|s| path.starts_with(s)) {
                return Err(Box::new(tool_residue(
                    "tool_scope_violation",
                    &format!("Pfad {path} ausserhalb des deklarierten Arbeitsbereichs"),
                )));
            }
        }
        if matches!(op, GitOperation::Commit(_) | GitOperation::Push) {
            match confirmation_ref {
                Some(r) if !r.is_empty() => {}
                _ => return Err(Box::new(tool_residue(
                    "tool_commit_without_confirmation",
                    "materielle Git-Aktion (commit/push) ohne aufgezeichnete Operator-Bestaetigung",
                ))),
            }
        }
        let used = self.calls_used.entry(manifest.tool_id.clone()).or_insert(0);
        if *used >= manifest.budget_calls {
            return Err(Box::new(tool_residue(
                "tool_budget_exceeded",
                &format!("Budget {} Aufrufe erschoepft", manifest.budget_calls),
            )));
        }
        *used += 1;
        let result = self.exec_git(manifest, tool, op)?;
        self.records.push(ToolCallRecord {
            tool_id: manifest.tool_id.clone(),
            operation: format!("git:{op:?}"),
            argument: match op {
                GitOperation::Add(p) => p.clone(),
                GitOperation::Commit(m) => m.clone(),
                GitOperation::Branch(b) => b.clone(),
                GitOperation::Status | GitOperation::Diff | GitOperation::Push => String::new(),
            },
            result_digest: cce_core::signature::sha256(result.as_bytes()).to_hex(),
        });
        Ok(result)
    }

    #[cfg(not(feature = "process"))]
    fn exec_git(
        &self,
        _manifest: &ToolManifest,
        tool: &mut GitTool,
        op: &GitOperation,
    ) -> Result<String, Box<Residue>> {
        Ok(match op {
            GitOperation::Status => tool.fixture_status.clone(),
            GitOperation::Diff => tool.fixture_diff.clone(),
            GitOperation::Add(path) => {
                tool.staged.push(path.clone());
                format!("staged:{path}")
            }
            GitOperation::Commit(msg) => {
                tool.commits.push(msg.clone());
                format!("commit:{msg}")
            }
            GitOperation::Branch(name) => {
                tool.branches.push(name.clone());
                format!("branch:{name}")
            }
            GitOperation::Push => {
                tool.pushed = true;
                "pushed".to_string()
            }
        })
    }

    /// Reale Ausfuehrung NUR unter Feature `process`: typisierter
    /// `git`-Aufruf je Operation (kein Shell-Sammelzugriff, §3 Verbot),
    /// in der deklarierten Arbeitskopie (`manifest.scope[0]`).
    #[cfg(feature = "process")]
    fn exec_git(
        &self,
        manifest: &ToolManifest,
        _tool: &mut GitTool,
        op: &GitOperation,
    ) -> Result<String, Box<Residue>> {
        let cwd = manifest.scope[0].clone();
        let argv: Vec<String> = match op {
            GitOperation::Status => vec!["git".into(), "status".into(), "--porcelain".into()],
            GitOperation::Diff => vec!["git".into(), "diff".into()],
            GitOperation::Add(path) => vec!["git".into(), "add".into(), path.clone()],
            GitOperation::Commit(msg) => {
                vec!["git".into(), "commit".into(), "-m".into(), msg.clone()]
            }
            GitOperation::Branch(name) => vec!["git".into(), "branch".into(), name.clone()],
            GitOperation::Push => vec!["git".into(), "push".into()],
        };
        run_typed_process(&argv, &cwd).map(|o| o.log)
    }

    /// build-Ausfuehrung. Bau-Default: Fixture (hermetisch); Feature
    /// `process`: typisierter Programmaufruf in `manifest.scope[0]`.
    pub fn run_build(
        &mut self,
        manifest: &ToolManifest,
        tool: &BuildTool,
    ) -> Result<ToolExecOutput, Box<Residue>> {
        self.run_exec_class(
            "build",
            manifest,
            &tool.argv,
            tool.fixture_exit_code,
            &tool.fixture_log,
        )
    }

    /// test-Ausfuehrung. Bau-Default: Fixture (hermetisch); Feature
    /// `process`: typisierter Programmaufruf in `manifest.scope[0]`.
    pub fn run_test(
        &mut self,
        manifest: &ToolManifest,
        tool: &TestTool,
    ) -> Result<ToolExecOutput, Box<Residue>> {
        self.run_exec_class(
            "test",
            manifest,
            &tool.argv,
            tool.fixture_exit_code,
            &tool.fixture_log,
        )
    }

    /// Gemeinsamer Pfad fuer build/test: beide sind strukturell lokal,
    /// nicht-remote, budgetiert, klassen-eigen gelockt.
    fn run_exec_class(
        &mut self,
        class: &str,
        manifest: &ToolManifest,
        argv: &[String],
        fixture_exit_code: i32,
        fixture_log: &str,
    ) -> Result<ToolExecOutput, Box<Residue>> {
        if let Err(e) = manifest.validate() {
            return Err(Box::new(tool_residue(
                "tool_manifest_missing",
                &format!("Pflichtfelder fehlen: {:?}", e.missing),
            )));
        }
        if manifest.tool_class != class {
            return Err(Box::new(tool_residue(
                "tool_class_mismatch",
                &format!("Manifest-Klasse {} ≠ {class}", manifest.tool_class),
            )));
        }
        if !self.lock_open(class) {
            return Err(Box::new(tool_residue(
                "tool_egress_without_tool_capability",
                &format!("{class} ohne offenen ToolCapabilityLock — keine implizite Freigabe"),
            )));
        }
        if manifest.egress == Egress::Remote {
            return Err(Box::new(tool_residue(
                "tool_egress_blocked",
                &format!("{class} ist lokal; Remote-Deklaration inkonsistent"),
            )));
        }
        let used = self.calls_used.entry(manifest.tool_id.clone()).or_insert(0);
        if *used >= manifest.budget_calls {
            return Err(Box::new(tool_residue(
                "tool_budget_exceeded",
                &format!("Budget {} Aufrufe erschoepft", manifest.budget_calls),
            )));
        }
        *used += 1;
        let output = exec_typed(argv, &manifest.scope[0], fixture_exit_code, fixture_log)?;
        self.records.push(ToolCallRecord {
            tool_id: manifest.tool_id.clone(),
            operation: class.to_string(),
            argument: argv.join(" "),
            result_digest: cce_core::signature::sha256(output.log.as_bytes()).to_hex(),
        });
        Ok(output)
    }
}

/// Bau-Default: hermetische Fixture, kein Subprozess.
#[cfg(not(feature = "process"))]
fn exec_typed(
    _argv: &[String],
    _cwd: &str,
    fixture_exit_code: i32,
    fixture_log: &str,
) -> Result<ToolExecOutput, Box<Residue>> {
    Ok(ToolExecOutput {
        exit_code: fixture_exit_code,
        log: fixture_log.to_string(),
    })
}

/// Feature `process`: real, aber typisiert (kein `sh -c`, §3 Verbot).
#[cfg(feature = "process")]
fn exec_typed(
    argv: &[String],
    cwd: &str,
    _fixture_exit_code: i32,
    _fixture_log: &str,
) -> Result<ToolExecOutput, Box<Residue>> {
    run_typed_process(argv, cwd)
}

/// Der EINE reale Subprozess-Pfad (Feature `process`): typisierter
/// `Command`-Aufruf, kein Shell-Sammelzugriff, kein freier Text.
#[cfg(feature = "process")]
fn run_typed_process(argv: &[String], cwd: &str) -> Result<ToolExecOutput, Box<Residue>> {
    if argv.is_empty() {
        return Err(Box::new(tool_residue(
            "tool_argv_empty",
            "typisierter Programmaufruf ist leer",
        )));
    }
    let output = std::process::Command::new(&argv[0])
        .args(&argv[1..])
        .current_dir(cwd)
        .output()
        .map_err(|e| {
            Box::new(tool_residue(
                "tool_process_spawn_failed",
                &format!("{argv:?} in {cwd}: {e}"),
            ))
        })?;
    let mut log = String::from_utf8_lossy(&output.stdout).into_owned();
    log.push_str(&String::from_utf8_lossy(&output.stderr));
    Ok(ToolExecOutput {
        exit_code: output.status.code().unwrap_or(-1),
        log,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn manifest() -> ToolManifest {
        ToolManifest {
            tool_id: "fsr-1".to_string(),
            tool_class: "fs_read".to_string(),
            scope: vec!["docs/".to_string()],
            side_effects: false,
            egress: Egress::None,
            budget_calls: 2,
            replay_strategy: "recorded".to_string(),
            lock_ref: "lock:fs_read".to_string(),
        }
    }

    #[test]
    fn no_lock_no_read_then_lock_opens_class_only() {
        let mut gw = ToolGateway::new();
        let tool = FsReadTool::with_files(&[("docs/a.md", b"inhalt")]);
        let m = manifest();
        let err = gw.run_fs_read(&m, &tool, "docs/a.md").unwrap_err();
        assert!(err.id.contains("tool_egress_without_tool_capability"));
        gw.open_lock("fs_read", "operator:sk", "ledger:e1");
        assert_eq!(gw.run_fs_read(&m, &tool, "docs/a.md").unwrap(), b"inhalt");
        // Der fs_read-Lock schaltet NICHT shell frei (keine Sammel-Freigabe).
        assert!(!gw.lock_open("shell"));
        assert_eq!(gw.records.len(), 1);
    }

    #[test]
    fn scope_and_budget_enforced() {
        let mut gw = ToolGateway::new();
        gw.open_lock("fs_read", "op", "l1");
        let tool = FsReadTool::with_files(&[("docs/a.md", b"x"), ("geheim/b.md", b"y")]);
        let m = manifest();
        let err = gw.run_fs_read(&m, &tool, "geheim/b.md").unwrap_err();
        assert!(err.id.contains("tool_scope_violation"));
        gw.run_fs_read(&m, &tool, "docs/a.md").unwrap();
        gw.run_fs_read(&m, &tool, "docs/a.md").unwrap();
        let err = gw.run_fs_read(&m, &tool, "docs/a.md").unwrap_err();
        assert!(err.id.contains("tool_budget_exceeded"));
    }

    fn manifest_class(tool_class: &str) -> ToolManifest {
        ToolManifest {
            tool_id: format!("{tool_class}-1"),
            tool_class: tool_class.to_string(),
            scope: vec!["repo/".to_string()],
            side_effects: true,
            egress: Egress::None,
            budget_calls: 5,
            replay_strategy: "recorded".to_string(),
            lock_ref: format!("lock:{tool_class}"),
        }
    }

    #[test]
    fn fs_write_scope_enforced_and_writes_arbeitskopie() {
        let mut gw = ToolGateway::new();
        let mut tool = FsWriteTool::default();
        let m = manifest_class("fs_write");
        let err = gw
            .run_fs_write(&m, &mut tool, "repo/src/lib.rs", b"fn f(){}")
            .unwrap_err();
        assert!(err.id.contains("tool_egress_without_tool_capability"));
        gw.open_lock("fs_write", "op", "l1");
        let err = gw
            .run_fs_write(&m, &mut tool, "aussen/x.rs", b"bad")
            .unwrap_err();
        assert!(err.id.contains("tool_scope_violation"));
        gw.run_fs_write(&m, &mut tool, "repo/src/lib.rs", b"fn f(){}")
            .unwrap();
        assert_eq!(tool.files.get("repo/src/lib.rs").unwrap(), b"fn f(){}");
    }

    // Fixture-Semantik (GitTool-Zustand wird direkt mutiert): gilt nur im
    // Bau-Default. Unter Feature `process` waere dies ein echter,
    // umgebungsabhaengiger Prozessaufruf — kein Bau-Zeuge.
    #[cfg(not(feature = "process"))]
    #[test]
    fn git_commit_and_push_need_confirmation_and_remote() {
        let mut gw = ToolGateway::new();
        let mut tool = GitTool::default();
        let m = manifest_class("git");
        gw.open_lock("git", "op", "l1");
        // commit ohne Bestaetigung: reject.
        let err = gw
            .run_git(&m, &mut tool, &GitOperation::Commit("fix".into()), None)
            .unwrap_err();
        assert!(err.id.contains("tool_commit_without_confirmation"));
        // commit MIT Bestaetigung: geht durch, materielle Aktion aufgezeichnet.
        gw.run_git(
            &m,
            &mut tool,
            &GitOperation::Commit("fix".into()),
            Some("operator:sk;ledger:e1"),
        )
        .unwrap();
        assert_eq!(tool.commits, vec!["fix".to_string()]);
        // push ohne Remote-Deklaration: reject, auch mit Bestaetigung.
        let err = gw
            .run_git(&m, &mut tool, &GitOperation::Push, Some("operator:sk"))
            .unwrap_err();
        assert!(err.id.contains("tool_egress_blocked"));
        // push MIT Remote-Deklaration + Bestaetigung: geht durch.
        let mut remote_manifest = manifest_class("git");
        remote_manifest.egress = Egress::Remote;
        gw.run_git(
            &remote_manifest,
            &mut tool,
            &GitOperation::Push,
            Some("operator:sk"),
        )
        .unwrap();
        assert!(tool.pushed);
    }

    // Fixture-Semantik (ToolExecOutput kommt aus Tool::fixture_*, kein
    // Subprozess): gilt nur im Bau-Default, s. Kommentar oben.
    #[cfg(not(feature = "process"))]
    #[test]
    fn build_and_test_are_local_fixture_evidence() {
        let mut gw = ToolGateway::new();
        let m = manifest_class("build");
        gw.open_lock("build", "op", "l1");
        let build = BuildTool::with_fixture(&["cargo", "build"], 0, "Compiling ok");
        let out = gw.run_build(&m, &build).unwrap();
        assert_eq!(out.exit_code, 0);
        assert!(out.log.contains("Compiling"));

        let mut tm = manifest_class("test");
        tm.tool_class = "test".to_string();
        gw.open_lock("test", "op", "l1");
        let test = TestTool::with_fixture(&["cargo", "test"], 101, "1 failed");
        let out = gw.run_test(&tm, &test).unwrap();
        assert_eq!(out.exit_code, 101);
        assert!(out.log.contains("failed"));
    }

    #[test]
    fn build_class_mismatch_and_remote_declaration_rejected() {
        let mut gw = ToolGateway::new();
        gw.open_lock("build", "op", "l1");
        let mut m = manifest_class("build");
        let build = BuildTool::with_fixture(&["cargo", "build"], 0, "ok");
        m.egress = Egress::Remote;
        let err = gw.run_build(&m, &build).unwrap_err();
        assert!(err.id.contains("tool_egress_blocked"));
    }
}
