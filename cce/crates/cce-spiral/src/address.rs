//! Spiraladresse a = (s, p, k, j, θ, r) (Formel/S15.6): die maschinen-
//! lesbare Ortsangabe jeder Zelle. θ als Milliturns (Integer — kein Float).

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SpiralAddress {
    /// Skala SCALE-s.
    pub s: u8,
    /// Phase p innerhalb der Skala.
    pub p: u32,
    /// Expansionsschritt k.
    pub k: u32,
    /// Zellindex j.
    pub j: u32,
    /// Winkel θ in Milliturns (0..1000 = eine Umdrehung).
    pub theta_milliturns: u32,
    /// Radius r (diskret).
    pub r: u32,
}

impl SpiralAddress {
    pub fn origin(s: u8, p: u32) -> Self {
        Self {
            s,
            p,
            k: 0,
            j: 0,
            theta_milliturns: 0,
            r: 0,
        }
    }

    /// Intrinsische Rotation: θ_{k+1} = θ_k + δ mod 2π — frei (F6),
    /// beruehrt NIE den Commitzaehler.
    pub fn rotate(mut self, delta_milliturns: u32) -> Self {
        self.theta_milliturns = (self.theta_milliturns + delta_milliturns) % 1000;
        self
    }
}
