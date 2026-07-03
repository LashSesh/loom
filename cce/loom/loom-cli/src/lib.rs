//! loom-cli-Bibliothek: Signatur- (P6(c)) und Transport-/Hashprofil-
//! Operationen (X1c) als testbare Einheiten. Externe Kisten (Ed25519,
//! zstd, blake3) leben AUSSCHLIESSLICH in diesem CLI-Blatt.
pub mod hashprofile;
pub mod sign;
pub mod transport;
