Phase: G0 Repo-Setup, Integrität, Toolchain
Eingang erfüllt: ja (Beauftragung 04_AGENT_AUFTRAG; Leseordnung 00_START_HIER §4 vollständig durchlaufen)
Gebaut:
- Integritätscheck: `sha256sum -c INTEGRITAET.sha256` in `cce-spec-repo/` — **24/24 OK**.
- Monorepo `cce/` gemäß Zielstruktur (00_START_HIER §5): 16 Motor-Crates (`crates/`),
  14 CSA-Crates + 3 Adapter (`nexus/`, CSA §13), 13 Container-Crates (`loom/`, LOOM Teil 11),
  `cockpit/` (cockpit-core + cockpit-app), `library/seed/`, `conformance/` (Harness leer, CI-gebunden),
  `docs/`, `ci/`, `schemas/`.
- Workspace-`Cargo.toml` (ein Workspace, resolver 2), `rust-toolchain.toml` gepinnt auf **1.94.1**.
- CI: `.github/workflows/ci.yml` + lokal ausführbar `cce/ci/run_ci.sh`
  (Integrität → fmt → clippy -D warnings → check_acyclic → test; fail-closed).
- `ci/check_acyclic.py` (INV-11): DAG-Prüfung + Schichtenregel (kein cce-* → loom-/nexus-/cockpit-*
  außer deklarierte SDK-Ports cce-runner/cce-observe).
Ausgangs-Gate:
- Integritätscheck grün = 24/24 `OK` (Kommando-Ausgabe, siehe CI-Schritt 1)
- Workspace baut leer = grün (`cargo build --workspace`)
- CI läuft = grün (`cce/ci/run_ci.sh` → "CI: GRUEN")
- Acyclic-Check aktiv = grün (`ci/check_acyclic.py` → "OK (48 Workspace-Crates, DAG, Schichten sauber)")
Residuen dieser Phase:
- R-Agent-1 (siehe reports/residuen.md): Ablageort der Spezifikationen.
Abweichungen von der Spec:
- Die Zielstruktur (00_START_HIER §5) zeigt `spec/` als Geschwisterverzeichnis von `cce/`.
  Im übergebenen Repository liegen die 24 Spezifikationen unter `cce-spec-repo/spec/`
  (ZIP-Layout des Auftraggebers). Es wird KEINE Kopie/Verschiebung vorgenommen
  (spec bleibt physisch unangetastet, R-9-Disziplin); CI und Verweise nutzen
  `cce-spec-repo/spec/` als read-only Wahrheitsquelle. Keine inhaltliche Abweichung.
