#!/usr/bin/env bash
# ci/package.sh — Auslieferungspaket (S11/G12) fuer das JEWEILIGE
# Build-OS. Inhalt: Cockpit-App, loom-CLI, motorfreier Viewer,
# nexus-Referenz-CLI, Saat-Bibliothek (.loom), Operator-Doku,
# PL-Kennzeichnung. Andere Ziel-OS erfordern deren Build-Hosts
# (dieselbe Pipeline; keine Cross-Signierung hier).
set -euo pipefail
cd "$(dirname "$0")/.."

TARGET_OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
VERSION="$(grep -m1 '^version' crates/cce-core/Cargo.toml | cut -d'"' -f2)"
PKG="cce-loom-${VERSION}-${TARGET_OS}"
OUT="target/package/${PKG}"

echo "== Release-Build =="
cargo build --release -p cockpit-app -p loom-cli -p loom-viewer -p nexus-cli

echo "== Paketbaum =="
rm -rf "$OUT"
mkdir -p "$OUT/bin" "$OUT/library/seed" "$OUT/docs/operator"
cp target/release/cce-cockpit "$OUT/bin/"
cp target/release/loom "$OUT/bin/"
cp target/release/loom-viewer "$OUT/bin/"
cp target/release/nexus "$OUT/bin/"
cp library/seed/*.loom "$OUT/library/seed/"
cp docs/operator/*.md "$OUT/docs/operator/"
cat > "$OUT/PL_KENNZEICHNUNG.md" <<'PLEOF'
# Produkt-Level-Kennzeichnung (Anti-Overclaim, S11)
- Dokument-Domaene D01 (Drei-Risiken-Memo-Reise): PL4 (Produkt-Kerntest gruen)
- Alle uebrigen 212 Katalog-Domaenen: PL1 (Registry-Slot, kein Vollausbau)
- Motor/Runner/Store/Bibliothek/HBM/CSA/.loom/Inference/Cockpit:
  gebaut + bezeugt (ein Regressionswaechter); Betriebsreife-Merkmale
  (Cloud-Provider, OAuth-Connector, Sync, Signatur-Registry) sind
  OFFEN und im End-Residuenregister gefuehrt.
- Kern ist OFFLINE lauffaehig (kein Netzpfad ausser CSA-/Gateway-Toren,
  im Bau ausschliesslich Snapshot/Mock).
PLEOF

echo "== Archiv =="
mkdir -p target/package
tar -C target/package -czf "target/package/${PKG}.tar.gz" "$PKG"
echo "PAKET: target/package/${PKG}.tar.gz"
