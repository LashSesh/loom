Bericht: P6-Härtung (Register-Einträge #16 und #25 aus
07_TOTAL_RESIDUE_TO_CLOSURE_REGISTER.md)
Eingang erfüllt: ja — Verifikationslauf T0 vollständig grün
(reports/verifikation_T0.md); Auswahl gemäß Auftraggeber-Freigabe:
klein, lokal, architekturfrei, ohne Netz-/Modellzugriff, ohne
Build-Host-Abhängigkeit, ohne Domänen-/Skalenauswahl.
Gebaut:
- **#16 CDDL-Extraktion (LC-R3, „mechanisch"):** `cce/loom/schemas/` —
  15 Dateien (common, header, segtab, manifest, canon-desc,
  gate-reports, residue, csa-nsb, ledger, replay-manifest,
  provider-manifest, inference-profile, inference-trace,
  candidate-outputs, tool-profile) + README. Wortgetreu aus den
  normativen Feldverträgen in loom-verify/loom-codec extrahiert; die
  Overlay-Verbotsklassen (autostart/activate_on_open/N15/N16,
  is_commit, enabled/implicit_grant, Score-Keys) sind als Kommentar-
  Verbote je Schema geführt. Der Rust-Code bleibt die Autorität —
  neuer Wächter-Test `loom-conformance/tests/schemas.rs` hält beide
  Formen deckungsgleich (alle 15 Dateien vorhanden + nicht leer; die
  13 Manifest-Pflichtfelder wortgleich; Verbotskeys benannt).
- **#25 Fuzz-Harness:** `loom-conformance/tests/fuzz_smoke.rs` —
  deterministisch (splitmix64-seeded, P9-konform: keine unseeded
  Randomness, reproduzierbar in CI), toolchain-frei (kein nightly,
  kein cargo-fuzz-Install nötig). Fünf Ziele über die gesamte
  Fremddatei-Leserfläche: loom_canon::decode, Frame::decode,
  Preamble/Footer::decode, decode_sealed+verify (inkl. Degenerate:
  JSON-Pseudocontainer, Nullbytes), plus 400 Mutationen je
  Golden-R-Datei (R1–R8). Mutationsformen: Byteflips (1–8),
  Truncation, Extension. Eigenschaft: **kein Panic — nur getypte
  Fehler/Verdikte** (jede Panik = benannter Parser-Härtungsbefund).
  Gesamt ≈20 800 Mutationen pro CI-Lauf, Laufzeit ~0,15 s.
- **#25 Threat-Model:** `cce/docs/security/threat_model.md` (2 Seiten):
  5 Schutzgüter, 5 Vertrauensgrenzen (T1 Fremddatei, T2 Quelle,
  T3 Modell, T4 Werkzeug, T5 GUI/Kanzel), 6 Angreifermodelle je mit
  den EXISTIERENDEN, testbewachten Gegenmaßnahmen (Zeugen namentlich)
  — und 5 Restrisiken ausdrücklich offen mit Register-Nummer
  (#14 Signaturen, #18 zstd, #19 NFC, egui-Lieferkette,
  Coverage-Fuzzing als Betriebsschritt). Keine Sicherheitsbehauptung
  über die Beweislage hinaus.
Ausgangs-Gate:
- Neue Wächter-Tests grün: fuzz_smoke (5/5), schemas (1/1).
- CI vollständig grün nach Integration (exit 0, „CI: GRUEN") — der
  Fuzz-Harness und die Schema-Wache laufen ab jetzt in jedem Lauf des
  einen Wächters.
Residuen dieser Phase:
- #25 bleibt im Register TEIL-geschlossen: der deterministische
  CI-Harness + Threat-Model sind geliefert; coverage-geleitetes
  cargo-fuzz (nightly/libFuzzer) bleibt als Betriebsschritt offen —
  im Threat-Model §4 sichtbar geführt.
- #16 geschlossen im Sinne des Registers („CDDL aus Code extrahieren,
  mechanisch"); formale CDDL-Tool-Validierung (z. B. cddl-Tool) wäre
  eine externe Abhängigkeit und bleibt Betriebsoption.
Abweichungen: keine Architektur-/Spec-Änderung; spec/ unangetastet.
Nicht begonnen (brauchen Freigabe): P3–P5, P7, P8 sowie alle Punkte
mit Netz-/Modell-/Build-Host-Bedarf (WO-2..5, OAuth, zstd, blake3,
Ed25519-Bindung, Domänen-/Skalen-Wellen).
