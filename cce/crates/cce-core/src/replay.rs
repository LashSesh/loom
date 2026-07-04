//! RunDescriptor & Replay (Teil 3.6/7.7, S5.5): Grundlage von INV-10.
//! Keine Wall-Clock, keine ungeseedete Zufaelligkeit — der RD traegt ALLE
//! replay-relevanten Parameter: crystal_digest, decisions, params, seed.

use crate::canonical::Canonicalize;
use crate::signature::Digest;
use crate::value::CanonValue;

/// Aufgezeichnete HITL-Entscheidung (S5-A2): PhaseBlock-Input
/// `{gate, entscheidung, operator, RD-Ref, Evidence-Ref}` — Replay spielt
/// sie ab, statt den Menschen erneut zu fragen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HitlDecision {
    pub gate: String,
    pub decision: String,
    pub operator: String,
    pub rd_ref: Option<String>,
    pub evidence_ref: Option<String>,
}

impl Canonicalize for HitlDecision {
    fn canonical_value(&self) -> CanonValue {
        CanonValue::map([
            ("gate", CanonValue::text(&self.gate)),
            ("decision", CanonValue::text(&self.decision)),
            ("operator", CanonValue::text(&self.operator)),
            (
                "rd_ref",
                self.rd_ref
                    .as_ref()
                    .map(CanonValue::text)
                    .unwrap_or(CanonValue::Null),
            ),
            (
                "evidence_ref",
                self.evidence_ref
                    .as_ref()
                    .map(CanonValue::text)
                    .unwrap_or(CanonValue::Null),
            ),
        ])
    }
}

/// RunDescriptor (S5.5 erweitert um Bauverfassungs-Felder):
/// gleicher RD + gleicher Input ⇒ gleiche kanonische Klasse (INV-10).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunDescriptor {
    pub version: String,
    pub crystal_digest: Digest,
    pub decisions: Vec<HitlDecision>,
    pub params: CanonValue,
    pub seed: u64,
    pub domain: String,
    pub export_profile: String,
    /// S-E2a I.5: `target_core_root`s zitierter `supports`/`derives`-Nähte
    /// sind Replay-INPUTS — Replay verlangt dieselben Ziele (der Resolver
    /// darf ein anderer sein, die Klassen nicht). Additiv, Default leer.
    pub input_digests: Vec<Digest>,
    /// S-E5 §5: die explizit aktivierten `norm_id`s dieses Laufs
    /// (`norm_profile`) sind RD-Input — Replay verlangt dieselben
    /// Norm-Klassen (ein anderes aktiviertes Set aendert die RD-Klasse).
    /// Additiv, Default leer (kein Lauf aktiviert Normen implizit).
    pub norm_profile: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RdError {
    /// Fehler `ReplayDescriptorIncomplete` (Teil 7.7).
    ReplayDescriptorIncomplete(&'static str),
}

impl RunDescriptor {
    pub fn new(crystal_digest: Digest, domain: &str, seed: u64) -> Self {
        Self {
            version: "rd-1".to_string(),
            crystal_digest,
            decisions: Vec::new(),
            params: CanonValue::map([]),
            seed,
            domain: domain.to_string(),
            export_profile: "default".to_string(),
            input_digests: Vec::new(),
            norm_profile: Vec::new(),
        }
    }

    pub fn with_decision(mut self, d: HitlDecision) -> Self {
        self.decisions.push(d);
        self
    }

    /// S-E2a I.5: einen zitierten Ziel-Digest als Replay-Input anhaengen.
    pub fn with_input(mut self, d: Digest) -> Self {
        self.input_digests.push(d);
        self
    }

    /// S-E5 §5: eine aktivierte `norm_id` anhaengen (`norm_profile`).
    pub fn with_norm(mut self, norm_id: impl Into<String>) -> Self {
        self.norm_profile.push(norm_id.into());
        self
    }

    pub fn with_params(mut self, params: CanonValue) -> Self {
        self.params = params;
        self
    }

    /// Vollstaendigkeitspruefung: ohne vollstaendigen RD kein Lauf (S5.9).
    pub fn validate(&self) -> Result<(), RdError> {
        if self.version.is_empty() {
            return Err(RdError::ReplayDescriptorIncomplete("version"));
        }
        if self.domain.is_empty() {
            return Err(RdError::ReplayDescriptorIncomplete("domain"));
        }
        if self.export_profile.is_empty() {
            return Err(RdError::ReplayDescriptorIncomplete("export_profile"));
        }
        Ok(())
    }
}

impl Canonicalize for RunDescriptor {
    fn canonical_value(&self) -> CanonValue {
        CanonValue::map([
            ("version", CanonValue::text(&self.version)),
            (
                "crystal_digest",
                CanonValue::Bytes(self.crystal_digest.0.to_vec()),
            ),
            (
                "decisions",
                CanonValue::List(self.decisions.iter().map(|d| d.canonical_value()).collect()),
            ),
            ("params", self.params.clone()),
            ("seed", CanonValue::Int(self.seed as i64)),
            ("domain", CanonValue::text(&self.domain)),
            ("export_profile", CanonValue::text(&self.export_profile)),
            (
                "input_digests",
                CanonValue::List(
                    self.input_digests
                        .iter()
                        .map(|d| CanonValue::Bytes(d.0.to_vec()))
                        .collect(),
                ),
            ),
            (
                "norm_profile",
                CanonValue::List(self.norm_profile.iter().map(CanonValue::text).collect()),
            ),
        ])
    }
}

/// Deterministischer, geseedeter Zufallsstrom (P9): ausschliesslich RD-seed-
/// gebunden. Es gibt im gesamten Kern keinen anderen Zufallspfad.
#[derive(Debug, Clone)]
pub struct SeededRng {
    state: u64,
}

impl SeededRng {
    pub fn from_rd(rd: &RunDescriptor) -> Self {
        Self {
            state: rd.seed ^ 0x9e37_79b9_7f4a_7c15,
        }
    }

    /// splitmix64 — deterministisch, plattformstabil.
    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::signature::sha256;

    /// INV-10-Grundlage: gleicher RD ⇒ gleiche RD-Klasse; Entscheidungen
    /// sind Teil des RD (S5.5).
    #[test]
    fn rd_class_covers_decisions() {
        let base = RunDescriptor::new(sha256(b"c"), "document", 42);
        let with_decision = base.clone().with_decision(HitlDecision {
            gate: "coverage-borderline".into(),
            decision: "accept".into(),
            operator: "op-1".into(),
            rd_ref: None,
            evidence_ref: None,
        });
        assert!(base.equivalent(&base.clone()));
        assert!(
            !base.equivalent(&with_decision),
            "HITL-Entscheidung MUSS die RD-Klasse aendern (sonst Replay unterbestimmt)"
        );
    }

    /// P9: geseedeter Zufall ist reproduzierbar.
    #[test]
    fn seeded_rng_is_deterministic() {
        let rd = RunDescriptor::new(sha256(b"c"), "document", 7);
        let a: Vec<u64> = {
            let mut r = SeededRng::from_rd(&rd);
            (0..4).map(|_| r.next_u64()).collect()
        };
        let b: Vec<u64> = {
            let mut r = SeededRng::from_rd(&rd);
            (0..4).map(|_| r.next_u64()).collect()
        };
        assert_eq!(a, b);
    }

    /// S-E2a I.5: ein zitierter Ziel-Digest ist Teil der RD-Klasse — sonst
    /// waere Replay gegen ein ANDERES Ziel unauffaellig gleich klassifiziert.
    #[test]
    fn input_digest_changes_rd_class() {
        let base = RunDescriptor::new(sha256(b"c"), "document", 42);
        let with_input = base.clone().with_input(sha256(b"target-a"));
        assert!(!base.equivalent(&with_input));
        let with_other_input = base.clone().with_input(sha256(b"target-b"));
        assert!(
            !with_input.equivalent(&with_other_input),
            "verschiedene Ziel-Digests muessen verschiedene RD-Klassen ergeben"
        );
    }

    #[test]
    fn incomplete_rd_is_rejected() {
        let mut rd = RunDescriptor::new(sha256(b"c"), "document", 1);
        rd.domain = String::new();
        assert_eq!(
            rd.validate(),
            Err(RdError::ReplayDescriptorIncomplete("domain"))
        );
    }
}
