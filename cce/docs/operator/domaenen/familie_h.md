# Familie H — Security/Infra/Ops (OPS01–15)

Über dem geteilten Kern (family_a). Safety-by-abstraction (S13-A3):
geprüft wird die STRUKTUR (Abwehr-/Fix-/Grenz-Nähte vorhanden), NICHT
die Ausführung von Angriffen. 12 Relation + 1 Azyklik (OPS09) + 2 Ketten
(OPS03 Incident, OPS15 Deployment). Alle PL3.

| Domäne | Kern-Residuum | | Domäne | Kern-Residuum |
|---|---|---|---|---|
| OPS01 Threat-Model | unmitigated_threat | | OPS09 Patch-Plan | unpatched_cve |
| OPS02 Runbook | undefined_branch | | OPS10 Vuln-Bericht | unremediated_vuln |
| OPS03 Incident-Response | missing_containment | | OPS11 SLO/SLA | unmeasurable_slo |
| OPS04 Härtung | unhardened_asset | | OPS12 Skalierung | scaling_gap |
| OPS05 Segmentierung | flat_network | | OPS13 Backup | unbacked_data |
| OPS06 RBAC | excess_privilege | | OPS14 Secrets | plaintext_secret |
| OPS07 Monitoring | blind_spot | | OPS15 Deployment | no_rollback |
| OPS08 Disaster-Recovery | untested_recovery | | | |

Beweis-Ort: `conformance/tests/family_h_catalog.rs`.
