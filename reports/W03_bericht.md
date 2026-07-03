Welle: W3 — Familie G (Governance/Compliance/Audit, GOV01–12)
Eingang erfüllt: ja (W2 grün, C/P4 grün, CI grün)
Gebaut: 12 Domänen `crates/cce-materialize/src/family_g_domains.rs` als
reine Profil-Daten über dem family_a-Kern (alle Relation-Regeln:
Subjekt·Nachweis-/Beleg-/Kontroll-Naht → Kern-Residuum aus S1 K.4).
Kein neuer Motor-Code. GOV05 „kein Score-Gate" ist über das geerbte
no_score-Gate erfüllt. Zeugen `conformance/tests/family_g_catalog.rs`
(Parität 11/11 · Referenz schließt · 24 Negative rot mit erwartetem
Residuum · Domänen-Kerntest ≃). Katalog GOV01–12 PL1→PL3;
WITNESSED_DOMAINS=42; overclaim leer. Doku familie_g.md.
Ausgangs-Gate W3: alle vier Familien-Zeugen grün; GUARD += "W3"; CI GRUEN.
Interleaving-Wahl (§3): nach W3 folgt **F/P6(c)** (Ed25519 + OS-Keyring,
nur CLI-Blatt) gemäß Wellenplan.
Residuen: PL4 für GOV01–12 offen (Nutzungs-/Review-Evidenz).
Abweichungen: keine; spec/ unangetastet.
