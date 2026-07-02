# CCE — DETAIL-SPEZIFIKATION S9: PERSISTENZ, ARBEITSBEREICHE & PROJEKTVERWALTUNG

**Achte D-Spec.** Das Substrat, in dem Läufe, Kristalle, Artefakte, Ledger und Bibliothek über Sitzungen leben — inhaltsadressiert, unveränderlich wo es zählt, in Arbeitsbereiche organisiert, versioniert und **von Anfang an sync-fähig ausgelegt** (R-Plan-5).

**Status:** Detail-Spezifikation v1.0, auf Bauverfassungs-Tiefe. Baut auf: BAUVERFASSUNG (content-addressing P10, Ledger INV-12, PHC-Portable), S3 (PersistenceAdapter S3.1.2, Persistenz-Sicht S3.7), S5 (Checkpoints, RunDescriptor), S6 (Ledger), S7 (Artefakt-Ablage, Zwei-Digest S7.2), S8 (Bibliotheks-Speicher).

---

## S9.0 — Einordnung & Leitsatz

S9 ist die **unterste Ebene der Menschenseite**: Alles, was die höheren Ebenen erzeugen, ruht hier persistent. Zugleich trägt S9 die Last der Entscheidung „Sync fest geplant" — es muss so gebaut sein, dass Synchronisation eine **eingeplante Dimension** ist, kein Nachrüsten.

Leitsatz:

> *Was unveränderlich sein muss, ist inhaltsadressiert und damit global eindeutig — deshalb ist sein Abgleich konfliktfrei. Was veränderlich ist, sind kleine Verweise darauf — und ihre einzigen Konflikte werden sichtbar gemacht, nie still aufgelöst.*

Zwei Grundfesten:

- **Zwei Speicher, sauber getrennt (§S9.1).** Ein **unveränderlicher, inhaltsadressierter Kern** (Kristalle, Artefakte, Checkpoints, Ledger-Einträge, Bibliotheks-Assets, Zertifikate) und eine **kleine, veränderliche Verweis-/Index-Schicht** (Namen, Workspace-Zugehörigkeit, „neueste Version"). Garantien im Kern, Bedienbarkeit obenauf.
- **Inhaltsadressierung ist die Grundlage von Replay UND Sync-Fähigkeit.** Gleicher Digest ⇒ gleiches Objekt ⇒ Abgleich ist bloßes Kopieren, nie ein Konflikt (§S9.6).

---

## S9.1 — Das Zwei-Speicher-Modell

- **Unveränderlicher Inhaltsspeicher (CAS — content-addressed store).** Jedes inhaltsadressierbare Objekt wird **nach seinem Digest** gespeichert, unveränderlich, dedupliziert, integritätsgesichert (Bauverfassung P10). Enthält: bestätigte Kristalle (S4), Checkpoints & RunDescriptors (S5), Ledger-Einträge (S6), Artefakte mit beiden Digests (S7), Bibliotheks-Assets (S8), Zertifikate. **Wird nie mutiert** — Änderung heißt neues Objekt mit neuem Digest.
- **Veränderliche Verweis-/Index-Schicht (Refs).** Klein und menschennah: benannte Verweise in den CAS („dieser Workspace", „neueste Fassung dieser Vorlage", „dieses Projekt"). Verweise sind veränderlich, aber sie **zeigen** nur in den unveränderlichen Kern.

Diese Trennung — unveränderliche Objekte + veränderliche Verweise — ist das klassische, bewährte Muster (wie Git): Sie liefert Unveränderlichkeit, wo Garantien hängen, und Beweglichkeit, wo ein Mensch Namen und Versionen braucht.

---

## S9.2 — Arbeitsbereiche (Workspaces)

- **Die organisierende Einheit.** Ein Workspace ist eine **benannte Sammlung von Verweisen** in den CAS plus ein **eigenes Ledger-Segment**. Er hält die Läufe, Kristalle, Artefakte und den Bibliothekszugang eines Arbeitszusammenhangs.
- **Einzelnutzer zuerst, erweiterbar (R-Plan-4).** Für den Einzelnutzer hat der Operator einen oder mehrere Workspaces; das **Identitätsmodell ist vorhanden, aber trivial** (ein Nutzer). Ein späterer **geteilter/Mehrnutzer-Workspace** dockt an, ohne den Kern zu ändern — der Workspace ist bereits als „Verweismenge + Ledger" modelliert, was Mehrnutzer natürlich aufnimmt.
- **Isolation.** Workspaces sind voneinander isoliert; Objekte im CAS können (durch Deduplizierung) geteilt werden, aber die Sicht/Refs sind je Workspace getrennt.

---

## S9.3 — Projekte (Projektverwaltung)

- **Gruppierung innerhalb eines Workspace.** Ein Projekt bündelt zusammengehörige Arbeit (z. B. „die Q3-Risiko-Memos"): Läufe, Artefakte, projektspezifische Vorlagen.
- **Leichte, menschennahe Ebene.** Projekte sind Refs-Gruppierungen (kein neuer Speicher-Mechanismus), die dem Operator Ordnung geben.
- **Kopplung.** Ein Projekt verweist auf seine Läufe/Artefakte im CAS; sein Verlauf steht im Workspace-Ledger.

---

## S9.4 — Versionierung

- **Version = Verweis auf unveränderliche Objektfassung.** Da der CAS unveränderlich ist, ist „Versionieren" schlicht: ein veränderlicher Verweis zeigt über die Zeit auf **verschiedene** unveränderliche Objekt-Digests.
- **Alte Versionen bleiben erhalten.** Jede Fassung eines Kristalls/Artefakts/einer Vorlage/eines Bibliotheks-Assets bleibt im CAS (nie überschrieben); die Versionskette ist eine Folge von Digests.
- **Kohärent mit S7/S8.** Diese Versionierung ist dieselbe, die S7 (Artefakt-Fassungen) und S8 (Bibliotheks-Versionen) nutzen — ein Mechanismus, nicht drei.

---

## S9.5 — Der Ledger in der Persistenz

- **Append-only, hash-verkettet, je Workspace.** Der Ledger (S6.4) ist persistent als unveränderliche, verkettete Historie; jeder Eintrag verweist auf den Vorgänger-Hash und liegt content-adressiert im CAS.
- **Prüfbar.** `verify_ledger` (S3.7/S6.4) läuft gegen den persistierten Ledger; eine nachträgliche Änderung eines Eintrags/Monolithen ist eine Abnahmeverletzung (Bauverfassung INV-12) und wird erkannt.
- **Audit-Rückgrat.** Der Ledger ist die persistente Grundlage der Nachvollziehbarkeit (S6.8) und der aufgezeichneten HITL-Entscheidungen (S5.4).

---

## S9.6 — Sync-Fähigkeit by design (R-Plan-5)

Auch wenn die eigentliche Sync-Umsetzung eine spätere Phase ist, ist der Speicher **so gebaut**, dass Sync eine eingeplante Dimension ist:

- **Unveränderliche Schicht: konfliktfrei.** Inhaltsadressierte Objekte sind global eindeutig; **gleicher Digest = gleiches Objekt**. Objekte abgleichen heißt kopieren — es gibt **keinen** Konflikt auf der unveränderlichen Schicht.
- **Ledger: append-only Zusammenführung.** Ledger sind append-only Logs; ihr Abgleich ist Zusammenführen wohl-verstandener, unveränderlicher Ketten — keine destruktiven Konflikte auf der Objektschicht.
- **Einziger möglicher Konflikt: veränderliche Verweise.** Zeigen zwei Geräte den „neueste Fassung"-Verweis unterschiedlich, entsteht ein **Ref-Konflikt**. Der wird **sichtbar gemacht und vom Operator aufgelöst** (wie ein Merge-Konflikt) — **nie still aufgelöst** (konsistent mit P4/V2). Ein Ref-Konflikt ist ein sichtbares Ding, kein verschwiegenes.
- **Die Naht: der PersistenceAdapter.** Der `PersistenceAdapter`-Trait (S3.1.2) trennt „lokaler Speicher jetzt" von „synchronisierter Speicher später" hinter einer **stabilen Schnittstelle**; die Sync-Ebene wird dahinter ergänzt, ohne höhere Ebenen zu ändern.
- **Geräteunabhängige Formate.** Portable Formate (`PHC-Portable`, S7) machen Objekte geräteunabhängig.

**Ergebnis:** Sync ist geplant, nicht nachgerüstet — die unveränderliche Schicht gleicht sich konfliktfrei ab, die einzigen Konflikte (Refs) sind explizit und operator-aufgelöst, und die Adapter-Naht isoliert die künftige Änderung.

---

## S9.7 — Integrität & Wiederherstellung

- **Selbst-prüfend.** Jedes CAS-Objekt trägt seinen Digest; ein beschädigtes Objekt fällt bei der Digest-Prüfung durch. Der Ledger fällt bei Manipulation durch `verify_ledger`.
- **Wiederherstellbar.** Aus dem unveränderlichen CAS **plus** Ledger lässt sich der Workspace-Zustand rekonstruieren — die Verweis-Schicht ist klein und aus der Historie ableitbar. Das System ist damit selbst-verifizierend und wiederherstellbar.

---

## S9.8 — Kopplung an alle vorherigen Ebenen (aus einem Guss)

S9 ist das Substrat, in das alles ablegt:

- **S4:** bestätigte Kristalle → CAS.
- **S5:** Checkpoints, RunDescriptors, aufgezeichnete Entscheidungen → CAS/Ledger.
- **S6:** Ledger → persistent, prüfbar.
- **S7:** Artefakte (beide Digests) → CAS, content-adressiert.
- **S8:** Bibliotheks-Assets (versioniert) → CAS.
- **S3:** der **CockpitCore** greift über den **PersistenceAdapter** auf all dies zu (S3.1.2).

Ein Speicher-Substrat, das die ganze Apparatur trägt — kein separater Mechanismus je Ebene.

---

## S9.9 — Read-only-/Integritäts-Garantien (Verbote auf Persistenzebene)

Abwesende Handhaben:

- **kein „unveränderliches Objekt mutieren"** — CAS-Objekte werden nie geändert; Änderung = neues Objekt.
- **kein stilles Auflösen eines Ref-Konflikts** — Ref-Konflikte sind sichtbar und operator-aufgelöst (§S9.6, V2).
- **kein „alte Version verlieren"** — Versionen bleiben erhalten (§S9.4).
- **kein „Ledger brechen"** — append-only, hash-verkettet, prüfbar (INV-12).
- **kein „Objekt ohne Digest"** — alles Inhaltsadressierbare trägt seinen Digest (P10).

---

## S9.10 — Sichtbare Residuen dieser Spezifikation

Kein stilles Loch:

- **R1 (Sync-Umsetzung).** Die konkrete Sync-Ebene (Transport, Konflikt-UX) ist eine spätere Phase; **dass** sie by design konfliktfrei/explizit ist, steht fest (§S9.6).
- **R2 (CAS-Backend).** Konkrete Ablage (Dateisystem-Layout vs. eingebettete DB) ist zu wählen; das Prinzip (content-addressiert, unveränderlich, dedupliziert) steht fest.
- **R3 (Projekt-/Workspace-UX).** Tiefe der Projekt-/Workspace-Verwaltung im Cockpit ist einem Gestaltungsdurchgang vorbehalten; das Modell (Refs + Ledger) steht fest.
- **R4 (Ref-Konflikt-UX).** Darstellung der Konfliktauflösung ist offen; das Prinzip (sichtbar, operator-aufgelöst, nie still) steht fest.

---

## S9.11 — Abnahme (DoD dieser Ebene)

```
DoD(S9) = 1  ⟺
    Alles persistiert über Sitzungen: Kristalle, Läufe, Checkpoints, RunDescriptors, Ledger, Artefakte, Bibliothek
  ∧ Zwei-Speicher-Modell: unveränderlicher inhaltsadressierter CAS + kleine veränderliche Verweis-Schicht (§S9.1)
  ∧ CAS-Objekte sind content-adressiert, unveränderlich, dedupliziert, integritätsgeprüft (P10)
  ∧ Workspaces (Refs + Ledger-Segment) organisieren die Arbeit; Einzelnutzer jetzt, Mehrnutzer ohne Kernänderung (§S9.2)
  ∧ Projekte gruppieren innerhalb eines Workspace (§S9.3)
  ∧ Versionierung erhält ALLE Fassungen (Verweis auf unveränderliche Objekt-Digests, §S9.4)
  ∧ sync-fähig by design: unveränderliche Schicht konfliktfrei, Ref-Konflikte sichtbar + operator-aufgelöst (nie still),
        PersistenceAdapter-Naht, portable Formate (§S9.6)
  ∧ Integrität ist prüfbar und Workspace-Zustand aus CAS+Ledger wiederherstellbar (§S9.7)
  ∧ KEINE Handhabe mutiert Unveränderliches, löst Ref-Konflikte still, verliert Versionen oder bricht den Ledger (§S9.9)
  ∧ Engine-DoD, DoD(S1/S3/S4/S5/S6/S7/S8) bleiben unberührt
```

---

## S9.12 — Anschluss

S9 schließt die Persistenz-Ebene: Alles ruht persistent auf einem unveränderlichen, inhaltsadressierten Kern mit kleiner veränderlicher Verweis-Schicht; Workspaces und Projekte ordnen die Arbeit; Versionen bleiben erhalten; Integrität ist prüfbar und der Zustand wiederherstellbar; und Synchronisation ist by design vorbereitet — konfliktfrei auf der Objektschicht, mit sichtbaren, operator-aufgelösten Ref-Konflikten.

**Nächste D-Spec (Systemlandkarte §6):** **S13 — Nutzungs-Governance / Produktverfassung** — die Verbotsachse (kein Score-als-Gate, Residuum sichtbar, Claim-Schranke) auf **Bedienebene**, und die KI-Orchestrierung an dieselben Regeln gebunden, als operierende Verfassung des fertigen Produkts (in S3.4/S6.7/S7.9 durchgängig verankert, hier als Governance-Ebene zusammengeführt). Danach S2, S11, S12 und die Produktabnahme S10 — bis `ProduktDoD=1`.

*Ende der Detail-Spezifikation S9.*
