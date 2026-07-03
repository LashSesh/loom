//! norm-gen — schreibt den Meilenstein R-NRM-1: die erste aktive Norm
//! (Containerklasse "norm"), destilliert aus drei geschlossenen
//! Familien-Referenz-Cubes, nach library/seed/. Deterministisch:
//! mehrfacher Lauf aendert kein Byte.

use loom_conformance::build_first_active_norm;

fn hex(b: &[u8; 34]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

fn main() {
    let root = std::env::args().nth(1).unwrap_or_else(|| ".".to_string());
    let (norm, sealed, report, _members) = build_first_active_norm();
    let path = format!("{root}/library/seed/norm_relation_regel.loom");
    std::fs::write(&path, &sealed.bytes).expect("seed write");
    println!(
        "{path}: {} bytes, core_root {}, norm_id {}, verdict {:?}",
        sealed.bytes.len(),
        hex(&sealed.core_root),
        norm.norm_id,
        report.verdict()
    );
}
