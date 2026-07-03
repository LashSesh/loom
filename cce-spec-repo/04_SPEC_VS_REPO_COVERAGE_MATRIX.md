# 04 — SPEC vs. REPO COVERAGE MATRIX (jede Schicht: Soll → Ist-Ort → Status)

**Status:** `built_verified` (Code gesehen ∨ strukturell zwingend) · `built_reported` (Report+konsistente Verdrahtung; Laufzeit WO-1) · `partial` (gebaut mit benannter Lücke) · `port_only` (Typ/Anschluss, bewusst nicht gebaut) · `spec_only` (nur Papier — per DoD §3 so gewollt).

| Ebene / Spezifikation | Ist-Ort im Repo | Status | Lücke/Notiz |
|---|---|---|---|
| L0–L1 Fundament F1–F6, Closure-Kern (BV T0/7/8) | cce-core (value/canonical/closure/residue/gate/ledger/replay/bcik/machine/qsna/pathinv/hf_import) | built_verified | Katalognummern rekonstruiert (R-Agent-2) |
| L1 CL-Substrat K1–K18 | cce-lattice + tests/k_catalog.rs | built_reported | — |
| L1 CCC C1–C14 | cce-ccc + tests/c_catalog.rs | built_reported | — |
| L2 PHC (V0–V9, Projektion) | cce-phc (loader, projection_calc) | built_verified (Loader-Aufruf im Kerntest gesehen) | — |
| L2 LOOM-Weave (Spindel, Z0, L1–L10) | cce-loom (radial_spindle NULLPOINT_TRAVERSAL=forbidden gesehen) | built_verified | — |
| L2 Observe/TAT/QLOGIC/DZ/PIO, Merkaba | cce-observe (reanalyze::observe im Kerntest), cce-merkaba | built_verified/reported | — |
| L3 PSPcore + MEF-1 | cce-kernel (pspcore, mef, scheduler, assembly) | built_reported | **R-6 GESCHLOSSEN** (MEF-1-Encoding) |
| L4 PhaseBlock/HyperDAG/Frontier/Consensus/Ledger=CommitProjection | cce-phaseblock (accept im Inference-Zeugen benutzt — gesehen) | built_verified | 5 Kantentypen (R-Agent-4, normativ korrekt) |
| L5/L6 Blue/Red, Skalenleiter, Spiral/Wrap/Ratchets (8G/12R) | cce-spiral + spiral_catalog | built_reported | Dyadic-Default RD-deklariert (R-7) |
| L7 HBM 0–9, HBM-01..20, Klonung deaktiviert | cce-hbm + hbm_catalog; cce-core::capability | built_reported | R-13 offen wie spezifiziert |
| L8 Dokument-Adapter D01 (11 Punkte, 7 Gates, Referenz-/Negativ-Cubes) | cce-materialize::{adapter,document} — im Kerntest-Quelltext benutzt | built_verified | Export .md; .docx = S1.10-R1 offen |
| L8 Katalog 213/16, PL-Slots | cce-materialize::catalog(+_data) | built_reported | 212 Domänen = PL1-Slots (per DoD §3) |
| Runner/Orchestrierung S5(+A) | cce-runner (drei Formen, HITL-Einspielung, journey) | built_reported (MotorEngine-Nutzung gesehen) | — |
| Persistenz S9(+A) CAS+Refs | cce-store (FsCas/MemoryCas, Refs, Re-Exports) | built_reported | Sync = Betrieb (per Spec) |
| Bibliothek/Wächter S8(+A) | cce-library + conformance (GUARD_PHASES gesehen) | built_verified (Einstieg) | — |
| L9a CSA (15 Gates, 3 Adapter, 13 Zeugen, PROD-INV-13..16) | nexus/* (policy+fetch **Quelltext gesehen**), csa_catalog **gesehen** | built_verified | realer HTTP-Transport = Betrieb (Snapshot-Pfad identisch) |
| L9b Nexus-Bridge | Typ-Stubs (Ratchet-Typ, BridgeNorm-Port in library) | port_only | R-1b — per DoD §3 |
| L9c Inference/Tool (14 Gates, 5 Provider, 16 Residuen, 19 Zeugen, recorded) | cce-inference, cce-toolgateway, inference_catalog **Quelltext gesehen** | built_verified | Cloud live/Agent live = Betrieb (per F.3) |
| .loom LBC-1 (Bytes, C0–C5, 24 Zeugen, Viewer motorfrei) | loom/* (format/codec/verify **Quelltext gesehen**), format_catalog/golden | built_verified (Byteform) / built_reported (Golden-Läufe) | zstd aus (sichtbar), CDDL offen (LC-R3), Signatur-Registry offen (LC-R1) |
| Cockpit S3(+A7..A10), COCK-INV-1..8 | cockpit-core (**kanzel.rs, cock_inv.rs gesehen**), cockpit-app (egui) | built_verified (Kern) / partial (Display) | Fensterstart headless nicht demonstriert — WO-2 |
| Reise/Doku S2/S12(+A) | journey.rs, docs/operator/* (**Volltexte vorhanden**, Doku=Verhalten-Tests gesehen) | built_verified | — |
| Governance S13(+A) PROD-INV-9..20, Locks, Claim-Schranke | quer (capability, hf_import, prod_inv-Tests **gesehen**) | built_verified | — |
| Abnahme S10/Matrix a–i | ABSCHLUSSBERICHT §1/§2 + FEATURE_PL | built_reported (Formel-Auswertung) | Laufzeit = WO-1 |
| Auslieferung S11(+A) | package.sh (**gesehen**), delivery.rs, update_dod (**gesehen**) | built_verified (Pipeline) / partial (nur Linux-Artefakt) | macOS/Win = WO-3; Schlüsselbund = Betrieb |
| S14 CoreExtension | Pfad ungenutzt (keine Extension gebaut) | spec_only (bewusst) | erste echte Extension = künftiger Anwendungsfall |
| S15 Workbench (ScaleAdapter, Capsule, Multicube, MSC) | cce-spiral (S15-Strukturen), nexus-cell (Capsule-Fall) | built_reported | SCALE≥2 = per DoD §3 offen |

**Deckungsverdikt:** Es existiert **keine Spezifikationsschicht ohne benannten Ist-Ort** und keine Implementierung ohne Spec-Anker (keine unspecced_implementation gefunden; einzige Zusatzstruktur cce-conformance ist die spezifizierte Wächter-Instanz). Alle `partial`/`port_only`/`spec_only` decken sich exakt mit der Nicht-Bestandteile-Liste der Master-DoD.
