# CCE — DETAIL-SPEZIFIKATION S3: COCKPIT & KI-KANZEL

**Erste D-Spec.** Die Bedienschale, die aus dem Motor (Bauverfassung, Teile 0–9) ein bedienbares Fahrzeug macht — der Linchpin von Ring B (Systemlandkarte §2).

**Status:** Detail-Spezifikation v1.0, auf Bauverfassungs-Tiefe. Realisierungsreif nach Klärung der fünf Weichen.

**Verankerte Entscheidungen (Systemlandkarte §7):**
| # | Weiche | Entscheidung |
|---|--------|--------------|
| R-Plan-1 | Cockpit-Form | **Native Desktop-App in reinem Rust** (kein Webview, kein Browser-Tab) |
| R-Plan-2 | Start-Domäne | **Dokument** (erster grüner Kerntest) |
| R-Plan-3 | KI-Kanzel-Reichweite | **Formen/Erklären autonom; jede materielle Aktion nur nach expliziter Bestätigung** |
| R-Plan-4 | Nutzerumfang | **Einzelnutzer zuerst, erweiterbar** (Rollenmodell dockt später ohne Kernänderung an) |
| R-Plan-5 | Persistenz | **Lokal, von Anfang an sync-fähig ausgelegt** |

---

## S3.0 — Einordnung & Leitsatz

Das Cockpit sitzt **auf** dem Motor und **unter** nichts mehr auf der Menschenseite: Es ist die oberste Fläche, an der ein Operator die Maschine bedient. Sein Leitsatz:

> *Der Operator kommt von seinem Wunsch zu einem geschlossenen, geprüften, replaybaren Artefakt — ohne je eine Konsole zu berühren. Der Motor bleibt die Autorität; die KI ist Dolmetscher und Führer, niemals Richter.*

Drei nicht verhandelbare Grundfesten, die diese ganze Spezifikation tragen:

1. **Der Motor entscheidet, das Cockpit zeigt.** Jedes Gate-Urteil, jedes Residuum, jeder Abschluss entsteht im deterministischen Rust-Motor. Das Cockpit **rendert** diese Fakten, es erzeugt sie nie.
2. **Die KI-Kanzel ist ein gebundener Erzähler.** Sie liest Motor-Ausgaben und übersetzt sie in Klartext; sie formt Wünsche in wohlgeformte Kristalle. Sie hat **keinen Schreibpfad** auf Urteile, Residuen oder den Ledger.
3. **Die KI steht außerhalb des Abschlusspfads.** Ein LLM ist nichtdeterministisch; deshalb liegt es **vor** der Bestätigungsgrenze. Der bestätigte Crystal ist der deterministische Eintrittspunkt; die Replay-Garantie hängt nie an der KI.

Diese drei Grundfesten sind die Bedienebenen-Form der Verbotsachse V1–V10 (Bauverfassung §8.4) und der Kern der Nutzungs-Governance (S13).

---

## S3.1 — Architektur des Cockpits

### S3.1.1 Bauart: native Rust-App, kein Webview

Gemäß R-Plan-1 ist das Cockpit eine **echte native Desktop-Anwendung in reinem Rust** — kein Electron, kein Tauri-Webview, kein Browser. Empfehlung für das GUI-Toolkit: **`egui`** (immediate-mode, ausgereift, minimale Abhängigkeiten, native Fenster auf Linux/macOS/Windows, ideal für Inspektions-Dashboards); Alternative **`iced`** (retained, Elm-artig). Die endgültige Toolkit-Wahl ist ein sichtbares Residuum (§S3.10-R1); die Spezifikation ist bewusst toolkit-agnostisch gehalten und beschreibt **Verträge und Verhalten**, nicht Pixel.

Ein Codepfad, eine Sprache (Rust) vom Motor bis zur Oberfläche. Der Operator startet die App wie jedes native Programm (Fenster, kein Terminal).

### S3.1.2 Komponenten und Grenzen

```
┌──────────────────────────────────────────────────────────────┐
│  COCKPIT (native Rust-App)                                     │
│                                                                │
│   ┌──────────────┐     liest/rendert      ┌────────────────┐   │
│   │  GUI (egui)  │ ◀───────────────────── │  CockpitCore   │   │
│   │  4 Flächen   │ ──────────────────────▶│  (Session +    │   │
│   └──────────────┘   Operator-Eingaben    │   Workspace-   │   │
│          ▲                                 │   Zustand)     │   │
│          │ nur Klartext                    └───────┬────────┘   │
│   ┌──────┴───────┐   Vorschläge (read-only)        │           │
│   │  KI-KANZEL   │ ──────────────────────▶         │           │
│   │ (LLM, außer- │   NUR lesen + erzählen           │           │
│   │  halb Fix(R))│ ◀── Motor-Ausgaben (read-only)   │           │
│   └──────────────┘                                  │           │
│                                                     │           │
│                          EnginePort (trait)  ◀──────┘           │
│                                │                                │
│                    ┌───────────┴────────────┐                   │
│                    │  PersistenceAdapter     │  (lokal,         │
│                    │  (content-addressed,    │   sync-fähig)    │
│                    │   PHC-Portable Ledger)  │                  │
│                    └─────────────────────────┘                  │
└────────────────────────────────┼──────────────────────────────┘
                                  │  deklarierte Ports (Bauverfassung Teil 6)
                    ┌─────────────┴──────────────┐
                    │  DER MOTOR / DAS WERK       │
                    │  cce-runner + cce-* Crates  │
                    │  (deterministisch, Autorität)│
                    └─────────────────────────────┘
```

- **GUI (egui):** Die vier Flächen (§S3.2). Zeigt nur, was CockpitCore ihr gibt.
- **CockpitCore:** Hält den Sitzungs- und Workspace-Zustand (das Cockpit-Zustandsmodell §S3.3). Einziger Vermittler zwischen GUI, KI-Kanzel, Motor und Persistenz. Enthält **keine** Motor-Logik — es ruft den Motor über den `EnginePort` (Trait) auf, exakt die deklarierten Ports aus Bauverfassung Teil 6.
- **KI-Kanzel:** LLM-gestützter Modul. **Zwei** erlaubte Fähigkeiten (autonom): Wunsch formen, Motor-Ausgaben erklären. **Keine** Schreibrechte auf Urteile/Residuen/Ledger. Liegt architektonisch **außerhalb** des Abschlusspfads (§S3.4).
- **EnginePort (trait):** Stabile, in-process Schnittstelle zum Motor. Für den Einzelnutzer-Fall (R-Plan-4) linkt das Cockpit die Motor-Crates direkt (kein IPC-Nichtdeterminismus). Der Trait ist so geschnitten, dass später dieselbe Core-Logik als lokaler Dienst exponiert werden kann (Mehrnutzer/Sync), **ohne** dass sich der GUI-Vertrag ändert.
- **PersistenceAdapter:** Lokaler, inhaltsadressierter Speicher; portabler Ledger (`PHC-Portable`). Von Beginn an **sync-fähig** strukturiert (R-Plan-5, §S3.7).

### S3.1.3 Determinismus-Grenze (architektonisch fixiert)

Die einzige nichtdeterministische Komponente ist die **KI-Kanzel** (LLM). Sie wird durch die **Bestätigungsgrenze** vom deterministischen Rest getrennt:

```
[ nichtdeterministisch ]        │  Bestätigungsgrenze  │        [ deterministisch, Fix(R) ]
 KI-Kanzel formt Wunsch  ──▶  bestätigter Crystal  ──▶  Motor (encode→…→reanalyze→equivalent)
                                (Operator bestätigt)       fester RunDescriptor, replaybar
```

Alles links der Grenze ist Vorbereitung und nie autoritativ. Alles rechts ist der geschlossene, replaybare Motorlauf. **Replay reproduziert aus dem bestätigten Crystal + RunDescriptor**, nie durch erneutes Befragen des LLM. Damit bleibt die Replay-Garantie (Bauverfassung INV-10) unberührt, obwohl ein LLM im Cockpit sitzt.

---

## S3.2 — Die vier Flächen im Detail

Das Cockpit hat genau vier Flächen. Jede wird hier mit Zweck, Anzeige, erlaubten Operator-Aktionen, gebundenen Motor-Daten und einem konkreten Dokument-Beispiel spezifiziert.

### S3.2.1 Wunsch-Fläche

- **Zweck:** Aus einem natürlichsprachlichen Anliegen einen **wohlgeformten Crystal** über die Wunsch-Normalform `W=(X,H,K,G,Res,Π,τ,Replay,Goal,Materialize)` (Bauverfassung Teil 3) machen.
- **Anzeige:** Eingabefeld für den Wunsch (Klartext); daneben der von der Kanzel geformte Crystal in lesbarer Struktur (Ziel, Randbedingungen K, Horizont H, Gegenhorizont, Residuen-Erwartung, Materialisierungsziel).
- **Operator-Aktionen:** Wunsch eingeben/ändern; Kanzel-Rückfragen beantworten; den geformten Crystal **bestätigen** (materielle Aktion → explizite Bestätigung) oder verwerfen.
- **Gebundene Motor-Daten:** Schema-Validierung des Crystals gegen `phc.schema.json` (Bauverfassung Teil 7) — der Motor prüft Wohlgeformtheit, das Cockpit zeigt grün/rot mit Grund.
- **Dokument-Beispiel:** Operator tippt „Ich brauche ein zweiseitiges Memo, das die drei Projektrisiken benennt, jede mit Gegenmaßnahme, ohne Bewertungszahlen." → Kanzel formt daraus einen Dokument-Crystal (Struktur: 3 Risiko-Zellen je mit Gegenmaßnahme-Naht; Randbedingung: keine Score-Felder → deckt sich mit V1; Materialisierungsziel: `.docx`/`.md`). Operator prüft und bestätigt.

### S3.2.2 Lauf-Fläche

- **Zweck:** Den bestätigten Crystal durch den geschlossenen Motorpfad fahren und den Fortschritt zeigen.
- **Anzeige:** Der Pfad `encode → project → loom → materialize → reanalyze → equivalent` als Fortschrittskette; aktuelle Stufe; laufende Gates.
- **Operator-Aktionen:** **Lauf starten** (materielle Aktion → explizite Bestätigung); pausieren/fortsetzen; an einem Human-in-the-Loop-Gate entscheiden (§S3.5).
- **Gebundene Motor-Daten:** Der `cce-runner`-Lauf mit festem RunDescriptor; Stufen-Ereignisse; Gate-Aufrufe.
- **Dokument-Beispiel:** Der bestätigte Memo-Crystal läuft; die Fläche zeigt „project ✓ → loom ✓ → materialize … → G3 Naht-Gate prüft, ob jede Risiko-Zelle eine Gegenmaßnahme-Naht hat".

### S3.2.3 Prüf-Fläche

- **Zweck:** Sichtbarkeit herstellen — **alles**, was der Motor an Urteilen und Residuen erzeugt hat, für einen Menschen lesbar (das ist P4/V2 als Pixel).
- **Anzeige:** (a) **Gate-Report** je Pflicht-Gate G1–G7 + BCIK/QLOGIC-Gates: grün/rot **mit Begründung**; (b) **Residuenliste**: jedes `B⁻x` als Eintrag, inkl. **„leer = geschlossen" explizit angezeigt**; Severity `{info, warning, blocking}`; (c) **Ledger**: die append-only Hash-Kette als lesbare Historie; (d) **Replay-Griff**: Knopf „identisch wiederholen"; (e) **Abschlussbeweis**: das Ergebnis des Kerntests `equivalent(reanalyze(materialize(loom(project(encode(C))))), C)` für diesen Lauf.
- **Operator-Aktionen:** Residuen aufklappen/prüfen; Ledger durchsehen; Replay auslösen (reproduziert dieselbe Klasse); an HITL-Gates entscheiden.
- **Gebundene Motor-Daten:** `GateReport`, `Residue`-Felder, `Ledger`, `Certificate`, Kerntest-Ergebnis — alles read-only aus dem Motor.
- **Harte UI-Regel:** Es gibt **keinen** „Trotzdem durchlassen"-Knopf für ein hartes rotes Gate (das wäre V7 Fail-open). Blockierende Residuen verhindern sichtbar `sealed`/`closed`.
- **Dokument-Beispiel:** Prüf-Fläche zeigt „G3 Naht-Gate: rot — Risiko 2 hat keine Gegenmaßnahme-Naht (Residuum, blocking)". Der Operator sieht sofort, was fehlt; kein Wegdrücken möglich.

### S3.2.4 Artefakt-Fläche

- **Zweck:** Das fertige, geschlossene Artefakt in **domänen-nativer** Form bereitstellen.
- **Anzeige:** Vorschau des materialisierten Artefakts; Entnahme-/Export-Optionen; Verweis auf den zugehörigen Ledger-Eintrag und Abschlussbeweis.
- **Operator-Aktionen:** Artefakt ansehen; exportieren/speichern (in den Workspace, §S3.7); im domänen-nativen Programm öffnen.
- **Gebundene Motor-Daten:** Das `Artifact` aus `cce-materialize`, inhaltsadressiert, mit Zertifikat.
- **Dokument-Beispiel:** Bei geschlossenem Lauf erscheint das Memo als `.docx`/`.md`; „Öffnen" startet es im Textprogramm; der Export legt es content-adressiert im Workspace ab.

---

## S3.3 — Zustandsmodell des Cockpits

Der Operator-Fluss ist eine **deterministische Zustandsmaschine**, die die Motor-Lebensachse (`raw→triangulated→gated→sealed→closed`, Bauverfassung Teil 3) auf Bedienebene spiegelt:

```
 LEER
   │ Wunsch eingeben
   ▼
 WUNSCH_ERFASST
   │ Kanzel formt (autonom) + Operator-Rückfragen
   ▼
 CRYSTAL_GEFORMT ──(Motor: Schema rot)──▶ zurück zu WUNSCH_ERFASST (mit Grund)
   │ Operator BESTÄTIGT  ← materielle Aktion
   ▼
 BESTÄTIGT  ═══ Bestätigungsgrenze: ab hier deterministisch ═══
   │ Operator startet Lauf  ← materielle Aktion
   ▼
 LÄUFT ──(Gate braucht Entscheidung)──▶ AN_GATE(HITL) ──Operator entscheidet──▶ LÄUFT
   │
   ├──(hartes Gate rot / blocking Residuum)──▶ ABGELEHNT (sichtbar, mit Grund, Residuum geführt)
   │
   ▼ (alle Gates grün, B⁻x=0, Kerntest grün)
 GESCHLOSSEN
   │ Materialisierung
   ▼
 ARTEFAKT_VERFÜGBAR
   │ Replay (jederzeit) ──▶ reproduziert dieselbe Klasse ──▶ ARTEFAKT_VERFÜGBAR
   ▼
 (Workspace: abgelegt, content-adressiert, Ledger fortgeschrieben)
```

Regeln: In jedem Zustand ist genau definiert, welche Aktionen erlaubt sind. Kein Zustand erlaubt das Überspringen eines Gates. `ABGELEHNT` ist ein **gültiger, ehrlicher** Endzustand (der Wunsch war nicht schließbar) — mit ausgewiesenem Grund und geführtem Residuum/Gegenhorizont, nie ein verstecktes Scheitern.

---

## S3.4 — Die KI-Kanzel: Verhalten und harte Bindung

Dies ist die Bedienebenen-Umsetzung von R-Plan-3 und der Nutzungs-Governance (S13).

### S3.4.1 Was die Kanzel autonom darf

1. **Wunsch formen.** Aus Klartext einen wohlgeformten Crystal (Wunsch-Normalform) bilden, Rückfragen stellen, den geformten Crystal zur Bestätigung vorlegen.
2. **Erklären.** Gates, Residuen, Ledger, Abschluss in Klartext übersetzen und einordnen.

Mehr nicht. Diese beiden sind read-only bzw. vorbereitend und berühren keine Motor-Autorität.

### S3.4.2 Was nur nach expliziter Bestätigung geschieht

**Jede materielle Aktion:**
- den geformten Crystal bestätigen (Eintritt in den deterministischen Pfad),
- einen Lauf starten,
- eine gate-relevante Operator-Entscheidung treffen (§S3.5),
- ein Artefakt exportieren/festschreiben.

Die Kanzel kann solche Aktionen **vorschlagen**, aber nie selbst auslösen. Jeder Vorschlag ist als Vorschlag markiert; der Operator bestätigt; der Motor validiert unabhängig.

### S3.4.3 Harte Bindungen (V1–V10 auf Bedienebene) — architektonisch erzwungen

Die Kanzel kann **niemals**:
- ein Gate als bestanden darstellen, das der Motor rot geliefert hat (V1/V7) — sie hat **keinen Schreibpfad** auf `GateReport`;
- ein Residuum verbergen, weichzeichnen oder weglassen (V2) — Residuen fließen Motor→Core→Anzeige, nie durch die Kanzel;
- eine Kennzahl in ein Gate verwandeln (V1);
- eine Behauptung über Physik/RH/`π,ζ`-als-Operator jenseits der Korpus-Reichweite aufstellen (V10) — ein Ausgabefilter erzwingt die Claim-Schranke (Bauverfassung INV-14);
- den Nullpunkt-Durchlauf, Boundary-ohne-Naht, Rohimport o. Ä. herbeireden (V3/V4/V6) — diese sind Motor-Prüfungen, die die Kanzel nur berichtet.

**Mechanik der Erzwingung:** Urteile und Residuen sind für die Kanzel **schreibgeschützt**. Der Datenfluss ist gerichtet: Motor → CockpitCore → (a) GUI-Anzeige, (b) Kanzel-als-Leser. Die Kanzel produziert nur Text und Vorschläge; beides ist nie autoritativ und wird vor jeder Wirkung durch Operator-Bestätigung **und** unabhängige Motor-Validierung geführt.

### S3.4.4 Transparenz der Kanzel

Kanzel-Ausgaben sind stets als **Interpretation** gekennzeichnet (nicht als Motor-Urteil). Die LLM-Interaktion kann für Nachvollziehbarkeit protokolliert werden, ist aber **nicht** Teil der Replay-Garantie (§S3.1.3).

---

## S3.5 — Human-in-the-Loop an Gates

Nicht jedes Gate ist gleich. Das Cockpit unterscheidet zwei Klassen scharf:

- **Harte Gates (automatisch, fail-closed, nicht übersteuerbar).** BCIK-Integrität, V1–V10-Verletzungen, Schema-/Signaturbrüche. Der Operator kann sie **nicht** übergehen; das Cockpit bietet dafür keinen Knopf. Rot heißt rot.
- **Ermessens-Gates (Human-in-the-Loop, wo der Korpus Operator-Eingabe zulässt).** Wo eine begründete menschliche Entscheidung vorgesehen ist, präsentiert das Cockpit die Entscheidung **mit vollem Kontext**: das betroffene Residuum, den Gegenhorizont, die Nullmodelle. Der Operator entscheidet; die Entscheidung wird **in den Ledger geschrieben** (nachvollziehbar, append-only).

In **keinem** Fall entscheidet eine Kennzahl (V1). In **keinem** Fall kann eine Operator- oder Kanzel-Entscheidung ein hartes Gate umkehren.

---

## S3.6 — Sichtbarkeit von Residuen und Abschluss (Inspektions-Kopplung)

Die Prüf-Fläche (§S3.2.3) ist der Ort, an dem „Residuum sichtbar" (Bauverfassung P4/V2) buchstäblich sichtbar wird. Rendering-Verträge:

- **Gate-Report:** je Gate ein Eintrag `{name, status∈{grün,rot}, Begründung}`. Rot ist nie ohne Begründung.
- **Residuenliste:** je Residuum `{quelle, B⁻x-Inhalt, severity}`. **Leeres Residuum wird explizit als „geschlossen (∅)" angezeigt**, nicht weggelassen — Schließung heißt `B⁻x=0` *mit* ausgewiesenem, dann leerem Feld.
- **Ledger:** die Hash-Kette als lesbare, unveränderliche Historie; jeder Eintrag content-adressiert.
- **Replay-Griff:** ein Knopf „identisch wiederholen"; erzeugt nachweislich dieselbe Commit-Klasse `[x]∈X/≡σ`.
- **Abschlussbeweis:** das Kerntest-Ergebnis für diesen Lauf, als grüner/roter Nachweis der Abschlussformel — nicht als Behauptung, sondern als geprüftes Faktum.

---

## S3.7 — Persistenz- und Workspace-Sicht (sync-fähig ausgelegt)

Gemäß R-Plan-5 (lokal, Sync fest geplant):

- **Workspace-Modell:** Läufe, Kristalle, Artefakte und Ledger leben in einem lokalen **Arbeitsbereich**. Einzelnutzer zuerst (R-Plan-4), aber mit einem Identitäts-/Workspace-Modell, an das später Mehrnutzer/Prüfer **ohne Änderung des GUI-Vertrags** andockt.
- **Inhaltsadressierter Store:** Artefakte und Kristalle werden über Inhalts-Digest abgelegt (Bauverfassung P10). Gleicher Inhalt ⇒ gleiche Adresse — die natürliche Grundlage für spätere Synchronisation ohne Konflikte.
- **Portabler Ledger (`PHC-Portable`):** Der Ledger ist im portablen Format geführt, sodass er auf einem anderen Gerät exakt wiederholbar ist (Bauverfassung Konformitätsstufe `PHC-Portable`).
- **Sync-Bereitschaft by design:** Store und Ledger sind so strukturiert (inhaltsadressiert, append-only, portabel), dass Synchronisation eine **eingeplante spätere Ebene** ist (S9), kein Nachrüsten am Kern. Der `PersistenceAdapter`-Trait trennt „lokaler Store jetzt" von „synchronisierter Store später" hinter einer stabilen Schnittstelle.

---

## S3.8 — Dokument-Domäne als erster konkreter Durchlauf

Gemäß R-Plan-2 ist **Dokument** die erste vollständig geschlossene Domäne. Ein vollständiger Cockpit-Durchlauf, konkret:

1. **Wünschen.** Operator: „Zweiseitiges Memo, drei Projektrisiken, jedes mit Gegenmaßnahme, keine Bewertungszahlen."
2. **Formen.** Kanzel bildet den Dokument-Crystal: drei Risiko-Zellen, je mit Gegenmaßnahme-Naht (Separator); Randbedingung „keine Score-Felder" (deckt sich mit V1); Horizont/Gegenhorizont „vollständige Risikoabdeckung / fehlende Gegenmaßnahme"; Materialisierungsziel `.docx`/`.md`. Operator bestätigt.
3. **Laufen.** Motor fährt `encode→project→loom→materialize→reanalyze→equivalent` mit festem RunDescriptor.
4. **Prüfen.** Prüf-Fläche zeigt Gates (u. a. G3 Naht-Gate: jede Risiko-Zelle hat eine Gegenmaßnahme-Naht), Residuen (idealerweise „geschlossen (∅)"), Ledger, Replay-Griff, Abschlussbeweis. Fehlt einem Risiko die Gegenmaßnahme → rotes Naht-Gate + blockierendes Residuum, sichtbar, nicht wegdrückbar.
5. **Verwenden.** Artefakt-Fläche zeigt das fertige Memo als `.docx`/`.md`; „Öffnen" startet das Textprogramm; Export legt es content-adressiert im Workspace ab.
6. **Wiederholen.** Replay reproduziert dasselbe Memo (gleiche Klasse).

Die genaue Eigenschafts-Grammatik der Dokument-Domäne (welche Struktur-/Randbedingungstypen es gibt) gehört in die **S1-Dokument-Instanziierungs-Spec** (nächste D-Spec), nicht hierher — hier zählt, dass das Cockpit diese Domäne vollständig bedienbar macht.

---

## S3.9 — Cockpit-Abnahme (DoD dieser Ebene)

Diese D-Spec gilt als **realisiert** genau dann, wenn:

```
DoD(S3) = 1  ⟺
    Ein Einzelnutzer-Operator kann OHNE Konsole
        Wunsch (Klartext) → bestätigter Dokument-Crystal
        → geschlossenes, materialisiertes Artefakt (.docx/.md)
        → sichtbar geprüft (Gates, Residuen, Ledger, Replay, Abschlussbeweis)
        → Artefakt entnehmen/öffnen
        → identisch wiederholen (gleiche Klasse)
    ∧ die KI-Kanzel formt/erklärt autonom, löst aber KEINE materielle Aktion ohne Bestätigung aus
    ∧ die Kanzel hat KEINEN Schreibpfad auf Urteile/Residuen/Ledger (V1/V2/V7 erzwungen)
    ∧ die Kanzel hält die Claim-Schranke (V10/INV-14)
    ∧ KEIN „Trotzdem durchlassen"-Knopf für harte Gates existiert (V7)
    ∧ blockierende Residuen verhindern sichtbar sealed/closed (P4/V2)
    ∧ das Cockpit-Zustandsmodell ist deterministisch; die KI liegt außerhalb Fix(R)
    ∧ Persistenz ist lokal UND sync-fähig ausgelegt (inhaltsadressiert, PHC-Portable)
    ∧ Engine-DoD(cce)=1 bleibt unberührt (der Motor ist Autorität)
```

**Cockpit-Invarianten (CI-prüfbar, wo möglich):**
- COCK-INV-1: Kein GUI-Pfad erzeugt oder verändert ein Gate-Urteil.
- COCK-INV-2: Kein GUI-Pfad verbirgt ein vorhandenes Residuum.
- COCK-INV-3: Jede materielle Aktion ist an eine explizite Operator-Bestätigung gebunden.
- COCK-INV-4: Die Kanzel-Ausgaben sind nie autoritativ und stets als Interpretation markiert.
- COCK-INV-5: Replay reproduziert aus bestätigtem Crystal + RunDescriptor, nie aus LLM-Neuabfrage.

---

## S3.10 — Sichtbare Residuen dieser Spezifikation

Konsistent mit der Werk-Philosophie — kein stilles Loch:

- **R1 (Toolkit-Endwahl).** `egui` vs. `iced` ist noch offen; Empfehlung `egui`. Entscheidbar bei Realisierungsbeginn; ändert die Verträge dieser Spec nicht.
- **R2 (Dokument-Grammatik).** Die genaue Typmenge der Dokument-Domäne (Struktur-/Randbedingungstypen) gehört in die S1-Dokument-Spec, nicht hierher — hier bewusst offen gelassen und dorthin verwiesen.
- **R3 (Visuelles Design).** Farben, Layout, Feinabstimmung der Flächen sind einem späteren Gestaltungsdurchgang vorbehalten; diese Spec fixiert Struktur und Verhalten, nicht Pixel.
- **R4 (LLM-Anbindung der Kanzel).** Welches Modell/welche Schnittstelle die Kanzel nutzt, ist offen; irrelevant für die Garantien, da die Kanzel außerhalb Fix(R) liegt und keinen Schreibpfad hat.

Diese vier Punkte sind der einzige offene Anteil dieser D-Spec — sichtbar ausgewiesen, nicht versteckt.

---

## S3.11 — Anschluss

Diese D-Spec macht den Motor bedienbar: native Rust-App, vier Flächen, deterministisches Zustandsmodell, eine gebundene KI-Kanzel außerhalb des Abschlusspfads, sichtbare Residuen und Abschlüsse, sync-fähige Persistenz — und ein vollständiger Dokument-Durchlauf ohne Konsole.

**Nächste D-Spec (Systemlandkarte §6):** **S1 — Dokument-Instanziierungsvertrag** — die konkrete Grammatik der Dokument-Domäne (was ein Dokument-Crystal an Struktur/Randbedingungen kennt, was materialize erzeugt), mit Adapter-Parität als Vorlage für alle weiteren Domänen. Danach S4 (Wunscherfassung), S6 (Inspektions-Modell), S5, S8, S9, S13, S2, S11, S12 — bis `ProduktDoD=1`.

*Ende der Detail-Spezifikation S3.*
