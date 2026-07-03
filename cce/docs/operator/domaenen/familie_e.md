# Familie E — Mathematik/Formale Struktur (MATH01–15)

Über dem geteilten Kern (family_a). **Claim-Schranke:** geprüft wird
die STRUKTUR (Ableit-/Kopplungs-/Prämissen-Nähte vorhanden, azyklisch,
lückenlos) — nie die mathematische Wahrheit eines Inhalts. Ein grünes
Gate heißt „strukturell wohlgeformt", nicht „bewiesen". Alle PL3.

| Domäne | Zweck | Kern-Regel | Kern-Residuum | PL |
|---|---|---|---|---|
| MATH01 | Beweis | Schrittkette lückenlos | gap_in_proof | PL3 |
| MATH02 | Gleichungssystem | Kopplung je Gleichung | inconsistent_system | PL3 |
| MATH03 | Optimierungsmodell | Constraint je Variable | infeasible | PL3 |
| MATH04 | Algorithmus-Spec | Kontrollfluss bis Terminierung | nontermination | PL3 |
| MATH05 | Formale Definition | Definitionen azyklisch | circular_definition | PL3 |
| MATH06 | Theorem+Lemma-Kette | Nutzung je Hypothese | unused_hypothesis | PL3 |
| MATH07 | Kombinatorik | Inzidenz je Objekt | double_counting | PL3 |
| MATH08 | Wahrscheinlichkeitsmodell | Abhängigkeit je Ereignis | unnormalized | PL3 |
| MATH09 | Logische Formel/SAT | Variablenbindung je Klausel | contradiction | PL3 |
| MATH10 | Typsystem/Kalkül | Prämissen azyklisch | broken_preservation | PL3 |
| MATH11 | Kategorielle Konstruktion | Komposition je Morphismus | noncommuting_diagram | PL3 |
| MATH12 | Differentialgleichung | Randbedingung je Term | ill_posed | PL3 |
| MATH13 | Gruppenstruktur | Operationsbindung je Element | not_closed | PL3 |
| MATH14 | Numerisches Verfahren | Fehlerschranke je Schritt | divergence | PL3 |
| MATH15 | Beweis-Skizze | Lückenausweis je Kernidee | hidden_gap | PL3 |

Beweis-Ort: `conformance/tests/family_e_catalog.rs`.
