//! Die HBM-Pipeline, Phasen 0–9 (Rebase §3/§4.2 Formel 6):
//! 0 Ingest → 1 Facets → 2 Gate_A → 3 Cube/HDAG → 4 Skeleton/JT →
//! 5 Kandidaten C1–C6 (Score-RANKING, θ_D Vorauswahl) → 6 ExclusionGate →
//! 7 LayerExpansion → 8 CrystalFinalization → 9 Registry/Replay.
//! Deterministisch, RD-gebunden; Gate-Dominanz: Materialize(B) ⇒ Gate(B)=Pass.

use crate::candidate::{
    crystal_finalization, exclusion_gate, generate_candidates, layer_expansion, BlueprintCandidate,
    CandidateStatus,
};
use crate::facet::{extract_facets, gate_a};
use crate::score::{rank, ScoreWeights};
use cce_core::gate::{GateChain, GateReport};
use cce_core::ledger::{Ledger, LedgerEventKind};
use cce_core::signature::Digest;
use cce_lattice::graph::{Graph, JunctionTree};

/// Eingang der Pipeline (RD-gebunden).
#[derive(Debug, Clone)]
pub struct MiningInput {
    pub corpus_id: String,
    pub lines: Vec<String>,
    pub weights: ScoreWeights,
    pub theta_d: u64,
    pub expansion_budget: usize,
}

#[derive(Debug)]
pub struct PipelineOutcome {
    pub facets: usize,
    pub candidates: Vec<BlueprintCandidate>,
    pub ranking: Vec<(String, u64)>,
    pub certified: Vec<(String, Digest)>,
    pub ledger: Ledger,
    pub holds: Vec<(String, String)>,
    pub treewidth: Option<usize>,
}

#[derive(Debug)]
pub enum PipelineError {
    GateAHold(GateReport),
}

/// Der deterministische Lauf 0–9.
pub fn run_pipeline(input: &MiningInput) -> Result<PipelineOutcome, PipelineError> {
    let mut ledger = Ledger::new();
    // 0 Ingest + 1 Facet-Extraktion.
    let line_refs: Vec<&str> = input.lines.iter().map(String::as_str).collect();
    let facets = extract_facets(&input.corpus_id, &line_refs);
    // 2 Gate_A (Adapter-Mining-Gate) — fail-closed.
    let ga = gate_a(&facets);
    if !ga.is_pass() {
        return Err(PipelineError::GateAHold(ga));
    }
    // 3 Cube/HDAG + 4 Skeleton/Junction-Tree (Baumweitenbewusstsein, 5.5).
    let mut g = Graph::new(&facets.iter().map(|f| f.id.as_str()).collect::<Vec<&str>>());
    for w in facets.windows(2) {
        g.add_edge(&w[0].id, &w[1].id);
    }
    let treewidth = JunctionTree::build(&g).map(|jt| jt.treewidth());
    // 5 Kandidaten C1–C6 + Score-RANKING (θ_D = Vorauswahl, nie Abnahme).
    let mut candidates = generate_candidates(&facets);
    let ranking = rank(&candidates, &input.weights, input.theta_d);
    let preselected: Vec<String> = ranking.iter().map(|(id, _)| id.clone()).collect();
    // 6–8: Nur vorausgewaehlte Kandidaten werden WEITERGERECHNET; die
    // Entscheidung faellt ausschliesslich an den Gates.
    let mut certified = Vec::new();
    let mut holds = Vec::new();
    for cand in candidates.iter_mut() {
        if !preselected.contains(&cand.id) {
            cand.status = CandidateStatus::Hold;
            cand.hold_diagnosis =
                Some("θ_D-Vorauswahl: nicht weitergerechnet (KEINE Abnahmeentscheidung)".into());
            holds.push((cand.id.clone(), cand.hold_diagnosis.clone().unwrap()));
            continue;
        }
        // 6 ExclusionGate.
        let exc = exclusion_gate(cand);
        if !exc.is_pass() {
            cand.status = CandidateStatus::Hold;
            cand.hold_diagnosis = Some(exc.reason.clone());
            holds.push((cand.id.clone(), exc.reason.clone()));
            ledger.append(LedgerEventKind::Residue, cand.class().0);
            continue;
        }
        // 7 LayerExpansion.
        let exp = layer_expansion(cand, input.expansion_budget);
        if !exp.is_pass() {
            cand.status = CandidateStatus::Hold;
            cand.hold_diagnosis = Some(exp.reason.clone());
            holds.push((cand.id.clone(), exp.reason.clone()));
            ledger.append(LedgerEventKind::Residue, cand.class().0);
            continue;
        }
        // 8 CrystalFinalization (FIN = CCC-Kristallbedingung).
        let mut gates = GateChain::new();
        for g in cce_core::gate::mandatory_gates() {
            gates.push(GateReport::pass(&g.id, "HBM-Pfad erfuellt"));
        }
        gates.push(exc);
        gates.push(exp);
        match crystal_finalization(cand, &gates) {
            Ok(crystal) => {
                cand.status = CandidateStatus::Pass;
                cand.hold_diagnosis = None;
                // 9 Registry/Replay: Commit in den Ledger.
                ledger.append(LedgerEventKind::Commit, crystal.class().0);
                certified.push((cand.id.clone(), crystal.class().0));
            }
            Err(reasons) => {
                cand.status = CandidateStatus::Reject;
                cand.hold_diagnosis = Some(format!("finalization_incomplete: {reasons:?}"));
                holds.push((cand.id.clone(), cand.hold_diagnosis.clone().unwrap()));
            }
        }
    }
    Ok(PipelineOutcome {
        facets: facets.len(),
        candidates,
        ranking,
        certified,
        ledger,
        holds,
        treewidth,
    })
}

/// Gate-Dominanz (Operatoralgebra): Materialize(B) ⇒ Gate(B) = Pass.
/// Es existiert kein Pfad, einen Nicht-Pass-Kandidaten zu materialisieren.
pub fn materialize_candidate(b: &BlueprintCandidate) -> Result<Vec<u8>, GateReport> {
    if b.status != CandidateStatus::Pass {
        return Err(GateReport::hold(
            "MaterializeGate",
            &format!(
                "Gate-Dominanz: Kandidat {} ist {:?}, nicht Pass — Materialisierung verweigert",
                b.id, b.status
            ),
        ));
    }
    Ok(b.content().encode())
}
