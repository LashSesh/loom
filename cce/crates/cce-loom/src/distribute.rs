//! `loom_generate`-Referenzalgorithmus (Teil 7.4, LOOM §20) mit
//! repair_or_reweave: fail-closed; ein gate_failed-Ergebnis wird NIEMALS
//! verschmolzen.

use crate::workcell::{LoomWorkcell, WorkcellStatus};
use cce_core::gate::{GateChain, GateReport};
use cce_core::ledger::{Ledger, LedgerEventKind};
use cce_core::signature::sha256;
use cce_core::value::CanonValue;

/// Lokaler Ausfuehrer (Agent/Solver/Mensch = lokaler Operator, NIE Autoritaet).
pub trait WorkcellRuntime {
    fn execute(&self, cell: &LoomWorkcell) -> CanonValue;
}

#[derive(Debug)]
pub enum LoomOutcome {
    /// Alle Zellen committed: Ergebnisliste + Ledger.
    Committed {
        results: Vec<(String, CanonValue)>,
        ledger: Ledger,
    },
    /// Hold: Diagnose + Ledger — Eingang fuer repair_or_reweave.
    RepairOrReweave {
        failed_cell: String,
        report: GateReport,
        ledger: Ledger,
    },
}

/// Fuehrt die Workcells in deterministischer Ordnung aus, gated fail-closed,
/// committed in den Ledger. Gate-Pruefung je Zelle via `gate_cell`.
pub fn loom_generate(
    cells: &mut [LoomWorkcell],
    runtime: &dyn WorkcellRuntime,
    gate_cell: impl Fn(&LoomWorkcell, &CanonValue) -> GateReport,
) -> LoomOutcome {
    let mut ledger = Ledger::new();
    let mut results = Vec::new();
    for cell in cells.iter_mut() {
        cell.status = WorkcellStatus::Running;
        let result = runtime.execute(cell);
        let report = gate_cell(cell, &result);
        if !report.is_pass() {
            cell.status = WorkcellStatus::GateFailed;
            let mut chain = GateChain::new();
            chain.push(report.clone());
            cell.gates = chain;
            ledger.append(LedgerEventKind::Residue, sha256(report.reason.as_bytes()));
            // Hold = Diagnose, nie Fire: sofortiger Ausstieg (fail-closed).
            return LoomOutcome::RepairOrReweave {
                failed_cell: cell.id.clone(),
                report,
                ledger,
            };
        }
        cell.status = WorkcellStatus::Emitted;
        let mut chain = GateChain::new();
        chain.push(report);
        cell.gates = chain;
        ledger.append(
            LedgerEventKind::Commit,
            sha256(&result.normalize().encode()),
        );
        cell.status = WorkcellStatus::Sealed;
        results.push((cell.id.clone(), result));
    }
    LoomOutcome::Committed { results, ledger }
}
