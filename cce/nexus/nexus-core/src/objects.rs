//! CSA-Objektmodell (CSA.2/CSA.3).

use cce_core::canonical::Canonicalize;
use cce_core::signature::Digest;
use cce_core::value::CanonValue;

/// Die harten `disallowed_actions` (CSA.3/PROD-INV-14): RD-verbindlich,
/// durch KEINEN Operator-, Kanzel- oder Zellpfad uebersteuerbar.
pub const DISALLOWED_ACTIONS: [&str; 6] = [
    "captcha_bypass",
    "paywall_bypass",
    "auth_circumvention",
    "bot_protection_evasion",
    "rate_limit_evasion",
    "terms_violation",
];

/// SourceHorizon HS = (goal, domain_scope, source_classes, source_policy,
/// data_policy, budget, capability, replay, stop) — der aktive Quellenraum
/// EINES Laufs (S13-A4-Vollform).
#[derive(Debug, Clone)]
pub struct SourceHorizon {
    pub goal: String,
    pub domain_scope: Vec<String>,
    pub source_classes: Vec<String>,
    pub source_policy: String,
    pub data_policy: String,
    pub budget_requests: u32,
    pub capability: String,
    pub replay: String,
    pub stop: String,
}

impl SourceHorizon {
    pub fn contains_class(&self, class: &str) -> bool {
        self.source_classes.iter().any(|c| c == class)
    }

    /// Deterministischer Beispiel-Horizont fuer Zeugen: lokaler Korpus,
    /// offizielle APIs, Git-Repositories — die drei G8-Referenzklassen.
    pub fn example_local() -> Self {
        Self {
            goal: "referenz-akquisition".to_string(),
            domain_scope: vec!["docs".to_string()],
            source_classes: vec![
                "local_corpus".to_string(),
                "official_api".to_string(),
                "git_repository".to_string(),
                "feed".to_string(),
            ],
            source_policy: "declared_only".to_string(),
            data_policy: "no_pii".to_string(),
            budget_requests: 16,
            capability: "read_only".to_string(),
            replay: "fixed_snapshot".to_string(),
            stop: "budget_exhausted".to_string(),
        }
    }
}

/// TaskSpec (CSA.3): Default scale_target = SCALE-1.
#[derive(Debug, Clone)]
pub struct TaskSpec {
    pub intent: String,
    pub scope: String,
    pub source_policy: String,
    pub data_policy: String,
    pub budget_requests: u32,
    pub output_contract: String,
    pub stop_rule: String,
    pub scale_target: u8,
}

impl TaskSpec {
    pub fn new(intent: &str, scope: &str) -> Self {
        Self {
            intent: intent.to_string(),
            scope: scope.to_string(),
            source_policy: "declared".to_string(),
            data_policy: "no_pii".to_string(),
            budget_requests: 8,
            output_contract: "nsb".to_string(),
            stop_rule: "budget_exhausted".to_string(),
            scale_target: 1,
        }
    }
}

/// CandidateSource (CSA.2): entdeckt, NOCH NICHT AKTIV — darf nicht
/// abgerufen werden, bevor ein Adapter das SourcePolicyGate bestanden hat.
#[derive(Debug, Clone)]
pub struct CandidateSource {
    pub locator: String,
    pub source_type: String,
    pub discovered_by: String,
    pub expected_schema: String,
    pub license_hint: Option<String>,
    pub risk: String,
    pub confidence_permille: u16,
}

/// RawObservation: der rohe Abrufkoerper (content-adressiert).
#[derive(Debug, Clone)]
pub struct RawObservation {
    pub locator: String,
    pub bytes: Vec<u8>,
    pub fetched_via: String,
    pub snapshot_id: String,
}

impl RawObservation {
    pub fn digest(&self) -> Digest {
        cce_core::signature::sha256(&self.bytes)
    }
}

/// Canonical Source Unit (CSA.3): die KLEINSTE Einheit, die an HBM,
/// DomainAdapter, Nexus oder PHC uebergeben werden darf.
#[derive(Debug, Clone)]
pub struct Csu {
    pub uid: String,
    pub kind: String,
    pub payload: CanonValue,
    pub schema: String,
    /// Qualitaetssignatur (ψ, ρ, ω) in Promille.
    pub quality: (u16, u16, u16),
    pub provenance: String,
    pub license: String,
    pub source_hash: Digest,
    pub residues: Vec<String>,
    pub domain_facet: String,
}

impl Canonicalize for Csu {
    fn canonical_value(&self) -> CanonValue {
        CanonValue::map([
            ("uid", CanonValue::text(&self.uid)),
            ("kind", CanonValue::text(&self.kind)),
            ("payload", self.payload.clone()),
            ("schema", CanonValue::text(&self.schema)),
            ("license", CanonValue::text(&self.license)),
            ("domain_facet", CanonValue::text(&self.domain_facet)),
        ])
    }
}

/// EvidencePack (CSA.3): bindet eine CSU an Abruf, Transformationspfad und
/// Policy-Snapshot. KEIN Export/Candidate/Crystal ohne EP (PROD-INV-15).
#[derive(Debug, Clone)]
pub struct EvidencePack {
    pub evidence_id: String,
    pub record_id: String,
    pub source: String,
    pub locator: String,
    pub transform_path: Vec<String>,
    pub policy_snapshot: String,
    pub license: String,
    /// Attribution wird MITGEFUEHRT, wo die Lizenz sie fordert (PROD-INV-16).
    pub attribution: Option<String>,
    pub raw_hash: Digest,
    pub csu_class: Digest,
}

/// NexusSourceBundle (CSA.3): der kanonische Uebergabekoerper.
#[derive(Debug, Clone)]
pub struct NexusSourceBundle {
    pub bundle_id: String,
    pub run_id: String,
    pub csu_set: Vec<Csu>,
    pub evidence_packs: Vec<EvidencePack>,
    pub source_graph: Vec<(String, String)>,
    pub quality_summary: String,
    pub residues: Vec<String>,
    pub export_contract: String,
}

impl NexusSourceBundle {
    /// EvidencePack-Pflicht (CSA.4): jede CSU traegt ihr EP.
    pub fn every_csu_has_evidence(&self) -> bool {
        self.csu_set
            .iter()
            .all(|c| self.evidence_packs.iter().any(|ep| ep.record_id == c.uid))
    }
}

/// Source-RunDescriptor (CSA.3): replaykritisch; boundary enthaelt
/// verbindlich die disallowed_actions.
#[derive(Debug, Clone)]
pub struct SourceRunDescriptor {
    pub run_id: String,
    pub intent: String,
    pub boundary_disallowed: Vec<String>,
    pub pii_mode: String,
    pub license_mode: String,
    pub adapters: Vec<String>,
    pub policy_snapshot: String,
    pub seed: u64,
}

impl SourceRunDescriptor {
    pub fn new(run_id: &str, intent: &str, seed: u64) -> Self {
        Self {
            run_id: run_id.to_string(),
            intent: intent.to_string(),
            // hart verdrahtet — kein Konstruktor ohne die volle Liste.
            boundary_disallowed: DISALLOWED_ACTIONS.iter().map(|s| s.to_string()).collect(),
            pii_mode: "exclude".to_string(),
            license_mode: "attribution_transported".to_string(),
            adapters: Vec::new(),
            policy_snapshot: "policy-1".to_string(),
            seed,
        }
    }

    /// PROD-INV-14: die Liste ist unverkuerzbar — jede Pruefung verlangt
    /// alle sechs Eintraege.
    pub fn disallowed_complete(&self) -> bool {
        DISALLOWED_ACTIONS
            .iter()
            .all(|a| self.boundary_disallowed.iter().any(|b| b == a))
    }
}
