//! Dyadisches Zellsubstrat (O-01/O-02, DZ): adressierbare Zellen W_{n,k}
//! mit Verfeinerung ch_ε(n,k) = (n+1, 2k+ε) und Filtration F_n — der
//! messbare Untergrund aller Beobachtung.

/// Dyadische Zelladresse (n, k).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DyadicCell {
    pub n: u32,
    pub k: u64,
}

impl DyadicCell {
    pub fn root() -> Self {
        Self { n: 0, k: 0 }
    }

    /// Verfeinerung: ch_ε(n, k) = (n+1, 2k+ε).
    pub fn child(self, epsilon: u8) -> DyadicCell {
        DyadicCell {
            n: self.n + 1,
            k: 2 * self.k + u64::from(epsilon & 1),
        }
    }

    pub fn parent(self) -> Option<DyadicCell> {
        (self.n > 0).then(|| DyadicCell {
            n: self.n - 1,
            k: self.k / 2,
        })
    }

    /// Filtrations-Zugehoerigkeit: Zelle liegt in F_n ihres Levels.
    pub fn level(self) -> u32 {
        self.n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// DZ §2: Verfeinerung ist konsistent mit parent (Filtration).
    #[test]
    fn dyadic_refinement_roundtrip() {
        let c = DyadicCell::root().child(1).child(0);
        assert_eq!(c, DyadicCell { n: 2, k: 2 });
        assert_eq!(c.parent().unwrap(), DyadicCell { n: 1, k: 1 });
        assert_eq!(c.parent().unwrap().parent().unwrap(), DyadicCell::root());
    }
}
