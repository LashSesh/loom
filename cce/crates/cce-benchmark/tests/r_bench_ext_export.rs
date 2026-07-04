//! R-BENCH-EXT-1 — GroundingPacket-Export fuer den Fremdarm (Dokument 22
//! §1/§5). LAEUFT NIE in `cargo test --workspace` (`#[ignore]`); rein ein
//! Betriebsschritt, der das GEMEINSAME GroundingPacket + die Digests
//! ausgibt, die BEIDE Arme (CCE nativ, Fremdwerkzeug als abgeleitete
//! Exportdatei) tragen muessen.
//!
//! Konvention:
//!   cargo test -p cce-benchmark --test r_bench_ext_export -- \
//!       --ignored --nocapture
//!
//! Die Ausgabe ist die massgebliche Quelle fuer:
//!   * die `.cursorrules`-artige Exportdatei, die der Mensch dem
//!     Fremdwerkzeug gibt (nicht-massgebliche Projektion, Dok 21 §2),
//!   * den `packet_digest` (muss auf beiden Armen identisch sein, §3),
//!   * den geerdeten `task_package_digest` (packet_digest eingefaltet),
//!     den auch der CCE-Arm-Lauf tragen wird.
//!
//! Das Aufgabenpaket ist DASSELBE numkit-artige Bug-Fix-Paket wie
//! R-BENCH-1 (Dokument 20) — jetzt gegen einen dritten Arm (§4).

use cce_benchmark::model::{BenchmarkTaskPackage, TaskClass};
use cce_swe::grounding::{
    compile_grounding, export_context, GroundingPacket, RuleAtom, RuleSeverity,
};

/// Das freigegebene R-BENCH-EXT-1-Aufgabenpaket (numkit, Vorzeichenfehler
/// in max_of) — identisch zu R-BENCH-1 §4, damit der Vergleich stimmt.
fn ext_coding_package() -> BenchmarkTaskPackage {
    BenchmarkTaskPackage {
        package_id: "r-bench-ext-1-coding".to_string(),
        task_class: TaskClass::Coding,
        task_text: "In src/lib.rs gibt die Funktion max_of das MINIMUM statt das MAXIMUM \
             zurueck (der Vergleichsoperator ist vertauscht). Behebe den Fehler, sodass \
             `cargo test` gruen wird. Aendere nur src/lib.rs."
            .to_string(),
        starting_files: vec![
            (
                "Cargo.toml".to_string(),
                b"[package]\nname = \"numkit\"\nversion = \"0.1.0\"\nedition = \"2021\"\n".to_vec(),
            ),
            (
                "src/lib.rs".to_string(),
                b"pub fn max_of(v: &[i64]) -> i64 {\n    let mut m = v[0];\n    for &x in v {\n        if x < m {\n            m = x;\n        }\n    }\n    m\n}\n".to_vec(),
            ),
            (
                "tests/it.rs".to_string(),
                b"use numkit::max_of;\n\n#[test]\nfn finds_maximum() {\n    assert_eq!(max_of(&[1, 9, 3]), 9);\n}\n".to_vec(),
            ),
        ],
        success_criteria: "cargo test gruen; nur src/lib.rs geaendert".to_string(),
        build_command: vec!["cargo".to_string(), "build".to_string()],
        test_command: vec!["cargo".to_string(), "test".to_string()],
        target_path: "src/lib.rs".to_string(),
    }
}

/// Das GEMEINSAME GroundingPacket (§1): drei belegte, damit nicht
/// herabgestufte Regeln. Beide Arme erhalten genau dies.
fn ext_shared_packet() -> GroundingPacket {
    compile_grounding(
        "r-bench-ext-1-coding",
        vec![
            RuleAtom {
                rule_id: "no-unwrap-in-src".to_string(),
                scope: "src/".to_string(),
                trigger: ".unwrap()".to_string(),
                prescription: "Kein `.unwrap()` in src/ — Fehler mit `?` propagieren (panics in \
                     Bibliothekscode sind verboten)."
                    .to_string(),
                severity: RuleSeverity::Blocking,
                evidence_ref: Some("CLAUDE.md#fehlerbehandlung".to_string()),
                gate_ref: Some("RuleComplianceGate".to_string()),
                decay: None,
            },
            RuleAtom {
                rule_id: "no-panic-macro".to_string(),
                scope: "src/".to_string(),
                trigger: "panic!(".to_string(),
                prescription: "Keine `panic!`-Makros im Zielcode.".to_string(),
                severity: RuleSeverity::Required,
                evidence_ref: Some("CLAUDE.md#fehlerbehandlung".to_string()),
                gate_ref: None,
                decay: None,
            },
            RuleAtom {
                rule_id: "no-unsafe".to_string(),
                scope: "src/".to_string(),
                trigger: "unsafe ".to_string(),
                prescription: "Kein `unsafe` fuer diese Aufgabe.".to_string(),
                severity: RuleSeverity::Blocking,
                evidence_ref: Some("CLAUDE.md#sicherheit".to_string()),
                gate_ref: Some("RuleComplianceGate".to_string()),
                decay: None,
            },
        ],
        vec![],
        vec![
            "fs_write".to_string(),
            "build".to_string(),
            "test".to_string(),
        ],
    )
    .packet
}

#[test]
#[ignore = "Betriebsschritt: erzeugt den GroundingPacket-Export + Digests"]
fn export_grounding_packet_for_ext_arm() {
    let pkg = ext_coding_package();
    let packet = ext_shared_packet();

    let packet_digest = packet.digest_hex();
    let grounded = pkg.grounded_digest_hex(&packet);
    let base = pkg.digest_hex();

    println!("=== R-BENCH-EXT-1 GroundingPacket-Export ===");
    println!("package_id            : {}", pkg.package_id);
    println!("task_class            : {}", pkg.task_class.as_str());
    println!("base_task_digest      : {base}");
    println!("packet_digest         : {packet_digest}");
    println!("grounded_task_digest  : {grounded}");
    println!("context_size (Zeichen): {}", packet.context_size());
    println!();
    println!("--- export_context (kanonische, NICHT-massgebliche Projektion) ---");
    println!("{}", export_context(&packet));
    println!("--- Ausgangsdateien (was der Fremdarm als Startzustand erhaelt) ---");
    for (path, content) in &pkg.starting_files {
        println!(
            "  {path} ({} Bytes)\n{}",
            content.len(),
            String::from_utf8_lossy(content)
        );
    }
    println!("=== Ende Export ===");
}
