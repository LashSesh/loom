# CDDL-Schemata je Segment-Kind (LC-R3)

Mechanisch aus den normativen Feldverträgen in `loom-verify` (L1/L2)
und `loom-codec` (SEGTAB/Header) extrahiert — die Rust-Prüfungen
bleiben die Autorität; diese Dateien sind die maschinenlesbare
Zweitform (RFC 8610). Grundregeln aus LOOM-CANON-1 gelten überall:
definite lengths, kürzeste Int-Kodierung, Map-Keys tstr/uint bytewise
sortiert, KEINE Floats (Decimal-Fraction = Tag 4 mit Integer-Mantisse),
Tag-Whitelist {0, 4}, Text NFC.

Dateien: header, segtab, manifest, canon-desc, gate-reports, residue,
csa-nsb, ledger, replay-manifest, provider-manifest (0x0060),
inference-profile (0x0061), inference-trace (0x0062),
candidate-outputs (0x0063), tool-profile (0x0064), common.
