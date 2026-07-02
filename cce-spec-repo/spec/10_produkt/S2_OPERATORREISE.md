# CCE — DETAIL-SPEZIFIKATION S2: OPERATOR-, ROLLENMODELL & VERBINDLICHE END-TO-END-REISE

**Zehnte D-Spec.** Die verbindliche Definition der gesamten Reise (Wunsch → geschlossenes Artefakt) mit eigener Abnahme — die Integrations-Ebene, die S1/S3–S9/S13 zu **einer** durchgängigen Nutzererfahrung zusammenbindet.

**Status:** Detail-Spezifikation v1.0, auf Bauverfassungs-Tiefe. Bindet alle bisherigen Ebenen. Baut auf: Systemlandkarte §3 (Reise-Skizze), S1/S3/S4/S5/S6/S7/S8/S9/S13.

---

## S2.0 — Einordnung & Leitsatz

Alle Ebenen sind einzeln spezifiziert. Aber ein **Produkt** ist kein Stapel von Ebenen — es ist **eine kohärente Reise**, die ein Mensch geht. S2 definiert diese Reise als verbindliches Ganzes: die Rollen, den vollständigen Weg vom Wunsch zum geschlossenen Artefakt, **die Nähte zwischen den Ebenen**, und die Abnahme, dass die Reise **als Ganzes** trägt.

Leitsatz:

> *Die Ebenen fallen zu einer Reise zusammen, nicht bloß nebeneinander. Jede Naht zwischen zwei Ebenen ist benannt und trägt ihre Garantie über — es gibt keinen Übergang, an dem eine Garantie verloren geht.*

Zwei Grundfesten:

- **S2 ist die Integrations-Ebene.** Hier wird „aus einem Guss" **auf Erfahrungsebene** geprüft — nicht Ebene für Ebene behauptet, sondern als durchgängige Reise nachgewiesen (§S2.7).
- **Die Reise hat Naht-Struktur (reflexiv).** Der Weg vom Wunsch zum Artefakt ist selbst eine **Komposition von Nähten** (§S2.3) — mit derselben Struktur wie die Kristall-Übergabe an Separatoren im Motor. V4/P5 (kein Boundary-Crossing ohne Naht) gilt damit auch auf **Reise-Ebene**: kein Ebenenübergang ohne benannte Naht.

---

## S2.1 — Das Rollenmodell

Drei Rollen, für den Einzelnutzer zusammenfallend, später trennbar (R-Plan-4, S9.2):

| Rolle | Tut | Ebenen |
|-------|-----|--------|
| **Operator** | formuliert Wünsche, führt Läufe, inspiziert, entnimmt Artefakte | S3–S7 |
| **Autor** | autorisiert Vorlagen, Fragmente, Bibliotheks-Assets (Cubes) | S4.6, S8 |
| **Prüfer** | inspiziert Läufe, Residuen, Ledger zur Nachprüfung | S6 |

- **Für den Einzelnutzer zusammengefallen.** Alle drei Rollen sind **eine Person**; das Rollenmodell ist **vorhanden, aber kollabiert**.
- **Später trennbar ohne Kernänderung.** Weil Workspaces bereits als „Verweismenge + Ledger" modelliert sind (S9.2) und die Autorenschaft bereits validiert eintritt (S8.2), dockt eine spätere Rollentrennung (getrennter Autor/Prüfer) an, ohne den Kern zu ändern.
- **Jede Rolle unter der Verfassung.** Auch Autor und Prüfer operieren **unter** S13 — keine Rolle hat Macht über die harten Garantien.

---

## S2.2 — Die verbindliche End-to-End-Reise

Der normative, vollständige Weg — jede Stufe an ihre Ebene gebunden:

```
 ÖFFNEN        native App startet (S3)                         [keine Konsole]
     │
     ▼ Naht 0
 WÜNSCHEN      Klartext-Wunsch; KI formt in DocCrystal;         (S4)
     │         Annahmen-Liste sichtbar; Motor validiert
     ▼ Naht 1 — BESTÄTIGUNGSGRENZE
 BESTÄTIGEN    Operator bestätigt; Crystal eingefroren,         (S4.5)
     │         content-adressiert; RunDescriptor beginnt        [nichtdet. → det.]
     ▼ Naht 2
 LAUFEN        Motor fährt encode→…→equivalent, deterministisch; (S5, S1.3)
     │         an Ermessens-Gates entscheidet der Operator (aufgezeichnet)
     ▼ Naht 3
 PRÜFEN        Gates/Residuen/Ledger/Replay/Abschlussbeweis      (S6)
     │         sichtbar, aufklärbar, read-only
     ▼ Naht 4
 ENTNEHMEN     Artefakt (.docx/.md) öffnen/exportieren;          (S7)
     │         zwei Digests, Herkunft, Zertifikat
     ▼ Naht 5
 ABLEGEN /     Artefakt content-adressiert im Workspace;         (S9, S6.5)
 WIEDERHOLEN   Replay reproduziert dieselbe Klasse
     │
     └── durchgängig unter S13 (Governance), mit S1 (Domäne) und S8 (Bibliothek)
```

Das ist **die** Reise — nicht eine von vielen möglichen, sondern die verbindliche Definition der Nutzererfahrung.

---

## S2.3 — Die Nähte zwischen den Ebenen (der Integrations-Kern)

Was S2 einzigartig beiträgt: Es benennt die **Übergaben** und garantiert Kontinuität. Jede Naht ist ein Separator im Sinne des Motors — sie trägt einen Grenzvertrag:

| Naht | Von → Nach | Übergabe | Garantie, die überträgt |
|------|-----------|----------|--------------------------|
| **0** | Öffnen → Wünschen | leerer Zustand → Wunsch-Fläche | Cockpit-Zustandsmaschine deterministisch (S3.3) |
| **1** | Wünschen → Bestätigen | Draft-Crystal + Annahmen → bestätigter Crystal | **Bestätigungsgrenze**: nichtdet. KI → det. Motor; Annahmen sichtbar (S4.5) |
| **2** | Bestätigen → Laufen | bestätigter Crystal → Lauf | content-adressiert, fester RunDescriptor, replaybar (S5.5) |
| **3** | Laufen → Prüfen | Gate-Reports/Residuen → Anzeige | Fakten fließen Motor→Anzeige; read-only (S6.0) |
| **4** | Prüfen → Entnehmen | geschlossener Crystal → Artefakt | zwei Digests, Herkunft, Zertifikat (S7.1/S7.2) |
| **5** | Entnehmen → Ablegen | Artefakt → Workspace/Replay | content-adressiert; Klasse reproduzierbar (S9, S6.5) |

**Die Reise ist die Komposition dieser Nähte.** Weil jede Naht ihre Garantie über den Übergang trägt, geht **keine** Garantie zwischen zwei Ebenen verloren. Das ist die Reise-Ebenen-Form von „kein Boundary-Crossing ohne Naht" (V4/P5) — und strukturell dieselbe Naht-Übergabe wie die Kristall-Übergabe an Separatoren im Motor (Bauverfassung Teil 5). Der Weg des Menschen und der Weg des Kristalls haben **dieselbe Naht-Struktur**.

---

## S2.4 — Kohärenz-Garantie (aus einem Guss, geprüft)

S2 stellt sicher und die Abnahme prüft, dass die Reise **ein kohärentes Ganzes** ist:

- **Jede Naht ist definiert** (§S2.3) — kein undefinierter Übergang.
- **Jede Übergabe erhält die Garantien** (Determinismus, Residuen-Sichtbarkeit, Herkunft) — kein Verlust an einer Naht.
- **Keine Lücke zwischen Ebenen** — der Ausgang einer Ebene ist exakt der Eingang der nächsten (Crystal, RunDescriptor, GateReport, Artefakt).
- **Keine Doppelarbeit, kein Widerspruch** — jede Ebene tut ihr Teil einmal; keine zwei Ebenen erzeugen dasselbe unterschiedlich.

Damit ist „aus einem Guss" nicht Behauptung, sondern **geprüfte Eigenschaft der Reise**.

---

## S2.5 — Die Fehl-Reisen (ehrliche, vollständige Wege zu benanntem Nicht-Abschluss)

Die Reise ist nicht nur der glückliche Pfad. S2 definiert die **ehrlichen Fehl-Reisen** — jede ein vollständiger, würdiger Weg zu einem benannten Nicht-Abschluss, keine Sackgasse:

- **Wunsch nicht erfassbar** → benannte Grenze/Residuum in der Wunsch-Fläche (S4.7).
- **Lauf abgelehnt** → `ABGELEHNT` mit benanntem Grund, aufgezeichnet, selbst replaybar (S5.6).
- **Formatverlust beim Export** → sichtbares `format_loss`-Residuum; bewusste Operator-Entscheidung (S7.4).

**Der Operator endet immer in einem lesbaren Zustand:** entweder geschlossenes Artefakt **oder** benannter Nicht-Abschluss mit Grund. Nie ein stummes Scheitern, nie ein illegibler Zustand.

---

## S2.6 — Das mentale Modell des Operators

Was ein Operator verstehen muss, um das Produkt zu bedienen (das konzeptuelle Gerüst, das die Reise voraussetzt; ausführliche Doku ist S12):

- **Wunsch → Crystal → Lauf → Artefakt.** Der Wunsch wird zu einem präzisen Kern, der geschlossen und materialisiert wird.
- **Die KI ist Führer, nicht Richter.** Sie hilft formen und erklären; die Urteile kommen vom Motor.
- **Residuen sind sichtbare Wahrheit.** Was offen blieb, steht da — leer heißt geschlossen.
- **Replay ist die unabhängige Prüfung.** Man muss nicht glauben; man reproduziert.

Vier Sätze — mehr braucht ein Operator konzeptuell nicht, um die Reise zu gehen.

---

## S2.7 — Die vollständige Reise-Abnahme (der Produkt-Kerntest)

Parallel zum Motor-Kerntest (Bauverfassung VC1⁺) definiert S2 den **Produkt-Kerntest** — der Zeuge, dass die **ganze Reise** trägt, nicht nur jede Ebene:

```
Produkt-Kerntest:
    Ein Operator geht in der Dokument-Domäne, OHNE Konsole:
        Wunsch → bestätigter Crystal → geschlossenes .docx
        → im Cockpit sichtbar geprüft → entnommen → repliziert (gleiche Klasse)
    ∧ an JEDER Naht (0–5) hält die zugehörige Garantie
    ∧ die Fehl-Reisen enden in benanntem, lesbarem Nicht-Abschluss
```

Ist dieser Test grün, ist die Reise **als Ganzes** nachgewiesen — die Integration der Ebenen, nicht nur ihre Einzelkorrektheit.

---

## S2.8 — Kopplung an alle Ebenen (Vollständigkeit der Bindung)

S2 ist die Integrations-Ebene und referenziert **jede** andere:

- S1 (Domänen-Grammatik) · S3 (Cockpit/Kanzel) · S4 (Wunscherfassung/Bestätigungsgrenze) · S5 (Orchestrierung/HITL) · S6 (Inspektion) · S7 (Ausgabe) · S8 (Bibliothek) · S9 (Persistenz) · S13 (Governance).

Jede Naht (§S2.3) bindet zwei dieser Ebenen; zusammen decken die Nähte alle Übergänge ab. **Keine Ebene bleibt unverbunden.**

---

## S2.9 — Read-only-/Kohärenz-Garantien (Verbote auf Reise-Ebene)

Abwesende Handhaben:

- **keine Lücke zwischen Ebenen** — kein Übergang ohne definierte Naht (§S2.3, V4/P5 auf Reise-Ebene).
- **kein Naht-Übergang ohne Garantie** — keine Übergabe verliert ihre Garantie.
- **kein illegibler Endzustand** — der Operator endet stets geschlossen oder in benanntem Nicht-Abschluss (§S2.5).
- **keine Rolle über den Garantien** — auch Autor/Prüfer operieren unter S13 (§S2.1).

---

## S2.10 — Sichtbare Residuen dieser Spezifikation

Kein stilles Loch:

- **R1 (Rollentrennungs-UX).** Die konkrete UX getrennter Rollen ist zurückgestellt (Einzelnutzer jetzt); das Modell (drei Rollen, kollabiert, trennbar) steht fest.
- **R2 (Reise-UX-Feinschliff).** Übergänge, Führung, Mikro-Interaktionen sind einem Gestaltungsdurchgang vorbehalten; die Nähte und ihre Garantien stehen fest.
- **R3 (Onboarding-Tiefe).** Wie das mentale Modell (§S2.6) vermittelt wird, gehört zu S12; hier nur das Modell selbst.

---

## S2.11 — Abnahme (DoD dieser Ebene)

```
DoD(S2) = 1  ⟺
    Die Rollen sind definiert (Operator/Autor/Prüfer), für Einzelnutzer kollabiert, später ohne Kernänderung trennbar
  ∧ die verbindliche Reise ist definiert, mit JEDER Naht (0–5) benannt und mit übertragender Garantie (§S2.2/S2.3)
  ∧ die Reise ist ein kohärentes Ganzes: keine Lücke, kein Garantieverlust, keine Doppelarbeit (§S2.4, aus einem Guss)
  ∧ der PRODUKT-KERNTEST ist grün: Wunsch → geschlossenes .docx → entnommen → repliziert, alle Nähte tragen (§S2.7)
  ∧ die Fehl-Reisen enden in benanntem, lesbarem Nicht-Abschluss (§S2.5); nie ein illegibler Zustand
  ∧ das mentale Modell des Operators ist benannt (§S2.6)
  ∧ jede Ebene S1/S3–S9/S13 ist in die Reise gebunden (§S2.8); keine bleibt unverbunden
  ∧ jede Rolle operiert unter der Verfassung S13 (§S2.1)
  ∧ Engine-DoD und DoD(S1/S3/S4/S5/S6/S7/S8/S9/S13) bleiben unberührt
```

---

## S2.12 — Anschluss

S2 schließt die Integrations-Ebene: Die Ebenen fallen zu **einer** Reise zusammen; jede Naht ist benannt und trägt ihre Garantie über; der Produkt-Kerntest bezeugt das Ganze; Fehl-Reisen enden lesbar. Die Reise des Menschen hat dieselbe Naht-Struktur wie die Kristall-Übergabe im Motor — Bedienung und Werk sind strukturgleich.

**Nächste D-Spec (Systemlandkarte §6):** **S11 — Auslieferung, Paketierung & Betrieb** — wie das fertige Produkt als installierbare native App ausgeliefert, konfiguriert und aktualisiert wird, ohne Konsole (in S3.1.1/R-Plan-1 berührt, hier als Auslieferungs-Ebene ausgearbeitet). Danach S12 und die Produktabnahme S10 — bis `ProduktDoD=1`.

*Ende der Detail-Spezifikation S2.*
