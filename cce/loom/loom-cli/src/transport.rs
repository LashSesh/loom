//! transport — zstd-Transportprofil (X1c, Oekosystem-Karte §2/E1).
//! Reine Ganzdatei-Kompression FUER DEN TRANSPORT: die kanonisch-
//! gespeicherte (canonical-stored) Form bleibt die Golden-Referenz
//! unveraendert — zstd verpackt/entpackt ausserhalb des Frame-Formats
//! (kein SEG_FLAG_COMPRESSED-Pfad, keine Aenderung an loom-format/
//! loom-codec). Entpacken liefert byte-identisch die Eingabe zurueck;
//! der Klassen-Digest ist damit trivial unberuehrt.

#[derive(Debug)]
pub enum TransportError {
    Decompress(String),
}

/// Verpackt beliebige (typischerweise: versiegelte .loom-)Bytes fuer
/// den Transport.
pub fn compress(bytes: &[u8]) -> Vec<u8> {
    zstd::encode_all(bytes, 19).expect("zstd-Kompression (In-Memory-Puffer kann nicht scheitern)")
}

/// Entpackt — MUSS byte-identisch die Original-Eingabe von `compress`
/// liefern (Golden-Referenz bleibt unveraendert, s. X1c-Zeuge).
pub fn decompress(bytes: &[u8]) -> Result<Vec<u8>, TransportError> {
    zstd::decode_all(bytes).map_err(|e| TransportError::Decompress(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_is_byte_identical() {
        let original =
            b"beliebige Bytes, hier stellvertretend fuer eine versiegelte .loom-Datei".repeat(50);
        let packed = compress(&original);
        // Echte Kompression, kein Passthrough: bei so repetitivem Inhalt
        // MUSS das Ergebnis kleiner sein.
        assert!(packed.len() < original.len());
        let unpacked = decompress(&packed).expect("entpacken");
        assert_eq!(unpacked, original);
    }

    #[test]
    fn decompress_rejects_malformed_input() {
        assert!(decompress(b"kein zstd-Rahmen").is_err());
    }
}
