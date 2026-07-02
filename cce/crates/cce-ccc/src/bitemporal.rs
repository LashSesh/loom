//! Bi-Temporalitaet (CCC, Teil 6.1): T2 (intrinsische Phase, frei rotierend),
//! T1 (Commit-Index, monoton append-only), n0 (Null-Anker, nie traversiert).

use cce_core::objects::NullAnchor;

/// Bi-temporale Uhr. T1 waechst NUR ueber `commit` (kein anderer Pfad).
#[derive(Debug, Clone)]
pub struct BiTemporal {
    t2_phase: u64,
    t1_commit: u64,
    pub anchor: NullAnchor,
}

impl BiTemporal {
    pub fn new() -> Self {
        Self {
            t2_phase: 0,
            t1_commit: 0,
            anchor: NullAnchor::new("n0"),
        }
    }

    /// Intrinsische Phase rotiert frei (Drift innen erlaubt, F6).
    pub fn rotate(&mut self, delta: u64) {
        self.t2_phase = self.t2_phase.wrapping_add(delta);
    }

    /// Commit-Index steigt monoton; kein Ruecklauf moeglich.
    pub fn commit(&mut self) -> u64 {
        self.t1_commit += 1;
        self.t1_commit
    }

    pub fn t1(&self) -> u64 {
        self.t1_commit
    }

    pub fn t2(&self) -> u64 {
        self.t2_phase
    }
}

impl Default for BiTemporal {
    fn default() -> Self {
        Self::new()
    }
}
