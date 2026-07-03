//! blueprint-reference-gen — schreibt den strukturellen
//! Blueprint-Kristall-Platzhalter (X2/E2, SCALE-3-Zelle) nach
//! library/seed/. Die echte, aus dem Eigenkorpus zertifizierte
//! Blueprint-Kette liefert Etappe X2/E3.

use loom_conformance::build_blueprint_reference_cube;

fn main() {
    let root = std::env::args().nth(1).unwrap_or_else(|| ".".to_string());
    let sealed = build_blueprint_reference_cube();
    let path = format!("{root}/library/seed/blueprint_reference_cube.loom");
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
