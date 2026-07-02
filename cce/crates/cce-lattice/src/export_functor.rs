//! Export-Funktor Ex: Commit_M → Sink (CL, closure-erhaltend):
//! exportiert AUSSCHLIESSLICH committete/zertifizierte Zustaende.

use crate::collapse::CollapseCertificate;

#[derive(Debug, Clone)]
pub struct ExportBundle {
    pub class: String,
    pub evidence_ref: String,
    pub mode: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExportError {
    /// Uncommitteter Zustand — Export verweigert (closure-erhaltend).
    NotCommitted,
}

pub fn export(cert: Option<&CollapseCertificate>) -> Result<ExportBundle, ExportError> {
    let c = cert.ok_or(ExportError::NotCommitted)?;
    Ok(ExportBundle {
        class: format!("{}", c.hull_class),
        evidence_ref: c.evidence_ref.clone(),
        mode: c.mode.clone(),
    })
}
