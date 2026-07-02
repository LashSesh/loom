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
}
