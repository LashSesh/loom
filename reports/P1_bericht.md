Etappe P1 — Frontier-Intelligenz real: CloudModelProviderOpenAI
(Dokument 17 §3, Overlay-Klausel: OpenAI statt Anthropic)

Eingang: Etappe X4 abgeschlossen und angenommen (`reports/X4_bericht.md`,
Ausgangs-Gate X4 ERFÜLLT). Direkter Anschluss nach Auftrag, kein neuer
Prompt. Overlay-Klausel zu Dokument 17 §3: der erste echte
CloudModelProvider ist OpenAI (nicht Anthropic) — Auftraggeber-Weisung.

## Was gebaut wurde

Ein neuer Provider `CloudModelProviderOpenAI`
(`crates/cce-inference/src/providers/openai.rs`) implementiert dasselbe
`ModelProvider`-Trait wie jeder andere Provider — keine Sonderbehandlung,
kein neuer Pfad am Gateway vorbei. Er läuft ausschließlich durch das
**unveränderte** `run_inference()` (dieselbe Vor-Egress-Gate-Kette:
ProviderManifestGate, ProviderTermsGate, ModelPrivacyGate,
PromptContextGate, ModelBudgetGate, ModelRateGate, ModelCapabilityGate,
ModelReplayGate, NoDirectCommitGate, NoGateOverrideGate — zehn Gates,
unverändert aus G8a).

**Manifest (vollständig, alle vom Auftrag genannten Felder real gesetzt):**
`ProviderClass::CloudModel`, `provider_terms_ref =
"terms:known_compatible:openai"`, `privacy_mode = "no_training_use"`,
`data_retention_mode = "zero_retention"`, `cost_budget`/`token_budget`
gesetzt, `replay_policy = ReplayPolicy::Recorded` (Auftraggeber-Weisung
wörtlich erfüllt), `capability_locks = ["model_egress:cloud-openai"]`
(CloudModel-Klasse verlangt den Lock — `ModelManifest::validate()` prüft
das strukturell).

**Schlüssel-Disziplin (S13):** `OPENAI_API_KEY` wird ausschließlich via
`std::env::var("OPENAI_API_KEY")` **innerhalb** von `infer()` gelesen —
kein struct-Feld, keine Datei, kein Log, kein Debug/Display-Pfad, keine
Rückgabe. Der Schlüssel verlässt die eine Anfrage nie.

**Netzwerk-Disziplin:** der reale HTTP-Pfad (POST
`https://api.openai.com/v1/chat/completions` via `ureq`, JSON via
`serde_json`) existiert **nur** unter dem opt-in-Feature `http`
(`crates/cce-inference/Cargo.toml`, Standard AUS) — dieselbe Disziplin wie
`nexus_fetch::HttpTransport`. CI baut und testet ohne dieses Feature,
bleibt also strukturell netzfrei. `ci/check_acyclic.py`s Socket-Scan (G8a)
lässt die `ureq`-Referenz ausschließlich unter
`crates/cce-inference/src/providers/` zu — genau dort liegt sie.

**Saubere Degradation ohne gesetzten Schlüssel/Feature:** ohne das
Feature `http` (Bau-Default) oder ohne gesetzten `OPENAI_API_KEY`
antwortet der Provider *immer* mit einer sichtbaren
`ResponseOutcome::Error("provider_unavailable: ...")` — kein stiller
Fallback-Inhalt, kein Socket-Versuch (das Feature ist nicht einmal
kompiliert, wenn es aus ist). Dieselbe Disziplin wie `DisabledProvider`.

## Zeugen (`conformance/inference/inference_catalog.rs`, Sektion „P1")

Drei neue Tests, alle im netzfreien Bau-Default grün:

1. `p1_cloud_openai_manifest_complete_terms_privacy_retention_budget` —
   Manifest validiert, `ProviderClass::CloudModel`,
   `provider_terms_ref`/`privacy_mode`/`data_retention_mode` gesetzt und
   kompatibel, `cost_budget`/`token_budget` vorhanden,
   `replay_policy == Recorded`, `capability_locks` nicht leer.
2. `p1_cloud_openai_without_key_or_feature_degrades_after_full_gate_chain_no_socket`
   — mit offenem Lock läuft die volle Vor-Egress-Gate-Kette durch (alle
   zehn Gates allow), der Provider selbst meldet sichtbar
   `provider_unavailable` — kein Socket-Versuch, kein stiller Inhalt.
3. `p1_cloud_openai_egress_blocked_before_provider_without_open_lock` —
   ohne offenen `model_egress`-Lock hält `ModelCapabilityGate` VOR jedem
   Providerkontakt (dieselbe strukturelle Garantie wie beim Mock,
   R-INF-3, jetzt für den echten Anbieter).

`conformance::GUARD_PHASES += "P1"`, `FEATURE_PL += cloud_openai_provider`
(PL2, Evidence: obiger erster Zeuge).

## CI GRUEN

`cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D
warnings` (Default-Build UND mit `--features http` einzeln geprüft),
`python3 ci/check_acyclic.py` (55 Workspace-Crates, DAG sauber,
Socket-Scan sauber — `ureq` nur unter `providers/` erlaubt),
`bash ci/run_ci.sh` — alle grün. 176 Testgruppen (vorher 175), 0
Fehlschläge. Alle Alt-Zeugen unangetastet, insbesondere
`conformance/inference/inference_catalog.rs`s bestehende 27 Tests
weiterhin grün.

## Wo und wie der API-Schlüssel einzutragen ist

**Genau eine Stelle, genau einmal:** die Umgebungsvariable
`OPENAI_API_KEY` in der Laufzeitumgebung dieser Agent-Session (z. B. als
Secret der Ausführungsumgebung / des CI-Runners, je nachdem wo ein
`CloudModelProviderOpenAI` mit dem Feature `http` tatsächlich betrieben
werden soll). **Nirgendwo sonst:**

- nicht ins Repository, nicht in eine `.env`-Datei im Repo, nicht in
  einen Commit, nicht in einen Report — der Code liest die Variable
  ausschließlich zur Laufzeit aus dem Prozess-Environment;
- das Bauen/Testen selbst (`cargo build`/`cargo test`/`ci/run_ci.sh`)
  braucht den Schlüssel **nicht** — das Feature `http` ist dort aus,
  der Provider degradiert sauber ohne jeden Netzzugriff;
- um den realen Anbieter tatsächlich zu benutzen, müssten zusätzlich
  BEIDE Bedingungen erfüllt sein: das Cargo-Feature `http` müsste an
  der aufrufenden Stelle aktiviert werden (`cargo build --features
  cce-inference/http` bzw. eine entsprechende Abhängigkeitszeile), UND
  `OPENAI_API_KEY` müsste in der Prozessumgebung gesetzt sein. Ohne
  beides bleibt der Pfad so, wie er im Bau immer läuft: netzfrei,
  sichtbar degradiert.

Diese Zweiteilung (Feature UND Schlüssel) ist bewusst: sie stellt sicher,
dass ein versehentlich gesetzter Schlüssel allein nichts auslöst, solange
niemand das Feature explizit für einen echten Lauf einschaltet — ein
zusätzliches, strukturelles Sicherheitsnetz über die reine
Secret-Hygiene hinaus.

## Damit teilweise aufgelöst

Dokument 17 §5: „Host-Leiste: durch P1 teilaufgelöst" — der GGUF-/lokale-
LLM-Teil der Host-Leiste braucht keinen eigenen Host-Termin mehr, sobald
ein Betreiber Feature+Schlüssel setzt; das bleibt ausdrücklich eine
Betriebsentscheidung (WO-4/5-Klasse), keine weitere Bau-Residuum.

## Residuen

Keine neuen Bau-Residuen. Die reale Erprobung gegen die tatsächliche
OpenAI-API (Antwortformat-Feinschliff über die hier abgedeckten Felder
hinaus, Streaming, Retry-/Backoff-Politik, konkrete Modell-ID-Wahl) ist
ein Betriebsschritt, sobald der Auftraggeber Feature+Schlüssel aktiviert
— explizit vorgemerkt, kein blockierendes Residuum für P1 selbst
(Dokument 17 §3 verlangt genau: Manifest + Gates + recorded-Replay hinter
dem unveränderten Gateway, real gebaut).

## Nächster Schritt

P2 · SWE-Tiefe (Dokument 17 §3): Software-Familie von PL3-Struktur auf
Arbeitsreife — Repo-Workbody, Tool-Klassen fs/git/build/test real unter
ToolCapabilityLocks.

Abweichungen: keine.
