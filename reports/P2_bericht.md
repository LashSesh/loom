Etappe P2 — SWE-Tiefe: Software-Arbeit unter Gates
(Dokument 18, Vorstufe zum Dogfooding P3)

Eingang: Etappe P1 abgeschlossen und angenommen (`reports/P1_bericht.md`,
inkl. Betriebsverifikation gegen `gpt-4o-mini`), Residuum R-Agent-15
(fehlende P2-Spezifikationsdatei) durch Nachlieferung von Dokument 18
(`cce-spec-repo/18 P2 SWE TIEFE SPEC.md`) GESCHLOSSEN. Direkter
Anschluss nach Auftrag, kein neuer Prompt. Gebaut exakt nach Dokument 18
§9, Reihenfolge a→h.

## Was gebaut wurde

### a) Neues Blatt-Crate `crates/cce-swe`

Objektmodell (§2), ohne die Container-Serialisierung anzufassen:

- **CodeUnit/RepoSnapshot** (`model.rs`): `(path, language,
  content_digest, role)` je Einheit; ein Snapshot ist eine
  deduplizierte, nach Pfad sortierte Menge von CodeUnits + Toolchain-Pin
  — die Merkle-Wurzel (`snapshot_root()`) ist deterministisch,
  unabhängig von der Eingabereihenfolge, und ändert sich mit dem
  Toolchain-Pin (§5: andere Compiler-Version = anderer Snapshot). Der
  TYPE_REGISTRY-Eintrag `unit:code` (additiv/minor, §6) IST dieser Typ +
  seine `as_str()`-Abdeckung — dieselbe Disziplin wie `unit:table`
  (`cce_materialize::document::UnitType`), kein separates
  Registry-Objekt.
- **DiffCandidate/DiffHunk** (SWE-A1): trägt nie einen Commit-Marker.
  Ein echter, minimaler unified-diff-Hunk-Applier (`apply_unified_diff`)
  wendet Hunks an — kein Fuzzy-Match: jede Kontext-/Entfernungszeile
  muss exakt zum Original passen, sonst `Err` (fail-closed).
- **BuildRun/TestRun** (SWE-A2, ToolEvidence): Fakt, nie Interpretation
  — `exit_code`/`log_digest`/`duration`/`artifacts`.
- **TaskLedger**: die `PhaseBlock`-Kette EINES Bauauftrags, an eine RD
  gebunden (`Ledger=CommitProjection` wie überall, hier auf einen Task
  verengt).

### b) Die fünf Werkzeugklassen real im ToolGateway

`crates/cce-toolgateway` (unverändertes Tor, L9c) bekommt vier neue
Klassen neben dem bestehenden `fs_read` (`TOOL_CLASSES` 9→11:
`build`/`test` additiv):

- **fs_write**/`run_fs_write`: schreibt nur in die deklarierte
  Arbeitskopie (In-Memory-Fixture im Bau-Default, dieselbe
  Hermetik-Disziplin wie `FsReadTool`), Scope-Prüfung identisch zu
  `run_fs_read`.
- **git**/`run_git` (`GitOperation`: status/diff/add/commit/branch/push):
  `commit`/`push` sind materielle Aktionen und verlangen einen
  aufgezeichneten `confirmation_ref`; `push` verlangt zusätzlich
  `Egress::Remote` — kein Egress ohne explizite Deklaration.
- **build**/**test** (`BuildTool`/`TestTool`, `run_build`/`run_test`):
  ein typisierter Programmaufruf (`argv: Vec<String>`, NIE ein
  Shell-String — §3-Verbot "kein Shell-Sammelzugriff" strukturell
  erzwungen) mit Fixture-Ergebnis im Bau-Default.

Reale Prozess-Aufrufe (git/build/test) liegen NUR unter dem neuen
opt-in-Feature `process` (Standard AUS, dieselbe Disziplin wie `http`
in `cce-inference`) hinter `std::process::Command` mit festem `argv`.
Default-CI bleibt hermetisch: kein echter Subprozess läuft in
`cargo test --workspace`/`bash ci/run_ci.sh`.

Kleine Begleitänderung: `ToolGateway::is_locked` (öffentliche, lesende
Lock-Abfrage) — der `CapabilityLock` selbst bleibt privat je Klasse
(keine Sammel-Freigabe), die SWE-Gate-Komposition braucht aber einen
Weg, den Lock-Zustand für die eigene `GateReport`-Kette abzufragen.

### c) Die sechs Werkzeug-Gates

`cce-swe/src/gates.rs`, alle in der `InfVerdict`-Form der 14
Inference-Gates (C.8):

- **Neu (4):** `ToolCapabilityGate` (`tool_capability_denied`, Dokument
  18 §4 wörtlich), `ToolScopeGate` (`tool_scope_violation`),
  `BuildEvidenceGate` (`build_unverified`), `TestEvidenceGate`
  (`tests_unverified`/`tests_red`, je nach Fehlerbild), `RegressionGate`
  (`regression_detected`).
- **Wiederverwendet (3), unverändert aus `cce_inference::gates`:**
  `ToolEgressGate`, `HumanConfirmationGate`, `NoDirectCommitGate` — kein
  neues Tor, keine Duplikate derselben Policy.

### d) Die Kern-Kette (dry-run)

`cce-swe/src/kette.rs::run_swe_task`, analog zu
`cce_inference::gateway::run_inference`: `DiffCandidate →
apply(dry-run) → BuildRun → TestRun → BuildEvidenceGate ∧
TestEvidenceGate ∧ RegressionGate → PhaseBlock | Hold`. Die volle
Vor-Wirkungs-Gate-Kette (`ToolCapabilityGate` je benötigter Klasse +
`ToolScopeGate` je Hunk) läuft VOR jedem Apply/Build/Test — kein Sprung
überspringbar. `git commit`/`push` sind bewusst NICHT Teil dieser
Funktion: eine separate, spätere Aktion über `ToolGateway::run_git`
(das die `HumanConfirmationGate`-Aufzeichnung bereits selbst erzwingt).

### e) Provider-erzeugte Diff (R-SWE-5, recorded)

`cce-swe/src/provider_diff.rs`: `RecordedOpenAiDiffProvider` läuft
durch dasselbe unveränderte `run_inference()` wie jeder andere Provider
— Manifest trägt den echten `provider_id = "cloud-openai"` und
`ProviderClass::CloudModel`, alle zehn Vor-Egress-Gates aus P1
unangetastet. Bau-Default bleibt hermetisch: eine aufgezeichnete
Fixture-Antwort statt Live-Netz — der reale `CloudModelProviderOpenAI`
aus P1 (Feature `http`) bleibt für den echten Betriebsfall vollständig
unangetastet. Ohne offenen `model_egress`-Lock hält `ModelCapabilityGate`
VOR jedem Providerkontakt (dieselbe Garantie wie P1/R-INF-3).

### f) Replay (R-SWE-3)

Zwei vollständig unabhängige Läufe (frisches `ToolGateway`/
`FsWriteTool`) desselben Auftrags liefern identische
`PhaseBlock`-Payload-Klasse + identischen Snapshot-Root nach Apply.
Für den Provider-Pfad zusätzlich: ein zweiter "Lauf" liest die
Aufzeichnung über `cce_inference::replay::replay_response` ein, OHNE
`infer()` ein zweites Mal aufzurufen (S-A5/A6: kein Live-Re-Call im
Replay-Pfad).

### RepoWorkbody-Container (§6, Teil von a)

`cce-swe/src/workbody.rs::seal_repo_workbody`: ein zertifizierter
Bauauftrag IST ein `.loom`-Workbody der neuen Containerklasse `"repo"`
(additiv in `loom_format::PROFILES`, 7→8; `required_kinds("repo")` in
`loom-verify` additiv erweitert um `CL_SUBSTRATE`/`LEDGER`/`RESIDUE`/
`EVIDENCE`/`REPLAY_MANIFEST`/`CANDIDATE_OUTPUTS`/`TOOL_PROFILE`,
dieselbe Disziplin wie die `"norm"`-Klasse aus `cce-bridge`). **Kein
neues Segment nötig:** CodeUnits als `ARTIFACT`+`CAS_BLOB` (der
Container trägt die materialisierten Bytes selbst, dieselbe Disziplin
wie X1a), TaskLedger im `LEDGER`, ToolEvidence im `EVIDENCE`, der
DiffCandidate in `CANDIDATE_OUTPUTS` (0x0063), Tool-Deklarationen in
`TOOL_PROFILE` (0x0064, ohne `enabled`/`implicit_grant` — Deklaration ≠
Aktivierung). `crates/cce-swe` bekam dafür dieselben benannten
`.loom`-Ports wie `cce-bridge` (`ci/check_acyclic.py`s
`ALLOWED_CORE_TO_OUTER`).

## Zeugen (`conformance/swe/swe_catalog.rs`, 14 Tests, dauerhaft im Wächter)

**Referenzen (grün):** R-SWE-1 (Snapshot-Determinismus) · R-SWE-2 (Diff
behebt failing test → volle Kette → PhaseBlock → RepoWorkbody `verify
== Valid`) · R-SWE-3 (Replay-Determinismus) · R-SWE-4 (git commit
verlangt aufgezeichnete Bestätigung) · R-SWE-5 (Provider-Diff durch die
volle Kette).

**Negative (rot):** N-SWE-1 (`build_unverified`, kein PhaseBlock) ·
N-SWE-2 (`tests_red`, Hold) · N-SWE-3 (`tool_scope_violation`, Reject
vor jedem Apply) · N-SWE-4 (`tool_capability_denied`, Reject) · N-SWE-5
(`RegressionGate` reject) · N-SWE-6 (push ohne `ToolEgressGate`,
reject) · N-SWE-7 (Modell-Confidence kann `BuildEvidenceGate`
strukturell nicht ersetzen — die Gate-Signatur nimmt gar kein
Konfidenz-Argument entgegen, PROD-INV-20) · N-SWE-8 (direkter
Commit-Versuch am Gate vorbei, `model_attempted_direct_commit`).

Zusätzlich innerhalb `crates/cce-swe` selbst: 10 Unit-/Integrationstests
(Modell, Residuen, Provider-Diff, volle Pipeline inkl. Container-Siegel
+ `loom_verify::verify`), die dieselben Pfade auf Implementierungsebene
absichern.

## CI GRÜN

`cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets --
-D warnings` (Default-Build UND `--features process` für
`cce-toolgateway`/`cce-swe`/`cce-conformance` einzeln geprüft),
`python3 ci/check_acyclic.py` (56 Workspace-Crates, DAG sauber,
Tor-Trennung + Socket-Scan sauber), `bash ci/run_ci.sh` — alle grün,
weiterhin netzfrei und ohne echte Subprozesse im Default. Alle
Alt-Zeugen (P1/X4/Ring E5/…) unangetastet.

## Wo und wie das Feature `process` einzuschalten ist

Genau wie bei P1s `http`-Feature: `cargo test -p cce-conformance
--features process ...` (bzw. `--features process` an
`cce-toolgateway`/`cce-swe` direkt) aktiviert den realen
`std::process::Command`-Pfad für git/build/test — Standard bleibt AUS.
Das Bauen/Testen selbst braucht das Feature nicht; ohne Feature
degradiert jede Klasse sauber auf ihre Fixture (dieselbe
Zweiteilungs-Disziplin wie P1: Feature UND — wo nötig — offener Lock
müssen BEIDE vorliegen, kein einzelner Schalter allein löst je Egress
aus).

## Ausgangs-Gate P2 (Dokument 18 §9) — erfüllt

- §2–§8 implementiert: Objektmodell, fünf Werkzeugklassen +
  Locks, sechs Werkzeug-Gates, Kern-Kette (dry-run-Disziplin),
  Provider-Diff (recorded), Replay, 13 Zeugen im Wächter. ✓
- Alt-Zeugen unverändert grün. ✓
- CI hermetisch grün (kein Netz, keine echten Prozess-Aufrufe im
  Default). ✓
- Kein neues Kern-Crate mit externen Abhängigkeiten (Prozess-/git-Kisten
  nur im Blatt `crates/cce-swe`/`crates/cce-toolgateway`, kein neuer
  externer Cargo-Dependency überhaupt — `std::process::Command` ist
  Standardbibliothek). ✓
- `reports/P2_bericht.md` (dieser Bericht). ✓

**DoD(P2) = 1:** ein RepoWorkbody eines Referenz-Repos (die
"Taschenrechner-Crate mit einem failing test" aus §8) ist real
zertifiziert (`R-SWE-2`, `verify == Valid`) UND die modellerzeugte Diff
(`R-SWE-5`) hat die volle Gate-Kette durchlaufen.

## Residuen

Keine neuen Bau-Residuen. Betriebsschritte, die bewusst nicht Teil des
P2-Bauumfangs sind (Dokument 18 §9 verlangt genau: Objektmodell + Gates
+ Kern-Kette + Provider-Diff + Replay + Zeugen, real gebaut, hermetisch):

- Eine reale, manuelle Betriebsverifikation des Features `process`
  gegen ein echtes lokales Referenz-Repository (echtes `cargo
  build`/`cargo test`/`git commit`) — analog zu P1s
  Betriebsverifikation gegen die echte OpenAI-API — ist ein natürlicher
  Folgeschritt, sobald ein Betreiber ihn anfordert; explizit
  vorgemerkt, kein Blocker für P2 selbst.
- Eine echte, live gegen die tatsächliche OpenAI-API laufende
  Diff-Erzeugung (statt der hermetischen `RecordedOpenAiDiffProvider`-
  Fixture) bleibt ebenfalls ein Betriebsschritt (Feature `http` +
  `OPENAI_API_KEY`, unverändert aus P1).

## Nächster Schritt

P3 · Dogfooding-Meilenstein (Prototyp-Kerntest, K9): CCE führt einen
echten, kleinen Bauauftrag AM EIGENEN Repository aus. Spezifikation
folgt vom Auftraggeber, sobald P2 angenommen ist (ausdrücklich NICHT
eigenständig begonnen).

Abweichungen: keine.
