//! Projektionskalkuel (G-14, PHC §10): π_c: H → H_c — lokal, gatebar, klein.
//! No-Horizon-Leakage: die Projektion traegt NUR die Zielzelle + deklarierte
//! Include-Felder, nie den globalen Horizont.

use crate::package::{PhcPackage, Projection as ProjectionProfile};
use cce_core::value::CanonValue;

/// Lokale Projektion (Ausgabe von `project`, Eingabe des Webstuhls).
#[derive(Debug, Clone)]
pub struct LocalProjection {
    pub cell_id: String,
    pub payload: CanonValue,
    pub allowed_ops: Vec<String>,
    pub gate_chain: Vec<String>,
    pub export_formats: Vec<String>,
    pub domain_mode: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectError {
    UnknownProjection(String),
    UnknownCell(String),
    /// Scope-Gate: Projektion ausserhalb des deklarierten Rahmens.
    ScopeViolation(String),
}

/// project → apply_projection → assert gate_scope (Teil 7.4).
pub fn project(p: &PhcPackage, projection_id: &str) -> Result<LocalProjection, ProjectError> {
    let profile: &ProjectionProfile = p
        .projections
        .iter()
        .find(|pr| pr.id == projection_id)
        .ok_or_else(|| ProjectError::UnknownProjection(projection_id.to_string()))?;
    let cell = p
        .cells
        .iter()
        .find(|c| c.id == profile.target_cell)
        .ok_or_else(|| ProjectError::UnknownCell(profile.target_cell.clone()))?;
    // Include-Schnitt: nur deklarierte Felder verlassen die Zelle.
    let payload = match &cell.fiber {
        CanonValue::Map(m) => CanonValue::Map(
            m.iter()
                .filter(|(k, _)| profile.include.iter().any(|i| i == *k))
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
        ),
        other => other.clone(),
    };
    // gate_scope: Exclude darf nie im Payload landen.
    if let CanonValue::Map(m) = &payload {
        for ex in &profile.exclude {
            if m.contains_key(ex) {
                return Err(ProjectError::ScopeViolation(format!(
                    "ausgeschlossenes Feld {ex} im Payload"
                )));
            }
        }
    }
    let gate_chain = p
        .workcells
        .iter()
        .find(|w| w.projection == profile.id)
        .map(|w| w.gate_chain.clone())
        .unwrap_or_else(|| vec!["G1-Scope".to_string()]);
    Ok(LocalProjection {
        cell_id: cell.id.clone(),
        payload,
        allowed_ops: profile.allowed_ops.clone(),
        gate_chain,
        export_formats: p.exports.clone(),
        domain_mode: p.manifest.domain_mode.clone(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::package::{Cell, Projection};
    use cce_core::signature::sha256;

    /// No-Horizon-Leakage: nur Include-Felder verlassen die Zelle.
    #[test]
    fn projection_is_local() {
        let p = PhcPackage::build(
            "t",
            "document",
            sha256(b"r"),
            vec![Cell {
                id: "c1".into(),
                address: "phc://t/cell/u=1".into(),
                axis_values: CanonValue::map([]),
                fiber: CanonValue::map([
                    ("inhalt", CanonValue::text("sichtbar")),
                    ("geheim", CanonValue::text("horizont")),
                ]),
                signature: sha256(b"c1"),
                status: "filled".into(),
            }],
            vec![],
            vec![Projection {
                id: "p1".into(),
                target_cell: "c1".into(),
                include: vec!["inhalt".into()],
                exclude: vec!["geheim".into()],
                max_tokens: None,
                allowed_ops: vec!["render".into()],
            }],
            vec![],
            vec![".md".into()],
        );
        let proj = project(&p, "p1").unwrap();
        assert!(proj.payload.get("inhalt").is_some());
        assert!(
            proj.payload.get("geheim").is_none(),
            "globaler Horizont ist geleakt (G-14 verletzt)"
        );
    }
}
