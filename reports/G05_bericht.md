Phase: G5 Runner & Orchestrierung
Eingang erfüllt: ja (G4-Gate grün, reports/G04_bericht.md)
Gebaut:
- `cce-runner`: Lauf-Lebenszyklus (S5.1) über dem geschlossenen Dokument-Pfad
  encode→project→loom→materialize→reanalyze→equivalent mit content-adressierten
  Checkpoints an jeder Stufengrenze (S5.2, `checkpoint.rs`); Pause/Resume als
  deterministische Fortsetzung (S5.3); HITL-Entscheidungen als aufgezeichnete,
  RD-gebundene Eingänge {gate, entscheidung, operator, RD-Ref, Evidence-Ref}
  (S5-A2) — Replay spielt die Aufzeichnung ab, fragt NIE neu; harte Gates
  besitzen keinen Pausierpfad (S5.4, baulich); drei Ausführungsformen (S5-A3):
  HyperDAG-Topologie+CommitProjection, Multi-Ratchet-Kaskade, generische
  Pipeline (`exec.rs`); Reise-PhaseLadder p₀…p₅ als Red(SCALE-1) (S2-A1,
  `journey.rs`); ClosureReport wertet ClosedCCE (`closure_report.rs`).
Ausgangs-Gate:
- Replay-Identität (INV-10) inkl. HITL = grün (`tests/g5_gate.rs::replay_identity_including_hitl` —
  Erstlauf zeichnet auf, Replay ohne Provider liefert dieselbe Klasse)
- Pause/Resume klassenerhaltend = grün (`pause_resume_preserves_class` — Ergebnisklasse und
  Checkpoint-Folge identisch)
- harte Gates pausieren nie = grün (`hard_gates_never_pause` — HardGateNotPausable)
- Reise-PhaseLadder p₀…p₅ als Red(SCALE-1) schließbar = grün
  (`journey_phase_ladder_closes_as_red_scale1`; Fehl-Reise = sichtbarer Hold, S2-A2)
- drei Ausführungsformen deterministisch = grün (`three_execution_forms_deterministic`)
- CI vollständig grün.
Residuen dieser Phase: keine neuen.
Abweichungen von der Spec: keine.
