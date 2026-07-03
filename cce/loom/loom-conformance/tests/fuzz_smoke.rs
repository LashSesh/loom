//! Fuzz-Harness (Register #25, P6-Haertung) — DETERMINISTISCH und
//! toolchain-frei: seeded splitmix64-Mutationen (P9: keine unseeded
//! Randomness) gegen die GESAMTE Reader-Flaeche. Erwartung: NIE ein
//! Panic — nur getypte Fehler (die benannten Fehlpunkte der
//! Verify-Kette). Jede Panik waere ein Parser-Haertungsbefund (§9.2).
//!
//! Hinweis: dies ersetzt kein Coverage-geleitetes cargo-fuzz
//! (nightly/libFuzzer, Betriebsschritt) — es ist der CI-gebundene,
//! reproduzierbare Kern desselben Anliegens.

use std::panic::{catch_unwind, AssertUnwindSafe};

/// splitmix64 — identische Konstante wie im Motor (plattformstabil).
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }
}

/// Mutiert eine Kopie: 1–8 Byteflips, gelegentlich Truncation/Extension.
fn mutate(base: &[u8], rng: &mut Rng) -> Vec<u8> {
    let mut buf = base.to_vec();
    match rng.next() % 8 {
        0 => {
            // Truncation an zufaelliger Stelle
            let cut = (rng.next() as usize) % (buf.len().max(1));
            buf.truncate(cut);
        }
        1 => {
            // Extension mit Zufallsbytes
            for _ in 0..(rng.next() % 16 + 1) {
                buf.push((rng.next() & 0xff) as u8);
            }
        }
        _ => {
            let flips = rng.next() % 8 + 1;
            for _ in 0..flips {
                if buf.is_empty() {
                    break;
                }
                let i = (rng.next() as usize) % buf.len();
                buf[i] ^= (rng.next() & 0xff) as u8;
            }
        }
    }
    buf
}

fn assert_no_panic<F: FnOnce() + std::panic::UnwindSafe>(what: &str, iter: u64, f: F) {
    if catch_unwind(f).is_err() {
        panic!("PANIK in {what} bei Iteration {iter} — Parser-Haertungsbefund");
    }
}

const ITERATIONS: u64 = 4000;

#[test]
fn fuzz_canon_decode_never_panics() {
    let seeds: Vec<Vec<u8>> = vec![
        loom_canon::Cv::map(vec![
            ("a", loom_canon::Cv::Uint(1)),
            ("b", loom_canon::Cv::decimal(1234, -2)),
            (
                "c",
                loom_canon::Cv::Array(vec![loom_canon::Cv::Text("x".into())]),
            ),
        ])
        .encode()
        .unwrap(),
        vec![],
        vec![0xff; 64],
    ];
    let mut rng = Rng(0x5eed_0001);
    for i in 0..ITERATIONS {
        let base = &seeds[(rng.next() as usize) % seeds.len()];
        let input = mutate(base, &mut rng);
        assert_no_panic("loom_canon::decode", i, || {
            let _ = loom_canon::decode(&input);
        });
    }
}

#[test]
fn fuzz_frame_decode_never_panics() {
    let frame = loom_format::Frame {
        kind: loom_format::KIND_MANIFEST,
        seg_flags: 0,
        payload: b"nutzlast".to_vec(),
    }
    .encode();
    let mut rng = Rng(0x5eed_0002);
    for i in 0..ITERATIONS {
        let input = mutate(&frame, &mut rng);
        let at = (rng.next() as usize) % (input.len().max(1));
        assert_no_panic("Frame::decode", i, || {
            let _ = loom_format::Frame::decode(&input, at % input.len().max(1));
        });
    }
}

#[test]
fn fuzz_sealed_container_verify_never_panics() {
    // Basis: echtes Golden-R1 plus zwei Degenerate.
    let r1 = loom_conformance::build_r1().bytes;
    let seeds: Vec<Vec<u8>> = vec![r1, b"{\"json\":true}".to_vec(), vec![0u8; 80]];
    let mut rng = Rng(0x5eed_0003);
    for i in 0..ITERATIONS {
        let base = &seeds[(rng.next() as usize) % seeds.len()];
        let input = mutate(base, &mut rng);
        assert_no_panic("decode_sealed+verify", i, || {
            let _ = loom_codec::decode_sealed(&input);
            let report = loom_verify::verify(&input);
            // Verdikt existiert immer — nie stilles Teilergebnis.
            let _ = report.verdict;
        });
    }
}

#[test]
fn fuzz_footer_and_preamble_never_panic() {
    let sealed = loom_conformance::build_r1().bytes;
    let mut rng = Rng(0x5eed_0004);
    for i in 0..ITERATIONS {
        let input = mutate(&sealed, &mut rng);
        assert_no_panic("Preamble/Footer::decode", i, || {
            let _ = loom_format::Preamble::decode(&input);
            let _ = loom_format::Footer::decode(&input);
        });
    }
}

#[test]
fn fuzz_mutated_r_files_yield_typed_verdicts() {
    // Staerkere Eigenschaft auf ganzen Containern: JEDES mutierte
    // Golden-File liefert ein getyptes Verdikt; wird zufaellig nichts
    // getroffen, bleibt es valid — beides zulaessig, nie ein Panic.
    let mut rng = Rng(0x5eed_0005);
    for (name, builder) in loom_conformance::REFERENCE_BUILDERS {
        let base = builder().bytes;
        for i in 0..400 {
            let input = mutate(&base, &mut rng);
            let outcome = catch_unwind(AssertUnwindSafe(|| loom_verify::verify(&input).verdict));
            assert!(outcome.is_ok(), "PANIK bei {name} Iteration {i}");
        }
    }
}
