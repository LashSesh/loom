//! QSNA (I-09, BCIK §15/§42): singularitaetsersetzender Attraktor mit
//! Banach-Fixpunkt und a-priori-Abbruchschranke
//! `d(x_n, x*) ≤ λⁿ/(1−λ) · d(x₀, Φx₀)`.
//! Nicht-Abschluss wird NIE still absorbiert: er wird als Counter-Horizon +
//! Schranke ausgewiesen (P12, §8.6). Rationale Arithmetik — kein Float.

use crate::objects::CounterHorizon;
use crate::residue::{Residue, ResidueKind, Severity};

/// Rationale Zahl (i128) — exakt, deterministisch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rat {
    pub num: i128,
    pub den: i128,
}

impl Rat {
    pub fn new(num: i128, den: i128) -> Self {
        assert!(den != 0);
        let g = gcd(num.abs().max(1), den.abs());
        let sign = if den < 0 { -1 } else { 1 };
        Self {
            num: sign * num / g,
            den: (den / g).abs(),
        }
    }

    pub fn abs(self) -> Rat {
        Rat {
            num: self.num.abs(),
            den: self.den,
        }
    }

    pub fn le(self, other: Rat) -> bool {
        self.num * other.den <= other.num * self.den
    }

    pub fn one() -> Rat {
        Rat { num: 1, den: 1 }
    }
}

impl std::ops::Mul for Rat {
    type Output = Rat;
    fn mul(self, other: Rat) -> Rat {
        Rat::new(self.num * other.num, self.den * other.den)
    }
}

impl std::ops::Sub for Rat {
    type Output = Rat;
    fn sub(self, other: Rat) -> Rat {
        Rat::new(
            self.num * other.den - other.num * self.den,
            self.den * other.den,
        )
    }
}

impl std::ops::Div for Rat {
    type Output = Rat;
    fn div(self, other: Rat) -> Rat {
        Rat::new(self.num * other.den, self.den * other.num)
    }
}

fn gcd(a: i128, b: i128) -> i128 {
    if b == 0 {
        a.max(1)
    } else {
        gcd(b, a % b)
    }
}

/// Ergebnis einer QSNA-Iteration.
#[derive(Debug, Clone)]
pub enum QsnaOutcome {
    /// Fixpunkt innerhalb Toleranz erreicht (zertifizierbar).
    Converged { iterations: u32, value: Rat },
    /// Endliche Sweeps erreichen Fix(R) nicht: ehrlicher Nicht-Abschluss
    /// mit a-priori-Schranke + Counter-Horizon (CCC §6.5, BCIK §42).
    NotClosed {
        iterations: u32,
        apriori_bound: Rat,
        counter_horizon: CounterHorizon,
        residue: Residue,
    },
}

/// Iteriert eine λ-Kontraktion Φ; bricht zertifizierbar ab.
pub fn iterate_contraction(
    x0: Rat,
    phi: impl Fn(Rat) -> Rat,
    lambda: Rat,
    tolerance: Rat,
    max_iter: u32,
) -> QsnaOutcome {
    assert!(
        lambda.le(Rat::one()) && lambda != Rat::one(),
        "λ < 1 noetig"
    );
    let d0 = (phi(x0) - x0).abs();
    let mut x = x0;
    let mut lambda_pow = Rat::one();
    for n in 0..max_iter {
        // a-priori: d(x_n, x*) ≤ λⁿ/(1−λ) · d(x0, Φx0)
        let bound = (lambda_pow / (Rat::one() - lambda)) * d0;
        if bound.le(tolerance) {
            return QsnaOutcome::Converged {
                iterations: n,
                value: x,
            };
        }
        x = phi(x);
        lambda_pow = lambda_pow * lambda;
    }
    let bound = (lambda_pow / (Rat::one() - lambda)) * d0;
    QsnaOutcome::NotClosed {
        iterations: max_iter,
        apriori_bound: bound,
        counter_horizon: CounterHorizon {
            null_models: vec!["Fix(R) durch endliche Sweeps unerreicht (CCC §6.5)".into()],
            pathologies: vec![format!(
                "Restabstand ≤ {}/{} nach {} Iterationen",
                bound.num, bound.den, max_iter
            )],
        },
        residue: Residue::new(
            "qsna-not-closed",
            "qsna",
            ResidueKind::OpenQuestion,
            Severity::Blocking,
            "Nicht-Abschluss mit a-priori-Schranke ausgewiesen (P12) — ClosedCCE ehrlich 0",
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Konvergenz: Kontraktion Φ(x) = x/2 (λ = 1/2) erreicht Toleranz.
    #[test]
    fn contraction_converges_with_certificate() {
        let out = iterate_contraction(
            Rat::new(1, 1),
            |x| x * Rat::new(1, 2),
            Rat::new(1, 2),
            Rat::new(1, 1000),
            64,
        );
        assert!(matches!(out, QsnaOutcome::Converged { .. }));
    }

    /// P12: bei zu wenig Iterationen wird der Nicht-Abschluss NIE still
    /// absorbiert — Schranke + Counter-Horizon + blocking Residuum.
    #[test]
    fn non_closure_is_visible_with_bound() {
        let out = iterate_contraction(
            Rat::new(1, 1),
            |x| x * Rat::new(1, 2),
            Rat::new(1, 2),
            Rat::new(1, 1_000_000_000),
            3,
        );
        match out {
            QsnaOutcome::NotClosed {
                counter_horizon,
                residue,
                ..
            } => {
                assert!(!counter_horizon.null_models.is_empty());
                assert_eq!(residue.severity, Severity::Blocking);
            }
            QsnaOutcome::Converged { .. } => panic!("unerwartete Konvergenz"),
        }
    }
}
