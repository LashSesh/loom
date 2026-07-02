# CCE — SYSTEMLANDKARTE & PRODUKTVERFASSUNG

**Die Ebene über der Bauverfassung.** Vom *Werk* (Motor) zum fertigen, bedienbaren *System* (Produkt) — vollständig auf Papier, bevor eine Zeile realisiert wird.

**Status:** Masterplan v1.0. Dies ist die Stadtplanung, nicht der Bauplan eines einzelnen Flügels. Sie legt fest, **was alles dazugehört, von Anfang bis Ende**, und in welcher Reihenfolge jede Ebene danach auf volle Bauverfassungs-Tiefe spezifiziert wird.

---

## 0 — Warum die Bauverfassung allein noch kein fertiges Projekt ist

Die `BAUVERFASSUNG.md` (Teile 0–9) ist vollständig — **aber sie beschreibt das Werk, nicht das Produkt.**

- Ein **Werk** ist der Motor: Fiber, Skeleton, zwei Sweeps, Kristall-Übergabe, Gates, Residuen, Replay, der grüne Kerntest `Reanalyze(Materialize(LOOM(Project(PHC(Crystal))))) ≃ Crystal`.
- Ein **Produkt** ist das ganze Fahrzeug: der Motor **plus** alles, was einen Menschen in die Lage versetzt, damit von seinem *Wunsch* zu einem *fertigen, geprüften Artefakt* zu kommen — ohne je eine Konsole zu berühren.

Nach dem Bau der Bauverfassung existiert die Maschine. Aber: **Wie bedient man sie?** Woher kommt der Kristall überhaupt? Wie sieht ein Mensch die Gates, die Residuen, den Abschlussbeweis? Wie bekommt er das Artefakt heraus und benutzt es? Diese Fragen sind im Werk *nicht* beantwortet — und dürfen es auch nicht sein, denn sie gehören auf eine andere Ebene.

Diese Systemlandkarte zieht diese Ebene ein. Sie wendet außerdem die **eigene Philosophie des Werks auf den Plan selbst an**: kein stilles Loch. Jeder echte offene Punkt in der Planung wird als **sichtbares Residuum** geführt (§7), nie verschwiegen. „100 % auf Papier" heißt nicht „allwissend", sondern **geschlossen im eigenen Sinn**: vollständige, kohärente Struktur, lückenlose Teileliste, und jede verbleibende Entscheidung offen ausgewiesen statt versteckt.

---

## 1 — Die vollständige Systemlandkarte (Kern + vier Ringe)

Das fertige System besteht aus dem **Kern (Werk)** und **vier Ringen** von Ebenen um ihn herum. Der Kern ist spezifiziert; die vier Ringe sind es zum größten Teil noch nicht — genau das sind die „mehreren Stufen, die noch nicht angerissen sind".

```
                    RING D — Abschluss & Auslieferung
        ┌───────────────────────────────────────────────────┐
        │   RING C — Substanz & Bestand                      │
        │   ┌───────────────────────────────────────────┐    │
        │   │   RING B — Bedienung (die fehlende Schale) │    │
        │   │   ┌───────────────────────────────────┐    │    │
        │   │   │   RING A — Sinn & Nutzer          │    │    │
        │   │   │   ┌───────────────────────────┐    │    │    │
        │   │   │   │        KERN               │    │    │    │
        │   │   │   │   DAS WERK / ENGINE       │    │    │    │
        │   │   │   │   (BAUVERFASSUNG 0–9)     │    │    │    │
        │   │   │   │   SPEZIFIZIERT ✓          │    │    │    │
        │   │   │   └───────────────────────────┘    │    │    │
        │   │   └───────────────────────────────────┘    │    │
        │   └───────────────────────────────────────────┘    │
        └───────────────────────────────────────────────────┘
```

**Status-Legende:** ✓ spezifiziert (Bauverfassung) · ◐ Primitive im Werk vorhanden, Produkt-Ebene offen · ○ offen (noch nicht angerissen)

| Ring | # | Ebene | Was dazugehört | Status |
|------|-----|-------|----------------|--------|
| **Kern** | — | **Das Werk / Engine** | Motor: Objekte, Operatoren, Gates, Residuen, Replay, Kerntest | **✓** |
| **A** | S1 | **Domänenrahmen & Instanziierung** | Konkreter Katalog der Domänen; was Crystal/Cube/Artefakt in *jeder* Domäne konkret **ist** (Dokument, Beweis, Graph, Modul …) | ◐ |
| **A** | S2 | **Operator-, Rollenmodell & End-to-End-Reise** | Wer benutzt es (Operator/Autor/Prüfer); die vollständige Reise Wunsch → Artefakt | ○ |
| **B** | S3 | **Nutzungsoberfläche / Cockpit** | Die Schale: GUI + gebundene KI-Orchestrierung. **Die direkte Antwort auf „wie bediene ich das ohne Konsole"** | ○ |
| **B** | S4 | **Wunscherfassung & Autorenschicht** | Natürlichsprachlicher Wunsch → wohlgeformter Crystal über die Wunsch-Normalform; Autorenschaft von Cubes | ◐ |
| **B** | S5 | **Orchestrierung & Human-in-the-Loop** | Auftragssteuerung, Fortschritt, Pause/Fortsetzen, menschliche Gate-Freigaben | ◐ |
| **B** | S6 | **Inspektion & Nachvollziehbarkeit (für Menschen)** | Gates, Residuen, Ledger, Replay, Abschlussbeweis — **sichtbar und verständlich** für einen Menschen | ◐ |
| **B** | S7 | **Artefakt-Ausgabe & domänenspezifische Verwendung** | Artefakt entnehmen, ansehen, exportieren, benutzen — in domänen-nativer Form | ◐ |
| **C** | S8 | **Bibliothek** | Referenz-Cubes & Negativ-Cubes als *kuratierte, echte* Bibliothek pro Domäne; Vorlagen, Beispiel-Wünsche | ◐ |
| **C** | S9 | **Persistenz, Arbeitsbereiche & Projektverwaltung** | Wo Läufe/Kristalle/Artefakte/Ledger über Sitzungen leben; Workspaces, Versionierung, inhaltsadressierter Speicher | ◐ |
| **D** | S10 | **Produktabnahme (End-to-End) & Vollständigkeitsvertrag** | Produkt-DoD; **Adapter-Parität** (identische, vollständige Teileliste je Domäne — das Modellbaukasten-Prinzip) | ○ |
| **D** | S11 | **Auslieferung, Paketierung, Betrieb** | Installierbare App, Konfiguration, Updates — **keine Konsole** | ○ |
| **D** | S12 | **Operator-Dokumentation & Onboarding** | Handbuch fürs *Benutzen*, nicht fürs Bauen; Einarbeitung | ○ |
| **D** | S13 | **Nutzungs-Governance / Produktverfassung** | Die Verbote (kein Score-als-Gate, Residuum sichtbar, Claim-Schranke) auf *Bedienebene*; die KI-Orchestrierung an dieselben Regeln gebunden | ◐ |

Dreizehn Ebenen um den Kern — jede genuin verschieden, keine entbehrlich. **Ring B ist die eigentliche Lücke:** die Bedienschale, die aus dem Motor ein Fahrzeug macht.

---

## 2 — Der Dreh- und Angelpunkt: das Cockpit (S3) — „wie bediene ich die Maschine?"

Dies ist Ihre Kernfrage, und sie verdient die konkreteste Antwort. Vorschlag mit Begründung:

### 2.1 Zweiteilige Bedienschale

**(1) Ein grafisches Operator-Cockpit** (lokale App — Desktop oder lokale Web-App). Das ist der reale Arbeitsplatz. Vier Flächen:

- **Wunsch-Fläche** — hier formuliert der Operator sein Anliegen (natürlichsprachlich).
- **Lauf-Fläche** — Start, Fortschritt, Pause/Fortsetzen, Gate-Freigaben.
- **Prüf-Fläche** — Gates (grün/rot + Begründung), Residuen (sichtbar, inkl. „leer = geschlossen"), Ledger, Replay, Abschlussbeweis.
- **Artefakt-Fläche** — das fertige Artefakt in domänen-nativer Darstellung; entnehmen/exportieren.

Der Operator berührt **nie** eine Konsole. `cargo build`, Rust, CLI-Runner sind reine Bauwerkzeuge des Coding-Agenten — sie liegen *unter* dem Cockpit und werden dem Menschen nie zugemutet.

**(2) Eine gebundene KI-Orchestrierung** *innerhalb* des Cockpits (die „Kanzel"). Sie ist genau auf Ihre Arbeitsweise zugeschnitten — Sie bedienen ein ernsthaftes Werkzeug über natürliche Sprache:

- Sie übersetzt den natürlichsprachlichen Wunsch in einen **wohlgeformten Crystal** über die Wunsch-Normalform `W=(X,H,K,G,Res,Π,τ,Replay,Goal,Materialize)` und legt ihn dem Operator zur Bestätigung vor.
- Sie **erklärt** Gates, Residuen und den Abschluss in Klartext.
- Aber sie ist **hart gebunden** (S13): Die KI kann **kein** Gate fälschen, das der Motor rot geliefert hat; **kein** Residuum verbergen; **keine** Reichweite überschreiten (Physik/RH/`π,ζ`-Behauptung). Die **Autorität liegt beim deterministischen Rust-Motor**, die KI ist nur **Dolmetscher und Führer**.

### 2.2 Warum diese Aufteilung genau richtig ist

Der Motor ist deterministisch, gate-getrieben und residuen-transparent — **ideal, um von einem Orchestrator gefahren zu werden**, der Wünsche in Kristalle übersetzt und Ergebnisse zurück in Menschensprache. Die Garantien (Determinismus, Abschluss, Replay) kommen aus dem *Rust-Werk* und sind unbestechlich; die *Bequemlichkeit* (natürliche Sprache, Erklärung) kommt aus der *KI-Kanzel*; und die Verbote V1–V10 bleiben **auf Bedienebene** in Kraft, weil die KI die Garantien des Motors nicht aufweichen darf. So bekommen Sie ein ernsthaftes Werkzeug, das Sie sprechend bedienen — ohne dass die KI je etwas beschönigen kann.

---

## 3 — Die End-to-End-Reise des Operators (S2) — konkret

Der greifbare Ablauf, „wie man die Maschine benutzt", auf Masterplan-Auflösung:

1. **Öffnen.** Operator startet das Cockpit (App, keine Konsole).
2. **Wünschen.** Er formuliert sein Anliegen natürlichsprachlich („Ich will *X*" — ein Dokument, einen Beweis, einen Graphen, ein Modul …).
3. **Formen.** Die gebundene KI-Kanzel erfragt die fehlenden Teile der Wunsch-Normalform (Ziel, Randbedingungen, Horizont/Gegenhorizont) und zeigt den resultierenden **wohlgeformten Crystal** zur Bestätigung.
4. **Laufen.** Operator bestätigt; der deterministische Motor fährt den geschlossenen Pfad `encode → project → loom → materialize → reanalyze → equivalent` mit festem RunDescriptor.
5. **Prüfen (sichtbar).** An jedem Pflicht-Gate zeigt das Cockpit grün/rot + Begründung; **blockierende Residuen werden sichtbar**; wo ein Gate eine menschliche Entscheidung braucht (Human-in-the-Loop), entscheidet der Operator — aber ein hartes Gate kann die KI **nie** übergehen.
6. **Abschluss.** Bei Schließung zeigt das Cockpit: das **materialisierte Artefakt**, die **Residuenliste** (sichtbar, inkl. leer = geschlossen), den **Ledger-/Replay-Griff** und den **Abschlussbeweis** (Kerntest grün für diesen Lauf).
7. **Verwenden.** Operator öffnet/exportiert/benutzt das Artefakt in domänen-nativer Form.
8. **Jederzeit.** Lauf **wiederholen** (gleiche Klasse), **Ledger** einsehen, **Residuen** erneut prüfen.

Das ist die vollständige Antwort auf „wie benutze ich die Maschine denn?".

---

## 4 — Domänenrahmen (S1) — was der Motor konkret produziert

Der Motor ist domänenagnostisch; das Produkt braucht einen **konkreten Katalog**. Für jede Domäne wird ein **Instanziierungsvertrag** spezifiziert (in der Detaillierungsphase auf volle Tiefe), der genau vier Dinge festlegt:

| Domäne | Was ist der Wunsch? | Was ist der Crystal? | Was ist das materialisierte Artefakt? | Was heißt „domänen-native Verwendung"? |
|--------|---------------------|----------------------|----------------------------------------|-----------------------------------------|
| **Dokument** | „Erzeuge/prüfe Dokument mit Eigenschaften …" | Kristallisierte Struktur + Randbedingungen | `.docx`/`.md`/`.pdf` | Öffnen im Textprogramm |
| **Graph** | „Erzeuge/prüfe Graphen/Netz mit …" | Kristallisierte Knoten/Kanten/Invarianten | Graph-Datei + Rendering | Anzeigen/Weiterverarbeiten |
| **Software-Modul** | „Erzeuge Modul, das … erfüllt" | Kristallisierte Spezifikation + Gates | Quell-/Testdateien | In ein Repo übernehmen |
| **Mathe/Struktur** | „Schließe folgende Struktur ab / triangulieren" | Kristallisierte Struktur + Gegenhorizont | Beweis-/Struktur-Artefakt | Als Nachweis lesen |

**Wichtig (Adapter-Parität, siehe §5):** Jede Domäne bekommt die **identische, vollständige** Teileliste — denselben Satz an Gates, Residuenfeldern, Replay-Griffen, Ausgabewegen. Keine Domäne wird halb gebaut.

---

## 5 — Produkt-Abnahme (S10): der Vollständigkeitsvertrag

Die Engine-DoD (Bauverfassung §8.7) sagt: „Kerntest grün." Das ist die *Motor*-Abnahme. Die **Produkt**-Abnahme ist strenger und bildet Ihr Modellbaukasten-Prinzip direkt ab:

```
ProduktDoD(CCE) = 1  ⟺
   für JEDE Domäne im Katalog (S1):
       Operator kann  Wunsch (natürlichsprachlich)
                 → wohlgeformter Crystal            (S3/S4)
                 → geschlossenes, materialisiertes Artefakt   (Werk)
                 → im Cockpit sichtbar geprüft
                    (Gates, Residuen, Ledger, Replay, Abschlussbeweis)   (S6)
                 → Artefakt entnehmen/verwenden     (S7)
       reproduzierbar (gleicher Wunsch + RunDescriptor ⇒ gleiche Klasse)
   ∧ ADAPTER-PARITÄT: jede Domäne hat die IDENTISCHE, vollständige Teileliste   ← Modellbaukasten
   ∧ Engine-DoD(cce) = 1        (Motor-Abschluss, VC1⁺ grün)
   ∧ keine Bedienung erfordert eine Konsole            (S3/S11)
   ∧ Nutzungs-Governance aktiv: KI-Orchestrator kann kein Gate fälschen,
       kein Residuum verbergen, keine Reichweite überschreiten   (S13 / V1–V10)
   ∧ Operator-Doku deckt die gesamte Reise ab          (S12)
   ∧ jeder echte offene Punkt ist als sichtbares Residuum im Plan geführt
       (Konsistenz mit der Werk-Philosophie: kein stilles Loch)
```

**Adapter-Parität** ist der Kern Ihrer Schrauben-Analogie: So wie jedes Modell eines Bausatzes exakt dieselbe Anzahl Schrauben hat, exponiert jeder Domänen-Adapter exakt denselben, vollständigen Satz an Teilen. Ein `ci/check_adapter_parity`-Gate erzwingt das maschinell: Fehlt einem Adapter auch nur ein Teil (ein Gate, ein Residuenfeld, ein Ausgabeweg), **rot** — das Produkt gilt als unfertig. Kein „ah, da fehlt noch was" beim Bauen.

---

## 6 — Der Weg zu „100 % auf Papier": Spezifikations-Reihenfolge

Diese Landkarte ist der Masterplan (Ebene: Struktur + Verträge + Abnahme). „100 % auf Papier" ist erreicht, wenn **jede** der 13 Ebenen auf dieselbe Tiefe gebracht ist wie das Werk in der Bauverfassung. Empfohlene Detaillierungs-Reihenfolge (jede Stufe endet mit ihrer eigenen Abnahme-Definition, bevor die nächste beginnt — Ihr Architektenprinzip):

```
Masterplan (dieses Dokument)  ✓ jetzt
   → D-Spec S3   Cockpit + KI-Kanzel (Bedien-Architektur, Flächen, Zustände, HITL)   ← Linchpin zuerst
   → D-Spec S1   Domänen-Instanziierungsverträge (alle Katalog-Domänen, Adapter-Parität)
   → D-Spec S4   Wunscherfassung & Autorenschicht (Wunsch → Crystal, Elicitation)
   → D-Spec S6   Inspektions-Modell (menschliche Sicht auf Gates/Residuen/Ledger/Replay)
   → D-Spec S7   Artefakt-Ausgabe je Domäne (Entnahme/Export/Nutzung)
   → D-Spec S5   Orchestrierung & Human-in-the-Loop (Auftrag/Fortschritt/Freigaben)
   → D-Spec S8   Bibliothek (kuratierte Referenz-/Negativ-Cubes, Vorlagen)
   → D-Spec S9   Persistenz & Arbeitsbereiche (Workspaces, Store, Versionierung)
   → D-Spec S13  Nutzungs-Governance (Verbote auf Bedienebene, KI-Bindung)
   → D-Spec S2   Operator-Reise (verbindliche End-to-End-Definition + Abnahme)
   → D-Spec S11  Auslieferung/Paketierung/Betrieb (App, Config, Updates)
   → D-Spec S12  Operator-Dokumentation & Onboarding
   → PRODUKT-ABNAHME  ProduktDoD = 1  über alle Domänen
```

Erst wenn diese Kette komplett ist, ist das Projekt „auf Papier" fertig — **dann** übernimmt der Coding-Agent (Bauverfassung Teil 9) und baut aus einem Guss.

---

## 7 — Sichtbare Residuen des Plans (offene Entscheidungen — nicht verschwiegen)

Konsistent mit der Werk-Philosophie führe ich die echten offenen Entscheidungspunkte **sichtbar** auf, statt sie still zu setzen. Jeder ist eine Weiche, die Ihre Richtungsentscheidung braucht:

- **R-Plan-1 (Cockpit-Form).** Desktop-App oder lokale Web-App? Empfehlung: lokale Web-App (plattformunabhängig, ein Codepfad), als Desktop-App paketierbar. **Zu bestätigen.**
- **R-Plan-2 (Domänen-Katalog Startumfang).** Welche Domäne wird als erste *vollständig* geschlossen (Prinzip P1 der Bauverfassung)? Empfehlung: **Dokument** oder **Graph** (klein, deterministisch, sichtbares Ergebnis). **Zu wählen.**
- **R-Plan-3 (Reichweite der KI-Kanzel).** Wie viel darf die KI eigenständig (Wunsch formen, erklären) vs. nur auf Bestätigung? Empfehlung: formen + erklären ja, **jede** materielle Aktion (Lauf, Gate-relevante Entscheidung) nur nach expliziter Operator-Bestätigung. **Zu bestätigen.**
- **R-Plan-4 (Einzelnutzer vs. Mehrnutzer).** Vorerst reines Einzelnutzer-Werkzeug (Sie als Operator), oder spätere Mehrnutzer-/Prüfer-Rollen mitdenken? Empfehlung: Einzelnutzer zuerst, Rollenmodell (S2) so anlegen, dass Mehrnutzer später ohne Kernänderung andockt. **Zu bestätigen.**
- **R-Plan-5 (Persistenz-Ort).** Rein lokal (Dateisystem, inhaltsadressiert) — oder später synchronisierbar? Empfehlung: lokal zuerst, Ledger als portables Format (Bauverfassung: `PHC-Portable`), Sync als spätere, optionale Ebene. **Zu bestätigen.**

Kein Loch bleibt still: Diese fünf Weichen sind der einzige „unfertige" Anteil dieses Masterplans — und sie sind hier offen ausgewiesen, nicht versteckt.

---

## 8 — Was jetzt ansteht

Dieser Masterplan schließt die **Struktur- und Abnahme-Ebene** des Gesamtprojekts: Er benennt alle 13 Ebenen von Anfang bis Ende, markiert ehrlich, was spezifiziert ist (der Kern) und was noch nicht (die vier Ringe), beantwortet Ihre Kernfrage („wie bediene ich das?") mit dem Cockpit-Modell, und bindet die Vollständigkeit an einen echten Produkt-Vertrag mit Adapter-Parität (Ihr Modellbaukasten-Prinzip).

**Nächster Schritt zur Detaillierung:** die erste D-Spec — das **Cockpit + die KI-Kanzel (S3)** — auf volle Bauverfassungs-Tiefe. Sie ist der Linchpin, weil sie die Bedienschale ist, die aus dem Motor ein Fahrzeug macht; alle weiteren Ringe hängen an ihr.

*Ende des Masterplans.*
