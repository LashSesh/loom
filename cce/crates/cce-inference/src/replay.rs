//! recorded-Replay (IG-A3, S5-A5/A6): der Runner spielt aufgezeichnete
//! Responses EIN — kein Live-Re-Call im Replay-Pfad. Verlangt ein Lauf
//! Live-Re-Inferenz und weicht sie ab, ist das `model_replay_weak`
//! (sichtbar), nie ein stiller Drift.

use crate::gateway::InferenceRecorder;
use crate::residues::model_residue;
use crate::response::InferenceResponse;
use cce_core::residue::Residue;

/// Replay-Einspielung: die Aufzeichnung IST die Antwort.
pub fn replay_response(
    recorder: &InferenceRecorder,
    request_id: &str,
) -> Result<InferenceResponse, Box<Residue>> {
    recorder.lookup(request_id).cloned().ok_or_else(|| {
        Box::new(model_residue(
            "model_trace_missing",
            &format!("keine Aufzeichnung fuer Request {request_id} — Replay unmoeglich"),
        ))
    })
}

/// Vergleich Live-Re-Inferenz vs. Aufzeichnung (nur wo verlangt):
/// Abweichung = sichtbares model_replay_weak.
pub fn check_live_reinference(
    recorded: &InferenceResponse,
    live: &InferenceResponse,
) -> Result<(), Box<Residue>> {
    if recorded.digest() == live.digest() {
        Ok(())
    } else {
        Err(Box::new(model_residue(
            "model_replay_weak",
            "Live-Re-Inferenz weicht von Aufzeichnung ab — sichtbar, kein stiller Drift",
        )))
    }
}
