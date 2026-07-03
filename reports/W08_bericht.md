Welle: W8 — Familie B (Software Engineering, SWE01–15)
Eingang erfüllt: ja (W7 grün, CI grün)
Gebaut: 15 Domänen `family_b_domains.rs` über dem family_a-Kern (12
Relation + 1 Azyklik SWE07 + 1 Kette SWE05 Pipeline-fail-closed).
Die im Wellenplan erwartete Sonderstellung der SWE-Familie (AST-artige
Rückgewinnung) bildet sich auf der geprüften Ebene vollständig als
Naht-Struktur ab: Aufruf-/Vertrags-/Abhängigkeits-Beziehungen sind
Relations-/Azyklik-/Ketten-Regeln; `equivalent` bleibt Klassenvergleich
(Formatierung normalisiert der Kern). Ein Voll-AST-Parser je Sprache
wäre PL4-Reifepfad, kein Strukturbeweis — Residuum vermerkt.
Zeugen `conformance/tests/family_b_catalog.rs` (Parität 11/11 · Referenz
schließt · 30 Negative rot · Kerntest ≃). Katalog SWE01–15 PL1→PL3;
WITNESSED_DOMAINS=111; overclaim leer. Doku familie_b.md.
Ausgangs-Gate W8: alle vier Familien-Zeugen grün; GUARD += "W8"; CI GRUEN.
Residuen: PL4 für SWE01–15 offen; sprachspezifische AST-Reanalyse =
Reifepfad (PL4), nicht Teil des Strukturbeweises.
Abweichungen: keine; spec/ unangetastet.
