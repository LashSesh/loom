//! DER Regressionswaechter, CI-gebunden (S8.3): ab G6 prueft JEDER Lauf
//! alle bisherigen Zeugen. Diese Datei ist der Einstiegspunkt — die
//! Zeugenmenge waechst mit jeder Phase (CSA G8, .loom G9, Inference G8a).

use cce_library::guard::{run_guard, GuardOutcome};
use cce_library::registry::Registry;

#[test]
fn regression_guard_all_witnesses() {
    let reg = Registry::seed().expect("Saat-Bibliothek");
    match run_guard(&reg) {
        GuardOutcome::Green { witnesses_checked } => {
            assert!(witnesses_checked >= 5);
        }
        GuardOutcome::Broken(breaks) => {
            panic!("REGRESSIONSWAECHTER ROT — Bau blockiert: {breaks:#?}")
        }
    }
}
