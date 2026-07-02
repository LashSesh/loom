# CCE — DETAIL-SPEZIFIKATION S8: BIBLIOTHEK

**Siebte D-Spec.** Die kuratierte, gepflegte, versionierte Sammlung von Referenz-Cubes, Negativ-Cubes, Vorlagen, Fragmenten und Beispiel-Wünschen — der bewiesene Bestand, der die Engine über die Zeit vertrauenswürdig und regressionsfest hält.

**Status:** Detail-Spezifikation v1.0, auf Bauverfassungs-Tiefe. Baut auf: BAUVERFASSUNG (Referenz-/Negativ-Cubes, DoD §8.7, CI-Gates), S1 (Test-Assets S1.9, Residuen-Vokabular S1.5), S4 (Autorenschaft: Vorlagen/Fragmente S4.6), S5 (Läufe, durch die Assets validiert werden), S9 (Persistenz, die die Bibliothek speichert — später).

---

## S8.0 — Einordnung & Leitsatz

Die Bibliothek ist der **bewiesene Bestand** der Engine: Sie sammelt nicht Beispiele, sondern **Zeugen**. Referenz-Cubes bezeugen, was die Engine korrekt **schließen** kann; Negativ-Cubes bezeugen, was sie korrekt **ablehnen** muss. Ohne diesen Bestand ist die Engine eine Maschine ohne nachgewiesenes Repertoire.

Leitsatz:

> *Die Bibliothek ist das Gedächtnis der Engine für das, was sie kann und was sie nie tun darf — und dieses Gedächtnis ist an die Bau-CI gebunden, sodass keine Änderung es unbemerkt verletzt.*

Zwei Grundfesten:

- **Jeder Bibliotheks-Eintrag ist selbst validiert.** Ein Referenz-Cube, der nicht wirklich schließt, oder ein Negativ-Cube, der nicht wirklich abgelehnt wird, gehört **nicht** in die Bibliothek. Kein unvalidiertes Asset tritt ein (§S8.2).
- **Die Bibliothek ist lebender Beweis, verdrahtet mit CI.** Referenz-Cubes müssen bei **jedem** Bau grün bleiben (korrekt schließen), Negativ-Cubes **rot** (korrekt abgelehnt). Bricht das, bricht der Bau (§S8.3). Das ist der Regressionswächter.

---

## S8.1 — Der Bestand (was in der Bibliothek lebt)

| Asset-Typ | Inhalt | Rolle |
|-----------|--------|-------|
| **Referenz-Cube** | ein perfekter Crystal, `ClosedCCE(C)=1` | **positiver Zeuge**: das kann die Engine schließen |
| **Negativ-Cube** | ein Crystal, der abgelehnt werden **muss**, mit erwartetem Residuum | **negativer Zeuge**: das lehnt die Engine korrekt ab |
| **Vorlage (Template)** | wiederverwendbare Crystal-Struktur (z. B. Drei-Risiken-Memo) | Autoren-Effizienz (S4.6) |
| **Fragment** | wiederverwendbarer Crystal-Baustein (z. B. `Risk`+`Countermeasure`) | Autoren-Effizienz (S4.6) |
| **Beispiel-Wunsch** | natürlichsprachlicher Wunsch, der auf einen wohlgeformten Crystal führt | Einarbeitung/Demonstration |

Alle Assets sind **pro Domäne** organisiert (Dokument zuerst; Graph/Software/Mathe später), inhaltsadressiert und versioniert.

---

## S8.2 — Kuratierung (kein Ablageplatz, sondern gepflegter Bestand)

- **Selbst-Validierung beim Eintritt.** Jedes Asset durchläuft beim Aufnehmen dieselbe **deterministische Validierung** wie jeder Crystal (S4.4): Ein Referenz-Cube wird **tatsächlich durch den Motor geschlossen** (`ClosedCCE=1`); ein Negativ-Cube wird **tatsächlich abgelehnt**, mit dem **erwarteten** Residuum/Grund; eine Vorlage/ein Fragment ist wohlgeformt. Fällt die Validierung durch, wird das Asset **nicht** aufgenommen.
- **Inhaltsadressiert & versioniert.** Jedes Asset trägt seinen Digest; Aktualisierungen erzeugen neue Versionen; alte Versionen bleiben erhalten (content-adressiert, Bauverfassung P10).
- **Herkunft.** Jedes Asset verzeichnet Autor, Zeitpunkt und Validierungsstatus (Kopplung an Ledger/Autorenschicht S4.6).
- **Explizite Stilllegung.** Ein Asset wird nie still gelöscht; Veralten ist eine **explizite, versionierte** Handlung (§S8.8).

---

## S8.3 — Die Bibliothek als lebender Beweis (CI-Kopplung, der Regressionswächter)

Dies ist die Rolle, die die Bibliothek vom „Ordner voller Beispiele" zum **erstklassigen Produktbestandteil** erhebt.

Die Referenz- und Negativ-Cubes sind **nicht** bloß Demonstration — sie sind das **Test-Korpus**, an dem die Bau-DoD hängt (Bauverfassung §8.7):

```
Bei JEDEM Bau / jeder Änderung prüft die CI:
    ∀ Referenz-Cube C_ref :  ClosedCCE(C_ref) = 1        (bleibt GRÜN — schließt weiterhin korrekt)
    ∀ Negativ-Cube  C_neg :  Reject(C_neg) mit erwartetem Residuum   (bleibt ROT — wird weiterhin korrekt abgelehnt)

    Bricht ein Referenz-Cube (schließt nicht mehr)      → BAU ROT
    Bricht ein Negativ-Cube (wird nicht mehr abgelehnt) → BAU ROT
```

**Damit ist die Bibliothek der Regressionswächter:** Sie ist das Mittel, mit dem man **weiß**, dass eine Änderung weder den Abschluss gebrochen noch eine Prohibition geschwächt hat. Eine Änderung, die heimlich ein Gate aufweicht, lässt einen Negativ-Cube durchrutschen → Bau rot, **bevor** es ausgeliefert wird. Das ist die technische Einlösung von „kein 'ah, da fehlt noch was' beim Bauen": Regressionen werden **vor** der Auslieferung gefangen, nicht danach entdeckt.

---

## S8.4 — Negativ-Cubes in der Tiefe (das Immunsystem der Engine)

Weil Negativ-Cubes beweisen, **was die Engine nie tut**, erhalten sie besondere Strenge:

- **Abdeckung der Verbotsachse.** Für **jede** Prohibition V1–V10 (Bauverfassung §8.4) gibt es mindestens einen Negativ-Cube, den die Engine mit dem passenden Grund ablehnt.
- **Abdeckung des Residuen-Vokabulars.** Für **jeden** Domänen-Residuentyp (S1.5: `uncovered_topic`, `unsupported_unit`, `orphan_unit`, `contradiction`, `boundary_crossing_without_seam`, `forbidden_score_field`, `semantic_loss`, `invented_semantic`) gibt es mindestens einen Negativ-Cube, der genau dieses Residuum auslöst.
- **Erwartetes Ergebnis fixiert.** Jeder Negativ-Cube trägt sein **erwartetes** Ablehnungs-Residuum; die CI prüft nicht nur „abgelehnt", sondern „**mit dem richtigen Grund** abgelehnt".
- **Wachsendes Immunsystem.** Wird eine neue Prohibition eingeführt oder ein neuer Fehlermodus entdeckt, entsteht ein **neuer Negativ-Cube** — und die Engine lehnt diesen Fall **für immer danach** ab (er ist ab dann CI-gewacht). So wird jeder einmal erkannte Fehler dauerhaft ausgeschlossen.

---

## S8.5 — Adapter-Parität für die Bibliothek

**Jede** Domäne hat dieselbe **Form** von Bibliothek (Modellbaukasten-Prinzip, Systemlandkarte §5):

- ≥ 1 Referenz-Cube (`ClosedCCE=1`),
- ≥ 1 Negativ-Cube **je** Prohibition und **je** Domänen-Residuentyp,
- einen Satz Vorlagen/Fragmente,
- Beispiel-Wünsche.

`check_adapter_parity` prüft, dass jeder Domänen-Bibliotheks-Slot **vollständig** ist. **Keine Domäne wird mit halb leerer Bibliothek ausgeliefert** — jede bringt denselben vollständigen Satz an Zeugen mit.

---

## S8.6 — Bibliotheks-Operationen

- **Durchsuchen/Auffinden.** Assets nach Domäne, Struktur oder Inhaltsklasse finden; über den Inhaltsklassen-Digest alle Fassungen derselben Bedeutung.
- **Anwenden.** Eine Vorlage/ein Fragment während der Wunscherfassung (S4.6) einsetzen — die KI **schlägt vor**, der Operator **bestätigt** (nie stiller Einsatz).
- **Beitragen.** Ein neues Asset aus einem bestätigten Crystal autorisieren (S4.6); es tritt erst nach Selbst-Validierung (§S8.2) ein.
- **Pflegen.** Aktualisieren/Stilllegen, versioniert; alte Versionen bleiben erhalten.

---

## S8.7 — Kopplung an S1, S4, S5, CI (aus einem Guss)

- **An S1:** Die Bibliothek hält die **S1-Test-Assets** (S1.9) und deckt das **S1-Residuen-Vokabular** (S1.5) mit Negativ-Cubes ab.
- **An S4:** Die **Vorlagen/Fragmente** der Bibliothek speisen die **Autorenschicht** (S4.6); Beiträge laufen durch die S4-Validierung.
- **An S5:** Assets werden durch die **Orchestrierung** (S5) real durch den Motor geführt, um ihre Validität nachzuweisen (Referenz schließt, Negativ wird abgelehnt).
- **An CI/Bauverfassung:** Die Cubes sind an die **Bau-DoD** (§8.7) und die CI-Gates gebunden (§S8.3) — der Regressionswächter.

---

## S8.8 — Read-only-/Kuratierungs-Garantien (Verbote auf Bibliotheksebene)

Abwesende Handhaben:

- **kein „unvalidiertes Asset aufnehmen"** — jedes Asset ist beim Eintritt validiert (§S8.2).
- **kein „falscher Referenz-Cube"** — ein als Referenz aufgenommener Crystal, der nicht schließt, wird abgewiesen.
- **kein „zahnloser Negativ-Cube"** — ein Negativ-Cube, der nicht (mit erwartetem Grund) abgelehnt wird, wird abgewiesen.
- **kein stilles Löschen** — Stilllegung ist explizit und versioniert; alte Versionen bleiben erhalten.
- **kein stiller Vorlagen-Einsatz** — die KI schlägt vor, der Operator bestätigt (S4.6).

---

## S8.9 — Sichtbare Residuen dieser Spezifikation

Kein stilles Loch:

- **R1 (Speicherformat/-ort).** Die konkrete Ablage der Bibliothek gehört zu S9 (Persistenz); hier steht der **Bestand und die Disziplin**, nicht der Speicher.
- **R2 (Anfangsbreite Dokument-Bibliothek).** Wie viele Referenz-Cubes die Dokument-Domäne zum Start umfasst, ist zu wählen (Minimum: der Drei-Risiken-Referenz-Cube + je ein Negativ-Cube pro Prohibition/Residuentyp, S1.9).
- **R3 (Kuratierungs-UX).** Darstellung von Durchsuchen/Beitragen/Pflegen ist einem Gestaltungsdurchgang vorbehalten; die Operationen (§S8.6) stehen fest.
- **R4 (Versionierungs-Politik).** Genaue Regeln für Versionsketten/Stilllegung sind zu fixieren; das Prinzip (inhaltsadressiert, alte Versionen erhalten, explizite Stilllegung) steht fest.

---

## S8.10 — Abnahme (DoD dieser Ebene)

```
DoD(S8) = 1  ⟺
    Die Bibliothek hält pro Domäne: Referenz-Cubes, Negativ-Cubes, Vorlagen, Fragmente, Beispiel-Wünsche
  ∧ JEDES Asset ist beim Eintritt validiert: Referenz schließt (ClosedCCE=1), Negativ wird mit erwartetem
        Residuum abgelehnt, Vorlage/Fragment ist wohlgeformt (§S8.2)
  ∧ die Bibliothek ist an CI gebunden: Referenz-Cubes bleiben grün, Negativ-Cubes bleiben rot,
        Bruch ⇒ Bau rot (§S8.3, der Regressionswächter)
  ∧ Negativ-Cubes decken JEDE Prohibition V1–V10 und JEDEN Domänen-Residuentyp ab, mit erwartetem Grund (§S8.4)
  ∧ jede Domäne hat einen VOLLSTÄNDIGEN Bibliotheks-Slot (§S8.5, Adapter-Parität, check_adapter_parity grün)
  ∧ Assets sind inhaltsadressiert, versioniert, mit Herkunft; Stilllegung explizit, nie stilles Löschen
  ∧ Vorlagen/Fragmente speisen die Autorenschicht (S4.6); Einsatz nur nach Operator-Bestätigung
  ∧ Engine-DoD, DoD(S1/S3/S4/S5/S6/S7) bleiben unberührt
```

---

## S8.11 — Anschluss

S8 schließt die Bibliotheks-Ebene: Die Engine hat einen bewiesenen, kuratierten, versionierten Bestand an Zeugen — positiv (was sie schließt) und negativ (was sie ablehnt) — und dieser Bestand ist an die Bau-CI gebunden, sodass keine Regression unbemerkt bleibt. Das ist das technische Rückgrat der Vollständigkeits-Garantie.

**Nächste D-Spec (Systemlandkarte §6):** **S9 — Persistenz, Arbeitsbereiche & Projektverwaltung** — wo Läufe, Kristalle, Artefakte, Ledger und die Bibliothek über Sitzungen leben; Workspaces, inhaltsadressierter Store, Versionierung, sync-fähig ausgelegt (in S3.7/S7.5 berührt, hier als Persistenz-Ebene ausgearbeitet). Danach S13, S2, S11, S12 — bis `ProduktDoD=1`.

*Ende der Detail-Spezifikation S8.*
