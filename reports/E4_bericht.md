Ring E4 — Erweiterbarkeit & SDK (Ökosystem-Expansionskarte §2/E4,
S-E4a Teil II + Dokument 14 Bau-Reihenfolge)

Eingang: Ring E3 abgeschlossen und angenommen. Spec-Lieferung S-E4a
(`cce-spec-repo/14_CITES_UND_COREEXTENSION_SPEC.md`, Teil II) für CE-1
bereits mit S-E2a zusammen geliefert und in Ring E2 gelesen. Bau-
Reihenfolge laut Dokument 14: „erst CE-1 nach Teil II, dann loom-sdk-
Fassade + wasm32-Viewer, dann Klassen-Registry" — in genau dieser
Reihenfolge umgesetzt, drei Einheiten, vier Commits.

## Einheit a — CE-1 Tabellen-Zellentyp (Commits e451cd5, 50e12d4)

Die erste echte CoreExtension durch den S14-Pfad: `UnitType::Table`
(additiv), `TableCell{Text, Int, DecFrac}` (keine Floats, strukturell
erzwungen), Pipe-Tabellen-Serialisierung mit Anker VOR der Kopfzeile.
Die Tabellendaten leben im bestehenden `DocUnit.text`-Feld (kompakte
Kodierung über zwei C0-Kontrollzeichen) — `cce-loom`/`cce-phc` bleiben
vollständig unangetastet. `DocG-Structure` verschärft (nicht ersetzt)
um `ragged_table`/`invalid_cell_type`.

Zeugen: R-TBL-1 (Referenz-Memo „Risikomatrix", voller Motorpfad, alle
sieben DocG-Gates grün, Pipe-Roundtrip real bestätigt,
`library/seed/risikomatrix_memo_workbody.loom`), R-TBL-2 (Table in der
echten PL3-Domäne D06, Familien-Kern `domain_core_gate`/`d06`
UNVERÄNDERT), N-TBL-1/2 (ragged_table/invalid_cell_type, Unit-Tests),
N-TBL-3 (Alt-Motor-Simulation, `unsupported_unit_type`-Hold). Wächter:
`GUARD_PHASES += "CE1"`, FEATURE_PL-Eintrag `ce1_tabellen_zellentyp`
(PL2), `docs/operator/extensions/ce1_tabelle.md`. Vollständiger
DoD-Beweiszug: `reports/CE1_beweiszug.md` (DoD(CE-1) = ERFÜLLT).

## Einheit b — loom-sdk-Fassade + wasm32-Viewer (Commit f9c2165)

Neues Crate `loom-sdk`: die schmale, stabile Fassade
open/inspect/verify/extract/replay über den bestehenden loom-*-Kern,
motorfrei (Reader-Prinzip, keine cce-*/nexus-*/cockpit-*-Abhängigkeit).
Vorbedingung dafür: `loom-replay` verlor eine ungenutzte `cce-core`-
Abhängigkeit (Cargo.toml, kein Code referenzierte sie) — sonst hätte
sie den wasm-Viewer über die Fassade transitiv „motorful" gemacht.
`replay` vergleicht eine vom Aufrufer bereits reproduzierte Klasse
gegen das deklarierte REPLAY_MANIFEST (`loom_replay::check_replay`,
unverändert) — die Motor-Reproduktion selbst bleibt beim Aufrufer.

`loom-viewer` hängt jetzt nur noch an `loom-sdk` (statt einzeln an
loom-mount/verify/canon/format/codec) und kompiliert zusätzlich nach
`wasm32-unknown-unknown` (`crate-type = ["cdylib","rlib"]`,
`render_views_wasm`, wasm-bindgen NUR in diesem Blatt-Crate).

Exit-Zeuge „wasm-Viewer verifiziert R1/N-Dateien im Headless-Browser-
Test": real ausgeführt (`build_wasm.sh` + `wasm-bindgen-cli` 0.2.126 +
Playwright/Chromium headless) gegen das echte Golden File R1 (Verdikt
`Valid`, alle fünf Ansichten sichtbar) UND eine gezielt beschädigte
Kopie (Byte-Flip → `BadEndMagic`, `reject:`-Präfix) — kein Mock, echte
Browser-Ausführung. Node-Werkzeug (`playwright`) und Build-Ausgabe
(`web/pkg/`) bewusst nicht versioniert, regenerierbar via
`build_wasm.sh` + `npm install`.

## Einheit c — Klassen-Registry (Commit f9144dd)

`loom-cites::registry`: `ClassRegistryEntry{core_root, class, domain,
signatures, path}` + additives `KIND_DOC`-Payload (dieselbe generische
Kind-Wiederverwendung wie `folder_meta`/`doc_meta`, kein neues
Segment-Kind). `RegistryResolver`: die ZWEITE Implementierung des
`CitationResolver`-Ports (I.2) — löst per Katalog-Eintrag auf statt per
Verzeichnis-Scan, verifiziert aber genauso streng (Digest-Gegenprobe +
volles L0-L2; ein Katalog-Eintrag mit falschem Pfad wird abgewiesen).

`build_class_registry()`: katalogisiert die real committeten Seed-
Container, jeder Eintrag aus dem echten MANIFEST des bereits
versiegelten Containers gelesen. Signaturen bewusst leer (kein
aktueller Seed trägt ein SIGNATURE-Segment) statt einer neuen
Kern-Abhängigkeit auf `loom-cli` nur für ein stets leeres Feld. Seed:
`library/seed/class_registry.loom`.

Exit-Zeuge „Registry-Workbody selbst Valid und cites-auflösend": der
Workbody ist `Valid`; die R-CIT-1/2-Zitationsszene aus Ring E2 (Memo
zitiert Welt-Kristall) löst über den `RegistryResolver` GENAUSO grün
auf wie zuvor über den `SeedResolver` — E2 hängt an keiner bestimmten
Resolver-Implementierung.

## Ausgangs-Gate E4 — Prüfung gegen Karte §2/E4 + Dokument 14

**Exit-Zeugen (Karte): „Extension-Beweispaket vollständig + alle
Alt-Zeugen grün · wasm-Viewer verifiziert R1/N-Dateien im Headless-
Browser-Test · Registry-Workbody selbst Valid und cites-auflösend":**
1. Extension-Beweispaket vollständig — grün (`reports/CE1_beweiszug.md`,
   DoD(CE-1) ERFÜLLT).
2. wasm-Viewer verifiziert R1/N-Dateien im Headless-Browser-Test —
   grün, real ausgeführt (Log-Auszug oben).
3. Registry-Workbody selbst Valid und cites-auflösend — grün.

**Alle Alt-Zeugen unverändert:** voller Workspace-Testlauf nach jeder
Einheit und final: 169 Testgruppen, 0 Fehlschläge. Alle 213 Domänen-
Referenz-Zeugen, R1–R8-Golden-Files, alle X1/E2/E3-Zeugen unangetastet.

**CI GRUEN:** `cargo fmt --all`, `cargo clippy --workspace
--all-targets -- -D warnings`, `python3 ci/check_acyclic.py` (54
Workspace-Crates, DAG sauber, Reader-Prinzip weiterhin erfüllt für
`loom-viewer`), `bash ci/run_ci.sh` — alle grün.

**Kein neues Kern-Crate mit externen Abhängigkeiten:** `loom-sdk` trägt
keine externe Kiste; `wasm-bindgen` ausschließlich in `loom-viewer`
(Blatt); `loom-cites::registry` trägt keine neue Abhängigkeit.

**Ausgangs-Gate E4: ERFÜLLT.**

## Residuen

- Keine der aktuell committeten Seed-Container trägt ein SIGNATURE-
  Segment — die Klassen-Registry führt das Feld sichtbar, aber leer.
  Sobald ein signierter Seed existiert, ist die Registry bereit, seine
  Rollen zu katalogisieren.
- NFC-Normalisierung von Table-Text-Zellen bleibt Eingabevertrag (aus
  `reports/CE1_beweiszug.md`, unverändert).
- Die interne Text-Kodierung der Tabellen (zwei C0-Kontrollzeichen)
  hat kein Escape für den (praktisch nicht vorkommenden) Fall, dass
  Zellinhalt selbst eines dieser Zeichen enthält (aus
  `reports/CE1_beweiszug.md`, unverändert).
- `wasm-bindgen-cli` und `playwright` sind lokale Werkzeuge dieser
  Sandbox (installiert via `cargo install`/`npm install`), nicht Teil
  des versionierten Rust-Workspace oder von `ci/run_ci.sh` — der
  Headless-Browser-Zeuge ist ein manuell ausgeführter, dokumentierter
  Nachweis (wie die UX-Klick-Durchläufe in Block 1), kein automatisierter
  CI-Schritt.

## Nicht begonnen (auftragsgemäß)

E5 (L9b Normic Memory) bleibt gesperrt bis S-E5 (Spec-Lieferung steht
laut Karte §4 aus, „ich" liefert sie). Host-Leiste unverändert
gesperrt. `spec/` unangetastet.

Abweichungen: keine.
