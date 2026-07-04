//! §8(c): RepoSnapshot auf eine klar benannte, KLEINE Zielstelle
//! beschraenkt — keine Vollspiegelung des Workspace. `cce_swe::model
//! ::RepoSnapshot::from_files` nimmt ohnehin nur explizit benannte
//! Dateien entgegen (kein Verzeichnis-Scan existiert irgendwo in
//! `cce-swe`); diese Funktion fuegt zusaetzlich die STRUKTURELLE
//! Garantie hinzu, dass jede eingehende Datei innerhalb des vom
//! Auftraggeber freigegebenen `TaskProposal::target_scope` liegt —
//! nichts kann in den Snapshot gelangen, das nicht Teil des
//! freigegebenen Vorschlags ist.

use crate::residues::dogfood_residue;
use cce_core::residue::Residue;
use cce_swe::model::{CodeUnitRole, RepoSnapshot};

/// Baut einen RepoSnapshot NUR aus Dateien, die innerhalb `scope`
/// liegen — jede Datei ausserhalb ⇒ `protected_path_violation`
/// (dieselbe Kategorie wie das ProtectedPathGate: ein Snapshot, der
/// Material ausserhalb des freigegebenen Scopes traegt, IST eine
/// Schutzzonenverletzung, unabhaengig davon, ob der Pfad selbst in der
/// harten Negativliste steht).
pub fn snapshot_for_scope(
    scope: &[String],
    files: &[(&str, &str, CodeUnitRole, &[u8])],
    toolchain_pin: &str,
    commit_ref: Option<&str>,
) -> Result<RepoSnapshot, Box<Residue>> {
    for (path, ..) in files {
        if !scope.iter().any(|s| path.starts_with(s.as_str())) {
            return Err(Box::new(dogfood_residue(
                "protected_path_violation",
                &format!("Datei '{path}' liegt ausserhalb des freigegebenen target_scope"),
            )));
        }
    }
    Ok(RepoSnapshot::from_files(files, toolchain_pin, commit_ref))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot_rejects_file_outside_scope() {
        let scope = vec!["cce/crates/cce-swe/".to_string()];
        let err = snapshot_for_scope(
            &scope,
            &[(
                "cce/crates/cce-core/src/gate.rs",
                "rust",
                CodeUnitRole::Source,
                b"x",
            )],
            "rustc-1.0",
            None,
        )
        .unwrap_err();
        assert!(err.id.contains("protected_path_violation"));
    }

    #[test]
    fn snapshot_accepts_small_named_target() {
        let scope = vec!["cce/crates/cce-swe/README.md".to_string()];
        let snap = snapshot_for_scope(
            &scope,
            &[(
                "cce/crates/cce-swe/README.md",
                "markdown",
                CodeUnitRole::Doc,
                b"# cce-swe\n",
            )],
            "rustc-1.0",
            None,
        )
        .expect("Datei innerhalb des Scopes wird akzeptiert");
        assert_eq!(snap.units.len(), 1);
    }
}
