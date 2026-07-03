# Familie K — Bildung/Training (EDU01–12)

Über dem geteilten Kern (family_a): 10 Relation-Regeln + 2 Ketten
(EDU04 Schwierigkeitsfolge, EDU08 Tutorial-Schritte). Alle PL3;
PL4 verlangt Nutzungs-Evidenz.

| Domäne | Zweck | Kern-Regel | Kern-Residuum | PL |
|---|---|---|---|---|
| EDU01 | Lehrplan/Curriculum | Voraussetzung je Lernziel | prerequisite_gap | PL3 |
| EDU02 | Lektion | Aktivität je Ziel | goalless_activity | PL3 |
| EDU03 | Quiz/Assessment | Lernzielbezug je Frage | unaligned_question | PL3 |
| EDU04 | Übungsaufgaben-Set | Schwierigkeitsfolge lückenlos | difficulty_jump | PL3 |
| EDU05 | Lernpfad | Abhängigkeit je Modul | orphan_module | PL3 |
| EDU06 | Rubrik | Niveau je Kriterium | ambiguous_level | PL3 |
| EDU07 | Kurs-Syllabus | Ziel je Einheit | uncovered_objective | PL3 |
| EDU08 | Tutorial | Schrittkette lückenlos | gap_in_steps | PL3 |
| EDU09 | Fallstudie | Lernzielbezug je Element | pointless_detail | PL3 |
| EDU10 | Kompetenzmodell | beobachtbare Ableitung | unobservable_competency | PL3 |
| EDU11 | Flashcard-Set | Konzeptbezug je Karte | ambiguous_card | PL3 |
| EDU12 | Schulungshandbuch | Ziel je Modul | uncovered_topic | PL3 |

Beweis-Ort: `conformance/tests/family_k_catalog.rs`.
