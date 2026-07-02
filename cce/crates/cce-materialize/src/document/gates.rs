//! Die sieben Dokument-Gates (S1.4) — boolesch, begruendet, fail-closed.
//! Jedes Rot benennt sein Residuum (S1.5-Vokabular).

use super::{DocArtifact, DocCrystal, DocumentAdapter, UnitType};
use crate::adapter::DomainAdapter;
use cce_core::gate::GateReport;

/// DocG-Coverage: jedes Thema in covers(T) von ≥1 Einheit abgedeckt.
pub fn coverage(c: &DocCrystal) -> GateReport {
    let uncovered: Vec<&String> = c
        .covers
        .iter()
        .filter(|topic| {
            !c.units
                .iter()
                .any(|u| u.text.to_lowercase().contains(&topic.to_lowercase()))
        })
        .collect();
    if uncovered.is_empty() {
        GateReport::pass("DocG-Coverage", "alle Themen abgedeckt")
    } else {
        GateReport::hold(
            "DocG-Coverage",
            &format!("uncovered_topic: {uncovered:?} nicht abgedeckt"),
        )
    }
}

/// DocG-Support: jede stuetzungspflichtige Einheit hat eine SupportSeam.
pub fn support(c: &DocCrystal) -> GateReport {
    let unsupported: Vec<&str> = c
        .units
        .iter()
        .filter(|u| u.unit_type.requires_support())
        .filter(|u| {
            !c.units.iter().any(|v| {
                v.seams
                    .iter()
                    .any(|(kind, to)| kind == "supports" && to == &u.id)
            })
        })
        .map(|u| u.id.as_str())
        .collect();
    if unsupported.is_empty() {
        GateReport::pass(
            "DocG-Support",
            "jede stuetzungspflichtige Einheit gestuetzt",
        )
    } else {
        GateReport::hold(
            "DocG-Support",
            &format!("unsupported_unit: {unsupported:?} ohne Stuetz-Naht"),
        )
    }
}

/// DocG-NoScore (Domaenenform von V1).
pub fn no_score(c: &DocCrystal) -> GateReport {
    if !c.no_score_fields {
        return GateReport::pass("DocG-NoScore", "no_score_fields nicht gefordert");
    }
    let offenders: Vec<&str> = c
        .units
        .iter()
        .filter(|u| {
            let t = u.text.to_lowercase();
            t.contains("score:") || t.contains("bewertung:") || t.contains("punktzahl:")
        })
        .map(|u| u.id.as_str())
        .collect();
    if offenders.is_empty() {
        GateReport::pass("DocG-NoScore", "kein Score-Feld als Entscheidung")
    } else {
        GateReport::hold(
            "DocG-NoScore",
            &format!("forbidden_score_field: {offenders:?} tragen Bewertungsfelder"),
        )
    }
}

/// DocG-Structure: keine verwaiste Einheit; geforderte Abschnitte vorhanden.
pub fn structure(c: &DocCrystal) -> GateReport {
    let orphans: Vec<&str> = c
        .units
        .iter()
        .filter(|u| u.unit_type != UnitType::Section)
        .filter(|u| {
            let has_out = !u.seams.is_empty();
            let has_in = c
                .units
                .iter()
                .any(|v| v.seams.iter().any(|(_, to)| to == &u.id));
            !has_out && !has_in
        })
        .map(|u| u.id.as_str())
        .collect();
    let missing_sections: Vec<&String> = c
        .required_sections
        .iter()
        .filter(|s| {
            !c.units
                .iter()
                .any(|u| u.unit_type == UnitType::Section && u.text.contains(s.as_str()))
        })
        .collect();
    if orphans.is_empty() && missing_sections.is_empty() {
        GateReport::pass("DocG-Structure", "wohlgeformt, keine verwaiste Einheit")
    } else {
        GateReport::hold(
            "DocG-Structure",
            &format!("orphan_unit: {orphans:?}; fehlende Abschnitte: {missing_sections:?}"),
        )
    }
}

/// DocG-NonContradiction: expliziter Widerspruch zweier Einheiten.
pub fn non_contradiction(c: &DocCrystal) -> GateReport {
    for a in &c.units {
        for b in &c.units {
            if a.id != b.id
                && a.text.to_lowercase() == format!("es gilt nicht: {}", b.text.to_lowercase())
            {
                return GateReport::hold(
                    "DocG-NonContradiction",
                    &format!("contradiction: {} widerspricht {}", a.id, b.id),
                );
            }
        }
    }
    GateReport::pass("DocG-NonContradiction", "kein direkter Widerspruch")
}

/// DocG-Seam (Domaenenform von V4): Querverweis im Text ohne Naht.
pub fn seam(c: &DocCrystal) -> GateReport {
    for u in &c.units {
        for v in &c.units {
            if u.id != v.id && u.text.contains(&format!("siehe {}", v.id)) {
                let connected = u.seams.iter().any(|(_, to)| to == &v.id)
                    || v.seams.iter().any(|(_, to)| to == &u.id);
                if !connected {
                    return GateReport::hold(
                        "DocG-Seam",
                        &format!(
                            "boundary_crossing_without_seam: {} verweist auf {} ohne Naht",
                            u.id, v.id
                        ),
                    );
                }
            }
        }
    }
    GateReport::pass("DocG-Seam", "kein Uebergang ohne Naht")
}

/// DocG-RoundTrip (Domaenen-Kerntest): materialize∘reanalyze klassenerhaltend.
pub fn roundtrip(adapter: &DocumentAdapter, c: &DocCrystal, artifact: &DocArtifact) -> GateReport {
    match adapter.reanalyze(artifact) {
        Err(e) => GateReport::hold(
            "DocG-RoundTrip",
            &format!("semantic_loss: Reanalyse scheiterte: {e}"),
        ),
        Ok(back) => {
            if adapter.equivalent(&back, c) {
                GateReport::pass("DocG-RoundTrip", "Reanalyse faellt auf die Kristallklasse")
            } else {
                // Diagnose: verloren oder erfunden?
                let orig_ids: std::collections::BTreeSet<&str> =
                    c.units.iter().map(|u| u.id.as_str()).collect();
                let back_ids: std::collections::BTreeSet<&str> =
                    back.units.iter().map(|u| u.id.as_str()).collect();
                let invented: Vec<&&str> = back_ids.difference(&orig_ids).collect();
                let lost: Vec<&&str> = orig_ids.difference(&back_ids).collect();
                let kind = if !invented.is_empty() {
                    format!("invented_semantic: erfundene Einheiten {invented:?}")
                } else if !lost.is_empty() {
                    format!("semantic_loss: verlorene Einheiten {lost:?}")
                } else {
                    "semantic_loss: Klassenabweichung im Inhalt".to_string()
                };
                GateReport::hold("DocG-RoundTrip", &kind)
            }
        }
    }
}

/// Alle sieben Gates in Pfadreihenfolge.
pub fn run_all(
    adapter: &DocumentAdapter,
    c: &DocCrystal,
    artifact: Option<&DocArtifact>,
) -> Vec<GateReport> {
    let mut reports = vec![
        coverage(c),
        support(c),
        no_score(c),
        structure(c),
        non_contradiction(c),
        seam(c),
    ];
    match artifact {
        Some(a) => reports.push(roundtrip(adapter, c, a)),
        None => reports.push(GateReport::hold(
            "DocG-RoundTrip",
            "kein Artefakt vorhanden — Round-Trip nicht pruefbar (fail-closed)",
        )),
    }
    reports
}
