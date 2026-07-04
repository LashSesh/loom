//! S-E5 §2: Objektmodell des L9b Normic Memory — NexusClass, Scope,
//! ProvenanceSet, NormCandidate, BridgeNorm, NormStatus, sowie das
//! vollstaendige Residuen-Vokabular (§8, jeweils ein Zeuge zugeordnet).

use crate::pattern::Pattern;
use cce_core::canonical::Canonicalize;
use cce_core::value::CanonValue;
use loom_cites::CiteEntry;

/// §2: abschliessende Aufzaehlung v1. Erweiterung nur ueber den S14-Pfad.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum NexusClass {
    StructuralRule,
    SeamPattern,
    ClosureProfile,
    VocabularyNorm,
    ProcessNorm,
}

impl NexusClass {
    pub fn as_str(self) -> &'static str {
        match self {
            NexusClass::StructuralRule => "structural_rule",
            NexusClass::SeamPattern => "seam_pattern",
            NexusClass::ClosureProfile => "closure_profile",
            NexusClass::VocabularyNorm => "vocabulary_norm",
            NexusClass::ProcessNorm => "process_norm",
        }
    }

    pub fn parse(s: &str) -> Option<NexusClass> {
        match s {
            "structural_rule" => Some(NexusClass::StructuralRule),
            "seam_pattern" => Some(NexusClass::SeamPattern),
            "closure_profile" => Some(NexusClass::ClosureProfile),
            "vocabulary_norm" => Some(NexusClass::VocabularyNorm),
            "process_norm" => Some(NexusClass::ProcessNorm),
            _ => None,
        }
    }
}

/// §2: Geltungsbereich einer Norm.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Scope {
    Domain(String),
    Family(String),
    Global,
}

impl Scope {
    pub fn as_string(&self) -> String {
        match self {
            Scope::Domain(id) => format!("domain:{id}"),
            Scope::Family(id) => format!("family:{id}"),
            Scope::Global => "global".to_string(),
        }
    }

    pub fn parse(s: &str) -> Option<Scope> {
        if s == "global" {
            return Some(Scope::Global);
        }
        if let Some(id) = s.strip_prefix("domain:") {
            return Some(Scope::Domain(id.to_string()));
        }
        if let Some(id) = s.strip_prefix("family:") {
            return Some(Scope::Family(id.to_string()));
        }
        None
    }
}

/// §6: Lebenszyklus-Status einer promovierten Norm.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NormStatus {
    Active,
    Deprecated,
    Revoked,
}

impl NormStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            NormStatus::Active => "active",
            NormStatus::Deprecated => "deprecated",
            NormStatus::Revoked => "revoked",
        }
    }

    pub fn parse(s: &str) -> Option<NormStatus> {
        match s {
            "active" => Some(NormStatus::Active),
            "deprecated" => Some(NormStatus::Deprecated),
            "revoked" => Some(NormStatus::Revoked),
            _ => None,
        }
    }
}

/// §2: deduplizierte, sortierte Menge von `core_root`s — die Herkunft
/// einer Norm. Kardinalitaet/Diversitaet werden vom BridgeGate geprueft
/// (das ProvenanceSet selbst traegt nur die Adressierung, nie Pfade).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ProvenanceSet {
    pub members: Vec<String>,
}

impl ProvenanceSet {
    pub fn new(mut members: Vec<String>) -> Self {
        members.sort();
        members.dedup();
        Self { members }
    }

    pub fn len(&self) -> usize {
        self.members.len()
    }

    pub fn is_empty(&self) -> bool {
        self.members.is_empty()
    }
}

/// §2: ein bekanntes Gegenbeispiel — Pflichtfeld, niemals verschwiegen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CounterExample {
    pub core_root_hex: String,
    pub reason: String,
}

/// §2: NormCandidate — Kandidat ≠ Norm (das Kandidaten-Commit-Verbot
/// gilt unveraendert). Quelle des `pattern` ist der Aufrufer (HBM-
/// Blueprints via `extract::blueprint_to_pattern` und/oder
/// Registry-Bestand als Pattern-Lieferant, §7) — L9b selbst destilliert
/// nicht algorithmisch, sondern gatet/promoviert. Dokument 16 §2b: die
/// `NexusClass` ist KEIN eigenes Feld mehr, sondern
/// `pattern.class()` — zwei Felder, die auseinanderlaufen koennten,
/// sind strukturell ausgeschlossen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormCandidate {
    pub pattern: Pattern,
    pub provenance_set: ProvenanceSet,
    pub n_support: u64,
    pub n_counter: u64,
    pub known_counterexamples: Vec<CounterExample>,
    pub scope: Scope,
    /// Digest des `RunDescriptor`, unter dem destilliert wurde (RD-Bindung).
    pub distillation_rd_class_hex: String,
}

impl NormCandidate {
    pub fn nexus_class(&self) -> NexusClass {
        self.pattern.class()
    }
}

/// §2: BridgeNorm — die promovierte Norm. `norm_id` = content_class des
/// Norm-Workbody selbst (die Norm IST der Container, nicht ein Verweis
/// darauf).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeNorm {
    pub norm_id: String,
    pub pattern: Pattern,
    pub provenance_set: ProvenanceSet,
    pub known_counterexamples: Vec<CounterExample>,
    pub scope: Scope,
    pub status: NormStatus,
    pub promotion_evidence: String,
    pub supersedes: Option<String>,
}

impl BridgeNorm {
    pub fn nexus_class(&self) -> NexusClass {
        self.pattern.class()
    }

    /// §2: das ProvenanceSet einer Norm IST ihre `external_citations`-
    /// Liste mit `cite_kind=derives` — S-E2a wirkt woertlich weiter.
    pub fn derives_cites(&self) -> Vec<CiteEntry> {
        self.provenance_set
            .members
            .iter()
            .map(|root| CiteEntry {
                unit_id: "norm".to_string(),
                target_core_root_hex: root.clone(),
                target_unit_ref: None,
                cite_kind: loom_cites::CiteKind::Derives,
            })
            .collect()
    }
}

/// Die kanonische Klasse eines Kandidaten IST spaeter der `norm_id`
/// (§2: "norm_id = content_class des Norm-Workbody") — zwei
/// unabhaengige Destillationen ueber denselben Registry-Snapshot +
/// dasselbe RD MUESSEN dieselbe Klasse ergeben (DistillationReplayGate,
/// §3 Stufe 6).
impl Canonicalize for NormCandidate {
    fn canonical_value(&self) -> CanonValue {
        CanonValue::map([
            ("pattern", CanonValue::text(self.pattern.describe())),
            ("nexus_class", CanonValue::text(self.nexus_class().as_str())),
            (
                "provenance_set",
                CanonValue::List(
                    self.provenance_set
                        .members
                        .iter()
                        .map(CanonValue::text)
                        .collect(),
                ),
            ),
            ("n_support", CanonValue::Int(self.n_support as i64)),
            ("n_counter", CanonValue::Int(self.n_counter as i64)),
            ("scope", CanonValue::text(self.scope.as_string())),
            (
                "distillation_rd_class",
                CanonValue::text(&self.distillation_rd_class_hex),
            ),
        ])
    }
}

/// §3: Verdikt der Promotionspruefung.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeVerdict {
    Allow,
    Hold,
    Reject,
}

/// §8: das vollstaendige Residuen-Vokabular, je eines mit zugeordnetem
/// Zeugen (in Klammern).
pub mod residue {
    /// N-NRM-1: κ < κ_min.
    pub const INSUFFICIENT_PROVENANCE: &str = "insufficient_provenance";
    /// DiversityGate nicht erfuellt (nicht separat bezeugt, s. Bericht).
    pub const DIVERSITY_UNMET: &str = "diversity_unmet";
    /// N-NRM-4: verschwiegene Gegenbeispiele.
    pub const COUNTEREXAMPLE_UNRESOLVED: &str = "counterexample_unresolved";
    /// N-NRM-5: Widerspruch zu aktiver Norm gleichen Scopes.
    pub const NORM_CONFLICT: &str = "norm_conflict";
    /// N-NRM-3: Pattern will ein Gate/Verbot/Tor/Invariante lockern.
    pub const NORM_SCOPE_VIOLATION: &str = "norm_scope_violation";
    /// R-NRM-4: ProvenanceSet-Mitglied invalid/widerrufen/quarantaenisiert.
    pub const PROVENANCE_EROSION: &str = "provenance_erosion";
    /// R-NRM-3: Reanalyse eines Laufs mit inzwischen widerrufener Norm.
    pub const NORM_SINCE_REVOKED: &str = "norm_since_revoked";
    /// N-NRM-8: gleicher Snapshot+RD ergibt einen anderen Kandidaten.
    pub const DISTILLATION_REPLAY_MISMATCH: &str = "distillation_replay_mismatch";
    /// Zitat auf eine Norm, die sich nicht aufloesen laesst.
    pub const UNRESOLVED_NORM_CITATION: &str = "unresolved_norm_citation";
    /// N-NRM-6: Anwendung ohne explizite Aktivierung (PROD-INV-22).
    pub const NORM_NOT_ACTIVATED: &str = "norm_not_activated";
    /// Dokument 16 §2a: ein Blueprint traegt keine erkennbare,
    /// whitelist-konforme Regelform — `extract::blueprint_to_pattern`
    /// liefert `None`, nie ein geratenes Pattern.
    pub const PATTERN_EXTRACTION_UNSUPPORTED: &str = "pattern_extraction_unsupported";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nexus_class_roundtrips() {
        for c in [
            NexusClass::StructuralRule,
            NexusClass::SeamPattern,
            NexusClass::ClosureProfile,
            NexusClass::VocabularyNorm,
            NexusClass::ProcessNorm,
        ] {
            assert_eq!(NexusClass::parse(c.as_str()), Some(c));
        }
    }

    #[test]
    fn scope_roundtrips() {
        for s in [
            Scope::Domain("d06".to_string()),
            Scope::Family("family_a".to_string()),
            Scope::Global,
        ] {
            assert_eq!(Scope::parse(&s.as_string()), Some(s));
        }
    }

    #[test]
    fn provenance_set_dedupes_and_sorts() {
        let set = ProvenanceSet::new(vec!["bb".to_string(), "aa".to_string(), "bb".to_string()]);
        assert_eq!(set.members, vec!["aa".to_string(), "bb".to_string()]);
    }

    #[test]
    fn derives_cites_matches_provenance_members() {
        let norm = BridgeNorm {
            norm_id: "norm:1".to_string(),
            pattern: Pattern::StructuralRule(crate::pattern::DomainRuleForm::UniqueSubjects),
            provenance_set: ProvenanceSet::new(vec!["aa".repeat(34), "bb".repeat(34)]),
            known_counterexamples: vec![],
            scope: Scope::Global,
            status: NormStatus::Active,
            promotion_evidence: "e".to_string(),
            supersedes: None,
        };
        assert_eq!(norm.nexus_class(), NexusClass::StructuralRule);
        let cites = norm.derives_cites();
        assert_eq!(cites.len(), 2);
        assert!(cites
            .iter()
            .all(|c| c.cite_kind == loom_cites::CiteKind::Derives));
    }
}
