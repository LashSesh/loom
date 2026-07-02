//! Dokument-Domaene (S1): Objektmodell, Kanonisierung (das ≃, S1.6),
//! Adapter-Implementierung (Referenz der Adapter-Paritaet).

pub mod assets;
pub mod gates;
pub mod parse;
pub mod render;

use crate::adapter::{DomainAdapter, OpenAction};
use cce_core::canonical::{CanonicalClass, CanonicalState, Canonicalize};
use cce_core::gate::{Gate, GateKind, GateReport};
use cce_core::objects::CounterHorizon;
use cce_core::residue::{Residue, ResidueKind, Severity};
use cce_core::signature::sha256;
use cce_core::value::CanonValue;
use cce_loom::weave::{weave, Weave};
use cce_phc::package::{Cell, PhcPackage, Projection, Workcell};
use cce_phc::projection_calc::LocalProjection;

/// Einheitstypen (S1.10-R2: additiv erweiterbar).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitType {
    Section,
    Claim,
    Support,
    Risk,
    Countermeasure,
    Definition,
    Step,
}

impl UnitType {
    pub fn as_str(self) -> &'static str {
        match self {
            UnitType::Section => "section",
            UnitType::Claim => "claim",
            UnitType::Support => "support",
            UnitType::Risk => "risk",
            UnitType::Countermeasure => "countermeasure",
            UnitType::Definition => "definition",
            UnitType::Step => "step",
        }
    }

    pub fn parse(s: &str) -> Option<UnitType> {
        Some(match s {
            "section" => UnitType::Section,
            "claim" => UnitType::Claim,
            "support" => UnitType::Support,
            "risk" => UnitType::Risk,
            "countermeasure" => UnitType::Countermeasure,
            "definition" => UnitType::Definition,
            "step" => UnitType::Step,
            _ => return None,
        })
    }

    /// Stuetzungspflichtige Typen (DocG-Support).
    pub fn requires_support(self) -> bool {
        matches!(self, UnitType::Risk | UnitType::Claim)
    }
}

/// DocUnit: semantische Einheit mit tripolarer Faser-Rolle (S1.1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocUnit {
    pub id: String,
    pub unit_type: UnitType,
    pub text: String,
    /// SupportSeams: (kind ∈ {supports, contains, refers}, ziel-id).
    pub seams: Vec<(String, String)>,
}

impl DocUnit {
    pub fn new(id: &str, unit_type: UnitType, text: &str) -> Self {
        Self {
            id: id.to_string(),
            unit_type,
            text: text.to_string(),
            seams: Vec::new(),
        }
    }

    pub fn with_seam(mut self, kind: &str, to: &str) -> Self {
        self.seams.push((kind.to_string(), to.to_string()));
        self
    }
}

/// DocCrystal (S1.1/S1.2): der kristallisierte Dokumentkern.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocCrystal {
    pub title: String,
    pub units: Vec<DocUnit>,
    /// covers(T): Abdeckungsziel.
    pub covers: Vec<String>,
    /// DocBoundary: geforderte Top-Abschnitte.
    pub required_sections: Vec<String>,
    /// Domaenenform von V1.
    pub no_score_fields: bool,
    /// ordering(neutral | meaningful) — steuert die Kanonisierung (S1.6).
    pub ordering: String,
}

/// Whitespace-Kosmetik wegstreifen (S1.6): Mehrfach-Leerzeichen kollabieren.
fn normalize_text(t: &str) -> String {
    t.split_whitespace().collect::<Vec<_>>().join(" ")
}

impl Canonicalize for DocCrystal {
    fn canonical_value(&self) -> CanonValue {
        let mut units: Vec<&DocUnit> = self.units.iter().collect();
        if self.ordering == "neutral" {
            units.sort_by(|a, b| a.id.cmp(&b.id));
        }
        let mut covers = self.covers.clone();
        covers.sort();
        CanonValue::map([
            ("title", CanonValue::text(normalize_text(&self.title))),
            ("ordering", CanonValue::text(&self.ordering)),
            (
                "covers",
                CanonValue::List(covers.iter().map(CanonValue::text).collect()),
            ),
            (
                "required_sections",
                CanonValue::List(
                    self.required_sections
                        .iter()
                        .map(CanonValue::text)
                        .collect(),
                ),
            ),
            ("no_score_fields", CanonValue::Bool(self.no_score_fields)),
            (
                "units",
                CanonValue::List(
                    units
                        .iter()
                        .map(|u| {
                            let mut seams = u.seams.clone();
                            seams.sort();
                            CanonValue::map([
                                ("id", CanonValue::text(&u.id)),
                                ("type", CanonValue::text(u.unit_type.as_str())),
                                ("text", CanonValue::text(normalize_text(&u.text))),
                                (
                                    "seams",
                                    CanonValue::List(
                                        seams
                                            .iter()
                                            .map(|(k, t)| {
                                                CanonValue::List(vec![
                                                    CanonValue::text(k),
                                                    CanonValue::text(t),
                                                ])
                                            })
                                            .collect(),
                                    ),
                                ),
                            ])
                        })
                        .collect(),
                ),
            ),
        ])
    }
}

/// DocWeave: Gewebe + Dokument-Metadaten.
#[derive(Debug, Clone)]
pub struct DocWeave {
    pub weave: Weave,
    pub title: String,
    pub covers: Vec<String>,
    pub required_sections: Vec<String>,
    pub no_score_fields: bool,
}

/// DocArtifact: die gerenderten Bytes (Zwei-Digest via Adapter, S7.2).
#[derive(Debug, Clone)]
pub struct DocArtifact {
    pub bytes: Vec<u8>,
    pub format: &'static str,
}

impl DocArtifact {
    pub fn byte_digest(&self) -> cce_core::signature::Digest {
        sha256(&self.bytes)
    }
}

/// Der Dokument-Adapter — Referenz-Implementierung der Teileliste.
#[derive(Debug, Default)]
pub struct DocumentAdapter;

impl DomainAdapter for DocumentAdapter {
    type Crystal = DocCrystal;
    type Weave = DocWeave;
    type Artifact = DocArtifact;

    fn domain_id(&self) -> &'static str {
        "D01-document"
    }

    fn wish_schema(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            ("required_sections", "geforderte Top-Abschnitte"),
            ("covers", "Abdeckungsziel (Themenmenge T)"),
            ("every_risk_has_countermeasure", "Stuetz-Naht-Pflicht"),
            (
                "no_score_fields",
                "keine Bewertungszahl als Entscheidung (V1)",
            ),
            ("ordering", "neutral | meaningful"),
        ]
    }

    fn validate_wish(&self, c: &DocCrystal) -> Result<(), Vec<Residue>> {
        let mut residues = Vec::new();
        if c.title.trim().is_empty() {
            residues.push(Residue::new(
                "wish-title",
                "validate_wish",
                ResidueKind::named("malformed_structure"),
                Severity::Blocking,
                "Titel fehlt",
            ));
        }
        if c.units.is_empty() {
            residues.push(Residue::new(
                "wish-units",
                "validate_wish",
                ResidueKind::named("malformed_structure"),
                Severity::Blocking,
                "keine Einheiten",
            ));
        }
        if c.ordering != "neutral" && c.ordering != "meaningful" {
            residues.push(Residue::new(
                "wish-ordering",
                "validate_wish",
                ResidueKind::named("malformed_structure"),
                Severity::Blocking,
                "ordering muss neutral|meaningful sein",
            ));
        }
        for u in &c.units {
            for (_, to) in &u.seams {
                if !c.units.iter().any(|v| &v.id == to) {
                    residues.push(Residue::new(
                        &format!("wish-seam-{}", u.id),
                        "validate_wish",
                        ResidueKind::named("boundary_crossing_without_seam"),
                        Severity::Blocking,
                        &format!("Naht von {} zeigt auf unbekannte Einheit {to}", u.id),
                    ));
                }
            }
        }
        if residues.is_empty() {
            Ok(())
        } else {
            Err(residues)
        }
    }

    fn to_canonical(&self, c: &DocCrystal) -> CanonicalState {
        CanonicalState::new(c.canonical_value())
    }

    fn encode(&self, c: &DocCrystal) -> PhcPackage {
        let fiber = c.canon();
        let root_class = c.canonical_class().0;
        let cell = Cell {
            id: "cell:document".to_string(),
            address: "phc://document/cell/domain=document".to_string(),
            axis_values: CanonValue::map([("domain_mode", CanonValue::text("document"))]),
            fiber,
            signature: root_class,
            status: "filled".to_string(),
        };
        let projection = Projection {
            id: "proj:materialize".to_string(),
            target_cell: "cell:document".to_string(),
            include: vec![
                "title".into(),
                "ordering".into(),
                "covers".into(),
                "required_sections".into(),
                "no_score_fields".into(),
                "units".into(),
            ],
            exclude: vec![],
            max_tokens: None,
            allowed_ops: vec!["render".into()],
        };
        let workcell = Workcell {
            id: "w:materialize".to_string(),
            cell: "cell:document".to_string(),
            projection: "proj:materialize".to_string(),
            intent: "Dokument materialisieren".to_string(),
            allowed_operations: vec!["render".into()],
            gate_chain: vec![
                "G1-Scope".into(),
                "G2-Boundary".into(),
                "G3-Type".into(),
                "G4-Residue".into(),
                "G5-Replay".into(),
            ],
            residue_policy: "visible".to_string(),
        };
        PhcPackage::build(
            &c.title,
            "document",
            root_class,
            vec![cell],
            vec![],
            vec![projection],
            vec![workcell],
            vec![".md".into()],
        )
    }

    fn loom(&self, p: &LocalProjection) -> Result<DocWeave, String> {
        let w = weave(p).map_err(|e| format!("{e:?}"))?;
        let get_text = |k: &str| match p.payload.get(k) {
            Some(CanonValue::Text(t)) => t.clone(),
            _ => String::new(),
        };
        let get_list = |k: &str| match p.payload.get(k) {
            Some(CanonValue::List(l)) => l
                .iter()
                .filter_map(|v| match v {
                    CanonValue::Text(t) => Some(t.clone()),
                    _ => None,
                })
                .collect(),
            _ => Vec::new(),
        };
        Ok(DocWeave {
            weave: w,
            title: get_text("title"),
            covers: get_list("covers"),
            required_sections: get_list("required_sections"),
            no_score_fields: matches!(
                p.payload.get("no_score_fields"),
                Some(CanonValue::Bool(true))
            ),
        })
    }

    fn materialize(&self, w: &DocWeave) -> DocArtifact {
        DocArtifact {
            bytes: render::render_markdown(w).into_bytes(),
            format: ".md",
        }
    }

    fn reanalyze(&self, a: &DocArtifact) -> Result<DocCrystal, String> {
        parse::parse_markdown(&a.bytes)
    }

    fn canonicalize(&self, c: &DocCrystal) -> CanonicalClass {
        c.canonical_class()
    }

    fn domain_gates(&self) -> Vec<Gate> {
        vec![
            Gate::new(
                "DocG-Coverage",
                GateKind::Integrity,
                "jedes Thema in covers(T) abgedeckt",
            ),
            Gate::new(
                "DocG-Support",
                GateKind::Topological,
                "jede stuetzungspflichtige Einheit hat SupportSeam",
            ),
            Gate::new(
                "DocG-NoScore",
                GateKind::Type,
                "kein Score-Feld als Entscheidung (V1)",
            ),
            Gate::new(
                "DocG-Structure",
                GateKind::Type,
                "wohlgeformte Struktur, keine verwaiste Einheit",
            ),
            Gate::new(
                "DocG-NonContradiction",
                GateKind::Integrity,
                "keine zwei Einheiten widersprechen sich",
            ),
            Gate::new(
                "DocG-Seam",
                GateKind::Topological,
                "kein Uebergang ohne SupportSeam (V4)",
            ),
            Gate::new(
                "DocG-RoundTrip",
                GateKind::Reanalysis,
                "equivalent(reanalyze(materialize(C)), C)",
            ),
        ]
    }

    fn run_domain_gates(&self, c: &DocCrystal, artifact: Option<&DocArtifact>) -> Vec<GateReport> {
        gates::run_all(self, c, artifact)
    }

    fn residue_vocabulary(&self) -> Vec<ResidueKind> {
        [
            "uncovered_topic",
            "unsupported_unit",
            "orphan_unit",
            "contradiction",
            "boundary_crossing_without_seam",
            "forbidden_score_field",
            "semantic_loss",
            "invented_semantic",
        ]
        .iter()
        .map(|s| ResidueKind::named(s))
        .collect()
    }

    fn counter_horizon(&self, c: &DocCrystal) -> CounterHorizon {
        CounterHorizon {
            null_models: vec![
                "Dokument nennt alle Themen, stuetzt aber keines (Nennung ohne Substanz)"
                    .to_string(),
            ],
            pathologies: self
                .residue_vocabulary()
                .iter()
                .map(|k| k.as_str().to_string())
                .chain(std::iter::once(format!(
                    "abdeckungsziel: {} Themen",
                    c.covers.len()
                )))
                .collect(),
        }
    }

    fn native_open(&self, a: &DocArtifact) -> OpenAction {
        OpenAction {
            program_class: "text_editor_or_markdown_viewer".to_string(),
            argument_hint: format!("{} ({} Bytes)", a.format, a.bytes.len()),
        }
    }

    fn export_formats(&self) -> Vec<&'static str> {
        // .docx: sichtbares Residuum S1.10-R1 (Bibliothekswahl offen).
        vec![".md"]
    }

    fn reference_cube(&self) -> DocCrystal {
        assets::three_risks_memo()
    }

    fn negative_cubes(&self) -> Vec<(DocCrystal, ResidueKind)> {
        assets::negative_cubes()
    }
}
