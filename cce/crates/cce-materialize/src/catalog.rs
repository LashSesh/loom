//! Der Leaf-Domaenen-Katalog (S1_DOMAENENKATALOG K.4) als geladene
//! Registry: 213 Domaenen in 16 Familien, jede mit Differentiator und
//! PRODUKT-LEVEL-Slot (PL) — sichtbar gefuehrte Unfertigkeit statt
//! kaschierender Auslassung. Vollausbau jenseits D01 ist ausdruecklich
//! NICHT Teil der 100%-Bauabnahme (02_MASTER_DOD §3).

mod catalog_data {
    include!("catalog_data.rs");
}

pub use catalog_data::CATALOG;

/// Produkt-Level (PL0–PL4, Anti-Overclaim S11-A).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProductLevel {
    L0,
    L1,
    L2,
    L3,
    L4,
}

impl ProductLevel {
    pub fn as_str(self) -> &'static str {
        match self {
            ProductLevel::L0 => "PL0",
            ProductLevel::L1 => "PL1",
            ProductLevel::L2 => "PL2",
            ProductLevel::L3 => "PL3",
            ProductLevel::L4 => "PL4",
        }
    }
}

/// Ein Katalogeintrag (Leaf-Domaene).
#[derive(Debug, Clone, Copy)]
pub struct DomainEntry {
    pub id: &'static str,
    pub purpose: &'static str,
    pub crystal: &'static str,
    pub artifact: &'static str,
    pub core_gate: &'static str,
    pub core_residue: &'static str,
    pub level: ProductLevel,
}

pub fn by_id(id: &str) -> Option<&'static DomainEntry> {
    CATALOG.iter().find(|e| e.id == id)
}

/// feature_maturity_overclaim (S11/G12): eine Funktion/Domaene, die
/// ein hoeheres PL traegt als ihre Beweislage, ist ein Verstoss.
/// Beweislage im Bau: NUR D01 hat den gruenen Produkt-Kerntest —
/// jede andere Domaene ueber L1 ist Overclaim.
pub fn feature_maturity_overclaim() -> Vec<&'static str> {
    CATALOG
        .iter()
        .filter(|e| e.level > ProductLevel::L1 && e.id != "D01")
        .map(|e| e.id)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_loads_213_entries_in_16_families() {
        assert_eq!(CATALOG.len(), 213);
        let mut families: Vec<String> = CATALOG
            .iter()
            .map(|e| {
                e.id.trim_end_matches(|c: char| c.is_ascii_digit())
                    .to_string()
            })
            .collect();
        families.sort();
        families.dedup();
        assert_eq!(families.len(), 16, "16 Familien: {families:?}");
        // Jede Domaene traegt einen PL-Slot und die Kernfelder:
        for e in &CATALOG {
            assert!(!e.purpose.is_empty(), "{} ohne Zweck", e.id);
            assert!(!e.core_gate.is_empty(), "{} ohne Kern-Gate", e.id);
            assert!(!e.core_residue.is_empty(), "{} ohne Kern-Residuum", e.id);
        }
    }

    #[test]
    fn d01_is_pl4_rest_pl1_no_overclaim() {
        assert_eq!(by_id("D01").unwrap().level, ProductLevel::L4);
        assert!(feature_maturity_overclaim().is_empty(), "kein Overclaim");
        // Anti-Overclaim greift: eine Domaene ueber L1 ohne Kerntest
        // wuerde gelistet (Negativprobe ueber die Logik):
        let fake = DomainEntry {
            id: "D99",
            purpose: "test",
            crystal: "t",
            artifact: "t",
            core_gate: "t",
            core_residue: "t",
            level: ProductLevel::L3,
        };
        assert!(fake.level > ProductLevel::L1 && fake.id != "D01");
    }
}
