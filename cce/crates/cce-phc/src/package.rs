//! PHC-Paket (Teil 3.5 / 7.1.1): PHC = (M, T, A, C, W, P, G, R, L, E).
//! Kanonisierung PHC-CANON-0.1 = kanonisches Wertemodell + SHA-256 (cce-core).

use cce_core::canonical::Canonicalize;
use cce_core::gate::Gate;
use cce_core::objects::Seam;
use cce_core::signature::Digest;
use cce_core::value::CanonValue;

/// Manifest M (PHC §5/§6): Identitaet, Version, Root-Hash.
#[derive(Debug, Clone)]
pub struct PhcManifest {
    pub codec_id: String,
    pub title: String,
    pub status: String,
    pub domain_mode: String,
    pub root_crystal: String,
    pub canonicalization: String,
    pub license_policy: String,
}

/// Zelle mit Adresse `phc://<codec_id>/cell/<axis=value>/…` (G-12).
#[derive(Debug, Clone)]
pub struct Cell {
    pub id: String,
    pub address: String,
    pub axis_values: CanonValue,
    pub fiber: CanonValue,
    pub signature: Digest,
    pub status: String,
}

/// Projektionsprofil π_c (G-14): kontrollierte Reduktion.
#[derive(Debug, Clone)]
pub struct Projection {
    pub id: String,
    pub target_cell: String,
    pub include: Vec<String>,
    pub exclude: Vec<String>,
    pub max_tokens: Option<u64>,
    pub allowed_ops: Vec<String>,
}

/// Workcell ω (G-06).
#[derive(Debug, Clone)]
pub struct Workcell {
    pub id: String,
    pub cell: String,
    pub projection: String,
    pub intent: String,
    pub allowed_operations: Vec<String>,
    pub gate_chain: Vec<String>,
    pub residue_policy: String,
}

/// Das PHC-Paket (Pflichtoberflaeche phc.schema.json, Teil 7.1.1).
#[derive(Debug, Clone)]
pub struct PhcPackage {
    pub phc_version: String,
    pub manifest: PhcManifest,
    pub axes: Vec<(String, String)>,
    pub cells: Vec<Cell>,
    pub seams: Vec<Seam>,
    pub projections: Vec<Projection>,
    pub workcells: Vec<Workcell>,
    pub gates: Vec<Gate>,
    pub residue_policy: String,
    pub ledger_mode: String,
    pub exports: Vec<String>,
}

impl PhcPackage {
    /// Baut ein Paket um einen kanonischen Kristallkern; codec_id ist
    /// content-adressiert (P10), die 7 Pflichtgates sind gebunden.
    /// (Argumentzahl folgt dem PHC-Tupel (M,T,A,C,W,P,G,R,L,E), Teil 3.5.)
    #[allow(clippy::too_many_arguments)]
    pub fn build(
        title: &str,
        domain_mode: &str,
        root_crystal_class: Digest,
        cells: Vec<Cell>,
        seams: Vec<Seam>,
        projections: Vec<Projection>,
        workcells: Vec<Workcell>,
        exports: Vec<String>,
    ) -> Self {
        let content_class = CanonValue::map([
            ("title", CanonValue::text(title)),
            ("root", CanonValue::Bytes(root_crystal_class.0.to_vec())),
        ])
        .canonical_class();
        PhcPackage {
            phc_version: "0.1".to_string(),
            manifest: PhcManifest {
                codec_id: format!("phc:sha256:{}", content_class.0.to_hex()),
                title: title.to_string(),
                status: "sealed".to_string(),
                domain_mode: domain_mode.to_string(),
                root_crystal: format!("crystal:{}", root_crystal_class),
                canonicalization: "PHC-CANON-0.1".to_string(),
                license_policy: "internal".to_string(),
            },
            axes: vec![
                ("semantic".to_string(), "unit_type".to_string()),
                ("material".to_string(), "export_format".to_string()),
            ],
            cells,
            seams,
            projections,
            workcells,
            gates: cce_core::gate::mandatory_gates(),
            residue_policy: "visible".to_string(),
            ledger_mode: "append_only".to_string(),
            exports,
        }
    }

    /// Root-Hash des Pakets (Can(P), PHC V2).
    pub fn root_hash(&self) -> Digest {
        self.canonical_class().0
    }
}

impl Canonicalize for PhcPackage {
    fn canonical_value(&self) -> CanonValue {
        CanonValue::map([
            ("phc_version", CanonValue::text(&self.phc_version)),
            ("title", CanonValue::text(&self.manifest.title)),
            ("domain_mode", CanonValue::text(&self.manifest.domain_mode)),
            (
                "root_crystal",
                CanonValue::text(&self.manifest.root_crystal),
            ),
            (
                "cells",
                CanonValue::List(
                    self.cells
                        .iter()
                        .map(|c| {
                            CanonValue::map([
                                ("id", CanonValue::text(&c.id)),
                                ("axis_values", c.axis_values.clone()),
                                ("fiber", c.fiber.clone()),
                            ])
                        })
                        .collect(),
                ),
            ),
            (
                "seams",
                CanonValue::List(
                    self.seams
                        .iter()
                        .map(|s| {
                            CanonValue::map([
                                ("id", CanonValue::text(&s.id)),
                                (
                                    "cells",
                                    CanonValue::List(
                                        s.cells.iter().map(CanonValue::text).collect(),
                                    ),
                                ),
                                ("rule", CanonValue::text(&s.rule)),
                            ])
                        })
                        .collect(),
                ),
            ),
            (
                "exports",
                CanonValue::List(self.exports.iter().map(CanonValue::text).collect()),
            ),
        ])
    }
}
