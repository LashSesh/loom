//! Familie O — Finanzen/Verwaltung/Planung (FIN01–10) über dem
//! Familien-Kern `family_a`. 8 Relation + 1 Azyklik (FIN04
//! Formel-Zirkelfreiheit) + 1 Kette (FIN08 Liquiditäts-Zeitachse).
//! Zahlen ordnen/erklaeren, Gates entscheiden (V1 geerbt; keine
//! Rechen-Engine behauptet — Struktur, nicht Arithmetik).

use crate::document::UnitType;
use crate::family_a::{DocProfile, DomainRule};
use crate::family_a_domains::{
    acyclic_domain, acyclic_negatives, acyclic_reference, chain_negatives, chain_reference,
    chained_domain, relation_domain, relation_negatives, relation_reference,
};

relation_domain!(
    fin01,
    "FIN01-budget",
    "Budget-Plan",
    "sums",
    "unbalanced_budget",
    "Position A summengebunden",
    "Position B summengebunden",
    "Gesamtsumme",
    &[".md"]
);
relation_domain!(
    fin02,
    "FIN02-costs",
    "Kostenaufstellung",
    "assigns",
    "unassigned_cost",
    "Posten A zugeordnet",
    "Posten B zugeordnet",
    "Kostentraeger",
    &[".md"]
);
relation_domain!(
    fin03,
    "FIN03-invoice",
    "Rechnung",
    "sums",
    "miscalculated_total",
    "Position A summengebunden",
    "Position B summengebunden",
    "Rechnungssumme",
    &[".md"]
);
relation_domain!(
    fin05,
    "FIN05-forecast",
    "Forecast",
    "derives",
    "unfounded_forecast",
    "Treiber A abgeleitet",
    "Treiber B abgeleitet",
    "Datenbasis",
    &[".md"]
);
relation_domain!(
    fin06,
    "FIN06-expenses",
    "Ausgaben-Report",
    "categorizes",
    "uncategorized_expense",
    "Ausgabe A kategorisiert",
    "Ausgabe B kategorisiert",
    "Kategorienplan",
    &[".md"]
);
relation_domain!(
    fin07,
    "FIN07-investment",
    "Investitionsrechnung",
    "discounts",
    "wrong_discounting",
    "Cashflow A diskontgebunden",
    "Cashflow B diskontgebunden",
    "Diskontsatz",
    &[".md"]
);
relation_domain!(
    fin09,
    "FIN09-costcenters",
    "Kostenstellen-Plan",
    "assigns",
    "orphan_cost_center",
    "Kostenstelle A zugeordnet",
    "Kostenstelle B zugeordnet",
    "Organisationsknoten",
    &[".md"]
);
relation_domain!(
    fin10,
    "FIN10-reconciliation",
    "Reconciliation",
    "matches",
    "unreconciled_item",
    "Buchung A abgeglichen",
    "Buchung B abgeglichen",
    "Gegenkonto",
    &[".md"]
);

acyclic_domain!(
    fin04,
    "FIN04-finmodel",
    "Finanzmodell",
    "formulates",
    "circular_reference",
    "Treiber A mit Formel",
    "Treiber B mit Formel",
    "Eingangsgroesse",
    &[".md"]
);
chained_domain!(
    fin08,
    "FIN08-liquidity",
    "Liquiditaetsplan",
    "timeline",
    "liquidity_gap",
    "Monat 1 Zufluss/Abfluss",
    "Monat 2 Zufluss/Abfluss",
    "Monat 3 Zufluss/Abfluss",
    &[".md"]
);

/// Alle 10 Profile der Familie O (FIN01–10).
pub fn all_profiles() -> Vec<DocProfile> {
    vec![
        fin01(),
        fin02(),
        fin03(),
        fin04(),
        fin05(),
        fin06(),
        fin07(),
        fin08(),
        fin09(),
        fin10(),
    ]
}
