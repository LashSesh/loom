//! Registry mit Eintritts-Selbstvalidierung (S8.2): KEIN unvalidiertes
//! Asset tritt ein. Referenz-Cubes muessen tatsaechlich schliessen,
//! Negativ-Cubes tatsaechlich (mit erwartetem Grund) abgelehnt werden.
//! Assets sind inhaltsadressiert, versioniert, mit Provenienz; Stilllegung
//! ist explizit, nie stilles Loeschen (S8.8).

use cce_core::canonical::Canonicalize;
use cce_core::residue::ResidueKind;
use cce_core::signature::Digest;
use cce_materialize::adapter::DomainAdapter;
use cce_materialize::document::{DocCrystal, DocumentAdapter};
use cce_phc::projection_calc::project;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetKind {
    ReferenceCube,
    NegativeCube,
    Template,
    Fragment,
    ExampleWish,
    /// S8-A1: neue Asset-Typen.
    CrystalWitness,
    PhaseBlockWitness,
    WorkbodyBlueprint,
    DomainNorm,
    /// Port-Typ, leer bis L9b (R-1b).
    BridgeNorm,
}

#[derive(Debug, Clone)]
pub struct LibraryAsset {
    pub kind: AssetKind,
    pub domain: String,
    pub name: String,
    pub class_digest: Digest,
    pub author: String,
    pub validation: String,
    pub retired: bool,
    /// Erwartetes Ablehnungs-Residuum (nur Negativ-Cubes).
    pub expected_residue: Option<ResidueKind>,
    pub crystal: DocCrystal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistryError {
    /// Referenz-Cube schliesst nicht wirklich.
    ReferenceDoesNotClose(String),
    /// Negativ-Cube wird nicht (mit erwartetem Grund) abgelehnt.
    NegativeNotRejected(String),
    /// „zahnloser" Negativ-Cube: abgelehnt, aber mit falschem Grund.
    WrongRejectionReason { expected: String, got: String },
}

#[derive(Debug, Default)]
pub struct Registry {
    assets: BTreeMap<String, LibraryAsset>,
}

/// Durchlaeuft den geschlossenen Pfad und liefert die Gate-Reports
/// (inkl. Round-Trip) fuer einen Kristall.
pub fn evaluate(crystal: &DocCrystal) -> Vec<cce_core::gate::GateReport> {
    let adapter = DocumentAdapter;
    let pkg = adapter.encode(crystal);
    let artifact = project(&pkg, "proj:materialize")
        .ok()
        .and_then(|proj| adapter.loom(&proj).ok())
        .map(|weave| adapter.materialize(&weave));
    adapter.run_domain_gates(crystal, artifact.as_ref())
}

impl Registry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Eintritt NUR nach Selbstvalidierung (S8.2).
    pub fn admit(
        &mut self,
        kind: AssetKind,
        name: &str,
        crystal: DocCrystal,
        author: &str,
        expected_residue: Option<ResidueKind>,
    ) -> Result<&LibraryAsset, RegistryError> {
        let reports = evaluate(&crystal);
        let all_pass = !reports.is_empty() && reports.iter().all(|r| r.is_pass());
        match kind {
            AssetKind::ReferenceCube | AssetKind::Template | AssetKind::Fragment => {
                if !all_pass {
                    let first = reports
                        .iter()
                        .find(|r| !r.is_pass())
                        .map(|r| format!("{}: {}", r.gate_id, r.reason))
                        .unwrap_or_default();
                    return Err(RegistryError::ReferenceDoesNotClose(first));
                }
            }
            AssetKind::NegativeCube => {
                let expected = expected_residue
                    .as_ref()
                    .map(|k| k.as_str().to_string())
                    .unwrap_or_default();
                let holds: Vec<&cce_core::gate::GateReport> =
                    reports.iter().filter(|r| !r.is_pass()).collect();
                if holds.is_empty() {
                    return Err(RegistryError::NegativeNotRejected(name.to_string()));
                }
                if !holds.iter().any(|h| h.reason.contains(&expected)) {
                    return Err(RegistryError::WrongRejectionReason {
                        expected,
                        got: holds
                            .iter()
                            .map(|h| h.reason.clone())
                            .collect::<Vec<_>>()
                            .join("; "),
                    });
                }
            }
            _ => {}
        }
        let asset = LibraryAsset {
            kind,
            domain: "D01-document".to_string(),
            name: name.to_string(),
            class_digest: crystal.canonical_class().0,
            author: author.to_string(),
            validation: "selbstvalidiert (S8.2)".to_string(),
            retired: false,
            expected_residue,
            crystal,
        };
        self.assets.insert(name.to_string(), asset);
        Ok(self.assets.get(name).expect("just inserted"))
    }

    /// NUR fuer Waechter-Negativ-Tests: schleust ein Asset OHNE
    /// Selbstvalidierung ein (simuliert Bestands-Drift/Manipulation).
    #[doc(hidden)]
    pub fn inject_unvalidated_for_tests(&mut self, asset: LibraryAsset) {
        self.assets.insert(asset.name.clone(), asset);
    }

    /// Explizite Stilllegung — nie stilles Loeschen (S8.8).
    pub fn retire(&mut self, name: &str) -> bool {
        if let Some(a) = self.assets.get_mut(name) {
            a.retired = true;
            true
        } else {
            false
        }
    }

    pub fn assets(&self) -> impl Iterator<Item = &LibraryAsset> {
        self.assets.values()
    }

    pub fn get(&self, name: &str) -> Option<&LibraryAsset> {
        self.assets.get(name)
    }

    /// Die Saat-Bibliothek: Dokument-Referenz + Negativ-Cubes (S11.6).
    pub fn seed() -> Result<Registry, RegistryError> {
        let adapter = DocumentAdapter;
        let mut reg = Registry::new();
        reg.admit(
            AssetKind::ReferenceCube,
            "drei-risiken-memo",
            adapter.reference_cube(),
            "saat",
            None,
        )?;
        for (i, (cube, expected)) in adapter.negative_cubes().into_iter().enumerate() {
            reg.admit(
                AssetKind::NegativeCube,
                &format!("negativ-{}-{}", i + 1, expected.as_str()),
                cube,
                "saat",
                Some(expected),
            )?;
        }
        Ok(reg)
    }
}
