//! scale2-folder-gen — schreibt die physisch gebuendelte SCALE-2-
//! Dokumentenmappe (Vollausbau Etappe X1b) nach library/seed/.
//! Deterministisch: mehrfacher Lauf aendert kein Byte.

use loom_conformance::build_scale2_folder_full;

fn main() {
    let root = std::env::args().nth(1).unwrap_or_else(|| ".".to_string());
    let sealed = build_scale2_folder_full();
    let path = format!("{root}/library/seed/scale2_projektmappe_full.loom");
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
