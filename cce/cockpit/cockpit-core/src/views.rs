//! Die Ansichten (S3.2 Pruef-Flaeche-Rendering + S3-A1..A6 + Overlay
//! S3-A7..A10). Jede Anzeige ist WURZEL-RUECKFUEHRBAR: jedes Element
//! traegt seine Motor-Quelle (source_ref) — das Cockpit rendert
//! Fakten, es erzeugt sie nie (COCK-INV-1). Alle Strukturen sind
//! bewusst OHNE Setter fuer Urteils-/Residuenfelder konstruiert:
//! sie entstehen nur aus Motor-Artefakten.

use cce_core::gate::GateReport;

/// Ein renderbares Element mit Wurzel-Rueckfuehrung.
#[derive(Debug, Clone)]
pub struct ViewItem {
    pub label: String,
    pub value: String,
    /// Motor-Artefakt, aus dem die Anzeige stammt (z. B. gate:G3,
    /// residue:csa:..., ledger:head, phaseblock:<id>).
    pub source_ref: String,
}

/// Gate-Report-Ansicht: gruen/rot MIT Begruendung — rot nie ohne Grund.
pub fn gate_report_view(reports: &[GateReport]) -> Vec<ViewItem> {
    reports
        .iter()
        .map(|g| ViewItem {
            label: g.gate_id.clone(),
            value: format!(
                "{} — {}",
                if g.is_pass() { "gruen" } else { "rot" },
                g.reason
            ),
            source_ref: format!("gate:{}", g.gate_id),
        })
        .collect()
}

/// Residuen-Ansicht: LEER wird EXPLIZIT als „geschlossen (∅)" gezeigt,
/// nie weggelassen (COCK-INV-2 / P4/V2 als Pixel).
pub fn residue_view(residues: &[(String, String, String)]) -> Vec<ViewItem> {
    if residues.is_empty() {
        return vec![ViewItem {
            label: "Residuenfeld".into(),
            value: "geschlossen (∅)".into(),
            source_ref: "residue_field:empty".into(),
        }];
    }
    residues
        .iter()
        .map(|(quelle, inhalt, severity)| ViewItem {
            label: quelle.clone(),
            value: format!("{inhalt} [{severity}]"),
            source_ref: format!("residue:{quelle}"),
        })
        .collect()
}

/// PhaseBlock-Ansicht (S3-A1).
pub fn phaseblock_view(blocks: &[(String, String)]) -> Vec<ViewItem> {
    blocks
        .iter()
        .map(|(id, status)| ViewItem {
            label: format!("PhaseBlock {id}"),
            value: status.clone(),
            source_ref: format!("phaseblock:{id}"),
        })
        .collect()
}

/// Frontier+Sync-Ansicht (S3-A2): Desync ist BLOCKIEREND sichtbar.
pub fn frontier_view(frontiers: &[(String, String)], desync: Option<&str>) -> Vec<ViewItem> {
    let mut items: Vec<ViewItem> = frontiers
        .iter()
        .map(|(scale, head)| ViewItem {
            label: format!("Frontier {scale}"),
            value: head.clone(),
            source_ref: format!("frontier:{scale}"),
        })
        .collect();
    if let Some(d) = desync {
        items.push(ViewItem {
            label: "frontier_desync".into(),
            value: format!("{d} [blocking]"),
            source_ref: "residue:frontier_desync".into(),
        });
    }
    items
}

/// Ratchet-Ansicht (S3-A3).
pub fn ratchet_view(position: usize, locked: bool) -> ViewItem {
    ViewItem {
        label: "Ratchet".into(),
        value: format!(
            "Position {position}, {}",
            if locked { "gelockt" } else { "offen" }
        ),
        source_ref: "ratchet:state".into(),
    }
}

/// Blue/Red+WrapStability-Ansicht (S3-A4).
pub fn bluered_view(blue_closed: bool, red_promoted: bool, wrap_stable: bool) -> Vec<ViewItem> {
    vec![
        ViewItem {
            label: "BlueCube".into(),
            value: if blue_closed {
                "geschlossen ⇒ PhaseBlock"
            } else {
                "offen"
            }
            .into(),
            source_ref: "bluered:blue".into(),
        },
        ViewItem {
            label: "RedCube".into(),
            value: if red_promoted {
                "geschlossen ⇒ Promote"
            } else {
                "offen"
            }
            .into(),
            source_ref: "bluered:red".into(),
        },
        ViewItem {
            label: "WrapStability".into(),
            value: if wrap_stable {
                "stabil (annullierter Drift sichtbar)"
            } else {
                "instabil"
            }
            .into(),
            source_ref: "spiral:wrap".into(),
        },
    ]
}

/// HBM-Candidate-Board (S3-A5): Score AUSSCHLIESSLICH als
/// Ranking-Spalte mit FESTEM Label — COCK-INV-6: kein UI-Pfad macht
/// eine Kennzahl zur Entscheidung. Der Status kommt NUR vom Gate.
pub const RANKING_LABEL: &str = "ordnet, entscheidet nicht";

#[derive(Debug, Clone)]
pub struct CandidateRow {
    pub candidate_id: String,
    /// Motor-Fakt (Gate): Pass/Hold/Reject mit Diagnose.
    pub gate_status: String,
    /// Reine Ordnungsspalte; Label ist konstant und nicht abschaltbar.
    pub ranking_display: String,
    pub source_ref: String,
}

pub fn hbm_board(rows: &[(String, String, u32)]) -> Vec<CandidateRow> {
    rows.iter()
        .map(|(id, gate_status, rank)| CandidateRow {
            candidate_id: id.clone(),
            gate_status: gate_status.clone(),
            ranking_display: format!("Rang {rank} ({RANKING_LABEL})"),
            source_ref: format!("hbm:candidate:{id}"),
        })
        .collect()
}

/// CSA-Quellen-/Residuen-Sicht (S3-A6).
pub fn csa_view(sources: &[(String, String)]) -> Vec<ViewItem> {
    sources
        .iter()
        .map(|(locator, verdict)| ViewItem {
            label: locator.clone(),
            value: verdict.clone(),
            source_ref: format!("csa:source:{locator}"),
        })
        .collect()
}

// ---------- Overlay S3-A7..A10 ----------

/// ProviderStatus-Ansicht (S3-A7): Modus, provider/model/version,
/// Kanzel-Zustand, Budget/Rate — alles aus dem Manifest (wurzel-
/// rueckfuehrbar, R-INF-7).
pub fn provider_status_view(
    mode: &str,
    manifest: &cce_inference::manifest::ModelManifest,
    kanzel_status: &str,
) -> Vec<ViewItem> {
    vec![
        ViewItem {
            label: "Modus".into(),
            value: mode.into(),
            source_ref: "runtime:mode".into(),
        },
        ViewItem {
            label: "Provider".into(),
            value: format!(
                "{} · {}@{}",
                manifest.provider_id, manifest.model_id, manifest.model_version
            ),
            source_ref: format!("manifest:{}", manifest.provider_id),
        },
        ViewItem {
            label: "Kanzel".into(),
            value: kanzel_status.into(),
            source_ref: "kanzel:status".into(),
        },
        ViewItem {
            label: "Budget/Rate".into(),
            value: format!("{:?}/{:?}", manifest.token_budget, manifest.rate_limit),
            source_ref: format!("manifest:{}", manifest.provider_id),
        },
    ]
}

/// Datenabfluss-Sicht (S3-A8): gesendet/blockiert je Request MIT Grund.
pub fn egress_view(sent: &[(String, String)], blocked: &[(String, String)]) -> Vec<ViewItem> {
    let mut items: Vec<ViewItem> = sent
        .iter()
        .map(|(req, digest)| ViewItem {
            label: format!("gesendet {req}"),
            value: format!("kontext-digest {digest}"),
            source_ref: format!("evidence:context:{req}"),
        })
        .collect();
    items.extend(blocked.iter().map(|(req, grund)| ViewItem {
        label: format!("blockiert {req}"),
        value: grund.clone(),
        source_ref: format!("gate:prompt_context:{req}"),
    }));
    items
}

/// CandidateOutput-Sicht (S3-A9): erzeugendes Modell, Gate-Ergebnisse,
/// Verbleib (→PhaseBlock | →Residue), Confirmation-Status — „warum
/// nicht committed" in einem Klick. COCK-INV-8: die Einschaetzung wird
/// NIE als Verdikt gerendert.
pub fn candidate_output_view(
    candidate_id: &str,
    model: &str,
    gate_results: &str,
    fate: &str,
    confirmed: bool,
    self_assessment_permille: Option<u16>,
) -> Vec<ViewItem> {
    let mut items = vec![ViewItem {
        label: format!("Kandidat {candidate_id}"),
        value: format!(
            "von {model} · Gates: {gate_results} · Verbleib: {fate} · Bestaetigung: {}",
            if confirmed {
                "aufgezeichnet"
            } else {
                "ausstehend"
            }
        ),
        source_ref: format!("candidate:{candidate_id}"),
    }];
    if let Some(p) = self_assessment_permille {
        items.push(ViewItem {
            label: "Einschaetzung".into(),
            value: format!("{p}‰ — Einschaetzung, kein Urteil"),
            source_ref: format!("candidate:{candidate_id}:assessment"),
        });
    }
    items
}
