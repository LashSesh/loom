# Familie O — Finanzen/Verwaltung/Planung (FIN01–10)

Über dem geteilten Kern (family_a): 8 Relation + 1 Azyklik (FIN04
Formeln zirkelfrei) + 1 Kette (FIN08 Liquiditäts-Zeitachse lückenlos).
Geprüft wird Struktur (Zuordnung/Summenbezug/Zirkelfreiheit), keine
Rechen-Engine behauptet; Zahlen ordnen, Gates entscheiden. Alle PL3.

| Domäne | Zweck | Kern-Regel | Kern-Residuum | PL |
|---|---|---|---|---|
| FIN01 | Budget-Plan | Summenbezug je Position | unbalanced_budget | PL3 |
| FIN02 | Kostenaufstellung | Zuordnung je Posten | unassigned_cost | PL3 |
| FIN03 | Rechnungs-Struktur | Summenbezug je Position | miscalculated_total | PL3 |
| FIN04 | Finanzmodell | Formeln zirkelfrei | circular_reference | PL3 |
| FIN05 | Forecast | Ableitung je Treiber | unfounded_forecast | PL3 |
| FIN06 | Ausgaben-Report | Kategorie je Ausgabe | uncategorized_expense | PL3 |
| FIN07 | Investitionsrechnung | Diskontbezug je Cashflow | wrong_discounting | PL3 |
| FIN08 | Liquiditätsplan | Zeitkette lückenlos | liquidity_gap | PL3 |
| FIN09 | Kostenstellen-Plan | Zuordnung je Kostenstelle | orphan_cost_center | PL3 |
| FIN10 | Reconciliation | Abgleich je Buchung | unreconciled_item | PL3 |

Beweis-Ort: `conformance/tests/family_o_catalog.rs`.
