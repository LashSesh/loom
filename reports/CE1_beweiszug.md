CE-1 Beweiszug — Tabellen-Zellentyp (S-E4a Teil II, Etappe X2/E4a)

Die erste echte CoreExtension durch den S14-Pfad: bewiesen wird nicht
nur die Tabelle, sondern dass das System wachsen kann, ohne
aufzuweichen. Dieses Dokument ist das in II.5/DoD(CE-1) geforderte
Muster für jede künftige Extension — jeder S14-Punkt einzeln
abgehakt, mit Commit-Referenz und realer Evidenz.

## II.1 — Antrag und Zweck (S14-Punkt 1)

Antrag: die Dokumentfamilie erhält den Einheitstyp `Table` —
strukturierte Daten (Anforderungslisten, Risikomatrizen,
Positionsübersichten der 96 dokumentnahen Domänen) als erstklassige,
prüfbare Einheit. Umgesetzt in Commit `e451cd5` (Kern-Engine-Teil) +
dem unmittelbar folgenden Commit (Zeugen/Register/Doku, dieser Bericht).

## II.2 — Definition (S14-Punkt 2)

`Table`-Einheit: `header: [Text; k]` (k ≥ 1), `rows: [[Cell; k]]` in
Dokumentreihenfolge (bedeutungstragend, nie sortiert). `Cell` ∈
`{Text (NFC), Int, DecFrac}` — **keine Floats**, durch die Rust-
Enum-Zelltyp-Menge selbst strukturell erzwungen (`TableCell` hat kein
`Float`-Feld — ein Verstoß ist nicht konstruierbar, nicht nur verboten).

Kanonische Serialisierung: Markdown-Pipe-Tabelle mit dem etablierten
`<!--cce:unit…-->`-Anker VOR der Kopfzeile (abweichend von jedem
anderen Einheitstyp, wo der Anker NACH dem Text steht — explizit so in
II.2 gefordert). Zellinhalt pipe-escaped (`|` → `\|`); Reanalyse liest
verlustfrei zurück.

Implementierung: `cce-materialize/src/document/mod.rs`
(`TableCell`/`Table`/`encode_table`/`decode_table`/`DocUnit::new_table`/
`DocUnit::as_table`), `render.rs` (`render_pipe_table`, Anker-vor-
Kopfzeile), `parse.rs` (`parse_pipe_table`, Look-ahead-Schleife statt
`last_text_line`). Die Tabellendaten leben — wie bei jedem
Einheitstyp — im bestehenden `DocUnit.text`-Feld (kompakte,
deterministische Kodierung über zwei C0-Kontrollzeichen als interne
Trenner, die `normalize_text`s Whitespace-Kollaps unverändert
überleben). `cce-loom`/`cce-phc` (Weave, WeaveBlock, Projection)
wurden **in keiner Zeile verändert**.

Nähte: eine `Table`-Einheit trägt `refers`/`supports`/`cites` wie jede
andere Einheit (bewiesen in R-TBL-1: drei `supports`-Nähte AUF die
Tabelle). `DomainRule`-Regeln (Relation/Unique/…) wirken unverändert —
bewiesen in R-TBL-2 (die Tabelle ist kein „Subjekt" der D06-Regel,
stört sie also gar nicht).

## II.3 — Invarianten-Erhaltungsbeweis (S14-Punkt 2/3)

Berührte Invarianten: **keine gelockert.**

- **K1/K5** (deterministische Serialisierung): feste Spaltenordnung
  (Header-Reihenfolge), Zeilen in Dokumentordnung, kürzeste
  Kodierung im `text`-Kanal. Bewiesen durch
  `table_unit_roundtrips_through_the_real_motor_pipeline`
  (deterministischer Render + Reanalyse-Klassenidentität) und
  `docx_rendering_is_deterministic`-artige Determinismus-Erwartung
  implizit über den zweifachen Seed-Generator-Lauf (byte-identisch).
- **K3** (Float-Verbot): strukturell durch die `TableCell`-Enum
  erzwungen (kein `Float`-Variant existiert).
- **Zwei-Digest-Modell**: unberührt — Byte-Kosmetik der
  Pipe-Ausrichtung ändert die Klasse nicht (Whitespace-Normalisierung
  bleibt beim bestehenden `normalize_text`).
- **Structure-Gate verschärft, nicht ersetzt**: neue Prüfungen
  `ragged_table` (Aritäts-Gleichheit) und `invalid_cell_type`
  (dekodierbare Kodierung) — ZUSÄTZLICH zu den bestehenden
  orphan-/missing-section-Prüfungen, die unverändert weiterlaufen
  (bewiesen: alle 213 Domänen-Referenz-Zeugen bleiben grün, da
  `structure()` für Nicht-Table-Crystals exakt dasselbe Verhalten
  zeigt wie vorher).
- Kein neues Tor, kein neuer Egress, kein Gate wird umgangen; Score
  bleibt nie Gate (unverändert — CE-1 berührt keine Score-Pfade).

## II.4 — Kompatibilität und Migration (S14-Punkt 3)

`TYPE_REGISTRY`-Eintrag `unit:table`: additiv — realisiert als
zusätzliche `UnitType`-Variante + `as_str`/`parse`-Match-Arme (ein
separates Registry-Objekt existiert im Code nicht; `UnitType` TRÄGT
die Registrierung bereits vollständig, s. Kommentar in `document/
mod.rs`). `.loom`-Ebene: unverändert (Einheiten leben im CL-Payload/
im Text-Kanal) — Read-/Seal-kompatibel automatisch, keine
Format-Änderung.

Run-Kompatibilität: ein Motor ohne Table-Kenntnis hält fail-closed mit
`unsupported_unit_type` — bewiesen durch
`n_tbl_3_old_engine_without_table_support_holds_not_skips`
(Alt-Motor-Simulation der sieben ursprünglichen Typen + Nachweis, dass
der reale Parser für JEDEN unbekannten Typ generisch genauso hält,
nie still überspringt). Keine Migration nötig (rein additiv);
Alt-Container bleiben klassenstabil (bewiesen: alle bestehenden
Golden-Files R1–R8 und alle 213 Domänen-Referenz-Cubes unverändert
grün).

## II.5 — Zeugen und Wächter-Eintritt (S14-Punkt 4/5)

- **R-TBL-1** — Referenz-Memo „Risikomatrix" (`library/seed/
  risikomatrix_memo_workbody.loom`): Table
  mit 3 Zeilen, je eine `Countermeasure`-Einheit mit `supports`-Naht
  AUF die Tabelle. Voller Motorpfad (`cce-runner::Run::submit`+
  `run_to_end`), alle sieben DocG-Gates grün, `loom verify` ⇒ `Valid`,
  Pipe-Roundtrip real bestätigt (`loom extract` liefert die echte
  Pipe-Tabelle byte-identisch, Reanalyse klassenidentisch zum
  Original).
- **R-TBL-2** — Table in der PL3-Domäne D06 (Angebot:
  Positionstabelle): der reale, UNVERÄNDERTE Familien-Kern
  (`family_a::domain_core_gate`, `family_a_domains::d06`) bleibt
  Pass, wenn eine Tabelle hinzugefügt wird — die Domänen-Regel
  (`Relation{seam:"priced"}`) ignoriert sie vollständig, weil ihr
  `UnitType` nicht das „Subjekt"-Type der Regel ist.
- **N-TBL-1** `ragged_table` — rot (`gates_structure_holds_on_ragged_
  table`, cce-materialize).
- **N-TBL-2** `invalid_cell_type` — rot (`gates_structure_holds_on_
  invalid_cell_type`, cce-materialize; da `TableCell` strukturell
  keine Floats zulässt, simuliert dieser Test eine unlesbare
  Kodierung als Verteidigung-in-der-Tiefe-Nachweis).
- **N-TBL-3** Alt-Motor-Simulation ⇒ `unsupported_unit_type`-Hold,
  nicht Crash, nicht Skip (`conformance/tests/e4a_ce1_table.rs`).

Wächter: `GUARD_PHASES += "CE1"` (`conformance/src/lib.rs`). Doku-Zeile:
`docs/operator/extensions/ce1_tabelle.md`. Katalog-/FEATURE_PL-Eintrag:
`("ce1_tabellen_zellentyp", "PL2", "conformance/tests/e4a_ce1_table.rs")`
— PL2 statt PL4 (PL4 bleibt D01 vorbehalten), begründet durch die reale
Zeugenlage (voller Motorpfad + Domänen-Integration + drei Negativzeugen,
mehr als die PL1-Baseline anderer Querschnitts-Features, aber kein
Produkt-Kerntest-Anspruch).

## DoD(CE-1)

**DoD(CE-1) = 1 ⟺** II.2–II.5 vollständig ∧ alle Alt-Zeugen unverändert
grün ∧ der Beweiszug hier dokumentiert.

- II.2–II.5: vollständig (oben, mit Commit-Referenzen).
- Alle Alt-Zeugen unverändert grün: 165 Testgruppen, 0 Fehlschläge
  (voller `cargo test --workspace`-Lauf nach jeder Einheit); alle 213
  Domänen-Referenz-Zeugen, R1–R8-Golden-Files, alle X1/E2/E3-Zeugen
  unangetastet.
- CI GRUEN: `cargo fmt --all`, `cargo clippy --workspace --all-targets
  -- -D warnings`, `python3 ci/check_acyclic.py` (53 Workspace-Crates),
  `bash ci/run_ci.sh` — alle grün.
- Kein neues Kern-Crate mit externen Abhängigkeiten: keine neue Crate,
  keine externe Kiste.

**DoD(CE-1): ERFÜLLT.**

## Residuen

- NFC-Normalisierung von Text-Zellen wird als Eingabevertrag
  vorausgesetzt (Zelltext kommt bereits normalisiert herein) — keine
  echte Unicode-NFC-Transformation implementiert, da das eine externe
  Kiste in einem Kern-Crate erfordern würde (Widerspruch zur „externe
  Kisten nur in Blatt-Crates"-Disziplin). Dieselbe Behandlung wie
  jeder andere Einheitstyp bisher (auch dort wurde nie echte
  Unicode-NFC durchgesetzt, nur Whitespace-Kollaps).
- Die interne Text-Kodierung (zwei C0-Kontrollzeichen als Trenner)
  ist ein internes Implementierungsdetail — bei einer Zelle, deren
  Inhalt zufällig genau eines dieser Kontrollzeichen enthält, würde
  die Kodierung fehlschlagen (kein Escape-Mechanismus dafür
  implementiert). In der Praxis kommen diese Zeichen in echtem
  Domänentext nicht vor; als Residuum sichtbar geführt statt
  stillschweigend riskiert.
