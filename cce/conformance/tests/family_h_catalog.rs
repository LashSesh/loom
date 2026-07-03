//! Familie-H-Zeugenkatalog (Welle W14, Masterplan §2): je Domäne
//! OPS01–15 Parität 11/11 · Referenz-Cube schließt (voller Pfad +
//! alle Gates grün) · ≥2 Negativ-Cubes mit ERWARTETEM Kern-Residuum ·
//! Domänen-Kerntest (Reanalyze∘Materialize∘LOOM∘Project∘PHC ≃ id).
//! Der eine Wächter nimmt diese Datei automatisch auf.

use cce_materialize::adapter::{check_adapter_parity_typed, DomainAdapter};
use cce_materialize::document::DocCrystal;
use cce_materialize::family_a::{domain_core_gate, DocDomainAdapter};
use cce_materialize::family_h_domains::all_profiles;
use cce_phc::projection_calc::project;

/// Fährt den geschlossenen Motorpfad für ein Crystal.
fn closed_path(
    adapter: &DocDomainAdapter,
    crystal: &DocCrystal,
) -> cce_materialize::document::DocArtifact {
    let package = adapter.encode(crystal);
    let projection = project(&package, "proj:materialize").expect("Projektion");
    let weave = adapter.loom(&projection).expect("Webstuhl");
    adapter.materialize(&weave)
}

#[test]
fn w14_all_domains_parity_11_of_11() {
    for p in all_profiles() {
        let a = DocDomainAdapter::new(p);
        let report = check_adapter_parity_typed(&a);
        assert!(
            report.is_pass(),
            "{}: Paritaet {}",
            a.domain_id(),
            report.reason
        );
    }
}

#[test]
fn w14_reference_cubes_close_all_gates_green() {
    for p in all_profiles() {
        let a = DocDomainAdapter::new(p);
        let reference = a.reference_cube();
        // Wohlgeformtheit
        assert!(
            a.validate_wish(&reference).is_ok(),
            "{}: reference nicht wohlgeformt",
            a.domain_id()
        );
        // voller Pfad + alle Gates (inkl. Domänen-Kern-Gate) grün
        let artifact = closed_path(&a, &reference);
        let reports = a.run_domain_gates(&reference, Some(&artifact));
        for r in &reports {
            assert!(
                r.is_pass(),
                "{}: Gate {} rot: {}",
                a.domain_id(),
                r.gate_id,
                r.reason
            );
        }
        // Domänen-Kern-Gate explizit grün
        assert!(
            domain_core_gate(&a.profile, &reference).is_pass(),
            "{}: Kern-Gate rot",
            a.domain_id()
        );
    }
}

#[test]
fn w14_negative_cubes_trigger_expected_core_residue() {
    for p in all_profiles() {
        let a = DocDomainAdapter::new(p);
        let negatives = (a.profile.negatives)();
        assert!(negatives.len() >= 2, "{}: <2 Negativ-Cubes", a.domain_id());
        for (cube, expected) in negatives {
            let report = domain_core_gate(&a.profile, &cube);
            assert!(
                !report.is_pass(),
                "{}: Negativ-Cube schliesst faelschlich",
                a.domain_id()
            );
            assert!(
                report.reason.contains(expected),
                "{}: erwartetes Residuum '{}' fehlt in '{}'",
                a.domain_id(),
                expected,
                report.reason
            );
        }
    }
}

#[test]
fn w14_domain_kerntest_reanalyze_is_identity() {
    for p in all_profiles() {
        let a = DocDomainAdapter::new(p);
        let reference = a.reference_cube();
        let artifact = closed_path(&a, &reference);
        let back = a.reanalyze(&artifact).expect("Reanalyse");
        assert!(
            a.equivalent(&back, &reference),
            "{}: Reanalyze∘Materialize∘LOOM∘Project∘PHC ≄ id",
            a.domain_id()
        );
    }
}
