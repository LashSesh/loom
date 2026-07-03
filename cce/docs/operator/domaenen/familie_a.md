# Familie A — Dokument/Text (D01–D15)

Alle Domänen der Familie teilen einen Kern (family_a): Einheit =
semantische Text-Einheit, Naht = Beziehung zwischen Einheiten;
`materialize` → Markdown (verlustfrei rückgewinnbar), `reanalyze` liest
Struktur+Einheiten zurück, `equivalent` = gleiche Einheiten/Naht-Graph.
Jede Domäne trägt ihr Produkt-Level (PL) sichtbar; nur D01 ist PL4
(Produkt-Kerntest), D02–D15 sind PL3 (Adapter 11/11 · Zeugen ·
Kerntest · diese Doku-Zeile). Was eine Domäne prüft, prüft sie
fail-closed; das genannte Kern-Residuum wird sichtbar, wenn die
Kern-Naht-Regel bricht.

| Domäne | Zweck | Kern-Naht-Regel | Kern-Residuum (sichtbar bei Bruch) | PL |
|---|---|---|---|---|
| D01 | Bericht/Memo | jede Aussage/jedes Risiko gestützt | unsupported_unit / unabgedeckt | PL4 |
| D02 | Vertrag | jede Klausel hat auflösbaren Verweis | dangling_clause | PL3 |
| D03 | Spezifikation | jede Anforderung ist ableitbar/testbar | untestable_req | PL3 |
| D04 | Handbuch | Schritte lückenlos geordnet | gap_in_procedure | PL3 |
| D05 | Brief | Anrede und Schluss vorhanden | missing_salutation | PL3 |
| D06 | Angebot | jede Position mit Preisbezug | orphan_line_item | PL3 |
| D07 | Richtlinie | jede Regel mit Geltungsbereich | unscoped_rule | PL3 |
| D08 | Protokoll | jeder Punkt mit Beschluss | actionless_item | PL3 |
| D09 | Zusammenfassung | jede Kernaussage quellenbelegt | invented_semantic | PL3 |
| D10 | Pressemitteilung | jede Botschaft faktenbelegt | unsupported_claim | PL3 |
| D11 | Lebenslauf | Stationen chronologisch lückenlos | timeline_gap | PL3 |
| D12 | Whitepaper | jede These mit Argumentkette | broken_argument | PL3 |
| D13 | FAQ | jede Frage beantwortet | unanswered_question | PL3 |
| D14 | Checkliste | jeder Prüfpunkt eindeutig | ambiguous_item | PL3 |
| D15 | Glossar | jeder Begriff definiert, ohne Zirkel | undefined_term | PL3 |

**PL3 → PL4** verlangt echte Nutzungs-Evidenz (bei
professionsgebundenen Domänen zusätzlich das ProfessionalReviewGate) —
das kann Agentenarbeit nicht ersetzen; es bleibt sichtbar offen.
Beweis-Ort der PL3-Hebung: `conformance/tests/family_a_catalog.rs`
(Parität 11/11, Referenz schließt, Negative rot mit erwartetem
Residuum, Domänen-Kerntest ≃).
