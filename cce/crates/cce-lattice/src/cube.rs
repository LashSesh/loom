//! Constraint Cube 𝒞 (F-07, Teil 3.2): typisierter Produktraum mit Graden
//! G0 blank · G1 +Constraints · G2 +Norm/Feld · G3 +Entfaltung/Projektion/
//! Gate/Ledger (ausfuehrbar).

use crate::propagation::Constraint;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Grade {
    G0,
    G1,
    G2,
    G3,
}

/// Dimension mit endlichem Wertebereich (deterministisch geordnet).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dimension {
    pub id: String,
    pub domain: Vec<String>,
}

impl Dimension {
    pub fn new(id: &str, domain: &[&str]) -> Self {
        Self {
            id: id.to_string(),
            domain: domain.iter().map(|s| s.to_string()).collect(),
        }
    }
}

/// Kopplung zweier Dimensionen (Kante des Kopplungsgraphen).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Coupling {
    pub a: String,
    pub b: String,
    pub kind: String,
}

/// Cube 𝒞 (Teil 3.2).
#[derive(Debug, Clone)]
pub struct Cube {
    pub dimensions: Vec<Dimension>,
    pub couplings: Vec<Coupling>,
    pub grade: Grade,
    pub constraints: Vec<Constraint>,
    pub norms: Vec<String>,
}

impl Cube {
    pub fn blank(dimensions: Vec<Dimension>) -> Self {
        Self {
            dimensions,
            couplings: Vec::new(),
            grade: Grade::G0,
            constraints: Vec::new(),
            norms: Vec::new(),
        }
    }

    /// G1: Constraints hinzufuegen (erhoeht den Grad).
    pub fn with_constraints(mut self, constraints: Vec<Constraint>) -> Self {
        // Kopplungen aus binaeren Constraints ableiten.
        for c in &constraints {
            if let Constraint::AllowedPairs { a, b, .. } = c {
                self.couplings.push(Coupling {
                    a: a.clone(),
                    b: b.clone(),
                    kind: "constraint".to_string(),
                });
            }
        }
        self.constraints = constraints;
        self.grade = self.grade.max(Grade::G1);
        self
    }

    /// G2: Normen/Feld.
    pub fn with_norms(mut self, norms: Vec<String>) -> Self {
        self.norms = norms;
        self.grade = self.grade.max(Grade::G2);
        self
    }

    /// G3: ausfuehrbar (Entfaltung/Projektion/Gate/Ledger angeschlossen).
    pub fn executable(mut self) -> Self {
        self.grade = Grade::G3;
        self
    }

    pub fn dimension(&self, id: &str) -> Option<&Dimension> {
        self.dimensions.iter().find(|d| d.id == id)
    }
}
