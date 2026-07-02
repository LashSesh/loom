//! Veraenderliche Verweis-Schicht (S9.1/S9.6): kleine, menschennahe Refs
//! auf den unveraenderlichen CAS. Der EINZIGE moegliche Konflikt des
//! Speichers lebt hier — und er ist SICHTBAR und operator-aufgeloest,
//! nie still (V2).

use cce_core::signature::Digest;
use std::collections::BTreeMap;

/// Sichtbarer Ref-Konflikt (wie ein Merge-Konflikt).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RefConflict {
    pub name: String,
    pub current: Digest,
    pub attempted: Digest,
    pub expected_old: Option<Digest>,
}

#[derive(Debug, Default)]
pub struct RefStore {
    refs: BTreeMap<String, Digest>,
    /// Offene Konflikte — bleiben sichtbar, bis der Operator aufloest.
    pub open_conflicts: Vec<RefConflict>,
}

impl RefStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, name: &str) -> Option<Digest> {
        self.refs.get(name).copied()
    }

    /// Compare-and-set: Konflikt wird NIE still aufgeloest, sondern als
    /// offener Eintrag gefuehrt.
    pub fn set(
        &mut self,
        name: &str,
        new: Digest,
        expected_old: Option<Digest>,
    ) -> Result<(), Box<RefConflict>> {
        let current = self.refs.get(name).copied();
        if current != expected_old {
            let conflict = RefConflict {
                name: name.to_string(),
                current: current.unwrap_or(Digest([0u8; 32])),
                attempted: new,
                expected_old,
            };
            self.open_conflicts.push(conflict.clone());
            return Err(Box::new(conflict));
        }
        self.refs.insert(name.to_string(), new);
        Ok(())
    }

    /// Operator-Aufloesung: waehlt explizit eine Seite; der Konflikt
    /// verschwindet erst durch diese benannte Handlung.
    pub fn resolve(&mut self, conflict: &RefConflict, choose_attempted: bool) {
        if choose_attempted {
            self.refs.insert(conflict.name.clone(), conflict.attempted);
        }
        self.open_conflicts
            .retain(|c| !(c.name == conflict.name && c.attempted == conflict.attempted));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cce_core::signature::sha256;

    /// S9.6: Ref-Konflikt sichtbar + operator-aufgeloest, nie still.
    #[test]
    fn ref_conflict_visible_and_operator_resolved() {
        let mut refs = RefStore::new();
        let v1 = sha256(b"v1");
        let v2 = sha256(b"v2");
        let v3 = sha256(b"v3");
        refs.set("workspace/neueste", v1, None).unwrap();
        refs.set("workspace/neueste", v2, Some(v1)).unwrap();
        // Zweites Geraet erwartet noch v1 ⇒ sichtbarer Konflikt:
        let conflict = refs.set("workspace/neueste", v3, Some(v1)).unwrap_err();
        assert_eq!(refs.open_conflicts.len(), 1);
        assert_eq!(
            refs.get("workspace/neueste"),
            Some(v2),
            "kein stilles Umbiegen"
        );
        // Operator loest auf (waehlt die versuchte Seite):
        refs.resolve(&conflict, true);
        assert!(refs.open_conflicts.is_empty());
        assert_eq!(refs.get("workspace/neueste"), Some(v3));
    }
}
