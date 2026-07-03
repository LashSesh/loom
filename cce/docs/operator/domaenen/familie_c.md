# Familie C — Daten/Analytics (DATA01–12)

Über dem geteilten Kern (family_a): 11 Relation-Regeln + 1 Kette
(DATA03 ETL-Lineage lückenlos). Kennzahlen ordnen, Gates entscheiden
(V1 geerbt) — auch für KPI-/Statistik-Domänen. Alle PL3.

| Domäne | Zweck | Kern-Regel | Kern-Residuum | PL |
|---|---|---|---|---|
| DATA01 | Datenbereinigung | Null-Regel je Feld | unhandled_null | PL3 |
| DATA02 | SQL-Query | Join-Bedingung je Klausel | cartesian_blowup | PL3 |
| DATA03 | ETL-Pipeline | Lineage-Kette lückenlos | broken_lineage | PL3 |
| DATA04 | Dashboard-Spec | Metrik je Widget | undefined_metric | PL3 |
| DATA05 | Datenmodell | Beziehung je Entität | orphan_entity | PL3 |
| DATA06 | Statistische Analyse | Annahmenprüfung je Test | violated_assumption | PL3 |
| DATA07 | Feature-Engineering | saubere Quelle je Feature | data_leakage | PL3 |
| DATA08 | Datenqualitäts-Regelwerk | Feldprüfung je Regel | unchecked_field | PL3 |
| DATA09 | Visualisierung | Datenbindung je Kanal | misleading_scale | PL3 |
| DATA10 | KPI-Definition | Formel je KPI | ambiguous_kpi | PL3 |
| DATA11 | Datenkatalog | Herkunft je Asset | undocumented_source | PL3 |
| DATA12 | Sampling/Kohorte | scharfe Zugehörigkeit | selection_bias | PL3 |

Beweis-Ort: `conformance/tests/family_c_catalog.rs`.
