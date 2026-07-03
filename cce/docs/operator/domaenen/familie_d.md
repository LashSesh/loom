# Familie D — Graph/Netzwerk/Struktur (GRA01–12)

Über dem geteilten Kern (family_a) — hier ist die Naht-Grammatik die
Domäne selbst: 8 Relation, 2 Azyklik (GRA02 is-a, GRA04 Dependencies),
2 Ketten (GRA03 FSM-Erreichbarkeit, GRA06 Workflow-Fluss). Alle PL3.

| Domäne | Zweck | Kern-Regel | Kern-Residuum | PL |
|---|---|---|---|---|
| GRA01 | Graph-Konstruktion | auflösbare Kante je Knoten | dangling_edge | PL3 |
| GRA02 | Ontologie/Taxonomie | is-a azyklisch | cycle_in_hierarchy | PL3 |
| GRA03 | Zustandsautomat | Übergangskette vom Start | unreachable_state | PL3 |
| GRA04 | Abhängigkeitsgraph | Dependencies azyklisch | dependency_cycle | PL3 |
| GRA05 | ER-Modell | Kardinalität je Relation | undefined_cardinality | PL3 |
| GRA06 | Workflow/DAG | Fluss lückenlos | orphan_step | PL3 |
| GRA07 | Mindmap/Konzeptnetz | Assoziation je Konzept | isolated_node | PL3 |
| GRA08 | Netzwerk-Topologie | Redundanz je Knoten | single_point_of_failure | PL3 |
| GRA09 | Baum/Hierarchie | genau eine Eltern-Naht | multiple_parents | PL3 |
| GRA10 | Petri-Netz | Feuerbarkeit je Transition | deadlock | PL3 |
| GRA11 | Kausaldiagramm | Messbezug je Faktor | unmeasured_confounder | PL3 |
| GRA12 | Routing/Pfad | gewichtete Kante je Knoten | unreachable_target | PL3 |

Beweis-Ort: `conformance/tests/family_d_catalog.rs`.
