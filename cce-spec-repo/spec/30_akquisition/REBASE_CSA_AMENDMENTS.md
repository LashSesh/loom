# CCE — CSA-NACHZIEHBLÖCKE (Amendment-Schicht II)

**Nachziehung von S1, S8, S9, S13, S15, Systemlandkarte und Build-Closure-Matrix auf die CSA-Schicht.** Geführt als Amendments (append-only, S13.7-konform), Rebase-Regel überall: `DoD(Si+) = DoD(Si) ∧ FoundationAligned ∧ NoNewSilentResidue` — nichts wird gebrochen, nichts gelockert. Externe Quellen gelangen ab jetzt **ausschließlich** über SourceHorizon → SourceAdapter/AdapterManifest → PolicyGates → EvidencePack → Ledger/Replay in den Korpus.

---

## 1 — S1-Amendment (Katalog: BridgeProfile wird Source-fähig)

- **S1-A5 (BridgeProfile → SourceProfile-Teil):** Der bisher leere BridgeProfile-Port (S1-A2) erhält seine erste normative Füllung — den **Source-Teil**: Jede Domäne kann deklarieren, (a) welche SourceAdapter-Klassen für sie zulässig sind (z. B. KNOW-Familie: OfficialAPI/Registry/Document; SWE-Familie: Repository/OfficialAPI), (b) welche `domain_facet`-Bindung ihre CSUs tragen (Anschluss an die Facet-Typen des MiningProfile), (c) welche Lizenz-/Privacy-Modi ihre Artefaktziele erfordern (LicenseGate-/PrivacyGate-Parameter). Der **Bridge-Teil** (Cross-Domain-BridgeNorms) bleibt Port (R-1b).
- **S1-A6 (MiningProfile-Erweiterung):** Das MiningProfile jeder Domäne erhält ein Feld `source_horizon_template` — der deklarierte Standard-Quellenraum für MiningQueries dieser Domäne (überschreibbar je Lauf, nie erweiterbar ohne Operator-Bestätigung).
- **S1-DoD⁺⁺:** DoD(S1⁺) ∧ Source-Teil des BridgeProfile typisiert ∧ kein Katalog-Eintrag behauptet aktive Quellen ohne Manifest.

## 2 — S8-Amendment (Bibliothek: Akquisitions-Zeugen)

- **S8-A4 (Neue Asset-Typen):** **AdapterManifests** (versioniert, selbst-validiert beim Eintritt), **NexusSourceBundles** als Referenz-Zeugen, die **5 CSA-Reference-Cubes** (LocalCorpus, Wikimedia, GitRepository, Feed, HBMImport) und die **8 CSA-Negative-Cubes** (PolicyBlockedFetch, RateLimitViolation, EvidenceMissing, LicenseIncompatible, ReplayDrift, ScoreAsGateAttempt, SourceUnknown, HTMLScopeLeak).
- **S8-A5 (Regressionswächter erweitert):** Der **eine** CI-Wächter (S8.3) prüft ab jetzt auch: alle CSA-Referenzen grün, alle CSA-Negativen rot — bei jeder Domänen-Aufnahme, jedem Update, jeder CoreExtension **und** jedem neuen SourceAdapter. Ein Adapter, der einen Negativ-Cube schwächte (z. B. Fetch vor PolicyGate ermöglichte), wird abgewiesen wie eine schlechte Domäne.
- **S8-DoD⁺⁺:** neue Typen aufgenommen ∧ 13 CSA-Zeugen CI-gebunden ∧ kein unvalidierter Manifest-Eintritt.

## 3 — S9-Amendment (Persistenz: Source-Provenienz durchgängig)

- **S9-A4 (CAS-Adressierbarkeit):** Der CAS adressiert zusätzlich: RawObservation-Digests, CSUs, EvidencePacks, NSBs, FetchPlans, GateReports, ResidueReports, AdapterManifests (versioniert), PolicySnapshots und den **SourceRunHyperDAG** (`Ledger = CommitProjection(SourceRunHyperDAG)` — dieselbe Projektion wie S9-A1, quellseitig instanziiert).
- **S9-A5 (Cache-/Cursor-Zustand):** ETags, Last-Modified, Feed-/Release-Cursor, Commit-SHAs, Dump-/Snapshot-IDs sind **Ref-Zustand** (veränderlich, workspace-gebunden) über unveränderlichen CAS-Objekten — Differenzabruf ist damit replay-konsistent; wiederholte Vollabrufe tragen ihre TaskSpec-Begründung im Ledger.
- **S9-A6 (Provenienz-Kette verlängert):** Die durchgängige content-adressierte Kette lautet jetzt `Source-Locator → RawObservation → CSU → EvidencePack → [Facet → BlueprintCandidate →] Crystal → Lauf → Artefakt` — jede Lücke ist `provenance_gap`, sichtbar.
- **S9-DoD⁺⁺:** neue Adressierbarkeit ∧ Cursor-als-Ref-Modell ∧ verlängerte Kette abfragbar.

## 4 — S13-Amendment (Governance: SourceHorizon-Vollform, Nicht-Umgehung)

- **S13-A4 (SourceHorizon-Vollform):** Der Governance-Typ SourceHorizon (S13-A1) wird zur Vollform `HS = (goal, domain_scope, source_classes, source_policy, data_policy, budget, capability, replay, stop)`. Jede MiningQuery, jeder Akquisitionslauf trägt genau einen HS; HS-Erweiterung ist materielle Aktion (Operator-Bestätigung); Exploration außerhalb ⇒ `exploration_out_of_horizon`, blocking (A8, unverändert).
- **S13-A5 (Neue Produkt-Invarianten):** **PROD-INV-13 (NoFetchBeforePolicyGate):** kein Netzabruf vor bestandenem SourcePolicy-/Access-/RobotsTerms-/License-/PrivacyGate — architektonisch, nicht konventionell. **PROD-INV-14 (Nicht-Umgehung):** die `disallowed_actions` {captcha_bypass, paywall_bypass, auth_circumvention, bot_protection_evasion, rate_limit_evasion, terms_violation} sind RD-verbindlich und durch keinen Operator-, Kanzel- oder Zellpfad übersteuerbar; `access_blocked`/`robots_blocked` sind Endzustände mit Residue. **PROD-INV-15 (EvidencePack-Pflicht):** kein Export, kein BlueprintCandidate, kein Crystal aus externer Quelle ohne EvidencePack. **PROD-INV-16 (Attribution-Transport):** `license_attribution_required` erzwingt Attributions-Mitführung in Evidence und Export.
- **S13-A6 (CapabilityLocks der Akquisition):** Netzzugriff je Adapterklasse ist einzeln gelockt; **ConnectorAdapter** (OAuth) trägt einen separaten CapabilityLock; StaticWebAdapter-Aktivierung ist eine eigene, protokollierte Freigabe. Zell-Scheduling darf Policy nie überschreiben (CSA-INV-2 als Governance-Norm).
- **S13-DoD⁺⁺:** HS-Vollform ∧ PROD-INV-13..16 ∧ Locks je Adapterklasse ∧ keine Verbots-Lockerung.

## 5 — S15-Amendment (Workbench: CSA als Eingangsschicht)

- **S15-A1 (§S15.8 erweitert):** Das HBM-Chassis erhält CSA als normative **Eingangsschicht**: MiningQueries mit externem Quellenbezug laufen `HS → CSA-Kette → NSB → Facets → Pipeline 0–9`. Facetten aus externen Quellen sind **nur** als evidence-gebundene CSA-Facetten zulässig (`HBMImport`-Formel, CSA.11).
- **S15-A2 (§S15.12 präzisiert):** Der Nexus-Port ist jetzt **halb gefüllt**: die Source-Seite (L9a) ist normativ (diese Schicht); die Bridge-Seite (L9b: BridgeNorm-Promotion, NexusClass, Normic Memory, Quiescence) bleibt Port — CSA liefert dafür bereits `BridgeCandidate`-Vorformen. „Kein freies Scrapen" ist ab jetzt positiv realisiert statt nur verboten.
- **S15-A3 (Kompositionsmatrix):** `Auftrag = (D, s, HS, q?)` verwendet die HS-**Vollform**; die Capsule-Boundary ist `B_D ∩ B_s ∩ HS` mit CSA-Gates im Gate-Satz; Akquisitionszellen sind Workbench-Capsules mit Source-Ratchet.
- **S15-DoD⁺⁺:** CSA vorgelagert integriert ∧ Nexus-Port-Status ehrlich (halb) ∧ Matrix HS-vollformig.

## 6 — Systemlandkarten-Amendment (L9 → L9a/L9b)

Ebene L9 wird verfeinert: **L9a — CSA Source-Akquisition** (SourceHorizon, SourceAdapter, EvidencePack, SourceRunHyperDAG): Status **◐** normativ spezifiziert, Bau ausstehend. **L9b — Domain-Nexus Bridge-Seite** (BridgeNorm, NexusClass, Normic Memory, hypertoroidale Wicklung, Quiescence): Status **○** spätere Foundation (R-1b). Die Rolle „Akquisitions- und Evidence-Unterbau für HBM und Nexus" liegt architektonisch **zwischen L7 und L9b**.

## 7 — Build-Closure-Matrix-Amendment (DoD-Matrix + Handoff)

- **Neue Stufe g:** `SourceAcquisitionDoD = DoD(CSA)` (CSA.16) — Status auf Papier: **1**.
- **PlatformFoundationDoD** wird `c ∧ d ∧ e ∧ g` (Rebase ∧ Spiralprozess ∧ Mining-Chassis ∧ Source-Akquisition).
- **Anti-Overclaim unverändert:** jede Fertig-Aussage nennt ihre Stufe; g behauptet Spezifikations-, nicht Bau-Reife.
- **Agent-Handoff-Ergänzung:** neuer Workspace `cce-nexus-acquisition/` (Crates + Erstadapter + Schemata + golden/integration/compliance-Tests gemäß CSA.13); der Regressionswächter lädt die 13 CSA-Zeugen; Baureihenfolge-Empfehlung: LocalCorpusAdapter zuerst (netzfrei, deckt Reference-Cube 1), dann Wikimedia-API, dann Git.

## 8 — Residuen-Update (Register-Fortschreibung)

- **R-1 wird gesplittet:** **R-1a (Source-Seite) — GESCHLOSSEN** auf Spezifikationsebene durch CSA (dieses Amendment-Paket). **R-1b (Bridge-Seite)** — offen: BridgeNorm-Promotion, NexusClass, Normic Memory, Quiescence-Kriterium; CSA liefert BridgeCandidates als Vorstufe.
- **Neu:** **R-14** Adapter-Erstbestand (= CSA-R1, Empfehlung LocalCorpus→Wikimedia→Git); **R-15** Schwellen-/Gewichte-Kalibrierung τ, w (= CSA-R2, RD-Parameter); **R-16** ConnectorAdapter-OAuth-Feinform + Schlüsselbund-Bindung (= CSA-R3); **R-17** Federation als Nicht-Kernpflicht-Port (= CSA-R4).
- Alle übrigen R-2…R-13 unverändert gültig und sichtbar.

## 9 — Abnahme der Amendment-Schicht II

```
DoD(CSA-Amendments) = 1 ⟺
    S1/S8/S9/S13/S15 nachgezogen, je DoD ungebrochen ∧ FoundationAligned ∧ NoNewSilentResidue
  ∧ externe Quellen erreichen den Korpus nur via HS → Manifest/Adapter → PolicyGates → EvidencePack → Ledger/Replay
  ∧ keine freie Scraping-Funktion; NoFetchBeforePolicyGate architektonisch; Nicht-Umgehung invariant
  ∧ Landkarte L9a/L9b geführt; DoD-Matrix um g erweitert; PlatformFoundationDoD = c∧d∧e∧g
  ∧ R-1a geschlossen, R-1b + R-14..17 sichtbar
```

*Ende der CSA-Nachziehblöcke.*
