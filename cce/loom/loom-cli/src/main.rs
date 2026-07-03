//! loom — Mindest-CLI (LOOM-Standard Teil 10.4), motorfreier Kernpfad:
//! inspect · verify · ls · extract · extract-children · pack-zstd ·
//! unpack-zstd · hash-profile. (run/replay/export laufen ueber die
//! SDK-Ports; pack/seal ist Workbench-Sache — loom-codec-API.)

fn main() {
    let mut args = std::env::args().skip(1);
    let cmd = args.next().unwrap_or_default();
    // Signatur-Subcommands (P6(c)) haben eigene Arity — vor dem generischen
    // Datei-Lesen behandeln.
    if cmd == "keygen" || cmd == "sign" || cmd == "verify-sig" {
        dispatch_sign(&cmd);
        return;
    }
    if cmd == "pack-zstd" || cmd == "unpack-zstd" || cmd == "hash-profile" {
        dispatch_transport(&cmd);
        return;
    }
    let path = args.next().unwrap_or_default();
    if path.is_empty() {
        eprintln!(
            "verwendung: loom <inspect|verify|ls|extract|extract-children|pack-zstd|unpack-zstd|hash-profile|sign|verify-sig|keygen> <datei.loom> [ziel]"
        );
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
        "extract" => {
            let out = args.next().unwrap_or_default();
            if out.is_empty() {
                eprintln!("verwendung: loom extract <datei.loom> <ziel-pfad>");
                std::process::exit(2);
            }
            let handle = match loom_mount::open(&bytes) {
                Ok(h) => h,
                Err(e) => {
                    eprintln!("reject: {e:?}");
                    std::process::exit(1);
                }
            };
            match loom_mount::extract_artifact(&handle) {
                Ok(artifact_bytes) => {
                    std::fs::write(&out, &artifact_bytes).expect("Ziel schreiben");
                    println!("extrahiert: {out} ({} Bytes)", artifact_bytes.len());
                }
                Err(e) => {
                    eprintln!("extract-fehler: {e:?}");
                    std::process::exit(1);
                }
            }
        }
        "extract-children" => {
            let out_dir = args.next().unwrap_or_default();
            if out_dir.is_empty() {
                eprintln!("verwendung: loom extract-children <mappe.loom> <ziel-verzeichnis>");
                std::process::exit(2);
            }
            let handle = match loom_mount::open(&bytes) {
                Ok(h) => h,
                Err(e) => {
                    eprintln!("reject: {e:?}");
                    std::process::exit(1);
                }
            };
            match loom_mount::all_cas_blobs(&handle) {
                Ok(blobs) => {
                    std::fs::create_dir_all(&out_dir).expect("Zielverzeichnis anlegen");
                    for (i, child_bytes) in blobs.iter().enumerate() {
                        let child_path =
                            std::path::Path::new(&out_dir).join(format!("child-{i}.loom"));
                        std::fs::write(&child_path, child_bytes).expect("Kind schreiben");
                        let verdict = loom_verify::verify(child_bytes).verdict;
                        println!(
                            "extrahiert: {} ({} Bytes) · verdikt {:?}",
                            child_path.display(),
                            child_bytes.len(),
                            verdict
                        );
                    }
                }
                Err(e) => {
                    eprintln!("extract-fehler: {e:?}");
                    std::process::exit(1);
                }
            }
        }
        other => {
            eprintln!("unbekanntes kommando '{other}'");
            std::process::exit(2);
        }
    }
}

fn dispatch_sign(cmd: &str) {
    match cmd {
        "keygen" => {
            let mut it = std::env::args().skip(2);
            let seed_hex = it.next().unwrap_or_default();
            let out = it.next().unwrap_or_default();
            match hex32(&seed_hex) {
                Some(seed) if !out.is_empty() => {
                    std::fs::write(&out, seed).expect("keyfile schreiben");
                    println!("seed geschrieben: {out} (32 Byte)");
                }
                _ => {
                    eprintln!("verwendung: loom keygen <seed-hex-64-zeichen> <out-keyfile>");
                    std::process::exit(2);
                }
            }
        }
        "sign" => {
            let mut it = std::env::args().skip(2);
            let file = it.next().unwrap_or_default();
            let key = it.next().unwrap_or_default();
            let out = it.next().unwrap_or_default();
            let bytes = std::fs::read(&file).expect("datei lesen");
            let seed_raw = std::fs::read(&key).expect("keyfile lesen");
            let seed: [u8; 32] = seed_raw
                .as_slice()
                .try_into()
                .expect("seed muss 32 Byte sein");
            match loom_cli::sign::sign(&bytes, &seed) {
                Ok(signed) => {
                    std::fs::write(&out, &signed).expect("signierte datei schreiben");
                    println!(
                        "signiert: {out} (SIGNATURE 0x0050 beigelegt, core_root unveraendert)"
                    );
                }
                Err(e) => {
                    eprintln!("sign-fehler: {e:?}");
                    std::process::exit(1);
                }
            }
        }
        "verify-sig" => {
            let file = std::env::args().nth(2).unwrap_or_default();
            let bytes = std::fs::read(&file).expect("datei lesen");
            match loom_cli::sign::verify_sig(&bytes) {
                Ok(vk) => {
                    let hex: String = vk.to_bytes().iter().map(|b| format!("{b:02x}")).collect();
                    println!("signatur gueltig · public_key {hex}");
                }
                Err(e) => {
                    eprintln!("signatur UNGUELTIG: {e:?}");
                    std::process::exit(1);
                }
            }
        }
        _ => unreachable!(),
    }
}

fn dispatch_transport(cmd: &str) {
    let mut it = std::env::args().skip(2);
    match cmd {
        "pack-zstd" => {
            let (inp, out) = (it.next().unwrap_or_default(), it.next().unwrap_or_default());
            if inp.is_empty() || out.is_empty() {
                eprintln!("verwendung: loom pack-zstd <datei.loom> <ziel.loom.zst>");
                std::process::exit(2);
            }
            let bytes = std::fs::read(&inp).expect("datei lesen");
            let packed = loom_cli::transport::compress(&bytes);
            std::fs::write(&out, &packed).expect("ziel schreiben");
            println!(
                "gepackt: {out} ({} -> {} Bytes, zstd-19)",
                bytes.len(),
                packed.len()
            );
        }
        "unpack-zstd" => {
            let (inp, out) = (it.next().unwrap_or_default(), it.next().unwrap_or_default());
            if inp.is_empty() || out.is_empty() {
                eprintln!("verwendung: loom unpack-zstd <datei.loom.zst> <ziel.loom>");
                std::process::exit(2);
            }
            let bytes = std::fs::read(&inp).expect("datei lesen");
            match loom_cli::transport::decompress(&bytes) {
                Ok(unpacked) => {
                    std::fs::write(&out, &unpacked).expect("ziel schreiben");
                    println!("entpackt: {out} ({} Bytes)", unpacked.len());
                }
                Err(e) => {
                    eprintln!("entpack-fehler: {e:?}");
                    std::process::exit(1);
                }
            }
        }
        "hash-profile" => {
            let (inp, profile) = (it.next().unwrap_or_default(), it.next().unwrap_or_default());
            if inp.is_empty() {
                eprintln!("verwendung: loom hash-profile <datei.loom> [blake3]");
                std::process::exit(2);
            }
            let bytes = std::fs::read(&inp).expect("datei lesen");
            match profile.as_str() {
                "" | "blake3" => {
                    println!("blake3:{}", loom_cli::hashprofile::blake3_hex(&bytes));
                }
                other => {
                    eprintln!("unbekanntes hash-profile '{other}' (bekannt: blake3)");
                    std::process::exit(2);
                }
            }
        }
        _ => unreachable!(),
    }
}

fn hex32(s: &str) -> Option<[u8; 32]> {
    if s.len() != 64 {
        return None;
    }
    let mut out = [0u8; 32];
    for (i, b) in out.iter_mut().enumerate() {
        *b = u8::from_str_radix(&s[i * 2..i * 2 + 2], 16).ok()?;
    }
    Some(out)
}
