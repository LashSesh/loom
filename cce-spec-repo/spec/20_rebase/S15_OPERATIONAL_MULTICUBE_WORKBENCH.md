# CCE — DETAIL-SPEZIFIKATION S15: OPERATIONAL MULTICUBE WORKBENCH

**Die skalenadaptive Arbeitskörper-Ebene.** S15 hebt die CCE von einer Artefakt-Engine (SCALE-1) zu einer **skalenadaptiven Arbeitskörper-Engine**: dieselbe Closure-Geometrie — BlueCube schließt Phase, RedCube schließt Skala — arbeitet auf jeder Stufe der Skalenleiter, getrieben von Spiralprozess und Multi-Ratchet, gespeist vom HBM-Mining-Chassis, gebunden an Evidence/Replay/Ledger.

**Status:** Detail-Spezifikation v1.0, auf Bauverfassungs-Tiefe. Entstanden **nach** `S15Ready = 1` (Rebase-Konsolidierung §6) als homogenes Produkt der Rebase — kein Anbau, keine Minimallösung. Baut auf: Bauverfassung, S1–S14 **mit** Amendment-Schicht, Systemlandkarte vNext (L10), Unified Normal Form.

**Verankerte Grundaxiome:** A1–A8 der Rebase (Fundament führt; Blue/Red/Skalenleiter; Expansion intrinsisch/Wicklung extrinsisch/Ratchet irreversibel/Gate entscheidet/Replay sichert/Crystal kondensiert; Multi-Ratchet-Kaskade; HBM als Chassis; **Score ranked, Gate decides**; Freiheit innerhalb Boundary).

---

## S15.0 — Einordnung & Leitsatz

Bisher endet die Produktreise bei einem geschlossenen **Artefakt** (SCALE-1). Reale Arbeit besteht aber aus Läufen, Workflows, Workspaces, Systemen — höheren Skalen, die dieselbe Schließungsdisziplin verdienen. S15 liefert die Werkbank dafür.

Leitsatz:

> *Ein Arbeitskörper wächst nur durch geschlossene Phasen und promoviert nur durch geschlossene Skalen. Innen ist unbegrenzte Variations- und Orbitfreiheit erlaubt; nach außen wird jede Drift durch Boundary, Wicklung, Gate, Evidence und Replay annulliert. Nichts rastet ohne Beleg; nichts steigt ohne Schluss; jeder offene Punkt bleibt sichtbar.*

Zwei Grundfesten:

- **Dieselbe Geometrie auf jeder Skala.** S15 erfindet keinen neuen Abschlussbegriff: `Close(Blue) ⇒ PhaseBlock`, `Close(Red) ⇒ Promote` sind die eine Regel, skalenindiziert. Der Motor-Kerntest ist ihre SCALE-1-Instanz; MultiScaleClosure (§S15.13) ihre Leiter-Form.
- **Text handelt nicht.** Auf der Werkbank wird jede materielle Wirkung als **ActionCandidate** geführt und nur unter **CapabilityLock + Gate + Evidence + Replay** ausgeführt (§S15.10). Kein Agent — auch nicht die KI-Kanzel, auch kein Mining-Klon — trifft globale Entscheidungen außerhalb seines ProjectionPackets.

---

## S15.1 — ScaleAdapter

Das skalenachsige Gegenstück zum `DomainAdapter` (horizontale Achse) und zur `CoreExtension` (vertikale Kernachse):

```
ScaleAdapter<s> = (
    CellType_s,        // was auf SCALE-s eine Zelle ist (SCALE-1: Artefakt; SCALE-2: Lauf; …)
    PhaseSet_s,        // die Phasen p ∈ P_s der Skala (die PhaseLadder)
    CloseBlue_s,       // domänen-/skalenkonkrete Close(Blue_{s,p})-Bedingung
    CloseRed_s,        // Close(Red_s)-Bedingung (Seams, Replay, Residuen)
    WrapPolicy_s,      // W_s: Supportmaß µ_s, Boundary B_s, ε_s, DispersionProfile
    PromotionGate_s,   // ScaleRatchetGate: Crystal_s → Cell_{s+1}
    ResidueVocab_s,    // skalenspezifische Residuen
    ReplayContract_s   // RD-Bindung je Skala
)
```

**Adapter-Parität in der Skalenachse:** Jede aktivierte Skala implementiert **denselben** ScaleAdapter-Vertrag vollständig (Modellbaukasten-Prinzip, jetzt vertikal entlang der Leiter). `check_scale_adapter_parity` prüft maschinell. **Reifegrad:** SCALE-1 ist über S1–S13 PL4-getragen; SCALE-2…8 sind hier strukturell spezifiziert, ihre Produktreife ist PL-geführtes Residuum (S15-R1) — **nicht behauptet**.

Kompositionsregel der drei Achsen: `DomainAdapter × ScaleAdapter` = eine Domäne auf einer Skala; `CoreExtension` wächst den Kern unter beiden; keiner ersetzt einen anderen.

## S15.2 — Operational Multicube

Der **laufende Arbeitskörper**:

```
Multicube = ( {Blue_{s,p}}, {Red_s}, H, MR, Apt, RD, L )
```

— die skalen- und phasenindizierte Menge der Cubes, ihr PhaseBlock-HyperDAG `H`, die Multi-Ratchet-Kaskade `MR`, das Aperturfeld `Apt` (Binnenpupille: sektorierte Radfenster/FBC über dem gewickelten Prozess — dieselbe Apertur beobachtet und webt), der RunDescriptor-Raum und die Ledger-Projektion. Der Multicube **ist** die Werkbank-Sicht auf einen Workspace: alles, was darin geschieht, geschieht als BlueCube-Arbeit, PhaseBlock-Accept, Ratchet-Lock oder Promotion — nichts daneben.

## S15.3 — Workbench Capsule

Die gekapselte Arbeitseinheit der Werkbank (Produktform der EphemeralMiningCell, verallgemeinert):

```
Capsule = (scope, budget, projection, allowedOps, gates, evidenceSchema, ttl, ledgerRef)
```

Eine Capsule bearbeitet **nur** ihren projizierten Teilraum (ProjectionPacket-Vertrag); sie kann eine Mining-Capsule (L7), eine Domänen-Arbeitszelle (SCALE-0/1) oder eine Lauf-Zelle (SCALE-2) sein. **Dissolution:** Nach Ablauf/Abschluss verschwindet die Capsule; persistiert werden nur Trace, Evidence und Kristalle (HBM-Axiom 5.4). Die `NexusMiningCapsule`-Vorform (HBMFile) ist das Austauschformat einer Capsule; ihre Vollform kommt mit der Nexus-Foundation (R-1).

## S15.4 — BlueCube / RedCube / PhaseLadder / ScaleLadder

Übernommen als normative Kerne (Unified Normal Form, Formel 2):

- **BlueCube** `Blue_{s,p} = (Σ,Ω,B,K,Cand,G,R,L)`; `Close(Blue) ⟺ Typed ∧ BoundaryValid ∧ Gate=Pass ∧ Evidence=1 ∧ Replay=1 ∧ WrapStable` ⇒ **PhaseBlock**.
- **RedCube** `Red_s = ({Blue_{s,p}}, E_phase, E_seam, E_gate, Close_s)`; `Close(Red) ⟺ ∀p Close(Blue) ∧ SeamsValid ∧ Replay ∧ Resid sichtbar` ⇒ **Promotion**.
- **PhaseLadder** = die geordnete Phasenfolge einer Skala (SCALE-1-Beispiel: die sechs Reisephasen aus S2-A1). **ScaleLadder** = SCALE-0…SCALE-8 (Unit → Artifact → Task/Run → Workflow → Workspace → System/Product → Infrastructure → Ecosystem → Normic Memory). Promotion nur leiterweise, nie überspringend.

## S15.5 — PhaseBlock-HyperDAG

Die kausale Commit-Ordnung der Werkbank: PhaseBlocks (10-Tupel, Accept-8-Kriterien) in `H = (V, E_dep, E_seam, E_phase, E_scale, E_commit)`; **Ledger = CommitProjection(H)**; Frontiers (HDAG + Ratchet) mit Sync-Pflicht (`frontier_desync` blocking); **CrystalConsensus** `= Accept ∧ ParentClosure ∧ FrontierCompatible ∧ LedgerAppendable` als struktureller Konsens — **kein** Blockchain-/BlockDAG-Import, keine Miner, keine Token. Kandidat ≠ Commit: ein Block ohne Accept ist sichtbarer Kandidat oder Hold, nie still verbucht.

## S15.6 — Spiralprozess / WrapProjection / AreaClass

Die Kinematik der Werkbank (Unified Normal Form, Formeln 3–4): jeder Fortschrittsschritt ist `Step_s = W_s ∘ E_{λ,α}` — intrinsische Expansion, extrinsisch gewickelt; **AreaClass-Invarianz** `[E^k(X)]_{µs} = [X]_{µs}` hält den Support-/Budgetrahmen; die **Spiraladresse** `(s,p,k,j,θ,r)` ist die maschinenlesbare Ortsangabe jeder Zelle; **DispersionProfile** (Dyadic/Golden/Rational/Adaptive) ist deklarierter RD-Parameter — *Profil, nicht Dogma*. Das Cockpit zeigt WrapStability und AreaClass-Status (S3-A4); Verletzungen sind die Spiral-Residuen (S15.14).

## S15.7 — Multi-Ratchet-Kaskade

Irreversibilität mit Kaskadendisziplin (Formel 5): CellRatchet→UnitCrystal, PhaseRatchet→PhaseBlock, RingRatchet→ScaleCrystal, ScaleRatchet→Cell_{s+1}, NexusRatchet→NexusClass (Port). `Lock(R_{n+1}) = 1 ⇒ ∀R_i ≺ R_{n+1}: Lock(R_i)=1 ∨ Resid(R_i) sichtbar` (Counter-Horizon). Intrinsische Phase rotiert frei; der Commitzähler steigt **nur** unter Gate/Evidence. **Externe Drift wird annulliert**: `d⊥ ≠ 0 ⇒ Resid_drift sichtbar` — die Kaskade verhindert Boundary-/Scope-/Claim-Ausbruch, während Innenvariation produktiv bleibt.

## S15.8 — HBM Mining-Chassis (vorgelagert)

Die Werkbank **findet** closure-würdige Blueprints, bevor sie baut: `Korpus/Domäne/Wunsch → Facets → Adapter → Cube/HDAG → Skeleton/JT → Candidate → ExclusionGate → LayerExpansion → CrystalFinalization → Registry/Replay` — mit Spiral-Lesart `Candidate_{k+1} = W(E(Candidate_k))`, ausschließlich innerhalb SourceHorizon × DomainBoundary × ScaleProfile × CapabilityProfile × Replay-Vertrag (A8). Neutralisierte Operatorik (Rebase §3), Axiome 5.1–5.7, HBM-01…20 als Abnahmekatalog, FixpointCalibration mit geloggten PhaseSwitches, EphemeralMiningCells + BoundedOperatorSpecialization (Budget/TTL/Ledger; im Produkt fail-closed deaktiviert bis CapabilityLock aktiv, R-13). **Score ordnet Kandidaten; Gate/Closed/Replay entscheiden** — der Schwellwert θ_D ist Vorauswahl, nie Abnahme.

## S15.9 — Kompositionsmatrix: DomainAdapter × ScaleAdapter × SourceHorizon × HBM

Jeder Werkbank-Auftrag ist ein Punkt in dieser Matrix:

```
Auftrag = (Domäne D, Skala s, SourceHorizon SH, MiningQuery? q)
   ⇒ Capsules mit projection = Π_D × Π_s, boundary = B_D ∩ B_s ∩ SH,
     gates = G1..G7 ∪ DocG/DomG_D ∪ SpiralGates ∪ HBM-Gates,
     evidence = E_D ∪ MEF, replay = RD(D,s,SH,q)
```

Die vier Achsen komponieren, ohne sich zu ersetzen: die Domäne gibt Bedeutung, die Skala gibt Reichweite, der SourceHorizon gibt die Explorationsgrenze, das Chassis liefert Kandidaten. Fehlt eine Deklaration ⇒ Hold (`boundary_unspecified`), nie stiller Default.

## S15.10 — ActionCandidate / CapabilityLock

**Text handelt nicht.** Jede beabsichtigte Wirkung (Lauf starten, Block akzeptieren, promovieren, exportieren, minen, klonen) ist ein **ActionCandidate**:

```
ActionCandidate = (kind, scope, preconditions, evidenceRequired, capability, rd)
Execute(a) = 1 ⟺ CapabilityLock(a.capability) = offen für diesen Operator/Kontext
              ∧ Gate(a) = Pass ∧ Evidence(a) = 1 ∧ Replay-gebunden ∧ Operator-bestätigt (materiell)
```

CapabilityLocks sind einzeln, fail-closed, ledger-protokolliert (S13-A1); harte Gates bleiben nicht übersteuerbar; die KI-Kanzel kann ActionCandidates **vorschlagen**, nie ausführen (S3.4 unverändert). `capability_violation` ist blocking.

## S15.11 — Evidence / Replay / Ledger

Jeder Accept trägt **EvidenceComplete** (MEF-gebundene Payloads), jeder Lauf einen **Replay-Pack** (RD + Inputs + aufgezeichnete Entscheidungen + Evidence-Refs), der Ledger ist die CommitProjection des HyperDAG (S9-A1). **Proof-of-Closure** ist das eine Commit-Kriterium: `PoC(x) ⟺ Gate=Pass ∧ Evidence ∧ Resid sichtbar ∧ Replay ∧ Reanalyze(x) ∼ Crystal(x)` — der Motor-Kerntest ist seine SCALE-1-Instanz. Ältere Prädikate (Proof-of-Resonance u. ä.) sind höchstens optionale Gate-Prädikate **innerhalb** von PoC.

## S15.12 — Nexus- / BridgeNorm-Anschluss (Port, spätere Foundation)

Die Werkbank deklariert die Anschlüsse, ohne die Foundation zu behaupten (R-1): **Domain = Apertur**, **BridgeNorm = Wicklung zwischen Aperturen**, **Nexus-Runde = globaler Ratchet (NexusRatchet → NexusClass)**, **Quiescence** = keine neue zulässige Strukturklasse in der nächsten Wickelrunde; **SourceHorizon** ist bereits aktiv als Governance-Typ (S13-A1). BridgeProfile-Slots (S1-A2) bleiben PL0-Ports. **Der Nexus darf niemals frei scrapen:** jede Quelle liegt innerhalb eines deklarierten SourceHorizon, jede Brücke innerhalb Boundary + Replay — sonst Hold/Residue, nie Pass.

## S15.13 — MultiScaleClosure

Der Leiter-Abschlussbegriff:

```
MultiScaleClosure(s*) = 1 ⟺ ∀ s ≤ s*:  Close(Red_s) = 1
                            ∧ Promote(s→s+1) via ScaleRatchet (Gate/Evidence/Replay/Resid/AreaClass)
                            ∧ Frontier-Sync auf jeder Stufe
                            ∧ jede Nicht-Schließung als Counter-Horizon sichtbar
```

SCALE-1-MultiScaleClosure = geschlossene Produktreise (S2-A1) = Produkt-Kerntest. Höhere s* sind strukturell definiert; ihre erste grüne Instanz ist der jeweilige Skalen-Kerntest (PL-geführt, S15-R1). Kein s* wird behauptet, dessen Kerntest nicht grün ist.

## S15.14 — S15-Gates und Residuen

**Gates (alle boolesch, fail-closed, begründet, kein Score):** die 8 Spiral-Pflichtgates — WrapGate, AreaClassGate, DriftGate, PhaseRatchetGate, ScaleRatchetGate, DispersionProfileGate, ApertureSectorGate, SpiralReplayGate — plus ExclusionGate/LayerExpansion-Gates/CrystalFinalization (L7), CapabilityGate (Locks), FrontierSyncGate, PromotionGate je Skala, und für Familie-P-Inhalte unverändert das ProfessionalReviewGate.

**Residuen-Vokabular (jedes sichtbar):** `wrap_support_violation`, `area_class_mismatch`, `external_drift_detected`, `phase_profile_undeclared`, `dispersion_not_replayable`, `ratchet_lock_without_evidence`, `bluecube_not_closed`, `redcube_seam_gap`, `scale_promotion_without_wrap_stability`, `aperture_sector_scope_leak`, `fibonacci_dogma_import`, `golden_angle_unjustified`, `frontier_desync`, `capability_violation`, `exploration_out_of_horizon`, `unbounded_cloning`, `score_as_gate_attempt`, `boundary_unspecified`, `provenance_gap_mining`, `feature_maturity_overclaim`. Für jedes existiert ein Negativ-Cube in der Bibliothek (S8-A1) — der eine Regressionswächter wacht auch hier.

## S15.15 — Abnahme (DoD dieser Ebene)

```
DoD(S15) = 1 ⟺
    ScaleAdapter-Vertrag definiert ∧ check_scale_adapter_parity spezifiziert (§S15.1)
  ∧ Operational Multicube, Workbench Capsule, Blue/Red/PhaseLadder/ScaleLadder,
        PhaseBlock-HyperDAG, Spiralprozess/Wrap/AreaClass, Multi-Ratchet definiert (§S15.2–S15.7)
  ∧ HBM-Chassis vorgelagert integriert; Score ordnet, Gate entscheidet (§S15.8)
  ∧ Kompositionsmatrix Domain×Scale×SourceHorizon×HBM definiert (§S15.9)
  ∧ ActionCandidate/CapabilityLock: Text handelt nicht; jede Wirkung gelockt+gegatet+belegt (§S15.10)
  ∧ Evidence/Replay/Ledger = CommitProjection; PoC als einziges Commit-Kriterium (§S15.11)
  ∧ Nexus nur als Port; kein freies Scrapen; SourceHorizon bindend (§S15.12)
  ∧ MultiScaleClosure definiert; SCALE-1-Instanz = Produkt-Kerntest; höhere Skalen PL-geführt (§S15.13)
  ∧ alle S15-Gates boolesch/fail-closed/begründet; alle Residuen sichtbar + Negativ-Cubes (§S15.14)
  ∧ keine Fundamentinvariante (F1–F6, INV-1..14, V1–V10) gelockert; Motor- und Produkt-Kerntest unberührt
  ∧ Engine-DoD, DoD(S1..S14)+Amendments, DoD-Matrix a–f bleiben unberührt
```

**Was S15 ausdrücklich nicht behauptet:** dass alle Domänen PL4-produktreif sind; dass der Nexus Quellen frei scrapen darf; dass Score Gates ersetzt; dass Fibonacci, Goldener Winkel oder Spiralen Naturgesetze des Systems sind; dass Blockchain oder BlockDAG importiert werden; dass Agenten globale Entscheidungen treffen dürfen; dass S15 eine Minimallösung ist.

**Was S15 behauptet:** dass die CCE durch S15 von der Artefakt-Engine zur skalenadaptiven Arbeitskörper-Engine wird; dass BlueCube Phase schließt und RedCube Skala; dass der Spiralprozess intrinsische Expansion und extrinsische Wicklung koppelt; dass die Multi-Ratchet-Kaskade externe Drift verhindert; dass HBM closure-würdige Blueprint-Kandidaten vorgelagert mined; dass jede Promotion Gate, Evidence, Replay und sichtbares Residuum braucht; und dass jeder offene Punkt als Residuum geführt wird.

## S15.16 — Sichtbare Residuen dieser Spezifikation

- **S15-R1 (Skalenreife):** SCALE-2…8 strukturell spezifiziert, Produktreife PL-geführt; erster Skalen-Kerntest je Stufe ausstehend (= R-10).
- **S15-R2 (ScaleAdapter-Instanzen):** SCALE-1-Adapter aus S1–S13 ableitbar; SCALE-2 (Task/Run) als nächste Instanz empfohlen; Instanziierung ausstehend.
- **S15-R3 (Nexus-Vollform):** Ports definiert; Foundation ausstehend (= R-1).
- **S15-R4 (Capsule-Feinformat):** NexusMiningCapsule-Vorform normiert (K1–K6); Byte-Format beim Bau (= R-6 verwandt).
- **S15-R5 (Klonungs-Aktivierung):** BoundedOperatorSpecialization fail-closed deaktiviert bis CapabilityLock-Bau (= R-13).
- **S15-R6 (Werkbank-UI):** Verträge fixiert (S3/S6-Amendments); Feindesign später (= R-12).

## S15.17 — Anschluss

Mit S15 ist die Rebase-Kette geschlossen: *Strukturpause spannt das Fundament. Spiralprozess erklärt die Kinematik. HBM findet closure-würdige Blueprints. S1–S14 sind homogen nachgezogen. S15 ist die skalenadaptive Operational-Multicube-Workbench* — dieselbe Abschlussformel, jetzt leiterfähig: Was auf SCALE-1 `Reanalyze(Materialize(LOOM(Project(PHC(Crystal))))) ≃ Crystal` ist, ist auf jeder Skala `PoC` unter MultiScaleClosure. Der Bau folgt der Bauverfassung Teil 9 plus Handoff (Rebase §8); nichts davon lockert eine Invariante, und jeder offene Punkt steht sichtbar in den Registern.

*Ende der Detail-Spezifikation S15.*
