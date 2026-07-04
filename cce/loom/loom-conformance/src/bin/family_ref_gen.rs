//! family-ref-gen — schreibt die drei echten, geschlossenen ("full")
//! Familien-Referenz-Cubes (S-E5 Meilenstein R-NRM-1: D02/D03/D06) nach
//! library/seed/. Deterministisch: mehrfacher Lauf aendert kein Byte.

use cce_materialize::family_a_domains::{d02, d03, d06};
use loom_conformance::build_family_reference_workbody;

fn hex(b: &[u8; 34]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

fn main() {
    let root = std::env::args().nth(1).unwrap_or_else(|| ".".to_string());
    for (name, profile) in [("d02", d02()), ("d03", d03()), ("d06", d06())] {
        let sealed = build_family_reference_workbody(&profile);
        let path = format!("{root}/library/seed/family_ref_{name}.loom");
        std::fs::write(&path, &sealed.bytes).expect("seed write");
        println!(
            "{path}: {} bytes, core_root {}",
            sealed.bytes.len(),
            hex(&sealed.core_root)
        );
    }
}
