//! Die zwei neuen P4-Gates (Dokument 20 §4): FairnessGate +
//! ComparisonSealGate. Dieselbe `InfVerdict`-Form wie alle P1/P2/P3-Gates
//! (aus `cce_swe::gates` wiederverwendet, nicht dupliziert).

use crate::model::{
    BenchmarkTaskPackage, CceRunResult, ComparisonMatrix, ExternalToolResult, RawRunResult,
};
use crate::residues::benchmark_residue;
pub use cce_swe::gates::InfVerdict;
use cce_swe::grounding::GroundingPacket;

/// FairnessGate (§2, strukturell): beide Arme MUESSEN denselben
/// `task_package_digest` tragen, und der ungegatete Arm MUSS VOR dem
/// CCE-Arm abgegeben haben (`submission_order` streng kleiner). Geprueft
/// ueber Digest-Gleichheit + Reihenfolge — keine Seite sieht die Antwort
/// der anderen vor der eigenen Abgabe.
pub fn fairness_gate(raw: &RawRunResult, cce: &CceRunResult) -> InfVerdict {
    if raw.task_package_digest != cce.task_package_digest {
        return InfVerdict::Reject {
            gate: "FairnessGate".into(),
            residue: Box::new(benchmark_residue(
                "benchmark_fairness_digest_mismatch",
                &format!(
                    "Raw-Arm task_package_digest {} != CCE-Arm {}",
                    raw.task_package_digest, cce.task_package_digest
                ),
            )),
        };
    }
    if raw.submission_order >= cce.submission_order {
        return InfVerdict::Reject {
            gate: "FairnessGate".into(),
            residue: Box::new(benchmark_residue(
                "benchmark_fairness_order_violation",
                &format!(
                    "Raw-Arm submission_order {} nicht strikt vor CCE-Arm {} — der ungegatete \
                     Arm muss zuerst und isoliert abgeben",
                    raw.submission_order, cce.submission_order
                ),
            )),
        };
    }
    InfVerdict::Allow {
        gate: "FairnessGate".into(),
        reason: "identischer task_package_digest, Raw-Arm strikt vor CCE-Arm".into(),
    }
}

/// ComparisonSealGate (§4): die Matrix darf NUR gebaut/versiegelt werden,
/// wenn beide Arme abgeschlossen sind UND `task_package_digest` auf
/// beiden Seiten identisch ist.
pub fn comparison_seal_gate(raw: Option<&RawRunResult>, cce: Option<&CceRunResult>) -> InfVerdict {
    let (raw, cce) = match (raw, cce) {
        (Some(r), Some(c)) => (r, c),
        _ => {
            return InfVerdict::Reject {
                gate: "ComparisonSealGate".into(),
                residue: Box::new(benchmark_residue(
                    "benchmark_arms_incomplete",
                    "Matrixbau vor Abschluss beider Arme — nicht zulaessig",
                )),
            };
        }
    };
    if raw.task_package_digest != cce.task_package_digest {
        return InfVerdict::Reject {
            gate: "ComparisonSealGate".into(),
            residue: Box::new(benchmark_residue(
                "benchmark_fairness_digest_mismatch",
                "task_package_digest der Arme weicht ab — keine Matrix",
            )),
        };
    }
    InfVerdict::Allow {
        gate: "ComparisonSealGate".into(),
        reason: "beide Arme abgeschlossen, identischer task_package_digest".into(),
    }
}

/// FairnessGate erweitert (Dokument 22 §3): der Drei-Arm-Fall. Prueft
/// ZUSAETZLICH `packet_digest`-Gleichheit ueber alle Arme — beide Seiten
/// erhalten dasselbe GroundingPacket, sonst ist der Vergleich nicht fair.
/// Da der `packet_digest` per `grounded_task_package_digest` in den
/// `task_package_digest` eingefaltet ist (§3), ist ein Regel-Unterschied
/// strukturell ausgeschlossen; dieses Gate macht die Pruefung zusaetzlich
/// EXPLIZIT und benannt:
///   1. Raw/CCE fair (bestehendes `fairness_gate`: identischer Digest,
///      Raw strikt vor CCE).
///   2. Der gemeinsame Digest MUSS der geerdete sein (Packet eingefaltet).
///   3. Der Fremdarm traegt denselben geerdeten `task_package_digest`.
///   4. Der `packet_digest` des Fremdarms == der des GroundingPacket
///      (explizite packet_digest-Gleichheit, N-BENCH-EXT-1).
///   5. Der Fremdarm gibt strikt VOR dem CCE-Arm ab (zuerst und isoliert).
pub fn three_arm_fairness_gate(
    package: &BenchmarkTaskPackage,
    packet: &GroundingPacket,
    raw: &RawRunResult,
    cce: &CceRunResult,
    ext: &ExternalToolResult,
) -> InfVerdict {
    // 1. Raw/CCE-Fairness (Digest-Gleichheit + Reihenfolge) unveraendert.
    let base = fairness_gate(raw, cce);
    if !base.allows() {
        return base;
    }
    // 2./3. Der gemeinsame Digest muss der GEERDETE sein und alle drei
    // Arme muessen ihn tragen (Regel-Unterschied strukturell
    // ausgeschlossen).
    let expected = package.grounded_digest_hex(packet);
    if cce.task_package_digest != expected {
        return InfVerdict::Reject {
            gate: "FairnessGate".into(),
            residue: Box::new(benchmark_residue(
                "benchmark_fairness_digest_mismatch",
                &format!(
                    "CCE-/Raw-Arm tragen nicht den geerdeten task_package_digest {expected} \
                     (packet_digest nicht eingefaltet)"
                ),
            )),
        };
    }
    if ext.task_package_digest != expected {
        return InfVerdict::Reject {
            gate: "FairnessGate".into(),
            residue: Box::new(benchmark_residue(
                "benchmark_fairness_digest_mismatch",
                &format!(
                    "Fremdarm task_package_digest {} != geerdeter Digest {expected}",
                    ext.task_package_digest
                ),
            )),
        };
    }
    // 4. Explizite packet_digest-Gleichheit (§3, N-BENCH-EXT-1).
    let packet_digest = packet.digest_hex();
    if ext.packet_digest != packet_digest {
        return InfVerdict::Reject {
            gate: "FairnessGate".into(),
            residue: Box::new(benchmark_residue(
                "benchmark_packet_digest_mismatch",
                &format!(
                    "Fremdarm erhielt packet_digest {} != gemeinsames GroundingPacket {} — die \
                     Arme sahen NICHT dasselbe Regelwerk",
                    ext.packet_digest, packet_digest
                ),
            )),
        };
    }
    // 5. Der Fremdarm gibt strikt vor dem CCE-Arm ab (§3: zuerst und
    // isoliert, damit CCE nicht beeinflusst sein kann).
    if ext.submission_order >= cce.submission_order {
        return InfVerdict::Reject {
            gate: "FairnessGate".into(),
            residue: Box::new(benchmark_residue(
                "benchmark_fairness_order_violation",
                &format!(
                    "Fremdarm submission_order {} nicht strikt vor CCE-Arm {} — der Fremdarm muss \
                     zuerst und isoliert abgeben",
                    ext.submission_order, cce.submission_order
                ),
            )),
        };
    }
    InfVerdict::Allow {
        gate: "FairnessGate".into(),
        reason: "alle drei Arme: identischer geerdeter task_package_digest, identischer \
                 packet_digest, Fremdarm strikt vor CCE-Arm"
            .into(),
    }
}

/// Matrix-Vollstaendigkeit (§5, fuer den Container-Verify gespiegelt):
/// genau sechs D-Zeilen.
pub fn matrix_complete_gate(matrix: &ComparisonMatrix) -> InfVerdict {
    if matrix.is_complete() {
        InfVerdict::Allow {
            gate: "MatrixCompleteGate".into(),
            reason: "D1–D6 vollstaendig".into(),
        }
    } else {
        InfVerdict::Reject {
            gate: "MatrixCompleteGate".into(),
            residue: Box::new(benchmark_residue(
                "benchmark_matrix_incomplete",
                &format!("{} Matrix-Zeilen statt 6", matrix.rows.len()),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::TaskClass;

    fn raw(digest: &str, order: u64) -> RawRunResult {
        RawRunResult::observed("p", digest, "o-raw", 100, Some(true), Some(true), 2, order)
    }
    fn cce(digest: &str, order: u64) -> CceRunResult {
        CceRunResult::certified(
            "p",
            digest,
            "o-cce",
            120,
            Some(true),
            Some(true),
            0,
            order,
            "root",
            12,
            true,
        )
    }

    #[test]
    fn fairness_gate_allows_correct_order_and_matching_digest() {
        assert!(fairness_gate(&raw("d", 1), &cce("d", 2)).allows());
    }

    #[test]
    fn fairness_gate_rejects_digest_mismatch() {
        let v = fairness_gate(&raw("d-raw", 1), &cce("d-cce", 2));
        assert!(!v.allows());
        assert!(v
            .residue()
            .unwrap()
            .id
            .contains("benchmark_fairness_digest_mismatch"));
    }

    #[test]
    fn fairness_gate_rejects_cce_before_raw() {
        let v = fairness_gate(&raw("d", 5), &cce("d", 2));
        assert!(!v.allows());
        assert!(v
            .residue()
            .unwrap()
            .id
            .contains("benchmark_fairness_order_violation"));
    }

    #[test]
    fn comparison_seal_gate_rejects_incomplete_arms() {
        let v = comparison_seal_gate(Some(&raw("d", 1)), None);
        assert!(!v.allows());
        assert!(v
            .residue()
            .unwrap()
            .id
            .contains("benchmark_arms_incomplete"));
    }

    #[test]
    fn comparison_seal_gate_allows_both_done_matching() {
        assert!(comparison_seal_gate(Some(&raw("d", 1)), Some(&cce("d", 2))).allows());
    }

    #[test]
    fn matrix_complete_gate_rejects_short_matrix() {
        let m = ComparisonMatrix {
            package_id: "p".to_string(),
            task_class: TaskClass::Coding,
            rows: vec![],
        };
        assert!(!matrix_complete_gate(&m).allows());
    }

    // ---------- Drei-Arm-Fairness (Dokument 22 §3) ----------

    fn tarm_package() -> BenchmarkTaskPackage {
        BenchmarkTaskPackage {
            package_id: "p".to_string(),
            task_class: TaskClass::Coding,
            task_text: "fix".to_string(),
            starting_files: vec![("src/lib.rs".to_string(), b"x".to_vec())],
            success_criteria: "test gruen".to_string(),
            build_command: vec!["cargo".to_string(), "build".to_string()],
            test_command: vec!["cargo".to_string(), "test".to_string()],
            target_path: "src/lib.rs".to_string(),
        }
    }

    fn tarm_packet(pkg_id: &str) -> GroundingPacket {
        use cce_swe::grounding::{compile_grounding, RuleAtom, RuleSeverity};
        compile_grounding(
            pkg_id,
            vec![RuleAtom {
                rule_id: "no-unwrap".to_string(),
                scope: "src/".to_string(),
                trigger: ".unwrap()".to_string(),
                prescription: "Fehler propagieren".to_string(),
                severity: RuleSeverity::Blocking,
                evidence_ref: Some("CLAUDE.md".to_string()),
                gate_ref: None,
                decay: None,
            }],
            vec![],
            vec!["fs_write".to_string()],
        )
        .packet
    }

    fn ext(digest: &str, packet_digest: &str, order: u64) -> ExternalToolResult {
        ExternalToolResult::observed(
            "p",
            digest,
            packet_digest,
            "Cursor",
            Some("0.42"),
            b"pub fn add(a:i32,b:i32)->i32{a+b}".to_vec(),
            60_000,
            2,
            true,
            "manuell ausgefuehrt",
            order,
        )
    }

    #[test]
    fn three_arm_fairness_allows_identical_grounded_digest_and_packet() {
        let pkg = tarm_package();
        let packet = tarm_packet("pkt");
        let d = pkg.grounded_digest_hex(&packet);
        let pd = packet.digest_hex();
        // Fremdarm (order 1) strikt vor CCE (order 3); Raw (order 2) < CCE.
        let v = three_arm_fairness_gate(&pkg, &packet, &raw(&d, 2), &cce(&d, 3), &ext(&d, &pd, 1));
        assert!(v.allows(), "{v:?}");
    }

    #[test]
    fn three_arm_fairness_rejects_ungrounded_digest() {
        let pkg = tarm_package();
        let packet = tarm_packet("pkt");
        let pd = packet.digest_hex();
        // Die Arme tragen den UNGEERDETEN Digest (packet nicht eingefaltet).
        let d = pkg.digest_hex();
        let v = three_arm_fairness_gate(&pkg, &packet, &raw(&d, 2), &cce(&d, 3), &ext(&d, &pd, 1));
        assert!(!v.allows());
        assert!(v
            .residue()
            .unwrap()
            .id
            .contains("benchmark_fairness_digest_mismatch"));
    }

    #[test]
    fn three_arm_fairness_rejects_ext_packet_digest_mismatch() {
        let pkg = tarm_package();
        let packet = tarm_packet("pkt");
        let d = pkg.grounded_digest_hex(&packet);
        // Fremdarm behauptet den geerdeten Digest, deklariert aber einen
        // ABWEICHENDEN packet_digest — er sah nicht dasselbe Regelwerk.
        let v = three_arm_fairness_gate(
            &pkg,
            &packet,
            &raw(&d, 2),
            &cce(&d, 3),
            &ext(&d, "ein-anderer-packet-digest", 1),
        );
        assert!(!v.allows());
        assert!(v
            .residue()
            .unwrap()
            .id
            .contains("benchmark_packet_digest_mismatch"));
    }

    #[test]
    fn three_arm_fairness_rejects_ext_not_before_cce() {
        let pkg = tarm_package();
        let packet = tarm_packet("pkt");
        let d = pkg.grounded_digest_hex(&packet);
        let pd = packet.digest_hex();
        // Fremdarm (order 5) NICHT vor CCE (order 3).
        let v = three_arm_fairness_gate(&pkg, &packet, &raw(&d, 2), &cce(&d, 3), &ext(&d, &pd, 5));
        assert!(!v.allows());
        assert!(v
            .residue()
            .unwrap()
            .id
            .contains("benchmark_fairness_order_violation"));
    }
}
