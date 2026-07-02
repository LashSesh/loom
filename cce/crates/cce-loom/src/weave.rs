//! Weave (G-05): gerichtete, teilgeordnete Workcell-/Block-Sequenz mit
//! Separatoren + Gates — endlicher Replaypfad. Der Webstuhl erzeugt aus
//! einer lokalen Projektion die Struktur mit Naehten.

use crate::radial_spindle::RadialSpindle;
use cce_core::canonical::Canonicalize;
use cce_core::value::CanonValue;
use cce_phc::projection_calc::LocalProjection;

/// Ein gewebter Block: getypte Einheit mit Naht-Referenzen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeaveBlock {
    pub unit_id: String,
    pub unit_type: String,
    pub text: String,
    /// Naehte zu anderen Einheiten: (kind, target_unit_id).
    pub seams: Vec<(String, String)>,
    /// Reihenfolge-Traeger: nur bedeutungstragend, wenn ordering=meaningful.
    pub position: u64,
}

/// Das Gewebe.
#[derive(Debug, Clone)]
pub struct Weave {
    pub blocks: Vec<WeaveBlock>,
    pub ordering: String,
    pub spindle: RadialSpindle,
    pub export_formats: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeaveError {
    /// Projektion traegt keine webbaren Einheiten.
    EmptyProjection,
    MalformedUnit(String),
}

/// Der Webstuhl: LocalProjection → Weave (radiale Spindel, Nullanker
/// markiert). Deterministisch; Reihenfolge aus der Projektion uebernommen.
pub fn weave(projection: &LocalProjection) -> Result<Weave, WeaveError> {
    let units = match projection.payload.get("units") {
        Some(CanonValue::List(us)) if !us.is_empty() => us.clone(),
        _ => return Err(WeaveError::EmptyProjection),
    };
    let ordering = match projection.payload.get("ordering") {
        Some(CanonValue::Text(t)) => t.clone(),
        _ => "meaningful".to_string(),
    };
    let mut spindle = RadialSpindle::new(&projection.cell_id);
    spindle.approach_zero("Weave-Beginn: Auswebung startet am Rand, nie im Zentrum");
    let mut blocks = Vec::new();
    for (i, u) in units.iter().enumerate() {
        let get = |k: &str| -> Result<String, WeaveError> {
            match u.get(k) {
                Some(CanonValue::Text(t)) => Ok(t.clone()),
                _ => Err(WeaveError::MalformedUnit(format!(
                    "Feld {k} fehlt in Unit {i}"
                ))),
            }
        };
        let seams = match u.get("seams") {
            Some(CanonValue::List(ss)) => ss
                .iter()
                .filter_map(|s| match s {
                    // kanonische Form: [kind, to]
                    CanonValue::List(pair) if pair.len() == 2 => match (&pair[0], &pair[1]) {
                        (CanonValue::Text(k), CanonValue::Text(t)) => Some((k.clone(), t.clone())),
                        _ => None,
                    },
                    // Map-Form: {kind, to}
                    _ => match (s.get("kind"), s.get("to")) {
                        (Some(CanonValue::Text(k)), Some(CanonValue::Text(t))) => {
                            Some((k.clone(), t.clone()))
                        }
                        _ => None,
                    },
                })
                .collect(),
            _ => Vec::new(),
        };
        blocks.push(WeaveBlock {
            unit_id: get("id")?,
            unit_type: get("type")?,
            text: get("text")?,
            seams,
            position: i as u64,
        });
    }
    Ok(Weave {
        blocks,
        ordering,
        spindle,
        export_formats: projection.export_formats.clone(),
    })
}

impl Canonicalize for Weave {
    fn canonical_value(&self) -> CanonValue {
        // Kanonische Klasse des Gewebes: Einheiten + Naht-Graph + Ordnung
        // (Positionswerte nur bei ordering=meaningful bedeutungstragend).
        let mut blocks: Vec<&WeaveBlock> = self.blocks.iter().collect();
        if self.ordering == "neutral" {
            blocks.sort_by(|a, b| a.unit_id.cmp(&b.unit_id));
        }
        CanonValue::map([
            ("ordering", CanonValue::text(&self.ordering)),
            (
                "blocks",
                CanonValue::List(
                    blocks
                        .iter()
                        .map(|b| {
                            let mut seams: Vec<&(String, String)> = b.seams.iter().collect();
                            seams.sort();
                            CanonValue::map([
                                ("id", CanonValue::text(&b.unit_id)),
                                ("type", CanonValue::text(&b.unit_type)),
                                ("text", CanonValue::text(&b.text)),
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
