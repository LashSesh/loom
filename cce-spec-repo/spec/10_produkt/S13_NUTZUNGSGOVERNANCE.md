# CCE — DETAIL-SPEZIFIKATION S13: NUTZUNGS-GOVERNANCE / PRODUKTVERFASSUNG

**Neunte D-Spec.** Die operierende Verfassung des fertigen Produkts: die Verbotsachse auf **Bedienebene**, die KI-Kanzel und der Operator an dieselben Regeln gebunden — das Laufzeit-Gegenstück zur Bau-Verfassung (Bauverfassung Teil 8).

**Status:** Detail-Spezifikation v1.0, auf Bauverfassungs-Tiefe. Vereinigt die bereits in S3–S9 verankerte Governance zu einer operierenden Verfassung. Baut auf: BAUVERFASSUNG Teil 8 (Bau-Governance), S3 (KI-Bindungen S3.4, Cockpit-Invarianten S3.9), S6 (Read-only S6.7), S7 (Ausgabe-Verbote S7.9), S5 (Orchestrierungs-Verbote S5.9), S9 (Persistenz-Verbote S9.9), S8 (Bibliotheks-Verbote S8.8).

---

## S13.0 — Einordnung & der geschlossene Kreis

Die Bauverfassung Teil 8 bindet den **bauenden** Agenten: Er darf **kein** System bauen, das die Verbotsachse V1–V10 verletzt. S13 bindet das **laufende** Produkt: Es darf **nicht in eine Verletzung hinein bedient** werden — weder durch den Operator noch durch die KI.

> **Der geschlossene Kreis:** Dieselben Verbote gelten auf **zwei** Ebenen. Teil 8 sorgt dafür, dass der Bauer kein verletzendes System errichtet (Bauzeit). S13 sorgt dafür, dass das errichtete System nicht in eine Verletzung hinein betrieben werden kann (Laufzeit). Zusammen schließen sie den Kreis: Die Garantien halten **beim Bauen und beim Benutzen**.

Leitsatz:

> *Die harten Garantien unterliegen niemandes Ermessen. Weder die KI noch der Operator kann ein Urteil fälschen, ein Residuum verbergen, eine Kennzahl zum Gate machen oder die Reichweite überschreiten. Wer benutzt, benutzt unter der Verfassung.*

Zwei Grundfesten:

- **S13 vereinigt, es erfindet nicht.** Alle Durchsetzungen sind bereits in S3–S9 verankert. S13 führt sie zu **einer** operierenden Verfassung zusammen — keine aufgesetzte Governance-Schicht (das wäre gegen den Geist von V5), sondern die Benennung dessen, was alle Ebenen bereits erzwingen.
- **Boundedness ist architektonisch, nicht nur Politik.** Die Bindungen (KI ohne Schreibpfad, kein Erzwingen-Knopf, Residuum stets gerendert) sind **baulich unmöglich zu umgehen**, nicht bloß per Regel verboten.

---

## S13.1 — Die Verbotsachse auf Bedienebene (vereinigt)

Jede Bauverfassungs-Prohibition V1–V10 (Bauverfassung §8.4) erhält ihre **Bedienform** und ihren **Durchsetzungsort**:

| # | Verbot | Bedienform (Laufzeit) | Durchsetzungsort |
|---|--------|------------------------|------------------|
| **V1** | kein Score-als-Gate | keine Kennzahl entscheidet je; kein Fortschrittsbalken als Schwelle | S3.5, S6.2/S6.7, S5.6 |
| **V2** | kein stilles Residuum | Residuen stets gerendert (leer = „geschlossen (∅)"); Formatverlust sichtbar; Ref-Konflikt sichtbar; KI-Annahmen sichtbar | S6.3, S7.4, S9.6, S4.3 |
| **V3** | kein Nullpunkt-Durchlauf | Motor-Prüfung, in Inspektion ehrlich gezeigt | Motor, S6 |
| **V4** | kein Boundary ohne Naht | DocG-Seam; in Inspektion gezeigt | S1.4, S6 |
| **V5** | keine Software-Reduktion | die kristalline Semantik (Fiber/Skeleton/zwei Sweeps/Übergabe) bleibt **im Betrieb** erhalten, nicht nur im Bau | ganze Apparatur |
| **V6** | kein Rohimport | Fremdmaterial nur durch H/F-Gate; in Inspektion gezeigt | Motor, S6 |
| **V7** | keine Fail-open-Lücke | **kein** „Trotzdem durchlassen"-Knopf; harte Gates nie übersteuerbar | S3.2.3, S6.7, S5.4 |
| **V8** | keine Prompt-/Prozess-Regression | Determinismus im Betrieb; aufgezeichnete Entscheidungen | S5.7, S5.4 |
| **V9** | kein Prime-Power-als-Atom | Motor-Atomizitätsregel; in Inspektion gezeigt | Motor, S6 |
| **V10** | keine Überreichweiten-Behauptung | Claim-Schranke auf KI-Ausgabe **und** in aller Produkt-Text/Doku | S3.4.3, S13.4 |

Diese Tabelle ist der **eine Ort**, an dem sichtbar wird, dass jedes Bau-Verbot ein Laufzeit-Gegenstück hat und wo es erzwungen wird.

---

## S13.2 — Die KI-Kanzel, verfassungsgebunden (das Herz)

S13 fasst die Bindungen der KI (S3.4) als Verfassungsrecht des Produkts:

- **Die KI ist ein gebundener Dolmetscher, nie eine Autorität.** Sie **formt** Wünsche und **erklärt** Motor-Ausgaben — autonom, aber nie autoritativ.
- **Kein Schreibpfad (architektonisch).** Sie hat **keinen** Schreibzugriff auf Gate-Urteile, Residuen oder den Ledger. Fakten fließen Motor→Anzeige; die KI liest und erzählt (S3.4.3, S6.0).
- **Kann nie fälschen.** Sie kann **kein** Gate als bestanden darstellen, das rot ist; **kein** Residuum verbergen; **keine** Kennzahl zum Gate machen; **keine** Reichweite überschreiten. Diese Unmöglichkeiten sind baulich, nicht bloß per Regel.
- **Jede materielle Aktion nur nach Bestätigung.** Wunsch bestätigen, Lauf starten, gate-relevant entscheiden, exportieren — stets explizite Operator-Bestätigung (R-Plan-3, S3.4.2).
- **Außerhalb des Abschlusspfads.** Die KI ist nichtdeterministisch und liegt **vor** der Bestätigungsgrenze; die Replay-Garantie hängt nie an ihr (S3.1.3, S4.5).

Das ist die Laufzeit-Form von „die KI-Konsole ist by design begrenzt".

---

## S13.3 — Die begrenzte Autorität des Operators

Auch der **Mensch** operiert **unter** der Verfassung — die harten Garantien sind auch seinem Ermessen entzogen:

- **Ermessen, wo der Korpus es zulässt.** Der Operator entscheidet Ermessens-Gates (S5.4) mit vollem Kontext; diese Entscheidungen werden aufgezeichnet (S5.4).
- **Keine Macht über die harten Garantien.** Der Operator kann **kein** hartes Gate übersteuern, **kein** Urteil ändern, **kein** Residuum verbergen, **kein** Zertifikat fälschen, **keinen** Ledger bearbeiten (S6.7, S7.9, S9.9).
- **Autorität im Ermessensraum, nicht über die Garantien.** Der Operator hat echte Entscheidungsmacht — aber sie endet an den harten Gates und den Prohibitionen. Die Garantien stehen über dem Operator, nicht zu seiner Disposition.

Damit ist ausgesprochen: **Die Garantien unterliegen niemandes Ermessen** — nicht der KI, nicht dem Menschen.

---

## S13.4 — Die Claim-Schranke im Betrieb (V10 / INV-14)

Besondere Strenge, in Treue zur Reichweiten-Ehrlichkeit des Korpus (Bauverfassung BCIK A8, P12):

- **Kein Reichweiten-Drift.** Das laufende Produkt macht **keine** Behauptung über Physik, Riemann-Hypothese, `π/ζ`-als-Operator o. Ä. jenseits der im Korpus zugelassenen Reichweite.
- **Auf KI-Ausgabe erzwungen.** Ein Ausgabefilter hält die KI-Kanzel an der Schranke (S3.4.3, Bauverfassung INV-14).
- **Auch in Doku und UI-Text.** Dieselbe Schranke gilt für alle Produkt-Texte, Erklärungen und die Operator-Dokumentation (S12) — nicht nur für die KI im Gespräch.

---

## S13.5 — Ehrlichkeit als Verfassungsprinzip (im Betrieb)

Das laufende Produkt **täuscht nie Abschluss vor** (Bauverfassung P12):

- **Kein vorgetäuschter Abschluss.** Wo ein Lauf nicht schließt, wird es mit Grund gesagt (S5.6).
- **Kein vorgetäuschter Wunsch.** Wo ein Wunsch nicht erfassbar ist, wird es gesagt (S4.7).
- **Kein vorgetäuschter Erhalt.** Wo ein Format Bedeutung verliert, wird es gesagt (S7.4).
- **Keine vorgetäuschte Reichweite.** Wo die Reichweite überschritten würde, wird nicht behauptet (§S13.4).

Die Ehrlichkeit des Produkts ist **verfassungsmäßig**, nicht zufällig — ein benannter Nicht-Abschluss ist ein gültiger, würdiger Endzustand, kein Versagen.

---

## S13.6 — Produkt-Invarianten (vereinigt, CI-prüfbar wo möglich)

Die verstreuten Invarianten (COCK-INV S3.9 u. a.) werden zu Produkt-Invarianten zusammengeführt:

- **PROD-INV-1:** Die KI hat keinen Schreibpfad auf Urteile/Residuen/Ledger.
- **PROD-INV-2:** Kein „Trotzdem durchlassen"-Affordance für harte Gates existiert.
- **PROD-INV-3:** Residuen werden stets gerendert; leer = „geschlossen (∅)".
- **PROD-INV-4:** Keine Kennzahl wirkt als Schwelle/Gate.
- **PROD-INV-5:** Die Claim-Schranke wird auf KI-Ausgabe und Produkt-Text erzwungen.
- **PROD-INV-6:** Determinismus des Abschlusspfads im Betrieb; einziger unbestimmter Eingang ist die aufgezeichnete Entscheidung.
- **PROD-INV-7:** Der Ledger ist append-only und prüfbar.
- **PROD-INV-8:** Jede materielle Aktion ist an explizite Operator-Bestätigung gebunden.

Wo baulich (nicht bloß per Test) erzwungen, ist die Invariante durch die Architektur garantiert; wo prüfbar, ist sie CI-Gate.

---

## S13.7 — Governance der Verfassung selbst

Wie Bauverfassung §8.8:

- **Die Verfassung ist ein Monolith im Ledger.** Änderungen sind append-only, begründet, mit Datum und Grund; keine stille Umschrift.
- **Keine Lockerung eines Verbots.** Eine Lockerung von V1–V10 ist **unzulässig**, solange der Korpus sie trägt.
- **Kein Drift über die Zeit.** Über lange Betriebszeiträume bleiben die Bindungen (KI, Operator, Claim-Schranke) invariant — auch unter Nützlichkeitsdruck.

---

## S13.8 — Kopplung: S13 als Vereinigung (aus einem Guss)

S13 fügt **keinen neuen Mechanismus** hinzu; es benennt und vereinigt die Governance, die S3–S9 bereits weben:

- KI-Bindungen ← S3.4; Cockpit-Read-only ← S3.9/S6.7; Ausgabe-Verbote ← S7.9; Orchestrierungs-Verbote ← S5.9; Persistenz-Verbote ← S9.9; Bibliotheks-Verbote ← S8.8; Claim-Schranke ← S3.4.3/Bauverfassung INV-14.

Damit ist die Governance **eingewoben**, nicht aufgesetzt — konsistent mit V5 (keine Reduktion, keine bolt-on-Schicht).

---

## S13.9 — Sichtbare Residuen dieser Spezifikation

Kein stilles Loch:

- **R1 (CI-Abdeckung der Produkt-Invarianten).** Einige PROD-INV sind **architektonisch** garantiert (kein Schreibpfad) und nicht als Test formulierbar; andere sind CI-Gates. Die genaue Aufteilung ist beim Bau zu fixieren.
- **R2 (Verfassungs-Änderungs-UX).** Darstellung des append-only Änderungsprozesses ist offen; das Prinzip (Monolith, begründet, keine Verbots-Lockerung) steht fest.
- **R3 (Claim-Filter-Kalibrierung).** Die genaue Justierung des Ausgabefilters (V10) ist beim Bau zu fixieren; die Schranke selbst (Korpus-Reichweite) steht fest.

---

## S13.10 — Abnahme (DoD dieser Ebene)

```
DoD(S13) = 1  ⟺
    Jedes Verbot V1–V10 hat seine Bedienform mit benanntem Durchsetzungsort (§S13.1)
  ∧ die KI ist architektonisch gebunden: kein Schreibpfad, kann nie fälschen/verbergen/überreichen,
        materielle Aktionen nur nach Bestätigung, außerhalb des Abschlusspfads (§S13.2)
  ∧ der Operator ist gebunden: Ermessen im zulässigen Raum, aber KEINE Macht über die harten Garantien (§S13.3)
  ∧ die Claim-Schranke gilt im Betrieb, auf KI-Ausgabe UND Produkt-Text/Doku (§S13.4)
  ∧ Ehrlichkeit ist verfassungsmäßig: kein vorgetäuschter Abschluss/Wunsch/Erhalt/Reichweite (§S13.5)
  ∧ die Produkt-Invarianten PROD-INV-1..8 sind architektonisch garantiert oder CI-geprüft (§S13.6)
  ∧ die Verfassung ist ein versionierter Monolith; keine Verbots-Lockerung; kein Drift (§S13.7)
  ∧ S13 ist Vereinigung, kein neuer Mechanismus; Governance eingewoben, nicht aufgesetzt (§S13.8, V5)
  ∧ Engine-DoD und DoD(S1/S3/S4/S5/S6/S7/S8/S9) bleiben unberührt
```

---

## S13.11 — Anschluss

S13 schließt die Governance-Ebene: Das fertige Produkt operiert unter einer vereinigten Verfassung — dieselben Verbote wie zur Bauzeit, jetzt zur Laufzeit; die KI und der Operator gebunden; die Garantien niemandes Ermessen unterworfen; die Reichweite ehrlich begrenzt; und die Ehrlichkeit verfassungsmäßig. Der Kreis aus Bau- und Nutzungs-Governance ist geschlossen.

**Nächste D-Spec (Systemlandkarte §6):** **S2 — Operator-, Rollenmodell & verbindliche End-to-End-Reise** — die verbindliche Definition der gesamten Reise (Wunsch → geschlossenes Artefakt) mit ihrer eigenen Abnahme, die die Ebenen S1/S3–S9/S13 zu **einer** durchgängigen Nutzererfahrung zusammenbindet (in der Systemlandkarte §3 skizziert, hier als verbindliche Reise-Ebene ausgearbeitet). Danach S11, S12 und die Produktabnahme S10 — bis `ProduktDoD=1`.

*Ende der Detail-Spezifikation S13.*
