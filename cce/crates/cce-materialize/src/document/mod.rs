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

/// Einheitstypen (S1.10-R2: additiv erweiterbar). `Table` ist die erste
/// echte CoreExtension durch den S14-Pfad (CE-1, S-E4a Teil II) — der
/// TYPE_REGISTRY-Eintrag `unit:table` IST diese Match-Arm-Erweiterung
/// (as_str/parse); ein separates Registry-Objekt existiert im Code
/// nicht, `UnitType` traegt die Registrierung bereits vollstaendig.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitType {
    Section,
    Claim,
    Support,
    Risk,
    Countermeasure,
    Definition,
    Step,
    /// CE-1 (S-E4a Teil II): strukturierte Tabellendaten (Header + Zeilen
    /// typisierter Zellen). Der Inhalt lebt — wie bei jedem Einheitstyp —
    /// im `text`-Feld, hier als kompakte, deterministische Kodierung
    /// (`encode_table`/`decode_table`); das haelt cce-loom/cce-phc
    /// UNVERAENDERT (kein neues Tor, keine neue Projektionsform).
    Table,
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
            UnitType::Table => "table",
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
            "table" => UnitType::Table,
            _ => return None,
        })
    }

    /// Stuetzungspflichtige Typen (DocG-Support).
    pub fn requires_support(self) -> bool {
        matches!(self, UnitType::Risk | UnitType::Claim)
    }
}

/// CE-1: eine Tabellenzelle. **Keine Floats** (K3, geerbt) — die
/// Zelltyp-Menge selbst erzwingt das strukturell (kein `TableCell::Float`
/// existiert); `DecFrac` traegt Dezimalbrueche exakt (num * 10^-scale).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TableCell {
    Text(String),
    Int(i64),
    DecFrac { num: i64, scale: u32 },
}

/// CE-1: `header: [Text;k]` (Spaltenarität k ≥ 1), `rows: [[Cell;k]]` in
/// Dokumentreihenfolge — bedeutungstragend, wird nie sortiert (K1/K5).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Table {
    pub header: Vec<String>,
    pub rows: Vec<Vec<TableCell>>,
}

/// Interne Trennzeichen fuer die `DocUnit.text`-Kodierung einer Tabelle —
/// C0-Kontrollzeichen, in echtem Zellinhalt praktisch nie vorkommend,
/// UND kein Unicode-Whitespace (ueberlebt `normalize_text` unveraendert).
const TABLE_CELL_SEP: char = '\u{1}';
const TABLE_ROW_SEP: char = '\u{2}';

fn encode_cell(c: &TableCell) -> String {
    match c {
        TableCell::Text(s) => format!("T{s}"),
        TableCell::Int(n) => format!("I{n}"),
        TableCell::DecFrac { num, scale } => format!("D{num}:{scale}"),
    }
}

fn decode_cell(s: &str) -> Option<TableCell> {
    let mut chars = s.chars();
    let tag = chars.next()?;
    let rest = chars.as_str();
    match tag {
        'T' => Some(TableCell::Text(rest.to_string())),
        'I' => rest.parse::<i64>().ok().map(TableCell::Int),
        'D' => {
            let (num_s, scale_s) = rest.split_once(':')?;
            Some(TableCell::DecFrac {
                num: num_s.parse().ok()?,
                scale: scale_s.parse().ok()?,
            })
        }
        _ => None,
    }
}

/// Kanonische, deterministische Kodierung einer Tabelle in einen
/// einzelnen String — der Traeger fuer `DocUnit.text` (kein neues Feld
/// am DocUnit, kein neuer Kanal in cce-loom/cce-phc).
pub fn encode_table(header: &[String], rows: &[Vec<TableCell>]) -> String {
    let mut parts = vec![header
        .iter()
        .map(|h| format!("T{h}"))
        .collect::<Vec<_>>()
        .join(&TABLE_CELL_SEP.to_string())];
    for row in rows {
        parts.push(
            row.iter()
                .map(encode_cell)
                .collect::<Vec<_>>()
                .join(&TABLE_CELL_SEP.to_string()),
        );
    }
    parts.join(&TABLE_ROW_SEP.to_string())
}

/// Rueckrichtung von `encode_table` — `None` bei unlesbarer Kodierung
/// (z. B. ein Zelltyp-Tag ausserhalb {T,I,D}: `invalid_cell_type`, s.
/// `gates::structure`).
pub fn decode_table(blob: &str) -> Option<Table> {
    let mut rows_iter = blob.split(TABLE_ROW_SEP);
    let header_row = rows_iter.next()?;
    let header: Vec<String> = header_row
        .split(TABLE_CELL_SEP)
        .map(|c| c.strip_prefix('T').unwrap_or(c).to_string())
        .collect();
    let mut rows = Vec::new();
    for row_str in rows_iter {
        let cells: Option<Vec<TableCell>> =
            row_str.split(TABLE_CELL_SEP).map(decode_cell).collect();
        rows.push(cells?);
    }
    Some(Table { header, rows })
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

    /// CE-1: eine Table-Einheit. `header`/`rows` werden deterministisch
    /// in `text` kodiert (`encode_table`) — dieselbe Faser wie jede
    /// andere Einheit, kein neues Feld, kein neuer Kanal.
    pub fn new_table(id: &str, header: &[&str], rows: Vec<Vec<TableCell>>) -> Self {
        let header_owned: Vec<String> = header.iter().map(|s| (*s).to_string()).collect();
        Self {
            id: id.to_string(),
            unit_type: UnitType::Table,
            text: encode_table(&header_owned, &rows),
            seams: Vec::new(),
        }
    }

    /// CE-1: dekodiert die Tabelle zurueck — `None` fuer Nicht-Table-
    /// Einheiten UND fuer unlesbare Kodierung (fail-closed, s.
    /// `gates::structure`s `invalid_cell_type`-Pruefung).
    pub fn as_table(&self) -> Option<Table> {
        if self.unit_type != UnitType::Table {
            return None;
        }
        decode_table(&self.text)
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
        // .docx: S1.10-R1 geschlossen (X1d, Oekosystem-Karte §2/E1) —
        // die Bibliothekswahl (zip, cce-docx-export-Blattcrate) ist
        // getroffen; hinter demselben materialize-Vertrag (DocWeave),
        // aber verlustig (format_loss, kein Reanalyse-Pfad wie .md).
        vec![".md", ".docx"]
    }

    fn reference_cube(&self) -> DocCrystal {
        assets::three_risks_memo()
    }

    fn negative_cubes(&self) -> Vec<(DocCrystal, ResidueKind)> {
        assets::negative_cubes()
    }
}

#[cfg(test)]
mod ce1_table_tests {
    use super::*;

    #[test]
    fn encode_decode_table_roundtrips() {
        let header = ["Risiko", "Wahrscheinlichkeit", "Kosten"];
        let rows = vec![
            vec![
                TableCell::Text("Serverausfall".to_string()),
                TableCell::DecFrac { num: 15, scale: 1 },
                TableCell::Int(5000),
            ],
            vec![
                TableCell::Text("Datenverlust".to_string()),
                TableCell::DecFrac { num: 5, scale: 1 },
                TableCell::Int(12000),
            ],
        ];
        let u = DocUnit::new_table("t1", &header, rows.clone());
        assert_eq!(u.unit_type, UnitType::Table);
        let back = u.as_table().expect("Tabelle muss dekodierbar sein");
        assert_eq!(back.header, header.map(String::from).to_vec());
        assert_eq!(back.rows, rows);
    }

    #[test]
    fn as_table_is_none_for_non_table_units() {
        let u = DocUnit::new("d1", UnitType::Definition, "kein Table");
        assert_eq!(u.as_table(), None);
    }

    #[test]
    fn decode_table_fails_closed_on_unknown_cell_tag() {
        // Simuliert eine unlesbare Zell-Kodierung (z. B. ein hypothetischer
        // Float-Tag "F" ausserhalb {T,I,D}) — muss None liefern, nicht
        // raten oder abstuerzen.
        let blob = "Ta\u{1}Tb\u{2}Fa\u{1}Ib".to_string();
        assert_eq!(decode_table(&blob), None);
    }

    #[test]
    fn unit_type_table_roundtrips_through_as_str_and_parse() {
        assert_eq!(UnitType::Table.as_str(), "table");
        assert_eq!(UnitType::parse("table"), Some(UnitType::Table));
    }

    /// R-TBL-1-Kern: voller Motorpfad (encode->project->loom->materialize
    /// ->reanalyze) — die "docx-Roundtrip-Klasse"-Disziplin (X1d), hier
    /// fuer die Pipe-Tabelle: derselbe Inhalt, dieselbe Klasse zurueck.
    #[test]
    fn table_unit_roundtrips_through_the_real_motor_pipeline() {
        let mut table_unit = DocUnit::new_table(
            "t1",
            &["Risiko", "Kosten"],
            vec![
                vec![TableCell::Text("Ausfall".to_string()), TableCell::Int(100)],
                vec![
                    TableCell::Text("Verlust".to_string()),
                    TableCell::DecFrac { num: 250, scale: 1 },
                ],
            ],
        );
        table_unit
            .seams
            .push(("refers".to_string(), "s1".to_string()));
        let crystal = DocCrystal {
            title: "Testtabelle".to_string(),
            units: vec![
                DocUnit::new("s1", UnitType::Section, "Uebersicht"),
                table_unit,
            ],
            covers: vec!["Uebersicht".to_string()],
            required_sections: vec!["Uebersicht".to_string()],
            no_score_fields: true,
            ordering: "neutral".to_string(),
        };
        let adapter = DocumentAdapter;
        let pkg = adapter.encode(&crystal);
        let proj =
            cce_phc::projection_calc::project(&pkg, "proj:materialize").expect("projizieren");
        let weave = adapter.loom(&proj).expect("weben");
        let artifact = adapter.materialize(&weave);
        let text = String::from_utf8(artifact.bytes.clone()).expect("UTF-8");
        assert!(
            text.contains("| Risiko | Kosten |"),
            "Pipe-Tabelle muss im gerenderten Markdown stehen:\n{text}"
        );
        assert!(
            text.contains("25.0"),
            "DecFrac muss dezimal gerendert werden:\n{text}"
        );
        let back = adapter.reanalyze(&artifact).expect("Reanalyse");
        assert!(adapter.equivalent(&back, &crystal), "Table-Roundtrip ≄ id");
    }

    #[test]
    fn gates_structure_holds_on_ragged_table() {
        let mut table_unit = DocUnit::new_table(
            "t1",
            &["a", "b", "c"],
            vec![vec![TableCell::Text("x".to_string())]], // nur 1 statt 3 Zellen
        );
        table_unit
            .seams
            .push(("refers".to_string(), "s1".to_string()));
        let crystal = DocCrystal {
            title: "Ragged".to_string(),
            units: vec![DocUnit::new("s1", UnitType::Section, "S"), table_unit],
            covers: vec![],
            required_sections: vec![],
            no_score_fields: false,
            ordering: "neutral".to_string(),
        };
        let report = crate::document::gates::structure(&crystal);
        assert!(!report.is_pass());
        assert!(report.reason.contains("ragged_table"), "{}", report.reason);
    }

    #[test]
    fn gates_structure_holds_on_invalid_cell_type() {
        // Direkt konstruierte, unlesbare Kodierung (simuliert einen
        // hypothetischen Encoder, der einen unbekannten Zelltyp schreibt).
        let mut table_unit = DocUnit::new("t1", UnitType::Table, "");
        table_unit.text = "Ta\u{1}Tb\u{2}Fx\u{1}Iy".to_string();
        table_unit
            .seams
            .push(("refers".to_string(), "s1".to_string()));
        let crystal = DocCrystal {
            title: "Invalid".to_string(),
            units: vec![DocUnit::new("s1", UnitType::Section, "S"), table_unit],
            covers: vec![],
            required_sections: vec![],
            no_score_fields: false,
            ordering: "neutral".to_string(),
        };
        let report = crate::document::gates::structure(&crystal);
        assert!(!report.is_pass());
        assert!(
            report.reason.contains("invalid_cell_type"),
            "{}",
            report.reason
        );
    }
}
