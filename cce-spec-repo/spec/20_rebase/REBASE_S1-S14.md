# CCE — REBASE-NACHZIEHBLÖCKE S1–S14 (Amendment-Schicht)

**Die konkreten, normativen Nachziehungen** je Spezifikation. Geführt als **Amendments** (append-only, S13.7-konform): Jeder Block gilt ab sofort als Bestandteil seiner Spec; die physische Einarbeitung in die Einzeldateien ist Redaktionsschritt R-9 (sichtbar). Rebase-Gültigkeitsregel überall: `DoD(Si+) = DoD(Si) ∧ FoundationAligned ∧ NoNewSilentResidue` — **keine bestehende DoD wird gebrochen oder gelockert.**

---

## 1 — PHASE 3: S1-Amendment — Domain-Katalog × HBM × Nexus

**S1-A1 (Adapter-Vereinheitlichung).** Es gibt **einen** Adapterbegriff: den S1-`DomainAdapter` (11 Punkte). Die HBM-Adapterfelder werden abgebildet:

| HBM-Feld `A_D` | S1-Vertragspunkt |
|---|---|
| `Π_D` (Projektion) | Punkt 2 `to_canonical` / Punkt 3 `encode` |
| `C_D` (Constraints) | Punkt 1 `wish_schema` (Randbedingungs-Vokabular) |
| `V_D` (Validierung) | Punkt 1 `validate_wish` + Punkt 8 `domain_gates` |
| `Can_D` (Kanonisierung) | Punkt 7 `canonicalize` |
| `σ_D` (Signatur) | Punkt 7 (Inhaltsklassen-Digest, Zwei-Digest-Modell S7.2) |
| `G_D` (Gates) | Punkt 8 |
| `E_D` (Evidence-Schema) | **neu ausgewiesen** im MiningProfile (S1-A2) |
| `Ex_D` (Export-Senke) | Punkt 10 `native_open` / `export_formats` |

**S1-A2 (Drei neue Slots je Domäne, adapter-paritätisch).** Jede Domäne erhält — zusätzlich zu den 11 Punkten — drei Slots, deren *Definition* jetzt Pflicht und deren *Befüllung* PL-geführter Reifepfad ist (R-11):

- **ScaleSlot:** BlueCube-/RedCube-/PhaseBlock-Anschluss der Domäne — welche Domänen-Objekte als `Blue_{s,p}`-Zustände auf SCALE-0/1 laufen und wie ihr `Close(Blue)` (Typed ∧ BoundaryValid ∧ Gate ∧ Evidence ∧ Replay ∧ WrapStable) domänenkonkret heißt.
- **MiningProfile:** Facet-Typen der Domäne (Entität, Operator, Constraint, Gate, Invariante, Messgröße, Prozessschritt, Boundary Contract, Risiko, Exportziel — domänenspezifisch belegt), Evidence-Schema `E_D`, zulässige Kandidaten-Generatoren (C1–C6), Score-Gewichte als RD-Parameter (nie Gate), Adapter-Mining-Gate `Gate_A = ValidTypes ∧ ResolvableRefs ∧ Testable ∧ NonTautological ∧ NonContradictory`.
- **BridgeProfile (Port, PL0):** Typ-Deklaration für spätere Nexus-Anbindung (welche DomainNorms exportierbar, welche BridgeNorm-Klassen zulässig) — nur Signatur, keine Funktion, bis L9-Foundation vorliegt (R-1).

**S1-A3 (PL-Umbenennung).** Produkt-Level heißen **PL0–PL4** (Disambiguierung gegen Landkarten-L0–L11). Alle Katalog-Aussagen bleiben inhaltlich unverändert; D01 = PL4, übrige 212 = PL1. **Neu:** PL2 setzt künftig zusätzlich ein definiertes MiningProfile voraus; ScaleSlot ab PL3; BridgeProfile bleibt bis Nexus-Foundation PL0-Port.

**S1-A4 (Katalog-Dynamik).** Der Katalog ist nicht nur statisch: HBM-**Adapter-Mining** (Pipeline-Phase 2) darf Adapter-*Kandidaten* für neue oder unterversorgte Domänen erzeugen — jeder Kandidat durchläuft `Gate_A`, wird als BlueprintCandidate geführt und tritt **nur** über die S8-Selbst-Validierung in den Katalog ein. Kein gemint er Adapter wird still normativ.

**S1-DoD⁺:** DoD(S1) ∧ Mapping-Tabelle vorhanden ∧ drei Slots definiert ∧ PL-Umbenennung vollzogen ∧ Katalog behauptet keine PL4-Reife jenseits D01.

---

## 2 — PHASE 4: S2–S9-Amendments (Produktreise und Produktschichten)

### S2-Amendment (Reise als PhaseLadder auf SCALE-1)

- **S2-A1:** Jede Naht 0–5 wird zusätzlich als **Seam + Replay-Abschnitt** typisiert; der Übergang über eine Naht ist protokollierbar als PhaseBlock-Accept (SCALE-1, Phase = Reisestufe). Die Reise ist damit eine **PhaseLadder(SCALE-1)**: Wünschen→Bestätigen→Laufen→Prüfen→Entnehmen→Ablegen sind Phasen p₀…p₅ eines `Red_{SCALE-1}`-Zyklus; `Close(Red_{SCALE-1})` = geschlossene Reise.
- **S2-A2:** Die Fehl-Reisen (S2.5) sind **Hold-Zustände** mit sichtbarem Residuum — nie stiller Abbruch (deckt sich mit Ratchet-`hold`).
- **S2-DoD⁺:** Reise als PhaseLadder lesbar ∧ Naht-Garantien unverändert ∧ SCALE-1-Bindung explizit (keine stille Höherskalierung).

### S3-Amendment (Cockpit: neue Ansichten, read-only)

- **S3-A1 (PhaseBlock-Ansicht):** je Block die 10 Felder + Accept-Status; Kandidat/Hold/Accepted optisch getrennt.
- **S3-A2 (Frontier-Ansicht):** HDAG-Frontier und Ratchet-Frontier nebeneinander + Sync-Status; `frontier_desync` blocking sichtbar.
- **S3-A3 (Ratchet-Ansicht):** je Ratchet `(s,p,j)`: Zustand advance/hold/lock, Commitzähler c, Evidence-Ref; Kaskadenbaum mit Lock-Bedingung.
- **S3-A4 (Blue/Red-Ansicht):** Schlussstatus je `Blue_{s,p}` und `Red_s` inkl. WrapStability und AreaClass-Status.
- **S3-A5 (HBM-Candidate-Board):** Kandidaten mit Status Pass/Hold/Reject, Hold-Diagnose maschinenlesbar (HBM-17); **Score sichtbar ausschließlich als Ranking-Spalte mit festem Label „ordnet, entscheidet nicht"** — COCK-INV-6: kein UI-Pfad macht eine Kennzahl zur Entscheidung.
- **S3-A6:** Alle neuen Ansichten sind Fakten-Schicht (Motor→Anzeige, read-only); die KI-Kanzel erklärt sie nur (S3.4 unverändert).
- **S3-DoD⁺:** sechs Ansichten spezifiziert ∧ COCK-INV-1..6 ∧ Zustandsmodell deterministisch unverändert.

### S4-Amendment (Wunsch → Crystal + BoundarySpec + CompletionSpace + MiningQuery)

- **S4-A1:** Die Wunsch-Normalform wird erweitert: `W = (X,H,K,G,Res,Π,τ,Replay,Goal,Materialize, BoundarySpec, CompletionSpace, ScaleTarget)`. **BoundarySpec** deklariert den erlaubten Support/Scope (Grundlage von WrapProjection und DriftGate); **CompletionSpace** ist der offene Füllraum, aus dem Completion Pressure entsteht (`Pressure = Unfilled + Inconsistent + Unseamed`); **ScaleTarget** default SCALE-1 (Artefakt) — höhere Ziele sind zulässig zu *formulieren*, tragen aber sichtbaren PL-Hinweis (R-10).
- **S4-A2 (MiningQuery, optional):** Ein Wunsch kann eine **MiningQuery** an L7 auslösen („finde/erzeuge BlueprintCandidates für …") — die Kanzel formt sie, der Operator bestätigt (materielle Aktion), das Chassis liefert **Kandidaten** zurück in die Wunsch-Fläche; kein Kandidat wird ohne Bestätigung + Motor-Validierung zum Crystal.
- **S4-A3:** Residuen `boundary_unspecified` (Hold: BoundarySpec fehlt für Nicht-Default-Fälle), `completion_space_open` (info: sichtbarer Füllstand).
- **S4-DoD⁺:** erweiterte Normalform ∧ MiningQuery-Pfad bestätigungspflichtig ∧ Annahmen-Liste deckt die neuen Felder.

### S5-Amendment (Orchestrierung: HyperDAG, Kaskade, HBM-Pipeline)

- **S5-A1 (Begriffsschärfung):** **Checkpoint** = inhaltsadressierter *Kandidaten*-Zustand (wiederaufnahmefähig); **PhaseBlock** = *akzeptierter, irreversibler* Abschluss (Accept-8-Kriterien). Jeder bisherige „Gate grün + Ledger-Eintrag"-Moment der S5-Zustandsmaschine ist ab jetzt ein PhaseBlock-Accept; `ABGELEHNT` ist Hold/Reject mit sichtbarem Residuum.
- **S5-A2 (HITL als PhaseBlock-Input):** Jede Ermessens-Entscheidung wird als **Input eines PhaseBlocks** fixiert: `{gate, entscheidung, operator, RD-Ref, Evidence-Ref}` → Bestandteil von `evidence` des Blocks; Replay spielt sie ab (S5.4 unverändert, jetzt block-gebunden).
- **S5-A3 (Ausführung):** Der Runner kann ausführen: (a) den **PhaseBlock-HyperDAG** (topologisch, deterministisch; Ledger = CommitProjection), (b) die **Multi-Ratchet-Kaskade** (Locks nur bei geschlossenen/residue-sichtbaren Unter-Ratchets), (c) die **HBM-Pipeline** Phasen 0–9 als deterministischen Lauf (RD-gebunden; PhaseSwitch nur geloggt).
- **S5-A4 (Neue Gates):** PhaseRatchetGate, ScaleRatchetGate im Pfad; `ratchet_lock_without_evidence`, `scale_promotion_without_wrap_stability` blocking.
- **S5-DoD⁺:** drei Ausführungsformen deterministisch ∧ Kandidat/Block-Trennung überall ∧ kein Commit ohne Evidence (Driftverbot 6).

### S6-Amendment (Inspektion: multi-skalar aufklärbar)

- **S6-A1 (Sechs neue Inspektionsobjekte):** PhaseBlock (10 Felder, Roh-Beleg = Block-Digest), Ratchet-Lock (Lock-Bedingung + Evidence), WrapStability (WrapGate-Report + Supportmaß), AreaClass (Klassen-Digest vorher/nachher), Frontier-Sync (beide Frontiers + Sync-Beleg), HBM-Candidate (Status, Hold-Diagnose, Provenienz Facets→Adapter→Cube→Skeleton).
- **S6-A2 (Multi-skalare Aufklär-Tiefe):** Die vier Ebenen (Überblick→Objekt→Element→Roh-Beleg) gelten **je SCALE**; die Nachvollziehbarkeits-Regel (jede Anzeige ↦ genau ein Motor-Artefakt) bleibt hart.
- **S6-A3 (Score-Darstellung):** Scores erscheinen nur in Ranking-Kontexten, stets mit „ordnet, entscheidet nicht"; die Fakten-/Erklär-Trennung gilt unverändert — die Kanzel kann einen Ratchet-Hold **nicht** beschönigen (kein Schreibpfad).
- **S6-DoD⁺:** sechs Objekte mit Rendering-Vertrag ∧ multi-skalar ∧ read-only ∧ `frontier_desync` sichtbar wirksam.

### S7-Amendment (Artefakt: SCALE-1, Mining-/Spiral-Provenienz)

- **S7-A1:** **Artefakte bleiben SCALE-1.** Die Herkunftskette wird erweitert: `Crystal → Lauf → Artefakt` wird zu `[BlueprintCandidate → ] Crystal → Lauf [→ PhaseBlock-Digest] → Artefakt [→ MiningCapsule-Ref]` — jedes Glied content-adressiert; fehlt ein deklariertes Glied: `provenance_gap_mining` (sichtbar).
- **S7-A2 (Promotionsfähigkeit, deklarativ):** Ein geschlossenes Artefakt ist eine **promotionsfähige Cell** für SCALE-2 — die Promotion selbst geschieht ausschließlich via ScaleRatchet nach `Close(Red_{SCALE-1})` (S15/L10) und wird hier nur als Fähigkeit deklariert, nicht ausgeführt.
- **S7-DoD⁺:** erweiterte Kette ∧ Zwei-Digest-Modell unverändert ∧ keine „schon Workbody"-Behauptung.

### S8-Amendment (Bibliothek: erweiterter Bestand)

- **S8-A1 (Neue Asset-Typen):** Crystals (als eigenständige Zeugen), PhaseBlocks (akzeptierte Exemplare als Referenz), **WorkbodyBlueprints** (zertifizierte HBM-Kristalle, PHC-kompatibel), DomainNorms, BridgeNorms (Port-Typ, leer bis L9), sowie **Negativ-Cubes je neuer Gate-Klasse**: `external_drift_detected`, `wrap_support_violation`, `area_class_mismatch`, `ratchet_lock_without_evidence`, `score_as_gate_attempt`, `unbounded_cloning`, `frontier_desync`.
- **S8-A2 (Eintritt):** Jeder Blueprint tritt nur zertifiziert (CrystalFinalization) und selbst-validiert ein (S8.2 gilt); Registry speichert Provenienz, Version, Score (als Metadatum), Domänenprofil (HBM-18).
- **S8-A3 (Perspektive):** Referenz-/Negativ-**Workbodies** je SCALE als künftige Slots deklariert (Befüllung PL-geführt).
- **S8-DoD⁺:** neue Typen + Negativ-Cubes CI-gewacht (Regressionswächter erweitert) ∧ kein unvalidierter Eintritt.

### S9-Amendment (Persistenz: HyperDAG-präziser Ledger)

- **S9-A1 (Ledger-Präzisierung):** Der Ledger **ist** die linearisierte Commit-Projektion des PhaseBlock-HyperDAG: `Ledger = CommitProjection(H)`. Der CAS adressiert zusätzlich: PhaseBlocks, MEF-Evidence-Payloads, Frontier-Schnitte, Promotion-Objekte, NexusMiningCapsule-Vorformen, Replay-Packs (RD + Inputs + Entscheidungen + Evidence-Refs als ein rekonstruierendes Bündel). Konsistenzprüfung `verify_ledger` wird ergänzt um `verify_hdag_projection` (`ledger_hdag_mismatch` blocking).
- **S9-A2 (Provenienz):** Source→Facet→Adapter→Candidate→Crystal→Artefakt als durchgängige content-adressierte Kette abfragbar.
- **S9-A3 (Kein Blockchain-Import):** ausdrücklich: kein Konsensnetz, keine Token, keine Miner — die Struktur ist HyperDAG + append-only Projektion (Driftverbot 2).
- **S9-DoD⁺:** neue Adressierbarkeit ∧ Projektions-Prüfung ∧ Zwei-Speicher-Modell/Sync-Fähigkeit unverändert.

---

## 3 — PHASE 5: S10–S14-Amendments (Abnahme, Delivery, Doku, Governance, CoreExtension)

### S10-Amendment (gestufte DoD-Matrix)

- **S10-A1:** Die DoD-Matrix a–f (Rebase-Konsolidierung §5) wird Bestandteil von S10: **a** ProductDoD(Document) — unverändert, bleibt 1; **b** DomainCatalogDoD; **c** RebaseDoD; **d** SpiralProcessDoD; **e** MiningChassisDoD; **f** S15ReadinessDoD. Dazu **PlatformFoundationDoD = c ∧ d ∧ e**.
- **S10-A2:** Keine Stufe darf eine andere überdecken: Jede Vollständigkeitsaussage nennt ihre Stufe. „Fertig" ohne Stufenangabe ist unzulässig (Anti-Overclaim).
- **S10-DoD⁺:** Matrix normativ ∧ Produkt-Kerntest + Motor-Kerntest unberührt.

### S11-Amendment (Delivery: PL-gestufte Sichtbarkeit)

- **S11-A1:** Die Auslieferung darf neue Layer (PhaseBlocks, Ratchets, HBM) **sichtbar führen** — jede Funktion trägt im Produkt ihr PL; Funktionen unter PL4 sind als „Vorschau/Struktur" gekennzeichnet, nie als produktreif. `feature_maturity_overclaim` ist ein Release-blockierendes Residuum.
- **S11-A2:** Update-DoD prüft zusätzlich alle neuen Negativ-Cubes (S8-A1); ein Update, das ein Spiral-/HBM-Gate schwächte, ist nicht auslieferbar.
- **S11-DoD⁺:** PL-Kennzeichnung ∧ erweiterter Regressionswächter im Update-Kanal ∧ Konsolenfreiheit unverändert.

### S12-Amendment (Operator-Doku ohne mathematische Überladung)

- **S12-A1 (Vier neue Operator-Lesarten, klartextlich):** *Phase & BlueCube:* „Ein Arbeitsabschnitt ist fertig, wenn er geprüft, belegt und wiederholbar ist — dann rastet er ein und fällt nicht zurück." *Skala & RedCube:* „Wenn alle Abschnitte einer Stufe eingerastet sind, wird die ganze Stufe zu einem Baustein der nächsten." *Ratchet & Frontier:* „Die Anzeige zeigt, was eingerastet ist, was hält, und woran es hängt — nichts rastet ohne Beleg." *HBM-Kandidat:* „Das System kann Bauplan-Kandidaten vorschlagen; eine Zahl ordnet sie, aber nur die Prüfung entscheidet." *Nexus (Ausblick):* als künftige Verbindungsschicht benannt, ohne Funktionsversprechen.
- **S12-A2:** Alle Erklärungen stehen unter der Claim-Schranke; keine Winkel-/Spiralen-Mystik, keine Naturgesetz-Behauptung.
- **S12-DoD⁺:** vier Lesarten ∧ Ethos-Vermittlung erweitert ∧ Doku=Verhalten.

### S13-Amendment (Governance: Boundary-Freiheit, Capability, Mining-Safety)

- **S13-A1 (Neue Governance-Typen):** **SourceHorizon** (deklarierte Menge zulässiger Quellen/Korpora je MiningQuery; Exploration außerhalb ⇒ `exploration_out_of_horizon`, blocking — A8); **CapabilityLock** (jede materielle Fähigkeit — Lauf, Promotion, Klonung, Export — ist einzeln gelockt; Text handelt nicht: nur **ActionCandidates** unter Lock + Gate + Evidence werden ausgeführt); **ExplorationPolicy** (RD-gebundene Budgets, seeded randomness, geloggte PhaseSwitches — HBM 5.6/HBM-10).
- **S13-A2 (Neue Produkt-Invarianten):** **PROD-INV-9** Score ranked, Gate decides — keine Kennzahl akzeptiert/blockiert/finalisiert (A7, verschärft V1); **PROD-INV-10** Exploration nur innerhalb SourceHorizon × DomainBoundary × ScaleProfile × CapabilityProfile × Replay-Vertrag (A8); **PROD-INV-11** keine unkontrollierte Operator-Klonung: Budget `|descendants|≤B_O`, TTL, LedgerRef, Ergebnisse gegatet; Klonung im Produkt **fail-closed deaktiviert** bis CapabilityLock implementiert (R-13); **PROD-INV-12** ProjectionPacket-Vertrag: kein Agent trifft globale Architektur-/Promotions-Entscheidungen außerhalb seines Packets.
- **S13-A3 (HBM-Safety):** Safety-by-abstraction ist Governance-Norm: aus Rohquellen werden nur formale Strukturmuster extrahiert; Tarn-/Umgehungs-/Offensiv-/Selbstvermehrungsinhalte sind ausgeschlossene Scope-Klassen (Risk-Gates, HBM-16).
- **S13-DoD⁺:** Typen + PROD-INV-9..12 ∧ Verfassung bleibt Monolith (append-only) ∧ keine Verbots-Lockerung.

### S14-Amendment (CoreExtension: Kinematik-Erhalt)

- **S14-A1 (Zusätzliche Beweispflicht für jede Extension):** *Kinematik-Erhalt* — eine CoreExtension darf **WrapProjection-Pflichten, Gate-Booleschheit, Replay-Identität, Residuen-Sichtbarkeit und Ratchet-Irreversibilität nicht lockern**. Prüfbar: alle Spiral-/PhaseBlock-Negativ-Cubes bleiben rot; `extension_breaks_wrap`, `extension_relaxes_ratchet` ⇒ Ablehnung.
- **S14-A2 (Kompatibilitätsrichtung):** Neue Kern-Fähigkeiten müssen an den Erweiterungspunkten **spiral-/HBM-kompatibel** andocken (ein neuer Operator deklariert sein Verhalten unter `W_s`; eine neue Gate-Klasse fügt sich in Ratchet-`advance/hold/lock`); die Strukturpause selbst läuft **durch** S14, nie daran vorbei — der Rebase erzeugt keine zweite vertikale Achse.
- **S14-DoD⁺:** Beweispflicht-Zeile ergänzt ∧ Extension-Invarianz erweitert um F1–F6 ∧ ein Wächter (S8.3) prüft weiterhin alles.

---

## 4 — Abnahme der Amendment-Schicht

```
DoD(REBASE_S1-S14) = 1 ⟺
    für jedes Si: Amendment-Block vorhanden ∧ DoD(Si) ungebrochen ∧ FoundationAligned ∧ NoNewSilentResidue
  ∧ ein Adapterbegriff (S1-A1-Mapping) ∧ PL-Nomenklatur vollzogen
  ∧ Checkpoint/PhaseBlock-Trennung, Ledger=CommitProjection, Score-ranked/Gate-decides überall konsistent
  ∧ alle neuen Gates boolesch, fail-closed, begründet; alle neuen Residuen sichtbar
  ∧ Einarbeitung in Einzeldateien als R-9 geführt (Amendment-Modell, append-only)
```

*Ende der Nachziehblöcke S1–S14.*
