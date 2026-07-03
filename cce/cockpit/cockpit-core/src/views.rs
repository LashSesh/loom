//! Die Ansichten (S3.2 Pruef-Flaeche-Rendering + S3-A1..A6 + Overlay
//! S3-A7..A10). Jede Anzeige ist WURZEL-RUECKFUEHRBAR: jedes Element
//! traegt seine Motor-Quelle (source_ref) — das Cockpit rendert
//! Fakten, es erzeugt sie nie (COCK-INV-1). Alle Strukturen sind
//! bewusst OHNE Setter fuer Urteils-/Residuenfelder konstruiert:
//! sie entstehen nur aus Motor-Artefakten.

use cce_core::gate::GateReport;
use cce_core::ledger::Ledger;
use cce_materialize::document::DocCrystal;

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

// ---------- LC-R5: Minimal-GUI-Inspektionspfad (5 Pflichtansichten) ----------
//
// spec/40_format/LOOM_CONTAINER_STANDARD_V1.md §Minimal-GUI-Inspektionspfad:
// "(1) Manifest (Klasse, PL, Claims, Profile), (2) Segmentliste mit
// Digest-Status, (3) Residuen + Verdikt, (4) Gate-Reports, (5)
// Ledger/PhaseBlocks — ohne Ausfuehrung, ohne Netz, ohne Schreibpfad."
// Kategorien 3/4 sind bereits residue_view/gate_report_view; hier die
// restlichen drei, bezogen auf DIESEN Arbeitskoerper (Dokument-Domaene-
// Lauf), nicht auf eine generische fremde .loom-Datei — das Cockpit ist
// kein universeller .loom-Viewer, sondern liest die eigenen Motor-Fakten
// in derselben Fuenf-Kategorien-Form.

/// Kategorie 1 — Manifest: Klasse/PL/Claims/Profile DIESES Laufs.
pub fn manifest_view(
    domain_id: &str,
    product_level: &str,
    content_class_hex: &str,
    format: &str,
) -> Vec<ViewItem> {
    vec![
        ViewItem {
            label: "Klasse".into(),
            value: format!("{domain_id} · content_class {content_class_hex}"),
            source_ref: format!("manifest:class:{domain_id}"),
        },
        ViewItem {
            label: "PL".into(),
            value: product_level.into(),
            source_ref: format!("catalog:{domain_id}:level"),
        },
        ViewItem {
            label: "Claims".into(),
            value: "materialize(document) · kein Egress im Kernpfad".into(),
            source_ref: "manifest:claims".into(),
        },
        ViewItem {
            label: "Profile".into(),
            value: format!("format {format} · digest sha2-256 (Zwei-Digest-Modell S7)"),
            source_ref: "manifest:profile".into(),
        },
    ]
}

/// Kategorie 2 — Segmentliste mit Digest-Status: die DocUnits des
/// bestaetigten Crystals als Segment-Analog (id/typ/naht-anzahl), plus
/// das Digest-Paar des Artefakts (byte_digest bindet die Datei,
/// content_class die Bedeutung — S7).
pub fn segment_list_view(crystal: &DocCrystal, byte_digest_hex: Option<&str>) -> Vec<ViewItem> {
    let mut items: Vec<ViewItem> = crystal
        .units
        .iter()
        .map(|u| ViewItem {
            label: format!("Segment {}", u.id),
            value: format!("{} · {} Naht/Naehte", u.unit_type.as_str(), u.seams.len()),
            source_ref: format!("crystal:unit:{}", u.id),
        })
        .collect();
    items.push(ViewItem {
        label: "Digest-Status".into(),
        value: match byte_digest_hex {
            Some(h) => format!("byte_digest {h}"),
            None => "noch kein Artefakt-Digest (vor Naht 4)".into(),
        },
        source_ref: "artifact:byte_digest".into(),
    });
    items
}

/// Kategorie 5 — Ledger/PhaseBlocks: jede Kettenposition (Art + Hash)
/// plus Kettenstatus (INV-12) — read-only, kein Replay-/Schreibpfad.
pub fn ledger_view(ledger: &Ledger) -> Vec<ViewItem> {
    let mut items: Vec<ViewItem> = ledger
        .events()
        .iter()
        .map(|e| ViewItem {
            label: format!("PhaseBlock #{} {}", e.seq, e.kind.as_str()),
            value: format!("entry_hash {}", e.entry_hash.to_hex()),
            source_ref: format!("ledger:seq:{}", e.seq),
        })
        .collect();
    items.push(ViewItem {
        label: "Kettenstatus".into(),
        value: if ledger.verify().is_ok() {
            "gruen (Hash-Kette geprueft, INV-12)".into()
        } else {
            "gebrochen — Manipulation erkannt".into()
        },
        source_ref: "ledger:chain".into(),
    });
    items
}

/// Verdikt-Zeile fuer Kategorie 3 (Residuen + VERDIKT): eine einzelne,
/// wurzel-rueckfuehrbare Zusammenfassung ueber alle Gate-Reports —
/// ergaenzt residue_view, ersetzt gate_report_view nicht.
pub fn verdict_view(gate_reports: &[GateReport]) -> ViewItem {
    let all_green = gate_reports.iter().all(|g| g.is_pass());
    ViewItem {
        label: "Verdikt".into(),
        value: if gate_reports.is_empty() {
            "noch kein Lauf".to_string()
        } else if all_green {
            format!("Valid ({} Gates gruen)", gate_reports.len())
        } else {
            let holds = gate_reports.iter().filter(|g| !g.is_pass()).count();
            format!("Hold ({holds} von {} Gates rot)", gate_reports.len())
        },
        source_ref: "verdict:from_gate_reports".into(),
    }
}
