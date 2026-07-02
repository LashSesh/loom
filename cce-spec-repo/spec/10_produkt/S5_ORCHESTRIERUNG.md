# CCE — DETAIL-SPEZIFIKATION S5: ORCHESTRIERUNG & HUMAN-IN-THE-LOOP

**Sechste D-Spec.** Die Steuerung eines Laufs auf Produktebene: Auftrag, Fortschritt, Pause/Fortsetzen, menschliche Gate-Freigaben — und die deterministische Wiederaufnahme, die all das erlaubt, ohne Replay zu brechen.

**Status:** Detail-Spezifikation v1.0, auf Bauverfassungs-Tiefe. Baut auf: BAUVERFASSUNG (RunDescriptor, Determinismus P9, Ledger INV-12, Replay INV-10), S3 (Lauf-Fläche S3.2.2, HITL an Gates S3.5), S4 (bestätigter Crystal als Eintrittspunkt S4.5), S6 (Ledger/Nachvollziehbarkeit), S1 (der domänen-konkrete Pfad).

---

## S5.0 — Einordnung & der zentrale Konflikt

S5 steuert den Lauf **nach** der Bestätigungsgrenze (S4.5) — also im **deterministischen** Bereich. Das erzeugt einen scheinbaren Widerspruch, den diese Spec auflöst:

> **Der Konflikt:** Ein Lauf soll pausierbar, fortsetzbar und an Ermessens-Gates durch einen Menschen entscheidbar sein. Zugleich muss er **deterministisch und replaybar** bleiben (Bauverfassung P9/INV-10). Menschliche Entscheidungen und Pausen sind aber genau das, was Determinismus zu brechen droht.

**Die Auflösung — Leitsatz:**

> *Jede menschliche Entscheidung und jeder Wiederaufnahmepunkt wird als bestimmter Eingang aufgezeichnet. Dadurch wird die Entscheidung Teil dessen, was den Lauf reproduzierbar macht — Replay spielt die aufgezeichnete Entscheidung ab, statt den Menschen erneut zu fragen.*

Damit koexistieren Human-in-the-Loop und Determinismus: Die einzige nichtdeterministische Zutat (die menschliche Wahl) wird **einmal** erfasst und ist danach ein fixer Eingang wie jeder andere.

---

## S5.1 — Der Lauf-Lebenszyklus

Der Lauf ist eine deterministische Zustandsmaschine auf Orchestrierungsebene, feiner als das Cockpit-Zustandsmodell (S3.3):

```
 EINGEREICHT   (bestätigter Crystal + initialer RunDescriptor)
     │ starten
     ▼
 LÄUFT ──────────────────────────────────────────────┐
     │  durchläuft die Pfadstufen mit Checkpoints:    │
     │  encode → project → loom → materialize →       │
     │  reanalyze → equivalent  (S1.3)                │
     │                                                │
     ├── Operator pausiert ─────▶ PAUSIERT ──fortsetzen──┘
     │
     ├── Ermessens-Gate ────────▶ AN_GATE(HITL) ──Entscheidung aufgezeichnet──▶ LÄUFT
     │
     ├── hartes Gate rot / blocking Residuum / interner Fehler ─▶ ABGELEHNT (Grund benannt, aufgezeichnet)
     │
     ▼ (alle Gates grün, B⁻x=0, Kerntest grün)
 GESCHLOSSEN → Materialisierung → Artefakt (S7)
```

Jeder Stufenübergang ist ein **Checkpoint** (§S5.2). `PAUSIERT`, `AN_GATE(HITL)` und `ABGELEHNT` sind gültige Zustände; `ABGELEHNT` ist ein ehrlicher Endzustand mit benanntem Grund, kein Absturz.

---

## S5.2 — Checkpointing (die Grundlage von Pause & Wiederaufnahme)

- **Wohldefinierte Checkpoint-Grenzen.** Der Lauf setzt Checkpoints an den Stufengrenzen des geschlossenen Pfads und vor jedem Gate.
- **Content-adressierter Checkpoint.** Ein Checkpoint erfasst den Lauf-Zustand **inhaltsadressiert** (Bauverfassung P10): gleicher Zustand ⇒ gleicher Checkpoint-Digest.
- **Deterministische Fortsetzung.** Wiederaufnahme setzt **exakt** am Checkpoint fort — als deterministische Fortsetzung, nicht als Neustart. Der Checkpoint enthält alles, was zur identischen Weiterführung nötig ist (Zwischen-Crystal, bisherige Gate-Reports, bisherige aufgezeichnete Entscheidungen).

---

## S5.3 — Pause & Fortsetzen (Inhaltsklasse bleibt erhalten)

- **Pausieren** erfasst einen Checkpoint und hält den Lauf an. Keine Motor-Arbeit läuft weiter.
- **Fortsetzen** nimmt den Lauf am Checkpoint deterministisch wieder auf.
- **Die Garantie:** Pause/Fortsetzen **ändert das Ergebnis nicht.** Ein pausierter-und-fortgesetzter Lauf erzeugt die **identische kanonische Inhaltsklasse** (S1.6) wie ein durchgehender Lauf. Das ist beweisbar, weil die Fortsetzung eine deterministische Weiterführung aus einem inhaltsadressierten Checkpoint ist — es fließt kein neuer, unbestimmter Eingang ein.

Damit ist Pause/Fortsetzen eine reine Bedien-Bequemlichkeit ohne semantische Wirkung.

---

## S5.4 — Human-in-the-Loop, deterministisch aufgezeichnet (der Kern)

Wenn ein **Ermessens-Gate** (S3.5) eine menschliche Entscheidung braucht:

```
 1. Der Lauf erreicht das Ermessens-Gate → Checkpoint → Zustand AN_GATE(HITL).
 2. Das Cockpit zeigt die Entscheidung mit vollem Kontext:
        das betroffene Residuum, den Gegenhorizont, die Nullmodelle  (S6.3).
 3. Der Operator entscheidet.
 4. Die Entscheidung wird als bestimmter Eingang in den Ledger geschrieben
        (append-only, S6.4): {gate, entscheidung, zeitpunkt, operator}.
 5. Die Entscheidung wird in den RunDescriptor aufgenommen (§S5.5).
 6. Der Lauf setzt mit der Entscheidung als fixem Eingang fort.
```

**Die entscheidende Eigenschaft:** Beim **Replay** wird die **aufgezeichnete Entscheidung abgespielt**, nicht der Mensch erneut gefragt. Die menschliche Wahl ist einmal erfasst und danach ein fixer Eingang — Replay reproduziert den Lauf **einschließlich** der Entscheidung und erzeugt dieselbe Klasse (Bauverfassung INV-10).

**Die harte Grenze:** **Harte Gates** (V1–V10-Verletzungen, BCIK-Integrität) pausieren **nie** für eine menschliche Entscheidung — sie sind automatisch fail-closed (S3.5). Nur **Ermessens-Gates**, wo der Korpus Operator-Eingabe zulässt, pausieren. Eine menschliche Entscheidung kann ein hartes Gate **nie** umkehren.

---

## S5.5 — Der RunDescriptor, erweitert (Replay wohldefiniert trotz HITL)

Damit Replay auch mit menschlichen Entscheidungen wohldefiniert ist, umfasst der RunDescriptor **drei** Bestandteile:

```
RunDescriptor = {
    crystal_digest      : Digest des bestätigten Crystal (S4.5),
    decisions           : geordnete Liste der aufgezeichneten HITL-Entscheidungen (§S5.4),
    params              : deterministische Parameter (Zielformat, Profil, ...)
}
```

Damit heißt **„gleicher RunDescriptor"** präzise: **gleicher Crystal + gleiche aufgezeichnete Entscheidungen + gleiche Parameter** ⇒ gleiche kanonische Inhaltsklasse. Ohne die `decisions`-Komponente wäre Replay bei HITL-Läufen unterbestimmt; mit ihr ist es exakt (Bauverfassung INV-10, CL Satz 10.6).

---

## S5.6 — Fortschritt & Fehler (ehrlich, aufgezeichnet)

- **Fortschritt.** Die Lauf-Fläche (S3.2.2) zeigt die Pfadstufen, die aktuelle Stufe und die Gates, während sie laufen. **Kein** Fortschrittsbalken als Kennzahl-Gate (V1) — Fortschritt ist Anzeige, nie Entscheidung.
- **Fehler ehrlich.** Ein Lauf kann scheitern: hartes Gate rot, blockierendes Residuum, interner Fehler. Das führt in `ABGELEHNT` mit **benanntem Grund** (welches Gate, welches Residuum) und wird aufgezeichnet (S6). Scheitern ist ein **gültiges, transparentes Ergebnis**, kein Absturz und keine Verlegenheit.
- **Fehler reproduzierbar.** Auch ein abgelehnter Lauf ist replaybar: Derselbe RunDescriptor führt zur selben Ablehnung mit demselben Grund — die Ablehnung ist so nachprüfbar wie ein Erfolg.

---

## S5.7 — Determinismus & Nebenläufigkeit (Grenzen)

- **Determinismus auf dem Abschlusspfad.** Kein Wall-Clock, keine ungeseedete Zufälligkeit, keine nichtdeterministische Nebenläufigkeit, die das Ergebnis beeinflusst (Bauverfassung P9). Die Orchestrierung darf **deterministisch** takten.
- **Einziger unbestimmter Eingang: die menschliche Entscheidung** — und die wird aufgezeichnet (§S5.4), also für Replay bestimmt gemacht.
- **Innerhalb eines Laufs:** der geschlossene Pfad ist eine **deterministische Sequenz** (P9 verbietet nichtdeterministische Nebenläufigkeit auf dem Abschlusspfad).
- **Über Läufe hinweg:** mehrere Läufe sind **unabhängig und isoliert** (inhaltsadressiert). Für das Einzelnutzer-Werkzeug (R-Plan-4) sind Läufe typischerweise sequenziell oder wenige; die genaue Mehrlauf-Politik gehört teils zu S9.
- **Die KI-Kanzel ist nicht in der Orchestrierungs-Autorität.** Sie kann den Fortschritt **erklären**, aber sie treibt keine Gates, fällt keine Entscheidungen, setzt keine Läufe fort ohne Operator-Bestätigung (S3.4).

---

## S5.8 — Kopplung an S3, S4, S6, S1 (aus einem Guss)

- **An S3:** S5 treibt die **Lauf-Fläche** (S3.2.2) und die **HITL-Punkte an Gates** (S3.5). Start/Pause/Fortsetzen/Entscheiden sind die materiellen Aktionen, die dort Operator-Bestätigung verlangen (S3.4.2).
- **An S4:** S5 beginnt am **bestätigten Crystal** (S4.5) — dem deterministischen Eintrittspunkt.
- **An S6:** S5 schreibt jede Entscheidung, jeden Checkpoint, jede Ablehnung in den **Ledger** (S6.4); die Nachvollziehbarkeit (S6.8) deckt Läufe vollständig ab.
- **An S1:** die Pfadstufen, Gates und Residuen sind die **domänen-konkreten** aus S1.3/S1.4/S1.5.

---

## S5.9 — Read-only-/Determinismus-Garantien (Verbote auf Orchestrierungsebene)

Abwesende Handhaben:

- **kein „hartes Gate übersteuern"** — eine menschliche Entscheidung kann ein hartes Gate nie umkehren (S5.4, V7).
- **kein „Stufe überspringen"** — der geschlossene Pfad wird vollständig durchlaufen; keine Stufe ist auslassbar.
- **kein „Lauf ohne RunDescriptor"** — jeder Lauf trägt einen vollständigen RunDescriptor (§S5.5); ohne ihn kein Lauf.
- **kein nichtdeterministischer Eingang außer der aufgezeichneten Entscheidung** — nichts fließt in den Lauf, was Replay unterbestimmt ließe (P9/INV-10).
- **kein „Fortschritt als Schwelle"** — eine Fortschrittszahl entscheidet nie (V1).

---

## S5.10 — Sichtbare Residuen dieser Spezifikation

Kein stilles Loch:

- **R1 (Checkpoint-Granularität).** Dass an Stufengrenzen und vor Gates gecheckpointet wird, steht fest; feinere Granularität ist justierbar, ohne die Garantien zu berühren.
- **R2 (Fortschritts-UX).** Genaue Darstellung des Fortschritts ist einem Gestaltungsdurchgang vorbehalten; die Nicht-Kennzahl-Regel steht fest.
- **R3 (Mehrlauf-Politik).** Ob/wie mehrere Läufe parallel laufen, ist teils S9; Grundregel: unabhängig, isoliert, inhaltsadressiert.
- **R4 (Pause/Fortsetzen-UX).** Darstellung von Pause/Wiederaufnahme ist offen; der Mechanismus (Checkpoint + deterministische Fortsetzung) steht fest.

---

## S5.11 — Abnahme (DoD dieser Ebene)

```
DoD(S5) = 1  ⟺
    Ein Operator kann einen Lauf einreichen, pausieren, fortsetzen, überwachen und an Ermessens-Gates entscheiden
  ∧ jede HITL-Entscheidung wird als bestimmter Eingang im Ledger aufgezeichnet und in den RunDescriptor aufgenommen;
        Replay spielt die aufgezeichnete Entscheidung ab, statt erneut zu fragen (§S5.4/S5.5)
  ∧ Pause/Fortsetzen erhält die kanonische Inhaltsklasse (deterministische Fortsetzung aus Checkpoint, §S5.3)
  ∧ harte Gates pausieren NIE für menschliche Eingabe und sind nie übersteuerbar; nur Ermessens-Gates pausieren
  ∧ Scheitern führt in ABGELEHNT mit benanntem Grund, aufgezeichnet und selbst replaybar (§S5.6)
  ∧ der Abschlusspfad ist deterministisch (P9); einziger unbestimmter Eingang ist die aufgezeichnete Entscheidung
  ∧ der RunDescriptor = {crystal_digest, decisions, params}; „gleicher RunDescriptor" ⇒ gleiche Klasse (INV-10)
  ∧ die KI-Kanzel treibt keine Gates/Entscheidungen/Fortsetzungen ohne Operator-Bestätigung
  ∧ Kopplung an S3/S4/S6/S1 konsistent
  ∧ Engine-DoD, DoD(S3), DoD(S1), DoD(S4), DoD(S6), DoD(S7) bleiben unberührt
```

---

## S5.12 — Anschluss

S5 schließt die Orchestrierungs-Ebene: Läufe sind steuerbar, pausierbar, menschlich entscheidbar — und dennoch vollständig deterministisch und replaybar, weil jede menschliche Entscheidung als bestimmter Eingang aufgezeichnet wird. Scheitern ist ehrlich, benannt und selbst reproduzierbar.

**Nächste D-Spec (Systemlandkarte §6):** **S8 — Bibliothek** — die kuratierte Sammlung von Referenz-Cubes und Negativ-Cubes pro Domäne, Vorlagen und Beispiel-Wünschen, als echter, gepflegter Bestandteil des Produkts (in S1.9/S4.6 berührt, hier als Bibliotheks-Ebene ausgearbeitet). Danach S9, S13, S2, S11, S12 — bis `ProduktDoD=1`.

*Ende der Detail-Spezifikation S5.*
