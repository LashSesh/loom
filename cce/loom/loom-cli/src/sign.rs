//! P6(c) + X1(e) (Oekosystem-Karte §2/E1) — Ed25519-Signatur(en) ueber
//! `core_root` ins SIGNATURE-Segment (Kind 0x0050, non_core: kann nicht
//! Teil des Signierten sein, LOOM Teil 9.5). Krypto liegt AUSSCHLIESSLICH
//! hier im CLI-Blatt.
//!
//! Signatur-Registry-Vollform (X1e): `sign()` ist ADDITIV — jeder Aufruf
//! fuegt eine WEITERE Signatur hinzu (eigenes SIGNATURE-Segment, eigener
//! Digest, dieselbe Dedupe-Logik wie bei mehreren CAS_BLOB-Kindern,
//! X1b), bestehende Signaturen bleiben unangetastet. Jede Signatur
//! traegt eine Rolle (Autor/Pruefer/ReviewGate der Familie P — REG01-08,
//! PL4 ist dort menschlich review-gebunden, s. catalog.rs); `verify_sig`
//! bleibt fuer Rueckwaertskompatibilitaet unveraendert (eine Signatur),
//! `verify_sig_all` prueft die GANZE Kette, jede Signatur einzeln
//! gruen/rot, keine stille Mehrheitsentscheidung.
//!
//! Schluessel: 32-Byte-Seed (deterministisch, S11 „Schluesselverwaltung
//! ausserhalb des Formats"). Bezug wahlweise aus Datei oder — unter
//! Feature `keyring-os` — aus dem OS-Schluesselbund.

use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use loom_canon::Cv;
use loom_codec::{decode_sealed, seal_canonical, Segment};
use loom_format::{KIND_HEADER, KIND_SEGTAB, KIND_SIGNATURE, SEG_FLAG_NON_CORE};

pub const ROLE_AUTHOR: &str = "author";
pub const ROLE_REVIEWER: &str = "reviewer";
/// Der ReviewGate-Slot der Familie P (Regulated Advisory, REG01-08):
/// PL4 ist dort MENSCHLICH review-gebunden (catalog.rs) — diese Rolle
/// traegt den Signatur-Platz dafuer, ersetzt aber keinen echten
/// menschlichen Review (der Agent kann diese Rolle nicht selbst fuellen).
pub const ROLE_REVIEW_GATE: &str = "review_gate";

#[derive(Debug)]
pub enum SignError {
    Decode(String),
    BadSeed,
    NoSignature,
    BadSignature,
    Verify(String),
}

/// Das Ergebnis EINER Signaturpruefung innerhalb der Registrierungskette.
#[derive(Debug, Clone)]
pub struct SignatureEntry {
    pub role: String,
    pub public_key: VerifyingKey,
    pub valid: bool,
}

fn signing_key(seed: &[u8; 32]) -> SigningKey {
    SigningKey::from_bytes(seed)
}

/// Baut EIN SIGNATURE-Segment (dCBOR) fuer eine benannte Rolle.
fn signature_segment(role: &str, vk: &VerifyingKey, sig: &Signature) -> Segment {
    let payload = Cv::map(vec![
        ("alg", Cv::Text("ed25519".into())),
        ("signed", Cv::Text("core_root".into())),
        ("role", Cv::Text(role.to_string())),
        ("public_key", Cv::Bytes(vk.to_bytes().to_vec())),
        ("signature", Cv::Bytes(sig.to_bytes().to_vec())),
    ])
    .encode()
    .expect("kanonisch");
    Segment {
        kind: KIND_SIGNATURE,
        // non_core ERZWUNGEN: die Signatur ist nicht Teil von core_root.
        seg_flags: SEG_FLAG_NON_CORE,
        payload,
        deps: vec![],
    }
}

/// `loom sign`: fuegt eine WEITERE Signatur (Rolle + Schluessel) ueber
/// core_root hinzu — ADDITIV zur bestehenden Registrierungskette (X1e);
/// core_root selbst bleibt unveraendert (SIGNATURE ist non_core).
pub fn sign(bytes: &[u8], seed: &[u8; 32], role: &str) -> Result<Vec<u8>, SignError> {
    let dec = decode_sealed(bytes).map_err(|e| SignError::Decode(format!("{e:?}")))?;
    let sk = signing_key(seed);
    let sig = sk.sign(&dec.footer.core_root);
    let sig_seg = signature_segment(role, &sk.verifying_key(), &sig);

    // Bestehende (non-Pseudo) Segmente BLEIBEN — inklusive bereits
    // vorhandener SIGNATURE-Segmente anderer Rollen/Signierer.
    let mut segments: Vec<Segment> = dec
        .frames
        .iter()
        .filter(|(e, _)| e.kind != KIND_HEADER && e.kind != KIND_SEGTAB)
        .map(|(e, f)| Segment {
            kind: e.kind,
            seg_flags: e.seg_flags,
            payload: f.payload.clone(),
            deps: e.deps.clone(),
        })
        .collect();
    segments.push(sig_seg);

    let (class, profiles) = manifest_class(&dec);
    let profiles_ref: Vec<&str> = profiles.iter().map(|s| s.as_str()).collect();
    let sealed = seal_canonical(&class, &profiles_ref, &segments)
        .map_err(|e| SignError::Decode(format!("{e:?}")))?;
    Ok(sealed.bytes)
}

/// `loom verify-sig`: prueft EINE eingebettete Signatur gegen core_root
/// (Rueckwaertskompatibel — unveraendert seit P6(c), fuer Container mit
/// genau einer Signatur). Bei mehreren: die erste in Segtab-Reihenfolge
/// (digest-sortiert); fuer die volle Kette s. `verify_sig_all`.
pub fn verify_sig(bytes: &[u8]) -> Result<VerifyingKey, SignError> {
    let dec = decode_sealed(bytes).map_err(|e| SignError::Decode(format!("{e:?}")))?;
    let sig_frame = dec
        .frames
        .iter()
        .find(|(e, _)| e.kind == KIND_SIGNATURE)
        .ok_or(SignError::NoSignature)?;
    let cv = loom_canon::decode(&sig_frame.1.payload)
        .map_err(|e| SignError::Decode(format!("{e:?}")))?;
    let pk = get_bytes(&cv, "public_key").ok_or(SignError::BadSignature)?;
    let sg = get_bytes(&cv, "signature").ok_or(SignError::BadSignature)?;
    let vk = VerifyingKey::from_bytes(&to32(&pk).ok_or(SignError::BadSignature)?)
        .map_err(|e| SignError::Verify(e.to_string()))?;
    let sig = Signature::from_bytes(&to64(&sg).ok_or(SignError::BadSignature)?);
    vk.verify(&dec.footer.core_root, &sig)
        .map_err(|e| SignError::Verify(e.to_string()))?;
    Ok(vk)
}

/// `loom verify-sig --all` (X1e-Exit-Zeuge): prueft JEDE eingebettete
/// Signatur EINZELN gegen core_root — die volle Mehrfachsignatur-Kette,
/// jede fuer sich gruen/rot; eine ungueltige Signatur haelt nicht die
/// anderen zurueck (keine stille Mehrheitsentscheidung, kein
/// gemeinsamer Fruehausstieg).
pub fn verify_sig_all(bytes: &[u8]) -> Result<Vec<SignatureEntry>, SignError> {
    let dec = decode_sealed(bytes).map_err(|e| SignError::Decode(format!("{e:?}")))?;
    let sig_frames: Vec<_> = dec
        .frames
        .iter()
        .filter(|(e, _)| e.kind == KIND_SIGNATURE)
        .collect();
    if sig_frames.is_empty() {
        return Err(SignError::NoSignature);
    }
    let mut out = Vec::new();
    for (_, frame) in sig_frames {
        let cv =
            loom_canon::decode(&frame.payload).map_err(|e| SignError::Decode(format!("{e:?}")))?;
        let role = get_text(&cv, "role").unwrap_or_else(|| "unbekannt".to_string());
        let pk = get_bytes(&cv, "public_key").ok_or(SignError::BadSignature)?;
        let sg = get_bytes(&cv, "signature").ok_or(SignError::BadSignature)?;
        let vk = VerifyingKey::from_bytes(&to32(&pk).ok_or(SignError::BadSignature)?)
            .map_err(|e| SignError::Verify(e.to_string()))?;
        let sig = Signature::from_bytes(&to64(&sg).ok_or(SignError::BadSignature)?);
        let valid = vk.verify(&dec.footer.core_root, &sig).is_ok();
        out.push(SignatureEntry {
            role,
            public_key: vk,
            valid,
        });
    }
    Ok(out)
}

fn manifest_class(dec: &loom_codec::Decoded) -> (String, Vec<String>) {
    for (e, f) in &dec.frames {
        if e.kind == loom_format::KIND_MANIFEST {
            if let Ok(Cv::Map(entries)) = loom_canon::decode(&f.payload) {
                let get = |k: &str| {
                    entries.iter().find_map(|(key, v)| match key {
                        Cv::Text(s) if s == k => Some(v.clone()),
                        _ => None,
                    })
                };
                let class = match get("container_class") {
                    Some(Cv::Text(s)) => s,
                    _ => "inspection".to_string(),
                };
                let profiles = match get("profiles_required") {
                    Some(Cv::Array(a)) => a
                        .iter()
                        .filter_map(|x| match x {
                            Cv::Text(s) => Some(s.clone()),
                            _ => None,
                        })
                        .collect(),
                    _ => vec![class.clone()],
                };
                return (class, profiles);
            }
        }
    }
    ("inspection".to_string(), vec!["inspection".to_string()])
}

fn get_bytes(cv: &Cv, key: &str) -> Option<Vec<u8>> {
    match cv {
        Cv::Map(entries) => entries.iter().find_map(|(k, v)| match (k, v) {
            (Cv::Text(s), Cv::Bytes(b)) if s == key => Some(b.clone()),
            _ => None,
        }),
        _ => None,
    }
}
fn get_text(cv: &Cv, key: &str) -> Option<String> {
    match cv {
        Cv::Map(entries) => entries.iter().find_map(|(k, v)| match (k, v) {
            (Cv::Text(s), Cv::Text(t)) if s == key => Some(t.clone()),
            _ => None,
        }),
        _ => None,
    }
}
fn to32(v: &[u8]) -> Option<[u8; 32]> {
    v.try_into().ok()
}
fn to64(v: &[u8]) -> Option<[u8; 64]> {
    v.try_into().ok()
}

/// Seed aus dem OS-Schluesselbund (nur Feature `keyring-os`).
#[cfg(feature = "keyring-os")]
pub fn seed_from_keyring(service: &str, user: &str) -> Result<[u8; 32], String> {
    let entry = keyring::Entry::new(service, user).map_err(|e| e.to_string())?;
    let secret = entry.get_password().map_err(|e| e.to_string())?;
    let bytes = hex_to_bytes(&secret).ok_or("Seed nicht hex-32")?;
    to32(&bytes).ok_or("Seed nicht 32 Byte".to_string())
}

#[cfg(feature = "keyring-os")]
pub fn seed_to_keyring(service: &str, user: &str, seed: &[u8; 32]) -> Result<(), String> {
    let entry = keyring::Entry::new(service, user).map_err(|e| e.to_string())?;
    entry
        .set_password(&seed.iter().map(|b| format!("{b:02x}")).collect::<String>())
        .map_err(|e| e.to_string())
}

#[cfg(feature = "keyring-os")]
fn hex_to_bytes(s: &str) -> Option<Vec<u8>> {
    if !s.len().is_multiple_of(2) {
        return None;
    }
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use loom_canon::Cv;
    use loom_codec::seal_canonical;

    fn minimal() -> Vec<u8> {
        let m = Segment::canonical(
            loom_format::KIND_MANIFEST,
            &Cv::map(vec![
                ("title", Cv::Text("sig-test".into())),
                ("container_class", Cv::Text("inspection".into())),
            ]),
        )
        .unwrap();
        let canon = Segment {
            kind: loom_format::KIND_CANON_DESC,
            seg_flags: 0,
            payload: Cv::Text(loom_canon::CANON_RULES_TEXT.into())
                .encode()
                .unwrap(),
            deps: vec![],
        };
        seal_canonical("inspection", &["inspection"], &[m, canon])
            .unwrap()
            .bytes
    }

    #[test]
    fn sign_then_verify_roundtrip() {
        let seed = [7u8; 32];
        let signed = sign(&minimal(), &seed, ROLE_AUTHOR).unwrap();
        // core_root unveraendert (SIGNATURE ist non_core).
        let before = decode_sealed(&minimal()).unwrap().footer.core_root;
        let after = decode_sealed(&signed).unwrap().footer.core_root;
        assert_eq!(
            before, after,
            "core_root darf sich durch Signatur nicht aendern"
        );
        // Verifikation gruen; oeffentlicher Schluessel = der des Seeds.
        let vk = verify_sig(&signed).unwrap();
        assert_eq!(
            vk.to_bytes(),
            SigningKey::from_bytes(&seed).verifying_key().to_bytes()
        );
    }

    #[test]
    fn tamper_breaks_signature() {
        let signed = sign(&minimal(), &[7u8; 32], ROLE_AUTHOR).unwrap();
        // Ein core-relevantes Byte kippen ⇒ core_root ≠ signierter Wert.
        let mut tampered = signed.clone();
        // Praeambel-Version kippen macht die Datei ungueltig; stattdessen
        // ein Payload-Byte im ersten Frame — Digest/Tabelle bricht zuerst.
        let n = tampered.len();
        tampered[n - loom_format::FOOTER_LEN + 20] ^= 0xff; // core_root im Footer
        assert!(verify_sig(&tampered).is_err());
    }

    #[test]
    fn missing_signature_is_error() {
        assert!(matches!(
            verify_sig(&minimal()),
            Err(SignError::NoSignature)
        ));
    }

    // ---------- X1(e): Signatur-Registry-Vollform (Mehrfachsignaturen) ----------

    #[test]
    fn multiple_signatures_accumulate_additively() {
        let base = minimal();
        let author_seed = [1u8; 32];
        let reviewer_seed = [2u8; 32];
        let gate_seed = [3u8; 32];

        let s1 = sign(&base, &author_seed, ROLE_AUTHOR).unwrap();
        let s2 = sign(&s1, &reviewer_seed, ROLE_REVIEWER).unwrap();
        let s3 = sign(&s2, &gate_seed, ROLE_REVIEW_GATE).unwrap();

        // core_root bleibt durch alle drei Signaturvorgaenge unveraendert.
        let root0 = decode_sealed(&base).unwrap().footer.core_root;
        let root3 = decode_sealed(&s3).unwrap().footer.core_root;
        assert_eq!(root0, root3);

        let entries = verify_sig_all(&s3).unwrap();
        assert_eq!(
            entries.len(),
            3,
            "alle drei Signaturen muessen erhalten bleiben"
        );
        let mut roles: Vec<&str> = entries.iter().map(|e| e.role.as_str()).collect();
        roles.sort();
        let mut expected = vec![ROLE_AUTHOR, ROLE_REVIEWER, ROLE_REVIEW_GATE];
        expected.sort();
        assert_eq!(roles, expected);
        assert!(
            entries.iter().all(|e| e.valid),
            "alle drei muessen gruen sein"
        );
    }

    #[test]
    fn verify_sig_all_reports_each_signature_independently_green_or_red() {
        let base = minimal();
        let good_seed = [4u8; 32];
        let s1 = sign(&base, &good_seed, ROLE_AUTHOR).unwrap();
        let s2 = sign(&s1, &[5u8; 32], ROLE_REVIEWER).unwrap();

        // Eine der beiden Signaturen manipulieren (Signatur-Bytes selbst,
        // nicht core_root) — die ANDERE muss trotzdem gruen bleiben (keine
        // stille Mehrheitsentscheidung, kein gemeinsamer Fruehausstieg).
        let dec = decode_sealed(&s2).unwrap();
        let mut segments: Vec<Segment> = Vec::new();
        let mut tampered_one = false;
        for (e, f) in &dec.frames {
            if e.kind == loom_format::KIND_HEADER || e.kind == loom_format::KIND_SEGTAB {
                continue;
            }
            if e.kind == KIND_SIGNATURE && !tampered_one {
                let cv = loom_canon::decode(&f.payload).unwrap();
                let role = get_text(&cv, "role").unwrap();
                let pk = get_bytes(&cv, "public_key").unwrap();
                let mut sg = get_bytes(&cv, "signature").unwrap();
                sg[0] ^= 0xff; // Signatur-Bytes kaputt machen.
                let bad_payload = Cv::map(vec![
                    ("alg", Cv::Text("ed25519".into())),
                    ("signed", Cv::Text("core_root".into())),
                    ("role", Cv::Text(role)),
                    ("public_key", Cv::Bytes(pk)),
                    ("signature", Cv::Bytes(sg)),
                ])
                .encode()
                .unwrap();
                segments.push(Segment {
                    kind: KIND_SIGNATURE,
                    seg_flags: e.seg_flags,
                    payload: bad_payload,
                    deps: e.deps.clone(),
                });
                tampered_one = true;
            } else {
                segments.push(Segment {
                    kind: e.kind,
                    seg_flags: e.seg_flags,
                    payload: f.payload.clone(),
                    deps: e.deps.clone(),
                });
            }
        }
        let (class, profiles) = manifest_class(&dec);
        let profiles_ref: Vec<&str> = profiles.iter().map(|s| s.as_str()).collect();
        let resealed = seal_canonical(&class, &profiles_ref, &segments).unwrap();

        let entries = verify_sig_all(&resealed.bytes).unwrap();
        assert_eq!(entries.len(), 2);
        let good_count = entries.iter().filter(|e| e.valid).count();
        let bad_count = entries.iter().filter(|e| !e.valid).count();
        assert_eq!(good_count, 1, "eine Signatur bleibt gruen");
        assert_eq!(
            bad_count, 1,
            "die manipulierte Signatur ist rot, nicht versteckt"
        );
    }

    #[test]
    fn resigning_with_same_key_and_role_does_not_duplicate() {
        let base = minimal();
        let seed = [9u8; 32];
        let once = sign(&base, &seed, ROLE_AUTHOR).unwrap();
        let twice = sign(&once, &seed, ROLE_AUTHOR).unwrap();
        // Identischer Signatur-Inhalt (gleicher Schluessel+Rolle+core_root)
        // dedupliziert ueber (kind, digest) — kein Blaehen der Kette.
        assert_eq!(verify_sig_all(&twice).unwrap().len(), 1);
    }
}
