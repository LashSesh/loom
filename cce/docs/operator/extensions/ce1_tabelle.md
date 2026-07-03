# CE-1 — Tabellen-Zellentyp (`unit:table`)

Die erste CoreExtension durch den S14-Pfad (S-E4a Teil II, Etappe
X2/E4a der Ökosystem-Expansionskarte). Die Dokumentfamilie trägt jetzt
einen achten Einheitstyp: `Table` — strukturierte Daten (Header + Zeilen
typisierter Zellen) als erstklassige, prüfbare Einheit.

## Was neu ist

Eine `Table`-Einheit trägt `header: [Text; k]` (Spaltenarität k ≥ 1) und
`rows: [[Cell; k]]` in Dokumentreihenfolge (bedeutungstragend, wird nie
sortiert). `Cell` ∈ `{Text (NFC-Eingabevertrag), Int, DecFrac}` —
**keine Floats**: die Zelltyp-Menge selbst erzwingt das strukturell.

Kanonische Darstellung: eine Markdown-Pipe-Tabelle. Anders als bei
jedem anderen Einheitstyp steht der `<!--cce:unit ...-->`-Anker VOR der
Kopfzeile (nicht danach). Zellinhalt ist pipe-escaped (`|` → `\|`);
Reanalyse liest die Tabelle verlustfrei zurück (Roundtrip-Klasse).

Eine `Table`-Einheit kann `refers`/`supports`/`cites`-Nähte tragen wie
jede andere Einheit. Bestehende Domänen-Regeln (Relation/Unique/…)
wirken unverändert — eine Tabelle ist normalerweise kein „Subjekt" einer
Domänenregel (ihr `UnitType` ist `Table`, nicht z. B. `Definition`),
stört den Familien-Kern also nicht.

## Wo es lebt

Die Tabellendaten liegen — wie bei jedem Einheitstyp — im bestehenden
`DocUnit.text`-Feld, hier deterministisch kodiert (zwei interne
Kontrollzeichen als Trenner, überleben die bestehende
Whitespace-Normalisierung unverändert). Das hält `cce-loom`/`cce-phc`
(Weave, Projection, WeaveBlock) **vollständig unangetastet** — kein
neues Tor, keine neue Projektionsform, dieselbe Faser wie jede andere
Einheit.

## Verschärfte Prüfung (DocG-Structure)

`DocG-Structure` prüft jetzt zusätzlich: `ragged_table` (alle Zeilen
einer Tabelle müssen dieselbe Spaltenanzahl wie die Kopfzeile tragen)
und `invalid_cell_type` (die Tabellen-Kodierung muss dekodierbar sein).
Beides ist eine Verschärfung, keine Ersetzung — die bestehenden
Prüfungen (verwaiste Einheit, fehlender Abschnitt) laufen unverändert
weiter.

## Kompatibilität

Additiv, minor: ein Motor ohne Table-Kenntnis hält fail-closed mit
`unsupported_unit_type` (der bestehende Reanalyse-Pfad tut das für
JEDEN unbekannten Einheitstyp) — kein stilles Überspringen. Keine
Migration nötig; Alt-Container bleiben klassenstabil.

## Zeugen

- **R-TBL-1** — Referenz-Memo „Risikomatrix" (`library/seed/
  risikomatrix_memo_workbody.loom`): eine 3-Zeilen-Tabelle, drei
  Gegenmaßnahmen-Einheiten je mit `supports`-Naht auf die Tabelle,
  voller Motorpfad (cce-runner), alle sieben DocG-Gates grün,
  Pipe-Roundtrip real bestätigt.
- **R-TBL-2** — eine Table-Einheit in der echten PL3-Domäne D06
  (Angebot): der Familien-Kern (`domain_core_gate`, `family_a_domains::
  d06`) bleibt dabei unverändert grün.
- **N-TBL-1/2** — `ragged_table`/`invalid_cell_type` fail-closed
  (`cce-materialize::document::ce1_table_tests`).
- **N-TBL-3** — Alt-Motor-Simulation: `unsupported_unit_type`-Hold statt
  stillem Überspringen (`conformance/tests/e4a_ce1_table.rs`).

Beweiszug: `reports/CE1_beweiszug.md`.
