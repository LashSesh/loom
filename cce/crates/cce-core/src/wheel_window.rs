//! WheelWindow / PhaseSpaceWindow (Teil 3.3, Identitaet I-7):
//! DER Doppelbaustein — beobachtend Apertur (Collect), generativ Nadel
//! (Distribute). EIN Typ, EIN Code-Pfad, von cce-observe UND cce-loom
//! genutzt (Teil 9 Phase F: "darf NICHT dupliziert werden") — der
//! mechanische Kern des ≃.

use crate::value::CanonValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowDirection {
    Inbound,
    Outbound,
    Bidirectional,
}

/// Das bidirektionale Fenster: dieselbe Struktur wirkt als Apertur (in)
/// und Nadel (out).
#[derive(Debug, Clone)]
pub struct WheelWindow {
    pub id: String,
    pub cell: String,
    pub projection_profile: String,
    pub gate: String,
    pub direction: WindowDirection,
}

impl WheelWindow {
    pub fn new(id: &str, cell: &str, projection_profile: &str, gate: &str) -> Self {
        Self {
            id: id.to_string(),
            cell: cell.to_string(),
            projection_profile: projection_profile.to_string(),
            gate: gate.to_string(),
            direction: WindowDirection::Bidirectional,
        }
    }

    /// Apertur-Rolle (Collect): laesst nur Items durch, die zum Zellen-Scope
    /// gehoeren — "Apertur vor Fokus" (PIO Axiom 2.1). Ueberschuss bleibt
    /// SICHTBAR (zweiter Rueckgabewert), wird nie still verworfen.
    pub fn aperture<'a>(
        &self,
        raw: &'a [(String, CanonValue)],
    ) -> (Vec<&'a CanonValue>, Vec<&'a str>) {
        let mut passed = Vec::new();
        let mut excess = Vec::new();
        for (scope, v) in raw {
            if scope == &self.cell {
                passed.push(v);
            } else {
                excess.push(scope.as_str());
            }
        }
        (passed, excess)
    }

    /// Nadel-Rolle (Distribute): fuehrt den Arbeitsfaden aus dem Kristall in
    /// eine Workcell-Sequenz — dieselbe Scope-Bindung wie die Apertur.
    pub fn needle(&self, crystal_parts: &[(String, CanonValue)]) -> Vec<CanonValue> {
        crystal_parts
            .iter()
            .filter(|(scope, _)| scope == &self.cell)
            .map(|(_, v)| v.clone())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// I-7: Apertur und Nadel sind ZWEI Richtungen EINER Struktur —
    /// derselbe Scope-Filter, bewiesen durch identische Durchlassmengen.
    #[test]
    fn aperture_and_needle_share_one_mechanism() {
        let w = WheelWindow::new("w1", "cell:risk", "profil", "G1-Scope");
        let parts = vec![
            ("cell:risk".to_string(), CanonValue::text("risiko 1")),
            ("cell:other".to_string(), CanonValue::text("fremd")),
            ("cell:risk".to_string(), CanonValue::text("risiko 2")),
        ];
        let (apt, excess) = w.aperture(&parts);
        let needle = w.needle(&parts);
        assert_eq!(
            apt.iter().map(|v| (*v).clone()).collect::<Vec<_>>(),
            needle,
            "beide Richtungen liefern dieselbe Durchlassmenge"
        );
        assert_eq!(excess, vec!["cell:other"], "Ueberschuss sichtbar (PIO)");
    }
}
