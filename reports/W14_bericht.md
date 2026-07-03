Welle: W14 — Familie H (Security/Infra/Ops, OPS01–15)
Eingang erfüllt: ja (Vorwelle grün, CI grün)
Gebaut: 15 Domänen `crates/cce-materialize/src/30:family_h_domains.rs` über
dem family_a-Kern (12 Relation + 1 Azyklik (OPS09) + 2 Ketten (OPS03/OPS15); Safety-by-abstraction (S13-A3)); kein neuer Motor-Code. Zeugen
`conformance/tests/30:family_h_catalog.rs` (Parität 11/11 · Referenz schließt ·
 Struktur geprueft, nie Angriffs-Ausfuehrung Negative rot · Kerntest ≃). Katalog auf PL3; overclaim leer. Doku
`docs/operator/domaenen/30:family_h.md`.
Ausgangs-Gate W14: alle vier Familien-Zeugen grün; GUARD += "W14"; CI GRUEN.
Residuen: PL4 offen (Nutzungs-Evidenz).
Abweichungen: keine; spec/ unangetastet.
