# 03 — INVENTAR & INTEGRITÄT (24 Spezifikationen + 5 Wurzeldokumente)

**Integritätscheck (G0-Pflicht):** Führe im Repo-Wurzelverzeichnis aus: `sha256sum -c INTEGRITAET.sha256` — alle 24 Zeilen MÜSSEN `OK` melden. Bei irgendeiner Abweichung: STOPP, Residuum melden, nicht weiterbauen. (Die Wurzeldokumente 00–04 sind bewusst nicht in der Summenliste: sie sind die prüfende Schicht selbst; ihre Autorität ergibt sich aus 00_START_HIER §2.)

## Spezifikations-Inventar

| Datei | Schicht | Rolle | Zeilen | SHA-256 (Kurzform) |
|---|---|---|---|---|
| `spec/00_kern/BAUVERFASSUNG.md` | L0–L2 | Motor-Verfassung: Objekte, Operatoren, Gates G1–G7, INV/V, Repo-Struktur, One-Shot Teil 9 | 1256 | `bb4334cb7b59f917…` |
| `spec/00_kern/SYSTEMLANDKARTE.md` | Karte | Produkt-Masterplan: Kern + 4 Ringe, Ebenen S1–S13, Weichen | 200 | `7ffa779ae7ca9311…` |
| `spec/00_kern/SYSTEMLANDKARTE_VNEXT.md` | Karte | Fundament-Vertikale L0–L11, Mapping alt→neu, Statusmarken | 67 | `f2acd7b0946d8b7b…` |
| `spec/10_produkt/S10_PRODUKTABNAHME.md` | L11 | ProduktDoD(Dokument), Fertig-Stufen | 149 | `f4a0d357eebc7b5d…` |
| `spec/10_produkt/S11_AUSLIEFERUNG.md` | L11 | Lebenszyklus konsolenfrei, offline-Kern, Pakete | 144 | `28ade05f9d66b0e9…` |
| `spec/10_produkt/S12_OPERATORDOKU.md` | L11 | Doku unter Claim-Schranke, Onboarding | 132 | `3f7b53a32f5d26c1…` |
| `spec/10_produkt/S13_NUTZUNGSGOVERNANCE.md` | L11 | V1–V10 im Betrieb, PROD-INV, Claim-Schranke | 167 | `c0d76b883bef3823…` |
| `spec/10_produkt/S14_KERNERWEITERUNG.md` | quer | CoreExtension: 5 Punkte, closure-erhaltende Beweispflichten | 176 | `2e2d88d6540283ff…` |
| `spec/10_produkt/S1_DOKUMENT.md` | L8 | Dokument-Domäne (Referenz) + DomainAdapter-Vorlage §S1.8 (11 Punkte) | 263 | `c8a165b19b59ca87…` |
| `spec/10_produkt/S1_DOMAENENKATALOG.md` | L8 | 213 Leaf-Domänen / 16 Familien, PL-Leiter, ProfessionalReviewGate | 483 | `5457c0baa1098034…` |
| `spec/10_produkt/S2_OPERATORREISE.md` | L11 | End-to-End-Reise, 6 Nähte, Produkt-Kerntest | 199 | `d6d7ed19d7505f15…` |
| `spec/10_produkt/S3_COCKPIT_KANZEL.md` | L11 | Natives Cockpit, 4 Flächen, KI-Kanzel-Grenzen, COCK-INV | 307 | `68168ce8c98ae686…` |
| `spec/10_produkt/S4_WUNSCHERFASSUNG.md` | L11 | 7-Stufen-Wunschpipeline, Annahmen-Liste, Bestätigungsgrenze | 184 | `bcc347a9aca44d63…` |
| `spec/10_produkt/S5_ORCHESTRIERUNG.md` | L11 | Läufe, HITL, RunDescriptor, Checkpoints | 180 | `f75c1f586a3120be…` |
| `spec/10_produkt/S6_INSPEKTION.md` | L11 | 5 Inspektionsobjekte, Fakten/Erklärung, Drill-down | 175 | `54327228e4e3d374…` |
| `spec/10_produkt/S7_ARTEFAKT.md` | L11 | Ausgabe, Zwei-Digest-Modell, Re-Import | 180 | `fa6bbf31185e6158…` |
| `spec/10_produkt/S8_BIBLIOTHEK.md` | L11 | Zeugen (Referenz/Negativ), CI-Regressionswächter | 155 | `b7d72e75545f3831…` |
| `spec/10_produkt/S9_PERSISTENZ.md` | L11 | CAS+Refs, Workspaces, Sync-Vorbereitung | 149 | `369d9ae179bb8fb4…` |
| `spec/20_rebase/REBASE_KONSOLIDIERUNG.md` | L3–L7 | Rebase-Register, Impact Map, HBM-Neutralisierung, Normalform, DoD-Matrix a–f, Residuen R-1..13, Handoff | 223 | `812bb7778ade41a0…` |
| `spec/20_rebase/REBASE_S1-S14.md` | Overlay | Normative Amendments S1-A…S14-A (gelten mit!) | 144 | `857b72ce159c6b4a…` |
| `spec/20_rebase/S15_OPERATIONAL_MULTICUBE_WORKBENCH.md` | L10 | ScaleAdapter, Multicube, Capsule, MultiScaleClosure, ActionCandidate/CapabilityLock | 178 | `539fe80f38150b23…` |
| `spec/30_akquisition/CSA_NEXUS_AKQUISITION.md` | L9a | Akquisitionskette, 15 Gates, 18 Residuen, 11 Adapterklassen, 13 Zeugen | 241 | `38de45329c256780…` |
| `spec/30_akquisition/REBASE_CSA_AMENDMENTS.md` | Overlay | Amendments S1/S8/S9/S13/S15 + Matrix-Stufe g + L9a/b (gelten mit!) | 68 | `41409284fbabe1fa…` |
| `spec/40_format/LOOM_CONTAINER_STANDARD_V1.md` | Format | .loom LBC-1: Bytes, Segmente, core_root, APIs, 21 Zeugen, C0–C5, Handoff B1–B12, Matrix-Stufe h | 340 | `7aebeb6862e8799b…` |

## Wurzeldokumente (die Übergabeschicht)

| Datei | Rolle |
|---|---|
| `00_START_HIER.md` | Pyramide, Autoritätsordnung, Leseordnung, Zielstruktur, Nicht-Verhandelbares |
| `01_MASTER_BUILD.md` | konsolidierte Bauordnung G0–G12 mit Ausgangs-Gates |
| `02_MASTER_DOD.md` | 100%-Formel, Abhak-Checkliste, explizite Nicht-Bestandteile |
| `03_INVENTAR.md` | dieses Inventar + Integritätsanker |
| `04_AGENT_AUFTRAG.md` | Beauftragungstext, Melde- und Residuum-Formate |

## Offene Punkte des Repositorys (sichtbar)

- **Lizenz/NOTICE:** bewusst nicht gesetzt — Entscheidung des Auftraggebers vor Veröffentlichung (Repo-R1).
- **R-9 (Amendment-Einarbeitung):** Overlays gelten normativ; physische Einarbeitung in die S-Dateien ist ein späterer Redaktionsschritt des Auftraggebers, nie des Bau-Agenten.
- Vollständiges Residuen-Bild: REBASE_KONSOLIDIERUNG §7, CSA §CSA.15, LOOM Teil 13.2, S15 §S15.16, MASTER_DOD §3.
