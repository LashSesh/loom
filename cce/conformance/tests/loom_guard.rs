//! Der EINE Waechter deckt ab G9 auch die Formatzeugen (LOOM Teil 10.3):
//! alle 8 Referenzen muessen valid bleiben — jede Regression im
//! Format-Stack faellt im selben CI-Lauf wie Motor-/CSA-/Inference-Zeugen.

#[test]
fn format_witnesses_under_the_one_guard() {
    assert!(cce_conformance::GUARD_PHASES.contains(&"G9"));
    for (name, builder) in loom_conformance::REFERENCE_BUILDERS {
        let sealed = builder();
        let report = loom_verify::verify(&sealed.bytes);
        assert!(
            matches!(
                report.verdict,
                loom_verify::Verdict::Valid | loom_verify::Verdict::ValidWithResidues
            ),
            "{name} unter dem Waechter nicht mehr valid: {:?}",
            report.diagnoses
        );
    }
}
