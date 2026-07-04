//! Der D1–D6-Matrixbau (Dokument 20 §3/§8). Die kategorische Wahrheit
//! ueber beide Arme: D1/D2/D3/D6 sind fuer den Raw-Arm STRUKTURELL nicht
//! vorhanden (Tatsachenfeststellung, kein Werturteil — es gibt dort
//! schlicht keinen Mechanismus), D4/D5 sind Aufgabenparitaet/Autonomie
//! (Parität genuegt laut Messlatte 17 §4).

use crate::model::{
    CceRunResult, ComparisonMatrix, MatrixCell, MatrixRow, RawRunResult, TaskClass,
};

/// Baut die vollstaendige D1–D6-Matrix aus beiden Arm-Ergebnissen.
/// Voraussetzung (Aufrufer prueft via `comparison_seal_gate`): beide
/// Arme abgeschlossen, identischer `task_package_digest`.
pub fn build_matrix(
    task_class: TaskClass,
    raw: &RawRunResult,
    cce: &CceRunResult,
) -> ComparisonMatrix {
    let rows = vec![
        // D1 Nachweisbarkeit: Beweis + Replay je Ergebnis. Der Raw-Arm
        // traegt strukturell keine Evidence.
        MatrixRow {
            dimension: "D1 Nachweisbarkeit".to_string(),
            raw: MatrixCell::structurally_absent(),
            cce: MatrixCell::present(&format!(
                "InferenceEvidence + {} GateReports, RepoWorkbody {}",
                cce.gate_report_count, cce.repo_workbody_ref
            )),
        },
        // D2 Wiederholbarkeit: gleicher Auftrag ⇒ gleiche Klasse,
        // maschinell geprueft. Der Raw-Arm hat keinen Replay-Mechanismus.
        MatrixRow {
            dimension: "D2 Wiederholbarkeit".to_string(),
            raw: MatrixCell::structurally_absent(),
            cce: if cce.replay_confirmed {
                MatrixCell::present("Replay: zweiter Lauf klassenidentisch (recorded)")
            } else {
                MatrixCell {
                    verdict: "present_unconfirmed".to_string(),
                    beleg: "Replay-Mechanismus vorhanden, in diesem Lauf nicht bestaetigt"
                        .to_string(),
                }
            },
        },
        // D3 Governance: kein Egress ohne Tor, kein stilles Scheitern.
        // Der Raw-Arm hat keine Gate-Kette.
        MatrixRow {
            dimension: "D3 Governance".to_string(),
            raw: MatrixCell::structurally_absent(),
            cce: MatrixCell::present(&format!(
                "volle Vor-Egress-Gate-Kette + {} Werkzeug-/Kern-GateReports, fail-closed",
                cce.gate_report_count
            )),
        },
        // D4 Aufgabenparitaet: identische Aufgabe, Qualitaet >= Gegner.
        // BEIDE Arme werden gegen dieselben success_criteria gemessen.
        MatrixRow {
            dimension: "D4 Aufgabenparitaet".to_string(),
            raw: MatrixCell::criteria(
                raw.criteria_met(),
                &format!(
                    "build={:?} test={:?} (beobachtet, ungegatet)",
                    raw.build_pass, raw.test_pass
                ),
            ),
            cce: MatrixCell::criteria(
                cce.criteria_met(),
                &format!(
                    "build={:?} test={:?} (gate-bezeugt)",
                    cce.build_pass, cce.test_pass
                ),
            ),
        },
        // D5 Autonomie unter Gates: lange Ketten ohne Menschenkorrektur,
        // mit Menschenhoheit. Beide Zellen tragen die Eingriffszahl;
        // der CCE-Arm zusaetzlich "unter Gates".
        MatrixRow {
            dimension: "D5 Autonomie unter Gates".to_string(),
            raw: MatrixCell {
                verdict: "observed".to_string(),
                beleg: format!(
                    "{} Eingriffe (ungegatet, keine Menschenhoheits-Aufzeichnung)",
                    raw.human_interventions_count
                ),
            },
            cce: MatrixCell {
                verdict: "observed".to_string(),
                beleg: format!(
                    "{} Eingriffe unter Gates (HumanConfirmationGate-Hoheit aufgezeichnet)",
                    cce.human_interventions_count
                ),
            },
        },
        // D6 Auditierbarkeit: der gesamte Arbeitsweg als pruefbarer
        // .loom-Koerper. Der Raw-Arm hinterlaesst keinen .loom-Koerper.
        MatrixRow {
            dimension: "D6 Auditierbarkeit".to_string(),
            raw: MatrixCell::structurally_absent(),
            cce: MatrixCell::present(&format!(
                "zertifizierter .loom-RepoWorkbody {} (verify == Valid)",
                cce.repo_workbody_ref
            )),
        },
    ];
    ComparisonMatrix {
        package_id: raw.package_id.clone(),
        task_class,
        rows,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_has_six_rows_raw_absent_in_d1_d2_d3_d6() {
        let raw = RawRunResult::observed("p", "d", "o", 100, Some(true), Some(true), 3, 1);
        let cce = CceRunResult::certified(
            "p",
            "d",
            "o",
            120,
            Some(true),
            Some(true),
            0,
            2,
            "root34",
            12,
            true,
        );
        let m = build_matrix(TaskClass::Coding, &raw, &cce);
        assert!(m.is_complete());
        // D1/D2/D3/D6 (Index 0,1,2,5): Raw strukturell nicht vorhanden.
        for i in [0usize, 1, 2, 5] {
            assert_eq!(m.rows[i].raw.verdict, "structurally_absent");
            assert_eq!(m.rows[i].cce.verdict, "present");
        }
        // D4: beide Arme criteria_met.
        assert_eq!(m.rows[3].raw.verdict, "criteria_met");
        assert_eq!(m.rows[3].cce.verdict, "criteria_met");
    }
}
