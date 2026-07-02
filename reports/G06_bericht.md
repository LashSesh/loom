Phase: G6 Persistenz & Bibliothek (der Wächter wird scharf)
Eingang erfüllt: ja (G5-Gate grün, reports/G05_bericht.md)
Gebaut:
- `cce-store`: CAS-Vertrag mit MemoryCas + FsCas (write-once, dedupliziert,
  Digest-Prüfung beim Lesen — S9.1/S9.7); adressierte Objektarten Crystals,
  PhaseBlocks, MEF, Frontiers, Replay-Packs, ResidueReports, Artefakte,
  Bibliotheks-Assets (S9-A1/A4); Ref-Schicht mit Compare-and-Set, sichtbaren
  Ref-Konflikten und expliziter Operator-Auflösung (S9.6, nie still);
  Re-Exporte verify_ledger + verify_hdag_projection (ein Mechanismus).
- `cce-library`: Registry mit Eintritts-Selbstvalidierung (S8.2 — Referenz
  muss real schließen, Negative muss real mit ERWARTETEM Grund abgelehnt
  werden, „zahnlose" Negative + falsche Gründe werden abgewiesen);
  Asset-Typen inkl. S8-A1-Erweiterungen (Crystals, PhaseBlocks,
  WorkbodyBlueprints, DomainNorms, BridgeNorm-Port); explizite Stilllegung;
  Saat-Bibliothek (Drei-Risiken-Memo + Negativ-Cubes);
  **Regressionswächter `run_guard`** (S8.3): prüft bei jedem Lauf JEDES
  Asset erneut gegen den Motor.
- CI-Bindung: `conformance/tests/guard.rs` — der Wächter läuft ab jetzt in
  jedem `cargo test` (CI-blockierend); Zeugenmenge wächst mit G7/G8/G8a/G9.
Ausgangs-Gate:
- CAS-Round-trip aller Objekttypen = grün (`cas_roundtrip_all_object_kinds`,
  Memory + Dateisystem, Dedup nachgewiesen)
- Ref-Konflikt sichtbar + operator-auflösbar = grün
  (`ref_conflict_visible_operator_resolvable`, zusätzlich Unit-Test in refs.rs)
- **Wächter blockiert nachweislich einen absichtlich eingeschleusten Bruch**
  = grün (`guard_blocks_injected_break` — drei Linien: Eintritts-Abweisung,
  Bestands-Drift einer Referenz ⇒ BAU ROT, entschärfter Negativ-Cube ⇒ BAU ROT)
- Dokument-Zeugen: alle Referenzen grün, alle Negativen rot = grün
  (`seed_library_witnesses_green_and_red`, `toothless_negative_cube_rejected`)
- CI vollständig grün (inkl. neuem Wächter-Einstieg im Conformance-Harness).
Residuen dieser Phase:
- S9-R2 (CAS-Backend-Wahl) umgesetzt als Dateisystem-Layout objects/<hex>;
  eingebettete DB bleibt sichtbare Alternative (kein Vertragsbruch).
Abweichungen von der Spec: keine.
