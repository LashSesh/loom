Track-Einheit: E/P8 — Skalen: SCALE-2 „Dokumentenmappe" (nach W4)
Eingang: W4 grün, CI grün.
Gebaut:
- `crates/cce-materialize/src/scale2_folder.rs` — die erste höhere
  Skala über SCALE-1: `DocFolder` (Zellen = D01-Memo-Workbodies,
  referenziert über ihre Inhaltsklasse; Nähte = Verweis-/Reihenfolge-
  Konsistenz). `materialize_index` (Mappen-Index als Markdown mit
  verlustfreien Ankern) + `parse_index` (Reanalyse) + `folder_equivalent`
  (kanonische Klasse). `folder_seams_valid`: Nähte zeigen auf existierende
  Einträge, `precedes`-Graph azyklisch.
- Zeugen `conformance/tests/scale2_msc.rs`:
  - scale2_adapter_parity_8_of_8 (voller ScaleAdapter-Vertrag; SCALE-1
    unverändert gültig),
  - red2_kerntest_three_memos… (3 Memos → Mappe geschlossen → Reanalyze
    klassenidentisch),
  - multi_scale_closure_1_to_2_green (RedCube je Skala 0/1/2 geschlossen ⇒
    MultiScaleClosure(1→2) = Ok; Promote(SCALE-2)→3).
- Katalog/FEATURE_PL: „scale2_dokumentenmappe" auf **PL2** (Struktur +
  Zeugen; SCALE-2-Reife/PL3+ ist S15-R1 nutzungs-/review-gebunden, offen).
Ausgangs-Gate:
- Red(2)-Kerntest grün · MSC(1→2)-Zeuge grün · Scale-2-Adapter 8/8 · CI
  GRUEN · GUARD += „P8-SCALE2".
- Keine Änderung an SCALE-1-Pfaden: der eine Wächter läuft vollständig
  grün (alle bisherigen Zeugen + Produkt-Kerntest unberührt).
Residuen:
- SCALE-2-Reife über PL2 hinaus (S15-R1): verlangt Nutzungs-/Review-
  Evidenz; die Mappe referenziert Memos über ihre Klasse (Klassenebene
  verlustfrei), die gebündelte .loom-Vollmaterialisierung (Memo-Bytes im
  Container) ist ein Folgeschritt.
- SCALE-3-Entwurf (Wellenplan) bleibt offen.
Abweichungen: keine; spec/ unangetastet.
