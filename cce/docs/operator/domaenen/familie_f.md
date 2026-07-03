# Familie F — Projekt/Prozess/Workflow (PM01–15)

Über dem geteilten Kern (family_a): 12 Relation-Regeln, 1 Azyklik
(PM01 Abhängigkeitsgraph zyklenfrei), 2 Ketten (PM02 Prozessfluss bis
End-Ereignis, PM13 Eskalationspfad lückenlos). Alle PL3; PL4 verlangt
Nutzungs-Evidenz.

| Domäne | Zweck | Kern-Regel | Kern-Residuum | PL |
|---|---|---|---|---|
| PM01 | Projektplan/Gantt | Abhängigkeiten azyklisch | dependency_cycle | PL3 |
| PM02 | Prozess-Definition | Fluss bis End-Ereignis | no_end_event | PL3 |
| PM03 | Risikoregister | Maßnahme je Risiko | unmitigated_risk | PL3 |
| PM04 | Meilenstein-Plan | Abhängigkeit je Meilenstein | orphan_milestone | PL3 |
| PM05 | Sprint-Backlog | Schätzung je Story | unestimated_story | PL3 |
| PM06 | RACI-Matrix | Verantwortlicher je Aufgabe | missing_accountable | PL3 |
| PM07 | Ressourcenplan | Zuweisung je Ressource | overallocation | PL3 |
| PM08 | Statusbericht | Beleg je Kennzahl | unbacked_status | PL3 |
| PM09 | Retrospektive | Aktion je Befund | actionless_finding | PL3 |
| PM10 | Entscheidungsvorlage | Kriterium je Option | undecided_criterion | PL3 |
| PM11 | Change-Request | Impact je Änderung | unassessed_impact | PL3 |
| PM12 | Abnahmekriterien/DoD | Prüfung je Kriterium | untestable_criterion | PL3 |
| PM13 | Eskalationspfad | Übergabekette lückenlos | dead_end_escalation | PL3 |
| PM14 | Kapazitätsplan | Bedarf je Einheit | capacity_gap | PL3 |
| PM15 | Roadmap | Abhängigkeit je Initiative | orphan_initiative | PL3 |

Beweis-Ort: `conformance/tests/family_f_catalog.rs`.
