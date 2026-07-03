Welle: W5 — Familie F (Projekt/Prozess/Workflow, PM01–15)
Eingang erfüllt: ja (W4 grün, E/P8 grün, CI grün)
Gebaut: 15 Domänen `family_f_domains.rs` über dem family_a-Kern (12
Relation + 1 Azyklik PM01 + 2 Ketten PM02/PM13). Neu im Kern: teilbarer
`acyclic_domain!`-Builder + acyclic_reference/negatives (für alle
Azyklik-Domänen wiederverwendbar). Zeugen
`conformance/tests/family_f_catalog.rs` (Parität 11/11 · Referenz
schließt · 30 Negative rot mit erwartetem Residuum · Kerntest ≃).
Katalog PM01–15 PL1→PL3; WITNESSED_DOMAINS=69; overclaim leer. Doku
familie_f.md.
Ausgangs-Gate W5: alle vier Familien-Zeugen grün; GUARD += "W5"; CI GRUEN.
Interleaving-Wahl (§3): nach W5 folgt **F** (zstd-Transportprofil) gemäß
Wellenplan.
Residuen: PL4 für PM01–15 offen.
Abweichungen: keine; spec/ unangetastet.
