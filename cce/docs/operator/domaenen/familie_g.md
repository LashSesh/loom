# Familie G — Governance/Compliance/Audit (GOV01–12)

Über dem geteilten Kern (family_a). Jede Domäne verlangt zu ihrem
Subjekt (Anforderung/Feststellung/Kontrolle/Regel/…) die genannte
Nachweis-/Beleg-/Kontroll-Naht; fehlt sie, wird das Kern-Residuum
sichtbar. GOV05 (Risikobewertung) prüft Methoden-Konsistenz — eine
Kennzahl ist NIE das Gate (geerbtes no_score-Gate). Alle PL3; PL4
verlangt echte Nutzungs-/Review-Evidenz (offen).

| Domäne | Zweck | Kern-Naht | Kern-Residuum | PL |
|---|---|---|---|---|
| GOV01 | Compliance-Checkliste | Nachweis je Anforderung | unevidenced_requirement | PL3 |
| GOV02 | Audit-Bericht | Beleg je Feststellung | unbacked_finding | PL3 |
| GOV03 | Kontroll-Matrix | Risiko-Abdeckung | uncovered_risk | PL3 |
| GOV04 | Control-Mapping | Anforderung↔Kontrolle | unmapped_control | PL3 |
| GOV05 | Risikobewertung | Methoden-Konsistenz (kein Score-Gate) | inconsistent_rating | PL3 |
| GOV06 | DPIA | Schutz je Verarbeitung | unmitigated_processing | PL3 |
| GOV07 | Evidence-Paket | Rückverfolgbarkeit | orphan_evidence | PL3 |
| GOV08 | Gap-Analyse | Soll↔Ist | unaddressed_gap | PL3 |
| GOV09 | Attestierung | Beleg je Aussage | unsupported_attestation | PL3 |
| GOV10 | Regelwerk-Konformität | Umsetzung je Regel | unimplemented_rule | PL3 |
| GOV11 | Incident-Report | Ursache je Ereignis | rootless_incident | PL3 |
| GOV12 | Zertifizierungs-Vorbereitung | Nachweis je Kriterium | uncovered_criterion | PL3 |

Beweis-Ort: `conformance/tests/family_g_catalog.rs`.
