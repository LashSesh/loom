Phase: G7 HBM Mining-Chassis
Eingang erfüllt: ja (G6-Gate grün, reports/G06_bericht.md)
Gebaut:
- `cce-hbm`: Pipeline-Phasen 0–9 (Ingest → Facets → Gate_A → Cube/HDAG →
  Skeleton/JT mit Baumweite → Kandidaten C1–C6 → Score-Ranking mit θ_D als
  reiner VORAUSWAHL → ExclusionGate → LayerExpansion → CrystalFinalization
  → Registry/Replay via Ledger-Commit); Facet-Typvokabular (10 Typen, S1-A2);
  Gate_A = ValidTypes ∧ ResolvableRefs ∧ Testable ∧ NonTautological ∧
  NonContradictory; neutralisierte Operatorik (ExclusionGate mit
  exclusion_fail(...)-Fehlersprache, LayerExpansion mit Budget,
  CrystalFinalization = CCC-Kristallbedingung, keine neue Theorie);
  Score als Ranking mit RD-Gewichten (Integer-Promille, R-8) +
  score_as_gate_attempt-Abweisung; FixpointCalibration (ψ,ρ,ω in Promille) +
  PhaseSwitchController mit RD-geloggten Wechseln (HBM-10);
  EphemeralMiningCells (scope/budget/ttl/trace, Dissolution konsumiert die
  Zelle — nur Trace/Evidence/Kristalle persistieren);
  **BoundedOperatorSpecialization implementiert, aber fail-closed
  DEAKTIVIERT (R-13)**: Aktivierungspfad nur via CapabilityLock (neu in
  cce-core::capability, S13-A1), Versuch ohne Lock ⇒ unbounded_cloning;
  unter offenem Lock gelten Scope⊆Eltern, endliches Budget, |descendants|≤B_O.
Ausgangs-Gate:
- HBM-01…HBM-20 als Testkatalog = grün (crates/cce-hbm/tests/hbm_catalog.rs,
  20 Tests; verankerte Nummern HBM-10/12/16/17/18/20 auf Spec-Position)
- Gate-Dominanz: Materialize ohne Gate=Pass unmöglich = grün (HBM-15)
- Negativ: score_as_gate_attempt rot = grün (HBM-19); unbounded_cloning rot
  = grün (HBM-13, plus Schranken-Test HBM-14)
- End-to-End: kleiner Korpus ⇒ zertifizierter Blueprint-Kristall, Replay
  klassenidentisch = grün (HBM-20, zusätzlich HBM-11 Determinismus)
- CI vollständig grün (inkl. Regressionswächter).
Residuen dieser Phase:
- R-13 bleibt offen wie spezifiziert (Klonung implementiert + deaktiviert).
- R-8 (Score-Gewichte/Schwellen) als RD-Parameter umgesetzt; Kalibrierung
  bleibt sichtbarer Betriebsschritt.
Abweichungen von der Spec: keine (HBM-Katalognummern gemäß R-Agent-5-Disziplin
rekonstruiert, verankerte Nummern auf Position).
