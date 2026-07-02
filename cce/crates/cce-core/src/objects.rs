//! Kanonisches Objektmodell (Bauverfassung Teil 3): NullAnchor, Seam,
//! BoundaryContract, Marker/Response/Horizon/CounterHorizon, Evidence,
//! Certificate, Artifact und die Wunsch-Normalform W (inkl. S4-A1-Felder
//! BoundarySpec, CompletionSpace, ScaleTarget).

use crate::canonical::Canonicalize;
use crate::gate::GateReport;
use crate::residue::ResidueField;
use crate::signature::{ContentAddress, Digest};
use crate::value::CanonValue;

/// Praetemporale Referenz Z0/NC/Π0 (Teil 3.1): NICHT traversierbar (P7/V3).
/// Es existiert bewusst keine Methode, die einen Pfad DURCH den Anker legt —
/// nur Markierung und Naeherungsmeldung.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NullAnchor {
    pub id: String,
    pub space_tag: String,
}

/// Ergebnis jeder Nullnaehe: BoundaryTrace + Residuum (LOOM §5, BCIK §15).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundaryTrace {
    pub anchor_id: String,
    pub note: String,
}

impl NullAnchor {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            space_tag: "A".to_string(),
        }
    }

    /// Jede Naeherung erzeugt einen Trace — nie einen Durchgang.
    pub fn approach(&self, note: &str) -> BoundaryTrace {
        BoundaryTrace {
            anchor_id: self.id.clone(),
            note: note.to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeamDirection {
    Inbound,
    Outbound,
    Bidirectional,
}

/// Separator/Seam (Teil 3.2): `S_ij = C_i ∩ C_j`, Mandorla-Naht.
/// `residue_policy` ist konstant `visible` (kein anderer Wert konstruierbar).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Seam {
    pub id: String,
    pub kind: String,
    pub cells: Vec<String>,
    pub direction: SeamDirection,
    pub rule: String,
    pub gate: String,
    pub replay_anchor: Option<String>,
}

impl Seam {
    pub const RESIDUE_POLICY: &'static str = "visible";

    pub fn new(id: &str, cells: &[&str], rule: &str, gate: &str) -> Self {
        Self {
            id: id.to_string(),
            kind: "boundary_equalizer".to_string(),
            cells: cells.iter().map(|c| c.to_string()).collect(),
            direction: SeamDirection::Bidirectional,
            rule: rule.to_string(),
            gate: gate.to_string(),
            replay_anchor: None,
        }
    }
}

/// BoundaryContract (Teil 3.2): kontrollierte offene Abhaengigkeit mit Expiry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundaryContract {
    pub interface: String,
    pub assumptions: Vec<String>,
    pub obligations: Vec<String>,
    pub verification: String,
    pub expiry: String,
}

/// Marker (Teil 3.3): typisierter partieller Beobachtungsoperator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Marker {
    pub id: String,
    pub marker_type: String,
    pub query: String,
    pub scope: Option<String>,
}

/// Response/ResponseCut (Teil 3.3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Response {
    pub marker: String,
    pub cut: CanonValue,
    pub evidence: Option<String>,
}

/// Horizont: sichtbar/latent/blockiert.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Horizon {
    pub visible: Vec<String>,
    pub latent: Vec<String>,
    pub blocked: Vec<String>,
}

/// Gegenhorizont (Teil 3.3/7.6): Nullmodelle/Gegendeutungen — verhindert
/// Schliessung durch reine Selbstbestaetigung.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CounterHorizon {
    pub null_models: Vec<String>,
    pub pathologies: Vec<String>,
}

/// Evidence: content-adressiertes Nachweisobjekt (Gate-Eingang).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Evidence {
    pub address: ContentAddress,
    pub description: String,
}

impl Evidence {
    pub fn new(digest: Digest, description: &str) -> Self {
        Self {
            address: ContentAddress::new("evidence", digest),
            description: description.to_string(),
        }
    }
}

/// Certificate (Teil 3.6): cert(K) = (idCan, σ, RD, GateReport, BoundaryReport,
/// ResidualReport, τ_head, Replaymanifest, PathReport).
#[derive(Debug, Clone)]
pub struct Certificate {
    pub id_can: String,
    pub signature: Digest,
    pub run_descriptor_class: Digest,
    pub gate_reports: Vec<GateReport>,
    pub boundary_report: String,
    pub residual_report: String,
    pub trace_head: Digest,
    pub replay_manifest: Option<String>,
    pub path_report: String,
}

/// Artifact (Teil 3.6): das materialisierte, closure-zertifizierte Ergebnis.
/// Zwei-Digest-Modell (S7.2): byte_digest identifiziert die Datei,
/// class_digest die Bedeutung.
#[derive(Debug, Clone)]
pub struct Artifact {
    pub bytes: Vec<u8>,
    pub byte_digest: Digest,
    pub class_digest: Digest,
    pub certificate: Option<Certificate>,
    pub ledger_index: u64,
    pub provenance: Vec<ContentAddress>,
}

/// Skalenziel (S4-A1): default SCALE-1; hoehere Ziele formulierbar,
/// tragen sichtbaren PL-Hinweis (R-10).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScaleTarget(pub u8);

impl Default for ScaleTarget {
    fn default() -> Self {
        ScaleTarget(1)
    }
}

impl ScaleTarget {
    /// PL-Hinweis fuer SCALE≥2 (keine stille Hoeherskalierung, S2-A1/R-10).
    pub fn pl_notice(self) -> Option<String> {
        (self.0 >= 2).then(|| {
            format!(
                "SCALE-{}: strukturell formulierbar, Produktreife ist PL-gefuehrtes Residuum (R-10)",
                self.0
            )
        })
    }
}

/// Wunsch-Normalform W (Teil 3.7, erweitert um S4-A1):
/// W = (X, H, K, G, Res, Π, τ, Replay, Goal, Materialize,
///      BoundarySpec, CompletionSpace, ScaleTarget).
#[derive(Debug, Clone)]
pub struct Wish {
    pub carrier: Vec<CanonValue>,
    pub horizon: Horizon,
    pub constraints: Vec<CanonValue>,
    pub gates: Vec<String>,
    pub residue_expectation: ResidueField,
    pub projection: Option<String>,
    pub trace_required: bool,
    pub replay_required: bool,
    pub goal: String,
    pub materialize: String,
    pub boundary_spec: Option<CanonValue>,
    pub completion_space: Vec<String>,
    pub scale_target: ScaleTarget,
    pub domain_mode: String,
}

impl Wish {
    pub fn new(goal: &str, materialize: &str, domain_mode: &str) -> Self {
        Self {
            carrier: Vec::new(),
            horizon: Horizon::default(),
            constraints: Vec::new(),
            gates: Vec::new(),
            residue_expectation: ResidueField::new(),
            projection: None,
            trace_required: true,
            replay_required: true,
            goal: goal.to_string(),
            materialize: materialize.to_string(),
            boundary_spec: None,
            completion_space: Vec::new(),
            scale_target: ScaleTarget::default(),
            domain_mode: domain_mode.to_string(),
        }
    }

    /// Residuen der Wunsch-Pruefung (S4-A3): `boundary_unspecified` (Hold bei
    /// Nicht-Default-Faellen), `completion_space_open` (info).
    pub fn boundary_unspecified(&self) -> bool {
        self.boundary_spec.is_none() && self.scale_target.0 >= 2
    }

    pub fn completion_space_open(&self) -> bool {
        !self.completion_space.is_empty()
    }
}

impl Canonicalize for Wish {
    fn canonical_value(&self) -> CanonValue {
        CanonValue::map([
            ("carrier", CanonValue::List(self.carrier.clone())),
            ("goal", CanonValue::text(&self.goal)),
            ("materialize", CanonValue::text(&self.materialize)),
            ("constraints", CanonValue::List(self.constraints.clone())),
            (
                "gates",
                CanonValue::List(self.gates.iter().map(CanonValue::text).collect()),
            ),
            (
                "boundary_spec",
                self.boundary_spec.clone().unwrap_or(CanonValue::Null),
            ),
            (
                "completion_space",
                CanonValue::List(self.completion_space.iter().map(CanonValue::text).collect()),
            ),
            (
                "scale_target",
                CanonValue::Int(i64::from(self.scale_target.0)),
            ),
            ("domain_mode", CanonValue::text(&self.domain_mode)),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// P7/V3: der Nullanker kennt keinen Durchgangs-Pfad, nur Trace.
    #[test]
    fn null_anchor_is_not_traversable() {
        let z0 = NullAnchor::new("Z0");
        let t = z0.approach("Nullnaehe im Weave");
        assert_eq!(t.anchor_id, "Z0");
        // Kein API-Pfad "traverse" existiert — dieser Test dokumentiert das
        // Fehlen als bauliche Garantie (Abwesenheits-Nachweis).
    }

    /// V4: Seam-Residue-Policy ist konstant `visible`.
    #[test]
    fn seam_residue_policy_is_visible() {
        assert_eq!(Seam::RESIDUE_POLICY, "visible");
    }

    /// S4-A1: ScaleTarget default SCALE-1; SCALE≥2 traegt PL-Hinweis.
    #[test]
    fn scale_target_defaults_and_notices() {
        assert_eq!(ScaleTarget::default(), ScaleTarget(1));
        assert!(ScaleTarget(1).pl_notice().is_none());
        assert!(ScaleTarget(3).pl_notice().unwrap().contains("R-10"));
    }

    /// S4-A3: boundary_unspecified nur bei Nicht-Default-Skala ohne Spec.
    #[test]
    fn wish_boundary_residues() {
        let mut w = Wish::new("Drei-Risiken-Memo", ".md", "document");
        assert!(!w.boundary_unspecified());
        w.scale_target = ScaleTarget(2);
        assert!(w.boundary_unspecified());
        w.boundary_spec = Some(CanonValue::text("scope: workspace"));
        assert!(!w.boundary_unspecified());
    }
}
