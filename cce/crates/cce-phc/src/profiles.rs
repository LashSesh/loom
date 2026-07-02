//! Konformitaetsprofile (G-20, PHC §26–28): PHC-CORE-0.1 / PHC-LOOM-0.1 /
//! PHC-MERKABA-0.1 — Etablierungsstufen inkl. `PHC-Portable`.

use crate::package::PhcPackage;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Profile {
    Core,
    Loom,
    Merkaba,
}

/// Prueft, welche Profile ein Paket erfuellt (kumulativ).
pub fn conformance(p: &PhcPackage) -> Vec<Profile> {
    let mut out = Vec::new();
    // CORE: Manifest + Zellen + Gates + Residue-Policy + Ledger-Modus.
    let core = !p.cells.is_empty()
        && p.gates.len() >= 7
        && p.residue_policy == "visible"
        && p.ledger_mode == "append_only";
    if core {
        out.push(Profile::Core);
        // LOOM: zusaetzlich Workcells + Projektionen (webbar).
        if !p.workcells.is_empty() && !p.projections.is_empty() {
            out.push(Profile::Loom);
            // MERKABA: zusaetzlich Seams + Exporte (zirkulierbar).
            if !p.exports.is_empty() {
                out.push(Profile::Merkaba);
            }
        }
    }
    out
}
