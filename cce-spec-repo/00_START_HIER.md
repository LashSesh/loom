# 00 — START HIER: CCE-Spezifikations-Repository (Übergabe an den Implementierungs-Agenten)

**Dieses Repository enthält die vollständige, geschlossene Papier-Spezifikation der Crystalline Closure Engine (CCE)** — Motor, Produkt, Domänen-Katalog, Fundament-Rebase, Kinematik, Mining-Chassis, Source-Akquisition und das `.loom`-Containerformat. Es ist so gebaut, dass ein Coding-Agent **ohne Suchen und ohne eine einzige neue Architekturentscheidung** die Implementierung bis zur definierten 100%-Abnahme durchführt.

**Deine ersten vier Schritte als Agent:**
1. Lies dieses Dokument vollständig.
2. Führe den Integritätscheck aus `03_INVENTAR.md` aus (SHA-256 aller Spezifikationen).
3. Lies die Spezifikationen in der Leseordnung (§4).
4. Arbeite danach ausschließlich `01_MASTER_BUILD.md` ab — Phase für Phase, Gate für Gate — bis `02_MASTER_DOD.md` vollständig grün ist.

---

## 1 — Die Pyramide (Schichten → Dokumente)

Die Architektur ist eine Vertikale von zwölf Ebenen (Systemlandkarte vNext). Jede Ebene ist durch benannte Dokumente gedeckt; nichts liegt daneben:

```
L11  Produkt/Delivery/Governance/Doku   → spec/10_produkt/S2–S13 (+Amendments in 20_rebase, 30_akquisition)
L10  Operational Multicube (Workbench)  → spec/20_rebase/S15_OPERATIONAL_MULTICUBE_WORKBENCH.md
L9b  Nexus-Bridge (BridgeNorm…)         → nur Ports (sichtbares Residuum R-1b) — NICHT bauen
L9a  Source-Akquisition (CSA)           → spec/30_akquisition/CSA_NEXUS_AKQUISITION.md (+Amendments)
L8   Domänen-Katalog/Adapter            → spec/10_produkt/S1_DOKUMENT.md, S1_DOMAENENKATALOG.md
L7   HBM Mining-Chassis                 → spec/20_rebase/REBASE_KONSOLIDIERUNG.md §3 + S15 §8
L6   Spiralprozess/Wrap/Multi-Ratchet   → spec/20_rebase/REBASE_KONSOLIDIERUNG.md + S15 §6–7
L5   BlueCube/RedCube/Skalenleiter      → dito (Unified Normal Form, Formel 2)
L4   PhaseBlock-HyperDAG                → dito (Formeln, Ledger = CommitProjection)
L3   PSP Execution-Kernel               → REBASE_KONSOLIDIERUNG §1 (PSPcore)
L2   PHC/LOOM/Workbody                  → spec/00_kern/BAUVERFASSUNG.md (Teil 3–5)
L1   Closure-Kern (Crystal/Gate/Replay) → BAUVERFASSUNG (Teil 0, 7, 8)
L0   Irreduzibles Fundament (F1–F6)     → BAUVERFASSUNG Teil 0 + REBASE_KONSOLIDIERUNG §1
Quer: .loom (physische Außenform)       → spec/40_format/LOOM_CONTAINER_STANDARD_V1.md
Quer: S14 CoreExtension (vertikale Achse), S1.8 DomainAdapter (horizontale Achse)
```

## 2 — Autoritätsordnung (bei scheinbarem Konflikt)

1. **Fundamentinvarianten** F1–F6, INV-1..14, Prohibitionen V1–V10, PROD-INV-9..16 — unantastbar.
2. **BAUVERFASSUNG.md** — das Rückgrat (Objekt-/Operatormodell, Gates, One-Shot-Ordnung Teil 9).
3. **Amendment-Schichten** (jünger schlägt älter, append-only): `REBASE_S1-S14.md` und `REBASE_CSA_AMENDMENTS.md` sind **normative Overlays** über den S-Spezifikationen — die S-Dateien sind physisch unverändert (geführtes Residuum R-9); du MUSST jede S-Spec **zusammen mit ihren Amendments** lesen.
4. Die jeweilige **Detail-Spezifikation** (S1–S15, CSA, LOOM-Standard).
5. Bei verbleibender Unklarheit: **nicht raten, nicht entscheiden** — als Residuum im Baubericht melden (Format in `04_AGENT_AUFTRAG.md`) und mit dem nächsten unabhängigen Schritt fortfahren.

## 3 — Begriffsklärung (drei Mal „LOOM", kein Konflikt)

- **LOOM** (Theorie): die generative Auswebung Crystal→Workbody (Fundament, L2).
- **`cce-loom`** (Crate, Bauverfassung Teil 6): die Weave-/Distribute-Engine im Motor.
- **`.loom` / `loom-*`** (Format + Crates): der Workbody-Container (spec/40_format). 
Ebenso: **SCALE-0…8** = Skalenleiter, **S1–S15** = Spezifikationen, **PL0–PL4** = Produkt-Reifegrade, **L0–L11** = Landkarten-Ebenen (Nomenklatur-Normalisierung, REBASE_KONSOLIDIERUNG §1.4).

## 4 — Leseordnung (verbindlich, einmal vollständig vor Baubeginn)

1. `00_START_HIER.md` (dieses Dokument) → 2. `03_INVENTAR.md` (+ Integritätscheck) → 3. `spec/00_kern/BAUVERFASSUNG.md` → 4. `SYSTEMLANDKARTE.md` → 5. `SYSTEMLANDKARTE_VNEXT.md` → 6. `spec/20_rebase/REBASE_KONSOLIDIERUNG.md` → 7. `REBASE_S1-S14.md` → 8.–21. `spec/10_produkt/`: S1_DOKUMENT, S1_DOMAENENKATALOG, S2…S14 (jeweils mit Amendment-Blick aus 7.) → 22. `S15_OPERATIONAL_MULTICUBE_WORKBENCH.md` → 23. `spec/30_akquisition/CSA_NEXUS_AKQUISITION.md` → 24. `REBASE_CSA_AMENDMENTS.md` → 25. `spec/40_format/LOOM_CONTAINER_STANDARD_V1.md` → 26. `01_MASTER_BUILD.md` → 27. `02_MASTER_DOD.md` → 28. `04_AGENT_AUFTRAG.md`.

## 5 — Ziel-Verzeichnisstruktur des Baus (du erzeugst sie neben `spec/`)

```
cce/                       # EIN Rust-Monorepo-Workspace (Rust stable, gepinnt)
  crates/                  # Motor: cce-core, cce-lattice, cce-ccc, cce-crystal, cce-phc,
                           #   cce-loom (Weave), cce-observe, cce-merkaba, cce-materialize,
                           #   cce-runner  (Bauverfassung Teil 6)
                           # Rebase: cce-kernel, cce-phaseblock, cce-spiral, cce-hbm
                           # Substanz: cce-store, cce-library
  nexus/                   # CSA-Workspace-Mitglieder: nexus-core … nexus-cli, adapters/ (CSA §13)
  loom/                    # Container: loom-format … loom-conformance, golden/, schemas/ (LOOM Teil 11)
  cockpit/                 # natives Desktop-Cockpit (egui) + cockpit-core (S3)
  library/seed/            # Saat-Zeugen; ab Phase G9 als .loom
  conformance/             # übergreifende Testmatrix + Regressionswächter-Einstieg
  docs/                    # generierte Operator-Doku (S12)
spec/                      # DIESE 24 Dokumente — read-only, niemals editieren
```

## 6 — Nicht-Verhandelbares (Kurzform; Vollform in den Specs)

Gates sind boolesch, fail-closed, begründet — **Score ordnet, Gate entscheidet** (A7, V1). Jede Unfertigkeit ist **sichtbares Residuum**, nie Auslassung. Kein Netz außer über die CSA-Kette (SourceHorizon → PolicyGates → EvidencePack → Ledger; `disallowed_actions` verbindlich: keine Umgehung von Auth/Captcha/Paywall/Bot-Schutz/Rate-Limits/Terms). Keine versteckte Ausführung beim Öffnen von `.loom`. KI-Kanzel hat keinen Schreibpfad auf Urteile/Residuen/Ledger. Ledger = CommitProjection(HyperDAG), kein Blockchain-Import. Kandidat ≠ Commit: nichts wird PhaseBlock ohne Gate+Evidence+Replay+sichtbares Residuum. Ein Regressionswächter schützt alles (Domänen, Updates, CoreExtensions, CSA, `.loom`).

**Kerntest (Bauachse):** `Reanalyze(Materialize(LOOM(Project(PHC(Crystal))))) ≃ Crystal` — auf jeder Skala als Proof-of-Closure.

*Weiter mit `01_MASTER_BUILD.md`.*
