# CCE — DETAIL-SPEZIFIKATION S14: KERN-ERWEITERUNGS-PROTOKOLL (VERTIKALE EXPANSION)

**Der letzte Baustein.** Wie die Fähigkeit *und* Kapazität des Kerns selbst friktionslos wächst — ohne je den Abschluss oder eine Invariante zu brechen. Das vertikale Gegenstück zum `DomainAdapter` (horizontal).

**Status:** Detail-Spezifikation v1.0, auf Bauverfassungs-Tiefe. Schließt die vertikale Achse und damit die Plattform-DoD auf Papier. Baut auf: BAUVERFASSUNG (Objekt-/Operatormodell, Metatheorie, INV-1..14, V1–V10, Crate-DAG Teil 6), S1.8 (DomainAdapter, das horizontale Analogon), S8.3 (Regressionswächter), S13.7 (keine Verbots-Lockerung).

---

## S14.0 — Einordnung & die zwei Achsen, vereinigt

Es gibt zwei Erweiterungsachsen, und sie sind **symmetrisch** durch dieselbe Disziplin garantiert:

| Achse | Was wächst | Mechanismus | Beispiel |
|-------|-----------|-------------|----------|
| **Horizontal (Breite)** | eine neue **Domäne** (neue Art, die bestehenden Kern-Objekte zu instanziieren) | `DomainAdapter` (S1.8) | „Software"-Domäne hinzufügen |
| **Vertikal (Tiefe)** | eine neue **Kern-Fähigkeit/-Kapazität** (die *allen* Domänen zur Verfügung steht) | `CoreExtension` (diese Spec) | neue Gate-Klasse jenseits G1–G7 |

Leitsatz:

> *Der Kern wächst nur durch sanktionierte Erweiterungspunkte mit closure-erhaltenden Beweispflichten. Eine Kern-Erweiterung, die ihre Pflichten nicht erfüllt oder einen Referenz-/Negativ-Cube bräche, ist unzulässig — der Abschluss und die Invarianten sind by construction erhalten.*

Zwei Grundfesten:

- **Kein beliebiger Kern-Umbau.** Der Kern lässt sich **nicht** frei ändern, sondern **nur** durch die fixen, vertraglich gebundenen Erweiterungspunkte (§S14.2). Das ist, was vertikale Erweiterung zugleich **friktionslos und sicher** macht: Die Verträge erzwingen die Abschluss-Erhaltung.
- **Ein Wächter schützt alles Dreifache.** Derselbe Regressionswächter (S8.3), der Domänen und Updates schützt, validiert **auch** Kern-Erweiterungen (§S14.4). Eine Erweiterung, die den Abschluss bräche oder eine Prohibition schwächte, fällt **vor** der Aufnahme durch — genau wie eine schlechte Domäne oder ein schlechtes Update.

---

## S14.1 — Horizontal vs. vertikal, präzise

- **Horizontal:** Eine neue Domäne nutzt die **bestehenden** Kern-Objekte (Cell/Seam/Crystal), Operatoren und Gates. Der Kern ist **unverändert**. Mechanismus: `DomainAdapter` (S1.8). Dafür braucht es kein S14 — es ist bereits gelöst.
- **Vertikal:** Eine neue Kern-Fähigkeit fügt einen **neuen Objekt-/Operator-/Gate-/Sweep-Typ** oder **Kapazität** hinzu, den *alle* Domänen dann nutzen können. Der Kern selbst **wächst**. Mechanismus: `CoreExtension`.

Merksatz: **Domäne = neue Instanz des Kerns (horizontal). Kern-Erweiterung = neue Fähigkeit des Kerns (vertikal).**

---

## S14.2 — Die sanktionierten Erweiterungspunkte

Der Kern exponiert eine **feste** Menge von Erweiterungspunkten, jeder mit einem Vertrag (Beweispflichten §S14.3):

- **`ObjectExtension` — neuer Objekttyp.** Ein neues fundamentales kanonisches Objekt jenseits der bestehenden Menge. Vertrag: definiert seine Kanonisierung (`Can∘Can=Can`), seinen Rand `(B, B⁻, seam)` (nichtleer), sein Residuenfeld, seine Signatur `Σsig` und seine Teilnahme an Skeleton/Junction-Tree.
- **`OperatorExtension` — neuer Operator.** Ein neuer fundamentaler Operator. Vertrag: bringt seine **Subject-Reduction**- (Signatur-Erhalt), **Progress**- und **Konfluenz-mod-≡σ**-Pflicht mit (INV-8/9) sowie Determinismus (P9).
- **`GateExtension` — neue Gate-Klasse.** Eine neue fundamentale Gate-Kategorie jenseits G1–G7. Vertrag: boolesch, fail-closed, begründet, **kein Score** (V1), fügt sich in die Gate-Kette.
- **`SweepExtension` — neue Sweep-Fähigkeit.** Eine neue Fähigkeit auf dem Collect-/Distribute-Sweep. Vertrag: erhält die Zwei-Sweep-Korrektheit (INV-7) und den Abschluss-Round-Trip.
- **`CapacityExtension` — Kapazität/Skalierung.** Größere Kristalle, tiefere Junction-Trees, mehr nebenläufige Läufe. Vertrag: erhält Determinismus/Replay (INV-10) und **ändert die kanonische Klasse nicht** (Skalierung ist keine Semantik).

Nur durch diese Punkte wächst der Kern. Ein Umbau außerhalb ist nicht vorgesehen (§S14.8).

---

## S14.3 — Closure-erhaltende Beweispflichten (das Herz)

Jeder Erweiterungstyp trägt **Beweispflichten**, die erfüllt sein müssen, damit die Erweiterung zugelassen wird — nicht optional, wie die Teileliste des `DomainAdapter` nicht optional ist. Die Pflichten sind exakt die **Invarianten und die Metatheorie, eingeschränkt auf das neue Element**:

| Erweiterung | Beweispflicht (muss erfüllt sein) |
|-------------|-----------------------------------|
| `ObjectExtension` | INV-1 (Kanon-Idempotenz), INV-2 (Signatur), INV-3 (nichtleerer Rand) für das neue Objekt |
| `OperatorExtension` | Subject Reduction, Progress, Konfluenz mod ≡σ (INV-8/9), Determinismus (P9) für den neuen Operator |
| `GateExtension` | boolesch, fail-closed, begründet, kein Score (V1/V7) |
| `SweepExtension` | Zwei-Sweep-Korrektheit (INV-7) und Abschlussformel bleiben gültig |
| `CapacityExtension` | Replay-Identität (INV-10) bei Skalierung; kanonische Klasse unverändert |

**Closure-erhaltend by construction:** Eine Erweiterung, die ihre Beweispflicht nicht erfüllt, ist **unzulässig**. Ein neuer Operator, der Konfluenz brechen würde; ein neues Objekt, dessen Kanonisierung nicht idempotent ist; ein neues Gate, das ein Score wäre — alle werden **abgewiesen**, bevor sie den Kern erreichen. So kann der Kern nicht in einen Zustand wachsen, der den Abschluss bricht.

---

## S14.4 — Validierung durch denselben Regressionswächter

Eine `CoreExtension` wird **exakt wie** eine Domäne oder ein Bibliotheks-Asset validiert (S8.2/S8.3):

```
Bei Aufnahme einer Kern-Erweiterung prüft die CI:
    ∀ Referenz-Cube C_ref (ALLER Domänen) :  ClosedCCE(C_ref) = 1   (bleibt GRÜN)
    ∀ Negativ-Cube  C_neg (ALLER Domänen) :  Reject(C_neg)          (bleibt ROT)
    ∧ die Beweispflichten der Erweiterung (§S14.3) sind erfüllt

    Bräche die Erweiterung einen Referenz-Cube (Abschluss verloren)   → ABGEWIESEN
    Schwächte sie einen Negativ-Cube (Prohibition verloren)           → ABGEWIESEN
```

**Ein Wächter, dreifacher Schutz:** Dieselbe CI-verdrahtete Bibliotheks-Mechanik schützt **Domänen** (horizontale Breite), **Updates** (S11.4) **und** **Kern-Erweiterungen** (vertikale Tiefe). Eine schlechte vertikale Erweiterung wird gefangen wie eine schlechte Domäne — vor der Aufnahme.

---

## S14.5 — Azyklische, port-basierte Integration

- **Über deklarierte Ports** (Bauverfassung Teil 6): Eine `CoreExtension` ist ein **neues Modul**, das Fähigkeit über sanktionierte Ports **hinzufügt** — nie ein Umschreiben des Kerns.
- **Azyklisch** (INV-11): Keine Erweiterung erzeugt einen Zyklus oder greift unsachgemäß in den Kern. `ci/check_acyclic` bleibt grün.
- **Additiv, nicht destruktiv:** Bestehende Objekte/Operatoren/Gates bleiben; die Erweiterung fügt hinzu.

---

## S14.6 — Invarianten über alle Erweiterungen (Extension-Invarianz) & vertikal∘horizontal

- **Extension-invariant:** INV-1..14 und V1–V10 gelten **über alle** Erweiterungen hinweg — vor und nach jeder zugelassenen Erweiterung. Eine Erweiterung, die eine Invariante/Prohibition lockerte, ist unzulässig (vertikales Analogon zu S13.7: keine Verbots-Lockerung).
- **Erweiterung ist ein Monolith im Ledger:** append-only, begründet, versioniert; Beweispflichten und Validierungsstatus aufgezeichnet. Kein stilles Kern-Vertiefen.
- **Vertikal komponiert mit horizontal:** Eine neu hinzugefügte Kern-Fähigkeit (z. B. eine neue Gate-Klasse) steht danach **allen** Domänen zur Verfügung — jede Domäne kann sie in ihren `domain_gates` nutzen. **Einmal vertieft, alle Domänen profitieren.** Das ist der Ertrag der Faktorierung.

---

## S14.7 — Fähigkeit und Kapazität (beide Bedeutungen abgedeckt)

Sie nannten „Kapazität/Fähigkeit" — beide sind vertikal, beide abgedeckt:

- **Fähigkeit (capability):** neue *Arten* — `ObjectExtension`, `OperatorExtension`, `GateExtension`, `SweepExtension`. Der Kern kann mehr *Kinds* von Dingen.
- **Kapazität (capacity):** *Skalierung* — `CapacityExtension`. Größere Kristalle, tiefere Bäume, mehr Läufe. Der Kern verarbeitet *mehr*, ohne dass sich die Semantik ändert: Ein größerer Kristall wird in **dieselbe** Klasse verarbeitet, die er definiert — **Skalierung ist keine Semantik**, und Determinismus/Replay bleiben erhalten (INV-10).

---

## S14.8 — Read-only-/Erhaltungs-Garantien (Verbote auf Erweiterungsebene)

Abwesende Handhaben:

- **kein beliebiger Kern-Umbau** — Wachstum nur durch die sanktionierten Erweiterungspunkte (§S14.2).
- **keine Aufnahme ohne erfüllte Beweispflicht** — eine Erweiterung ohne discharge ihrer Pflichten ist unzulässig (§S14.3).
- **kein Bruch eines Referenz-Cubes / keine Schwächung eines Negativ-Cubes** — vom Regressionswächter gefangen (§S14.4).
- **keine Invarianten-/Verbots-Lockerung** — Extension-Invarianz (§S14.6).
- **kein Zyklus** — azyklische Integration bleibt erzwungen (§S14.5).

---

## S14.9 — Sichtbare Residuen dieser Spezifikation

Kein stilles Loch:

- **R1 (Beweisführungs-Werkzeug).** Wie die Beweispflichten (§S14.3) technisch geführt werden (Property-Tests, ggf. formale Nachweise), ist beim Bau zu wählen; **dass** sie Aufnahmebedingung sind, steht fest.
- **R2 (Erweiterungspunkt-Menge).** Die fünf Erweiterungspunkte (§S14.2) decken Objekt/Operator/Gate/Sweep/Kapazität; sollte sich ein weiterer fundamentaler Punkt zeigen, wird er **selbst** als sanktionierter Punkt mit Vertrag hinzugefügt (nach denselben Regeln), nie als Umgehung.
- **R3 (Kapazitäts-Grenzen).** Konkrete Skalierungsziele je `CapacityExtension` sind beim Bedarf zu fixieren; das Prinzip (Determinismus/Klasse erhalten) steht fest.

---

## S14.10 — Abnahme (DoD dieser Ebene)

```
DoD(S14) = 1  ⟺
    das CoreExtension-Protokoll definiert die sanktionierten Erweiterungspunkte
        (Object/Operator/Gate/Sweep/Capacity), jeder mit closure-erhaltender Beweispflicht (§S14.2/S14.3)
  ∧ eine Erweiterung wird nur zugelassen, wenn ihre Beweispflichten erfüllt sind (closure-erhaltend by construction)
  ∧ Erweiterungen werden vom SELBEN Regressionswächter validiert: alle Referenz-Cubes grün, alle Negativ-Cubes rot (§S14.4)
  ∧ Integration ist azyklisch über Ports; additiv, nicht destruktiv (§S14.5)
  ∧ INV-1..14 und V1–V10 sind extension-invariant; keine Erweiterung lockert eine Invariante/Prohibition (§S14.6)
  ∧ jede Erweiterung ist ein versionierter Monolith im Ledger mit aufgezeichneten Beweispflichten
  ∧ Fähigkeit (neue Arten) UND Kapazität (Skalierung) sind abgedeckt; Kapazität erhält Determinismus/Klasse (§S14.7)
  ∧ vertikale Fähigkeit komponiert mit allen Domänen (einmal vertieft, alle profitieren, §S14.6)
  ∧ kein beliebiger Kern-Umbau ist möglich (§S14.8)
  ∧ Engine-DoD, DoD(S1..S13), S10, Domänen-Katalog bleiben unberührt
```

---

## S14.11 — Schluss: die Plattform-DoD ist auf Papier geschlossen

Mit S14 sind **beide Achsen** spezifiziert:

- **Horizontal (Breite):** `DomainAdapter` (S1.8) + Domänen-Katalog (213 Domänen, L1-vollständig) — friktionslose Breite, ohne Kernberührung.
- **Vertikal (Tiefe):** `CoreExtension` (diese Spec) — friktionslose Tiefe, closure-erhaltend by construction, vom selben Wächter geschützt.

Beide durch **dieselbe Disziplin** garantiert: Abschluss, Invarianten, Regressionswächter, azyklische Ports. Der Kern kann in Breite *und* Tiefe wachsen — und **kann dabei nie den Abschluss brechen**.

Damit ist die **Plattform-DoD vollständig auf Papier**:

```
Plattform-DoD(CCE) = 1  ⟺
    Produkt-DoD(Dokument) = 1                    (S10 — das fertige Produkt)
  ∧ Domänen-Katalog vollständig (L1), Reifepfad je Domäne sichtbar   (horizontal)
  ∧ DomainAdapter garantiert friktionslose Breite  (S1.8)
  ∧ CoreExtension garantiert friktionslose Tiefe   (S14)
  ∧ ein Regressionswächter schützt Domänen, Updates und Kern-Erweiterungen  (S8.3/S11.4/S14.4)
  ∧ alle Invarianten/Prohibitionen extension-invariant über beide Achsen
  ∧ jede verbleibende Unfertigkeit ist sichtbares Residuum (Produkt-Level, benannte Reifepfade), nie kaschiert
```

Die Stadt ist vollständig geplant — das Werk, das Produkt, alle Domänen, und die Wachstumsstrategie auf beiden Achsen. Jede verbleibende Arbeit ist ein **sichtbarer, geführter Reifegrad**, kein verstecktes Loch. Danach übernimmt der Coding-Agent nach der One-Shot-Implementierungsanweisung (Bauverfassung Teil 9) und errichtet `cce/` aus einem Guss.

*Ende der Detail-Spezifikation S14 — und der Systemspezifikation auf Plattform-DoD-Niveau.*
