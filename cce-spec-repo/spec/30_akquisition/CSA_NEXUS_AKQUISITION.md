# CCE — SOURCE-ACQUISITION-SCHICHT (CSA): NEXUS-AKQUISITIONSPROTOKOLL

**Die closure-fähige Eingangsschicht für externe Quellen.** Kein Scraper, kein freier Crawler — die formale Source- und API-Schicht zwischen Domain-Nexus, HBM-Mining-Chassis, PHC/LOOM und der Operational-Multicube-Workbench. Kern: **zielgerichtete Freiheit innerhalb eines deklarierten SourceHorizon**; außerhalb davon entsteht Hold/Residue, niemals Pass.

**Status:** Normative Detail-Spezifikation v1.0, auf Bauverfassungs-Tiefe. Assimiliert das „CCE Nexus-Akquisitionsprotokoll v0.1" (Neutralisierung von ODN + OMNIPOL) als Ebene **L9a** der Systemlandkarte vNext — zwischen L7 (HBM) und L9b (BridgeNorm, weiterhin Port). Löst Residuum **R-1a** (Source-Seite des Nexus). Baut auf: Rebase-Konsolidierung (A1–A8), S13+Amendments (SourceHorizon-Typ, CapabilityLock, ExplorationPolicy), S15 (§S15.8/§S15.12), S8/S9.

---

## CSA.0 — Einordnung & Leitsatz

Bisher galt (S15.12): „Der Nexus darf niemals frei scrapen" — als Verbot mit Port. CSA liefert jetzt die **positive Form**: wie zulässige offene Quellen, APIs, Dumps, Feeds, Repositories und lokale Korpora **closure-fähig** in den Korpus gelangen.

Leitsatz:

> *Der Nexus ist zielgerichtet frei: Er darf Quellen dynamisch entdecken, Adapterkandidaten erzeugen, Fetch-Pläne bilden und Suchräume erweitern — genau innerhalb eines deklarierten SourceHorizon und nur durch Policy-, Lizenz-, Privacy-, Provenienz-, Rate-, Budget-, Capability- und Replay-Gates hindurch. Kein Fetch vor dem PolicyGate. Kein Export ohne EvidencePack. Kein stiller Fehler. Freiheit ohne Horizon ist kein Pass, sondern Hold.*

Positionsbild (Landkarte vNext, verfeinert):

```
L10  Operational Multicube / Workbench Capsule
L9b  Domain-Nexus: BridgeNorm / NexusClass / Normic Memory        (Port, R-1b)
L9a  CSA: SourceHorizon · SourceAdapter · EvidencePack · RunLedger  ← diese Spezifikation
L7   HBM Mining-Chassis: Facets · Cube/HDAG · Skeleton · Candidate
L1–L6  Closure · PHC/LOOM · PhaseBlock · Spiralprozess · Multi-Ratchet
```

Leitformel und erweiterte Kette:

```
Intent —[SourceHorizon]→ SourcePlan —[Gates]→ Observation —[Can+Evidence+Replay]→ NexusSourceBundle | ResidueReport
NexusSourceBundle → Facets → HBM → BlueprintCandidate → Crystal → PHC/LOOM
```

## CSA.1 — Die drei Axiome

- **CSA-A1 (Zielgerichtete Freiheit).** Dynamische Quellensuche, Adapterkandidaten-Erzeugung, FetchPlan-Bildung und Suchraum-Erweiterung sind zulässig — genau innerhalb des deklarierten SourceHorizon und nur über die Pflichtgates. Freiheit ohne Horizon ⇒ Hold.
- **CSA-A2 (API/Dump/Feed/Connector vor HTML).** Offizielle APIs, Dumps, Feeds, Snapshots und Connectoren haben Vorrang. Generische HTML-Beobachtung ist nur als eng begrenzter **StaticWebAdapter** zulässig, wenn keine geeignetere Quelle vorliegt, der Zugriff im SourceHorizon liegt und **alle** SourcePolicyGates bestanden sind.
- **CSA-A3 (Score ranked, Gate decides).** Qualitätsmetriken, Frischewerte und Ranking-Funktionen dürfen Priorität bestimmen — nie Zulassung ersetzen. Nur Gates entscheiden: **allow, hold, reject, quarantine**.

## CSA.2 — SourceHorizon, CandidateSource, ActiveSource

```
HS = (goal, domain_scope, source_classes, source_policy, data_policy, budget, capability, replay, stop)
```

Der SourceHorizon ist der aktive, zielgerichtete Quellenraum **eines Laufs**: welche Quellenklassen, Adapter, Aktionen, Datenarten, Budgets, Lizenzen, Privacy-Modi und Stop-Regeln zulässig sind. Er ist Governance-Objekt (S13-A4) und Teil des RunDescriptors (policy_snapshot).

```
CandidateSource cs = (locator, type, discovered_by, expected_schema, license_hint, risk, confidence, residue)
```

Eine **entdeckte, noch nicht aktive** Quelle. Harte Regel: `cs` darf **nicht abgerufen** werden, bevor ein SourceAdapter existiert oder ein Adapterkandidat das SourcePolicyGate bestanden hat (`source_unknown` bleibt passiv).

```
ActiveSource(cs) = 1 ⟺ Manifest(cs) = valid ∧ Preflight(cs) = Pass ∧ FetchPlan(cs) = valid
```

## CSA.3 — Objektmodell

- **RunDescriptor** `RD = (run_id, intent, boundary, budget, determinism, adapters, policy_snapshot, seed)` — replaykritisch; jede nichtdeterministische Wahl, jeder AdaptiveProfile-Parameter, jeder Backoff-Typ, jede Tie-Break-Regel liegt im RD. Die `boundary` enthält verbindlich `disallowed_actions ⊇ {captcha_bypass, paywall_bypass, auth_circumvention, bot_protection_evasion, rate_limit_evasion, terms_violation}`, `pii_mode` und `license_mode`.
- **TaskSpec** `T = (intent, scope, source_policy, data_policy, budget, output_contract, stop_rule, scale_target)` — Default `scale_target = SCALE-1` (importfähiges Evidence-/Data-Bundle); höhere ScaleTargets erzeugen keine direkte Ausführung, sondern Workbench-/HBM-Kandidaten für nachgelagerte Gates.
- **SourceAdapter (Port-Vertrag, Adapterparität):**

```rust
trait SourceAdapter {
  fn manifest(&self) -> AdapterManifest;          // Identität, Klasse, Policy, License, Privacy,
                                                  // Budget/Rate, Provenance- & Replay-Strategie
  fn preflight(&self, task: &TaskSpec) -> GateReport;
  fn plan(&self, task: &TaskSpec, budget: &Budget) -> FetchPlan;
  async fn acquire(&self, plan: &FetchPlan) -> Vec<RawObservation>;
  fn extract(&self, raw: &RawObservation) -> Vec<ExtractedRecord>;
  fn normalize(&self, record: &ExtractedRecord) -> Vec<CanonicalSourceUnit>;
  fn validate(&self, unit: &CanonicalSourceUnit) -> ValidationReport;
  fn cite(&self, unit: &CanonicalSourceUnit) -> Vec<EvidenceAtom>;
}
```

  Neue Quellenarten ändern den Kern **nicht** — sie implementieren denselben Portvertrag (`check_source_adapter_parity`, analog S1.8/S15.1). **Jede Quelle braucht:** AdapterManifest, Preflight, Budget, RateLimit, Policy, License, Privacy, Provenance und Replay-Strategie — alle neun sind Manifest-Pflichtfelder; fehlt eines ⇒ `manifest_missing`, Quelle bleibt passiv.
- **Canonical Source Unit** `CSU = (uid, kind, payload, schema, quality, provenance, license, timestamps, hashes, residues, domain_facet)` — die **kleinste** Einheit, die an HBM, DomainAdapter, Nexus oder PHC übergeben werden darf.
- **EvidencePack** `EP = (evidence_id, record_id, source, locator, transform_path, policy_snapshot, license, hashes)` — bindet eine CSU an Abruf, Transformationspfad und Policy-Snapshot. **Kein Export, kein BlueprintCandidate, kein Crystal ohne EvidencePack.**
- **NexusSourceBundle** `NSB = (bundle_id, run_id, csu_set, evidence_packs, source_graph, quality_summary, residues, export_contract)` — der kanonische Übergabekörper an HBM und PHC.

## CSA.4 — Die Akquisitionskette (normativ, Station für Station)

```
Intent ──I_χ──▶ SourceHorizon ──D_δ──▶ SourceDiscovery ──▶ CandidateSource
   │ [SourceHorizonGate]                                        │ [SourcePolicyGate · AccessGate ·
   ▼                                                            ▼  RobotsTermsGate · LicenseGate · PrivacyGate]
 FetchPlan (B_β: Budget/Rate/Capability) ──A_α──▶ RawObservation
   │ [RateBudgetGate; Cache/Differenzabruf]          │ E_ε Extraktion · Locator-Bindung
   ▼                                                 ▼ [SchemaGate]
 CanonicalSourceUnit (N_ν) ──V_φ──▶ validiert+dedupliziert+Qualität ──P_π──▶ EvidencePack + Ledger-Anker
   │ [QualityGate · ProvenanceGate · EvidenceGate]
   ▼
 NexusSourceBundle (C_κ) ──R_ρ──▶ HBM | DomainAdapter | PHC | Export | ResidueReport
   [ReplayGate · HBMImportGate · PHCProjectionGate · ExportGate]
```

Hart überall: **Kein Fetch vor PolicyGate** (kein Netzabruf, bevor SourcePolicy/Access/RobotsTerms/License/Privacy grün sind); **jede** zulässige Beobachtung wird CSU + EvidencePack; **jede** unzulässige wird ResidueReport — kein stiller Erfolg, kein stiller Fehler.

## CSA.5 — Operatorstack

```
Ω_CSA = R_ρ ∘ C_κ ∘ P_π ∘ V_φ ∘ N_ν ∘ E_ε ∘ A_α ∘ B_β ∘ D_δ ∘ I_χ
```

| Operator | Bedeutung |
|---|---|
| I_χ | Intent-Kanonisierung, BoundarySpec-Erzeugung |
| D_δ | SourceDiscovery, CandidateSource-Erzeugung |
| B_β | Budgetierung, Rate-/Risk-Planung, Capability-Prüfung |
| A_α | Akquisition über erlaubte APIs, Dumps, Feeds, Git, lokale Korpora oder eng erlaubtes HTML |
| E_ε | Extraktion, Parsing, Locator-Bindung |
| N_ν | Normalisierung zu CSU |
| V_φ | Validierung, Deduplikation, Qualitätsmessung, GateReports |
| P_π | Provenienzbindung, EvidencePack, Ledger-Anker |
| C_κ | Kondensation zu NexusSourceBundle |
| R_ρ | Routing an HBM, DomainAdapter, PHC, Export oder ResidueReport |

**Kerninvarianz (CSA-INV-1):** Ω_CSA kennt keine domänenspezifischen Selektoren, keine festen Webseiten, keine harten Output-Annahmen. Domänenspezifik liegt ausschließlich in AdapterManifest, Parser, Normalizer, Validator, DomainAdapter und MiningProfile.

## CSA.6 — Akquisitionszellen und Source-Ratchet

`CellCluster = (cells, edges, weights, state, ledger)` — Zellenrollen: discovery, planning, fetch, decode, normalize, validate, evidence, export, residue. Zellen sind Workbench-Capsules (S15.3) im Akquisitionskontext: scope-gebunden, budgetiert, ttl-begrenzt, dissolution-pflichtig.

**CSA-INV-2 (Zellenadaptation ohne Policy-Drift):** Eine Zelle darf interne Gewichte, Backoff, Priorität und Scheduling anpassen. Sie darf **keine** SourcePolicy, kein LicenseGate, kein PrivacyGate, kein CapabilityLock und kein Replay-Gate überschreiben. Gewichte beeinflussen Scheduling, niemals Policy.

**Source-Ratchet:** `R_src = (phase, cursor, budget, evidence, residue, commit, trace)` mit `advance(R_src) = 1 ⟺ Gate = Pass ∧ Evidence = 1 ∧ Residue sichtbar` — der Akquisitionslauf ist ein Ratchet der Multi-Ratchet-Kaskade (CellRatchet-Klasse); Cursor-Fortschritt ohne Gate/Evidence ist ausgeschlossen.

## CSA.7 — Die Pflichtgates

Alle boolesch, fail-closed, begründet, **ledgerpflichtig**; Entscheid stets `allow | hold | reject | quarantine`:

| Gate | Bedeutung |
|---|---|
| SourceHorizonGate | Auftrag liegt innerhalb des deklarierten Quellenhorizonts |
| SourcePolicyGate | Quelle besitzt Manifest, Policy, Access-Methode, Nutzungsstatus, zulässige Klasse |
| AccessGate | Auth/API-Key/public-private-Status geklärt; **keine Umgehung** |
| RobotsTermsGate | robots.txt/Terms/äquivalente Quellpolicy stehen nicht entgegen oder sind nicht anwendbar |
| LicenseGate | Lizenz, Attribution, Redistribution, Exportmodus kompatibel |
| PrivacyGate | PII-Modus, Datenminimierung, Ausschlussregeln eingehalten |
| RateBudgetGate | Quellen- und Globalbudget, Backoff, Cache-Policy eingehalten |
| SchemaGate | erwartete Struktur parsebar oder mit Residue geführt |
| QualityGate | Achsen-Mindestwerte erreicht (CSA.10); Score entscheidet nicht allein |
| ProvenanceGate | Locator, Abrufzeit, Hashes, Adapter-/Parserversion, Lizenzstatus vorhanden |
| EvidenceGate | jede exportierte CSU besitzt EvidencePack |
| ReplayGate | fixer Snapshot erzeugt gleiche IDs, Hashes, Ledgerpfade |
| HBMImportGate | CSU/NSB ist als Facet-Quelle für HBM typisiert |
| PHCProjectionGate | Bundle in PHC-/BlueCube-Input projizierbar |
| ExportGate | ExportContract, Lizenztransport, ResidueReport vollständig |

**Lauf-Gate-Formel:**

```
DoD_run = G_horizon ∧ G_policy ∧ G_access ∧ G_rate ∧ G_license ∧ G_privacy
        ∧ G_provenance ∧ G_evidence ∧ G_replay ∧ G_export
```

Scheitert ein Gate: kein stiller Erfolg, sondern ResidueReport. `quarantine` isoliert bereits geholte Beobachtungen ohne Export-/Importpfad, bis ein Gate-Neuentscheid vorliegt.

## CSA.8 — Residuen-Vokabular

`source_unknown` · `manifest_missing` · `terms_unknown` · `robots_blocked` · `access_blocked` (Auth/Paywall/Captcha/Zugriffskontrolle blockiert — Endzustand, keine Umgehung) · `license_incompatible` · `license_attribution_required` (Attribution muss in Evidence/Export transportiert werden) · `privacy_risk` · `rate_budget_exceeded` · `schema_unparseable` · `provenance_gap` · `evidence_missing` · `quality_axis_missing` · `score_as_gate_attempt` · `replay_drift` · `hbm_projection_missing` · `phc_projection_missing` · `export_blocked`. Jedes sichtbar, jedes im ResidueReport, jedes mit Negativ-Zeuge (CSA.12).

## CSA.9 — SourceAdapter-Klassen (mit Vorrangordnung)

Vorrang gemäß CSA-A2: **1→4 vor 5–6; 7–11 nach Quellenlage.**

| # | Klasse | Rolle |
|---|---|---|
| 1 | OfficialAPIAdapter | offizielle APIs (Wikimedia/Wikidata, GitHub/GitLab, Crossref, OpenAlex, arXiv, PubMed, Paketregister, Standardregister) |
| 2 | BulkDumpAdapter | offizielle Dumps, Snapshots, OAI-PMH, Archivdaten — sofern Policy kompatibel |
| 3 | FeedAdapter | RSS, Atom, Change-/Releasefeeds (Cursor-basiert) |
| 4 | ConnectorAdapter | OAuth-/Connector-gebundene Quellen — **separater CapabilityLock** |
| 5 | SitemapAdapter | deklarierte Seitenmengen, **kein freier Crawl** |
| 6 | StaticWebAdapter | konservative HTML-Beobachtung, **nur** bei erlaubter Nutzung + engem Scope + Rate-Grenze + deklarierter Parserregel; Scope-Verlassen = blocking Residue |
| 7 | RepositoryAdapter | Git-Repositories: Dateien, Issues, PRs, Releases, Commits, Lizenzdaten |
| 8 | DocumentAdapter | PDFs, Markdown, TXT, CSV, JSON, XML aus erlaubten Quellen |
| 9 | RegistryAdapter | strukturierte Register, Normdaten, Ontologien, Wissensgraphen |
| 10 | SearchAPIAdapter | Suchdienste als **CandidateSource-Finder**, nicht als Inhaltsersatz; strenge Quote/Terms |
| 11 | LocalCorpusAdapter | lokale Dateien, ZIPs, Datenbestände, Nutzerkorpus |

## CSA.10 — FetchPlan, Caching, Qualitätssignatur

**FetchPlan** (content-adressiert): `{plan_id, run_id, adapter_id, operations[{method, endpoint_template, expected_status, cost{requests, bytes_estimate}}], gates[...], cache_policy{etag, last_modified, ttl_sec}, backoff_policy}` — jeder Abruf ist geplant, budgetiert, gate-gelistet.

**Caching/Differenzabruf (Pflicht, wo vorhanden):** ETags, Last-Modified, Release-/Feed-Cursor, Commit-SHAs, Dump-Versionen, Snapshot-IDs. Wiederholte Vollabrufe brauchen eine TaskSpec-Begründung.

**Qualitätssignatur:** `σ(c) = (ψ, ρ, ω) ∈ [0,1]³` — ψ semantische Passung, ρ Informationsdichte/Vollständigkeit, ω Aktualität/Frische. Ranking: gewichtetes geometrisches Mittel `D(c) = ψ^wψ · ρ^wρ · ω^wω`, `wψ+wρ+wω = 1`. **Gate-Zulassung verlangt Achsen-Mindestwerte:** `G_quality(c) = 1 ⟺ D(c) ≥ τ_D ∧ ψ(c) ≥ τ_ψ ∧ ρ(c) ≥ τ_ρ ∧ ω(c) ≥ τ_ω` — der Score ordnet die Warteschlange, die Schwellen sind deklarierte RD-Parameter, das Gate entscheidet. Der Qualitätsvektor ist **kein Wahrheitsbeweis** (Neutralisierung „Resonanz").

## CSA.11 — Integration: HBM, PHC, BridgeNorm

- **HBM-Import:** CSU → Facet `(id, type, scope, source, evidence, confidence, domain_projection)`; `HBMImport(c) = 1 ⟺ EvidenceGate(c) ∧ ProvenanceGate(c) ∧ DomainProjection(c) ∧ ReplayGate(c)`. Damit speist CSA die Pipeline-Phase 1 (Facet Extraction) mit **belegten** Facetten.
- **PHC-/BlueCube-Projektion:** `NSB —Π_PHC→ Blue_{source,p} = (Σ,Ω,B,K,Cand,G,R,L)` — CSUs werden Zustands-/Kandidatenelemente, EvidencePacks werden Proof-Payloads (MEF-gebunden), Residues werden Counter-Horizon, LedgerEvents werden PhaseBlock-Inputs. Die Quelle wird damit selbst ein BlueCube mit regulärem Schließungsweg.
- **BridgeNorm-Vorbereitung:** CSA erzeugt **keine** finalen BridgeNorms — nur `BridgeCandidate = (source_domain, target_domain, pattern, evidence, license, risk, replay)`. Finale BridgeNorm-Promotion bleibt Aufgabe der Nexus-Foundation (L9b, R-1b).

## CSA.12 — Reference- und Negative-Cubes (S8-gewacht)

**Reference-Cubes (grün, je Pflichtkategorie):** 1. **LocalCorpusReference** — lokale Markdown/JSON-Dateien → CSUs + EvidencePacks (Kategorie Evidence). 2. **WikimediaReference** — offizieller API-Abruf mit Attribution + Replay (SourcePolicy/License). 3. **GitRepositoryReference** — Repo-Datei mit Lizenz, Commit-Ref, Content-Hash (Provenance/License). 4. **FeedReference** — RSS/Atom mit Cursor + Frischeachse ω (Rate/Replay). 5. **HBMImportReference** — NSB → Facets → BlueprintCandidate (HBMImport; die PHC-Projektion desselben Bundles deckt PHCProjection).

**Negative-Cubes (rot, blockieren für immer):** 1. **PolicyBlockedFetch** — PolicyGate failt **vor** jedem Netzabruf (SourcePolicy). 2. **RateLimitViolation** — Adapter versucht Überschreitung; muss holden (Rate). 3. **EvidenceMissing** — Record ohne Evidence darf nicht exportiert werden (Evidence). 4. **LicenseIncompatible** — ExportContract blockiert (License). 5. **ReplayDrift** — Snapshot-Wiederholung erzeugt abweichende IDs (Replay). 6. **ScoreAsGateAttempt** — hoher Score ohne GatePass wird abgelehnt (Score/Gate). 7. **SourceUnknown** — CandidateSource ohne Manifest bleibt passiv (SourcePolicy). 8. **HTMLScopeLeak** — StaticWebAdapter verlässt Scope; blocking (SourcePolicy/Rate). — Die geforderten Kategorien SourcePolicy, Rate, License, Evidence, Replay, HBMImport, PHCProjection sind damit je durch Referenz- und Negativ-Zeugen gedeckt; alle CI-gebunden (S8.3, ein Wächter).

## CSA.13 — Repository- und Schema-Normalform (Bau, später)

Rust-first-Workspace `cce-nexus-acquisition/` mit Crates `nexus-core, -policy, -cell, -adapter, -ingress, -fetch, -decode, -normalize, -validate, -evidence, -ledger, -store, -export, -cli`, Erstadaptern `local_file, json_api, generic_html, git_repository, wikimedia`, `schemas/`, `tests/{golden,integration,compliance}`. **Pflichtschemata:** RunDescriptor, AdapterManifest, CellCluster, RawObservation, CSU/NormalizedRecord, EvidencePack, LedgerEvent, NSB/ExportBundle, ResidueReport, FetchPlan, GateReport. Der Ledger ist `CommitProjection(SourceRunHyperDAG)` — S9-konform, kein Blockchain-Import.

## CSA.14 — Verbotene Handhaben (abwesend by design)

- **keine freie Scraping-Funktion** — es existiert kein Codepfad „hole URL ohne Horizon/Manifest/Gates";
- **kein Fetch vor PolicyGate** — der Netzpfad ist architektonisch hinter SourcePolicy/Access/RobotsTerms/License/Privacy verriegelt;
- **keine Umgehung** von Auth, Captchas, Paywalls, Bot-Schutz, Rate-Limits oder Terms — `disallowed_actions` sind RD-verbindlich; `access_blocked`/`robots_blocked` sind **Endzustände** mit Residue, keine Herausforderungen;
- **kein Export ohne EvidencePack**; **kein stiller Fehler** (jeder Nicht-Pass wird ResidueReport); **kein Score als Gate**; **keine Zell-Policy-Drift** (CSA-INV-2); **keine finale BridgeNorm** aus dieser Schicht.

## CSA.15 — Sichtbare Residuen dieser Spezifikation

- **CSA-R1 (Adapter-Erstbestand):** Empfohlen als erste PL-Reifung: LocalCorpusAdapter + ein OfficialAPIAdapter (Wikimedia) + RepositoryAdapter (Git) — deckt die fünf Reference-Cubes; Instanziierung ausstehend.
- **CSA-R2 (Schwellen-Kalibrierung):** τ_D, τ_ψ, τ_ρ, τ_ω und Gewichte w sind RD-Parameter; Kalibrierung beim Bau; nie Gate-Ersatz.
- **CSA-R3 (ConnectorAdapter-Feinspezifikation):** OAuth-Fluss + Secrets im OS-Schlüsselbund (S11) + separater CapabilityLock — Feinform beim Bau.
- **CSA-R4 (Federation):** Node-/Capability-Austausch bleibt Nicht-Kernpflicht; nur als späterer Port notiert.
- **CSA-R5 (BridgeNorm-Promotion):** = R-1b, Nexus-Foundation ausstehend.

## CSA.16 — Abnahme

```
DoD(CSA) = 1 ⟺
    SourceHorizon definiert (HS-Tupel, Governance-Objekt, RD-gebunden)
  ∧ SourceAdapter-Parität (ein Portvertrag, 8 Methoden, check_source_adapter_parity;
        jede Quelle: Manifest+Preflight+Budget+RateLimit+Policy+License+Privacy+Provenance+Replay)
  ∧ NoFetchBeforePolicyGate (architektonisch verriegelt)
  ∧ EvidencePack für jede CSU (kein Export/Candidate/Crystal ohne EP)
  ∧ Replay deterministisch (ReplayGate; fixer Snapshot ⇒ gleiche IDs/Hashes/Ledgerpfade)
  ∧ ResidueReport vollständig (18 Residuen, kein stiller Fehler, quarantine-Regime)
  ∧ HBMImportGate definiert (CSU→Facet, evidence-gebunden)
  ∧ PHCProjectionGate definiert (NSB→Blue_source, Residues→Counter-Horizon)
  ∧ Reference-Cubes (5) und Negative-Cubes (8) vorhanden, alle Pflichtkategorien gedeckt, CI-gewacht
  ∧ keine Fundamentinvariante gelockert; A7/A8 überall; Engine-/Produkt-DoD unberührt
```

**Abschlussformel:** `Observe_HS(source) → CSU + Evidence → NSB → HBM/PHC → CrystalInput` — und: `¬Gate ⇒ ResidueReport`, `Gate ∧ Evidence ∧ Replay ⇒ ImportableBundle`.

*Ende der CSA-Spezifikation.*
