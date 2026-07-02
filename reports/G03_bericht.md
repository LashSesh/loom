Phase: G3 Transport, Weben, Materialisieren + Motor-Kerntest
Eingang erfüllt: ja (G2-Gate grün, reports/G02_bericht.md)
Gebaut:
- `cce-core::wheel_window`: WheelWindow (Radfenster=Nadelapertur, Identität I-7) als EIN
  geteilter Typ/Code-Pfad für cce-observe (Apertur) und cce-loom (Nadel) — nie dupliziert.
- `cce-phc`: PHC-Paket (M,T,A,C,W,P,G,R,L,E) mit content-adressiertem codec_id, PHC-CANON-0.1,
  Zelladressen phc://…, Loader V0–V9 (jede Phase isolierter Fehlpunkt), Projektionskalkül
  (No-Horizon-Leakage inkl. Exclude-Gate), Profile CORE/LOOM/MERKABA.
- `cce-loom`: RadialSpindle (Z0 markiert, NULLPOINT_TRAVERSAL=forbidden, Mandorla-Naht),
  Weave (teilgeordnet, kanonisierbar), LoomWorkcell (keine Prompt-Regression),
  loom_generate mit repair_or_reweave (gate_failed nie verschmolzen).
- `cce-observe`: dyadisches Substrat, QLOGIC-Register + PoR-Gate, PIO-Optik (Überschuss
  sichtbar), TAT-Collect-Kette (Embed→Mark→Respond→Horizon→Triangulate→Gate→Crystalize),
  reanalyze/observe → MatrixCrystal.
- `cce-merkaba`: Organ-Kalkül (∘_Can), Gesamtumlauf F mit Emissionsbedingung
  (E∈L ∧ Replay ∧ Pass(G_E) ∧ Res≤ε).
- `cce-materialize`: typstarker DomainAdapter-Vertrag (11 Punkte) + check_adapter_parity;
  DocumentAdapter (D01) vollständig: Wunsch-Grammatik, Validierung, encode, loom,
  materialize (Markdown, nur Kosmetik + verlustfreie Struktur-Anker), reanalyze-Parser,
  Kanonisierung/≃ (S1.6: Whitespace-Kollaps, neutral-Sortierung, Naht-Graph), 7 Dokument-
  Gates, 8-teiliges Residuen-Vokabular, Gegenhorizont, Referenz-Cube „Drei-Risiken-Memo",
  Negativ-Cubes.
Ausgangs-Gate:
- **Motor-Kerntest ≃ = GRÜN**: `conformance/tests/closure_roundtrip.rs::
  motor_kerntest_reanalyze_materialize_loom_project_phc_is_identity` — inkl.
  q(Obs(A))=q(C) über den echten Collect-Sweep (MatrixCrystal-Klasse).
- 7 Dokument-Gates grün am Referenz-Cube = grün (`seven_document_gates_green_on_reference`)
- rot an Negativ-Cubes mit ERWARTETEM Residuum = grün (`negative_cubes_red_with_expected_residue`,
  `tampered_artifacts_break_roundtrip_with_named_residue` — invented_semantic & semantic_loss)
- Zwei-Digest-Trennung = grün (`two_digest_separation`: Kosmetik ändert Byte-, nie Klassen-Digest)
- Adapter-Parität Dokument 11/11 = grün (`document_adapter_parity_green`)
- LOOM-Abnahmekatalog (10) = grün (crates/cce-loom/tests/loom_catalog.rs L1–L10)
- TAT P1–P7 + QLOGIC-/DZ-/PIO-Rollen = grün (crates/cce-observe/tests/tat_catalog.rs)
- CI vollständig grün (103 Testsuiten; fmt, clippy -D warnings, check_acyclic, Integrität)
Residuen dieser Phase:
- R-Agent-5 (residuen.md): Nummernzuordnung V0–V9 / LOOM-10 / P1–P7 rekonstruiert.
- S1.10-R1 (docx-Bibliothek) bleibt offen: Export vorerst .md (verlustarm, nativ öffnbar);
  .docx als sichtbarer Reifepfad.
Abweichungen von der Spec: keine.
