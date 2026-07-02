Phase: G1 Kern-Substrat (Motor-Objekte, Kanonisierung, Gates)
Eingang erfüllt: ja (G0-Gate grün, reports/G00_bericht.md)
Gebaut:
- `cce-core`: floatfreies kanonisches Wertemodell (`value.rs`), SHA-256 eigenständig (`signature.rs`),
  Can/σ (`canonical.rs`, Can∘Can=Can), tripolare Faser B/B⁻ (`reflection.rs`), Closure-Gesetz +
  Zertifikat (`closure.rs`), Residuen stets sichtbar inkl. „geschlossen (∅)" (`residue.rs`),
  Gates G1–G7 fail-closed + GateReport boolesch/begründet + Score-Abweisung (`gate.rs`),
  append-only Ledger + verify_ledger (`ledger.rs`), RunDescriptor {crystal_digest, decisions,
  params, seed} + SeededRng (`replay.rs`), Objektmodell inkl. Wunsch-Normalform W mit
  BoundarySpec/CompletionSpace/ScaleTarget aus S4-A1 (`objects.rs`), BCIK-Zentralgesetz
  (`bcik.rs`), BCIK-Maschine mit Subject-Reduction/Progress (`machine.rs`), H/F-Import-Gate +
  Claim-Schranke (`hf_import.rs`), QSNA mit a-priori-Schranke, rational (`qsna.rs`),
  Pfadinvarianz-Prüfer (`pathinv.rs`), check_adapter_parity-Gerüst (`adapter_parity.rs`).
- `cce-lattice`: Cube G0–G3, Propagationshülle (extensiv/monoton/idempotent, lfp, Residuen
  sichtbar), Feasible-Enumeration, Chordalität/MCS/PEO/min-fill/Junction Tree/RIP/Separatoren,
  Chameleon-Projektion (No-Horizon-Leakage), Entfaltungsplan, Collapse-Zertifikat, Export-Funktor.
- `cce-ccc`: NodeFiber/Restriktion, Verklebung (eindeutig, Seam-konsistent), ZWEI Sweeps über
  EINER Junction-Tree-Struktur (P6/INV-7), Bi-Temporalität T1/T2/n0, Kristall-Protokoll.
- `cce-crystal`: Crystal als Klasse [Can(c)]_σ (nur via Protokoll konstruierbar), MatrixCrystal,
  Monolith (append-only Commit), Quotient q / equivalent (≃), Zwei-Digest-Grundlage.
Ausgangs-Gate:
- INV-1 = grün (`canonical::tests::can_idempotent`)
- INV-2 = grün (`signature::tests::signature_is_stable_under_canonicalization`,
  `machine::tests::subject_reduction_preserves_class`)
- INV-3 = grün (`bcik::tests::empty_boundary_is_rejected`, `reflection::tests::fiber_exposes_residual`)
- INV-8 = grün (`machine::tests::{progress_terminates, subject_reduction_preserves_class}`)
- INV-9 = grün (`pathinv::tests::insertion_order_is_confluent`, `c14_replay_confluence`)
- G1–G7 als Typen mit fail-closed Default = grün (`gate::tests::seven_mandatory_gates_fail_closed`)
- V1-Negativtest = grün (`gate::tests::{score_as_gate_is_rejected, numeric_verdict_is_rejected}`)
- check_adapter_parity-Gerüst = grün (`adapter_parity::tests::parity_full_is_green_missing_is_red`)
- Kataloge: CL K1–K18 grün (crates/cce-lattice/tests/k_catalog.rs, 18 Tests),
  CCC C1–C14 grün (crates/cce-ccc/tests/c_catalog.rs, 14 Tests)
- CI vollständig grün (fmt, clippy -D warnings, check_acyclic, 90 Tests)
Residuen dieser Phase:
- R-Agent-2 (reports/residuen.md): K-/C-Katalog-Nummernzuordnung rekonstruiert.
Abweichungen von der Spec:
- Chordalität/PEO/Junction Tree liegen in `cce-lattice` (gemäß 01_MASTER_BUILD G1-Arbeitsliste);
  `cce-ccc` nutzt sie über die Crate-Grenze statt sie zu duplizieren (Bauverfassung Teil 6 nennt
  chordal.rs unter cce-ccc). Eine Struktur, keine Rückkante — INV-11 gewahrt. Begründung:
  01_MASTER_BUILD ist die konsolidierte (jüngere) Bauordnung; Duplikation wäre V5-widrig.
- Kein Float im gesamten Kern: Kennzahlen als i64/Dezimalbruch/Rational (G1-Verbot erfüllt).
