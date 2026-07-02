//! nexus-cell — Akquisitionszellen als Workbench-Capsules (CSA.9).
//!
//! Eine Zelle ist ein befristetes Arbeitsorgan fuer GENAU EINE Quelle
//! in GENAU EINEM Lauf. Der Source-Ratchet ist die Fortschrittsregel:
//!
//!   advance(R_src) = 1  ⟺  Gate = Pass ∧ Evidence = 1 ∧ Residue sichtbar
//!
//! CSA-INV-2 (kein Policy-Drift): die Zelle traegt einen unveraenderlichen
//! Policy-Schnappschuss aus dem ApprovedFetchPlan; jede Adaptation der
//! Zelle laesst diesen Schnappschuss byte-identisch — es gibt keinen
//! Mutationspfad (privates Feld, keine Setter).

use cce_core::gate::GateReport;
use cce_core::residue::Residue;
use nexus_core::residues::csa_residue;
use nexus_policy::ApprovedFetchPlan;

/// Rollen im Zellcluster (CSA.9): Planer, Holer, Extraktor,
/// Normalisierer, Validierer, Zitierer — je Rolle ein Ratchet-Schritt.
pub const CELL_ROLES: [&str; 6] = [
    "planner",
    "fetcher",
    "extractor",
    "normalizer",
    "validator",
    "citer",
];

/// Ein Ratchet-Schritt: benanntes Gate + Evidence-Referenz + sichtbares
/// (ggf. leeres) Residuenfeld.
#[derive(Debug, Clone)]
pub struct RatchetStep {
    pub role: String,
    pub gate: GateReport,
    pub evidence_refs: Vec<String>,
    pub residues: Vec<Residue>,
}

/// Der Source-Ratchet: monotone Fortschrittsachse der Zelle.
/// Jeder `advance` prueft die Dreifach-Bedingung; ein Fehlschlag ist
/// ein Endzustand des Schritts (kein stiller Rueckbau, INV: Ratchet
/// kennt kein Zurueck).
#[derive(Debug, Default)]
pub struct SourceRatchet {
    steps: Vec<RatchetStep>,
}

impl SourceRatchet {
    pub fn position(&self) -> usize {
        self.steps.len()
    }

    /// advance(R_src)=1 ⟺ Gate=Pass ∧ Evidence=1 ∧ Residue sichtbar.
    /// „Residue sichtbar" heisst: das Feld ist deklariert (auch leer
    /// zaehlt — leer ist „geschlossen (∅)", nicht „verschwiegen").
    pub fn advance(&mut self, step: RatchetStep) -> Result<usize, Box<Residue>> {
        if !step.gate.is_pass() {
            return Err(Box::new(csa_residue(
                "score_as_gate_attempt",
                &format!(
                    "ratchet_no_advance: Rolle {} ohne Gate=Pass ({})",
                    step.role, step.gate.reason
                ),
            )));
        }
        if step.evidence_refs.is_empty() {
            return Err(Box::new(csa_residue(
                "evidence_missing",
                &format!("ratchet_no_advance: Rolle {} ohne Evidence", step.role),
            )));
        }
        self.steps.push(step);
        Ok(self.steps.len())
    }

    pub fn steps(&self) -> &[RatchetStep] {
        &self.steps
    }
}

/// Akquisitionszelle: befristet, quellengebunden, policy-versiegelt.
/// Nach Abschluss wird sie AUFGELOEST (consuming `dissolve`) — es bleibt
/// nur das Protokoll (Schritte + Policy-Schnappschuss-Digest).
#[derive(Debug)]
pub struct AcquisitionCell {
    pub cell_id: String,
    pub adapter_id: String,
    /// Unveraenderlicher Policy-Schnappschuss (CSA-INV-2): bei Bau aus
    /// dem ApprovedFetchPlan kopiert; kein Setter, kein &mut-Zugriff.
    policy_snapshot: String,
    pub ratchet: SourceRatchet,
}

/// Protokoll einer aufgeloesten Zelle: das einzige, was ueberlebt.
#[derive(Debug, Clone)]
pub struct CellProtocol {
    pub cell_id: String,
    pub adapter_id: String,
    pub policy_snapshot: String,
    pub steps_completed: usize,
    pub roles: Vec<String>,
}

impl AcquisitionCell {
    /// Zellen entstehen NUR aus einem versiegelten ApprovedFetchPlan —
    /// eine Zelle ohne Policy-Beleg ist nicht konstruierbar.
    pub fn spawn(cell_id: &str, approved: &ApprovedFetchPlan) -> Self {
        Self {
            cell_id: cell_id.to_string(),
            adapter_id: approved.plan.adapter_id.clone(),
            policy_snapshot: approved.policy_snapshot.clone(),
            ratchet: SourceRatchet::default(),
        }
    }

    /// Lesender Zugriff auf den Schnappschuss (fuer Drift-Pruefung).
    pub fn policy_snapshot(&self) -> &str {
        &self.policy_snapshot
    }

    /// CSA-INV-2-Pruefung: Zellenadaptation ohne Policy-Drift — der
    /// Schnappschuss der Zelle muss dem des Belegs byte-gleich sein.
    pub fn check_no_policy_drift(&self, approved: &ApprovedFetchPlan) -> GateReport {
        if self.policy_snapshot == approved.policy_snapshot {
            GateReport::pass(
                "csa_inv2_no_policy_drift",
                "Policy-Schnappschuss byte-identisch",
            )
        } else {
            GateReport::hold(
                "csa_inv2_no_policy_drift",
                "policy_drift: Zellen-Schnappschuss weicht vom Beleg ab",
            )
        }
    }

    /// Aufloesung (consuming): die Zelle hoert auf zu existieren,
    /// zurueck bleibt nur das Protokoll.
    pub fn dissolve(self) -> CellProtocol {
        CellProtocol {
            cell_id: self.cell_id,
            adapter_id: self.adapter_id,
            policy_snapshot: self.policy_snapshot,
            steps_completed: self.ratchet.position(),
            roles: self
                .ratchet
                .steps()
                .iter()
                .map(|s| s.role.clone())
                .collect(),
        }
    }
}

/// Zellcluster: gemeinsamer Lauf mehrerer Zellen (eine je Quelle);
/// der Cluster selbst hat KEINE eigene Policy — er delegiert an die
/// versiegelten Belege der Zellen.
#[derive(Debug, Default)]
pub struct CellCluster {
    pub protocols: Vec<CellProtocol>,
}

impl CellCluster {
    pub fn absorb(&mut self, protocol: CellProtocol) {
        self.protocols.push(protocol);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nexus_adapter::manifest::AdapterManifest;
    use nexus_adapter::port::{FetchPlan, PlannedOp};
    use nexus_core::objects::{SourceHorizon, TaskSpec};
    use nexus_policy::{approve_fetch, SourceDeclaration};

    fn approved() -> ApprovedFetchPlan {
        let hs = SourceHorizon::example_local();
        let task = TaskSpec::new("test", "scope:test");
        let manifest = AdapterManifest::complete("a-local", "local_corpus", "cc-by-4.0");
        let decl = SourceDeclaration {
            adapter_class: "local_corpus".into(),
            access_method: "filesystem".into(),
            robots_or_terms: Some("not_applicable".into()),
            license: Some("cc-by-4.0".into()),
            contains_pii: false,
            requested_actions: vec![],
        };
        let plan = FetchPlan {
            plan_id: "p1".into(),
            adapter_id: "a-local".into(),
            operations: vec![PlannedOp {
                method: "read".into(),
                endpoint_template: "corpus://{id}".into(),
                expected_status: 200,
                cost_requests: 1,
            }],
            gates: vec!["rate_budget_gate".into()],
            cache_etag: None,
            cache_cursor: None,
            backoff_policy: "none".into(),
        };
        approve_fetch(&hs, &task, &manifest, &decl, plan).expect("policy allow")
    }

    fn pass_step(role: &str) -> RatchetStep {
        RatchetStep {
            role: role.into(),
            gate: GateReport::pass(role, "ok"),
            evidence_refs: vec![format!("ev:{role}")],
            residues: vec![],
        }
    }

    #[test]
    fn ratchet_advances_only_with_gate_and_evidence() {
        let mut r = SourceRatchet::default();
        assert_eq!(r.advance(pass_step("planner")).unwrap(), 1);
        // Gate=Hold ⇒ kein advance
        let mut hold = pass_step("fetcher");
        hold.gate = GateReport::hold("fetcher", "nicht bereit");
        assert!(r.advance(hold).is_err());
        // Evidence leer ⇒ kein advance
        let mut no_ev = pass_step("fetcher");
        no_ev.evidence_refs.clear();
        assert!(r.advance(no_ev).is_err());
        assert_eq!(r.position(), 1);
    }

    #[test]
    fn cell_spawns_sealed_and_dissolves_to_protocol() {
        let ap = approved();
        let mut cell = AcquisitionCell::spawn("c1", &ap);
        assert!(cell.check_no_policy_drift(&ap).is_pass());
        for role in CELL_ROLES {
            cell.ratchet.advance(pass_step(role)).unwrap();
        }
        let protocol = cell.dissolve();
        assert_eq!(protocol.steps_completed, 6);
        assert_eq!(protocol.roles, CELL_ROLES.to_vec());
    }
}
