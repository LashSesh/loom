Track-Einheit: F/P6(c) — Härtung: Ed25519-Signatur + OS-Keyring (nach W3)
Eingang: W3 grün, CI grün.
Gebaut (ausschließlich im CLI-Blatt loom-cli — kein Kern-Crate erhält
neue Abhängigkeiten):
- `loom/loom-cli/src/sign.rs`: Ed25519-Signatur über `core_root` ins
  SIGNATURE-Segment (Kind 0x0050, `non_core` ERZWUNGEN — die Signatur ist
  nicht Teil des Signierten, LOOM Teil 9.5). `sign()` legt das Segment bei
  und re-sealt; core_root bleibt UNVERÄNDERT (non_core geht nicht in die
  Merkle-Wurzel). `verify_sig()` prüft die eingebettete Signatur gegen
  core_root. Schlüssel = 32-Byte-Seed (deterministisch, S11
  „Schlüsselverwaltung außerhalb des Formats").
- CLI-Subcommands: `loom keygen <seed-hex> <keyfile>` · `loom sign
  <file.loom> <keyfile> <out.loom>` · `loom verify-sig <file.loom>`.
- OS-Schlüsselbund NUR unter opt-in-Feature `keyring-os`
  (`seed_from_keyring`/`seed_to_keyring`): headless-CI hat keinen Backend,
  daher standardmäßig aus; der seed-/dateibasierte Pfad ist Default.
- ed25519-dalek + keyring (optional) NUR in loom-cli (Blatt).
Ausgangs-Gate:
- Zeugen (loom-cli): sign_then_verify_roundtrip (core_root unverändert,
  public_key = Seed-Ableitung), tamper_breaks_signature (gekippter
  core_root ⇒ Verifikation rot), missing_signature_is_error = grün.
- Live-Demo über golden/r1.loom: keygen → sign → verify-sig „signatur
  gueltig"; die signierte Datei bleibt L0–L2 `verdikt: Valid` (SIGNATURE
  non_core, Format Teil 9.5 erfüllt).
- INV-11/Schichtung: loom-cli ist Blatt, kein Kern-Crate erhält Krypto;
  check_acyclic grün. clippy auch mit `keyring-os` sauber. CI GRUEN.
Residuen:
- OS-Keyring-Integration ist implementiert, aber in dieser headless-
  Umgebung nicht ausführbar (kein secret-service/dbus-Backend) — der
  Feature-Pfad ist code-vollständig und clippy-clean; ein Live-Nachweis
  auf dem Keyring braucht einen Desktop-Host (Track G-nah, gemeldet).
- Signatur-Registry-Vollform (Mehrfachsignaturen, ProfessionalReviewGate-
  Freigaben der Familie P) bleibt LC-R1-Folgeschritt; hier: Einzel-
  signatur über core_root.
Abweichungen: keine; spec/ unangetastet; SIGNATURE bleibt non_core.
