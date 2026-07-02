# 05 — RESUME & INFERENCE-AMENDMENT (Einzeldatei-Overlay, in sich vollständig)

**Zweck:** Diese eine Datei (a) nimmt den unterbrochenen Bau wieder auf und (b) zieht die Model-/Inference-/Agent-Schicht (L9c) vollständig nach. Sie ist ein **normativer Overlay** (append-only) über `00_START_HIER.md`, `01_MASTER_BUILD.md`, `02_MASTER_DOD.md`, `04_AGENT_AUFTRAG.md` und `spec/` — **ein Neuupload des Repositorys ist nicht nötig.** Bei Konflikt gilt: Fundamentinvarianten (F1–F6, INV, V, PROD-INV) > **dieses Dokument** > Root-Dokumente 00–04 > Overlay-Specs (`REBASE_S1-S14.md`, `REBASE_CSA_AMENDMENTS.md`) > S-Spezifikationen. In der Leseordnung gilt es als Position **25a** (direkt nach dem LOOM-Container-Standard).

> **Start-Prompt für den Agenten (vom Auftraggeber zu senden):**
> „Kontingent ist wieder verfügbar. Lies zuerst `05_RESUME_INFERENCE_AMENDMENT.md` im Repo-Root vollständig. Führe die Bestandsaufnahme aus Teil A durch und lege `reports/RESUME_bestandsaufnahme.md` an. Setze dann die Master-Bauordnung an der ermittelten Phase fort — mit den Overlays aus den Teilen B–G dieses Dokuments. Alle bisherigen Regeln aus 00–04 gelten weiter, präzisiert durch dieses Dokument."

---

## TEIL A — WIEDERAUFNAHME-PROTOKOLL (Pflicht vor jeder neuen Code-Zeile)

**A.1 Bestandsaufnahme.** (1) Lies vorhandene `reports/G*_bericht.md`; fehlen Berichte für begonnene Arbeit, rekonstruiere den Stand aus dem Repo selbst: existierende Crates/Ordner auflisten, `cargo build --workspace` und `cargo test --workspace` ausführen, Ergebnis festhalten. (2) Bestimme die **aktuelle Phase** = die erste Phase aus `01_MASTER_BUILD.md` (inkl. G8a aus Teil F), deren **Ausgangs-Gate nicht vollständig grün** ist. (3) Schreibe `reports/RESUME_bestandsaufnahme.md`:

```
Stand: <Datum> · Letzte abgeschlossene Phase: G<nn> (Nachweis: Tests/Berichte)
Aktuelle Phase: G<nn> · Offene Ausgangs-Gate-Punkte: <Liste>
Halbfertige Einheiten: <Crate/Modul: fortführen | zurücksetzen, Begründung>
Abweichungen vom Spec-Stand: <Liste oder "keine">
```

**A.2 Fortsetzen, nicht neu anfangen.** Grüne Phasen bleiben unangetastet. Halbfertige Einheiten werden entweder zu Ende geführt oder **sauber entfernt** (kein Zombie-Code, kein auskommentierter Restbestand). `spec/` und die Root-Dokumente bleiben read-only; du schreibst nur Code, Tests und `reports/`.

**A.3 Kontingent-Disziplin (ab jetzt verbindlich).** Kleine Commits je Gate-Punkt; nach **jedem** grünen Ausgangs-Gate: Commit + Baubericht. Vor rechenintensiven Schritten (große Testläufe, Golden-Generierung) zuerst committen. So ist jeder künftige Abbruch verlustarm und die Bestandsaufnahme trivial.

---

## TEIL B — DIE HÄRTESTE KORREKTUR (übersteuert 00 §6 und den Verbote-Absatz in 04)

Die Kurzform „Kein Netz außer CSA" wird **präzisiert, nicht gelockert** — CSA bleibt die einzige Source-/API-/Korpus-Akquisitionsschicht, wörtlich wie spezifiziert:

> **Kein Source-Fetch außer CSA. Kein Modell-Egress außer InferenceGateway. Kein Tool-Egress außer ToolGateway. Kein Commit aus Source-, Model- oder Tool-Egress ohne Gate + Evidence + Replay + sichtbares Residuum.**

Das InferenceGateway ist keine Quelle und kein Scraper (Modellantwort = Kandidat, nie Quelle; Quellen gehen durch CSA). Das ToolGateway ist keine Source-Akquisition. Zusatz zu 04: Modell-/Agentenausgaben sind CandidateOutputs — nie Commit, nie Gate-Urteil; Modell-Confidence ist nie ein Gate.

---

## TEIL C — DIE NEUE SCHICHT L9c (normatives Kondensat, in sich vollständig)

**C.0 Einordnung.** Landkarten-Ebene L9 ist die Außenwelt-Ebene mit drei Toren: **L9a** CSA (Quellen) · **L9b** Nexus-Bridge (Port, R-1b) · **L9c** Inference-/Tool-Gateway (Modelle, Agenten, Werkzeuge — diese Schicht). Zielstruktur-Ergänzung (00 §5): Crates `cce-inference`, `cce-toolgateway`.

**C.1 Axiome.** **IG-A1** Drei disjunkte Egress-Capabilities: `source_acquisition` (CSA), `model_egress`, `tool_egress` — kein Pfad bedient zwei Tore. **IG-A2** Modell-/Agentenausgaben sind CandidateOutputs: Eingaben für Gates, nie Urteile/Commits/Ledger-Schreiber. **IG-A3** Eine InferenceResponse ist ein **aufgezeichneter, RD-gebundener Eingang** (wie eine HITL-Entscheidung, S5.4): Replay spielt die Aufzeichnung ein; der Motor bleibt deterministisch; Modell-Nichtdeterminismus wird — nur wo Live-Re-Inferenz verlangt ist — sichtbares Residuum `model_replay_weak`. **IG-A4** Score/Confidence/Ranking/LLM-Selbstbewertung ersetzen nie ein Gate. **IG-A5** Deklaration ≠ Aktivierung: Manifeste/Profile/`.loom`-Segmente deklarieren; aktiviert wird nur lokal unter CapabilityLocks + Operator-Bestätigung; kein Autostart, keine versteckte Inferenz beim Öffnen.

**C.2 InferenceGateway.** Der **einzige** Port zu lokalen oder entfernten Modellen: Provider-Registry (nur manifestierte Provider), Gate-Kette **vor** jedem Egress, Kontext-Schnitt (nur `allowed_context`), Aufzeichnung (Evidence+Trace+Ledger-Anker), Antwort-Typisierung (→ CandidateOutput). Es hat keinen Schreibpfad auf Gates, Urteile, Residuen, Ledger-Verdikte oder Commits.

**C.3 Provider-Klassen (fünf).** `LocalModelProvider` (lokal, kein Egress, stärkstes Replay-Profil) · `CloudModelProvider` (Anbieter-API via `model_egress`; nur mit vollem Manifest inkl. Terms/Privacy/Retention) · `ExternalAgentProvider` (externer KI-/Coding-Agent: erhält **nur** ProjectionPackets/Workcell-Kontext; keine Repo-Freiheit ohne expliziten ToolCapabilityLock) · `EmbeddedSmallModelProvider` (eingebettetes Kleinmodell, kein Egress) · `DisabledProvider/DegradedMode` (kein Provider: Kern + Cockpit voll funktionsfähig, Kanzel-Funktionen sichtbar degradiert).

**C.4 ModelManifest (Pflichtfelder).** `provider_id · provider_class · model_id · model_version · context_window · modality · supported_ops · privacy_mode · data_retention_mode · logging_mode · cost_budget · token_budget · rate_limit · deterministic_settings · replay_policy ∈ {recorded (Default), strict, weak} · provider_terms_ref · safety_boundary · capability_locks`. Ohne valides Manifest bleibt ein Provider passiv.

**C.5 InferenceRequest (Pflichtfelder).** `request_id · run_id/RD-Ref · projection_id · workcell_id · allowed_context · forbidden_context (Prüfvorrang) · output_schema · system_contract · tool_policy · privacy_profile · budget · sampling_policy · seed/determinism controls (soweit möglich) · trace_id`. Egress nur, wenn alle Gates (C.8) grün; der gesendete Kontext-Digest wird in der Evidence fixiert.

**C.6 InferenceResponse (Pflichtfelder).** `response_id · request_id · candidate_output · provider_metadata · model_metadata · token_usage · latency · refusal/error (Modell-Weigerung = regulärer sichtbarer Zustand `model_refusal`, nie stiller Retry mit aufgeweichtem Kontext) · trace · digest · replay_notes`.

**C.7 CandidateOutput & InferenceEvidence.** CandidateOutput: **niemals Commit, niemals Gate-Urteil, niemals Ledger-Schreibzugriff**; einziger Weg: `CandidateOutput → Motor-Gates + Evidence + Replay → PhaseBlock | ResidueReport`; materielle Wirkungen daraus sind ActionCandidates unter CapabilityLock + HumanConfirmationGate. InferenceEvidence bindet Request-Digest (inkl. Kontext-Schnitt), Manifest-Ref, ProjectionPacket-Ref, RD-Ref, Response-Digest, Egress-Gate-Reports, Kosten/Zeit — ohne sie ist ein Kandidat nicht gate-fähig (`model_trace_missing`).

**C.8 Die 14 Pflichtgates** (boolesch, fail-closed, begründet, ledger-/tracefähig; Verdikt allow|hold|reject|quarantine): **ProviderManifestGate** (Manifest vollständig+valide) · **ProviderTermsGate** (`terms_ref` bekannt+kompatibel, sonst `model_terms_unknown`) · **ModelPrivacyGate** (privacy/retention/logging kompatibel zur Lauf-Datenpolicy, sonst `model_privacy_block`/`model_retention_incompatible`) · **PromptContextGate** (Egress ⊆ allowed, ∩ forbidden = ∅, ⊆ ProjectionPacket-Boundary — **vor** jedem Egress; sonst `prompt_context_boundary_violation`/`sensitive_context_egress_blocked`) · **ModelBudgetGate** · **ModelRateGate** · **ModelCapabilityGate** (`model_egress`-Lock offen; supported_ops decken Anfrage) · **ModelReplayGate** (Aufzeichnung vollständig; replay_policy erfüllt) · **OutputSchemaGate** (`model_output_schema_invalid`, kein stilles Umformen) · **NoDirectCommitGate** (strukturell kein Pfad Kandidat→Commit ohne Motor-Gates; Versuch = `model_attempted_direct_commit`, Reject) · **NoGateOverrideGate** (kein Provider-/Kanzel-Pfad schreibt Verdikte/Residuen/Ledger; Versuch = `model_attempted_gate_override`, Reject) · **ToolCapabilityGate** (Tool-Klasse hat offenen Lock im Scope) · **ToolEgressGate** (Remote-Tool explizit erlaubt, budgetiert, aufgezeichnet) · **HumanConfirmationGate** (jede materielle Aktion aus Modell-/Tool-Kandidaten trägt aufgezeichnete Operator-Bestätigung).

**C.9 ModelResidues (16, jedes sichtbar):** `provider_unavailable · provider_manifest_missing · model_context_overflow · model_privacy_block · model_budget_exceeded · model_rate_limited · model_terms_unknown · model_retention_incompatible · model_output_schema_invalid · model_refusal · model_replay_weak · model_trace_missing · model_attempted_direct_commit · model_attempted_gate_override · prompt_context_boundary_violation · sensitive_context_egress_blocked`.

**C.10 ToolGateway.** Einziger Port für Werkzeug-Ausführung, nie stillschweigend. ToolManifest: `tool_id · tool_class ∈ {fs_read, fs_write, shell, git, package_manager, browser, ci, network_tool, custom} · scope · side_effects · egress(none|remote) · budget · replay_strategy · lock_ref`. **Jede Klasse eigener ToolCapabilityLock** — Paketmanager, Git, Browser, Shell, CI, Dateisystem, Netzwerktools sind niemals implizit frei; Remote zusätzlich ToolEgressGate. Tools beschaffen keine Quellen (dafür CSA). **Cursor-/Coding-Agent-Modus** damit möglich ohne Architektur-Öffnung: ExternalAgentProvider (Denken) × ToolGateway (Wirken) × ProjectionPacket (Sicht) × lokale Gates (Urteil).

**C.11 Betriebsmodi.** **Offline Core:** open/inspect/verify/replay ohne Netz; lokale Modelle möglich; Cockpit degradiert ohne Kanzel — Motor/Prüfung/Artefakte vollständig. **Hybrid:** lokaler Kern + Cloud-LLM via Gateway; Abfluss nur durch PromptContext+ModelPrivacy+ProviderManifest(+Budget/Rate/Terms); Antworten nur Kandidaten. **External Agent:** Agent = ExternalAgentProvider; nur ProjectionPackets; Outputs zurück als Kandidaten; Commits lokal gegatet. **Fully Local:** LocalModelProvider, kein Cloud-Egress; Nichtdeterminismus bleibt sichtbares Residuum. Modus/Provider jederzeit im Cockpit sichtbar; Wechsel = materielle Aktion (Bestätigung + Ledger).

**C.12 KI-Kanzel (Client des Gateways, nie eigener Netzpfad).** **Darf:** Wünsche formulieren helfen; ProjectionPackets erklären; CandidateOutputs erzeugen; Residuen erklären; Reparaturvorschläge formulieren; Doku-/Artefaktentwürfe vorschlagen. **Darf nicht:** Gate-Urteile schreiben; Residuen löschen; Ledger direkt beschreiben; PolicyGates überschreiben; CSA umgehen; Toolzugriffe selbst freischalten; harte Gates „trotzdem" passieren — strukturell erzwungen (C.8 NoDirectCommit/NoGateOverride), nicht appellativ.

---

## TEIL D — OVERLAYS AUF S3/S4/S5/S6/S13/S15 (normativ)

**S3-A7..A10 (Cockpit):** ProviderStatus-Ansicht (Modus, provider_id/model_id/version, Kanzel-Zustand, Budget/Rate) · Datenabfluss-Sicht (gesendet/blockiert je Request, mit Grund) · CandidateOutput-Sicht (erzeugendes Modell, Gate-Ergebnisse, Verbleib →PhaseBlock|→Residue, Confirmation-Status; „warum nicht committed" in einem Klick) · COCK-INV-7: kein UI-Pfad aktiviert Provider ohne Bestätigung; COCK-INV-8: Confidence nie als Verdikt dargestellt („Einschätzung, kein Urteil").
**S4-A4 (Wunsch):** Kanzel-Formung läuft nur als InferenceRequests; die Annahmen-Liste vermerkt je Annahme mensch- oder modellgeformt; bindend bleiben nur Motor-Validierung + Bestätigung.
**S5-A5/A6 (Orchestrierung):** InferenceResponses sind bestimmte Eingänge wie HITL: `{request_digest, provider/manifest_ref, response_digest, evidence_ref}` im PhaseBlock-`evidence`; **Replay spielt die Aufzeichnung ein** (kein Live-Re-Call im Replay-Pfad); Live-Re-Inferenz-Abweichung = `model_replay_weak`. Der Runner hat keinen Inference-Direktpfad an den Gates vorbei.
**S6-A4 (Inspektion):** neue Objekte InferenceTrace (Request→Response-Kette), PromptContext-Report (gesendet/blockiert), ProviderStatus — Vier-Ebenen-Drill-down, jede Anzeige ↦ genau ein Artefakt.
**S13-A7..A9 (Governance):** **PROD-INV-17** kein Modell-Egress außer InferenceGateway · **PROD-INV-18** kein Tool-Egress außer ToolGateway (je Klasse eigener Lock) · **PROD-INV-19** kein Provider-/Kanzel-/Agent-Pfad schreibt Urteile/Residuen/Ledger/Commits · **PROD-INV-20** Confidence/Score ersetzt nie ein Gate. Lock-Familie: `model_egress` je Provider-Klasse, `tool_egress` je Tool-Klasse; ExternalAgent zusätzlich an ProjectionPacket-Vertrag (PROD-INV-12); `privacy_profile`/`pii_mode` gelten für Modell-Egress wie für Quellen.
**S15-A4 (Workbench):** Auftrag = `(D, s, HS, q?, IP?)` mit optionalem InferenceProfile; Capsule-`allowedOps` kennt `model_egress`/`tool_egress` als lock-pflichtige Einträge; External-Agent-Betrieb = Capsule-Fall, Promotion bleibt lokal.

---

## TEIL E — .loom-NACHZUG (Kind-Registry minor-additiv; übersteuert LOOM-Standard Teil 3/10 additiv)

| Kind | Name | Rolle (Pflichtfelder-Kern) | core | Profil | Verify |
|---|---|---|---|---|---|
| 0x0060 | PROVIDER_MANIFEST | deklarierte ModelManifeste (C.4) — Deklaration, nie Aktivierung | core | Pflicht wenn 0x0061 vorhanden | L2: kein Autostart-/Aktivierungsfeld zulässig |
| 0x0061 | INFERENCE_PROFILE | Provider-Klassen/Budgets je Workcell; Modus-Empfehlung | core | runtime/full bei Inferenz | L2: Manifeste referenziert+vorhanden |
| 0x0062 | INFERENCE_TRACE | InferenceEvidence-Ketten (Digests, Gate-Reports, Kosten) | core | runtime/full | L2: jede Response evidence-gebunden |
| 0x0063 | CANDIDATE_OUTPUTS | transportierte Kandidaten + Verbleib | core | optional | L2: Evidence-Ref je Output; keiner als Commit markiert |
| 0x0064 | TOOL_PROFILE | ToolManifeste + geforderte Locks (deklarativ) | core | runtime/full bei Tools | L2: keine implizite Freigabe |

Capability-Aufspaltung: `network_request` → `source_acquisition | model_egress | tool_egress` (Altdateien: konservativ keine der drei ohne Neu-Deklaration aktivierbar; read-kompatibel, sichtbar). Öffnungs-Härtung: der Seiteneffektfrei-Test von open/inspect/verify umfasst „kein Modell-/Tool-Egress". Zeugen: **R8** Deklariert-nicht-aktiviert-Container (valid) · **N15** `hidden_model_call_on_open` (reject) · **N16** `provider_autostart_flag` (reject) → .loom-Zeugen gesamt **24**.

---

## TEIL F — BAUORDNUNG-OVERLAY (übersteuert 01/02)

**F.1 Neue Phase G8a** (nach G8, vor G9): Crates `crates/cce-inference/` (gateway, manifest, request/response/evidence, residue, gates, providers/{local, cloud-mock, external_agent, embedded, disabled}, replay-recorded) und `crates/cce-toolgateway/` (manifest, gateway). Ports: cce-core, cce-kernel, cce-runner, cce-store, S13-Locks; **keine** Abhängigkeit von cce-nexus-* (Tor-Trennung im Crate-Graph; Verbots-Test: kein HTTP-Pfad außerhalb providers/cloud). **Reihenfolge a–j:** Typen+Manifest+Residuen → Gate-Kette (PromptContextGate strikt vor jedem Egress-Pfad) → DisabledProvider + Offline-Nachweis (voller Motor-/Produktpfad ohne Provider) → LocalModel-Echtpfad (deterministischer Stub zulässig, replay_policy deklariert) → Cloud-**Mock** hinter ModelCapabilityGate → ExternalAgent (ProjectionPacket, Mock-Agent) → recorded-Replay (Runner spielt Responses ein; Klassenidentität) → ToolGateway (ein lokales Referenz-Tool fs_read; Remote nur Mock hinter ToolEgressGate) → 19 Zeugen in den Wächter → schreiblose Kanzel-API (form/explain/propose) für G10. **Ausgangs-Gate G8a:** kein Modell-/Tool-Socket außerhalb der Gateways (Dependency-/Symbol-Scan + Laufzeit-Negativtest) · 14 Gates einzeln getestet, fail-closed · Offline Core Mode grün · recorded-Replay klassenidentisch · 7 R-INF grün + 12 N-INF rot im Wächter · PROD-INV-17..20 negativ-getestet · Deklaration≠Aktivierung nachgewiesen. **Verbote:** kein Provider-SDK außerhalb providers/; kein „bei Gate-Fail trotzdem senden"; kein stilles Kontext-Kürzen (Overflow ⇒ Hold); keine Sammel-Freischaltung von Tool-Klassen; **keine realen Cloud-Anbieter anzubinden** (Betriebsschritt, F.3).
**F.2 Meilenstein:** `G8a | Modelle & Werkzeuge gezähmt | kein Egress außer Gateways; Kandidat nie Commit`. G9 nimmt die Kinds 0x0060–0x0064 auf; G10 verdrahtet die vier Cockpit-Ansichten + Kanzel-API.
**F.3 MASTER-DoD-Deltas:** DoD-Matrix **a–i** mit **i = InferenceGatewayDoD** (Gateway einziger Port, 5 Provider-Klassen, 14 Gates, 19 Zeugen, recorded-Replay, Offline ungebrochen) · Negativ-Test-Umfang PROD-INV-**9..20** · Wächter-Zeile: `+ .loom 8R+16N ∧ Inference 7R+12N` · Checkliste: neue Zeile „G8a Kein Egress außer Gateways · 14 Gates · 19 Zeugen · Offline Core ungebrochen · recorded-Replay · Kanzel-API schreiblos"; G9-Zeile: „24 Zeugen (inkl. R8/N15/N16)" · **§3 Nicht-Bestandteile ergänzt:** Live-Cloud-Provider-Vollintegration (Mocks + LocalModel-Echtpfad = Bau-Pflicht; Anbieter-Onboarding = Betrieb) und ExternalAgent-Produktivbetrieb (implementiert + mock-getestet; realer Agent = Betrieb).

---

## TEIL G — ZEUGEN (Conformance-Kondensat; Ablage `conformance/inference/`, ab G8a dauerhaft im einen Wächter)

**Referenzen (grün):** R-INF-1 OfflineDisabledProvider (voller Motor-+Produktpfad ohne Provider; Kanzel sichtbar degradiert) · R-INF-2 LocalModelProviderMock (seed-deterministisch; Manifest/Evidence/Trace vollständig; replay=strict) · R-INF-3 CloudProviderMock+PrivacyGate-Pass (Egress erst nach allen Gates; Kontext-Digest = gesendet) · R-INF-4 ExternalAgent+ProjectionPacket (kein Zugriff außerhalb Packet) · R-INF-5 Kandidat→Gate→PhaseBlock (Commit trägt InferenceEvidence-Ref + Confirmation) · R-INF-6 Kandidat→Residue (`model_output_schema_invalid`; Verbleib lesbar) · R-INF-7 ProviderStatus-Datenmodell (headless gerendert, wurzel-rückführbar).
**Negative (rot, am definierten Prüfpunkt):** N-INF-1 provider_manifest_missing (Hold, kein Egress) · N-INF-2 cloud_egress_without_privacy_gate (struktureller Reject **vor** Socket) · N-INF-3 model_attempted_direct_commit · N-INF-4 model_attempted_gate_override · N-INF-5 forbidden_context_sent_to_provider (kein Egress) · N-INF-6 model_output_schema_invalid · N-INF-7 provider_terms_unknown (Hold) · N-INF-8 model_budget_exceeded (Hold vor Egress) · N-INF-9 external_agent_requests_full_repo_without_lock · N-INF-10 tool_egress_without_tool_capability · N-INF-11 hidden_model_call_on_open (Format-Reject; = LOOM-N15) · N-INF-12 model_confidence_used_as_gate (PROD-INV-20).

---

## TEIL H — ABNAHME DIESES OVERLAYS

`DoD(05-Overlay) = 1 ⟺` Bestandsaufnahme liegt vor (A.1) ∧ Bau an der ermittelten Phase fortgesetzt ∧ Egress-Vierteilung als Bau-Invariante nachgewiesen (Teil B) ∧ G8a mit Ausgangs-Gate grün (F.1) ∧ .loom-Kinds + 3 Zeugen ab G9 (Teil E) ∧ Cockpit-/Kanzel-Overlays in G10 (Teil D) ∧ MASTER-DoD in der F.3-Fassung erfüllt ∧ alle 19 Inference-Zeugen dauerhaft im Wächter. — *CSA holt Quellen. InferenceGateway ruft Modelle. ToolGateway ruft Werkzeuge. Kein Commit ohne Gate + Evidence + Replay + sichtbares Residuum.*

*Ende der Einzeldatei.*
