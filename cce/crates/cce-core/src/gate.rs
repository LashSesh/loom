//! Gate-Familie (Teil 3.6/7.3): fail-closed Praedikate, KEIN Score.
//! Semantik: `Pass` nur, wenn alle Pflichtbedingungen erfuellt; sonst `Hold`
//! mit Begruendung (nie `Fire` bei Unsicherheit). V1 ist hier baulich
//! erzwungen: `GateReport` besitzt kein numerisches Feld, und der
//! Untyped-Eingang weist Score-Felder aktiv ab.

use crate::residue::{Residue, ResidueKind, Severity};
use crate::value::CanonValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GateKind {
    Admission,
    Topological,
    Type,
    Integrity,
    Determinism,
    Export,
    Reanalysis,
}

impl GateKind {
    pub fn as_str(self) -> &'static str {
        match self {
            GateKind::Admission => "admission",
            GateKind::Topological => "topological",
            GateKind::Type => "type",
            GateKind::Integrity => "integrity",
            GateKind::Determinism => "determinism",
            GateKind::Export => "export",
            GateKind::Reanalysis => "reanalysis",
        }
    }
}

/// Boolesches Verdikt — bewusst OHNE numerische Repraesentation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GateVerdict {
    Pass,
    Hold,
}

/// Fail-closed Gate (Teil 3.6). `failure` ist konstant fail_closed —
/// es gibt keinen Konstruktor fuer ein fail-open Gate (V7).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gate {
    pub id: String,
    pub kind: GateKind,
    pub rule: String,
    pub hard: bool,
}

impl Gate {
    pub fn new(id: &str, kind: GateKind, rule: &str) -> Self {
        Self {
            id: id.to_string(),
            kind,
            rule: rule.to_string(),
            hard: true,
        }
    }

    /// Ermessens-Gate (HITL zulaessig, S3.5); harte Gates pausieren nie.
    pub fn discretionary(id: &str, kind: GateKind, rule: &str) -> Self {
        Self {
            hard: false,
            ..Self::new(id, kind, rule)
        }
    }

    pub const FAILURE: &'static str = "fail_closed";
}

/// Ergebnis einer Gate-Pruefung: boolesch + begruendet (04_AGENT_AUFTRAG).
/// Rot ist NIE ohne Begruendung (S6.2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateReport {
    pub gate_id: String,
    pub verdict: GateVerdict,
    pub reason: String,
    pub subject: Option<String>,
    pub evidence_ref: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GateReportError {
    /// V1: eine Kennzahl darf niemals als (Teil eines) Verdikt(s) auftreten.
    ScoreAsGateAttempt(String),
    /// Hold ohne Begruendung ist unzulaessig (Gates sind begruendet).
    MissingReason,
    /// Unbekanntes/untypisiertes Verdikt ⇒ fail-closed Ablehnung (V7).
    UntypedVerdict(String),
}

impl GateReport {
    pub fn pass(gate_id: &str, reason: &str) -> Self {
        Self {
            gate_id: gate_id.to_string(),
            verdict: GateVerdict::Pass,
            reason: reason.to_string(),
            subject: None,
            evidence_ref: None,
        }
    }

    pub fn hold(gate_id: &str, reason: &str) -> Self {
        assert!(
            !reason.trim().is_empty(),
            "Hold ohne Begruendung ist verfassungswidrig (Gates begruendet)"
        );
        Self {
            gate_id: gate_id.to_string(),
            verdict: GateVerdict::Hold,
            reason: reason.to_string(),
            subject: None,
            evidence_ref: None,
        }
    }

    pub fn with_subject(mut self, subject: &str) -> Self {
        self.subject = Some(subject.to_string());
        self
    }

    pub fn with_evidence(mut self, evidence_ref: &str) -> Self {
        self.evidence_ref = Some(evidence_ref.to_string());
        self
    }

    pub fn is_pass(&self) -> bool {
        self.verdict == GateVerdict::Pass
    }

    /// Admission-Pfad fuer untypisierte Reports (z. B. aus Containern):
    /// weist Score-Felder als V1-Verstoss ab und erzwingt boolesches,
    /// begruendetes Verdikt. Fail-closed: alles Unklare ⇒ Fehler.
    pub fn from_untyped(v: &CanonValue) -> Result<GateReport, GateReportError> {
        let m = match v {
            CanonValue::Map(m) => m,
            _ => return Err(GateReportError::UntypedVerdict("kein Map-Report".into())),
        };
        // V1-Schranke: keine numerischen Bewertungsfelder im Verdikt-Traeger.
        for (k, val) in m {
            let numeric = matches!(val, CanonValue::Int(_) | CanonValue::Decimal { .. });
            if numeric
                && (k.contains("score") || k.contains("rating") || k == "verdict" || k == "result")
            {
                return Err(GateReportError::ScoreAsGateAttempt(k.clone()));
            }
        }
        let gate_id = match m.get("gate_id") {
            Some(CanonValue::Text(t)) => t.clone(),
            _ => return Err(GateReportError::UntypedVerdict("gate_id fehlt".into())),
        };
        let verdict = match m.get("verdict") {
            Some(CanonValue::Text(t)) if t == "pass" => GateVerdict::Pass,
            Some(CanonValue::Text(t)) if t == "hold" => GateVerdict::Hold,
            other => {
                return Err(GateReportError::UntypedVerdict(format!(
                    "verdikt nicht boolesch: {other:?}"
                )))
            }
        };
        let reason = match m.get("reason") {
            Some(CanonValue::Text(t)) if !t.trim().is_empty() => t.clone(),
            _ => return Err(GateReportError::MissingReason),
        };
        Ok(GateReport {
            gate_id,
            verdict,
            reason,
            subject: None,
            evidence_ref: None,
        })
    }

    /// Residuum aus einem Hold (jeder Nicht-Pass ist sichtbar, V2).
    pub fn to_residue(&self) -> Option<Residue> {
        match self.verdict {
            GateVerdict::Pass => None,
            GateVerdict::Hold => Some(Residue::new(
                &format!("residue:{}", self.gate_id),
                &format!("gate:{}", self.gate_id),
                ResidueKind::FailedGate,
                Severity::Blocking,
                &self.reason,
            )),
        }
    }
}

/// Gate-Kette: Konjunktion, fail-closed. Ein Hold beendet mit Hold —
/// ein `gate_failed`-Ergebnis darf NIEMALS verschmolzen werden (Teil 7.3).
#[derive(Debug, Clone, Default)]
pub struct GateChain {
    pub reports: Vec<GateReport>,
}

impl GateChain {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, r: GateReport) {
        self.reports.push(r);
    }

    pub fn all_pass(&self) -> bool {
        !self.reports.is_empty() && self.reports.iter().all(GateReport::is_pass)
    }

    pub fn first_hold(&self) -> Option<&GateReport> {
        self.reports.iter().find(|r| !r.is_pass())
    }
}

/// Die sieben Pflichtgates G1–G7 (PHC §12.2 / Teil 7.3) als Typen mit
/// fail-closed Default (01_MASTER_BUILD G1-Gate).
pub fn mandatory_gates() -> Vec<Gate> {
    vec![
        Gate::new(
            "G1-Scope",
            GateKind::Admission,
            "alle Outputs im deklarierten Scope",
        ),
        Gate::new(
            "G2-Boundary",
            GateKind::Topological,
            "kein Boundary-Uebergang ohne Seam; kein stiller Nullpunkt-Durchgang",
        ),
        Gate::new(
            "G3-Type",
            GateKind::Type,
            "alle Objekte typisiert, Rolle+Lebenszyklus",
        ),
        Gate::new(
            "G4-Residue",
            GateKind::Integrity,
            "jeder fallengelassene Inhalt sichtbar als Residuum",
        ),
        Gate::new(
            "G5-Replay",
            GateKind::Determinism,
            "gleicher RunDescriptor ⇒ gleiche Digest-Klasse",
        ),
        Gate::new(
            "G6-Export",
            GateKind::Export,
            "Materialisierung nur nach Gate/Evidence/Trace/Replay",
        ),
        Gate::new("G7-Reanalysis", GateKind::Reanalysis, "q(Obs(A)) = q(C_P)"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// G1-Gate (01_MASTER_BUILD): G1–G7 als Typen, fail-closed Default.
    #[test]
    fn seven_mandatory_gates_fail_closed() {
        let gates = mandatory_gates();
        assert_eq!(gates.len(), 7);
        for g in &gates {
            assert!(g.hard);
            assert_eq!(Gate::FAILURE, "fail_closed");
        }
    }

    /// V1-Negativtest: Score-Feld im GateReport wird abgewiesen.
    #[test]
    fn score_as_gate_is_rejected() {
        let report = CanonValue::map([
            ("gate_id", CanonValue::text("G6-Export")),
            ("score", CanonValue::Int(97)),
            ("verdict", CanonValue::text("pass")),
            ("reason", CanonValue::text("score hoch")),
        ]);
        match GateReport::from_untyped(&report) {
            Err(GateReportError::ScoreAsGateAttempt(f)) => assert_eq!(f, "score"),
            other => panic!("Score-als-Gate wurde NICHT abgewiesen: {other:?}"),
        }
    }

    /// V1-Negativtest (Variante): numerisches Verdikt wird abgewiesen.
    #[test]
    fn numeric_verdict_is_rejected() {
        let report = CanonValue::map([
            ("gate_id", CanonValue::text("G1-Scope")),
            ("verdict", CanonValue::Int(1)),
            ("reason", CanonValue::text("x")),
        ]);
        assert!(matches!(
            GateReport::from_untyped(&report),
            Err(GateReportError::ScoreAsGateAttempt(_)) | Err(GateReportError::UntypedVerdict(_))
        ));
    }

    /// Hold ohne Begruendung ist unzulaessig.
    #[test]
    fn hold_requires_reason() {
        let report = CanonValue::map([
            ("gate_id", CanonValue::text("G4-Residue")),
            ("verdict", CanonValue::text("hold")),
        ]);
        assert_eq!(
            GateReport::from_untyped(&report),
            Err(GateReportError::MissingReason)
        );
    }

    /// Fail-closed Kette: ein Hold verschmilzt nie zu Pass.
    #[test]
    fn chain_is_fail_closed() {
        let mut c = GateChain::new();
        c.push(GateReport::pass("G1-Scope", "im Scope"));
        c.push(GateReport::hold("G4-Residue", "stille Absorption erkannt"));
        assert!(!c.all_pass());
        assert_eq!(c.first_hold().unwrap().gate_id, "G4-Residue");
        // leere Kette ist ebenfalls kein Pass (fehlende Pruefung ⇒ Ablehnung, V7)
        assert!(!GateChain::new().all_pass());
    }
}
