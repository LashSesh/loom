# CCE — SYSTEMLANDKARTE vNEXT

**Die nachgezogene Gesamtlandkarte nach der Korpus-Strukturpause.** Ersetzt nicht die Produktsicht der bisherigen Systemlandkarte (Kern + 4 Ringe), sondern **unterspannt** sie: Die bisherige Karte zeigte das Produkt um den Motor; diese Karte zeigt die vollständige Fundament-zu-Produkt-Vertikale, in die alles homogen eingespannt ist.

**Status:** Landkarte vNext v1.0, normativ. Amendment zur SYSTEMLANDKARTE.md (append-only, kein stilles Überschreiben).

---

## 1 — Das Ebenenmodell L0–L11

```
L11  Product / Delivery / Governance / Docs        ← die Tapete (S2–S13)
L10  Operational Multicube / S15                   ← skalenadaptive Workbench
L9   Nexus / SourceHorizon / BridgeNorm            ← spätere Foundation (Ports)
L8   DomainCatalog / DomainAdapter                 ← horizontale Achse (S1)
L7   HBM Mining-Chassis                            ← vorgelagerte Blueprint-Gewinnung
L6   Spiralprozess / WrapProjection / Multi-Ratchet ← Kinematik
L5   BlueCube / RedCube / Skalenleiter             ← Phasen- und Skalengeometrie
L4   PhaseBlock-HyperDAG                           ← kausale Commit-Ordnung
L3   PSP Execution Kernel                          ← Maschinengewinde (Σ,Ω,RD,Trace,Evidence,Manifest)
L2   PHC / LOOM / Workbody                         ← Transport & generative Auswebung
L1   Crystal / Boundary / Gate / Replay            ← Closure-Kern
L0   Irreduzibles Fundament                        ← MERKABA·LOOM·PHC·BCIK·CL·CCC·Panoptikum·QLOGIC·TAT + F1–F6
```

**Leserichtung:** L0 bestimmt die Geometrie aller darüberliegenden Ebenen (A2: Fundament bleibt führend). Keine höhere Ebene darf eine niedere verbiegen (Driftverbot „Tapetenkrümmung"). Die vertikale Achse S14 (CoreExtension) durchzieht L0–L8 als einziger sanktionierter Wachstumspfad des Kerns.

## 2 — Die Ebenen im Einzelnen

| Ebene | Inhalt | Trägt | Quelle | Status |
|---|---|---|---|---|
| **L0 Irreduzibles Fundament** | MERKABA, LOOM, Topologisches Panoptikum, Panoptische Instrumentenoptik, PHC, BCIK, Constraint Lattice, CCC, QLOGIC, Attraktortriangulation; Fundamentinvarianten **F1** Nullanker (Z0 ∉ X, Referenzbedingung für Π/Close/Replay), **F2** Boundary vor Absorption, **F3** Collect/Distribute-Dualität, **F4** Gate/Evidence/Replay vor Commit, **F5** 4π-Orientierungsschluss als dyadische Closure-Ordnung, **F6** Drift intern, nicht extern | alles | Korpus + Strukturpause §3 | ✓ fixiert |
| **L1 Closure-Kern** | Crystal, Boundary/Seam, Gate (boolesch, fail-closed, begründet), Residuum (stets sichtbar), Evidence, Replay, Ledger, Proof-of-Closure | L2–L11 | Bauverfassung | ✓ |
| **L2 PHC / LOOM / Workbody** | Transportform (PHC-Pakete), radiale Spindel, Nadelapertur=Radfenster, Materialisierung; **WorkbodyProjection**: Distribute in skalierte Arbeitskörper | Artefakt- und Workbody-Erzeugung | Bauverfassung + Strukturpause | ✓ (Workbody-Lesart neu) |
| **L3 PSP Execution Kernel** | `PSPcore = (Σ, Ω, RD, Trace, Evidence, Manifest)`; AssemblyGraph ↦ PhaseBlockHDAG; MEF-Block als Evidence-/Proof-Payload; Operator-/Gate-Registry; deterministisches Scheduling | konkrete Ausführung jeder PhaseBlock-Erzeugung | Strukturpause §4 (Klasse B, nach unten typisiert) | ◐ Rebase-normiert, Bau ausstehend |
| **L4 PhaseBlock-HyperDAG** | PhaseBlock (10-Tupel; Accept ⟺ Typed ∧ BoundaryValid ∧ SeamConsistent ∧ Gate=Pass ∧ EvidenceComplete ∧ ResidueVisible ∧ Replayable ∧ ReanalysisCompatible); `H=(V,E_dep,E_seam,E_phase,E_scale,E_commit)`; **Ledger = CommitProjection(H)**; Frontiers (HDAG/Ratchet) + Sync-Pflicht; CrystalConsensus | kausale Ordnung über allem Committen | Strukturpause §6/§8 | ◐ |
| **L5 BlueCube / RedCube / Skalenleiter** | `Blue_{s,p}` (lokaler Phasenraum), `Red_s` (Phasenmatrix); **SCALE-0…SCALE-8** (Unit → Artifact → Task/Run → Workflow → Workspace → System/Product → Infrastructure → Ecosystem → Normic Memory); `Close(Blue) ⇒ PhaseBlock`, `Close(Red) ⇒ Promote(s→s+1)` | Phasen-/Skalenabschluss | Strukturpause §5, Spiralprozess §3/§7 | ◐ |
| **L6 Spiralprozess / WrapProjection / Multi-Ratchet** | `z_{k+1}=W_s(E_{λ,α}(z_k))`; Spiraladresse `(s,p,k,j,θ,r)`; AreaClass `[E^k(X)]_{µs}=[X]_{µs}`; WrapProjection (4 Pflichten); Cell/Phase/Ring/Scale/Nexus-Ratchets; Kaskadenregel; Binnenpupille = Aperturfeld über gewickeltem Spiralprozess; FBC/ActiveSector; DispersionProfile (Profil statt Dogma); 8 Pflichtgates, 12 Residuen | Kinematik: Drift innen erlaubt, außen annulliert | Spiralprozessschicht | ◐ |
| **L7 HBM Mining-Chassis** | Pipeline: Korpus/Domäne/Wunsch → Facets → Adapter → Cube/HDAG → Skeleton/JT → Candidate → **ExclusionGate** → **LayerExpansion** → **CrystalFinalization** → Registry/Replay; Axiome 5.1–5.7; **FixpointCalibration/PhaseSwitchController**; **EphemeralMiningCells**; **BoundedOperatorSpecialization**; **DistributedMiningField** (Metrik); **NexusMiningCapsule-Vorform**; HBM-01…20; A7: Score ranked, Gate decides | vorgelagerte Gewinnung closure-würdiger BlueprintCandidates/WorkbodyBlueprints | HBM (neutralisiert, §3 der Rebase) | ◐ |
| **L8 DomainCatalog / DomainAdapter** | S1-Vertrag (11 Punkte) = **die** Adapterform; 213 Domänen / 16 Familien; PL0–PL4 (umbenannt); neue Slots: **ScaleSlot** (Blue/Red/PhaseBlock-Anschluss), **MiningProfile**, **BridgeProfile**, **ScaleProfile**; ProfessionalReviewGate (Familie P) | horizontale Achse | S1 + Katalog + Rebase-Amendment | ✓ Katalog / ◐ Slots |
| **L9 Nexus / SourceHorizon / BridgeNorm** | Domain-Nexus (hypertoroidale Wicklung: Domain=Apertur, BridgeNorm=Wicklung, Nexus-Runde=globaler Ratchet, Quiescence); SourceHorizon als Explorationsgrenze; NexusRatchet → NexusClass; Normic Memory (SCALE-8) | Cross-Domain-Ordnung | Spiralprozess §12 + Auftrag | ○ **spätere Foundation** — nur Typen/Ports (R-1); SourceHorizon als Governance-Typ bereits aktiv (S13) |
| **L10 Operational Multicube / S15** | ScaleAdapter, Operational Multicube, WorkbenchCapsule, PhaseLadder/ScaleLadder, MultiScaleClosure, ActionCandidate/CapabilityLock — die skalenadaptive Arbeitskörper-Engine | Produkt-Werkbank über allen Skalen | S15 (nach S15Ready=1) | ◐ spezifiziert (S15), Bau ausstehend |
| **L11 Product / Delivery / Governance / Docs** | die Tapete: S2 Reise, S3 Cockpit, S4 Wunsch, S5 Orchestrierung, S6 Inspektion, S7 Artefakt, S8 Bibliothek, S9 Persistenz, S10 Abnahme (DoD-Matrix a–f), S11 Auslieferung, S12 Doku, S13 Governance (+ SourceHorizon/CapabilityLock/ExplorationPolicy) | Bedienbarkeit | S2–S13 + Rebase-Amendments | ✓ Produkt-DoD(Dokument) / ◐ Amendments |

**Legende:** ✓ geschlossen · ◐ normativ spezifiziert, Einarbeitung/Bau ausstehend (sichtbares Residuum) · ○ spätere Foundation.

## 3 — Mapping: alte Landkarte → vNext

| Alt (Kern + 4 Ringe) | vNext |
|---|---|
| Kern (Werk/Motor, Bauverfassung) | L0–L2 (+ L3/L4 als neu eingezogener Unterbau des Committens) |
| Ring A (Sinn & Nutzer: S1, S2) | L8 (S1) + L11 (S2) |
| Ring B (Bedienung: S3–S7) | L11 |
| Ring C (Substanz: S8, S9) | L11 (mit L4-Ledger-Präzisierung in S9) |
| Ring D (Abschluss & Auslieferung: S10–S13) | L11 |
| Vertikale Achse S14 | quer durch L0–L8 (unverändert; + Kinematik-Erhalt-Pflicht) |
| — (neu) | L3 Execution-Kernel, L4 PhaseBlock-HyperDAG, L5 Blue/Red/Skalen, L6 Spiralprozess, L7 HBM, L9 Nexus-Ports, L10 S15 |

Die alte Karte bleibt als Produktsicht gültig; vNext ist die Fundament-Vertikale darunter. Beide beschreiben **dieselbe** Apparatur.

## 4 — Sichtbare Residuen dieser Landkarte

- L9 ist **Ports-only** (R-1): keine Nexus-Quellspezifikation vorliegend; keine Vollständigkeitsbehauptung.
- ◐-Ebenen benennen Einarbeitung/Bau als ausstehend (R-9 Amendment-Einarbeitung; Bau folgt Bauverfassung Teil 9 + Handoff §8 der Rebase).
- SCALE≥2-Produktreife ist **nicht** behauptet (R-10); die Skalenleiter ist Struktur, PL misst Reife.

*Ende der Systemlandkarte vNext.*
