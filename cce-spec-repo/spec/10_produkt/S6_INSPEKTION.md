# CCE — DETAIL-SPEZIFIKATION S6: INSPEKTIONS-MODELL

**Vierte D-Spec.** Die menschliche Sicht auf alles, was der deterministische Motor erzeugt: Gates, Residuen, Ledger, Replay, Abschlussbeweis — vollständig, aufklärbar, nachvollziehbar, und strikt read-only.

**Status:** Detail-Spezifikation v1.0, auf Bauverfassungs-Tiefe. Baut auf: BAUVERFASSUNG (GateReport, Residue, Ledger, Certificate), S3 (Prüf-Fläche S3.2.3, Inspektions-Kopplung S3.6), S1 (Dokument-Gates/-Residuen, DocG-RoundTrip), S4 (die Eingabe-Transparenz, deren Audit-Gegenstück S6 ist).

---

## S6.0 — Einordnung & Leitsatz

S6 ist die **Audit-Ebene**: Sie macht die Autorität des Motors für einen Menschen **lesbar**. Wo S4 die Transparenz der *Eingabe* herstellt (was die KI aus dem Wunsch machte), stellt S6 die Transparenz des *Ergebnisses* her (was der Motor tat und warum).

Leitsatz:

> *Jeder angezeigte Sachverhalt stammt aus einem Motor-Artefakt, ist bis auf seine Wurzel aufklärbar, und kann durch den Menschen nur gelesen, nie geändert werden.*

Zwei Grundfesten tragen die ganze Spec:

- **Zwei Schichten, nie verwechselt.** Die **Fakten-Schicht** (Gate-Urteile, Residuen, Ledger, Zertifikate) kommt autoritativ aus dem Motor. Die **Erklär-Schicht** (Klartext der KI-Kanzel) ist optional, stets als Interpretation markiert (S3.4.4), nie autoritativ. Ein Mensch verwechselt eine Erklärung nie mit einem Urteil.
- **Read-only, ohne Ausnahme.** Die Inspektion bietet **keine** Handhabe, ein Urteil zu ändern, ein Residuum zu verbergen, ein Gate zu erzwingen oder den Ledger zu bearbeiten (S6.7). Das ist die Inspektions-Form der Verbotsachse V1/V2/V7 und der Cockpit-Invarianten COCK-INV-1/2 (S3.9).

---

## S6.1 — Die fünf Inspektionsobjekte (Überblick)

Die Prüf-Fläche (S3.2.3) zeigt genau fünf Inspektionsobjekte. Jedes hat einen Rendering-Vertrag (§S6.2–S6.6) und eine Aufklär-Tiefe (§S6.8):

| # | Objekt | Motor-Quelle | Kernfrage, die es beantwortet |
|---|--------|--------------|-------------------------------|
| 1 | **Gate-Report** | `GateReport` | Welche Gates liefen, welches ist grün/rot, **warum**? |
| 2 | **Residuenliste** | `Residue`-Felder | Was blieb offen (`B⁻x`)? Was blockiert? Was ist geschlossen (∅)? |
| 3 | **Ledger** | `Ledger` (Hash-Kette) | Was geschah in welcher Reihenfolge, unveränderlich belegt? |
| 4 | **Replay** | RunDescriptor + Store | Lässt sich der Lauf identisch wiederholen? |
| 5 | **Abschlussbeweis** | Kerntest-Ergebnis + `Certificate` | Gibt die Re-Analyse den Crystal bis auf kanonische Klasse zurück? |

---

## S6.2 — Gate-Report (Rendering-Vertrag)

- **Pro Gate ein Eintrag:** `{name, status ∈ {grün, rot}, Begründung, Bezug}`.
  - `name`: das Gate (Pflicht-Gates G1–G7 **und** Dokument-Gates DocG-Coverage/Support/NoScore/Structure/NonContradiction/Seam/RoundTrip, S1.4).
  - `status`: boolesch. **Keine Kennzahl** (V1) — nie eine Prozentzahl, nie ein Score.
  - `Begründung`: **rot ist nie ohne Begründung**. Grün trägt optional eine knappe Bestätigung.
  - `Bezug`: auf welchen Crystal/welche DocUnit/welche SupportSeam sich das Urteil bezieht (Grundlage der Aufklärung §S6.8).
- **Gate-Klassen sichtbar unterschieden:** harte Gates (automatisch, nicht übersteuerbar) vs. Ermessens-Gates (Human-in-the-Loop, S3.5) sind optisch getrennt, damit der Operator weiß, wo er entscheiden darf und wo nie.
- **Reihenfolge:** entlang des geschlossenen Pfads (`encode→…→reanalyze→equivalent`), sodass der Report den Lauf nacherzählt.

---

## S6.3 — Residuenliste (Rendering-Vertrag) — die ethische Krone

Die Residuenliste ist der Ort, an dem P4/V2 („Residuum sichtbar") vollständig eingelöst wird. Sie erhält daher die strengsten Regeln:

- **Immer vollständig gezeigt.** **Kein** Residuum wird herausgefiltert, zusammengefasst weggerundet oder in „Sonstiges" versteckt.
- **Pro Residuum:** `{kind, B⁻x-Inhalt, severity ∈ {info, warning, blocking}, quelle}`. Der `kind` stammt aus dem Dokument-Residuen-Vokabular (S1.5): `uncovered_topic`, `unsupported_unit`, `orphan_unit`, `contradiction`, `boundary_crossing_without_seam`, `forbidden_score_field`, `semantic_loss`, `invented_semantic`.
- **Leeres Residuum explizit.** Ein geschlossenes Residuum wird als **„geschlossen (∅)"** angezeigt, nie weggelassen — Schließung heißt `B⁻x=0` **mit** ausgewiesenem, dann leerem Feld.
- **Blockierend sichtbar wirksam.** `blocking`-Residuen sind optisch abgesetzt und **verhindern nachweislich** `sealed`/`closed`: Die Fläche zeigt, dass der Lauf **wegen** dieses Residuums nicht schließt.
- **Gegenhorizont sichtbar.** Der DocCounterHorizon (S1.5) wird als **die geprüfte Pathologien-Menge** gezeigt: Der Mensch sieht nicht nur „Residuum X feuerte", sondern **wogegen** geprüft wurde (welche Nullmodelle/Pathologien), damit Schließung nicht wie reine Selbstbestätigung wirkt.

---

## S6.4 — Ledger (Rendering-Vertrag)

- **Append-only, hash-verkettet, lesbar.** Der Ledger erscheint als unveränderliche Historie; jeder Eintrag content-adressiert.
- **Pro Eintrag:** `{event, digest, RunDescriptor-Referenz, Vorgänger-Hash}`. Der Mensch sieht die Sequenz der Ereignisse (Bestätigung, Gate-Läufe, Ermessens-Entscheidungen, Materialisierung, Versiegelung).
- **Ermessens-Entscheidungen protokolliert.** Jede Human-in-the-Loop-Entscheidung (S3.5) steht als Eintrag im Ledger — nachvollziehbar, wer/wann/was entschied.
- **Kettenprüfung.** Ein „Kette prüfen"-Aufruf (`verify_ledger`, S3.7) bestätigt die Unversehrtheit; eine nachträgliche Änderung eines Eintrags/Monolithen ist eine Abnahmeverletzung (Bauverfassung INV-12) und wird hier sichtbar.

---

## S6.5 — Replay (Rendering-Vertrag)

- **Der Replay-Griff.** Ein Knopf „identisch wiederholen"; erzeugt nachweislich dieselbe Commit-Klasse `[x] ∈ X/≡σ` (Bauverfassung INV-10, CL Satz 10.6).
- **Die Garantie sichtbar.** Die Fläche zeigt die Bedingung: **gleicher content-adressierter Input + gleicher RunDescriptor + deterministische Operatoren ⇒ dieselbe Klasse.**
- **Replay als unabhängige Prüfung.** Replay ist das Mittel, mit dem ein Mensch einen vergangenen Lauf **selbst** nachprüft — nicht der KI glauben, sondern reproduzieren. Der Replay reproduziert aus dem **bestätigten Crystal + RunDescriptor**, nie aus erneuter Erfassung oder KI-Neuabfrage (S4.5).
- **Replay wird protokolliert.** Ein Replay ist selbst ein Ledger-Ereignis (nachvollziehbar, dass und wann nachgeprüft wurde).

---

## S6.6 — Abschlussbeweis (Rendering-Vertrag) — der aufklärbare Beweis

Die Krone der Inspektion. Der Abschlussbeweis zeigt das Kerntest-Ergebnis **nicht** als undurchsichtiges „grün", sondern als **einen Beweis, in den ein Mensch hineinsehen kann**:

- **Die Aussage:** `equivalent(reanalyze(materialize(loom(project(encode(C))))), C)` — für die Dokument-Domäne konkret DocG-RoundTrip (S1.4).
- **Die zwei Seiten:** die Fläche zeigt die **kanonische Inhaltsklasse** des ursprünglichen DocCrystal `C` **und** die des re-analysierten `C'` — und dass sie übereinstimmen (S1.6).
- **Aufklärbar, warum sie übereinstimmen:** der Mensch kann hineingehen: **gleiche DocUnits, gleicher Naht-Graph, gleiche Abdeckung, gleiche bedeutungstragende Reihenfolge**. Der Beweis ist damit prüfbar, nicht bloß behauptet.
- **Bei Bruch benannt:** stimmen die Klassen nicht überein, zeigt der Beweis **welches** Residuum brach (`semantic_loss` = Materialisierung verlor Inhalt; `invented_semantic` = Materialisierung erfand Inhalt), mit Bezug auf die betroffene Einheit.
- **Das Zertifikat.** Der Abschluss trägt ein `Certificate` (Bauverfassung VC10), das **unabhängig prüfbar** ist — die Inspektion erlaubt „Zertifikat prüfen", eine erneute, deterministische Verifikation.

Damit ist der Abschluss kein Vertrauensakt gegenüber der Maschine, sondern ein **nachvollziehbarer, reproduzierbarer, zertifizierter Sachverhalt**.

---

## S6.7 — Read-only: die abwesenden Handhaben (Verbote auf Inspektionsebene)

Die Inspektion erzwingt die Prohibitionen dadurch, dass bestimmte Handhaben **schlicht nicht existieren**. Explizit abwesend:

- **kein „Urteil ändern"** — kein Pfad verändert einen GateReport (COCK-INV-1).
- **kein „Residuum ausblenden"** — kein Pfad verbirgt ein vorhandenes Residuum (COCK-INV-2, V2).
- **kein „Trotzdem durchlassen"** für ein hartes rotes Gate (V7 Fail-open, S3.2.3).
- **kein „Ledger bearbeiten"** — der Ledger ist append-only; Bearbeiten ist unmöglich, nicht nur verboten (INV-12).
- **kein „Score als Schwelle setzen"** — nirgends wird eine Kennzahl zum Entscheidungskriterium (V1).

Die KI-Kanzel kann in der Inspektion **erklären**, aber keine dieser Handhaben herstellen — sie hat keinen Schreibpfad auf die Fakten-Schicht (S3.4.3).

---

## S6.8 — Aufklär-Tiefe (Drill-down) & Nachvollziehbarkeit

Inspektion ist **nicht flach**. Vier Ebenen, jede lesbar und bis zur Wurzel rückführbar:

```
 Ebene 0 — ÜBERBLICK     „Der Lauf schloss / wurde abgelehnt."  (Status + Abschlussbeweis-Kurzform)
     │ aufklappen
 Ebene 1 — OBJEKT        die fünf Inspektionsobjekte je einzeln (Gates, Residuen, Ledger, Replay, Beweis)
     │ aufklappen
 Ebene 2 — ELEMENT       ein einzelnes Gate / ein einzelnes Residuum / ein einzelner Ledger-Eintrag,
     │                   mit Bezug (welche DocUnit / welche SupportSeam)
     │ aufklappen
 Ebene 3 — ROH-BELEG     das zugrundeliegende Motor-Artefakt (GateReport-Datensatz, Residue-Feld,
                         Ledger-Hash, Certificate) — die Wurzel, aus der der Sachverhalt stammt
```

**Nachvollziehbarkeits-Regel:** Jeder auf Ebene 0–2 gezeigte Sachverhalt ist auf Ebene 3 auf **genau ein** Motor-Artefakt zurückführbar. **Nichts Angezeigtes ist unbelegt.** Gibt es keine Wurzel, wird der Sachverhalt nicht gezeigt (es gibt keine „freischwebende" Anzeige).

---

## S6.9 — Kopplung an S3, S1, S4 (aus einem Guss)

- **An S3:** S6 rendert in der **Prüf-Fläche** (S3.2.3) und setzt die dort skizzierte Inspektions-Kopplung (S3.6) vollständig um. Die Fakten fließen Motor→CockpitCore→Anzeige; die KI liest und erklärt nur.
- **An S1:** S6 zeigt die **domänen-konkreten** Gates (DocG-*) und Residuen (DocResidue) und den Abschlussbeweis als **DocG-RoundTrip** mit der kanonischen Inhaltsklasse aus S1.6. Ohne S1 wäre die Anzeige abstrakt; mit S1 ist sie konkret prüfbar.
- **An S4:** S6 ist das **Audit-Gegenstück** zur Eingabe-Transparenz. S4 zeigt die Annahmen-Liste (was die KI aus dem Wunsch machte); S6 zeigt, was der Motor daraus machte. Zusammen decken sie die gesamte Reise transparent ab — Eingang und Ergebnis.

---

## S6.10 — Sichtbare Residuen dieser Spezifikation

Kein stilles Loch:

- **R1 (Visuelles Design).** Genaue Darstellung (Layout, Farbkodierung grün/rot, Aufklapp-Interaktion) ist einem Gestaltungsdurchgang vorbehalten; diese Spec fixiert **Verträge und Tiefe**, nicht Pixel.
- **R2 (Drill-down-Kalibrierung).** Wie viel je Ebene sofort sichtbar vs. aufklappbar ist, ist feinjustierbar; die vier Ebenen und die Nachvollziehbarkeits-Regel stehen fest.
- **R3 (Erklär-Umfang der KI).** Wie viel Klartext-Erklärung standardmäßig eingeblendet wird (vs. auf Abruf), ist justierbar; die Trennung Fakten-/Erklär-Schicht ist fix.
- **R4 (Zertifikats-Prüf-UX).** Die genaue Darstellung der unabhängigen Zertifikatsprüfung ist offen; dass sie deterministisch und wiederholbar ist, steht fest.

---

## S6.11 — Abnahme (DoD dieser Ebene)

```
DoD(S6) = 1  ⟺
    Ein Operator kann für JEDEN Lauf inspizieren:
        jeden Gate-Report (grün/rot + Begründung + Bezug),
        die vollständige Residuenliste (nichts gefiltert; leer = „geschlossen (∅)"; blocking sichtbar wirksam),
        den Gegenhorizont (die geprüfte Pathologien-Menge),
        den Ledger (append-only, hash-verkettet, prüfbar),
        den Replay-Griff (reproduziert dieselbe Klasse; selbst protokolliert),
        den Abschlussbeweis (kanonische Klassen beider Seiten, aufklärbar warum sie übereinstimmen; Zertifikat prüfbar)
  ∧ jede Anzeige ist bis auf genau ein Motor-Artefakt aufklärbar (Nachvollziehbarkeits-Regel §S6.8);
        nichts Angezeigtes ist unbelegt
  ∧ Fakten-Schicht (Motor, autoritativ) und Erklär-Schicht (KI, markiert, nie autoritativ) sind nie verwechselbar
  ∧ die Inspektion ist strikt read-only: KEINE Handhabe ändert/verbirgt/erzwingt ein Urteil,
        bearbeitet den Ledger oder setzt eine Kennzahl als Schwelle (§S6.7; V1/V2/V7)
  ∧ Kopplung an S3 (Prüf-Fläche), S1 (DocG-*/DocResidue/RoundTrip) und S4 (Audit-Gegenstück) ist konsistent
  ∧ Engine-DoD, DoD(S3), DoD(S1), DoD(S4) bleiben unberührt
```

---

## S6.12 — Anschluss

S6 schließt die Audit-Ebene: Alles, was der Motor tat, ist für einen Menschen lesbar, bis zur Wurzel aufklärbar, unabhängig reproduzierbar und zertifiziert — und nichts davon ist durch den Menschen änderbar. Zusammen mit S4 ist damit die gesamte Reise transparent: Eingang (S4) und Ergebnis (S6).

**Nächste D-Spec (Systemlandkarte §6):** **S7 — Artefakt-Ausgabe je Domäne** — wie das geschlossene Artefakt entnommen, exportiert und domänen-nativ verwendet wird (für Dokument: `.docx`/`.md` öffnen; content-adressierte Ablage; Byte-Digest vs. Inhaltsklassen-Digest, S1.7). Danach S5, S8, S9, S13, S2, S11, S12 — bis `ProduktDoD=1`.

*Ende der Detail-Spezifikation S6.*
