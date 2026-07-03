//! blueprint-eigenkorpus-gen — schreibt den ECHTEN, aus dem Eigenkorpus
//! (213 Familien-Referenzprofile + Katalog) zertifizierten
//! Blueprint-Kristall (X2/E3, Karte §2/E3) nach library/seed/.
//! Deterministisch: mehrfacher Lauf aendert kein Byte (HBM-20-Disziplin).

use loom_conformance::build_blueprint_eigenkorpus;

fn main() {
    let root = std::env::args().nth(1).unwrap_or_else(|| ".".to_string());
    let sealed = build_blueprint_eigenkorpus();
    let path = format!("{root}/library/seed/blueprint_eigenkorpus.loom");
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
