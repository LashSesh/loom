//! ToolManifest (C.10) — deklarativ; Deklaration ≠ Aktivierung.

/// Die elf Tool-Klassen (C.10 + P2/Dokument 18 §3: build/test additiv) —
/// je Klasse ein eigener Lock.
pub const TOOL_CLASSES: [&str; 11] = [
    "fs_read",
    "fs_write",
    "shell",
    "git",
    "package_manager",
    "browser",
    "ci",
    "network_tool",
    "custom",
    "build",
    "test",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Egress {
    None,
    Remote,
}

#[derive(Debug, Clone)]
pub struct ToolManifest {
    pub tool_id: String,
    pub tool_class: String,
    pub scope: Vec<String>,
    pub side_effects: bool,
    pub egress: Egress,
    pub budget_calls: u32,
    pub replay_strategy: String,
    pub lock_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolManifestError {
    pub missing: Vec<&'static str>,
}

impl ToolManifest {
    pub fn validate(&self) -> Result<(), ToolManifestError> {
        let mut missing = Vec::new();
        if self.tool_id.is_empty() {
            missing.push("tool_id");
        }
        if !TOOL_CLASSES.contains(&self.tool_class.as_str()) {
            missing.push("tool_class");
        }
        if self.scope.is_empty() {
            missing.push("scope");
        }
        if self.budget_calls == 0 {
            missing.push("budget");
        }
        if self.replay_strategy.is_empty() {
            missing.push("replay_strategy");
        }
        if self.lock_ref.is_empty() {
            missing.push("lock_ref");
        }
        if missing.is_empty() {
            Ok(())
        } else {
            Err(ToolManifestError { missing })
        }
    }
}
