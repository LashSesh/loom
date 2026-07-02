//! cockpit-core — Sitzungs-/Workspace-Zustand (S3.1.2): einziger
//! Vermittler zwischen GUI, KI-Kanzel, Motor und Persistenz.
//! Enthaelt KEINE Motor-Logik — der Motor wird ueber den EnginePort
//! (Trait) gerufen. Grundfesten (S3.0): der Motor entscheidet, das
//! Cockpit zeigt; die Kanzel ist gebundener Erzaehler; die KI steht
//! ausserhalb des Abschlusspfads.

pub mod engine;
pub mod kanzel;
pub mod persistence;
pub mod state;
pub mod views;
