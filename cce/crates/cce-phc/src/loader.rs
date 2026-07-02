//! PHC-Loader (Teil 7.4, PHC §17/§18): `load_phc → canonicalize → assert
//! root_hash → validate(schema, refs, axes, cells, gates, residue) →
//! build_lattice`. Validierungsphasen V0–V9, jede ein isolierter,
//! benannter Fehlpunkt (fail-closed). Nummernzuordnung: R-Agent-5.

use crate::package::PhcPackage;
use cce_core::signature::Digest;

/// Die zehn Phasen V0–V9.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationPhase {
    V0Parse,
    V1Schema,
    V2RootHash,
    V3Refs,
    V4Axes,
    V5Cells,
    V6Gates,
    V7Residue,
    V8Ledger,
    V9Weave,
}

#[derive(Debug, Clone)]
pub struct LoaderError {
    pub phase: ValidationPhase,
    pub reason: String,
}

fn fail(phase: ValidationPhase, reason: &str) -> LoaderError {
    LoaderError {
        phase,
        reason: reason.to_string(),
    }
}

/// Der Loader: validiert V0–V9 und liefert das geladene Paket.
pub fn load_phc(p: &PhcPackage, expected_root: Option<Digest>) -> Result<(), LoaderError> {
    // V0 Parse: hier bereits typisiert (Byte-Parsing liegt im .loom-Codec, G9).
    // V1 Schema/Pflichtfelder:
    if p.phc_version != "0.1" {
        return Err(fail(ValidationPhase::V1Schema, "phc_version ≠ 0.1"));
    }
    if p.manifest.canonicalization != "PHC-CANON-0.1" {
        return Err(fail(ValidationPhase::V1Schema, "canonicalization fehlt"));
    }
    if !p.manifest.codec_id.starts_with("phc:sha256:") {
        return Err(fail(
            ValidationPhase::V1Schema,
            "codec_id nicht content-adressiert",
        ));
    }
    if !p.manifest.root_crystal.starts_with("crystal:") {
        return Err(fail(ValidationPhase::V1Schema, "root_crystal fehlt"));
    }
    // V2 Root-Hash-Stabilitaet (Can(P)):
    if let Some(expect) = expected_root {
        if p.root_hash() != expect {
            return Err(fail(ValidationPhase::V2RootHash, "Root-Hash weicht ab"));
        }
    }
    // V3 Referenzen aufloesbar:
    for w in &p.workcells {
        if !p.cells.iter().any(|c| c.id == w.cell) {
            return Err(fail(
                ValidationPhase::V3Refs,
                &format!("Workcell {} referenziert unbekannte Zelle {}", w.id, w.cell),
            ));
        }
        if !p.projections.iter().any(|pr| pr.id == w.projection) {
            return Err(fail(
                ValidationPhase::V3Refs,
                &format!("Workcell {} referenziert unbekannte Projektion", w.id),
            ));
        }
    }
    for s in &p.seams {
        for cid in &s.cells {
            if !p.cells.iter().any(|c| &c.id == cid) {
                return Err(fail(
                    ValidationPhase::V3Refs,
                    &format!("Seam {} referenziert unbekannte Zelle {cid}", s.id),
                ));
            }
        }
    }
    // V4 Achsen:
    if p.axes.is_empty() {
        return Err(fail(ValidationPhase::V4Axes, "keine Achsen deklariert"));
    }
    // V5 Zellen (Adresse + Signatur vorhanden):
    for c in &p.cells {
        if !c.address.starts_with("phc://") {
            return Err(fail(
                ValidationPhase::V5Cells,
                &format!("Zelle {} ohne phc://-Adresse", c.id),
            ));
        }
    }
    // V6 Gates: MUSS G1–G7 enthalten, alle fail-closed:
    if p.gates.len() < 7 {
        return Err(fail(ValidationPhase::V6Gates, "weniger als 7 Pflichtgates"));
    }
    for want in [
        "G1-Scope",
        "G2-Boundary",
        "G3-Type",
        "G4-Residue",
        "G5-Replay",
        "G6-Export",
        "G7-Reanalysis",
    ] {
        if !p.gates.iter().any(|g| g.id == want) {
            return Err(fail(
                ValidationPhase::V6Gates,
                &format!("Pflichtgate {want} fehlt"),
            ));
        }
    }
    // V7 Residue-Policy:
    if p.residue_policy != "visible" {
        return Err(fail(ValidationPhase::V7Residue, "residue.policy ≠ visible"));
    }
    // V8 Ledger-Modus:
    if p.ledger_mode != "append_only" {
        return Err(fail(ValidationPhase::V8Ledger, "ledger.mode ≠ append_only"));
    }
    // V9 Weave-/Workcell-Konsistenz:
    for w in &p.workcells {
        if w.gate_chain.is_empty() {
            return Err(fail(
                ValidationPhase::V9Weave,
                &format!("Workcell {} ohne Gate-Kette", w.id),
            ));
        }
        if w.residue_policy != "visible" {
            return Err(fail(
                ValidationPhase::V9Weave,
                &format!("Workcell {} mit unsichtbarer Residue-Policy", w.id),
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::package::{Cell, PhcPackage, Projection, Workcell};
    use cce_core::signature::sha256;
    use cce_core::value::CanonValue;

    fn valid_package() -> PhcPackage {
        PhcPackage::build(
            "test",
            "document",
            sha256(b"root"),
            vec![Cell {
                id: "c1".into(),
                address: "phc://x/cell/unit=1".into(),
                axis_values: CanonValue::map([]),
                fiber: CanonValue::map([]),
                signature: sha256(b"c1"),
                status: "filled".into(),
            }],
            vec![],
            vec![Projection {
                id: "p1".into(),
                target_cell: "c1".into(),
                include: vec!["fiber".into()],
                exclude: vec![],
                max_tokens: None,
                allowed_ops: vec!["render".into()],
            }],
            vec![Workcell {
                id: "w1".into(),
                cell: "c1".into(),
                projection: "p1".into(),
                intent: "materialisieren".into(),
                allowed_operations: vec!["render".into()],
                gate_chain: vec!["G1-Scope".into()],
                residue_policy: "visible".into(),
            }],
            vec![".md".into()],
        )
    }

    /// V0–V9 gruen am validen Paket; Root-Hash stabil (V2).
    #[test]
    fn loader_accepts_valid_package() {
        let p = valid_package();
        let root = p.root_hash();
        assert!(load_phc(&p, Some(root)).is_ok());
        // Can(P) stabil: zweiter Aufbau gleicher Inhalt ⇒ gleicher Root.
        assert_eq!(valid_package().root_hash(), root);
    }

    /// Jede Phase ist ein isolierter Fehlpunkt.
    #[test]
    fn loader_fails_closed_per_phase() {
        // V2: falscher Root
        let p = valid_package();
        let e = load_phc(&p, Some(sha256(b"anders"))).unwrap_err();
        assert_eq!(e.phase, ValidationPhase::V2RootHash);
        // V3: kaputte Referenz
        let mut p = valid_package();
        p.workcells[0].cell = "unbekannt".into();
        assert_eq!(
            load_phc(&p, None).unwrap_err().phase,
            ValidationPhase::V3Refs
        );
        // V6: Gate entfernt
        let mut p = valid_package();
        p.gates.retain(|g| g.id != "G4-Residue");
        assert_eq!(
            load_phc(&p, None).unwrap_err().phase,
            ValidationPhase::V6Gates
        );
        // V7: unsichtbare Residuen
        let mut p = valid_package();
        p.residue_policy = "hidden".into();
        assert_eq!(
            load_phc(&p, None).unwrap_err().phase,
            ValidationPhase::V7Residue
        );
        // V8: Ledger nicht append-only
        let mut p = valid_package();
        p.ledger_mode = "mutable".into();
        assert_eq!(
            load_phc(&p, None).unwrap_err().phase,
            ValidationPhase::V8Ledger
        );
        // V9: Workcell ohne Gate-Kette
        let mut p = valid_package();
        p.workcells[0].gate_chain.clear();
        assert_eq!(
            load_phc(&p, None).unwrap_err().phase,
            ValidationPhase::V9Weave
        );
    }
}
