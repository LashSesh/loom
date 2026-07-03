//! class-registry-gen — schreibt die Klassen-Registry (X2/E4c, Karte
//! §2/E4c) nach library/seed/. Deterministisch: mehrfacher Lauf aendert
//! kein Byte.

use loom_conformance::build_class_registry;

fn main() {
    let root = std::env::args().nth(1).unwrap_or_else(|| ".".to_string());
    let sealed = build_class_registry();
    let path = format!("{root}/library/seed/class_registry.loom");
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
