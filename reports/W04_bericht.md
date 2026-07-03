Welle: W4 — Familie N (Kommunikation/CRM/Organisation, COM01–12)
Eingang erfüllt: ja (W3 grün, F/P6(c) grün, CI grün)
Gebaut: 12 Domänen `family_n_domains.rs` über dem family_a-Kern (10
Relation-Regeln + 2 Ketten COM05/COM09). Neu im Kern: teilbarer
`chained_domain!`-Builder + chain_reference/chain_negatives-Helfer (für
alle künftigen Ketten-Domänen wiederverwendbar) — sonst kein neuer
Motor-Code. Zeugen `conformance/tests/family_n_catalog.rs` (Parität
11/11 · Referenz schließt · 24 Negative rot mit erwartetem Residuum ·
Kerntest ≃). Katalog COM01–12 PL1→PL3; WITNESSED_DOMAINS=54; overclaim
leer. Doku familie_n.md.
Ausgangs-Gate W4: alle vier Familien-Zeugen grün; GUARD += "W4"; CI GRUEN.
Interleaving-Wahl (§3): nach W4 folgt **E/P8** (SCALE-2 „Dokumentenmappe",
Red(2)-Kerntest, MSC 1→2) gemäß Wellenplan.
Residuen: PL4 für COM01–12 offen.
Abweichungen: keine; spec/ unangetastet.
