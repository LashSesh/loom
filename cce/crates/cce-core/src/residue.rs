//! Residuen (F-05/I-05, Teil 3.6): `B⁻x` ist STETS sichtbares Feld (P4/V2).
//! Schliessung heisst `B⁻x = 0` MIT ausgewiesenem (dann leerem) Residuenfeld,
//! nie stilles Absorbieren.

use crate::value::CanonValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Info,
    Warning,
    Blocking,
}

impl Severity {
    pub fn as_str(self) -> &'static str {
        match self {
            Severity::Info => "info",
            Severity::Warning => "warning",
            Severity::Blocking => "blocking",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResidueStatus {
    Open,
    AbsorbedByGate,
    ClosedByEvidence,
    Deferred,
}

/// Residuenart: die Kernarten aus Teil 3.6 plus domaenen-/schichtspezifische
/// Arten als benannte Kennung (z. B. `uncovered_topic`, `frontier_desync`).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ResidueKind {
    UnresolvedConstraint,
    ExcludedMaterial,
    FailedGate,
    OpenQuestion,
    Named(String),
}

impl ResidueKind {
    pub fn named(s: &str) -> Self {
        ResidueKind::Named(s.to_string())
    }

    pub fn as_str(&self) -> &str {
        match self {
            ResidueKind::UnresolvedConstraint => "unresolved_constraint",
            ResidueKind::ExcludedMaterial => "excluded_material",
            ResidueKind::FailedGate => "failed_gate",
            ResidueKind::OpenQuestion => "open_question",
            ResidueKind::Named(s) => s,
        }
    }
}

/// Sichtbarer Rest (Teil 3.6). Alle Felder Pflicht ausser `visible_to`
/// (leer = fuer alle sichtbar).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Residue {
    pub id: String,
    pub origin: String,
    pub kind: ResidueKind,
    pub severity: Severity,
    pub content: String,
    pub visible_to: Vec<String>,
    pub resolution_status: ResidueStatus,
}

impl Residue {
    pub fn new(
        id: &str,
        origin: &str,
        kind: ResidueKind,
        severity: Severity,
        content: &str,
    ) -> Self {
        Self {
            id: id.to_string(),
            origin: origin.to_string(),
            kind,
            severity,
            content: content.to_string(),
            visible_to: Vec::new(),
            resolution_status: ResidueStatus::Open,
        }
    }

    pub fn canonical_value(&self) -> CanonValue {
        CanonValue::map([
            ("id", CanonValue::text(&self.id)),
            ("origin", CanonValue::text(&self.origin)),
            ("kind", CanonValue::text(self.kind.as_str())),
            ("severity", CanonValue::text(self.severity.as_str())),
            ("content", CanonValue::text(&self.content)),
        ])
    }
}

/// Das STETS ausgewiesene Residuenfeld: auch leer existiert es explizit
/// („geschlossen (∅)" — S3.6/S6.3). Es gibt keinen Pfad, ein Residuum zu
/// entfernen — nur Statusuebergaenge (kein stilles Loeschen, V2).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ResidueField {
    entries: Vec<Residue>,
}

impl ResidueField {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, r: Residue) {
        self.entries.push(r);
    }

    pub fn entries(&self) -> &[Residue] {
        &self.entries
    }

    /// `B⁻x = 0` — leer, aber ausgewiesen.
    pub fn is_closed_empty(&self) -> bool {
        self.entries
            .iter()
            .all(|r| r.resolution_status == ResidueStatus::ClosedByEvidence)
    }

    pub fn has_blocking(&self) -> bool {
        self.entries
            .iter()
            .any(|r| r.severity == Severity::Blocking && r.resolution_status == ResidueStatus::Open)
    }

    /// Anzeigeform: explizites „geschlossen (∅)" statt Weglassen.
    pub fn render_summary(&self) -> String {
        if self.entries.is_empty() {
            "geschlossen (∅)".to_string()
        } else {
            format!("{} Residuum/Residuen sichtbar", self.entries.len())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// P4/V2: das leere Residuenfeld ist explizit ausgewiesen, nie weggelassen.
    #[test]
    fn empty_residue_field_is_visible() {
        let f = ResidueField::new();
        assert!(f.is_closed_empty());
        assert_eq!(f.render_summary(), "geschlossen (∅)");
    }

    #[test]
    fn blocking_residue_prevents_closure() {
        let mut f = ResidueField::new();
        f.push(Residue::new(
            "r1",
            "gate:DocG-Support",
            ResidueKind::named("unsupported_unit"),
            Severity::Blocking,
            "Risiko 2 ohne Gegenmassnahme",
        ));
        assert!(f.has_blocking());
        assert!(!f.is_closed_empty());
    }
}
