# 02 — BUILD & TEST AUDIT (Was ist bewiesen, was ist berichtet, was ist hier nicht ausführbar)

**Harte Wahrheit dieses Audits:** In dieser Chat-Umgebung existiert kein Zugriff auf das private Repository als Dateisystem und kein Rust-Toolchain-Lauf gegen den echten Stand. **Jeder Laufzeitnachweis (cargo build/test, CI, Paket, Golden-Doppellauf) ist daher `NOT_EXECUTED` und trägt den Status REPORTED** — mit einer Ausnahme-Klasse: Quelltexte, deren bloße Existenz + Inhalt die Behauptung strukturell tragen (VERIFIED_CODE). Der Hebel, der alles auf EXECUTED hebt, ist **ein einziger Verifikationslauf in Claude Code** (§4).

## 1. Behauptete Laufzeitlage (REPORTED, aus reports/)

`ci/run_ci.sh` = „CI: GRUEN" (Integrität 24/24 → fmt → clippy -D warnings → check_acyclic → alle Tests) · zuletzt gemessene Testzahlen: 101 Tests/100 Suiten (nach G2), „103 Testsuiten" (nach G3), danach „CI vollständig grün" je Phase ohne Einzelzahl · Paket `target/package/cce-loom-0.1.0-linux.tar.gz` (4,2 MB) erzeugt · `offline_core_proof` grün · golden-gen Doppellauf byte-identisch.

## 2. Was strukturell BEWIESEN ist (VERIFIED_CODE, unabhängig vom Laufzeitlauf)

1. **Die Verbote sind Typen, keine Konventionen:** `ApprovedFetchPlan{_sealed:()}` + `fetch(&ApprovedFetchPlan,…)` ⇒ kein Abrufpfad ohne Policy-Kette (PROD-INV-13 architektonisch). `GateReport::from_untyped` weist Score-Felder ab (V1 strukturell). CandidateOutput ohne Status-/Verdikt-/Ledger-Feld. Kanzel-APIs ohne Schreib-Rückgabetypen. `force_through()` liefert immer Fehler.
2. **Die Byteform stimmt mit dem Standard überein** (Magic/Footer/Frames/SEGTAB-eintragslos/Merkle-promote-odd/13-Felder-Manifest/Profil-Matrix) — jede N1/N3/N4/N9/N13-Klasse hat ihren benannten Fehlpfad im Code (`FormatError`, `DecodeError`).
3. **Die Zeugen sind echte Tests, keine Prosa:** Quelltexte von neg_1 (CountingTransport==0), r_inf_3 (egress_calls==1 NACH Gates), recorded_replay_class_identical, prod_inv_13..20, cock_inv_1..5, Doku=Verhalten-Tests (lesen docs/operator/ zur Laufzeit) — gesichtet.
4. **Der Wächter ist verdrahtet:** conformance/Cargo.toml hängt an allen Prüflingen (Motor+nexus+loom+cockpit) und bindet `inference/inference_catalog.rs` als [[test]]; GUARD_PHASES führt G0…G12.

## 3. Diskrepanz-Prüfung Zahlenwerk

**51 vs 48:** Cargo.toml = 51 Mitglieder (Ist). „48 Workspace-Crates" stammt aus dem G0-Acyclic-Log — Stand VOR G8a (ohne cce-inference, cce-toolgateway) und vor einem weiteren später hinzugefügten Mitglied; kein Widerspruch, aber der G0-Wortlaut ist historisch. **Zeugen-Summen:** 14 Gates/16 Residuen sind als `len()==14/16`-Asserts im Code (nicht nur behauptet). **24/24 Integrität:** Datei INTEGRITAET.sha256 + CI-Schritt REPORTED; Inhalt hier nicht gesichtet → §4.

## 4. WO-1: Der eine Verifikationslauf (Copy-&-Paste für Claude Code, im Repo-Root)

```bash
set -e
sha256sum -c INTEGRITAET.sha256                        # 24/24 OK erwartet
cd cce
cargo --version && rustc --version                     # 1.94.1 (rust-toolchain.toml)
cargo metadata --format-version 1 --no-deps | python3 -c "import sys,json;print(len(json.load(sys.stdin)['packages']),'workspace-crates')"   # 51 erwartet
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
python3 ci/check_acyclic.py                            # DAG + Tore + Reader-Prinzip + Symbol-Scan
cargo test --workspace 2>&1 | tail -20                 # ALLE Zeugen; Endsumme notieren
bash ci/run_ci.sh                                      # "CI: GRUEN" erwartet
bash ci/package.sh                                     # Linux-Paket; Größe notieren
cargo run -p loom-cli -- verify library/seed/drei_risiken_memo_workbody.loom
cargo run -p loom-cli -- verify library/seed/minimal_inspect.loom
cargo run -p loom-conformance --bin golden-gen && git diff --stat -- loom/  # Doppellauf: leer erwartet
```
**Abnahme WO-1:** jede Zeile grün ⇒ alle REPORTED-Laufzeitclaims dieses Audits werden EXECUTED; irgendeine rote Zeile ⇒ präziser Befund an Position, Audit-Register aktualisieren.

## 5. Verdikt dieses Dokuments

Build-/Test-Behauptungen: **konsistent, quelltextgestützt, laufzeitunbestätigt (hier)**. Es wurde kein einziger Widerspruch zwischen Berichten und gesichtetem Code gefunden; die Berichte zitieren reale Testnamen, reale Pfade, reale Konstanten. Restunsicherheit ist ausschließlich die Ausführungsdimension — und die ist mit WO-1 in Minuten schließbar.
