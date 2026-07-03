# 01 — ACTUAL REPOSITORY INVENTORY (Ist-Zustand, quellverifiziert)

**Quelle der Wahrheit:** der in den Projektdateien gesynchte Repository-Stand `LashSesh/loom@main` (Zugriff: Projektwissen-Index; das Repo ist privat, kein externer Clone). **Statuslegende (im gesamten Audit):** `VERIFIED_CODE` = Quelltext im Index gesehen · `VERIFIED_STRUCTURE` = Existenz/Verdrahtung über Cargo.toml/Cargo.lock/Dateiverweise belegt · `REPORTED` = nur durch Bauberichte belegt · `NOT_EXECUTED` = Laufzeitnachweis hier nicht ausführbar · `OPEN` = per Spezifikation offen.

## 1. Wurzelstruktur

```
/ (Repo-Root)
├─ 00_START_HIER.md … 04_AGENT_AUFTRAG.md, 05_RESUME_INFERENCE_AMENDMENT.md   [Übergabeschicht]
├─ INTEGRITAET.sha256                                                          [REPORTED: 24/24 OK in G0/CI]
├─ cce-spec-repo/spec/{00_kern,10_produkt,20_rebase,30_akquisition,40_format}/ [24 Specs, read-only — R-Agent-1]
├─ cce/                                                                        [das Monorepo, s. §2]
└─ reports/  G00–G12+G08a_bericht.md, RESUME_bestandsaufnahme.md, residuen.md, ABSCHLUSSBERICHT.md  [19 Dateien, VERIFIED (liegen zusätzlich als Projektdateien vor)]
```

## 2. Monorepo `cce/` — Workspace **VERIFIED_CODE**

`cce/Cargo.toml`: **51 Workspace-Mitglieder**, resolver 2, edition 2021, Version 0.1.0, `[profile.release] lto=true, codegen-units=1`:

| Gruppe | Crates (Zahl) |
|---|---|
| Motor `crates/` | cce-core, cce-lattice, cce-ccc, cce-crystal, cce-phc, cce-loom, cce-observe, cce-merkaba, cce-materialize, cce-runner (10) |
| Rebase `crates/` | cce-kernel, cce-phaseblock, cce-spiral, cce-hbm (4) |
| Substanz `crates/` | cce-store, cce-library (2) |
| Gateways L9c `crates/` | cce-inference, cce-toolgateway (2) |
| CSA `nexus/` | nexus-core, -policy, -cell, -adapter, -ingress, -fetch, -decode, -normalize, -validate, -evidence, -ledger, -store, -export, -cli (14) + adapters/{local_corpus, wikimedia, git_repository} (3) |
| Container `loom/` | loom-canon, -format, -codec, -verify, -mount, -project, -gate, -replay, -export, -viewer, -runner, -cli, -conformance (13) |
| Cockpit | cockpit-core, cockpit-app (2) |
| Testmatrix | conformance (1) |

**Cargo.lock VERIFIED_CODE:** cce-core ist **dependency-frei**; alle loom-* Crates hängen ausschließlich an loom-* (Reader-Prinzip im Lock sichtbar); einzige externe Abhängigkeitskette = eframe/egui-Stack ausschließlich unter cockpit-app; cce-inference → {cce-core, cce-store}; cce-toolgateway → {cce-core} (Tor-Trennung im Lock sichtbar, kein nexus-*-Bezug).

## 3. CI/Skripte

`ci/check_acyclic.py` **VERIFIED_CODE**: DAG/INV-11 + Schichtenregel + **IG-A1-Tor-Trennung** (Gateways ↮ nexus-*, Gateways nicht verschmolzen) + **Reader-Prinzip** (transitive Hülle von loom-viewer frei von cce-/nexus-/cockpit-*) + Socket-/HTTP-Symbol-Scan. `ci/package.sh` **VERIFIED_CODE**: Release-Build (cockpit-app, loom-cli, loom-viewer, nexus-cli), Paketbaum mit `library/seed/*.loom`, `docs/operator/*.md`, generierter `PL_KENNZEICHNUNG.md`, tar.gz. `ci/run_ci.sh`, `.github/workflows/ci.yml`, `rust-toolchain.toml` (1.94.1): **VERIFIED_STRUCTURE/REPORTED** (referenziert aus G0/CI-Bericht; Wortlaut nicht einzeln gesichtet).

## 4. Zentrale Quelltext-Anker (Stichproben VERIFIED_CODE)

`loom/loom-format/src/lib.rs` (MAGIC = 89 4C 4F 4F 4D 0D 0A 1A 0A; END_MAGIC "LOOM_END"; Footer-Layout exakt Standard Teil 2.4; Kind-Registry 0x0000–0x0064 inkl. Overlay-Kinds; FormatError inkl. `CompressionUnsupported`) · `loom-codec/src/lib.rs` (seal_canonical, SEGTAB eintragslos + SegtabSelfEntry-Reject, Merkle 0x00/0x01 + promote-odd, recompute_core_root) · `loom-verify/src/lib.rs` (MANIFEST_REQUIRED_FIELDS[13], required_kinds je Profil = Standard §3.4, L0–L2, Verdikte) · `nexus/nexus-policy/src/lib.rs` (ApprovedFetchPlan mit `_sealed: ()`, approve_fetch = einziger Konstruktor, 6 Policy-Gates) · `nexus-fetch/src/lib.rs` (fetch(&ApprovedFetchPlan,…) = einziger Netzpfad, SnapshotTransport, FetchCache/ETag) · `crates/cce-inference/src/kanzel.rs` (schreiblose Kanzel) · `cockpit/cockpit-core/src/kanzel.rs` (INTERPRETATION_MARKER, DegradedKanzel, LocalKanzel) · `conformance/src/lib.rs` (GUARD_PHASES = G0…G12 [14 Einträge], PRODUCT_VERSION 0.1.0, update_dod, FEATURE_PL [13 Funktionen, D01=PL4]) · Tests: `conformance/tests/closure_roundtrip.rs`, `product_journey.rs`, `csa_catalog.rs`, `conformance/inference/inference_catalog.rs`, `cockpit-core/tests/cock_inv.rs` — alle mit vollem Quelltext gesichtet.

## 5. Weitere Bestände (VERIFIED_STRUCTURE/REPORTED)

`docs/operator/handbuch.md` + `lesarten.md` (Volltexte liegen als Projektdateien vor = Repo-Kopien; von Tests in product_journey.rs gelesen — Doku=Verhalten) · `library/seed/{drei_risiken_memo_workbody.loom, minimal_inspect.loom}` (REPORTED + von delivery.rs/golden.rs referenziert) · `cce-materialize/src/catalog.rs` + generiertes `catalog_data.rs` (213/16, REPORTED; FEATURE_PL-Eintrag verweist auf dessen Tests) · Testkataloge k_catalog(18)/c_catalog(14)/loom_catalog(10)/tat_catalog/spiral_catalog/hbm_catalog(20)/g2_gate/g5_gate/g6_gate/format_catalog/golden.rs/guard.rs/loom_guard.rs (REPORTED mit präzisen Pfaden; Existenz konsistent mit conformance-Verdrahtung).

## 6. Zählstand der Zeugen (Soll=Ist-Behauptung)

Dokument-Zeugen (Referenz-/Negativ-Cubes) · Spiral 8G/12R · HBM-01..20 · CSA 5R+8N+PROD-INV-13..16 · Inference 7R+12N+PROD-INV-17..20 (**Quelltext gesehen**) · .loom R1–R8/N1–N16/C0–C5 · COCK-INV-1..8. Gesamtsumme laut CI-Bericht: alle unter EINEM Wächter (GUARD_PHASES **VERIFIED_CODE**). Laufzeitbestätigung: **NOT_EXECUTED** hier → Dokument 02.
