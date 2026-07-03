//! hashprofile — blake3-Zweitprofil (X1c, Oekosystem-Karte §2/E1).
//! Ein zusaetzlicher, unabhaengig nachrechenbarer Digest ueber dieselben
//! kanonischen Bytes — additiv, nicht ersetzend: die Frame-/Segtab-
//! Digests bleiben sha2-256 (loom-format, hand-gefuehrt, core-only).
//! blake3 lebt bewusst NUR hier im CLI-Blatt (dieselbe Disziplin wie
//! Ed25519/zstd) und wird im MANIFEST als verfuegbares Profil deklariert
//! (s. `declare_hash_profiles`), nicht stillschweigend mitgefuehrt.

pub const PROFILE_SHA256: &str = "sha2-256";
pub const PROFILE_BLAKE3: &str = "blake3";

/// blake3-Digest als Hex — das Zweitprofil.
pub fn blake3_hex(bytes: &[u8]) -> String {
    blake3::hash(bytes).to_hex().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Bekannte BLAKE3-Digests (leere Eingabe + [0,1,2]) — mit der
    /// offiziellen `blake3`-Kiste selbst nachgerechnet und hier als
    /// Regressions-Anker fixiert; beweist eine echte, korrekte
    /// Implementierung, keine Attrappe.
    #[test]
    fn matches_known_blake3_digests() {
        assert_eq!(
            blake3_hex(b""),
            "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262"
        );
        assert_eq!(
            blake3_hex(&[0, 1, 2]),
            "e1be4d7a8ab5560aa4199eea339849ba8e293d55ca0a81006726d184519e647f"
        );
    }

    #[test]
    fn deterministic_and_context_dependent() {
        let a = blake3_hex(b"Kristall");
        let b = blake3_hex(b"Kristall");
        let c = blake3_hex(b"anderer Inhalt");
        assert_eq!(a, b);
        assert_ne!(a, c);
    }
}
