# Familie M — Hardware/CAD/Mechatronik/Fertigung (HW01–15)

Über dem geteilten Kern (family_a): 12 Relation + 3 Ketten (HW05
Fertigungs-Routing, HW07 Mechatronik-FSM, HW10 Montage). Struktur
(Verbau-/Netz-/Passungs-Nähte), keine Physiksimulation behauptet.
Alle PL3.

| Domäne | Kern-Residuum | | Domäne | Kern-Residuum |
|---|---|---|---|---|
| HW01 Stückliste | missing_part | | HW09 Materialauswahl | unmet_property |
| HW02 Schaltplan | floating_pin | | HW10 Montageanleitung | impossible_assembly |
| HW03 CAD-Modell | underdimensioned | | HW11 Wartungsplan | unmaintained_part |
| HW04 Toleranzplan | tolerance_stackup | | HW12 FMEA | unmitigated_failure |
| HW05 Fertigungsplan | infeasible_step | | HW13 Leistungsbudget | power_overrun |
| HW06 Prüfplan | uninspected_feature | | HW14 3D-Druck | unsupported_overhang |
| HW07 Mechatronik | unsafe_state | | HW15 Kinematik | overconstrained |
| HW08 PCB-Layout | violated_clearance | | | |

Beweis-Ort: `conformance/tests/family_m_catalog.rs`.
