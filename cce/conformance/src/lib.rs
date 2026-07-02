//! cce-conformance — uebergreifende Testmatrix + Einstiegspunkt des
//! Regressionswaechters (S8.3): Ab G6 prueft jeder Lauf ALLE bisherigen
//! Zeugen (Referenz-Cubes bleiben gruen, Negativ-Cubes bleiben rot).
//! In G0 ist das Harness leer, aber vorhanden und CI-gebunden.

/// Kennung des Regressionswaechters; waechst mit jeder Phase.
pub const GUARD_PHASES: &[&str] = &["G0", "G1", "G2", "G3", "G4", "G5"];

#[cfg(test)]
mod tests {
    #[test]
    fn harness_exists() {
        assert!(crate::GUARD_PHASES.contains(&"G0"));
    }
}
