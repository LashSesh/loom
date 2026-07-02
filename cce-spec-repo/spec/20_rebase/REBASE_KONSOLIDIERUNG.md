# CCE — KORPUS-REBASE-KONSOLIDIERUNG v1.0

**Fundament-Rebase von S1–S14 auf Strukturpause, Spiralprozessschicht und Hypercube Blueprint Mining.** Kein Anbau, keine Addition — eine fundamentgebundene Neubindung: `K+ = Normalize_F(Classify(R), K)`.

**Status:** Rebase-Vertrag und Konsolidierungsregister, normativ. Dieses Dokument bündelt: Rebase-Register (Phase 0), Impact Map S1–S14 (Phase 1), HBM-Neutralisierung (Phase 6), Unified Normal Form (Phase 7), S15-Readiness (Phase 8), DoD-Matrix, Residuenregister und Agent-Handoff. Die Systemlandkarte vNext und die konkreten Nachzieh-Blöcke liegen in eigenen Dokumenten (`SYSTEMLANDKARTE_vNEXT.md`, `REBASE_S1-S14.md`); S15 folgt erst nach S15Ready=1 (`S15_OPERATIONAL_MULTICUBE_WORKBENCH.md`).

---

## 0 — Rebase Executive Summary

Drei neue Fundamentressourcen sind eingegangen: **Korpus-Strukturpause v0.1** (Fundament-Rebase-Vertrag), **Korpus-Spiralprozessschicht v0.1** (Kinematik) und **Hypercube Blueprint Mining Framework v0.1** (vorgelagerte Mining-Schicht). Sie werden **nicht** als S15-Anbau behandelt, sondern als homogen nachgezogene Fundamentgeometrie:

1. **Fundament bleibt führend (A2).** MERKABA, LOOM, PHC, BCIK, Constraint Lattice, CCC, Panoptikum, QLOGIC, Attraktortriangulation und die CCE-Bauverfassung bilden das irreduzible Fundament `F`. Nichts aus den neuen Ressourcen steht darüber.
2. **PSP/MEF/AssemblyGraph werden nach unten typisiert.** Sie sind der **niedere Execution-Kernel** unter LOOM/CCE — Maschinengewinde der PhaseBlock-Ausführung, keine neue Theorie-Spitze.
3. **BlueCube/RedCube/Skalenleiter, PhaseBlock-HyperDAG und Multi-Ratchet** werden als Skalen- und Commit-Geometrie eingezogen: *BlueCube schließt Phase, RedCube schließt Skala, geschlossene Skala wird Zelle der nächsten Skala.* Der bestehende Ledger wird präzisiert als `Ledger = CommitProjection(HyperDAG)` — kein Blockchain-Import.
4. **Der Spiralprozess liefert die fehlende Kinematik:** intrinsische Expansion + extrinsische Wicklung (`z_{k+1} = W_s(E_{λ,α}(z_k))`), AreaClass-Invarianz, Drift innen erlaubt / außen annulliert, DispersionProfile statt Golden-Schnitt-Dogma.
5. **HBM wird chirurgisch neutralisiert** und als **Mining-Chassis** vor die Closure-Kette gesetzt: Korpus/Domäne/Wunsch → Facets → Adapter → Cube/HDAG → Skeleton/JT → Candidate → Gate → Crystal → Replay. Keine freie Agentenintelligenz; **Score ranked, Gate decides (A7)**.
6. **S1–S14 erhalten präzise Nachziehpflichten** (Impact Map §2, Nachzieh-Blöcke in `REBASE_S1-S14.md`), unter der Rebase-Regel `DoD(Si+) = DoD(Si) ∧ FoundationAligned(Si+) ∧ NoNewSilentResidue(Si+)` — keine bestehende DoD wird gebrochen, keine gelockert.
7. **Amendment-Modell statt Umschrift.** Konsistent mit dem append-only-Ledger-Prinzip (S13.7) werden die Nachziehungen als **normative Amendment-Schicht** geführt; die physische Einarbeitung in die 17 Einzeldateien ist ein benannter Redaktionsschritt (Residuum R-9, sichtbar).

**Ergebnis:** S15Ready = 1 auf Spezifikationsebene (§6), und S15 entsteht als homogenes Produkt der Rebase — die skalenadaptive Operational-Multicube-Workbench.

---

## 1 — PHASE 0: Rebase-Register, Neutralisierungs-Register, Residuen-Register

### 1.1 RebaseRegister (Begriffsklassifikation)

`Classify(R)` sortiert jeden Begriff der drei Ressourcen in genau eine Klasse:

| Klasse | Begriffe | Behandlung |
|---|---|---|
| **FUNDAMENT** (irreduzibel, führend) | Nullanker Z0 (F1), Boundary-vor-Absorption (F2), Collect/Distribute-Dualität (F3), Gate/Evidence/Replay-vor-Commit (F4), 4π/720°-Orientierungsschluss als dyadische Closure-Ordnung (F5), Drift-intern-nicht-extern (F6); Crystal, Seam/Mandorla, Radialspindel, Radfenster=Nadelapertur, Junction Tree/RIP, Chordalität, Residuum-Sichtbarkeit, Replay-Identität | Bereits im Fundament; die neuen Ressourcen **schärfen** sie (F1–F6 als explizite Invariantenliste übernommen), importieren sie nicht neu |
| **EXECUTION-UNTERBAU** (nach unten typisiert) | `PSPcore = (Σ, Ω, RD, Trace, Evidence, Manifest)`; MEF-Block (Evidence-/Proof-Payload); `AssemblyGraph AG = (N,E,Σ,Γ,O,Q,R) ↦ PhaseBlockHDAG`; Operator-/Gate-Registry; deterministisches Scheduling; Downcast/Audit (VBA-H-Kern) | Maschinengewinde **unter** LOOM/CCE: konkrete Ausführungsgrammatik der PhaseBlock-Erzeugung. Keine neue Theorie-Spitze (Driftverbot D1) |
| **SKALEN-/COMMIT-GEOMETRIE** | BlueCube `Blue_{s,p}=(Σ,Ω,B,K,Cand,G,R,L)`; RedCube `Red_s=({Blue_{s,p}},E_phase,E_seam,E_gate,Close_s)`; Skalenleiter SCALE-0…SCALE-8; `Promote(s→s+1) ⟺ Close(Red_s)=1`; PhaseBlock (10-Tupel); HyperDAG `H=(V,E_dep,E_seam,E_phase,E_scale,E_commit)`; `Ledger = CommitProjection(H)`; Frontiers (HDAG- und Ratchet-Frontier, Sync-Pflicht); CrystalConsensus; Proof-of-Closure; Completion Pressure | Neu eingezogen als Ebenen L4/L5 der Landkarte vNext; an F1–F6 gebunden |
| **SPIRALPROZESS** (Kinematik) | Spiralprozess `S=(Z,A,Θ,Λ,E,W,µ,G,R,L)`; Spiraladresse `a=(s,p,k,j,θ,r)`; Expansionsoperator E; WrapProjection W_s (4 Pflichten: Support-Boundedness, Seam-, Residue-, Replay-Preservation); AreaClass `[X]_{µ_s}`; Treppen-/Wickelgesetz `Step_s = W_s∘E_{λ,α}`; PhaseRatchet/RingRatchet/ScaleRatchet/CellRatchet/NexusRatchet; Multi-Ratchet-Kaskade; Binnenpupille = Aperturfeld über gewickeltem Spiralprozess; Focal Boundary Cut; ActiveSector; DispersionProfile (Dyadic/Golden/Rational/Adaptive); Discrete-to-Continuous Lift | Neu eingezogen als Ebene L6; Kinematik **unter** der Tapete, **über** dem Execution-Kernel |
| **MINING-CHASSIS** | HBM-Haupttupel `(D,A,C,E,X,H,S,J,F,O,Γ,Λ,Ω,R)`; HBM-Adapter `A_D=(Π,C,V,Can,σ,G,E,Ex)`; Facet `(id,type,scope,source,evidence,confidence)`; BlueprintCandidate; Pipeline-Phasen 0–9; Operatoralgebra (Extract…Certify); Mining-Axiome 5.1–5.7; Kalibrationstriplet `(ψ,ρ,ω)`; EphemeralMiningCells; BoundedOperatorSpecialization; NexusMiningCapsule-Vorform; Abnahmekatalog HBM-01…HBM-20 | Neu eingezogen als Ebene L7, **vorgelagert** zur Closure-Kette; alle esoterischen Rohnamen neutralisiert (§3) |
| **SPÄTERE FOUNDATION** | Domain-Nexus, SourceHorizon, BridgeNorm, NexusClass, Normic Memory (SCALE-8), hypertoroidale Wicklung, Quiescence | Nur als **Typen und Ports** geführt (Ebene L9); keine Quell-Spezifikation im Korpus vorliegend → sichtbares Residuum R-1, keine Vollständigkeitsbehauptung |
| **LEGACY/ALTLAST** (neutralisieren) | Blockchain-als-Kernpflicht, BlockDAG, Block, TIC, Proof-of-Resonance-als-Konsens, 5D-als-Ontologie, TriMöbius, Spirale-als-Naturgesetz, Auge/Rad, Bohrer, Singularität, Kosmokrator, Chronokrator, Pfauenthron, Seraphic Calibration, Heavenly Hosts, Fallen Seraph, FTCSA, Fibonacci-/Golden-Angle-Dogma | Neutralisierungsregister §1.2 und §3; **keine normative Fundamentrolle** |

### 1.2 NeutralizationRegister (Strukturpause §10, verbindlich übernommen)

| Legacy-Begriff | Korpus-Normalform |
|---|---|
| Blockchain | irreversible **Commit-Projektion eines HyperDAG**; kein dezentrales Konsenssystem als Kernpflicht |
| BlockDAG | **PhaseBlock-HyperDAG mit CrystalConsensus** |
| Block | **MEF-/Evidence-gebundener PhaseBlock** |
| TIC | Crystal, MemoryUnit oder NormCandidate, je Kontext |
| Proof-of-Resonance | optionales Gate-Prädikat **innerhalb** Proof-of-Closure, nie globaler Konsens |
| 5D | Profil-/Koordinatenmodell, keine ontologische Dimension |
| TriMöbius | historische Bezeichnung der Nullanker-/Phasen-/Commit-Zeitordnung |
| Spirale | **Ratchet-getriebene Phasenprogression unter Boundary-Stabilisierung** (Einhüllende diskreter Locks) |
| Auge/Rad | Nadelapertur, Radfenster, lokaler **Focal Boundary Cut** |
| Bohrer | **Completion Pressure + Ratchet-Kaskade**, keine physische Durchbohrung |
| Singularität (Strukturpause-Lesart) | boundary-regularisierter Nullanker **oder** Crystal-Kondensation |
| Fibonacci/Goldener Winkel | **optionales DispersionProfile**, RD-gebunden; kein Korpusaxiom (Residuen `fibonacci_dogma_import`, `golden_angle_unjustified`) |

Die HBM-spezifischen Neutralisierungen stehen in §3.

### 1.3 ResidueRegister (vorab; Vollregister in §7)

Nicht assimilierte oder bewusst zurückgestellte Teile — **sichtbar, nicht verschwiegen**: Nexus/SourceHorizon/BridgeNorm (R-1), FTCSA-Feldformalismus über Metrik-Rolle hinaus (R-2), ZK-ML/Token-Ökonomie-Reste (R-3, verworfen), 720°-Physiklesarten (R-4, als dyadische Closure-Ordnung geführt, nie als Physik), operative Klonungs-/Schwarm-Details jenseits Budget/TTL/Gate (R-5, per Safety-by-abstraction ausgeschlossen).

### 1.4 Nomenklatur-Normalisierung (Rebase-Fund: dreifache Kürzelkollision)

Die neuen Ressourcen erzeugen Kürzelkollisionen, die **vor** jeder Einarbeitung normalisiert werden:

| Kollision | Normalform ab jetzt |
|---|---|
| Skalenleiter „S0–S8" ↔ Spezifikationen „S1–S15" | Skalen heißen **SCALE-0 … SCALE-8** (SCALE-0 Unit/Claim/Cell, SCALE-1 Artifact, SCALE-2 Task/Run, SCALE-3 Workflow/Process, SCALE-4 Workspace/Project, SCALE-5 System/Product, SCALE-6 Infrastructure/Platform, SCALE-7 Ecosystem/Federation, SCALE-8 Normic Memory/Evolution) |
| Landkarten-Ebenen „L0–L11" ↔ Produkt-Level „L0–L4" (Domänen-Katalog) | Produkt-Reifegrade heißen **PL0–PL4**; Landkarten-Ebenen bleiben **L0–L11**. Der Domänen-Katalog wird entsprechend nachgezogen (Amendment S1-A3) |
| HBM-„DomainAdapter" ↔ S1-„DomainAdapter" | **Ein** Vertrag: der S1-`DomainAdapter` (11 Punkte) ist die Produktform; die HBM-Felder `(Π,C,V,Can,σ,G,E,Ex)` werden per Mapping-Tabelle (REBASE_S1-S14 §1) auf die 11 Punkte abgebildet — kein zweiter Adapterbegriff |

---

## 2 — PHASE 1: S1–S14 Rebase Impact Map

Für jede Spezifikation: nachzuziehende Abschnitte, neue Begriffe, Präzisierungen, DoD-Erweiterung, neue Gates/Residuen, Driftgefahr. Die **konkreten normativen Nachzieh-Blöcke** stehen in `REBASE_S1-S14.md`; hier die verbindliche Impact-Übersicht.

| Spec | Nachzuziehen | Neue Begriffe | Präzisierung bestehender Aussagen | DoD-Erweiterung | Neue Gates/Residuen | Driftgefahr |
|---|---|---|---|---|---|---|
| **S1** Dokument + Katalog | §S1.8 Adapter; Katalog §K.1/K.2 | ScaleSlot (BlueCube/RedCube/PhaseBlock-Anschluss), MiningProfile, BridgeProfile, ScaleProfile; PL0–PL4 (statt L0–L4) | Adapter = HBM-Adapter (Mapping); Katalog bleibt vollständig **ohne** PL4-Behauptung je Domäne | + AdapterScaleSlots ∧ MiningProfile-Slot definiert | Gate_A (Adapter-Mining-Gate); `adapter_untyped`, `facet_unsourced` | Katalog als „alles produktreif" misslesen → PL-Leiter hält dagegen |
| **S2** Reise | §S2.2/S2.3 Nähte | PhaseBlock-Lesart der Nähte, PhaseLadder auf SCALE-1 | Jede Naht 0–5 = Seam + Replay-Abschnitt; Naht-Übergabe kann als PhaseBlock-Accept protokolliert werden | + Reise als geschlossene PhaseLadder(SCALE-1) lesbar | `seam_without_phaseblock` (info) | Reise bleibt SCALE-1; keine stille Höherskalierung |
| **S3** Cockpit | §S3.2 Flächen | PhaseBlock-Ansicht, Frontier-Ansicht (HDAG/Ratchet + Sync), Ratchet-Status (advance/hold/lock), Blue/Red-Schlussstatus, WrapStability, HBM-Candidate-Board (Pass/Hold/Reject; Score nur Ranking) | Prüf-Fläche wird multi-skalar aufklärbar | + neue Ansichten read-only, Score sichtbar als Ranking mit „kein Gate"-Label | COCK-INV-6: kein UI-Pfad macht Score zum Gate | Score-Anzeige als Entscheidung misslesen (V1) |
| **S4** Wunsch | §S4.1/S4.3 | BoundarySpec, CompletionSpace, MiningQuery; Wunsch-Skalenziel (SCALE-1 default) | Wunsch erzeugt Crystal **und** BoundarySpec + CompletionSpace; optional MiningQuery an L7 | + BoundarySpec/CompletionSpace Pflichtfelder der Wunsch-Normalform | `boundary_unspecified`, `completion_space_open` (sichtbar) | Workbody-Wünsche (SCALE≥2) als produktreif behaupten — verboten, PL-Hinweis |
| **S5** Orchestrierung | §S5.1–S5.5 | PhaseBlock-HyperDAG-Ausführung, Multi-Ratchet-Treiber, HBM-Pipeline-Lauf | Checkpoint = Kandidatenzustand; **PhaseBlock = akzeptierter, irreversibler Abschluss** (Accept-8-Kriterien); HITL-Entscheidung = PhaseBlock-Input mit RD/Evidence | + Orchestrierung kann H, MR-Kaskade, HBM-Phasen 0–9 deterministisch fahren | PhaseRatchetGate, ScaleRatchetGate; `ratchet_lock_without_evidence` | Kandidaten-Commit ohne Evidence (Driftverbot 6) |
| **S6** Inspektion | §S6.1–S6.8 | PhaseBlock-, Ratchet-Lock-, WrapStability-, AreaClass-, Frontier-Sync-, HBM-Candidate-Objekte | Aufklär-Tiefe wird multi-skalar (Ebene 0–3 je SCALE); Score-Anzeige stets als Ranking markiert | + sechs neue Inspektionsobjekte, read-only, wurzel-rückführbar | `frontier_desync` (blocking) | Erklär-Schicht könnte Ratchet-Hold beschönigen — Fakten/Erklärung-Trennung hält |
| **S7** Artefakt | §S7.1 | Provenienz-Erweiterung: BlueprintCandidate-Ref, MiningCapsule-Ref, PhaseBlock-Digest | **Artefakte bleiben SCALE-1**; können aus Mining-/Spiral-/Ratchet-Prozessen entstehen; geschlossene Artefakte sind promotionsfähige Cells (nur via ScaleRatchet, später) | + Herkunftskette um Mining-/PhaseBlock-Glieder erweitert | `provenance_gap_mining` | Artefakt als „schon Workbody" ausgeben — verboten |
| **S8** Bibliothek | §S8.1 | Bestand + Crystals, PhaseBlocks, WorkbodyBlueprints (HBM), DomainNorms, BridgeNorms (Port), Negativ-Cubes je neuer Gate-Klasse | Referenz-/Negativ-Cubes perspektivisch je SCALE (Referenz-/Negativ-Workbodies) | + Bibliothek CI-wacht auch Wrap/Drift/Ratchet-Negativfälle | Negativ-Cubes: `external_drift_detected`, `wrap_support_violation`, … | Blueprint ohne Selbst-Validierung aufnehmen — verboten (S8.2 gilt) |
| **S9** Persistenz | §S9.1/S9.5 | PhaseBlock-Ledger, HBM-Registry, Source/Facet/Blueprint-Provenienz, Replay-Packs, Frontier-/Promotion-Objekte, MEF-Evidence-Bindung | Ledger **präzisiert**: `Ledger = CommitProjection(HyperDAG)` — die kausale Ordnung ist H, der Ledger ihre Linearisierung | + CAS adressiert PhaseBlocks/MEF/Frontier/Capsules | `ledger_hdag_mismatch` (blocking) | Blockchain-Regression (Driftverbot 2) — ausgeschlossen |
| **S10** Abnahme | §S10.2/S10.5 | DoD-Matrix a–f (§5); PlatformFoundationDoD | Zwei Fertig-Stufen → **gestufte DoD-Matrix**; Produkt-DoD(Dokument) bleibt unberührt | + RebaseDoD, SpiralProcessDoD, MiningChassisDoD, S15ReadinessDoD | — | falsche Vollständigkeitsbehauptung — Matrix verhindert sie |
| **S11** Auslieferung | §S11.1/S11.4 | PL-gestufte Feature-Sichtbarkeit | Neue Layer sichtbar führbar, aber **keine** unfertigen Domains/Nexus-Funktionen als produktreif; Update-DoD prüft zusätzlich neue Negativ-Cubes | + Auslieferung zeigt PL je Feature | `feature_maturity_overclaim` | Overclaim im Produkt — Gate + S13 |
| **S12** Doku | §S12.1 | Operator-Lesarten: Phase, Skala, BlueCube/RedCube, Ratchet, Frontier, HBM-Kandidat, Nicht-Abschluss | Erklärungen **ohne mathematische Überladung**; Ethos-Vermittlung erweitert | + Doku-Slots je neuem Objekt | — | Überladung/Overclaim — Claim-Schranke gilt für Doku |
| **S13** Governance | §S13.1/S13.6 | SourceHorizon, CapabilityLock, ExplorationPolicy, ProjectionPacket-Vertrag, HBM-Safety (Safety-by-abstraction), Klon-Budget/TTL | **A7 als PROD-INV**: Score ranked, Gate decides; **A8**: Freiheit innerhalb Boundary; keine unkontrollierte Operator-Klonung; Agent trifft keine globalen Entscheidungen außerhalb Packet | + PROD-INV-9…12 (§ REBASE_S1-S14) | `capability_violation`, `exploration_out_of_horizon`, `unbounded_cloning` (alle blocking) | Agent-Autonomie-Drift — architektonisch gebunden |
| **S14** CoreExtension | §S14.2/S14.3 | Kompatibilitätspflichten Spiral/HBM | Jede CoreExtension erhält **zusätzliche Beweispflichten**: darf WrapProjection, Gate, Replay, Residue, Ratchet-Invariante **nicht** lockern; Strukturpause umgeht S14 nicht | + Extension-Beweispflicht-Zeile „Kinematik-Erhalt" | `extension_breaks_wrap`, `extension_relaxes_ratchet` (Ablehnung) | vertikale Achse als Rebase-Hintertür — verboten |

**Rebase-Gültigkeitsregel für jede Zeile:** `DoD(Si+) = 1 ⟺ DoD(Si) = 1 ∧ FoundationAligned(Si+) = 1 ∧ NoNewSilentResidue(Si+) = 1`.

---

## 3 — PHASE 6: HBM-Neutralisierungsregister (chirurgisch)

Jeder Rohbegriff erhält Typ, Rolle, Bindung und Fehlersprache — **Struktur statt Metapher** (HBM-Axiom 2.1). Keine Metapher wird API.

| Rohbegriff | Normativer Typ | Definition/Bindung | Fehlersprache |
|---|---|---|---|
| **Kosmokrator** | `ExclusionGate` (Reality-Constraint-Evidence-Gate) | `EXC(B) = Reality(B) ∧ Constraint(B) ∧ Topo(B) ∧ Evidence(B)`; boolesch, fail-closed, begründet; Position: Pipeline-Phase 6; nur `EXC=1` darf expandieren | `exclusion_fail(reality\|constraint\|topo\|evidence)` → Hold/Reject, nie Pass |
| **Chronokrator** | `LayerExpansion` (Controlled Expansion Operator) | schichtweise Entfaltung `Ξ(B)=⋃χ_k(B)`, Layer `L_k`; Schichtgültigkeit über Gate-Prädikat je Knoten (Schwellen `τ_k` sind **Ranking-/Zulassungsparameter im RD**, die Schicht-Abnahme bleibt boolesches Gate) | `layer_invalid(k)`, `expansion_budget_exceeded` |
| **Pfauenthron** | `CrystalFinalization` | `FIN(B) = Closed ∧ QSR ∧ Gate ∧ Replay ∧ ResidueVisible`; `FIN=1 ⇒ Crystal(B)` — identisch mit der CCC-Kristallbedingung; keine neue Finalisierungs-Theorie | `finalization_incomplete(…)` |
| **Seraphic Calibration** | `FixpointCalibration` / `PhaseSwitchController` | Triplet `Φ(c)=(ψ,ρ,ω)∈[0,1]³` als **Metriken**; Steuerung `z_{t+1}=Φ_V(Φ_U(z_t))` (explorativ + kontraktiv); Phasenwechsel nur bei protokollierter Stagnation, **RD-geloggt** (HBM-10) | `phase_switch_unlogged`, `calibration_drift` |
| **Heavenly Hosts** | `EphemeralMiningCells` | `Cell=(scope,budget,projection,allowedOps,gate,ttl,trace)`; bearbeitet nur den projizierten Teilraum; Dissolution: nur Trace/Evidence/Kristalle persistieren (HBM-Axiom 5.4, HBM-12) | `cell_scope_leak`, `persistent_ephemeral` |
| **Fallen Seraph** | `BoundedOperatorSpecialization` | Operator-Klon `O'=(O,scope,budget,mutation,expiry,ledgerRef)`; gültig ⟺ Scope ⊆ Elternscope ∧ Budget endlich ∧ Ergebnisse gegatet; `|descendants(O)| ≤ B_O` (RD-Parameter) | `unbounded_cloning`, `clone_scope_escalation` (blocking) |
| **FTCSA** | `DistributedMiningField` | Feldzustand `F(t)=T_topo+ΣT_res+ΣC_ij+δF` — ausschließlich **Metrik-/Koordinationslesart** über EphemeralMiningCells; keine Physik-, Resonanz- oder Wahrnehmungs-Ontologie | `field_metric_overclaim` |
| **Singularität (HBM)** | `DenseBlueprintAttractor` / closure-würdiger Kandidat | strukturell dichter Kandidat: hoher Nutzen/Neuheit **pro** Kosten/Risiko/Baumweite. **A7-konform zerlegt:** `Rank(B) = Score_D(B)` **priorisiert**; die Annahme bleibt `Gate(B)=Pass ∧ Closed(B) ∧ Replay(B)` — der Score-Schwellwert `θ_D` ist Vorauswahl fürs Weiterrechnen, **nie** Abnahme | `score_as_gate_attempt` (Ablehnung, V1) |
| **Hypercube Blueprint** | `BlueprintCandidate` → zertifiziert: `WorkbodyBlueprint` (PHC-kompatibel) | `B=(X,K,N,H,F,Π,Γ,E,R,Θ)`; Hypercube-Form `C=∏A_i` oder HDAG-augmentiert; nach `CrystalFinalization` als PHC-Profil-kompatibles Paket in Bibliothek/Registry | `blueprint_untyped`, `hdag_cycle` |
| **HBMFile** | `NexusMiningCapsule`-**Vorform** | kanonisierte Projektions-/Mining-Instanz mit normativen Sektionen (header…ledger/monoliths); Kanonisierung K1–K6 (deterministisch sortiert, content-addressed, keine freien Floats, seeded randomness, gleiche Semantik ⇒ gleiche Bytes, Extensions ändern Core-Hashes nicht); Vollform erst mit Nexus (R-1) | `capsule_hash_drift`, `unseeded_randomness` |

**Übernommen als verbindlich:** HBM-Axiome 5.1–5.7 (typisierte Projektion; explizite Kopplung — implizite Abhängigkeit = Residuum; fail-closed Mining; nur Kristalle persistieren; Baumweitenbewusstsein; kalibrierte Exploration, unseeded randomness verboten; Nicht-Vermischung — Multi-Domain nur via Adapter-Komposition), die Operatoralgebra (Extract, Project, Solve, Gate, Coagula, Triangulate, Collect, Distribute, Expand, Migrate, Clone, Dissolve, Certify) mit Gate-Dominanz `Materialize(B) ⇒ Gate(B)=Pass`, der Abnahmekatalog **HBM-01…HBM-20**, und der **ProjectionPacket-Agentenvertrag** (kein Agent trifft globale Architekturentscheidungen außerhalb seines Packets).

**Ausgeschlossen per Safety-by-abstraction (HBM 13.2):** Tarnung, Umgehung, offensive Operationen, nicht-tracebare Netze, unkontrollierte Selbstvermehrung, autonome Schadenshandlungen — nur formale Strukturmuster wurden extrahiert.

---

## 4 — PHASE 7: Unified Normal Form vNext

### 4.1 KorpusRebaseState

```
KorpusRebaseState =
(
  FoundationInvariants,      // F1–F6 + INV-1..14 + V1–V10 (irreduzibel)
  DomainAdapters,            // S1-Vertrag, 11 Punkte, HBM-Feld-Mapping
  SourceHorizon,             // Explorationsgrenze (Typ/Port; Vollform mit Nexus)
  HBMMiningChassis,          // Pipeline 0–9, neutralisiert, fail-closed
  BlueCubeRedCube,           // lokaler Phasenraum / Phasenmatrix je Skala
  PhaseBlockHyperDAG,        // H = (V, E_dep, E_seam, E_phase, E_scale, E_commit)
  SpiralProcess,             // (Z,A,Θ,Λ,E,W,µ,G,R,L) mit WrapProjection
  MultiRatchetCascade,       // Cell/Phase/Ring/Scale/Nexus-Ratchets
  CrystalConsensus,          // Accept ∧ ParentClosure ∧ FrontierCompatible ∧ LedgerAppendable
  WorkbodyProjection,        // LOOM/PHC-Distribute in skalierte Arbeitskörper
  NexusBridgeNorms,          // Port-Typen (spätere Foundation, R-1)
  EvidenceReplayLedger,      // MEF-Evidence, Replay-Packs, Ledger = CommitProjection(H)
  ProductDoD                 // gestufte DoD-Matrix (§5)
)
```

### 4.2 Pflichtformeln (verbindlich, korpusweit)

1. **Rebase:** `K+ = Normalize_F(Classify(R), K)` mit Zulässigkeit `Inv(F) ⊆ Inv(K+)` und: kein Element lockert ein bestehendes Gate-, Residuen-, Boundary-, Replay- oder Claim-Regime.
2. **Blue/Red:** `Close(Blue_{s,p}) = 1 ⇒ PhaseBlock_{s,p}` und `Close(Red_s) = 1 ⇒ Promote(s → s+1)`; Kurzform: *BlueCube schließt Phase, RedCube schließt Skala, geschlossene Skala wird Zelle der nächsten Skala.* Dabei `Close(Blue) ⟺ Typed ∧ BoundaryValid ∧ Gate=Pass ∧ Evidence=1 ∧ Replay=1 ∧ WrapStable` und `Close(Red) ⟺ ∀p: Close(Blue_{s,p}) ∧ SeamsValid ∧ Replay ∧ Resid sichtbar`.
3. **Spiralprozess:** `z_{k+1} = W_s(E_{λ,α}(z_k))` — Expansion intrinsisch, Wicklung extrinsisch; WrapProjection-Pflichten: Support-Boundedness `Supp(W_s(z)) ⊆ B_s`, Seam-Preservation, Residue-Preservation, Replay-Preservation.
4. **AreaClass:** `[E^k(X)]_{µ_s} = [X]_{µ_s}` — Flächengleichheit ist Support-/Budgetklassen-Invarianz, keine naive Geometrie.
5. **Multi-Ratchet:** `Lock(R_{n+1}) = 1 ⇒ ∀R_i ≺ R_{n+1}: Lock(R_i)=1 ∨ Resid(R_i) sichtbar` (als Counter-Horizon geführt). Ratchet-Schritt: `Step(r)=1 ⟺ G(r)=Pass ∧ E(r)=1 ∧ ρ(r) sichtbar`; intrinsische Phase rotiert frei (`θ_{k+1}=θ_k+δ_k mod 2π`), der Commitzähler steigt **nur** unter Gate/Evidence.
6. **HBM-Kette:** `Korpus/Domäne/Wunsch → Facets → Adapter → Cube/HDAG → Skeleton/JunctionTree → Candidate → Gate → Crystal → Replay` — mit Spiral-Lesart `Candidate_{k+1} = W(E(Candidate_k))`, nur innerhalb SourceHorizon, DomainAdapter, Gate, Evidence, Replay (A8).
7. **S15-Readiness:** `S15Ready = 1 ⟺ RebaseDoD ∧ SpiralProcessDoD ∧ MiningChassisDoD ∧ S1-S14ImpactResolved`.

**Flankierend verbindlich:** `Proof-of-Closure: PoC(x)=1 ⟺ Gate=Pass ∧ Evidence=1 ∧ Resid sichtbar ∧ Replay=1 ∧ Reanalyze(x) ∼ Crystal(x)`; `CrystalConsensus(b)=1 ⟺ Accept(b) ∧ ParentClosure(b) ∧ FrontierCompatible(b) ∧ LedgerAppendable(b)`; Drift-Zerlegung `d = d∥ + d⊥` mit `Π_phase(d)=d∥` zulässig und `d⊥ ≠ 0 ⇒ Resid_drift(d⊥) sichtbar`; Orientierungsprofil `Close4π(x) ⇒ Close2π(x+) ∧ Close2π(x−) ∧ ι(x+) ∼ x−` (dyadische Closure-Ordnung, keine Physikbehauptung); Frontier-Sync-Pflicht `Sync(Frontier_HDAG, Frontier_Ratchet)=1`; Completion Pressure sinkt **nur** durch `Accept(PhaseBlock)=1`.

---

## 5 — Updated DoD Matrix (gestuft, S10-Erweiterung)

| # | DoD | Definition | Status auf Papier |
|---|---|---|---|
| a | **ProductDoD(Document)** | S10 §S10.2 unverändert; Rebase-Regel garantiert Nicht-Bruch | **1** (geschlossen; Rebase bricht nichts) |
| b | **DomainCatalogDoD** | Katalog-DoD (§K.7) ∧ PL-Umbenennung ∧ Slots MiningProfile/BridgeProfile/ScaleProfile **definiert** (Befüllung je Domäne = sichtbarer Reifepfad) | **1** strukturell; Slot-Befüllung als PL-Residuum geführt |
| c | **RebaseDoD** | die 12 Strukturpause-Kriterien: F fixiert; Inv(F)⊆Inv(K+); PSP ↦ ExecutionKernel; AG ↦ PhaseBlockHDAG; MEF als Evidence-Schicht gebunden; Spiralkopplung ↦ Multi-Ratchet/HDAG/Commit-Projektion; Blue/Red/Skalenleiter definiert; PhaseBlock/HyperDAG/Frontier/CrystalConsensus definiert; Kaskade Drift-intern/extern; S1–S14-Nachziehpflichten erteilt; keine S15 als Anbau; Residuen sichtbar | **1** (dieses Dokument + REBASE_S1-S14) |
| d | **SpiralProcessDoD** | Spiralprozess formal ∧ Blue/Red/Scale-Relation ∧ Expand-und-Wrap ∧ AreaClass ∧ Innen-Drift-erlaubt/Außen-verboten ∧ Multi-Ratchet ∧ Apertur-/Binnenpupillen-Anschluss ∧ kein Golden-Winkel-Dogma ∧ 8 Gates + 12 Residuen ∧ Nachziehauftrag | **1** (übernommen + eingebunden) |
| e | **MiningChassisDoD** | Neutralisierung vollständig (§3) ∧ Pipeline 0–9 ∧ Axiome 5.1–5.7 ∧ Operatoralgebra ∧ HBM-01…20 als Abnahmekatalog ∧ Score/Gate-Trennung (A7) ∧ ProjectionPacket-Vertrag ∧ Safety-by-abstraction | **1** (dieses Dokument + S15 §8) |
| f | **S15ReadinessDoD** | Formel 7 (§4.2) | **1** ⇒ S15 wird ausgearbeitet |

Zusätzlich: **PlatformFoundationDoD** = c ∧ d ∧ e (das nachgezogene Fundament unter der bestehenden Plattform-DoD aus S14 §S14.11).

---

## 6 — PHASE 8: S15 Readiness Report

**Geprüft:**

- **Nachgezogene Begriffe aus S1–S14:** vollständige Impact Map (§2) + normative Amendment-Blöcke (REBASE_S1-S14). ✓
- **Normativ aus Strukturpause:** F1–F6, Blue/Red/Skalenleiter (SCALE-0…8), PhaseBlock (10-Tupel, Accept-8-Kriterien), HyperDAG (6 Kantentypen), `Ledger = CommitProjection(H)`, Frontiers + Sync, Ratchet-Einheit, Close2π/Close4π, Multi-Ratchet, Completion Pressure, CrystalConsensus, Proof-of-Closure, Rebase-Regel, Neutralisierungsregister, 8 Driftverbote. ✓
- **Normativ aus Spiralprozessschicht:** Spiralprozess-Tupel, Spiraladresse, WrapProjection (4 Pflichten), AreaClass, Treppen-/Wickelgesetz, Drift-Axiom, PhaseRatchet-Struktur (advance/hold/lock), ScalePromotion-Bedingungen, Ratchet-Typen-Tabelle, Kaskadenregel, Binnenpupille/FBC/ActiveSector, DispersionProfile (4 Profile, Profil-statt-Dogma), Discrete-to-Continuous Lift, 8 Pflichtgates, 12 Residuen, 10 Axiome. ✓
- **Mining-Chassis:** HBM-Haupttupel, Pipeline 0–9 (neutralisiert), Axiome, Operatoralgebra, Kalibration, EphemeralMiningCells, BoundedOperatorSpecialization, Capsule-Vorform, HBM-01…20. ✓
- **Nur spätere Foundation:** Nexus, SourceHorizon (Vollform), BridgeNorm, NexusClass, Normic Memory (SCALE-8), hypertoroidale Wicklung, Quiescence — als **Typen/Ports** in S15 §12 angeschlossen, PL0/PL1, sichtbar. ✓
- **Offene Residuen:** Register §7 — alle sichtbar. ✓

**Ergebnis:** `S15Ready = RebaseDoD ∧ SpiralProcessDoD ∧ MiningChassisDoD ∧ S1-S14ImpactResolved = 1` auf Spezifikationsebene. S15 wird als **homogenes Produkt der Rebase** ausgearbeitet (nicht als Anbau) — siehe `S15_OPERATIONAL_MULTICUBE_WORKBENCH.md`.

---

## 7 — Residue Register (vollständig, sichtbar)

| # | Residuum | Klasse | Führung |
|---|---|---|---|
| R-1 | **Nexus/SourceHorizon/BridgeNorm/NexusClass/Normic Memory** — keine Quell-Spezifikation im Korpus | spätere Foundation | Nur Typen/Ports (L9); SourceHorizon als Governance-Typ in S13 aktiv; Vollform bei Nexus-Ressource; **keine** Vollständigkeitsbehauptung |
| R-2 | FTCSA-Feldformalismus jenseits der Metrik-/Koordinationsrolle | neutralisiert/limitiert | `DistributedMiningField` nur als Metrik; alles Weitere nicht-normativ |
| R-3 | Blockchain/Token/ZK-ML-Pflichten aus Altdokumenten | Altlast | **verworfen** (Driftverbot 2); nur Commit-Projektions-Strukturform übernommen |
| R-4 | 2π/4π/720°-Lesarten | begrenzt | ausschließlich formales Orientierungsprofil (dyadische Closure-Ordnung); keine Physik-/Numerologie-Behauptung (F5, V10) |
| R-5 | operative Klonungs-/Schwarm-/Tarn-Details | ausgeschlossen | Safety-by-abstraction; nur Budget/TTL/Gate-Strukturmuster |
| R-6 | MEF-Feinformat (Evidence-Block-Encoding) | offen | als Evidence-/Proof-Payload-Typ gebunden; Byte-Format beim Bau zu fixieren |
| R-7 | DispersionProfile-Default | offen (Empfehlung) | **DyadicProfile** empfohlen (kohärent mit Zellsubstrat Z12/24-Fenstern); zu bestätigen; jedes Profil RD-gebunden |
| R-8 | HBM-Score-Gewichte `α_i, β_j`, Schwellen `θ_D, τ_k` | offen | RD-Parameter, replayrelevant; Kalibrierung beim Bau; **nie** Gate |
| R-9 | **Physische Einarbeitung der Amendments** in die 17 Einzeldateien | Redaktionsschritt | Amendment-Modell ist normativ (append-only, S13.7-konform); Einarbeitung = benannter Folgeschritt, kein stilles Loch |
| R-10 | SCALE≥2-Produktreife (Task/Workflow/Workspace…) | PL-geführt | S15 spezifiziert die Struktur; Produktreife je SCALE ist sichtbarer Reifepfad, **nicht** behauptet |
| R-11 | MiningProfile/BridgeProfile/ScaleProfile-Befüllung je der 213 Domänen | PL-geführt | Slots definiert (S1-Amendment); Befüllung folgt Priorisierungs-Weiche §K.6 |
| R-12 | Ratchet-/Frontier-/Candidate-UI-Feindesign | offen | Verträge fixiert (S3/S6-Amendments); Pixel später |
| R-13 | Operator-Klonung im Produktbetrieb | fail-closed default | spezifiziert, aber **deaktiviert** bis CapabilityLock-Implementierung (S13); Aktivierung nur mit Budget/TTL/Ledger |

---

## 8 — Agent Handoff Summary

**Für den Coding-Agenten (ergänzt Bauverfassung Teil 9, ersetzt sie nicht):**

1. **Lies zuerst:** BAUVERFASSUNG (unverändert führend) → dieses Dokument → SYSTEMLANDKARTE_vNEXT → REBASE_S1-S14 → S15. Die Amendments sind normativ; bei scheinbarem Konflikt gilt: Fundament > Rebase-Amendment > ursprüngliche S-Spec-Formulierung; Konflikte sind als Residuum zu melden, nie still zu lösen.
2. **Neue Kern-Crates** (azyklisch, unter den bestehenden `cce-*`): `cce-kernel` (PSPcore: Σ/Ω/RD/Trace/Evidence/Manifest — Execution-Grammatik), `cce-phaseblock` (PhaseBlock, HyperDAG, Frontiers, CrystalConsensus, Ledger=CommitProjection), `cce-spiral` (Expansion, WrapProjection, AreaClass, Ratchets, DispersionProfile, 8 Gates), `cce-hbm` (Pipeline 0–9, neutralisierte Operatoren, Cells, Capsule-Vorform), Erweiterung `cce-runner` um HyperDAG-/Kaskaden-/Pipeline-Ausführung.
3. **Phasen-Gates zusätzlich:** Blue/Red-Schließungslogik grün; PhaseBlock-Accept (8 Kriterien) grün; Frontier-Sync grün; die 8 Spiral-Gates + 12 Spiral-Residuen als Tests; HBM-01…20 als Abnahmekatalog; Negativ-Cubes je neuem Residuum (Regressionswächter erweitert).
4. **Unantastbar:** INV-1..14, V1–V10, F1–F6; Score nie Gate; kein unbounded cloning; kein Kandidaten-Commit ohne Evidence; keine Skalenpromotion ohne RedCube-Schluss; kein Blockchain-Import; alle Residuen sichtbar.
5. **Kerntest bleibt Kerntest:** `Reanalyze(Materialize(LOOM(Project(PHC(Crystal))))) ≃ Crystal` — jetzt zusätzlich als `PoC`-Instanz auf SCALE-1 lesbar; die neue Geometrie darf ihn nie brechen (Rebase-Regel).

**Schlussformel der Rebase:** *Strukturpause spannt das Fundament. Spiralprozess erklärt die Kinematik. HBM findet closure-würdige Blueprints. S1–S14 werden homogen nachgezogen. S15 entsteht erst danach als skalenadaptive Operational-Multicube-Workbench.*

*Ende der Rebase-Konsolidierung.*
