//! P6(c) — Ed25519-Signatur ueber `core_root` ins SIGNATURE-Segment
//! (Kind 0x0050, non_core: kann nicht Teil des Signierten sein, LOOM
//! Teil 9.5). Krypto liegt AUSSCHLIESSLICH hier im CLI-Blatt.
//!
//! Schluessel: 32-Byte-Seed (deterministisch, S11 „Schluesselverwaltung
//! ausserhalb des Formats"). Bezug wahlweise aus Datei oder — unter
//! Feature `keyring-os` — aus dem OS-Schluesselbund.

use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use loom_canon::Cv;
use loom_codec::{decode_sealed, seal_canonical, Segment};
use loom_format::{KIND_HEADER, KIND_SEGTAB, KIND_SIGNATURE, SEG_FLAG_NON_CORE};

#[derive(Debug)]
pub enum SignError {
    Decode(String),
    BadSeed,
    NoSignature,
    BadSignature,
    Verify(String),
}

fn signing_key(seed: &[u8; 32]) -> SigningKey {
    SigningKey::from_bytes(seed)
}

/// Baut das SIGNATURE-Segment (dCBOR) ueber den core_root.
fn signature_segment(vk: &VerifyingKey, sig: &Signature) -> Segment {
    let payload = Cv::map(vec![
        ("alg", Cv::Text("ed25519".into())),
        ("signed", Cv::Text("core_root".into())),
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

/// `loom sign`: signiert core_root und legt das SIGNATURE-Segment bei;
/// core_root bleibt unveraendert (non_core geht nicht in die Wurzel).
pub fn sign(bytes: &[u8], seed: &[u8; 32]) -> Result<Vec<u8>, SignError> {
    let dec = decode_sealed(bytes).map_err(|e| SignError::Decode(format!("{e:?}")))?;
    let sk = signing_key(seed);
    let sig = sk.sign(&dec.footer.core_root);
    let sig_seg = signature_segment(&sk.verifying_key(), &sig);

    // Bestehende (non-Pseudo, non-SEGTAB) Segmente + neues SIGNATURE-Segment.
    let mut segments: Vec<Segment> = dec
        .frames
        .iter()
        .filter(|(e, _)| e.kind != KIND_HEADER && e.kind != KIND_SEGTAB && e.kind != KIND_SIGNATURE)
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

/// `loom verify-sig`: prueft die eingebettete Signatur gegen core_root.
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
        let signed = sign(&minimal(), &seed).unwrap();
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
        let signed = sign(&minimal(), &[7u8; 32]).unwrap();
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
}
