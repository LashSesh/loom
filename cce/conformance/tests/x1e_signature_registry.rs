//! Etappe X1(e) (Oekosystem-Karte §2/E1) — Zeuge: Signatur-Registry-
//! Vollform. Mehrfachsignaturen (Autor/Pruefer/ReviewGate der Familie
//! P) akkumulieren additiv im SIGNATURE-Segment (0x0050); core_root
//! bleibt durch jede Signatur unberuehrt; `verify_sig_all` prueft die
//! ganze Kette, jede Signatur einzeln gruen/rot.

use loom_cli::sign::{sign, verify_sig_all, ROLE_AUTHOR, ROLE_REVIEWER, ROLE_REVIEW_GATE};
use loom_conformance::build_welt_kristall_wikimedia;

#[test]
fn three_role_signature_chain_on_the_real_welt_kristall_container() {
    let base = build_welt_kristall_wikimedia();
    let root_before = loom_mount::open(&base.bytes)
        .unwrap()
        .decoded
        .footer
        .core_root;

    let author_seed = [11u8; 32];
    let reviewer_seed = [22u8; 32];
    let gate_seed = [33u8; 32];

    let s1 = sign(&base.bytes, &author_seed, ROLE_AUTHOR).expect("Autor signiert");
    let s2 = sign(&s1, &reviewer_seed, ROLE_REVIEWER).expect("Pruefer signiert");
    let s3 = sign(&s2, &gate_seed, ROLE_REVIEW_GATE).expect("ReviewGate signiert");

    // core_root unveraendert durch alle drei Signaturvorgaenge.
    let root_after = loom_mount::open(&s3).unwrap().decoded.footer.core_root;
    assert_eq!(root_before, root_after);

    // Der Container bleibt als Ganzes Valid — Signaturen sind non_core
    // und stoeren die bestehenden Pflichtsegmente/Gates nicht.
    let report = loom_verify::verify(&s3);
    assert_eq!(report.verdict, loom_verify::Verdict::Valid);

    // Alle drei Rollen sind vorhanden UND jede fuer sich gruen.
    let entries = verify_sig_all(&s3).expect("Kette pruefen");
    assert_eq!(entries.len(), 3);
    assert!(entries.iter().all(|e| e.valid));
    let mut roles: Vec<&str> = entries.iter().map(|e| e.role.as_str()).collect();
    roles.sort();
    let mut expected = vec![ROLE_AUTHOR, ROLE_REVIEWER, ROLE_REVIEW_GATE];
    expected.sort();
    assert_eq!(roles, expected);
}

#[test]
fn tampered_signature_on_the_real_container_is_red_without_hiding_the_others() {
    use loom_canon::Cv;
    use loom_codec::{seal_canonical, Segment};
    use loom_format::{KIND_HEADER, KIND_SEGTAB, KIND_SIGNATURE};

    let base = build_welt_kristall_wikimedia();
    let s1 = sign(&base.bytes, &[1u8; 32], ROLE_AUTHOR).unwrap();
    let s2 = sign(&s1, &[2u8; 32], ROLE_REVIEWER).unwrap();

    // Eine der beiden Signaturen inhaltlich manipulieren (Signatur-Bytes,
    // nicht core_root) UND den Container neu versiegeln — der einzig
    // korrekte Weg, "diese eine Signatur ist kryptografisch falsch"
    // darzustellen, ohne die Frame-Digest-Pruefung (Reader-Prinzip, L0)
    // vorzeitig mit einem blossen Byte-Flip auszuloesen.
    let dec = loom_codec::decode_sealed(&s2).expect("dekodieren");
    let mut segments: Vec<Segment> = Vec::new();
    let mut tampered_once = false;
    for (e, f) in &dec.frames {
        if e.kind == KIND_HEADER || e.kind == KIND_SEGTAB {
            continue;
        }
        if e.kind == KIND_SIGNATURE && !tampered_once {
            let Cv::Map(fields) = loom_canon::decode(&f.payload).unwrap() else {
                panic!("SIGNATURE ist keine Map")
            };
            let get = |k: &str| {
                fields
                    .iter()
                    .find(|(key, _)| matches!(key, Cv::Text(s) if s == k))
                    .map(|(_, v)| v.clone())
                    .unwrap()
            };
            let Cv::Bytes(mut sig_bytes) = get("signature") else {
                panic!("signature")
            };
            sig_bytes[0] ^= 0xff;
            let bad = Cv::map(vec![
                ("alg", get("alg")),
                ("signed", get("signed")),
                ("role", get("role")),
                ("public_key", get("public_key")),
                ("signature", Cv::Bytes(sig_bytes)),
            ]);
            segments.push(Segment {
                kind: KIND_SIGNATURE,
                seg_flags: e.seg_flags,
                payload: bad.encode().unwrap(),
                deps: e.deps.clone(),
            });
            tampered_once = true;
        } else {
            segments.push(Segment {
                kind: e.kind,
                seg_flags: e.seg_flags,
                payload: f.payload.clone(),
                deps: e.deps.clone(),
            });
        }
    }
    assert!(
        tampered_once,
        "es muss eine Signatur zum Manipulieren geben"
    );
    let resealed = seal_canonical("full", &["full"], &segments).unwrap();

    let entries = verify_sig_all(&resealed.bytes).unwrap();
    assert_eq!(
        entries.len(),
        2,
        "beide Signaturen bleiben strukturell vorhanden"
    );
    let valid_count = entries.iter().filter(|e| e.valid).count();
    assert_eq!(
        valid_count, 1,
        "genau eine Signatur muss durch die Manipulation rot werden, die andere bleibt gruen"
    );
}
