//! Der D1–D6-Matrixbau (Dokument 20 §3/§8). Die kategorische Wahrheit
//! ueber beide Arme: D1/D2/D3/D6 sind fuer den Raw-Arm STRUKTURELL nicht
//! vorhanden (Tatsachenfeststellung, kein Werturteil — es gibt dort
//! schlicht keinen Mechanismus), D4/D5 sind Aufgabenparitaet/Autonomie
//! (Parität genuegt laut Messlatte 17 §4).

use crate::model::{
    CceRunResult, ComparisonMatrix, ExternalToolResult, MatrixCell, MatrixRow, RawRunResult,
    TaskClass,
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

// ---------- Drei-Arm-Matrix (Dokument 22 §2, additiv) ----------

/// Eine D-Zeile fuer den Drei-Arm-Vergleich: Raw / CCE / ExternalTool.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThreeArmMatrixRow {
    pub dimension: String,
    pub raw: MatrixCell,
    pub cce: MatrixCell,
    pub ext: MatrixCell,
}

/// Die D1–D6-Matrix mit drei Armen je Aufgabenklasse (§2). Traegt
/// zusaetzlich `tool_name` (das beobachtete Fremdwerkzeug) — CCE stellt
/// keine Behauptung ueber das Werkzeug auf, ausser dem beobachteten Namen.
#[derive(Debug, Clone)]
pub struct ThreeArmComparisonMatrix {
    pub package_id: String,
    pub task_class: TaskClass,
    pub tool_name: String,
    pub rows: Vec<ThreeArmMatrixRow>,
}

impl ThreeArmComparisonMatrix {
    pub fn is_complete(&self) -> bool {
        self.rows.len() == 6
    }
}

/// Baut die vollstaendige Drei-Arm-D1–D6-Matrix. Der ExternalTool-Arm
/// wird — wie der Raw-Arm — in D1/D2/D3/D6 als STRUKTURELL nicht
/// vorhanden gefuehrt: CCE kann das Innere des Fremdwerkzeugs nicht
/// beobachten und stellt keinen Beweis-/Replay-/Governance-/
/// Auditierbarkeits-Mechanismus fest (§1: kein Versuch, das Werkzeug zu
/// zertifizieren). D4/D5 tragen die beobachteten Tatsachen (Kriterien,
/// Eingriffe) — Paritaet genuegt (Messlatte). Voraussetzung (Aufrufer
/// prueft via `three_arm_fairness_gate`): alle Arme tragen denselben
/// geerdeten `task_package_digest`, Fremdarm strikt vor CCE.
pub fn build_three_arm_matrix(
    task_class: TaskClass,
    raw: &RawRunResult,
    cce: &CceRunResult,
    ext: &ExternalToolResult,
) -> ThreeArmComparisonMatrix {
    // Der ExternalTool-Arm ist in den vier kategorischen Dimensionen
    // strukturell nicht beobachtbar (kein von CCE einsehbarer Mechanismus).
    let ext_absent = || MatrixCell {
        verdict: "structurally_absent".to_string(),
        beleg: format!(
            "Fremdwerkzeug '{}' — kein von CCE beobachtbarer Beweis-/Replay-/Governance-/\
             Auditierbarkeits-Mechanismus",
            ext.tool_name
        ),
    };

    // Zuerst die bestehende Zwei-Arm-Matrix (Raw/CCE, D1–D6) bauen und
    // die Raw-/CCE-Zellen woertlich uebernehmen — die kategorische
    // Wahrheit ueber diese beiden Arme ist unveraendert (P4).
    let base = build_matrix(task_class, raw, cce);

    let rows = base
        .rows
        .into_iter()
        .enumerate()
        .map(|(i, row)| {
            let ext_cell = match i {
                // D1/D2/D3/D6 (Index 0,1,2,5): strukturell nicht vorhanden.
                0 | 1 | 2 | 5 => ext_absent(),
                // D4 Aufgabenparitaet: mechanisch geprueft (§1).
                3 => MatrixCell::criteria(
                    ext.criteria_met,
                    &format!(
                        "success_criteria mechanisch geprueft ({}), output_digest {}",
                        ext.tool_name,
                        ext.output_digest()
                    ),
                ),
                // D5 Autonomie: beobachtete Eingriffe (keine
                // Menschenhoheits-Aufzeichnung im Fremdwerkzeug).
                _ => MatrixCell {
                    verdict: "observed".to_string(),
                    beleg: format!(
                        "{} Eingriffe ({}, keine Menschenhoheits-Aufzeichnung): {}",
                        ext.human_interventions_count, ext.tool_name, ext.observer_note
                    ),
                },
            };
            ThreeArmMatrixRow {
                dimension: row.dimension,
                raw: row.raw,
                cce: row.cce,
                ext: ext_cell,
            }
        })
        .collect();

    ThreeArmComparisonMatrix {
        package_id: raw.package_id.clone(),
        task_class,
        tool_name: ext.tool_name.clone(),
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

    fn ext_arm(criteria_met: bool) -> ExternalToolResult {
        ExternalToolResult::observed(
            "p",
            "grounded",
            "packet",
            "Cursor",
            Some("0.42"),
            b"pub fn add(a:i32,b:i32)->i32{a+b}".to_vec(),
            60_000,
            2,
            criteria_met,
            "manuell ausgefuehrt",
            1,
        )
    }

    #[test]
    fn three_arm_matrix_ext_absent_in_categorical_criteria_in_d4() {
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
        let ext = ext_arm(true);
        let m = build_three_arm_matrix(TaskClass::Coding, &raw, &cce, &ext);
        assert!(m.is_complete());
        assert_eq!(m.tool_name, "Cursor");
        // D1/D2/D3/D6: Raw UND ExternalTool strukturell nicht vorhanden,
        // CCE present (D1/D2/D3/D6 kategorisch, §6 DoD).
        for i in [0usize, 1, 2, 5] {
            assert_eq!(m.rows[i].raw.verdict, "structurally_absent");
            assert_eq!(m.rows[i].ext.verdict, "structurally_absent");
            assert_eq!(m.rows[i].cce.verdict, "present");
        }
        // D4: alle drei Arme criteria_met (Paritaet).
        assert_eq!(m.rows[3].raw.verdict, "criteria_met");
        assert_eq!(m.rows[3].cce.verdict, "criteria_met");
        assert_eq!(m.rows[3].ext.verdict, "criteria_met");
        // D5: ExternalTool observed.
        assert_eq!(m.rows[4].ext.verdict, "observed");
    }

    #[test]
    fn three_arm_matrix_reflects_ext_failure_honestly() {
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
        // Fremdwerkzeug besteht die Kriterien NICHT — steht so in D4.
        let ext = ext_arm(false);
        let m = build_three_arm_matrix(TaskClass::Coding, &raw, &cce, &ext);
        assert_eq!(m.rows[3].ext.verdict, "criteria_failed");
    }
}
