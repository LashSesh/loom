# Verifikationslauf T0 (WO-1 aus 02_BUILD_AND_TEST_AUDIT.md §4)

Ausgeführt: real, in dieser Repository-Arbeitskopie, Branch
`claude/loom-cce-build-4lx1te` (= origin/main nach Merge von PR #1 +
Audit-Upload a488719). Jede Zeile: Kommando · Exit-Code · Kurzausgabe.
Rein mechanische Abweichungen vom Audit-Wortlaut sind je Zeile vermerkt
(keine Architektur-/Spec-Änderung; spec/ unangetastet).

| # | Kommando (wie ausgeführt) | Exit | Kurzausgabe |
|---|---|---|---|
| 1 | `cd cce-spec-repo && sha256sum -c INTEGRITAET.sha256` | 0 | **24/24 OK** (alle Zeilen „: OK", Zählung 24). *Mechanisch: Datei liegt in `cce-spec-repo/`, nicht im Repo-Root — Pfad angepasst, Inhalt unverändert.* |
| 2 | `cargo --version && rustc --version` (in `cce/`) | 0 | cargo 1.94.1 (29ea6fb6a) · rustc 1.94.1 (e408947bf) — Pin aus rust-toolchain.toml greift |
| 3 | `cargo metadata … \| python3 -c "…len(packages)…"` | 0 | **51 workspace-crates** (erwartet 51 ✓) |
| 4 | `cargo fmt --all -- --check` | 0 | keine Abweichung |
| 5 | `cargo clippy --workspace --all-targets -- -D warnings` | 0 | Finished, 0 Warnungen |
| 6 | `python3 ci/check_acyclic.py` | 0 | „check_acyclic: OK (51 Workspace-Crates, DAG, Schichten sauber, Tor-Trennung + Socket-Scan sauber)" — inkl. Reader-Prinzip-Regel |
| 7 | `cargo test --workspace` | 0 | **291 passed · 0 failed · 122 Suiten** (Volllog: alle `test result: ok.`, kein FAILED) |
| 8 | `bash ci/run_ci.sh` | 0 | letzte Zeile: **„CI: GRUEN"** |
| 9 | `bash ci/package.sh` | 0 | `target/package/cce-loom-0.1.0-linux.tar.gz` erzeugt, **4 226 658 Bytes** (≈4,2 MB, deckungsgleich mit G12-Bericht) |
| 10 | `cargo run -p loom-cli --bin loom -- verify library/seed/drei_risiken_memo_workbody.loom` | 0 | „verdikt: Valid". *Mechanisch: `--bin loom` ergänzt (Paket `loom-cli` trägt die Binary `loom`).* |
| 11 | `cargo run -p loom-cli --bin loom -- verify library/seed/minimal_inspect.loom` | 0 | „verdikt: Valid" |
| 12 | `cargo run -p loom-conformance --bin golden-gen -- .` dann `git diff --stat -- cce/loom cce/library` | 0 | **Diff leer, Arbeitsbaum sauber** (`git status --porcelain` ohne Einträge) — Doppellauf byte-identisch. *Mechanisch: golden-gen nimmt das Wurzelverzeichnis als Argument (`-- .`); diff-Pfade auf `cce/…` präfixiert, da git-Root eine Ebene höher liegt.* |

## Abnahme WO-1

**Jede Zeile grün ⇒ alle REPORTED-Laufzeitclaims des Audits sind ab
sofort EXECUTED** (02_BUILD_AND_TEST_AUDIT §4-Abnahmeklausel):
CI-Grün, Testlauf (aktuelle Endsumme: 291/0 über 122 Suiten —
Referenzzahl für künftige Audits), Paketbau + Größe, Seed-.loom-Verify,
Golden-Doppellauf-Byteidentität, Integrität 24/24, Crate-Zahl 51.

## Mechanische Abweichungen (vollständig)

1. `INTEGRITAET.sha256`: Pfad `cce-spec-repo/` statt Repo-Root (Zeile 1).
2. `loom-cli`-Aufruf braucht `--bin loom` (Zeilen 10/11).
3. `golden-gen` braucht das Root-Argument `.`; der Doppellauf-Diff wird
   vom git-Root mit `cce/`-Präfix geprüft (Zeile 12).

Keine inhaltliche rote Zeile; kein Befund fürs Audit-Register.
