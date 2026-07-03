//! citing-memo-gen — schreibt das erste Memo, das sich real ueber
//! Workbody-Grenzen hinweg auf den Welt-Kristall stuetzt (X2/E2, S-E2a
//! Teil I), nach library/seed/. Deterministisch: mehrfacher Lauf
//! aendert kein Byte.

use loom_conformance::{build_welt_kristall_wikimedia, seal_citing_memo_welt_kristall};

fn main() {
    let root = std::env::args().nth(1).unwrap_or_else(|| ".".to_string());
    let welt = build_welt_kristall_wikimedia();
    let welt_root_hex = hex(&welt.core_root);
    let sealed = seal_citing_memo_welt_kristall(&welt_root_hex);
    let path = format!("{root}/library/seed/citing_memo_welt_kristall.loom");
    std::fs::write(&path, &sealed.bytes).expect("seed write");
    println!(
        "{path}: {} bytes, core_root {}, zitiert {welt_root_hex}",
        sealed.bytes.len(),
        hex(&sealed.core_root)
    );
}

fn hex(b: &[u8; 34]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}
