# 14 — SPEC-LIEFERUNG FÜR X2: `cites`-Naht-Norm (S-E2a) und CoreExtension „Tabellen-Zellentyp" (S-E4a)

**Zwei normative Stücke in einem Dokument** — append-only-Overlay über spec/ (Autoritätsordnung wie etabliert: Fundament > dieses Dokument > Karte 13 > Root-Docs > spec-Overlays > S-Specs). Teil I macht Ring E2 baubar, Teil II macht Ring E4a baubar. Ring E3 (HBM auf Eigenkorpus) braucht keine neue Norm — HBM ist vollspezifiziert; der Eigenkorpus ist gewöhnlicher Ingest.

---

# TEIL I — S-E2a: Die `cites`-Naht (Cross-Workbody-Referenz)

## I.1 Zweck und Semantik

`cites` ist die **externe** Schwester der internen Nähte: Eine Einheit eines Workbody stützt sich beweisbar auf einen **anderen zertifizierten Container** — adressiert über dessen Inhaltsklasse, nie über Pfade. Anker: `cites(unit_id → target)` mit `target = (target_core_root [Multihash], target_unit_ref?, cite_kind ∈ {supports, refers, derives})`. Bedeutung: *supports* = Stützungsanspruch (closure-relevant), *refers* = Bezug (informativ), *derives* = die Einheit ist Ableitung aus der Zieleinheit (closure-relevant, Replay-Input).

## I.2 Auflösung: der CitationResolver-Port (entkoppelt E2 von E4)

Auflösung läuft über einen Port `CitationResolver { resolve(core_root) -> Option<VerifiedTarget> }` mit zunächst **einer** Bau-Implementierung: `SeedResolver` (durchsucht `library/seed/` + eine explizit konfigurierte Pfadliste; verifiziert Kandidaten L0–L2 vor Annahme; Digest-Vergleich gegen `target_core_root` ist Pflicht). Die Klassen-Registry aus E4c wird später eine **zweite** Implementierung desselben Ports — E2 hängt nicht an E4. `VerifiedTarget` liefert: Verdikt, Manifest-Kern, Einheiten-Index (für `target_unit_ref`-Prüfung).

## I.3 CitationGate (fail-closed) und Residuen

Prüfkette je `cites`-Naht: (1) Ziel auflösbar → sonst Residuum `unresolved_citation`; (2) aufgelöster Container `Valid` (L0–L2) → sonst `citation_target_invalid`; (3) `core_root` byte-identisch → sonst `citation_class_mismatch`; (4) falls `target_unit_ref`: Einheit existiert im Ziel → sonst `citation_unit_missing`; (5) der `supports`/`derives`-Teilgraph über Workbody-Grenzen ist **azyklisch** → sonst `circular_support_citation` (`refers` darf frei zyklisch sein — Bezug ist kein Stützungszirkel).

**Verdikt-Semantik (wichtig):** Ein Container mit unaufgelösten `cites` bleibt **transportierbar**: `verify` ⇒ `valid_with_residues` (sichtbar), niemals reject — wie eine fehlende Bibliothek, nicht wie Korruption. Aber **Closure/Commit** eines Workbody mit `supports`/`derives`-cites verlangt Gate-Pass: `Close(x)=1 ⇒ CitationGate(x)=Pass` — keine geschlossene Arbeit auf ungeprüfter Fremdstütze.

## I.4 Format-Einbettung (minor-additiv, kein neues Segment)

Die `cites`-Nähte leben als Seam-Daten im CL_SUBSTRATE (Seam-Typ `cites` mit target-Tripel, dCBOR nach LOOM-CANON-1). Zusätzlich Pflichtfeld im MANIFEST: `external_citations[]` = deduplizierte, sortierte Liste aller `target_core_root` — damit zeigt jeder Viewer die Fremdabhängigkeiten in der Streaming-Inspektion ohne Vollparse. **L2-Konsistenzregel:** `external_citations` ≡ Menge der tatsächlichen cites-Ziele (Abweichung = `manifest_citation_mismatch`, reject). Kompatibilität: additives Manifest-Feld + neuer Seam-Typ = Read-/Seal-kompatibel; alte Reader zeigen den Seam-Typ als unbekannt-sichtbar.

## I.5 Replay und Reanalyse

Zitierte Klassen sind **Inputs**: Der RunDescriptor eines Laufs mit `supports`/`derives`-cites listet die `target_core_root`s als Input-Digests. Replay verlangt dieselben Ziele (Resolver darf anders sein, Klassen nicht). Reanalyse einer zitierenden Arbeit ohne verfügbare Ziele ⇒ `valid_with_residues` + Closure-Hold, klassenstabil dokumentiert.

## I.6 SCALE-3 „Projektraum" (Kurznorm)

Red(3)-Körper: **Zellen** = SCALE-2-Mappen, Quellen-Workbodies (CSA-getragen), Blueprint-Kristalle; **Nähte** = `contains` (Mappe/Quelle gehört zum Projekt), `cites` (I.1), `precedes` (Reihenfolge, azyklisch); **Schließung**: alle enthaltenen Körper geschlossen ∧ alle `supports`/`derives`-cites Gate-Pass ∧ Projekt-Index materialisierbar und reanalysierbar (≃). MSC(2→3) nach S15-Formel. ScaleAdapter(3)-Parität 8/8 Pflicht.

## I.7 Zeugen (Wächter-Eintritt mit E2)

**R-CIT-1** Memo `cites`→Welt-Crystal (`supports`, auflösbar via SeedResolver) — geschlossen, alle Gates grün; *der erste beweisbare Wissens-Verbund*. **R-CIT-2** cites mit `target_unit_ref` auf die Definitionseinheit `d1` des Welt-Crystals. **R-CIT-3** Red(3)-Referenzprojekt (2 Mappen + Welt-Crystal + 1 Blueprint), MSC(1→2→3) grün. **N-CIT-1** unaufgelöstes Ziel ⇒ `valid_with_residues` + Closure-Hold. **N-CIT-2** manipulierter target_core_root ⇒ `citation_class_mismatch`, Commit-Reject. **N-CIT-3** zirkuläre supports-cites ⇒ rot. **N-CIT-4** Ziel-Container beschädigt ⇒ `citation_target_invalid`. **N-CIT-5** Manifest-Liste ≠ Ist ⇒ `manifest_citation_mismatch`, reject.

---

# TEIL II — S-E4a: CoreExtension CE-1 „Tabellen-Zellentyp" (der S14-Beweiszug, ausgefüllt)

## II.1 Antrag und Zweck (S14-Punkt 1)

Die Dokumentfamilie erhält den Einheitstyp **`Table`** — strukturierte Daten als erstklassige, prüfbare Einheit (Anforderungslisten, Risikomatrizen, Positionsübersichten der 96 dokumentnahen Domänen). Erste echte Nutzung des S14-Pfads: bewiesen wird nicht nur die Tabelle, sondern **dass das System wachsen kann, ohne aufzuweichen**.

## II.2 Definition

`Table`-Einheit: `header: [Text; k]` (k ≥ 1, Spaltenarität), `rows: [[Cell; k]]` (Zeilen in **Dokumentreihenfolge** — bedeutungstragend, wird nie sortiert), `Cell ∈ {Text (NFC), Int, DecFrac}` — **keine Floats** (K3 geerbt). Kanonische Serialisierung: Markdown-Pipe-Tabelle mit dem etablierten `<!--cce:unit…-->`-Anker vor der Kopfzeile; Zellinhalt pipe-escaped; Reanalyse parst sie verlustfrei zurück (Roundtrip-Klasse). Nähte: `Table` kann `refers`/`supports`/`cites` tragen wie jede Einheit; DomainRules (Relation/Unique/…) wirken unverändert auf Table-Einheiten.

## II.3 Invarianten-Erhaltungsbeweis (S14-Punkt 2)

Berührte Invarianten: **keine gelockert.** K1/K5: Serialisierung deterministisch (feste Spaltenordnung = Header-Ordnung, Zeilen = Dokumentordnung, NFC, kürzeste dCBOR-Kodierung im CL-Payload). K3: Float-Verbot durch Zelltyp-Menge erzwungen. Zwei-Digest-Modell unberührt (Byte-Kosmetik der Pipe-Ausrichtung ändert die Klasse nicht — Whitespace-Normalisierung im Parser). Structure-Gate wird **verschärft, nicht ersetzt**: neue Prüfung Aritäts-Gleichheit aller Zeilen ⇒ Residuum `ragged_table`; Zelltyp-Verstoß ⇒ `invalid_cell_type`. Kein neues Tor, kein neuer Egress, kein Gate wird umgangen; Score bleibt nie Gate.

## II.4 Kompatibilität und Migration (S14-Punkt 3)

TYPE_REGISTRY erhält den Eintrag `unit:table` (additiv ⇒ minor). `.loom`-Ebene: unverändert (Einheiten leben im CL-Payload) — Read-/Seal-kompatibel automatisch. **Run-Kompatibilität:** ein Motor ohne Table-Kenntnis endet am unbekannten Einheitstyp mit begründetem Hold `unsupported_unit_type` (fail-closed, nie stilles Überspringen). Keine Migration nötig (rein additiv); Alt-Container bleiben klassenstabil.

## II.5 Zeugen (S14-Punkt 4) und Wächter-Eintritt (S14-Punkt 5)

**R-TBL-1** Referenz-Memo „Risikomatrix" (Table mit 3 Zeilen, je `supports`-Naht auf eine Gegenmaßnahmen-Einheit) — voller Motorpfad, alle Gates grün, Kerntest ≃ inkl. Pipe-Roundtrip. **R-TBL-2** Table in einer PL3-Domäne (D06 Angebot: Positionstabelle) — Familien-Kern unverändert. **N-TBL-1** ragged_table rot. **N-TBL-2** Float-Zelle ⇒ `invalid_cell_type` rot. **N-TBL-3** Alt-Motor-Simulation ⇒ `unsupported_unit_type`-Hold (nicht Crash, nicht Skip). Wächter: `GUARD += "CE1"`; Doku-Zeile `docs/operator/extensions/ce1_tabelle.md`; Katalog-/FEATURE_PL-Eintrag `ce1_tabellen_zellentyp` mit PL nach Zeugenlage. **DoD(CE-1) = 1 ⟺** II.2–II.5 vollständig ∧ alle Alt-Zeugen unverändert grün ∧ der Beweiszug als `reports/CE1_beweiszug.md` dokumentiert (das Muster für jede künftige Extension).

---

**Bau-Reihenfolge X2 (für den Agenten):** E2 (I.1–I.7, inkl. R-CIT-3/SCALE-3) → E3 (HBM auf Eigenkorpus: die 213 Familien-Referenz-Cubes + Seeds als Ingest; erste zertifizierte Blueprints aus Eigenarbeit; R-13-Lock unangetastet, Negativzeuge bleibt rot) → E4 (erst CE-1 nach Teil II, dann loom-sdk-Fassade + wasm32-Viewer, dann Klassen-Registry als `.loom`-Katalog-Workbody und als **zweite** CitationResolver-Impl). Je Ring: eigener Bericht, eigenes Gate, Alt-Zeugen unverändert, Status/Register nachgeführt. E5 bleibt gesperrt bis S-E5.
