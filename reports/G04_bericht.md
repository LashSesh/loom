Phase: G4 Spiral-Kinematik
Eingang erfüllt: ja (G3-Gate grün inkl. Motor-Kerntest, reports/G03_bericht.md)
Gebaut:
- `cce-spiral`: Spiraladresse (s,p,k,j,θ,r) mit θ in Milliturns (floatfrei);
  Expansionsoperator E_{λ,α} (Drift innen erlaubt); WrapProjection W_s mit den
  4 Pflichten (Support-Boundedness, Seam-/Residue-/Replay-Preservation),
  externe Drift annulliert + sichtbar; AreaClass als Support-/Budgetklassen-
  Digest; DispersionProfile Dyadic (Default, R-7) / Golden / Rational /
  Adaptive — RD-Deklarationspflicht; Ratchets Cell/Phase/Ring/Scale + Nexus-
  Typ-Stub (R-1b), Step nur unter Gate∧Evidence∧sichtbarem Residuum, Lock
  irreversibel, Rotation frei (F6); Kaskadenregel; die 8 Spiral-Gates
  (WrapGate, AreaClassGate, DriftGate, PhaseRatchetGate, ScaleRatchetGate,
  DispersionProfileGate, ApertureSectorGate, SpiralReplayGate); die 12
  Spiral-Residuen (S15.14); Blue/Red-Schließungslogik (Formel 2) mit
  Close(Blue)⇒PhaseBlock-Vorstufe und Close(Red)⇒Promote; S15-Strukturen
  (DoD-Matrix f): ScaleAdapter + check_scale_adapter_parity + SCALE-1-Instanz,
  WorkbenchCapsule mit Dissolution (nur Trace/Evidence/Kristalle persistieren),
  Multicube, MultiScaleClosure (strukturell; Skalenreife ≥2 = R-10, nicht behauptet).
Ausgangs-Gate:
- alle 8 Gates implementiert + getestet = grün (tests/spiral_catalog.rs g1..g8)
- alle 12 Residuen erzeugbar + sichtbar = grün (`all_12_residues_constructible_and_visible`)
- Negativ-Zeugen rot = grün: `external_drift_detected` (g1), `wrap_support_violation` (g1),
  `ratchet_lock_without_evidence` (g4), `fibonacci_dogma_import` / undeklariertes Profil (g6)
- Kaskadentest: höheres Lock ohne untere Locks/Residuen unmöglich = grün
  (`cascade_lock_impossible_without_lower`)
- Zusätzlich (Rebase-Handoff §8.3): Blue/Red-Schließungslogik grün (g5),
  Ratchet-Irreversibilität + freie Rotation (F6) grün.
- CI vollständig grün (fmt, clippy -D warnings, check_acyclic, alle Suiten).
Residuen dieser Phase: keine neuen (R-7 als Default-Empfehlung umgesetzt:
DyadicProfile ist Default UND muss dennoch RD-deklariert werden — konservativ).
Abweichungen von der Spec: keine.
