//! welt-kristall-gen — schreibt das erste Welt-Crystal (Vollausbau
//! Block 3: JSON->CSU-Extraktor Wikimedia) nach library/seed/.
//! Deterministisch: mehrfacher Lauf aendert kein Byte (gleiche eingefrorene
//! Fixture, gleicher Motor-Lauf, gleiche Digests).

use loom_conformance::build_welt_kristall_wikimedia;

fn main() {
    let root = std::env::args().nth(1).unwrap_or_else(|| ".".to_string());
    let sealed = build_welt_kristall_wikimedia();
    let path = format!("{root}/library/seed/kristall_wikimedia_workbody.loom");
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
