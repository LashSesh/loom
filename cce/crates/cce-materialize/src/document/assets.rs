//! Test-Assets der Dokument-Domaene (S1.9): der perfekte Referenz-Cube
//! „Drei-Risiken-Memo" + die Negativ-Cubes (je Verbot einer).

use super::{DocCrystal, DocUnit, UnitType};
use cce_core::residue::ResidueKind;

/// Der Referenz-Cube: drei Risiken, je mit Gegenmassnahme via SupportSeam;
/// covers(T) = die drei Risiko-Themen; no_score_fields; ordering neutral.
pub fn three_risks_memo() -> DocCrystal {
    DocCrystal {
        title: "Drei-Risiken-Memo".to_string(),
        units: vec![
            DocUnit::new("s1", UnitType::Section, "Risiken"),
            DocUnit::new(
                "r1",
                UnitType::Risk,
                "Serverausfall gefaehrdet den Go-Live-Termin",
            ),
            DocUnit::new(
                "c1",
                UnitType::Countermeasure,
                "Failover-Cluster mit automatischem Umschalten vorbereiten",
            )
            .with_seam("supports", "r1"),
            DocUnit::new(
                "r2",
                UnitType::Risk,
                "Datenverlust bei Migration der Altbestaende",
            ),
            DocUnit::new(
                "c2",
                UnitType::Countermeasure,
                "Vollstaendige Sicherung und Probelauf der Migration",
            )
            .with_seam("supports", "r2"),
            DocUnit::new(
                "r3",
                UnitType::Risk,
                "Lieferverzug der externen Komponenten",
            ),
            DocUnit::new(
                "c3",
                UnitType::Countermeasure,
                "Zweitlieferant qualifizieren und Puffer einplanen",
            )
            .with_seam("supports", "r3"),
        ],
        covers: vec![
            "Serverausfall".to_string(),
            "Datenverlust".to_string(),
            "Lieferverzug".to_string(),
        ],
        required_sections: vec!["Risiken".to_string()],
        no_score_fields: true,
        ordering: "neutral".to_string(),
    }
}

/// Negativ-Cubes (S1.9): jeder MUSS mit dem erwarteten Residuum abgelehnt
/// werden. (invented_semantic entsteht durch Artefakt-Manipulation und wird
/// im Kerntest-Harness erzeugt, nicht als Kristall.)
pub fn negative_cubes() -> Vec<(DocCrystal, ResidueKind)> {
    // 1. unabgedecktes viertes Thema → uncovered_topic
    let mut uncovered = three_risks_memo();
    uncovered.covers.push("Budgetueberschreitung".to_string());

    // 2. Risiko ohne Gegenmassnahme → unsupported_unit
    let mut unsupported = three_risks_memo();
    unsupported.units.retain(|u| u.id != "c2");

    // 3. Bewertungszahl trotz no_score_fields → forbidden_score_field
    let mut scored = three_risks_memo();
    scored.units.push(
        DocUnit::new(
            "x1",
            UnitType::Claim,
            "Bewertung: 8/10 — Risiko akzeptierbar",
        )
        .with_seam("refers", "r1"),
    );

    // 4. Querverweis ohne Naht → boundary_crossing_without_seam
    let mut seamless = three_risks_memo();
    seamless.units.push(DocUnit::new(
        "x2",
        UnitType::Claim,
        "Details siehe r2 im Abschnitt oben",
    ));
    // x2 verweist textlich auf r2, traegt aber keine Naht — und haengt sonst
    // an nichts (zugleich orphan; das Seam-Gate feuert zuerst gezielt).
    seamless
        .units
        .iter_mut()
        .find(|u| u.id == "x2")
        .expect("x2")
        .seams
        .push(("refers".to_string(), "r1".to_string()));

    vec![
        (uncovered, ResidueKind::named("uncovered_topic")),
        (unsupported, ResidueKind::named("unsupported_unit")),
        (scored, ResidueKind::named("forbidden_score_field")),
        (
            seamless,
            ResidueKind::named("boundary_crossing_without_seam"),
        ),
    ]
}
