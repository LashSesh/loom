# Familie B — Software Engineering (SWE01–15)

Über dem geteilten Kern (family_a): Unit = Code-/Spec-Einheit, Naht =
Aufruf-/Vertrags-/Abhängigkeits-Beziehung. 12 Relation-Regeln, 1 Azyklik
(SWE07 Architektur zyklenfrei), 1 Kette (SWE05 Pipeline-Stufen bis zum
Gate lückenlos, fail-closed). `equivalent` = Klassenvergleich, nie
Formatierung. Alle PL3; PL4 verlangt Nutzungs-Evidenz.

| Domäne | Zweck | Kern-Regel | Kern-Residuum | PL |
|---|---|---|---|---|
| SWE01 | Modul/Funktion | Testbezug je Funktion | untested_path | PL3 |
| SWE02 | API-Spezifikation | Schema je Endpoint | undefined_response | PL3 |
| SWE03 | Test-Suite | Abdeckung je Anforderung | uncovered_requirement | PL3 |
| SWE04 | DB-Schema | auflösbarer FK je Tabelle | orphan_fk | PL3 |
| SWE05 | CI/CD-Pipeline | Stufenkette bis Gate lückenlos | missing_gate | PL3 |
| SWE06 | Refactoring-Plan | Verhaltens-Erhalt je Schritt | behavior_change | PL3 |
| SWE07 | Architektur/ADR | Schnittstellen azyklisch | dependency_cycle | PL3 |
| SWE08 | Bugfix/Patch | Ursache je Änderung | regression | PL3 |
| SWE09 | Konfigurationsdatei | Geltung je Schlüssel | missing_key | PL3 |
| SWE10 | CLI-Tool | Doku je Flag | undocumented_flag | PL3 |
| SWE11 | Library/Package | Vertrag je Public-API | breaking_change | PL3 |
| SWE12 | Code-Review-Bericht | Fundstelle je Befund | unlocated_finding | PL3 |
| SWE13 | SBOM | Lizenz je Abhängigkeit | unknown_license | PL3 |
| SWE14 | Infrastructure-as-Code | Sollbezug je Ressource | drift | PL3 |
| SWE15 | Grammatik/Parser | eindeutige Ableitung je Regel | ambiguous_grammar | PL3 |

Beweis-Ort: `conformance/tests/family_b_catalog.rs`.
