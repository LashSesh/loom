//! loom — Mindest-CLI (LOOM-Standard Teil 10.4), motorfreier Kernpfad:
//! inspect · verify · ls. (run/replay/export laufen ueber die
//! SDK-Ports; pack/seal ist Workbench-Sache — loom-codec-API.)

fn main() {
    let mut args = std::env::args().skip(1);
    let cmd = args.next().unwrap_or_default();
    let path = args.next().unwrap_or_default();
    if path.is_empty() {
        eprintln!("verwendung: loom <inspect|verify|ls> <datei.loom>");
        std::process::exit(2);
    }
    let bytes = match std::fs::read(&path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("lesefehler: {e}");
            std::process::exit(1);
        }
    };
    match cmd.as_str() {
        "inspect" | "ls" => match loom_mount::open(&bytes) {
            Ok(handle) => {
                let r = loom_mount::inspect(&bytes, &handle);
                println!("segmente: {:?}", r.segment_kinds);
                println!("core_root: {}", r.core_root_hex);
                println!("verdikt: {:?} · residuen: {}", r.verdict, r.residue_count);
                for d in r.diagnoses {
                    println!("  {d}");
                }
            }
            Err(e) => {
                eprintln!("reject: {e:?}");
                std::process::exit(1);
            }
        },
        "verify" => {
            let r = loom_verify::verify(&bytes);
            println!("verdikt: {:?}", r.verdict);
            for d in &r.diagnoses {
                println!("  [{}] {}: {}", d.level, d.point, d.detail);
            }
            if !matches!(
                r.verdict,
                loom_verify::Verdict::Valid | loom_verify::Verdict::ValidWithResidues
            ) {
                std::process::exit(1);
            }
        }
        other => {
            eprintln!("unbekanntes kommando '{other}'");
            std::process::exit(2);
        }
    }
}
