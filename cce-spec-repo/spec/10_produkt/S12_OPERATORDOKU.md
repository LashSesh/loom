# CCE — DETAIL-SPEZIFIKATION S12: OPERATOR-DOKUMENTATION & ONBOARDING

**Zwölfte D-Spec.** Das Handbuch fürs *Benutzen* (nicht fürs Bauen) und der Onboarding-Pfad — die Brücke von „funktioniert im Prinzip" zu „ein Mensch bedient es sicher".

**Status:** Detail-Spezifikation v1.0, auf Bauverfassungs-Tiefe. Baut auf: S2.6 (mentales Modell), S2.2 (Reise), S3 (Cockpit), S4 (Wunsch), S5 (HITL), S6 (Inspektion), S7 (Artefakt), S8 (Bibliothek), S11 (Betrieb), S13.4 (Claim-Schranke, die auch für Doku gilt).

---

## S12.0 — Einordnung & Leitsatz

Ein fertiges Produkt braucht, dass der Operator es **lernen und benutzen** kann. S12 liefert **keine** Entwickler-Doku (die Bauverfassung, für den Coding-Agenten), sondern **Operator-Doku**: wie man die laufende App bedient. Es ist das letzte menschenzugewandte Stück.

Leitsatz:

> *Die Dokumentation steht selbst unter der Verfassung: Sie überschreitet nie die Reichweite, verspricht nie eine Garantie, die das Produkt nicht hält — und sie lehrt die Ehrlichkeit des Produkts (Residuen sichtbar, Nicht-Abschluss gültig), nicht nur seine Knöpfe.*

Zwei Grundfesten:

- **Die Doku ist unter S13.4.** Sie macht **keine** Behauptung über Physik/RH/`π,ζ` jenseits der Korpus-Reichweite; sie ist ehrlich über das, was das Produkt tut **und** nicht tut; sie widerspricht nie dem tatsächlichen Verhalten.
- **Sie lehrt den Ethos, nicht bloß die Bedienung.** Warum Residuen sichtbar sind, warum ein benannter Nicht-Abschluss ein würdiger Endzustand ist, warum die KI Führer und nicht Richter ist — der Operator versteht das *Warum*, nicht nur das *Wie*.

---

## S12.1 — Aufbau der Operator-Dokumentation

| Abschnitt | Inhalt | Quelle |
|-----------|--------|--------|
| **Das mentale Modell** | Wunsch → Crystal → Lauf → Artefakt; KI ist Führer, nicht Richter; Residuen sind sichtbare Wahrheit; Replay ist die unabhängige Prüfung | S2.6 |
| **Die Reise, Schritt für Schritt** | Öffnen → Wünschen → Bestätigen → Laufen → Prüfen → Entnehmen → Wiederholen | S2.2 |
| **Wünsche formulieren** | was einen Wunsch wohlformbar macht; wie die KI hilft; die Annahmen-Liste lesen; wann antworten vs. Default annehmen | S4 |
| **Gates und Residuen lesen** | was ein Gate-Urteil bedeutet; was ein Residuum ist; „geschlossen (∅)"; was „blocking" heißt; Aufklappen bis zur Wurzel | S6 |
| **Ermessens-Entscheidungen treffen** | welcher Kontext gezeigt wird; dass harte Gates nicht übersteuerbar sind; dass Entscheidungen aufgezeichnet werden | S5.4 |
| **Artefakte entnehmen und verwenden** | Öffnen/Exportieren; Datei vs. Bedeutung (die zwei Digests in Klartext); äußere Bearbeitung bricht das Zertifikat nachweisbar | S7 |
| **Replay: selbst nachprüfen** | wie man einen Lauf reproduziert und warum das das Ergebnis beweist | S6.5 |
| **Wenn etwas nicht schließt** | einen benannten Nicht-Abschluss lesen; dass er ehrlich und gültig ist; was als Nächstes | S2.5 |
| **Bibliothek und Vorlagen** | Vorlagen/Beispiele nutzen | S8 |
| **Einstellungen** | Konfiguration in-App (Workspace, KI-Zugang, Export, Sync) | S11.3 |

---

## S12.2 — Der Onboarding-Pfad (Lernen durch Tun)

- **Der erste Lauf, geführt.** Ein neuer Operator wird durch seinen **ersten Durchlauf** geführt — anhand des Saat-Referenz-Cubes (Drei-Risiken-Memo) oder eines ersten eigenen Dokument-Wunsches.
- **Schnell zum ersten geschlossenen Artefakt.** Das Onboarding bringt den Operator **zügig** zu seinem ersten geschlossenen `.docx` — er lernt durch Tun, nicht durch Lesen.
- **Die Saat-Bibliothek liefert das Material** (S11.6): ab Installation ist ein funktionierendes Beispiel da.
- **Der Ethos gleich mit.** Schon im ersten Lauf sieht der Operator ein Residuum („geschlossen (∅)"), einen Gate-Report und den Abschlussbeweis — und lernt so von Anfang an, was das Produkt sichtbar macht.

---

## S12.3 — Die Ehrlichkeit der Dokumentation (unter S13.4)

Ein besonderer Punkt: Die Doku ist **unter der Verfassung**.

- **Kein Überschreiten der Reichweite.** Keine Behauptung über Physik/RH/`π,ζ` als Operator jenseits des Korpus (S13.4).
- **Kein Versprechen jenseits der Garantie.** Die Doku verspricht **nur**, was das Produkt hält — kein „macht immer alles", sondern die tatsächlichen, geprüften Garantien.
- **Kein Widerspruch zum Verhalten.** Was die Doku sagt, stimmt mit dem tatsächlichen Produktverhalten überein; wo das Produkt einen Fall ablehnt, sagt die Doku, dass und warum.
- **Sie erklärt die eigene Ehrlichkeit.** Die Doku macht dem Operator die Prinzipien verständlich (Residuen sichtbar, Nicht-Abschluss gültig, KI gebunden), sodass er das Verhalten des Produkts **nicht als Mangel, sondern als Verlässlichkeit** liest.

---

## S12.4 — In-App-Hilfe und die KI-Kanzel als Doku-Fläche

- **Zwei konsistente Flächen.** Kontextuelle **In-App-Hilfe** (an den vier Cockpit-Flächen, S3.2) **und** ein eigenständiges **Operator-Handbuch**.
- **Die KI-Kanzel ist selbst eine Doku-Fläche.** Sie erklärt Gates/Residuen/Abschluss in Klartext (S3.4) — aber **gebunden** (S13.4): Sie kann nicht überreichen, nicht fälschen, nicht verbergen.
- **Konsistenz.** Doku, In-App-Hilfe und KI-Erklärung sagen **dasselbe** und stehen alle unter der Verfassung — der Operator bekommt nie widersprüchliche Auskunft, und keine Fläche beschönigt.

---

## S12.5 — Was die Doku bewusst NICHT tut

- **Kein Bauen lehren.** Das Errichten von `cce/` ist die Bauverfassung (für den Coding-Agenten), nicht die Operator-Doku.
- **Keine Konsole zeigen.** Der Operator braucht sie nie (S11); die Doku führt keine Terminal-Umwege ein.
- **Kein Überclaim.** Keine Reichweite jenseits des Korpus (S13.4).

Diese Grenze ist selbst dokumentiert — der Operator weiß, wofür die Doku da ist und wofür nicht.

---

## S12.6 — Adapter-Parität für die Dokumentation

Jede Domäne bekommt beim Hinzufügen **dieselbe** Doku-Struktur (§S12.1): Die **Reise ist invariant**; nur die domänenspezifische Wunsch-Grammatik (S1.2) und der Artefakt-Typ (S7) unterscheiden sich. Die Doku skaliert mit Domänen **ohne Umstrukturierung** — ein Doku-Gerüst für alle Domänen.

---

## S12.7 — Read-only-/Ehrlichkeits-Garantien (Verbote auf Doku-Ebene)

Abwesende Handhaben:

- **kein Überclaim** — keine Behauptung jenseits der Korpus-Reichweite (S13.4).
- **kein Versprechen jenseits der Garantie** — die Doku verspricht nur Gehaltenes.
- **kein Konsolen-Umweg** — die Doku lehrt keine Terminal-Bedienung.
- **kein Widerspruch zum Verhalten** — Doku und Produktverhalten stimmen überein.

---

## S12.8 — Sichtbare Residuen dieser Spezifikation

Kein stilles Loch:

- **R1 (Doku-Medium).** Ob In-App, Web oder PDF-Handbuch (oder alle) ist zu wählen; die Struktur (§S12.1) und die Ehrlichkeitsbindung stehen fest.
- **R2 (Onboarding-UX).** Der genaue geführte Erstlauf ist einem Gestaltungsdurchgang vorbehalten; das Prinzip (Lernen durch Tun, zum ersten geschlossenen Artefakt) steht fest.
- **R3 (Lokalisierung).** Die Doku ist zunächst auf Deutsch (dieser Operator); Mehrsprachigkeit ist eine spätere, additive Ebene.
- **R4 (Kontexthilfe-Tiefe).** Wie tief die In-App-Hilfe je Fläche geht, ist justierbar; die Konsistenz mit Handbuch und KI-Kanzel steht fest.

---

## S12.9 — Abnahme (DoD dieser Ebene)

```
DoD(S12) = 1  ⟺
    Die Operator-Doku deckt ab: mentales Modell, Reise Schritt für Schritt, Wunsch-Formulierung,
        Gates/Residuen lesen, Ermessens-Entscheidungen, Artefakt-Entnahme/Verwendung, Replay,
        Fehl-Reisen, Bibliothek, Einstellungen (§S12.1)
  ∧ ein Onboarding-Pfad bringt einen neuen Operator durch Tun zu seinem ersten geschlossenen Artefakt (§S12.2)
  ∧ die Doku steht unter der Claim-Schranke: ehrlich, kein Überclaim, kein Versprechen jenseits der Garantie,
        kein Widerspruch zum Verhalten (§S12.3, S13.4)
  ∧ die Doku lehrt den Ethos des Produkts (Residuen sichtbar, Nicht-Abschluss gültig, KI gebunden), nicht nur Knöpfe
  ∧ In-App-Hilfe, Handbuch und KI-Kanzel sind konsistent und alle gebunden (§S12.4)
  ∧ die Doku lehrt kein Bauen, keine Konsole, keinen Überclaim (§S12.5)
  ∧ die Doku-Struktur skaliert je Domäne ohne Umstrukturierung (§S12.6, Adapter-Parität)
  ∧ Engine-DoD und DoD(S1/S2/S3/S4/S5/S6/S7/S8/S9/S11/S13) bleiben unberührt
```

---

## S12.10 — Anschluss

S12 schließt die Dokumentations-Ebene: Der Operator kann das Produkt lernen und sicher bedienen — geführt durch einen Erstlauf, gestützt durch konsistente In-App-Hilfe, Handbuch und gebundene KI-Kanzel, und geleitet von einer Doku, die selbst unter der Verfassung steht und den Ethos des Produkts vermittelt.

**Damit sind alle zwölf D-Specs geschrieben.** Es bleibt der **Schlussstein: S10 — Produktabnahme** — der die gesamte `ProduktDoD=1` über alle Ebenen prüft und das ganze Werk auf Papier schließt. Danach übernimmt der Coding-Agent (Bauverfassung Teil 9) und baut aus einem Guss.

*Ende der Detail-Spezifikation S12.*
