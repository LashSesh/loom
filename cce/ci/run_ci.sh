#!/usr/bin/env bash
# Deterministische CI-Pipeline (01_MASTER_BUILD G0):
#   Integritaet -> fmt -> clippy(-D warnings) -> check_acyclic -> test
# fail-closed: erster Fehler bricht ab.
set -euo pipefail
cd "$(dirname "$0")/.."

echo "== [1/5] Spec-Integritaet (24 SHA-256) =="
(cd ../cce-spec-repo && sha256sum -c INTEGRITAET.sha256 --quiet) && echo "Integritaet: OK"

echo "== [2/5] cargo fmt --check =="
cargo fmt --all -- --check

echo "== [3/5] cargo clippy -D warnings =="
cargo clippy --workspace --all-targets -- -D warnings

echo "== [4/5] check_acyclic (INV-11) =="
python3 ci/check_acyclic.py

echo "== [5/5] cargo test (alle Zeugen; Regressionswaechter) =="
cargo test --workspace --quiet

echo "CI: GRUEN"
