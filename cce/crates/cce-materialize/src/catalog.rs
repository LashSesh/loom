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

/// Domänen mit committeter Beweislage (Adapter 11/11 + Zeugen +
/// Kerntest + Doku-Zeile) — wächst je abgeschlossener Welle. Diese
/// Liste ist die EINE Wahrheit, an der PL2/PL3 hängt.
/// - D01: Produkt-Kerntest (PL4).
/// - D02–D15: Welle W1, family_a-Zeugen (PL3).
/// - KNOW01–15: Welle W2, family_j-Zeugen (PL3).
pub const WITNESSED_DOMAINS: [&str; 30] = [
    "D01", "D02", "D03", "D04", "D05", "D06", "D07", "D08", "D09", "D10", "D11", "D12", "D13",
    "D14", "D15", "KNOW01", "KNOW02", "KNOW03", "KNOW04", "KNOW05", "KNOW06", "KNOW07", "KNOW08",
    "KNOW09", "KNOW10", "KNOW11", "KNOW12", "KNOW13", "KNOW14", "KNOW15",
];

/// feature_maturity_overclaim (S11/G12): eine Domäne, die ein höheres
/// PL trägt als ihre committete Beweislage, ist ein Verstoss. PL2/PL3
/// verlangt Mitgliedschaft in WITNESSED_DOMAINS; PL4 ist D01 vorbehalten
/// (Produkt-Kerntest), professionsgebundene PL4 zusätzlich review-gebunden.
pub fn feature_maturity_overclaim() -> Vec<&'static str> {
    CATALOG
        .iter()
        .filter(|e| {
            // PL4 nur fuer D01 (Nutzungs-/Kerntest-Evidenz).
            let pl4_overclaim = e.level == ProductLevel::L4 && e.id != "D01";
            // PL2/PL3 nur mit Zeugen.
            let pl23_overclaim = (e.level == ProductLevel::L2 || e.level == ProductLevel::L3)
                && !WITNESSED_DOMAINS.contains(&e.id);
            pl4_overclaim || pl23_overclaim
        })
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
    fn d01_pl4_family_a_pl3_no_overclaim() {
        assert_eq!(by_id("D01").unwrap().level, ProductLevel::L4);
        // Welle W1: D02–D15 auf PL3 mit committeten Zeugen.
        for id in ["D02", "D08", "D15"] {
            assert_eq!(by_id(id).unwrap().level, ProductLevel::L3);
        }
        assert!(feature_maturity_overclaim().is_empty(), "kein Overclaim");
        // Anti-Overclaim greift: PL3 ohne Zeugen-Mitgliedschaft waere ein
        // Verstoss (Negativprobe ueber die Logik).
        assert!(!WITNESSED_DOMAINS.contains(&"D16"));
    }
}
