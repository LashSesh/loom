//! risikomatrix-gen — schreibt R-TBL-1 (X2/E4a, CE-1 Tabellen-Zellentyp)
//! nach library/seed/. Deterministisch: mehrfacher Lauf aendert kein Byte.

use loom_conformance::build_risikomatrix_workbody;

fn main() {
    let root = std::env::args().nth(1).unwrap_or_else(|| ".".to_string());
    let sealed = build_risikomatrix_workbody();
    let path = format!("{root}/library/seed/risikomatrix_memo_workbody.loom");
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
