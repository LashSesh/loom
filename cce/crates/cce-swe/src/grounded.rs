//! Der GROUNDED Einstieg (Dokument 21 §4): ein `GroundingPacket` wirkt
//! als zusaetzliche Vorpruefung VOR der unveraenderten P2-Kern-Kette
//! (`run_swe_task` bleibt exakt wie in P2/P3 — Alt-Zeugen unberuehrt).
//! Reihenfolge der Vorpruefung: ContextBudgetGate (Packet-Groesse) →
//! RuleComplianceGate (blocking-Regeln, UNABHAENGIG vom Modell-Output) →
//! DeltaBudgetGate (Aenderungsgroesse). Erst wenn alle drei durchgehen,
//! laeuft die bestehende Werkzeug-/Kern-Gate-Kette.

use crate::gates::{
    context_budget_gate, delta_budget_gate, diff_delta_lines, rule_compliance_gate, InfVerdict,
};
use crate::grounding::GroundingPacket;
use crate::kette::{run_swe_task, SweOutcome};
use crate::model::{DiffCandidate, RepoSnapshot};
use cce_core::signature::Digest;
use cce_toolgateway::gateway::{BuildTool, FsWriteTool, TestTool, ToolGateway};
use cce_toolgateway::manifest::ToolManifest;

/// Ergebnis eines grounded Laufs.
#[derive(Debug)]
pub enum GroundedOutcome {
    /// Grounding-Vorpruefung bestanden — Ergebnis der P2-Kern-Kette.
    Swe(SweOutcome),
    /// Grounding-Vorpruefung hielt/rejizierte VOR jeder Werkzeug-Wirkung
    /// (kein Apply, kein Build/Test versucht).
    BlockedByGrounding(Vec<InfVerdict>),
}

/// Die Grounding-Vorpruefung allein (ohne die P2-Kette) — nuetzlich fuer
/// Zeugen und fuer Aufrufer, die nur pruefen wollen.
pub fn grounding_precheck(
    packet: &GroundingPacket,
    context_char_limit: usize,
    diff: &DiffCandidate,
    delta_budget: usize,
    delta_increase_confirmation: Option<&str>,
) -> Vec<InfVerdict> {
    let verdicts = [
        context_budget_gate(packet, context_char_limit),
        rule_compliance_gate(diff, packet),
        delta_budget_gate(
            diff_delta_lines(diff),
            delta_budget,
            delta_increase_confirmation,
        ),
    ];
    verdicts.into_iter().filter(|v| !v.allows()).collect()
}

/// Grounded Kern-Kette: Grounding-Vorpruefung, dann die UNVERAENDERTE
/// `run_swe_task`.
#[allow(clippy::too_many_arguments)]
pub fn run_grounded_swe_task(
    packet: &GroundingPacket,
    context_char_limit: usize,
    delta_budget: usize,
    delta_increase_confirmation: Option<&str>,
    diff: &DiffCandidate,
    base: &RepoSnapshot,
    gw: &mut ToolGateway,
    fs_write_manifest: &ToolManifest,
    fs_write_tool: &mut FsWriteTool,
    build_manifest: &ToolManifest,
    build_tool: &BuildTool,
    test_manifest: &ToolManifest,
    test_tool: &TestTool,
    breaks_existing_witness: bool,
    rd_ref: Digest,
) -> GroundedOutcome {
    let failed = grounding_precheck(
        packet,
        context_char_limit,
        diff,
        delta_budget,
        delta_increase_confirmation,
    );
    if !failed.is_empty() {
        return GroundedOutcome::BlockedByGrounding(failed);
    }
    GroundedOutcome::Swe(run_swe_task(
        diff,
        base,
        gw,
        fs_write_manifest,
        fs_write_tool,
        build_manifest,
        build_tool,
        test_manifest,
        test_tool,
        breaks_existing_witness,
        rd_ref,
    ))
}
