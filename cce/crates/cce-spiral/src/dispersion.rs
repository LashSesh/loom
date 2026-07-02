//! DispersionProfile (R-7, S15.6): Dyadic | Golden | Rational | Adaptive —
//! PROFIL statt Dogma: jedes Profil ist RD-deklariert; ein undeklarierter
//! Fibonacci-/Golden-Import ist ein abzulehnendes Residuum
//! (`fibonacci_dogma_import`, `golden_angle_unjustified`).

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DispersionProfile {
    /// Default (R-7-Empfehlung, kohaerent mit dem dyadischen Zellsubstrat).
    #[default]
    Dyadic,
    /// Goldener Winkel — zulaessig NUR als deklariertes Profil.
    Golden,
    /// Rationaler Winkel p/q Umdrehungen.
    Rational { p: u32, q: u32 },
    /// Adaptiv (RD-gebundene Parameter).
    Adaptive,
}

impl DispersionProfile {
    pub fn as_str(&self) -> String {
        match self {
            DispersionProfile::Dyadic => "dyadic".to_string(),
            DispersionProfile::Golden => "golden".to_string(),
            DispersionProfile::Rational { p, q } => format!("rational:{p}/{q}"),
            DispersionProfile::Adaptive => "adaptive".to_string(),
        }
    }

    /// Winkelinkrement in Milliturns (deterministisch, floatfrei).
    pub fn step_milliturns(&self, k: u32) -> u32 {
        match self {
            // Dyadisch: halbierende Fenster 500, 250, 125, ...
            DispersionProfile::Dyadic => 500u32 >> (k % 9).min(9),
            // Golden: 1000 * (2 - φ) ≈ 381.966 → 382 Milliturns (rationalisiert).
            DispersionProfile::Golden => 382,
            DispersionProfile::Rational { p, q } => {
                if *q == 0 {
                    0
                } else {
                    (1000 * p / q) % 1000
                }
            }
            DispersionProfile::Adaptive => 1 + (k * 37) % 999,
        }
    }
}

/// Die RD-Deklaration eines Laufs: welches Profil gilt. Fehlt sie, darf
/// KEIN Profil wirken (fail-closed; DispersionProfileGate).
#[derive(Debug, Clone, Default)]
pub struct DispersionDeclaration {
    pub declared: Option<DispersionProfile>,
}
