//! Familie M — Hardware/CAD/Mechatronik/Fertigung (HW01–15) über dem
//! Familien-Kern `family_a`. 12 Relation + 1 Kette (HW10 Montage-
//! Reihenfolge) + 2 Ketten (HW05 Fertigungs-Routing, HW07 Mechatronik-
//! FSM sichere Zustandskette). Struktur (Verbau-/Netz-/Passungs-Nähte),
//! keine Physiksimulation behauptet.

use crate::document::UnitType;
use crate::family_a::{DocProfile, DomainRule};
use crate::family_a_domains::{
    chain_negatives, chain_reference, chained_domain, relation_domain, relation_negatives,
    relation_reference,
};

relation_domain!(
    hw01,
    "HW01-bom",
    "Stueckliste",
    "assembles",
    "missing_part",
    "Bauteil A verbaut",
    "Bauteil B verbaut",
    "Baugruppe",
    &[".md"]
);
relation_domain!(
    hw02,
    "HW02-netlist",
    "Schaltplan",
    "nets",
    "floating_pin",
    "Bauteil A netzgebunden",
    "Bauteil B netzgebunden",
    "Netzknoten",
    &[".md"]
);
relation_domain!(
    hw03,
    "HW03-cad",
    "CAD-Modell",
    "references",
    "underdimensioned",
    "Feature A bezugsgebunden",
    "Feature B bezugsgebunden",
    "Bezugsebene",
    &[".md"]
);
relation_domain!(
    hw04,
    "HW04-tolerance",
    "Toleranzplan",
    "fits",
    "tolerance_stackup",
    "Mass A passungsgebunden",
    "Mass B passungsgebunden",
    "Passungsbasis",
    &[".md"]
);
relation_domain!(
    hw06,
    "HW06-inspection",
    "Pruefplan",
    "inspects",
    "uninspected_feature",
    "Merkmal A geprueft",
    "Merkmal B geprueft",
    "Pruefmittel",
    &[".md"]
);
relation_domain!(
    hw08,
    "HW08-pcb",
    "PCB-Layout",
    "clears",
    "violated_clearance",
    "Netz A regelkonform",
    "Netz B regelkonform",
    "Designregeln",
    &[".md"]
);
relation_domain!(
    hw09,
    "HW09-material",
    "Materialauswahl",
    "meets",
    "unmet_property",
    "Anforderung A materialgedeckt",
    "Anforderung B materialgedeckt",
    "Materialkennwerte",
    &[".md"]
);
relation_domain!(
    hw11,
    "HW11-maintenance",
    "Wartungsplan",
    "intervals",
    "unmaintained_part",
    "Aufgabe A intervallgebunden",
    "Aufgabe B intervallgebunden",
    "Intervallplan",
    &[".md"]
);
relation_domain!(
    hw12,
    "HW12-fmea",
    "FMEA",
    "mitigates",
    "unmitigated_failure",
    "Fehler A mit Massnahme",
    "Fehler B mit Massnahme",
    "Massnahmenkatalog",
    &[".md"]
);
relation_domain!(
    hw13,
    "HW13-powerbudget",
    "Leistungsbudget",
    "budgets",
    "power_overrun",
    "Verbraucher A budgetgebunden",
    "Verbraucher B budgetgebunden",
    "Gesamtbudget",
    &[".md"]
);
relation_domain!(
    hw14,
    "HW14-3dprint",
    "3D-Druck-Vorbereitung",
    "supports",
    "unsupported_overhang",
    "Region A gestuetzt",
    "Region B gestuetzt",
    "Stuetzstruktur",
    &[".md"]
);
relation_domain!(
    hw15,
    "HW15-kinematics",
    "Kinematik-Modell",
    "joints",
    "overconstrained",
    "Glied A gelenkgebunden",
    "Glied B gelenkgebunden",
    "Gelenkbasis",
    &[".md"]
);

chained_domain!(
    hw05,
    "HW05-routing",
    "Fertigungsplan",
    "next",
    "infeasible_step",
    "Schritt Rohteil",
    "Schritt Bearbeitung",
    "Schritt Endmontage",
    &[".md"]
);
chained_domain!(
    hw07,
    "HW07-mechatronics",
    "Mechatronik-Steuerung",
    "transition",
    "unsafe_state",
    "Sicherer Startzustand",
    "Betriebszustand",
    "Sicherer Endzustand",
    &[".md"]
);
chained_domain!(
    hw10,
    "HW10-assembly",
    "Montageanleitung",
    "next",
    "impossible_assembly",
    "Schritt 1 Basis",
    "Schritt 2 Aufbau",
    "Schritt 3 Abschluss",
    &[".md"]
);

/// Alle 15 Profile der Familie M (HW01–15).
pub fn all_profiles() -> Vec<DocProfile> {
    vec![
        hw01(),
        hw02(),
        hw03(),
        hw04(),
        hw05(),
        hw06(),
        hw07(),
        hw08(),
        hw09(),
        hw10(),
        hw11(),
        hw12(),
        hw13(),
        hw14(),
        hw15(),
    ]
}
