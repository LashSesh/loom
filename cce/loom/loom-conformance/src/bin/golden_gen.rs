//! golden-gen — schreibt die canonical-stored Golden Files (R1–R8)
//! nach loom/golden/ und die Saat-Bibliothek nach library/seed/.
//! Deterministisch: mehrfacher Lauf aendert kein Byte.

use loom_conformance::REFERENCE_BUILDERS;

fn main() {
    let root = std::env::args().nth(1).unwrap_or_else(|| ".".to_string());
    for (name, builder) in REFERENCE_BUILDERS {
        let sealed = builder();
        let path = format!("{root}/loom/golden/{}.loom", name.to_lowercase());
        std::fs::write(&path, &sealed.bytes).expect("golden write");
        println!(
            "{path}: {} bytes, core_root {}",
            sealed.bytes.len(),
            hex(&sealed.core_root)
        );
    }
    // Saat-Bibliothek (G9-Arbeitsauftrag): der Full-Workbody (R7,
    // Drei-Risiken-Memo) und der Inspektions-Minimalkoerper (R1).
    for (name, seed_name) in [
        ("R7", "drei_risiken_memo_workbody"),
        ("R1", "minimal_inspect"),
    ] {
        let sealed = REFERENCE_BUILDERS
            .iter()
            .find(|(n, _)| *n == name)
            .unwrap()
            .1();
        let path = format!("{root}/library/seed/{seed_name}.loom");
        std::fs::write(&path, &sealed.bytes).expect("seed write");
        println!("{path}: {} bytes", sealed.bytes.len());
    }
}

fn hex(b: &[u8; 34]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}
