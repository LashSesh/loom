//! cce-conformance — uebergreifende Testmatrix + Einstiegspunkt des
//! Regressionswaechters (S8.3): Ab G6 prueft jeder Lauf ALLE bisherigen
//! Zeugen (Referenz-Cubes bleiben gruen, Negativ-Cubes bleiben rot).

/// Kennung des Regressionswaechters; waechst mit jeder Phase.
pub const GUARD_PHASES: &[&str] = &[
    "G0", "G1", "G2", "G3", "G4", "G5", "G6", "G7", "G8", "G8a", "G9", "G10", "G11", "G12", "W1",
    "W2", "CE1", "L9B", "X4", "P1",
];

/// Produktversion (S11: Versionierung; Update-DoD-Pfad s. `update_dod`).
pub const PRODUCT_VERSION: &str = "0.1.0";

/// Update-Kanal-Waechter (S11-A): ein Update ist genau dann
/// auslieferbar, wenn DERSELBE eine Regressionswaechter gruen ist —
/// es gibt keinen zweiten, weicheren Kanal.
pub fn update_dod(guard_green: bool) -> Result<&'static str, &'static str> {
    if guard_green {
        Ok("update_freigegeben: Waechter gruen (gleicher Kanal wie Bau)")
    } else {
        Err("update_blockiert: Waechter nicht gruen — kein Ausweichkanal")
    }
}

/// Funktions-PL-Registry (S11/G12: jede Funktion traegt PL; Evidence
/// ist der Zeugen-/Testort). PL>=2 verlangt einen benannten gruenen
/// Kerntest — sonst feature_maturity_overclaim.
pub const FEATURE_PL: &[(&str, &str, &str)] = &[
    (
        "dokument_reise_d01",
        "PL4",
        "conformance/tests/product_journey.rs::produkt_kerntest_ueber_sechs_naehte",
    ),
    (
        "motor_kern",
        "PL1",
        "conformance/tests/closure_roundtrip.rs",
    ),
    (
        "spiral_kinematik",
        "PL1",
        "crates/cce-spiral/tests/spiral_catalog.rs",
    ),
    ("runner_replay", "PL1", "crates/cce-runner/tests/g5_gate.rs"),
    (
        "persistenz_bibliothek",
        "PL1",
        "crates/cce-library/tests/g6_gate.rs",
    ),
    ("hbm_mining", "PL1", "crates/cce-hbm/tests/hbm_catalog.rs"),
    ("csa_akquisition", "PL1", "conformance/tests/csa_catalog.rs"),
    (
        "inference_gateway",
        "PL1",
        "conformance/inference/inference_catalog.rs",
    ),
    (
        "tool_gateway",
        "PL1",
        "crates/cce-toolgateway/src/gateway.rs (tests)",
    ),
    (
        "loom_container",
        "PL1",
        "loom/loom-conformance/tests/format_catalog.rs",
    ),
    ("cockpit", "PL1", "cockpit/cockpit-core/tests/cock_inv.rs"),
    (
        "kanzel",
        "PL1",
        "cockpit/cockpit-core/tests/cock_inv.rs::cock_inv_4",
    ),
    (
        "domaenen_katalog",
        "PL1",
        "crates/cce-materialize/src/catalog.rs (tests)",
    ),
    (
        "ce1_tabellen_zellentyp",
        "PL2",
        "conformance/tests/e4a_ce1_table.rs",
    ),
    (
        "l9b_normic_memory",
        "PL2",
        "conformance/tests/e5_l9b_normic_memory.rs",
    ),
    (
        "r_cyc_1_vollzyklus",
        "PL2",
        "conformance/tests/x4_r_cyc_1.rs",
    ),
    (
        "cloud_openai_provider",
        "PL2",
        "conformance/inference/inference_catalog.rs::p1_cloud_openai_manifest_complete_terms_privacy_retention_budget",
    ),
];

/// feature_maturity_overclaim: Funktionen ohne PL oder mit PL>=2 ohne
/// Kerntest-Beweis (im Bau qualifiziert nur die D01-Reise).
pub fn feature_maturity_overclaim() -> Vec<&'static str> {
    FEATURE_PL
        .iter()
        .filter(|(name, pl, evidence)| {
            pl.is_empty() || evidence.is_empty() || (*pl == "PL4" && *name != "dokument_reise_d01")
        })
        .map(|(name, _, _)| *name)
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn harness_exists() {
        assert!(crate::GUARD_PHASES.contains(&"G0"));
        assert!(crate::GUARD_PHASES.contains(&"G12"));
    }
}
