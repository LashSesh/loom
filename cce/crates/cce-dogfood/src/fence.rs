//! ProtectedPathFence (Dokument 19 §2/§3): eine geprüfte Funktion
//! `path_allowed(path) -> bool` gegen die Schutzzone — jeder Treffer
//! außerhalb ⇒ `protected_path_violation`, harter Reject VOR jedem
//! Apply. Repo-relative Pfade (ausgehend vom Git-Wurzelverzeichnis,
//! das `cce/` neben `cce-spec-repo/`/`reports/` enthält).
//!
//! Konservative Auslegung (Residuum, s. `reports/residuen.md`): §2
//! nennt "alle Gate-tragenden Kern-Crates" und "alle
//! Gate-Definitionen in cce-inference/cce-toolgateway" — `cce-toolgateway`
//! hat keine eigene `gates.rs` (seine Durchsetzung ist in
//! `gateway.rs` inline verwoben, s. P2), daher wird hier der GESAMTE
//! Crate-Baum beider Crates geschuetzt statt nur einzelner Dateien
//! (Sicherheit-zuerst-Prinzip, §1: lieber zu viel schuetzen als zu
//! wenig). Diese Fence schuetzt zusaetzlich sich selbst
//! (`cce-dogfood`) UND `cce-swe`/`cce-phaseblock`-benachbarte
//! Sicherheitsinfrastruktur, obwohl `cce-swe` in §2 nicht namentlich
//! genannt ist — die Dogfooding-Infrastruktur darf nicht ihr eigenes
//! Ziel sein.

/// Repo-relative Pfad-Praefixe, die JEDER TaskProposal-Scope und JEDER
/// tatsaechliche Diff-Hunk-Pfad respektieren muss.
pub const PROTECTED_PATH_PREFIXES: &[&str] = &[
    // §2 woertlich:
    "spec/",
    "cce-spec-repo/",
    "cce/crates/cce-core/",
    "cce/crates/cce-lattice/",
    "cce/crates/cce-ccc/",
    "cce/crates/cce-phaseblock/",
    "cce/crates/cce-crystal/",
    "cce/crates/cce-kernel/",
    "cce/crates/cce-spiral/",
    "cce/crates/cce-hbm/",
    "cce/crates/cce-bridge/",
    "cce/crates/cce-inference/",
    "cce/crates/cce-toolgateway/",
    "cce/loom/loom-format/",
    "cce/loom/loom-codec/",
    "cce/loom/loom-verify/",
    "cce/ci/",
    // Konservative Ergaenzung (s. Moduldoc oben): die
    // Dogfooding-Infrastruktur selbst + ihr unmittelbarer Kern-Nachbar.
    "cce/crates/cce-dogfood/",
    "cce/crates/cce-swe/src/kette.rs",
    "cce/crates/cce-swe/src/gates.rs",
    "cce/crates/cce-swe/src/workbody.rs",
];

/// Exakte (nicht praefixbasierte) verbotene Dateien — jede
/// Workspace-Cargo.toml (§2 woertlich).
pub const PROTECTED_EXACT_FILES: &[&str] = &["cce/Cargo.toml"];

/// Prueft EINEN Pfad gegen die Schutzzone. `true` = erlaubt.
pub fn path_allowed(path: &str) -> bool {
    if PROTECTED_EXACT_FILES.contains(&path) {
        return false;
    }
    !PROTECTED_PATH_PREFIXES.iter().any(|p| path.starts_with(p))
}

/// Prueft eine ganze Scope-Liste; liefert den ERSTEN verbotenen Pfad
/// (falls einer existiert) fuer eine sprechende Fehlermeldung.
pub fn first_forbidden(scope: &[String]) -> Option<&str> {
    scope.iter().find(|p| !path_allowed(p)).map(String::as_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn protected_paths_are_rejected() {
        assert!(!path_allowed("cce-spec-repo/18 P2 SWE TIEFE SPEC.md"));
        assert!(!path_allowed("cce/crates/cce-core/src/gate.rs"));
        assert!(!path_allowed("cce/crates/cce-inference/src/gates.rs"));
        assert!(!path_allowed("cce/loom/loom-verify/src/lib.rs"));
        assert!(!path_allowed("cce/ci/run_ci.sh"));
        assert!(!path_allowed("cce/Cargo.toml"));
        assert!(!path_allowed("cce/crates/cce-dogfood/src/fence.rs"));
    }

    #[test]
    fn leaf_crate_paths_are_allowed() {
        assert!(path_allowed("cce/crates/cce-swe/src/model.rs"));
        assert!(path_allowed("cce/crates/cce-swe/README.md"));
        assert!(path_allowed("cce/conformance/swe/swe_catalog.rs"));
    }

    #[test]
    fn first_forbidden_reports_the_violation() {
        let scope = vec![
            "cce/crates/cce-swe/src/model.rs".to_string(),
            "cce/crates/cce-core/src/gate.rs".to_string(),
        ];
        assert_eq!(
            first_forbidden(&scope),
            Some("cce/crates/cce-core/src/gate.rs")
        );
    }
}
