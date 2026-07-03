//! scale2-folder-b-gen — schreibt "Mappe B" (X2/E2, SCALE-3-Zelle) nach
//! library/seed/: buendelt das zitierende Memo statt eines neutralen
//! Kurzhinweises. Deterministisch: mehrfacher Lauf aendert kein Byte.

use loom_conformance::{build_scale2_folder_b, build_welt_kristall_wikimedia};

fn main() {
    let root = std::env::args().nth(1).unwrap_or_else(|| ".".to_string());
    let welt = build_welt_kristall_wikimedia();
    let welt_root_hex = hex(&welt.core_root);
    let sealed = build_scale2_folder_b(&welt_root_hex);
    let path = format!("{root}/library/seed/scale2_projektmappe_b.loom");
    std::fs::write(&path, &sealed.bytes).expect("seed write");
    println!(
        "{path}: {} bytes, core_root {}",
        sealed.bytes.len(),
        hex(&sealed.core_root)
    );
}

fn hex(b: &[u8; 34]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}
