#!/usr/bin/env bash
# build_wasm.sh — E4b: baut loom-viewer nach wasm32-unknown-unknown und
# erzeugt die JS-Bindungen (wasm-bindgen) nach web/pkg/. Voraussetzung:
# `rustup target add wasm32-unknown-unknown` + `cargo install
# wasm-bindgen-cli --version <= wasm-bindgen-Version in Cargo.lock>`.
set -euo pipefail
cd "$(dirname "$0")"

cargo build -p loom-viewer --target wasm32-unknown-unknown --release
mkdir -p web/pkg
wasm-bindgen --target web --out-dir web/pkg \
    ../../target/wasm32-unknown-unknown/release/loom_viewer.wasm

echo "web/pkg/ erzeugt (nicht versioniert — Build-Ausgabe, s. .gitignore)."
