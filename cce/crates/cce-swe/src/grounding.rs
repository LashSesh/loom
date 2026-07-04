//! Agent-Grounding (Dokument 21): Regeln und offene Entscheidungen als
//! GEPRUEFTE Objekte. Kein Fliesstext-Prompt ist die Quelle — nur das
//! kompilierte, gehashte `GroundingPacket`. Abgeleitete, nicht-
//! massgebliche Export-Projektionen (menschenlesbare Kontextdatei fuer
//! ein Fremdwerkzeug) entstehen als reine Funktion `export_context`; sie
//! sind selbst NIE Eingabequelle fuer irgendeine Pruefung.
//!
//! Kein neues Kern-Crate, keine externe Kiste — reine Erweiterung von
//! `cce-swe` ueber `cce-core` (Digest/Residue), die dieselbe
//! Determinismus-Disziplin wie `RepoSnapshot::snapshot_root()` teilt.

use crate::residues::swe_residue;
use cce_core::residue::Residue;
use cce_core::signature::{sha256, Digest};

/// Der Verbindlichkeitsgrad einer Regel (Dokument 21 §2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleSeverity {
    Advisory,
    Required,
    Blocking,
}

impl RuleSeverity {
    pub fn as_str(self) -> &'static str {
        match self {
            RuleSeverity::Advisory => "advisory",
            RuleSeverity::Required => "required",
            RuleSeverity::Blocking => "blocking",
        }
    }
}

/// RuleAtom (§2): eine einzelne, geprueфte Projektregel. `trigger` ist
/// die Zeichenkette, die — taucht sie in einer HINZUGEFUEGTEN Diff-Zeile
/// innerhalb des `scope`-Pfads auf — die Regel ausloest (dieselbe
/// konservative, sichtbare Stichwort-Disziplin wie das ScopeGate v1 in
/// cce-bridge; als solche dokumentiert, keine Semantikanalyse).
#[derive(Debug, Clone)]
pub struct RuleAtom {
    pub rule_id: String,
    /// Pfad-/Modul-/Befehls-Praefix, fuer den die Regel gilt.
    pub scope: String,
    /// Ausloeser: verbotene Zeichenkette in einer `+`-Zeile.
    pub trigger: String,
    /// Was stattdessen zu tun ist (menschenlesbar).
    pub prescription: String,
    pub severity: RuleSeverity,
    /// Beleg: Repo-Fundstelle, Auftraggeber-Direktive oder vorher
    /// akzeptierte Entscheidung. Ohne Beleg wird die Regel beim
    /// Kompilieren automatisch auf `advisory` herabgestuft.
    pub evidence_ref: Option<String>,
    pub gate_ref: Option<String>,
    /// Optionaler Verfalls-/Zerfallshinweis (rein deklarativ).
    pub decay: Option<String>,
}

impl RuleAtom {
    fn digest_into(&self, buf: &mut Vec<u8>) {
        for part in [
            self.rule_id.as_str(),
            self.scope.as_str(),
            self.trigger.as_str(),
            self.prescription.as_str(),
            self.severity.as_str(),
            self.evidence_ref.as_deref().unwrap_or(""),
            self.gate_ref.as_deref().unwrap_or(""),
            self.decay.as_deref().unwrap_or(""),
        ] {
            buf.extend_from_slice(part.as_bytes());
            buf.push(0x1f);
        }
        buf.push(0x1e);
    }
}

/// Der Lebenszyklus einer architektonischen Entscheidung (§2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecisionStatus {
    Open,
    Proposed,
    Resolved,
    Stale,
    Superseded,
}

impl DecisionStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            DecisionStatus::Open => "open",
            DecisionStatus::Proposed => "proposed",
            DecisionStatus::Resolved => "resolved",
            DecisionStatus::Stale => "stale",
            DecisionStatus::Superseded => "superseded",
        }
    }
}

/// DecisionSlot (§2): eine offene architektonische Entscheidung wird NIE
/// stillschweigend von einem Modell getroffen — sie bleibt `Open`, bis
/// ein PhaseBlock sie explizit aufloest (aufgezeichnet wie jede
/// HITL-Entscheidung, S5-A5: der `Resolved`-Uebergang traegt RD-Ref +
/// Evidence).
#[derive(Debug, Clone)]
pub struct DecisionSlot {
    pub decision_id: String,
    pub question: String,
    /// Zulaessige Optionen.
    pub domain: Vec<String>,
    pub status: DecisionStatus,
    pub candidates: Vec<String>,
    pub evidence_ref: Option<String>,
    pub resolved_as: Option<String>,
    pub expires: Option<String>,
}

impl DecisionSlot {
    pub fn open(decision_id: &str, question: &str, domain: &[&str]) -> Self {
        Self {
            decision_id: decision_id.to_string(),
            question: question.to_string(),
            domain: domain.iter().map(|s| s.to_string()).collect(),
            status: DecisionStatus::Open,
            candidates: Vec::new(),
            evidence_ref: None,
            resolved_as: None,
            expires: None,
        }
    }

    /// Aufloesung durch einen PhaseBlock (S5-A5): `Open`/`Proposed` →
    /// `Resolved`, mit aufgezeichneter RD-Ref + Evidence. Eine
    /// Entscheidung ausserhalb der `domain` ist unzulaessig (fail-closed).
    pub fn resolve(
        &self,
        resolved_as: &str,
        rd_ref: &str,
        evidence_ref: &str,
    ) -> Result<DecisionSlot, Box<Residue>> {
        if !self.domain.iter().any(|d| d == resolved_as) {
            return Err(Box::new(swe_residue(
                "decision_left_open",
                &format!(
                    "'{resolved_as}' ausserhalb der zulaessigen domain {:?} — Slot bleibt offen",
                    self.domain
                ),
            )));
        }
        let mut next = self.clone();
        next.status = DecisionStatus::Resolved;
        next.resolved_as = Some(resolved_as.to_string());
        next.evidence_ref = Some(format!("{evidence_ref}; rd={rd_ref}"));
        Ok(next)
    }

    fn digest_into(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(self.decision_id.as_bytes());
        buf.push(0x1f);
        buf.extend_from_slice(self.question.as_bytes());
        buf.push(0x1f);
        buf.extend_from_slice(self.status.as_str().as_bytes());
        buf.push(0x1f);
        for d in &self.domain {
            buf.extend_from_slice(d.as_bytes());
            buf.push(0x1d);
        }
        buf.extend_from_slice(self.resolved_as.as_deref().unwrap_or("").as_bytes());
        buf.push(0x1e);
    }
}

/// GroundingPacket (§2): die kanonische, kompilierte Fassung.
/// `packet_digest` ist ein Merkle ueber die SORTIERTEN Regeln,
/// Entscheidungen und `tool_surface` — deterministisch, unabhaengig von
/// der Einfuegereihenfolge (dieselbe Disziplin wie `snapshot_root()`).
#[derive(Debug, Clone)]
pub struct GroundingPacket {
    pub package_id: String,
    pub rules: Vec<RuleAtom>,
    pub open_decisions: Vec<DecisionSlot>,
    pub tool_surface: Vec<String>,
}

impl GroundingPacket {
    pub fn packet_digest(&self) -> Digest {
        let mut rules = self.rules.clone();
        rules.sort_by(|a, b| a.rule_id.cmp(&b.rule_id));
        let mut decisions = self.open_decisions.clone();
        decisions.sort_by(|a, b| a.decision_id.cmp(&b.decision_id));
        let mut surface = self.tool_surface.clone();
        surface.sort();

        let mut buf = Vec::new();
        buf.extend_from_slice(self.package_id.as_bytes());
        buf.push(0x1e);
        for r in &rules {
            r.digest_into(&mut buf);
        }
        buf.push(0x1e);
        for d in &decisions {
            d.digest_into(&mut buf);
        }
        buf.push(0x1e);
        for s in &surface {
            buf.extend_from_slice(s.as_bytes());
            buf.push(0x1f);
        }
        sha256(&buf)
    }

    pub fn digest_hex(&self) -> String {
        self.packet_digest().to_hex()
    }

    /// Ungeloeste Slots (fuer `decision_left_open`-Sichtbarkeit).
    pub fn has_open_decisions(&self) -> bool {
        self.open_decisions
            .iter()
            .any(|d| matches!(d.status, DecisionStatus::Open | DecisionStatus::Proposed))
    }

    /// Gesamtzeichenzahl (fuer das ContextBudgetGate).
    pub fn context_size(&self) -> usize {
        export_context(self).len()
    }
}

/// Das Ergebnis von `compile_grounding`: das Packet + die sichtbaren
/// automatischen Herabstufungen (RuleEvidenceGate, §2).
#[derive(Debug, Clone)]
pub struct CompiledGrounding {
    pub packet: GroundingPacket,
    /// Je herabgestufter Regel ein sichtbares `rule_missing_evidence`.
    pub downgrades: Vec<Residue>,
}

/// `compile_grounding(Π) -> GroundingPacket` (§4): KOMPILIERT, nicht
/// diktiert. Jede Regel ohne `evidence_ref`, die nicht bereits
/// `advisory` ist, wird automatisch auf `advisory` herabgestuft — nie
/// stillschweigend als `blocking` gefuehrt (RuleEvidenceGate). Die
/// Herabstufung ist als `rule_missing_evidence` (Warning) sichtbar.
pub fn compile_grounding(
    package_id: &str,
    rules: Vec<RuleAtom>,
    open_decisions: Vec<DecisionSlot>,
    tool_surface: Vec<String>,
) -> CompiledGrounding {
    let mut downgrades = Vec::new();
    let compiled_rules: Vec<RuleAtom> = rules
        .into_iter()
        .map(|mut r| {
            if r.evidence_ref.is_none() && r.severity != RuleSeverity::Advisory {
                downgrades.push(swe_residue(
                    "rule_missing_evidence",
                    &format!(
                        "Regel '{}' ohne evidence_ref: {} -> advisory (automatisch, sichtbar)",
                        r.rule_id,
                        r.severity.as_str()
                    ),
                ));
                r.severity = RuleSeverity::Advisory;
            }
            r
        })
        .collect();
    CompiledGrounding {
        packet: GroundingPacket {
            package_id: package_id.to_string(),
            rules: compiled_rules,
            open_decisions,
            tool_surface,
        },
        downgrades,
    }
}

/// Abgeleitete, NICHT-massgebliche Export-Projektion (§2): eine
/// menschenlesbare Kontextdatei mit sichtbarem Herkunftsverweis auf
/// `packet_digest`. Reine Funktion — der Export ist selbst nie
/// Eingabequelle fuer irgendeine Pruefung.
pub fn export_context(packet: &GroundingPacket) -> String {
    let mut rules = packet.rules.clone();
    rules.sort_by(|a, b| a.rule_id.cmp(&b.rule_id));
    let mut decisions = packet.open_decisions.clone();
    decisions.sort_by(|a, b| a.decision_id.cmp(&b.decision_id));
    let mut surface = packet.tool_surface.clone();
    surface.sort();

    let mut out = String::new();
    out.push_str(&format!("# grounding-packet: {}\n", packet.package_id));
    out.push_str(&format!("# packet-digest: {}\n", packet.digest_hex()));
    out.push_str(
        "# ABGELEITETE ANSICHT — nicht massgeblich, nie Eingabequelle einer Pruefung.\n\n",
    );
    out.push_str("## Regeln\n");
    for r in &rules {
        out.push_str(&format!(
            "- [{}] {} (scope: {}) — {}\n",
            r.severity.as_str(),
            r.rule_id,
            r.scope,
            r.prescription
        ));
    }
    out.push_str("\n## Offene Entscheidungen\n");
    for d in &decisions {
        out.push_str(&format!(
            "- [{}] {}: {} (Optionen: {})\n",
            d.status.as_str(),
            d.decision_id,
            d.question,
            d.domain.join(" | ")
        ));
    }
    out.push_str("\n## Werkzeugoberflaeche (build/test/lint)\n");
    for s in &surface {
        out.push_str(&format!("- {s}\n"));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rule(id: &str, sev: RuleSeverity, evidence: Option<&str>) -> RuleAtom {
        RuleAtom {
            rule_id: id.to_string(),
            scope: "src/".to_string(),
            trigger: ".unwrap()".to_string(),
            prescription: "kein unwrap in src/".to_string(),
            severity: sev,
            evidence_ref: evidence.map(str::to_string),
            gate_ref: None,
            decay: None,
        }
    }

    #[test]
    fn rule_without_evidence_is_downgraded_visibly() {
        let compiled = compile_grounding(
            "pkg",
            vec![rule("R1", RuleSeverity::Blocking, None)],
            vec![],
            vec!["cargo test".to_string()],
        );
        assert_eq!(compiled.packet.rules[0].severity, RuleSeverity::Advisory);
        assert_eq!(compiled.downgrades.len(), 1);
        assert!(compiled.downgrades[0].id.contains("rule_missing_evidence"));
    }

    #[test]
    fn rule_with_evidence_keeps_blocking() {
        let compiled = compile_grounding(
            "pkg",
            vec![rule(
                "R1",
                RuleSeverity::Blocking,
                Some("repo:src/lib.rs:12"),
            )],
            vec![],
            vec![],
        );
        assert_eq!(compiled.packet.rules[0].severity, RuleSeverity::Blocking);
        assert!(compiled.downgrades.is_empty());
    }

    #[test]
    fn packet_digest_is_order_independent() {
        let a = compile_grounding(
            "pkg",
            vec![
                rule("R1", RuleSeverity::Blocking, Some("e1")),
                rule("R2", RuleSeverity::Required, Some("e2")),
            ],
            vec![],
            vec!["cargo build".to_string(), "cargo test".to_string()],
        )
        .packet;
        let b = compile_grounding(
            "pkg",
            vec![
                rule("R2", RuleSeverity::Required, Some("e2")),
                rule("R1", RuleSeverity::Blocking, Some("e1")),
            ],
            vec![],
            vec!["cargo test".to_string(), "cargo build".to_string()],
        )
        .packet;
        assert_eq!(a.packet_digest(), b.packet_digest());
    }

    #[test]
    fn decision_resolves_only_within_domain() {
        let slot = DecisionSlot::open("D1", "Welche Log-Bibliothek?", &["tracing", "log"]);
        assert!(slot.resolve("serde", "rd:1", "ev:1").is_err());
        let resolved = slot.resolve("tracing", "rd:1", "ev:1").expect("in domain");
        assert_eq!(resolved.status, DecisionStatus::Resolved);
        assert_eq!(resolved.resolved_as.as_deref(), Some("tracing"));
        assert!(resolved.evidence_ref.as_deref().unwrap().contains("rd:1"));
    }

    #[test]
    fn export_context_carries_digest_provenance() {
        let packet = compile_grounding(
            "pkg",
            vec![rule("R1", RuleSeverity::Blocking, Some("e1"))],
            vec![DecisionSlot::open("D1", "?", &["a", "b"])],
            vec!["cargo test".to_string()],
        )
        .packet;
        let text = export_context(&packet);
        assert!(text.contains(&packet.digest_hex()));
        assert!(text.contains("nicht massgeblich"));
        assert!(text.contains("R1"));
    }
}
