# CCE — DETAIL-SPEZIFIKATION S4: WUNSCHERFASSUNG & AUTORENSCHICHT

**Dritte D-Spec.** Die Vorgrenz-Ebene zwischen dem natürlichsprachlichen Wunsch und dem bestätigten Crystal — der Elicitation-Fluss, der aus Klartext einen wohlgeformten DocCrystal formt.

**Status:** Detail-Spezifikation v1.0, auf Bauverfassungs-Tiefe. Baut auf: BAUVERFASSUNG (Wunsch-Normalform), S3 (Cockpit/Kanzel, Bestätigungsgrenze), S1 (Dokument-Grammatik, Schema).

---

## S4.0 — Einordnung & Leitsatz

S4 liegt **vor der Bestätigungsgrenze** (S3.1.3): Alles hier ist Vorbereitung, nichts ist autoritativ. Die Ebene überführt einen Klartext-Wunsch in einen wohlgeformten DocCrystal (S1-Grammatik), der dann — nach expliziter Bestätigung — zum deterministischen Eintrittspunkt des Motors wird.

Leitsatz:

> *Der Wunsch wird durch geführte, transparente Erfassung zum Crystal: die KI entwirft und fragt, der Motor validiert, der Mensch bestätigt — und keine Annahme der KI bleibt verborgen.*

Die **Dreiteilung der Verantwortung** ist das Rückgrat dieser Spec und die Vertiefung der Grundfesten aus S3.0:

- **KI-Kanzel: entwirft und fragt** (autonom, aber nie autoritativ, nie schreibend auf Urteile).
- **Motor: validiert** (deterministisch, autoritativ — die KI erklärt das Ergebnis, erzeugt es nie).
- **Operator: bestätigt** (die einzige Instanz, die die Bestätigungsgrenze überschreitet).

Und die S4-spezifische Vertiefung von P4/V2 (kein stilles Residuum): **Auch die eigenen Annahmen der KI werden sichtbar gemacht.** Nicht nur Motor-Residuen, sondern jede Deutung, jeder Default, jede gefüllte Lücke der KI steht in einer ausgewiesenen Annahmen-Liste (§S4.3). Die KI füllt nichts still aus.

---

## S4.1 — Die Erfassungs-Pipeline

Sieben Stufen führen vom rohen Wunsch zum bestätigten Crystal. Die Pipeline läuft in der **Wunsch-Fläche** des Cockpits (S3.2.1); die KI-Kanzel ist ihr Agent.

```
 1. AUFNAHME        roher Klartext-Wunsch
        │
        ▼
 2. DEUTUNG         KI erkennt: welche Teile der Wunsch-Normalform sind da,
        │           welche fehlen, welche sind mehrdeutig  (gegen S1-Grammatik)
        ▼
 3. RÜCKFRAGE       KI stellt gezielte, minimale Fragen, um Lücken zu füllen
        │           (nur was zur Wohlgeformtheit nötig ist; §S4.2)
        ▼
 4. ENTWURF         KI baut einen Draft-DocCrystal + Annahmen-Liste (§S4.3)
        │
        ▼
 5. VALIDIERUNG     MOTOR prüft deterministisch gegen phc.schema.json + S1-Grammatik
        │           → Validierungsbericht (grün/rot + Gründe + Residuen); §S4.4
        │─── rot ──▶ zurück zu 3. RÜCKFRAGE (mit benannter Lücke)
        ▼ grün
 6. SICHTBARMACHUNG Draft-Crystal + Annahmen-Liste + Validierungsbericht
        │           werden dem Operator gezeigt (nichts verborgen)
        ▼
 7. BESTÄTIGUNG     Operator bestätigt (materielle Aktion → Bestätigungsgrenze)
        │           oder schickt zurück zur Überarbeitung
        ▼
   ═══ ab hier deterministisch: bestätigter Crystal → Motor (Lauf-Fläche S3.2.2) ═══
```

Die Pipeline ist **konvergent gedacht**: Rückfrage und Validierung wiederholen sich, bis der Crystal wohlgeformt ist — oder bis sich zeigt, dass der Wunsch nicht erfassbar ist (§S4.7), was ein ehrlicher Endzustand ist, kein Fake-Crystal.

---

## S4.2 — Die Rückfrage-Disziplin

Wie die KI fragt, ist geregelt — nicht beliebig:

- **Minimal und gezielt.** Es wird nur gefragt, was zur Wohlgeformtheit fehlt. Keine Über-Befragung; vorzugsweise eine Frage nach der anderen, aus den offenen Pflicht-Slots der S1-Grammatik abgeleitet (fehlt `required_sections`? `covers(T)`? Format? eine `SupportSeam`-Pflicht?).
- **Grammatik-getrieben.** Jede Frage entspringt einem konkreten fehlenden Slot des Schemas, nicht freier Neugier.
- **Keine stille Annahme bei Bedeutung.** Wo eine Lücke **semantisch folgenreich** ist (Inhalt, Abdeckung, Nahtpflicht), wird **gefragt**, nie still gefüllt. Wo ein Default **konventionell und harmlos** ist (z. B. Zielformat `.docx`), darf die KI ihn setzen — aber **nur sichtbar** in der Annahmen-Liste, nie verborgen.
- **Mehrdeutigkeit auflösen, nicht raten.** Bei mehrdeutigem Wunsch legt die KI die Deutungen vor und fragt, statt eine still zu wählen.
- **Beschränkte Runden.** Die Erfassung soll rasch konvergieren. Gelingt das nicht (Wunsch zu vage/widersprüchlich), wird die Lücke als Residuum ausgewiesen (§S4.7), statt endlos weiterzufragen.

---

## S4.3 — Der Wunsch-Entwurf & die Annahmen-Liste

Der Draft-DocCrystal wird **stets** von einer expliziten **Annahmen-Liste** begleitet. Sie ist die S4-Form von „kein stilles Loch" und macht die *Entscheidungen der KI selbst* sichtbar:

Jeder Eintrag der Annahmen-Liste ist eines von:
- **Default** — ein konventioneller Wert, den die KI gesetzt hat (z. B. „Format: `.docx` angenommen — änderbar").
- **Deutung** — eine Interpretation eines mehrdeutigen Wunschteils, die die KI gewählt und hier ausgewiesen hat.
- **Füllung** — eine Lücke, die die KI (harmlos) gefüllt hat, mit Angabe, worauf sie beruht.
- **Offen** — eine Lücke, die die KI **nicht** gefüllt hat und die eine Rückfrage oder Operator-Entscheidung braucht.

Die Annahmen-Liste erscheint neben dem Draft-Crystal in der Wunsch-Fläche (S3.2.1). Der Operator sieht auf einen Blick, was er sagte, was die KI daraus machte, und **wo sie annahm statt zu wissen**. Nichts, was die KI entschied, bleibt verborgen.

---

## S4.4 — Validierung (deterministisch, durch den Motor)

Die Validierung des Draft-Crystals ist **Sache des Motors, nicht der KI** (S3.0-Grundfeste 1 und 2):

- **Schema-Konformität:** gegen `phc.schema.json` (Bauverfassung Teil 7).
- **Grammatik-Konformität:** gegen das S1-Vokabular (Einheitstypen, Nahttypen, Randbedingungstypen).
- **Wohlgeformtheit:** nichtleerer Rand, Signatur-Konsistenz (Bauverfassung INV-2/INV-3), keine verwaiste Einheit.

Der Validierungsbericht ist **deterministisch und autoritativ**: grün/rot mit Gründen und benannten Residuen. Die KI darf ihn **erklären** (in Klartext), aber nicht **erzeugen** oder beschönigen. Rot ⇒ zurück in die Rückfrage-Stufe mit der konkret benannten Lücke.

Damit ist die Autorität auch in der Vorgrenz-Ebene beim Motor: Selbst *ob ein Wunsch wohlgeformt ist*, entscheidet der deterministische Kern, nicht das LLM.

---

## S4.5 — Die Bestätigungsgrenze

Der präzise Übergang vom Nichtdeterministischen zum Deterministischen (Vertiefung von S3.1.3):

- **Vor der Bestätigung:** alles ist Entwurf — KI-geformt, nicht autoritativ, überarbeitbar. Die Annahmen-Liste ist offen einsehbar. Kein RunDescriptor, kein Motorlauf.
- **Die Bestätigung** ist eine **explizite Operator-Aktion** (materielle Aktion, S3.4.2). Der Operator bestätigt den Draft-Crystal (samt gesichteter Annahmen-Liste) — oder schickt ihn zurück.
- **Bei der Bestätigung:** Der Crystal wird **eingefroren**, content-adressiert (Bauverfassung P10) und zum **deterministischen Eintrittspunkt** mit festem RunDescriptor. Die Rolle der KI **endet** für diesen Crystal.
- **Nach der Bestätigung:** Der deterministische Motor übernimmt (Lauf-Fläche S3.2.2). **Replay reproduziert aus dem bestätigten Crystal + RunDescriptor** — nie durch erneute Erfassung, nie durch erneutes Befragen der KI (Bauverfassung INV-10).

Die Bestätigungsgrenze ist die Naht, an der die nichtdeterministische Vorbereitung sauber in den replaybaren Kern übergeht.

---

## S4.6 — Autorenschicht

Wiederverwendbare Autorenschaft, damit nicht jeder Wunsch bei null beginnt:

- **Vorlagen (Templates).** Ein bestätigter Crystal (oder ein Fragment) kann als **Vorlage** gespeichert werden — z. B. die „Drei-Risiken-Memo"-Struktur als wiederverwendbares Muster. Die Vorlage wird beim späteren Wunsch angeboten und von der KI gefüllt. (Speicherformat/-ort gehört in S8; S4 definiert, **wie** aus einem Crystal eine Vorlage entsteht.)
- **Fragment-Wiederverwendung.** Wiederkehrende Bausteine (z. B. das Paar `Risk`+`Countermeasure` mit `SupportSeam`) als wiederverwendbare Crystal-Fragmente, die die KI in der Erfassung vorschlagen darf — als Vorschlag, nie still eingesetzt.
- **Cube-Autorenschaft.** Ein Autor kann Referenz-Cubes und Negativ-Cubes (die Test-Assets aus S1.9) definieren. Für Einzelnutzer-zuerst (R-Plan-4) ist Autor = Operator; die Fähigkeit ist aber so spezifiziert, dass eine getrennte Autoren-/Prüfer-Rolle später **ohne Kernänderung** andockt.
- **Selbst-Validierung.** Jede Vorlage, jedes Fragment, jeder autorisierte Cube unterliegt **derselben deterministischen Validierung** (§S4.4) — ein Autoren-Artefakt muss wohlgeformt sein, sonst wird es nicht gespeichert.

---

## S4.7 — Fehlschläge & Nicht-Erfassbarkeit (ehrliche Endzustände)

Nicht jeder Wunsch lässt sich in einen wohlgeformten Crystal überführen. Das ist zulässig und wird **ehrlich** behandelt, nie durch einen Fake-Crystal überdeckt (konsistent mit Bauverfassung P12, §8.6):

- **Unvereinbarer Wunsch.** Widersprüchliche Randbedingungen (z. B. `covers(T)` fordert Inhalt, den `max_length` verbietet) werden als **Residuum** sichtbar gemacht — der Konflikt wird dem Operator gezeigt, nicht still aufgelöst.
- **Zu vage.** Bleibt der Wunsch nach beschränkter Rückfrage (S4.2) unterspezifiziert, wird die verbleibende Lücke als **Residuum/Offen** benannt — kein stiller Default für semantisch folgenreiche Lücken.
- **Außerhalb der Domäne.** Ist der Wunsch kein Dokument-Anliegen, sagt die KI das offen (und leitet später, bei mehreren Domänen, an den richtigen Adapter). Das ist ehrliche Grenze, keine Verlegenheitslösung.

In allen Fällen: Die Erfassung **weigert sich, einen Crystal vorzutäuschen**. Ein benannter Nicht-Erfassungs-Zustand ist ein gültiges Ergebnis dieser Ebene.

---

## S4.8 — Kopplung an S3 und S1 (aus einem Guss)

- **An S3:** Die Pipeline läuft in der **Wunsch-Fläche** (S3.2.1); die **KI-Kanzel** ist ihr Agent mit exakt den Bindungen aus S3.4 (autonom entwerfen/erklären, keine materielle Aktion ohne Bestätigung, kein Schreibpfad auf Urteile). Die **Bestätigungsgrenze** ist die aus S3.1.3. Die Annahmen-Liste und der Validierungsbericht werden über CockpitCore an die GUI gebunden.
- **An S1:** Die Grammatik, die S4 füllt, ist die **DocCrystal-Grammatik** (S1.2). Die Validierung nutzt das **S1-Schema und -Vokabular** (S1.4). Der Draft zielt auf das **S1-Objektmodell** (DocUnit/SupportSeam/DocBoundary/DocHorizon). Die von S4 erzeugten bestätigten Crystals sind exakt die Eingaben des geschlossenen Pfads aus S1.3.

Damit greift S4 lückenlos in Cockpit und Domäne — dieselbe Apparatur, kein Bruch.

---

## S4.9 — Sichtbare Residuen dieser Spezifikation

Kein stilles Loch:

- **R1 (Frage-UX).** Genaue Formulierung und Darstellung der Rückfragen ist einem Gestaltungsdurchgang vorbehalten; diese Spec fixiert die **Disziplin** (§S4.2), nicht den Wortlaut.
- **R2 (Auto-Vorschlag-Reichweite).** Wie aktiv die KI Fragmente vorschlagen darf (vs. nur auf Nachfrage), ist eine feine Justierung; Grundregel: Vorschlag ja, stiller Einsatz nein.
- **R3 (Vorlagen-Speicherung).** Format/Ort der Vorlagen gehört in S8; hier nur die Entstehung aus einem Crystal.
- **R4 (Runden-Schranke).** Dass die Erfassung beschränkt konvergiert, steht fest; die exakte Runden-/Abbruch-Schranke ist beim Auftreten zu fixieren.

---

## S4.10 — Abnahme (DoD dieser Ebene)

```
DoD(S4) = 1  ⟺
    Ein Operator kann einen Klartext-Wunsch über die Erfassungs-Pipeline (§S4.1)
        zu einem wohlgeformten, validierten, bestätigten DocCrystal führen
  ∧ die KI entwirft/fragt autonom, aber JEDE Annahme steht sichtbar in der Annahmen-Liste
        (kein stilles Ausfüllen semantisch folgenreicher Lücken)
  ∧ die Validierung ist deterministisch und durch den Motor; die KI erklärt, urteilt nie
  ∧ die Bestätigung ist eine explizite Operator-Aktion; erst danach beginnt der deterministische Pfad;
        Replay reproduziert aus dem bestätigten Crystal, nie aus erneuter Erfassung
  ∧ unvereinbare / zu vage / domänenfremde Wünsche werden als ehrliche Residuen/Grenzen ausgewiesen,
        nie als Fake-Crystal
  ∧ die Autorenschicht erlaubt Vorlagen / Fragmente / Cube-Autorenschaft, jeweils selbst validiert
  ∧ Kopplung an S3 (Wunsch-Fläche, Kanzel-Bindungen, Bestätigungsgrenze) und S1 (Grammatik/Schema/Objektmodell)
        ist konsistent
  ∧ Engine-DoD, DoD(S3), DoD(S1) bleiben unberührt
```

---

## S4.11 — Anschluss

S4 schließt die Vorgrenz-Ebene: Aus Klartext wird ein wohlgeformter, transparent geformter, deterministisch validierter, explizit bestätigter Crystal — mit sichtbaren Annahmen und ehrlichen Fehlschlägen. Der Übergang in den Motor ist sauber.

**Nächste D-Spec (Systemlandkarte §6):** **S6 — Inspektions-Modell** — die menschliche Sicht auf Gates, Residuen, Ledger, Replay und Abschlussbeweis in voller Tiefe (in S3.2.3/S3.6 skizziert, hier ausgearbeitet: Rendering-Verträge, Aufklär-Tiefe, Nachvollziehbarkeit). Danach S5, S8, S9, S13, S2, S11, S12 — bis `ProduktDoD=1`.

*Ende der Detail-Spezifikation S4.*
