//! loom-gate — Gate-Ausfuehrung UEBER Containerinhalt (Port: cce-core).
//! Read-execute: Gates LESEN Containerinhalt und urteilen; KEIN
//! API-Pfad schreibt Verdikte von aussen (LOOM Teil 5.4).

use cce_core::gate::{GateChain, GateReport};
use loom_canon::Cv;

/// Baut das GATE_REPORTS-Segment-Payload aus Motor-GateReports —
/// boolesch + begruendet, NIE mit Score-Feld (N14 strukturell:
/// GateReport besitzt kein numerisches Feld).
pub fn gate_reports_segment(reports: &[GateReport]) -> Cv {
    Cv::map(vec![(
        "reports",
        Cv::Array(
            reports
                .iter()
                .map(|r| {
                    Cv::map(vec![
                        ("gate_id", Cv::Text(r.gate_id.clone())),
                        ("verdict", Cv::Bool(r.is_pass())),
                        ("reason", Cv::Text(r.reason.clone())),
                    ])
                })
                .collect(),
        ),
    )])
}

/// Liest GATE_REPORTS-Payload und prueft fail-closed als Kette.
pub fn evaluate_reports(seg: &Cv) -> Result<bool, String> {
    let Cv::Map(_) = seg else {
        return Err("GATE_REPORTS ist keine Map".into());
    };
    let reports = match seg {
        Cv::Map(entries) => entries
            .iter()
            .find(|(k, _)| matches!(k, Cv::Text(s) if s == "reports"))
            .map(|(_, v)| v),
        _ => None,
    };
    let Some(Cv::Array(items)) = reports else {
        return Err("reports fehlt".into());
    };
    let mut chain = GateChain::new();
    for it in items {
        let (mut gate_id, mut verdict, mut reason) = (None, None, None);
        if let Cv::Map(fields) = it {
            for (k, v) in fields {
                match (k, v) {
                    (Cv::Text(s), Cv::Text(t)) if s == "gate_id" => gate_id = Some(t.clone()),
                    (Cv::Text(s), Cv::Bool(b)) if s == "verdict" => verdict = Some(*b),
                    (Cv::Text(s), Cv::Text(t)) if s == "reason" => reason = Some(t.clone()),
                    _ => {}
                }
            }
        }
        let (Some(g), Some(v), Some(r)) = (gate_id, verdict, reason) else {
            return Err("GateReport unvollstaendig".into());
        };
        chain.push(if v {
            GateReport::pass(&g, &r)
        } else {
            GateReport::hold(&g, &r)
        });
    }
    Ok(chain.all_pass())
}
