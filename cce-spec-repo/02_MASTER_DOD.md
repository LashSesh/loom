# 02 — MASTER-DoD: Die 100%-Abnahme (präzise definiert)

**„100%" ist kein Gefühl, sondern diese Formel.** Der Bau ist genau dann vollständig, wenn jede Zeile dieser Checkliste nachweislich grün ist — und **nur** diese. Was per Spezifikation sichtbares Residuum ist, gehört ausdrücklich **nicht** zur 100%-Bauabnahme (§3): es zu bauen wäre Überbau, es zu verschweigen wäre Drift.

## 1 — Die Formel

```
DoD_100(CCE-Bau) = 1 ⟺
    G0–G12: alle Ausgangs-Gates der Master-Bauordnung grün                     (01_MASTER_BUILD)
  ∧ Engine-DoD(cce) = 1: VC1–VC10⁺ erfüllt; INV-1..14 + F1–F6 als Tests grün;
        V1–V10 + PROD-INV-9..16 als Negativ-Tests rot                          (BAUVERFASSUNG T8)
  ∧ Motor-Kerntest: Reanalyze∘Materialize∘LOOM∘Project∘PHC ≃ id  am Referenz-Cube   (BV T7)
  ∧ ProduktDoD(Dokument) = 1: Produkt-Kerntest über alle 6 Nähte im Cockpit,
        D01 = PL4 real erreicht                                                 (S10, S2)
  ∧ DoD-Matrix a–h auf BAU-Ebene = 1:
        a Produkt(Dokument) ✓ · b Katalog strukturell (PL-Slots implementiert; 213 Einträge
        als Registry geladen) · c Rebase-Strukturen gebaut (PSPcore, PhaseBlock-HDAG,
        Frontiers, CrystalConsensus) · d Spiral-Kinematik gebaut (8 Gates, 12 Residuen)
        · e HBM-Chassis gebaut (HBM-01..20 grün) · f S15-Strukturen gebaut (ScaleAdapter-
        Vertrag, Multicube-/Capsule-Typen, MultiScaleClosure(SCALE-1) = Produkt-Kerntest)
        · g CSA gebaut (15 Gates, 3 Adapter, 13 Zeugen) · h .loom gebaut (C0–C5, 21 Zeugen)
  ∧ EIN Regressionswächter, ALLE Zeugen: Dokument-Cubes ∧ Spiral-/PhaseBlock-Negative
        ∧ CSA 5R+8N ∧ .loom 7R+14N — jede Referenz grün, jede Negative rot, CI-blockierend
  ∧ Cockpit: nativ lauffähig, COCK-INV-1..6 getestet, Kanzel schreiblos & optional
  ∧ Auslieferung: installierbare Pakete, Kern offline nachgewiesen, Saat-Bibliothek
        als .loom enthalten, jede Funktion PL-gekennzeichnet
  ∧ Abschlussbericht: DoD_100-Checkliste Punkt für Punkt + End-Residuenregister
        (kein stilles Residuum; jede Abweichung von einer Spec ist gemeldet)
```

## 2 — Abhak-Checkliste (der Agent führt sie im Abschlussbericht)

- [ ] G0 Integrität (24/24 SHA-256) · Workspace · CI · Acyclic
- [ ] G1 Kernobjekte · G1–G7-Gerüst · V1-Negativtest
- [ ] G2 PSPcore · PhaseBlock-Accept(8) · Frontier-Sync · Ledger=CommitProjection · verify_hdag_projection
- [ ] G3 **Motor-Kerntest ≃** · Dokument-Adapter (11 Punkte) · 7 Dokument-Gates · Zwei-Digest
- [ ] G4 8 Spiral-Gates · 12 Residuen · Kaskadenregel · Profil-statt-Dogma-Negativtest
- [ ] G5 Replay-Identität · Pause/Resume klassenerhaltend · HITL-als-PhaseBlock-Input · Reise=Red(SCALE-1)
- [ ] G6 CAS+Refs · **Wächter blockt eingeschleusten Bruch** · Dokument-Zeugen grün/rot
- [ ] G7 HBM-01..20 · Gate-Dominanz · Klonung fail-closed-deaktiviert (Test) · End-to-End-Kristall
- [ ] G8 **Kein Socket vor PolicyGate** · 3 Adapter · 15 Gates · 13 CSA-Zeugen · PROD-INV-13..16
- [ ] G9 .loom C0–C5 · 21 Zeugen · Viewer motorfrei · open() seiteneffektfrei · Saat als .loom
- [ ] G10 4 Flächen + 6 Ansichten · COCK-INV-1..6 · Kanzel-Aus voll funktionsfähig
- [ ] G11 **Produkt-Kerntest über 6 Nähte** · Re-Import klassenidentisch · Doku=Verhalten
- [ ] G12 Pakete je OS · offline · PL-Kennzeichnung · **Abschlussbericht + End-Residuenregister**

## 3 — Ausdrücklich NICHT Teil der 100%-Bauabnahme (sichtbare, geführte Residuen)

Diese Punkte sind spezifizierte Reifepfade bzw. spätere Foundations — sie werden im Endbericht als offen **gelistet**, nicht gebaut, nicht kaschiert: die **212 Domänen jenseits D01** über PL1 hinaus (Priorisierungs-Weiche §K.6; Registry-Einträge ja, Vollausbau nein) · **SCALE-2…8-Kerntests** (S15-R1/R-10; Strukturen ja, Skalenreife-Behauptung nein) · **Nexus-Bridge L9b** (BridgeNorm-Promotion, NexusClass, Normic Memory — R-1b; nur Typen/Ports) · **Klonungs-Aktivierung** (R-13; implementiert, deaktiviert) · **ConnectorAdapter-OAuth-Vollform** (R-16/CSA-R3; Portvertrag ja) · **Sync-Mehrgeräte-Betrieb** (S9: vorbereitet, lokal-first ausgeliefert) · **generic_html-StaticWebAdapter** (nur wenn nach G8-Kern noch Budget: mit allen Sonderregeln, sonst offen) · **blake3-Zweitprofil, Signatur-Registry-Vollform, GUI-Feindesign** (LC-R1/2/5) · **physische Amendment-Einarbeitung in die S-Dateien** (R-9; Amendments gelten als Overlays — der Agent editiert spec/ NIE).

Meldet der Agent „100%", ohne dass §3 als offene Liste im Endbericht steht, ist die Abnahme **nicht** erfüllt (Anti-Overclaim, S10-A2).

*Weiter mit `04_AGENT_AUFTRAG.md` (für den Auftraggeber) bzw. Baubeginn bei G0.*
