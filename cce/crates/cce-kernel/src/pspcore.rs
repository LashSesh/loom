//! PSPcore = (Σ, Ω, RD, Trace, Evidence, Manifest) — Rebase §1.1.

use crate::mef::MefBlock;
use cce_core::ledger::{Ledger, LedgerEventKind};
use cce_core::replay::RunDescriptor;
use cce_core::signature::Digest;
use cce_core::value::CanonValue;
use std::collections::BTreeMap;

/// Der Execution-Kern eines Laufs.
#[derive(Debug)]
pub struct PspCore {
    /// Σ — kanonischer Zustandsraum.
    pub sigma: CanonValue,
    /// Ω — registrierte Operator-Kennungen (deterministisch geordnet).
    pub omega: Vec<String>,
    /// RD — der Replay-Vertrag des Laufs.
    pub rd: RunDescriptor,
    /// Trace — append-only (Ledger-gebunden).
    pub trace: Ledger,
    /// Evidence — MEF-Bloecke, content-adressiert.
    pub evidence: BTreeMap<String, MefBlock>,
    /// Manifest — deklarative Selbstbeschreibung.
    pub manifest: CanonValue,
}

impl PspCore {
    pub fn new(sigma: CanonValue, rd: RunDescriptor) -> Self {
        Self {
            sigma: sigma.normalize(),
            omega: Vec::new(),
            rd,
            trace: Ledger::new(),
            evidence: BTreeMap::new(),
            manifest: CanonValue::map([("kernel", CanonValue::text("pspcore-1"))]),
        }
    }

    pub fn register_operator(&mut self, id: &str) {
        if !self.omega.iter().any(|o| o == id) {
            self.omega.push(id.to_string());
            self.omega.sort();
        }
    }

    /// Evidence anbinden: MEF-Block wird content-adressiert abgelegt und
    /// im Trace verzeichnet.
    pub fn attach_evidence(&mut self, block: MefBlock) -> Digest {
        let d = block.digest();
        self.trace.append(LedgerEventKind::Execute, d);
        self.evidence.insert(format!("mef:{d}"), block);
        d
    }

    pub fn has_evidence(&self, digest: Digest) -> bool {
        self.evidence.contains_key(&format!("mef:{digest}"))
    }
}
