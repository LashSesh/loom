# CCE — DETAIL-SPEZIFIKATION S1: DOKUMENT-INSTANZIIERUNGSVERTRAG

**Zweite D-Spec.** Die konkrete Grammatik der **Dokument-Domäne** (der ersten vollständig geschlossenen Domäne, R-Plan-2) — und zugleich die **Vorlage der Adapter-Parität**: die kanonische Teileliste, die jede weitere Domäne identisch erfüllen muss (Modellbaukasten-Prinzip, Systemlandkarte §5).

**Status:** Detail-Spezifikation v1.0, auf Bauverfassungs-Tiefe. Baut auf: BAUVERFASSUNG (Motor), SYSTEMLANDKARTE (Ring A/S1), S3 (Cockpit, das diese Domäne bedient).

---

## S1.0 — Einordnung & doppelter Auftrag

Diese Spec hat **zwei** Aufgaben zugleich:

1. **Konkretisierung.** Sie füllt für die Dokument-Domäne die vier Fragen des Instanziierungsvertrags (Systemlandkarte §4): *Was ist der Wunsch? Was ist der Crystal? Was ist das Artefakt? Was heißt domänen-native Verwendung?*
2. **Vorlage.** Sie definiert die **kanonische Teileliste** (`DomainAdapter`, §S1.8), die **jede** Domäne — Graph, Software-Modul, Mathe/Struktur — identisch bereitstellen muss. Dokument ist die **Referenz-Implementierung** dieser Teileliste. Das `check_adapter_parity`-Gate (Systemlandkarte §5) misst jede Domäne gegen genau diese Liste.

Leitsatz:

> *Ein Dokument-Artefakt ist erst dann fertig, wenn seine Re-Analyse den Dokument-Crystal bis auf kanonische Inhaltsklasse zurückgibt — nicht bis auf Byte-Gleichheit der gerenderten Datei.*

Das ist die domänen-konkrete Form der Abschlussformel und der Kern, warum die Dokument-Domäne schließt (§S1.6).

---

## S1.1 — Das Dokument-Objektmodell (Instanziierung der Motor-Objekte)

Die abstrakten Motor-Objekte (Bauverfassung Teil 3) werden für Dokumente konkret so instanziiert:

| Motor-Objekt (abstrakt) | Dokument-Instanz (konkret) | Bedeutung |
|--------------------------|----------------------------|-----------|
| `CanonicalState` | **DocState** | kanonisierter Dokumentinhalt |
| `Cell` / `TripolarFiber` | **DocUnit** `(B, B⁻, seam)` | eine semantische Einheit: Abschnitt, Aussage, Beleg, Risiko, Gegenmaßnahme, Definition, Schritt |
| `Separator` / `Seam` / Mandorla | **SupportSeam** | eine Beziehung über eine Grenze: Aussage↔Beleg, Risiko↔Gegenmaßnahme, Abschnitt⊃Unterabschnitt |
| `BoundaryContract` | **DocBoundary** | Vertrag des Dokuments als Ganzes: deckt Themenmenge T, hat die geforderten Top-Abschnitte |
| `Horizon` | **DocHorizon** | Vollständigkeitsziel: jedes geforderte Thema abgedeckt, jede Aussage gestützt |
| `CounterHorizon` | **DocCounterHorizon** | Pathologien: unbelegte Aussage, unabgedecktes Thema, Widerspruch, verwaiste Einheit, Selbstbestätigung |
| `Constraint` (K) | **DocConstraint** | z. B. `no_score_fields`, `required_sections`, `covers(T)`, `every(Risk).has(Countermeasure)`, `max_length` |
| `Crystal` | **DocCrystal** | der kristallisierte Dokumentkern: Einheiten + Nähte + Grenze + Horizont + Randbedingungen |
| `Weave` / `Thread` | **DocWeave** | die konkrete Abschnitts-/Block-Struktur mit Nähten, vom Webstuhl (LOOM) erzeugt |
| `Artifact` | **DocArtifact** | die gerenderte Datei (`.docx`/`.md`/optional `.pdf`) |
| `Residue` (`B⁻x`) | **DocResidue** | z. B. unabgedecktes Thema, unbelegte Einheit, verwaiste Einheit (§S1.5) |

**DocUnit** trägt wie jede Motor-Zelle die tripolare Faser `(B, B⁻, seam)`: den Inhalt (`B`), das Residuum (`B⁻`, stets ausgewiesen), und die Naht zum Nachbarn. Das Closure-Gesetz `B⁻x=0` heißt für eine DocUnit: sie ist inhaltlich abgeschlossen und ihr Residuum (fehlende Stütze, fehlende Abdeckung) ist leer **und** ausgewiesen.

---

## S1.2 — Die Wunsch-Grammatik der Dokument-Domäne

Die Kanzel (S3) bildet einen natürlichsprachlichen Wunsch in die **Wunsch-Normalform** `W=(X,H,K,G,Res,Π,τ,Replay,Goal,Materialize)` (Bauverfassung Teil 3), konkret befüllt für Dokumente:

- **X (Träger):** die Menge der DocUnits (typisiert: `Section | Claim | Support | Risk | Countermeasure | Definition | Step`).
- **H (Horizont):** das Vollständigkeitsziel — welche Themen/Aussagen abgedeckt/gestützt sein müssen.
- **K (Randbedingungen):** die DocConstraints. Kanonische Typen:
  - `required_sections([...])` — geforderte Top-Abschnitte,
  - `covers(TopicSet)` — Abdeckungsziel,
  - `every(UnitType).has(SeamType)` — z. B. jedes `Risk` hat eine `Countermeasure`-Naht,
  - `no_score_fields` — keine Bewertungszahl als Entscheidungsfeld (Domänenform von V1),
  - `max_length` / `min_length` — Umfangsgrenzen,
  - `ordering(neutral|meaningful)` — ob Reihenfolge semantisch zählt (wichtig für Kanonisierung, §S1.6).
- **G (Gates):** die Dokument-Gates (§S1.4).
- **Res (Residuen-Erwartung):** welche DocResidues auftreten dürfen und welche `blocking` sind.
- **Goal / Materialize:** Zielformat (`.docx` primär, `.md` sekundär).

Eine Wunsch-Grammatik-Prüfung validiert, dass der geformte DocCrystal wohlgeformt gegen `phc.schema.json` + dieses Vokabular ist. Rot ⇒ zurück in der Wunsch-Fläche (S3.2.1) mit Grund.

---

## S1.3 — Der geschlossene Pfad, konkret für Dokumente

Der Motorpfad `encode → project → loom → materialize → reanalyze → equivalent` (Bauverfassung Teil 4) instanziiert sich so:

| Schritt | Dokument-konkret | Ergebnis |
|---------|------------------|----------|
| **encode** | DocCrystal → PHC-Paket (Dokument-Profil) | `PHCPackage` |
| **project** | Projektion auf die Materialisierungsachse (Zielformat, Gate-Scope) | `Projektion` |
| **loom** | Webstuhl erzeugt aus der Projektion die Abschnitts-/Block-Struktur mit Nähten (radiale Spindel, Nullanker markiert, nie durchlaufen) | `DocWeave` |
| **materialize** | DocWeave → gerenderte Datei (`.docx`/`.md`), **nur kosmetisches Format hinzugefügt, kein neuer semantischer Inhalt** | `DocArtifact` (Bytes) |
| **reanalyze** | gerenderte Datei zurücklesen in einen DocCrystal (Einheiten, Nähte, Struktur extrahieren) | `DocCrystal'` |
| **equivalent** | kanonische Inhaltsklasse von `DocCrystal'` == die von `DocCrystal`? | `bool` (das ≃) |

Der kritische Vertrag an `materialize` und `reanalyze`:
- **materialize erfindet nichts** — es fügt nur kosmetisches Format hinzu (Schrift, Abstände, Seitenumbrüche), das die Kanonisierung wegstreift. Jeder semantische Inhalt im Artefakt, der nicht im Crystal war, ist ein Residuum (`invented_semantic`) und bricht die Äquivalenz.
- **reanalyze verliert nichts** — es gewinnt den semantischen Kern vollständig zurück. Jeder im Crystal vorhandene, in der Materialisierung verlorene Inhalt ist ein Residuum (`semantic_loss`) und bricht die Äquivalenz.

Damit ist der Round-Trip die maschinelle Prüfung, dass die Materialisierung **treu** war.

---

## S1.4 — Dokument-Gates (auf G1–G7 aufgesetzt)

Die sieben Pflicht-Gates (Bauverfassung Teil 7, §7.4) gelten unverändert; darüber liegen die domänenspezifischen Dokument-Gates:

| Gate | Dokument-Bedeutung | Residuum bei Rot |
|------|--------------------|-------------------|
| **DocG-Coverage** | jedes Thema im Abdeckungsziel `covers(T)` ist von ≥1 DocUnit abgedeckt | `uncovered_topic` |
| **DocG-Support** (Domänenform von G3 Naht-Gate) | jede DocUnit, die Stütze fordert (z. B. `Claim`, `Risk`), hat eine `SupportSeam` zur stützenden Einheit | `unsupported_unit` |
| **DocG-NoScore** (Domänenform von V1) | kein Bewertungs-/Score-Feld wirkt als Entscheidung, wo `no_score_fields` gilt | `forbidden_score_field` (ablehnend) |
| **DocG-Structure** | wohlgeformte Abschnitts-Verschachtelung, keine verwaiste Einheit | `orphan_unit` / `malformed_structure` |
| **DocG-NonContradiction** | keine zwei Einheiten widersprechen sich direkt (Gegenhorizont-Prüfung) | `contradiction` |
| **DocG-Seam** (Domänenform von V4) | kein Übergang zwischen Einheiten über eine Grenze ohne `SupportSeam` | `boundary_crossing_without_seam` |
| **DocG-RoundTrip** (Domänen-Kerntest) | `equivalent(reanalyze(materialize(C)), C)` | `semantic_loss` / `invented_semantic` |

Alle Gates sind **fail-closed** und boolesch mit Begründung. Kein Gate ist eine Kennzahl (V1). Harte Gates sind im Cockpit nicht übersteuerbar (S3.5); wo der Korpus Ermessen zulässt (z. B. Grenzfall der Abdeckung), erscheint ein Human-in-the-Loop-Gate mit vollem Kontext.

---

## S1.5 — Dokument-Residuen-Vokabular (jedes sichtbar)

Das vollständige `B⁻x`-Vokabular der Dokument-Domäne (Bauverfassung P4/V2 — jedes stets ausgewiesen, nie still absorbiert):

- `uncovered_topic` — ein gefordertes Thema ist nicht abgedeckt.
- `unsupported_unit` — eine stützungspflichtige Einheit hat keine Stütz-Naht.
- `orphan_unit` — eine Einheit hängt an keiner Struktur.
- `contradiction` — zwei Einheiten widersprechen sich.
- `boundary_crossing_without_seam` — Übergang ohne Naht.
- `forbidden_score_field` — verbotenes Score-Feld wirkt als Entscheidung.
- `semantic_loss` — Materialisierung hat semantischen Inhalt verloren (Round-Trip bricht).
- `invented_semantic` — Materialisierung hat Inhalt erfunden (Round-Trip bricht).

**DocCounterHorizon** (Gegenhorizont, Bauverfassung PHC §13.1): die geordnete Menge dieser Pathologien plus Nullmodelle (z. B. „ein Dokument, das alle Themen nennt, aber keines stützt" — verhindert Schließung durch reine Nennung ohne Substanz). Jedes Residuum trägt `severity ∈ {info, warning, blocking}`; `blocking` verhindert sichtbar `sealed`/`closed` und wird in der Prüf-Fläche (S3.2.3) als offener Eintrag gezeigt, ein leeres Residuum explizit als „geschlossen (∅)".

---

## S1.6 — Kanonisierung & Äquivalenz für Dokumente (das ≃)

Das Herz der Domäne. `equivalent(c1, c2)` ist `canonicalize(c1) == canonicalize(c2)` — Gleichheit **kanonischer Inhaltsklassen**, nie Byte-Gleichheit (Bauverfassung P2).

**Die Kanonisierung `canonicalize(DocCrystal)` streift weg (kosmetisch, irrelevant):**
- exakte Whitespace-/Zeilenumbruch-Details, Schriftart, Schriftgröße, Farben,
- Seitenumbrüche und reine Layout-Elemente,
- Reihenfolge dort, wo `ordering(neutral)` gilt (z. B. eine Aufzählung gleichrangiger Risiken wird als Menge kanonisiert),
- Formulierungsvarianten auf reiner Oberflächenebene, soweit sie denselben semantischen Kern tragen (kontrolliert, konservativ — im Zweifel **nicht** gleichsetzen).

**Die Kanonisierung bewahrt (semantisch, wesentlich):**
- welche DocUnits existieren, mit Typ und Rolle,
- die Stütz-/Enthaltens-Nähte zwischen ihnen (der Naht-Graph),
- die Abdeckung der Themenmenge,
- die Reihenfolge dort, wo `ordering(meaningful)` gilt (z. B. Schritte einer Anleitung),
- die Grenzvertrags-Erfüllung (DocBoundary).

Zwei Dokumente sind ≃ genau dann, wenn ihre kanonischen Inhaltsklassen übereinstimmen: **gleiche Einheiten, gleicher Naht-Graph, gleiche Abdeckung, gleiche bedeutungstragende Reihenfolge** — unabhängig davon, wie die `.docx` kosmetisch aussieht.

**Warum das schließt:** `materialize` fügt nur Kosmetik hinzu (die die Kanonisierung entfernt), `reanalyze` gewinnt den semantischen Kern zurück; also fällt `reanalyze(materialize(C))` in dieselbe kanonische Klasse wie `C`. Bricht das, benennt der DocG-RoundTrip exakt das Residuum (`semantic_loss` oder `invented_semantic`).

*Konfluenz-Hinweis (Bauverfassung INV-9):* Die kanonische Klasse ist reihenfolgeunabhängig mod ≡σ; zwei zulässige Webstuhl-Reihenfolgen erzeugen dieselbe Klasse.

---

## S1.7 — Materialisiertes Artefakt & domänen-native Verwendung

- **Primärformat:** `.docx` — im gebauten Rust-System über eine docx-schreibende Bibliothek (z. B. `docx-rs`); exakte Wahl ist sichtbares Residuum (§S1.10-R1).
- **Sekundärformat:** `.md` (Klartext, verlustarm, gut versionierbar).
- **Tertiär (später):** `.pdf` (zurückgestellt, §S1.10-R3).
- **Speicherung:** Die Artefakt-Datei wird **content-adressiert nach ihrem Byte-Digest** im Workspace abgelegt (S3.7). Die **Äquivalenz/Replay-Identität** hingegen läuft über den **Digest der kanonischen Inhaltsklasse** — beide getrennt geführt: der Byte-Digest identifiziert die Datei, die Inhaltsklasse identifiziert die Bedeutung.
- **Native Verwendung:** `.docx` öffnet in Word/LibreOffice; `.md` in jedem Editor/Vorschau. Die Artefakt-Fläche (S3.2.4) bietet „Öffnen" und „Exportieren".

---

## S1.8 — Adapter-Parität: die kanonische Teileliste (`DomainAdapter`)

**Dies ist die Vorlage für alle Domänen.** Jede Domäne implementiert **exakt** diesen Vertrag — dieselbe, vollständige Teileliste (Ihr Modellbaukasten: identische Schraubenzahl). Dokument ist die Referenz-Implementierung. Fehlt einem Adapter auch nur ein Teil, ist `check_adapter_parity` **rot** und das Produkt gilt als unfertig.

```rust
// Kanonische Teileliste — jede Domäne MUSS alle Punkte bereitstellen.
trait DomainAdapter {
    type Crystal;   type Weave;   type Artifact;   type Residue;

    // 1. Wunsch-Grammatik + Validierung (Träger-/Randbedingungs-Vokabular der Domäne)
    fn wish_schema(&self) -> Schema;
    fn validate_wish(&self, c: &Self::Crystal) -> Result<(), Vec<Self::Residue>>;

    // 2. Instanziierung auf die abstrakten Motor-Objekte
    fn to_canonical(&self, c: &Self::Crystal) -> CanonicalState;

    // 3. encode: Crystal -> PHC-Paket (Domänen-Profil)
    fn encode(&self, c: &Self::Crystal) -> PHCPackage;

    // 4. loom: Projektion -> Gewebe (radiale Spindel, Nullanker markiert)
    fn loom(&self, p: &Projection) -> Self::Weave;

    // 5. materialize: Gewebe -> Artefakt (nur Kosmetik, kein neuer Inhalt)
    fn materialize(&self, w: &Self::Weave) -> Self::Artifact;

    // 6. reanalyze: Artefakt -> Crystal' (Collect-Pfad, verlustfrei)
    fn reanalyze(&self, a: &Self::Artifact) -> Self::Crystal;

    // 7. Kanonisierung + Äquivalenz (das ≃ der Domäne)
    fn canonicalize(&self, c: &Self::Crystal) -> CanonicalClass;
    fn equivalent(&self, c1: &Self::Crystal, c2: &Self::Crystal) -> bool;

    // 8. Domänen-Gates (auf G1–G7 aufgesetzt), boolesch + begründet, fail-closed
    fn domain_gates(&self) -> Vec<Gate>;

    // 9. Domänen-Residuen-Vokabular + Gegenhorizont
    fn residue_vocabulary(&self) -> Vec<ResidueKind>;
    fn counter_horizon(&self, c: &Self::Crystal) -> CounterHorizon;

    // 10. native Ausgabe/Verwendung
    fn native_open(&self, a: &Self::Artifact) -> OpenAction;
    fn export_formats(&self) -> Vec<Format>;

    // 11. Test-Assets: ein perfekter Referenz-Cube + Negativ-Cubes (je Verbot einer)
    fn reference_cube(&self) -> Self::Crystal;
    fn negative_cubes(&self) -> Vec<(Self::Crystal, ResidueKind /* erwartete Ablehnung */)>;
}
```

`check_adapter_parity` prüft maschinell: Jede Domäne stellt alle 11 Punkte bereit, mit demselben Satz an Pflicht-Gate-Anschlüssen, Residuenfeldern und Ausgabewegen. **Keine Domäne wird halb gebaut.**

---

## S1.9 — Test-Assets der Dokument-Domäne

**Referenz-Cube (perfekt, erfüllt `ClosedCCE(C)=1`):** das Drei-Risiken-Memo — drei `Risk`-Einheiten, je mit `Countermeasure` über `SupportSeam`; `covers(T)` mit T = die drei Risiken; `no_score_fields`; alle Dokument-Gates grün; Residuum „geschlossen (∅)"; DocG-RoundTrip grün. Liegt in `reference-cubes/document/`.

**Negativ-Cubes (je Verbot einer, MUSS abgelehnt werden), in `negative-cubes/document/`:**
- Memo mit einem **unabgedeckten** vierten Risiko-Thema → `uncovered_topic` (DocG-Coverage rot).
- Memo mit einem Risiko **ohne** Gegenmaßnahme → `unsupported_unit` (DocG-Support rot).
- Memo, das eine **Bewertungszahl** als Entscheidungskriterium führt, wo `no_score_fields` gilt → `forbidden_score_field` (DocG-NoScore rot).
- Memo, dessen Materialisierung einen Absatz **erfindet**, der nicht im Crystal war → `invented_semantic` (DocG-RoundTrip rot).
- Memo mit einem Übergang zwischen Einheiten **ohne Naht** → `boundary_crossing_without_seam` (DocG-Seam rot).

Jeder Negativ-Cube ist ein rot-getesteter Nachweis, dass die Engine den Fall dauerhaft ablehnt (Bauverfassung §8.4).

---

## S1.10 — Sichtbare Residuen dieser Spezifikation

Kein stilles Loch:

- **R1 (docx-Bibliothek).** `docx-rs` vs. eigene Schreib-Schicht — offen; ändert die Verträge dieser Spec nicht.
- **R2 (Ontologie-Tiefe).** Diese Spec fixiert die Einheitstypen `Section | Claim | Support | Risk | Countermeasure | Definition | Step`. Eine reichere Ontologie kann später **additiv** andocken, ohne den Vertrag zu brechen (neue Typen sind neue `UnitType`-Werte, kein Kernumbau).
- **R3 (PDF-Ausgabe).** Als Tertiärformat zurückgestellt.
- **R4 (Kanonisierungs-Grenzfälle).** Die Kernregeln (§S1.6) stehen; einzelne Grenzfälle der Formulierungs-Gleichsetzung sind konservativ zu behandeln (im Zweifel nicht gleichsetzen) und beim Auftreten als Test zu fixieren.

---

## S1.11 — Abnahme (DoD dieser Ebene)

```
DoD(S1) = 1  ⟺
    DomainAdapter<Document> implementiert die vollständige Teileliste (§S1.8, Adapter-Parität)
  ∧ Operator kann via S3-Cockpit  Dokument-Wunsch → geschlossenes .docx/.md,
        sichtbar geprüft (Gates/Residuen/Ledger/Replay/Abschlussbeweis), replaybar
  ∧ Dokument-Kerntest grün:
        equivalent(reanalyze(materialize(loom(project(encode(C))))), C)   für den Referenz-Cube
  ∧ alle Dokument-Gates (Coverage, Support, NoScore, Structure, NonContradiction, Seam, RoundTrip)
        definiert, boolesch, fail-closed, getestet
  ∧ Dokument-Residuen-Vokabular vollständig, jedes sichtbar; Gegenhorizont vorhanden
  ∧ Kanonisierung/Äquivalenz (§S1.6) implementiert; Gleichheit = kanonische Inhaltsklasse, nie Byte
  ∧ Referenz-Cube (perfekt) + Negativ-Cubes (je Verbot einer) vorhanden und korrekt (rot)getestet
  ∧ check_adapter_parity grün (Document erfüllt die kanonische Teileliste)
  ∧ Engine-DoD(cce)=1 und DoD(S3)=1 bleiben unberührt
```

---

## S1.12 — Anschluss

Diese Spec macht die Dokument-Domäne vollständig instanziiert und liefert die Adapter-Parität-Vorlage für alle weiteren Domänen. Der geschlossene Pfad ist domänen-konkret; das ≃ ist präzise als kanonische Inhaltsklasse definiert; Gates, Residuen und Test-Cubes stehen.

**Nächste D-Spec (Systemlandkarte §6):** **S4 — Wunscherfassung & Autorenschicht** — der Elicitation-Fluss, mit dem die Kanzel einen natürlichsprachlichen Wunsch in einen wohlgeformten DocCrystal (diese Grammatik) überführt: geführte Rückfragen, Validierung, Bestätigung. Danach S6 (Inspektions-Modell), S5, S8, S9, S13, S2, S11, S12 — bis `ProduktDoD=1`.

*Ende der Detail-Spezifikation S1.*
