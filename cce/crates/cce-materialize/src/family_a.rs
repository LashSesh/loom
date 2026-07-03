//! Familien-Kern A — Dokument/Text (S1_DOMAENENKATALOG Familie A).
//!
//! Masterplan §2: EIN geteilter Kern, Domänen sind reine Spezialisierung
//! (nur ihre Differenz ist Eigencode, NIE Kopie). Die gesamte Motorik
//! (encode/loom/materialize/reanalyze/canonicalize/equivalent) delegiert
//! an den bestehenden `DocumentAdapter` (D01); je Domäne kommt nur
//! hinzu: (a) ihre Kern-Gate-REGEL + Kern-Residuum aus dem Katalog,
//! (b) Referenz- und Negativ-Cubes. So bleibt D01 unberührt und die
//! 14 Folgedomänen D02–D15 teilen sich einen Kern.

use crate::adapter::DomainAdapter;
use crate::document::{DocArtifact, DocCrystal, DocUnit, DocWeave, DocumentAdapter, UnitType};
use cce_core::canonical::{CanonicalClass, CanonicalState};
use cce_core::gate::{Gate, GateKind, GateReport};
use cce_core::objects::CounterHorizon;
use cce_core::residue::{Residue, ResidueKind};
use cce_phc::package::PhcPackage;
use cce_phc::projection_calc::LocalProjection;

/// Die Kern-Regel einer Domäne (verallgemeinert die S1-Referenzprofile).
#[derive(Debug, Clone, Copy)]
pub enum DomainRule {
    /// Jede Subjekt-Einheit (Claim/Risk) braucht eine Naht `seam` zu
    /// einer existierenden Einheit — sonst `core_residue`.
    /// (D02 Verweis, D03 Ableit, D06 Preis, D07 Geltung, D08 Beschluss,
    ///  D09 Quell, D10 Beleg, D12 Argument, D13 Antwort)
    Relation { seam: &'static str },
    /// Wie Relation, zusätzlich: der Relations-Graph muss AZYKLISCH sein
    /// (D15 Glossar: keine Definitions-Zirkel).
    AcyclicRelation { seam: &'static str },
    /// Subjekt-Einheiten mit `seam`-Zeitnaht müssen lückenlos verkettet
    /// sein (D11 CV: Chronologie ohne Lücke).
    ChainedRelation { seam: &'static str },
    /// Subjekt-Texte müssen EINDEUTIG sein (D14 Checkliste).
    UniqueSubjects,
    /// Reihenfolge-Vollständigkeit: Schritte lückenlos nummeriert über
    /// die `next`-Naht (D04 Manual).
    OrderedSteps { seam: &'static str },
    /// Geforderte Struktur-Marker müssen in Section-Einheiten vorkommen
    /// (D05 Brief: Anrede/Schluss-Vollständigkeit).
    StructuralPresence { markers: &'static [&'static str] },
}

/// Ein Domänenprofil: der gesamte Unterschied einer Leaf-Domäne.
#[derive(Clone, Copy)]
pub struct DocProfile {
    pub id: &'static str,
    /// Der Unit-Typ der „Subjekt"-Einheit (nutzt das geteilte Vokabular).
    pub subject: UnitType,
    pub rule: DomainRule,
    /// Kern-Residuum aus dem Katalog (K.4-Spalte).
    pub core_residue: &'static str,
    pub reference: fn() -> DocCrystal,
    pub negatives: fn() -> Vec<(DocCrystal, &'static str)>,
    pub export_formats: &'static [&'static str],
}

/// Der generische Domänen-Adapter über dem Familien-Kern. Hält ein
/// Profil und einen inneren DocumentAdapter für die Motorik.
pub struct DocDomainAdapter {
    pub profile: DocProfile,
    inner: DocumentAdapter,
}

impl DocDomainAdapter {
    pub fn new(profile: DocProfile) -> Self {
        Self {
            profile,
            inner: DocumentAdapter,
        }
    }
}

/// Die domänenspezifische Kern-Gate-Funktion: setzt die Regel des
/// Profils durch und benennt bei Verletzung das Kern-Residuum.
pub fn domain_core_gate(profile: &DocProfile, c: &DocCrystal) -> GateReport {
    let gate_id = format!("core:{}", profile.id);
    let exists = |id: &str| c.units.iter().any(|u| u.id == id);
    let subjects: Vec<&DocUnit> = c
        .units
        .iter()
        .filter(|u| u.unit_type == profile.subject)
        .collect();
    match profile.rule {
        DomainRule::Relation { seam } => {
            for s in &subjects {
                if !s.seams.iter().any(|(k, to)| k == seam && exists(to)) {
                    return GateReport::hold(
                        &gate_id,
                        &format!(
                            "{}: Einheit {} ohne aufloesbare {seam}-Naht",
                            profile.core_residue, s.id
                        ),
                    );
                }
            }
            GateReport::pass(&gate_id, "jede Subjekt-Einheit relational geschlossen")
        }
        DomainRule::AcyclicRelation { seam } => {
            // erst Relation, dann Azyklizität ueber die seam-Kanten.
            for s in &subjects {
                if !s.seams.iter().any(|(k, to)| k == seam && exists(to)) {
                    return GateReport::hold(
                        &gate_id,
                        &format!("{}: {} ohne {seam}-Naht", profile.core_residue, s.id),
                    );
                }
            }
            if has_cycle(c, seam) {
                return GateReport::hold(
                    &gate_id,
                    &format!("{}: Zirkel im {seam}-Graph", profile.core_residue),
                );
            }
            GateReport::pass(&gate_id, "relational geschlossen und azyklisch")
        }
        DomainRule::ChainedRelation { seam } => {
            // Kette: genau eine Wurzel (ohne eingehende seam), lueckenlos.
            let targets: std::collections::BTreeSet<&str> = subjects
                .iter()
                .flat_map(|u| u.seams.iter())
                .filter(|(k, _)| k == seam)
                .map(|(_, t)| t.as_str())
                .collect();
            let roots = subjects
                .iter()
                .filter(|u| !targets.contains(u.id.as_str()))
                .count();
            if subjects.len() > 1 && roots != 1 {
                return GateReport::hold(
                    &gate_id,
                    &format!(
                        "{}: Kette nicht lueckenlos ({roots} Wurzeln)",
                        profile.core_residue
                    ),
                );
            }
            GateReport::pass(&gate_id, "Kette lueckenlos verknuepft")
        }
        DomainRule::UniqueSubjects => {
            let mut seen = std::collections::BTreeSet::new();
            for s in &subjects {
                let key = s.text.split_whitespace().collect::<Vec<_>>().join(" ");
                if !seen.insert(key) {
                    return GateReport::hold(
                        &gate_id,
                        &format!(
                            "{}: mehrdeutiger/duplizierter Pruefpunkt {}",
                            profile.core_residue, s.id
                        ),
                    );
                }
            }
            GateReport::pass(&gate_id, "alle Pruefpunkte eindeutig")
        }
        DomainRule::StructuralPresence { markers } => {
            for m in markers {
                let present = c
                    .units
                    .iter()
                    .any(|u| u.unit_type == UnitType::Section && u.text.contains(m));
                if !present {
                    return GateReport::hold(
                        &gate_id,
                        &format!("{}: Struktur-Marker '{m}' fehlt", profile.core_residue),
                    );
                }
            }
            GateReport::pass(&gate_id, "alle geforderten Struktur-Marker vorhanden")
        }
        DomainRule::OrderedSteps { seam } => {
            let targets: std::collections::BTreeSet<&str> = subjects
                .iter()
                .flat_map(|u| u.seams.iter())
                .filter(|(k, _)| k == seam)
                .map(|(_, t)| t.as_str())
                .collect();
            let roots = subjects
                .iter()
                .filter(|u| !targets.contains(u.id.as_str()))
                .count();
            if subjects.len() > 1 && roots != 1 {
                return GateReport::hold(
                    &gate_id,
                    &format!("{}: Ablauf unvollstaendig geordnet", profile.core_residue),
                );
            }
            GateReport::pass(&gate_id, "Ablauf lueckenlos geordnet")
        }
    }
}

fn has_cycle(c: &DocCrystal, seam: &str) -> bool {
    use std::collections::{BTreeMap, BTreeSet};
    let mut adj: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for u in &c.units {
        for (k, to) in &u.seams {
            if k == seam {
                adj.entry(u.id.as_str()).or_default().push(to.as_str());
            }
        }
    }
    // DFS-Zyklenerkennung.
    let mut color: BTreeMap<&str, u8> = BTreeMap::new();
    fn dfs<'a>(
        n: &'a str,
        adj: &BTreeMap<&'a str, Vec<&'a str>>,
        color: &mut BTreeMap<&'a str, u8>,
    ) -> bool {
        color.insert(n, 1);
        if let Some(ns) = adj.get(n) {
            for &m in ns {
                match color.get(m).copied().unwrap_or(0) {
                    1 => return true,
                    0 => {
                        if dfs(m, adj, color) {
                            return true;
                        }
                    }
                    _ => {}
                }
            }
        }
        color.insert(n, 2);
        false
    }
    let nodes: BTreeSet<&str> = adj.keys().copied().collect();
    for n in nodes {
        if color.get(n).copied().unwrap_or(0) == 0 && dfs(n, &adj, &mut color) {
            return true;
        }
    }
    false
}

impl DomainAdapter for DocDomainAdapter {
    type Crystal = DocCrystal;
    type Weave = DocWeave;
    type Artifact = DocArtifact;

    fn domain_id(&self) -> &'static str {
        self.profile.id
    }

    fn wish_schema(&self) -> Vec<(&'static str, &'static str)> {
        let mut s = self.inner.wish_schema();
        s.push(("core_rule", "domaenenspezifische Kern-Naht-Regel"));
        s
    }

    fn validate_wish(&self, c: &DocCrystal) -> Result<(), Vec<Residue>> {
        self.inner.validate_wish(c)
    }

    fn to_canonical(&self, c: &DocCrystal) -> CanonicalState {
        self.inner.to_canonical(c)
    }

    fn encode(&self, c: &DocCrystal) -> PhcPackage {
        self.inner.encode(c)
    }

    fn loom(&self, p: &LocalProjection) -> Result<DocWeave, String> {
        self.inner.loom(p)
    }

    fn materialize(&self, w: &DocWeave) -> DocArtifact {
        self.inner.materialize(w)
    }

    fn reanalyze(&self, a: &DocArtifact) -> Result<DocCrystal, String> {
        self.inner.reanalyze(a)
    }

    fn canonicalize(&self, c: &DocCrystal) -> CanonicalClass {
        self.inner.canonicalize(c)
    }

    fn domain_gates(&self) -> Vec<Gate> {
        let mut g = self.inner.domain_gates();
        g.push(Gate::new(
            &format!("core:{}", self.profile.id),
            GateKind::Integrity,
            "domaenenspezifische Kern-Naht-Regel",
        ));
        g
    }

    fn run_domain_gates(&self, c: &DocCrystal, artifact: Option<&DocArtifact>) -> Vec<GateReport> {
        let mut reports = self.inner.run_domain_gates(c, artifact);
        reports.push(domain_core_gate(&self.profile, c));
        reports
    }

    fn residue_vocabulary(&self) -> Vec<ResidueKind> {
        let mut v = self.inner.residue_vocabulary();
        v.push(ResidueKind::named(self.profile.core_residue));
        v
    }

    fn counter_horizon(&self, c: &DocCrystal) -> CounterHorizon {
        self.inner.counter_horizon(c)
    }

    fn native_open(&self, a: &DocArtifact) -> crate::adapter::OpenAction {
        self.inner.native_open(a)
    }

    fn export_formats(&self) -> Vec<&'static str> {
        self.profile.export_formats.to_vec()
    }

    fn reference_cube(&self) -> DocCrystal {
        (self.profile.reference)()
    }

    fn negative_cubes(&self) -> Vec<(DocCrystal, ResidueKind)> {
        (self.profile.negatives)()
            .into_iter()
            .map(|(c, r)| (c, ResidueKind::named(r)))
            .collect()
    }
}
