//! Crystal (Teil 3.4): closure-zertifizierte KLASSE `[Can(c)]_σ`.
//! Alle Felder Pflicht. Ein Crystal entsteht NUR ueber das Kristall-Protokoll
//! (cce-ccc::crystal_protocol) — kein unbelegter Konstruktor.

use cce_ccc::crystal_protocol::{is_crystal, CrystalCandidate};
use cce_core::canonical::{CanonicalClass, Canonicalize};
use cce_core::closure::ClosureCertificate;
use cce_core::signature::{ContentAddress, Digest};
use cce_core::value::CanonValue;

#[derive(Debug, Clone)]
pub struct Crystal {
    /// content-addressed `crystal:<hash>` (P10).
    pub id: ContentAddress,
    pub signature: Digest,
    pub type_status: String,
    pub provenance: String,
    pub boundary_state: String,
    /// Residuenzusammenfassung — STETS ausgewiesen, auch „geschlossen (∅)".
    pub residue_state: String,
    pub replay_contract: String,
    pub closure_certificate: ClosureCertificate,
    /// Der kanonische Kern (traegt die Klasse).
    pub canon: CanonValue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CrystalError {
    ProtocolViolated(Vec<String>),
}

impl Crystal {
    /// Einziger Weg zum Crystal: durch das Zentralgesetz.
    pub fn certify(
        canon_content: CanonValue,
        candidate: &CrystalCandidate,
        provenance: &str,
        cert: ClosureCertificate,
    ) -> Result<Crystal, CrystalError> {
        is_crystal(candidate).map_err(CrystalError::ProtocolViolated)?;
        let canon = canon_content.normalize();
        let class = canon.canonical_class();
        Ok(Crystal {
            id: ContentAddress::new("crystal", class.0),
            signature: class.0,
            type_status: "H".to_string(),
            provenance: provenance.to_string(),
            boundary_state: "nonempty".to_string(),
            residue_state: candidate
                .residue_field
                .as_ref()
                .map(|f| f.render_summary())
                .unwrap_or_default(),
            replay_contract: "rd-1".to_string(),
            closure_certificate: cert,
            canon,
        })
    }

    /// Die Klasse `[Can(c)]_σ` — der Inhaltsklassen-Digest (S7.2).
    pub fn class(&self) -> CanonicalClass {
        self.canon.canonical_class()
    }
}

impl Canonicalize for Crystal {
    fn canonical_value(&self) -> CanonValue {
        self.canon.clone()
    }
}
