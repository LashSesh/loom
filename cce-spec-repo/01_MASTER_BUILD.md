# 01 — MASTER-BAUORDNUNG (konsolidierte One-Shot-Implementierung, Phasen G0–G12)

**Die eine Bauordnung.** Sie verschränkt die vier Teil-Bauordnungen des Korpus — Bauverfassung Teil 9 (Phasen A–J), Rebase-Handoff (REBASE_KONSOLIDIERUNG §8), CSA-Handoff (REBASE_CSA_AMENDMENTS §7) und LOOM-Container-Handoff (LOOM Teil 12, B1–B12) — zu **einer** strikt sequenziellen Phasenfolge. Jede Phase hat Eingangsbedingung, Arbeitsinhalt mit Spezifikationsquellen, ein **Ausgangs-Gate** (Tests, die grün sein MÜSSEN, bevor die nächste Phase beginnt) und Verbote. Keine Phase wird übersprungen, keine parallel begonnen, kein Gate „vorläufig" passiert.

**Durchgängige Regeln:** Rust stable (Version in G0 pinnen); jedes Crate mit Unit-Tests; jede Invariante (INV-1..14, F1–F6) als Property-/Unit-Test; jede Prohibition (V1–V10, PROD-INV-9..16) als **Negativ-Test** (muss abgewiesen werden); CI ab G0, Regressionswächter wächst mit jeder Phase; jeder Phasenabschluss erzeugt einen **Baubericht** (Format: 04_AGENT_AUFTRAG §3). Determinismus überall: seeded randomness, feste Tie-Breaks, keine Wall-Clock in signaturrelevanten Pfaden.

---

## G0 — Repo-Setup, Integrität, Toolchain
**Quellen:** 00_START_HIER §5, 03_INVENTAR, Bauverfassung Teil 6/9-A.
**Arbeit:** Integritätscheck aller 24 Specs (SHA-256 gegen 03_INVENTAR — bei Abweichung: STOPP, Residuum melden). Monorepo-Skeleton `cce/` gemäß Zielstruktur; Workspace-`Cargo.toml`; Toolchain pinnen (`rust-toolchain.toml`); CI-Skeleton (fmt, clippy -D warnings, test); leeres `conformance/`-Harness; `ci/check_acyclic` für Crate-Graph (INV-11).
**Ausgangs-Gate:** Integritätscheck grün · Workspace baut leer · CI läuft · Acyclic-Check aktiv.
**Verbote:** keine Spec-Datei anfassen; keine Abkürzung „erstmal alles in ein Crate".

## G1 — Kern-Substrat (Motor-Objekte, Kanonisierung, Gates)
**Quellen:** Bauverfassung Teile 1–4, 7 (Gates G1–G7), 8 (INV/V); Phasen A–C.
**Arbeit:** `cce-core` (kanonische Objekte: Cell/Seam/Crystal, Wunsch-Normalform W inkl. BoundarySpec/CompletionSpace/ScaleTarget aus S4-A1; Residuen-Typen; GateReport-Typ boolesch+begründet), `cce-lattice` (Constraints, Chordalisierung, PEO, Junction Tree, Separatoren), `cce-ccc` (Closure-Kalkül, ≃/Kanonisierungsklassen, Can∘Can=Can), `cce-crystal` (Crystal-Aufbau, Zwei-Digest-Grundlage: Inhaltsklassen-Digest). Gate-Gerüst G1–G7 fail-closed.
**Ausgangs-Gate:** INV-1..3, INV-8/9-Grundtests grün · G1–G7 als Typen mit Fail-closed-Default · Negativ-Tests: Score-Feld im GateReport wird abgewiesen (V1) · `check_adapter_parity`-Gerüst vorhanden.
**Verbote:** kein Float in signaturrelevanten Strukturen; keine „später begründen"-Gates.

## G2 — Execution-Kernel & PhaseBlock-HyperDAG
**Quellen:** REBASE_KONSOLIDIERUNG §1 (PSPcore, Klasse B), §4.2 Formeln 2/5; Strukturpause-Inhalte darin.
**Arbeit:** `cce-kernel` (PSPcore = Σ, Ω, RD, Trace, Evidence, Manifest; deterministisches Scheduling; MEF-Payload-Typ mit deklariertem Byte-Encoding — R-6 hier schließen und im Baubericht dokumentieren), `cce-phaseblock` (PhaseBlock-10-Tupel; Accept-8-Kriterien; HyperDAG H mit 6 Kantentypen; Frontiers + Sync; CrystalConsensus; `Ledger = CommitProjection(H)` + `verify_hdag_projection`).
**Ausgangs-Gate:** Accept-Kriterien einzeln getestet (jede fehlende Bedingung ⇒ Hold, nie Commit) · Frontier-Desync wird erkannt (blocking) · Ledger-Projektion round-trip-getestet · Negativ: Kandidaten-Commit ohne Evidence abgewiesen (Driftverbot 6).

## G3 — Transport, Weben, Materialisieren + Motor-Kerntest
**Quellen:** Bauverfassung Teile 3–5, Phasen D–E; S7 (Zwei-Digest-Modell); S1 (Dokument-Adapter als erste Instanz).
**Arbeit:** `cce-phc` (Pakete, Domänen-Profile), `cce-loom` (Weave/Distribute, radiale Spindel, Nullanker-Markierung), `cce-materialize` (Artefakt-Erzeugung, Byte- vs. Inhaltsklassen-Digest), `cce-observe`/`cce-merkaba` gemäß Teil 6. **DomainAdapter „Dokument"** vollständig (S1.8, 11 Punkte) mit Referenz-Cube „Drei-Risiken-Memo".
**Ausgangs-Gate:** **Motor-Kerntest grün:** `Reanalyze(Materialize(LOOM(Project(PHC(Crystal))))) ≃ Crystal` für den Referenz-Cube · alle 7 Dokument-Gates grün am Referenz-Cube, rot an je einem Negativ-Cube · Zwei-Digest-Trennung nachgewiesen (Format-Kosmetik ändert Byte-, nie Klassen-Digest).

## G4 — Spiral-Kinematik
**Quellen:** REBASE_KONSOLIDIERUNG §1/§4.2 (Formeln 3/4/5), S15 §6–7.
**Arbeit:** `cce-spiral`: Expansionsoperator, WrapProjection (4 Pflichten), AreaClass, Spiraladresse, DispersionProfile (Dyadic default per R-7-Empfehlung — im RD deklariert, austauschbar), Ratchet-Typen Cell/Phase/Ring/Scale (+Nexus als Typ-Stub), Kaskadenregel, die 8 Spiral-Gates.
**Ausgangs-Gate:** alle 8 Gates implementiert+getestet · alle 12 Spiral-Residuen erzeugbar+sichtbar · Negativ-Zeugen: `external_drift_detected`, `wrap_support_violation`, `ratchet_lock_without_evidence`, `fibonacci_dogma_import` (undeklariertes Profil wird abgewiesen) rot · Kaskadentest: höheres Lock ohne untere Locks/Residuen unmöglich.

## G5 — Runner & Orchestrierung
**Quellen:** Bauverfassung Phase F; S5 + S5-A (Checkpoint vs. PhaseBlock, HITL als PhaseBlock-Input); S2+S2-A (Reise als PhaseLadder SCALE-1).
**Arbeit:** `cce-runner`: deterministische Ausführung (a) HyperDAG topologisch, (b) Multi-Ratchet-Kaskade, (c) generischer Pipeline-Lauf (RD-gebunden); Checkpoints (Kandidaten) vs. PhaseBlocks (Accepts); Pause/Fortsetzen klassenerhaltend; HITL-Entscheidungen als PhaseBlock-Inputs {gate, entscheidung, operator, RD-Ref, Evidence-Ref}; RunDescriptor vollständig (crystal_digest, decisions, params, seed).
**Ausgangs-Gate:** Replay-Identität: gleicher RD + Inputs ⇒ gleiche Klasse (INV-10) · Pause/Resume ändert Klasse nicht · harte Gates pausieren nie (Test) · Reise-PhaseLadder p₀…p₅ als Red(SCALE-1) schließbar.

## G6 — Persistenz & Bibliothek (der Wächter wird scharf)
**Quellen:** S9 + S9-A (CAS+Refs, Adressierbarkeit, Cursor-als-Ref), S8 + S8-A (Asset-Typen, Selbst-Validierung, CI-Bindung).
**Arbeit:** `cce-store` (unveränderlicher CAS + veränderliche Refs; adressiert Crystals, PhaseBlocks, MEF, Frontiers, Replay-Packs, ResidueReports; `verify_ledger` + `verify_hdag_projection`), `cce-library` (Referenz-/Negativ-Cubes der Dokument-Domäne; Eintritts-Selbstvalidierung; Registry mit Provenienz). **CI-Regressionswächter aktiv:** ab jetzt prüft jeder Commit ALLE bisherigen Zeugen.
**Ausgangs-Gate:** CAS-Round-trip aller Objekttypen · Ref-Konflikt sichtbar+operator-auflösbar (Sync-Vorbereitung) · Wächter blockiert nachweislich einen absichtlich eingeschleusten Bruch (Testfall) · Dokument-Zeugen: alle Referenzen grün, alle Negativen rot.

## G7 — HBM Mining-Chassis
**Quellen:** REBASE_KONSOLIDIERUNG §3 (Neutralisierung, verbindliche Übernahmen), S15 §8; Abnahmekatalog HBM-01…20.
**Arbeit:** `cce-hbm`: Pipeline-Phasen 0–9 (Ingest→Facets→Gate_A-Adapter→Cube/HDAG→Skeletonizer/JT→Kandidaten C1–C6→ExclusionGate→LayerExpansion→CrystalFinalization→Registry/Replay); Operatoralgebra Extract…Certify; Score als Ranking (RD-Gewichte), θ_D nur Vorauswahl; FixpointCalibration mit geloggten PhaseSwitches; EphemeralMiningCells (scope/budget/ttl/trace, Dissolution); BoundedOperatorSpecialization **implementiert, aber fail-closed deaktiviert** (R-13 — Aktivierungspfad nur via CapabilityLock, Test: Aktivierungsversuch ohne Lock wird abgewiesen).
**Ausgangs-Gate:** HBM-01…HBM-20 als Testkatalog grün · Gate-Dominanz: Materialize ohne Gate=Pass unmöglich · Negativ: `score_as_gate_attempt`, `unbounded_cloning` rot · End-to-End: kleiner Korpus ⇒ zertifizierter Blueprint-Kristall, Replay klassenidentisch (HBM-20).

## G8 — Source-Akquisition (CSA)
**Quellen:** CSA_NEXUS_AKQUISITION vollständig; REBASE_CSA_AMENDMENTS (PROD-INV-13..16, Locks, Matrix-Stufe g).
**Arbeit:** `nexus/`-Workspace gemäß CSA §13 (nexus-core … nexus-cli); SourceAdapter-Portvertrag (8 Methoden) + `check_source_adapter_parity`; Adapter-Reihenfolge: **LocalCorpusAdapter → Wikimedia-OfficialAPIAdapter → Git-RepositoryAdapter** (deckt die 5 Reference-Cubes); die 15 Pflichtgates mit Vier-Wege-Verdikt; 18 Residuen; FetchPlan/Caching/Differenzabruf; Qualitätssignatur (ψ,ρ,ω) mit Achsen-Mindestwert-Gate; HBMImport- und PHCProjection-Brücken. **NoFetchBeforePolicyGate architektonisch:** der Netzpfad existiert nur hinter der Gate-Kette; `disallowed_actions` hart verdrahtet.
**Ausgangs-Gate:** CSA-Zeugen: 5 Referenzen grün, 8 Negative rot (PolicyBlockedFetch beweist: kein Socket vor PolicyGate) · PROD-INV-13..16 als Negativ-Tests · Replay: fixer Snapshot ⇒ gleiche IDs/Hashes/Ledgerpfade · Wächter um CSA-Zeugen erweitert.
**Verbote:** kein generic_html-Adapter vor Abschluss aller anderen; StaticWebAdapter nur mit Scope/robots/Terms/Rate/Parserregel und eigenem Aktivierungs-Lock.

## G9 — `.loom`-Containerformat
**Quellen:** LOOM_CONTAINER_STANDARD_V1 vollständig; Reihenfolge = dessen Teil 12 (B1–B12), hier als Unterphasen.
**Arbeit:** `loom/`-Workspace: loom-canon → loom-format → loom-codec (pack/seal, journal & canonical) → loom-verify L0–L3 → SEGTAB/Merkle/core_root (eintragslose Tabelle!) → loom-mount + loom-viewer (L0–L2 **ohne** Motor-Abhängigkeit — Reader-Prinzip als Build-Nachweis: eigenes, motorfreies Build-Target) → R2–R7 über Motor-Ports → loom-runner (CapabilityLocks) → loom-replay → loom-export (Attribution-Transport) → loom-migrate → Conformance C0–C5. Saat-Bibliothek: `library/seed/` wird als `.loom`-Dateien ausgeprägt (R7-Full-Workbody = Drei-Risiken-Memo-Workbody).
**Ausgangs-Gate:** Golden Files: encode∘decode byte-identisch (canonical-stored) · 7 Referenzen valid, 14 Negative reject/quarantine am richtigen Prüfpunkt · C0–C5 grün · Viewer-Target baut ohne cce-* -Abhängigkeit · Öffnen ist nachweislich seiteneffektfrei (Test: kein FS-Write, kein Netz bei open/inspect/verify).

## G10 — Cockpit (natives Desktop, KI-Kanzel-Ports)
**Quellen:** S3 + S3-A1..A6; S4 + S4-A (Wunsch-Fläche inkl. MiningQuery-Pfad); S6 + S6-A (Inspektionsobjekte).
**Arbeit:** `cockpit/`: cockpit-core (deterministische Zustandsmaschine, EnginePort, PersistenceAdapter) + egui-App; vier Flächen (Wunsch/Lauf/Prüf/Artefakt) + die sechs neuen Ansichten (PhaseBlock, Frontier+Sync, Ratchet, Blue/Red+WrapStability, HBM-Candidate-Board mit „ordnet, entscheidet nicht"-Label, CSA-Quellen-/Residuen-Sicht); KI-Kanzel als **Port** (formen/erklären; ohne konfigurierten Anbieter lauffähig-degradiert; niemals Schreibpfad auf Urteile/Residuen/Ledger); Bestätigungsgrenze für materielle Aktionen.
**Ausgangs-Gate:** COCK-INV-1..6 als Tests (insb.: kein UI-Pfad macht Score zum Gate; kein „Trotzdem durchlassen") · Cockpit startet nativ auf Ziel-OS · alle Anzeigen wurzel-rückführbar auf Motor-Artefakte (Stichproben-Test) · Kanzel-Aus-Modus voll funktionsfähig.

## G11 — Produktreise, Doku, Feinschluss
**Quellen:** S2 (6 Nähte), S12 + S12-A (Operator-Lesarten), S7 (Ausgabe/Re-Import), S6 (Drill-down), S13 (Governance im Betrieb).
**Arbeit:** End-to-End-Verkabelung der Reise; `docs/` Operator-Doku aus S12-Slots (inkl. der vier Klartext-Lesarten, Claim-Schranke); Artefakt-Ausgabe mit Zertifikat + Re-Import-Erkennung; Fehl-Reisen enden lesbar (Hold-Zustände).
**Ausgangs-Gate:** **Produkt-Kerntest grün:** Drei-Risiken-Memo-Wunsch → Bestätigung → Lauf → Prüfung „geschlossen (∅)" → Artefakt-Export → Re-Import klassenidentisch — vollständig im Cockpit, ohne Konsole · Doku=Verhalten-Stichproben.

## G12 — Auslieferung & Gesamtabnahme
**Quellen:** S11 + S11-A (PL-Kennzeichnung, Update-Kanal-Wächter), 02_MASTER_DOD.
**Arbeit:** Installierbare Pakete je Ziel-OS; Secrets im OS-Schlüsselbund (Kanzel/Connector); Offline-Nachweis des Kerns; Saat-Bibliothek (.loom) im Paket; `feature_maturity_overclaim`-Prüfung (jede Funktion trägt PL); Versionierung + Update-DoD-Pfad; **Abschlussbericht**: DoD_100-Checkliste Punkt für Punkt + End-Residuenregister (Stand aller R-/CSA-R-/LC-R-/S15-R-Nummern).
**Ausgangs-Gate:** `02_MASTER_DOD.md` vollständig grün — sonst ist der Bau nicht abgeschlossen.

---

## Meilenstein-Übersicht

| Phase | Meilenstein | Kern-Nachweis |
|---|---|---|
| G3 | **Motor lebt** | Motor-Kerntest ≃ |
| G6 | **Wächter scharf** | eingeschleuster Bruch wird geblockt |
| G8 | **Außenwelt closure-fähig** | kein Socket vor PolicyGate |
| G9 | **Arbeitskörper hat einen Körper** | .loom C0–C5 |
| G11 | **Produkt lebt** | Produkt-Kerntest über 6 Nähte |
| G12 | **100%** | MASTER_DOD grün + Endbericht |

*Weiter mit `02_MASTER_DOD.md`.*
