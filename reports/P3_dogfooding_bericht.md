Etappe P3 — Dogfooding: der Prototyp-Kerntest (K9)
(Dokument 19, direkter Anschluss nach Etappe P2)

Eingang: Etappe P2 abgeschlossen und angenommen (`reports/P2_bericht.md`).
Dokument 19 (`cce-spec-repo/19 P3 DOGFOODING SPEC.md`) vollständig
gelesen, insbesondere §1 (Klarstellung: das gebaute System CCE selbst —
seine eigene Kanzel, sein eigenes `cce-swe`, sein eigenes ToolGateway —
erzeugt den Diff; der äußere Coding-Agent verdrahtet und beobachtet nur)
und §2 (Schutzzone). Gebaut exakt nach Dokument 19 §8, Reihenfolge a→g.
Nach d) wurde angehalten und EIN konkreter TaskProposal vorgelegt; erst
nach ausdrücklicher Freigabe des Auftraggebers liefen e)–g).

## a)–d): Bauteile (`crates/cce-dogfood`)

Neues Blatt-Crate, verdrahtet ausschließlich bestehende P1/P2-Bausteine
(keine neue Kern-Logik, keine `loom-*`-Laufzeitabhängigkeit — der
RepoWorkbody-Bau bleibt exklusiv `cce_swe::workbody`):

- **TaskProposal** (`proposal.rs`): `(proposal_id, wish_text, task_class,
  target_scope, estimated_cost_budget, rationale)`. `task_class` ist
  bewusst ein String + Laufzeit-Whitelist (`["doc", "additive_test",
  "lint_fix"]`), nicht ein Enum — sonst wäre N-DOG-1 (eine Klasse
  außerhalb der Whitelist) durch Typkonstruktion unmöglich statt ein
  echter Laufzeit-Reject. **OperatorDecision** (`Approved`/`Rejected`)
  + **DogfoodRun** (bindet Proposal + Branch + Entscheidung + TaskLedger
  aus P2).
- **ProtectedPathFence** (`fence.rs`): `path_allowed(path) -> bool` gegen
  Dokument 19 §2 (`spec/`, `cce-spec-repo/`, alle Gate-tragenden
  Kern-Crates, `cce-inference`/`cce-toolgateway` komplett — konservativ,
  da `cce-toolgateway` keine eigene `gates.rs` hat, `loom-format/codec/
  verify`, `ci/`, jede Workspace-`Cargo.toml`). Konservativ ergänzt:
  schützt auch sich selbst und die P2-Kern-Kette/-Gates/-Workbody-Datei
  in `cce-swe` — die Dogfooding-Infrastruktur darf nicht ihr eigenes
  Ziel sein.
- **Vier neue Gates** (`gates.rs`, `InfVerdict`-Form wie überall):
  `TaskProposalGate`, `ProtectedPathGate`, `BranchIsolationGate`
  (ausschließlich `dogfood/p3-*`, `main` immer Reject),
  `MergeExclusionGate` (strukturell — fängt nur den deklarativen
  Versuch, dieselbe Form wie `no_direct_commit_gate`).
- **snapshot_for_scope** (`snapshot.rs`): jede Datei außerhalb des
  freigegebenen `target_scope` ⇒ `protected_path_violation`;
  `RepoSnapshot::from_files` nimmt ohnehin nur explizit benannte Dateien
  entgegen (kein Verzeichnis-Scan existiert in `cce-swe`) — keine
  Vollspiegelung des 57-Crate-Workspace, hier auf eine einzelne, klar
  benannte Datei beschränkt.
- **run_dogfood_task** (`kette.rs`): `Rejected`-Entscheidung hält SOFORT
  (R-DOG-2-Prinzip, kein Gate, kein Werkzeug berührt); sonst die drei
  neuen Vor-Gates + Scope-Check je Diff-Hunk, dann die UNVERÄNDERTE
  P2-Kern-Kette (`cce_swe::kette::run_swe_task`) unangetastet
  wiederverwendet.

16 interne Tests (alle grün im Bau-Default); Details/Commit:
`c304562`.

## Der vorgelegte und freigegebene Vorschlag

**TaskProposal 001** (Auftraggeber-Freigabe im Chatverlauf):

- **Aufgabenklasse:** `additive_test`
- **Zielpfad:** `cce/crates/cce-swe/src/model.rs` (ausschließlich
  innerhalb des bestehenden `#[cfg(test)] mod tests`)
- **Wunsch:** „Füge in `crates/cce-swe/src/model.rs` einen
  zusätzlichen, rein additiven Unit-Test hinzu, der beweist, dass
  `apply_unified_diff()` auch einen unified diff mit mehreren, nicht
  zusammenhängenden Hunks in derselben Datei korrekt anwendet."
- **Branch:** `dogfood/p3-001` (niemals `main`)
- **Budget:** 1

## e) Der eine reale Lauf (R-DOG-1)

Datum: 2026-07-04. Test: `crates/cce-dogfood/tests/
r_dog_1_betriebsverifikation.rs::r_dog_1_real_task_runs_full_chain_on_isolated_branch`
(`#[ignore]`, Konvention: `cargo test -p cce-dogfood --features process
--features cce-inference/http --test r_dog_1_betriebsverifikation --
--ignored --nocapture`). Modell: `gpt-4o-mini` (echter
`CloudModelProviderOpenAI` aus P1, unverändertes Gateway, Feature
`http` real gegen `https://api.openai.com` aktiviert).

### Ablauf der Kette (§4), Station für Station

1. **Kanzel formt den DiffCandidate:** ein realer `run_inference()`-Lauf
   (alle zehn Vor-Egress-Gates aus P1 unverändert durchlaufen) lieferte
   den Kanzel-generierten unified diff.
2. **apply (unverändert aus P2, `apply_unified_diff`):** real gegen den
   tatsächlichen Inhalt von `cce/crates/cce-swe/src/model.rs`
   angewendet.
3. **BuildRun + TestRun ECHT** (Feature `process`, `cce-toolgateway`
   unverändert): `cargo build -p cce-swe` und `cargo test -p cce-swe`
   als echte Unterprozesse.
4. **Alle P2-Gates + die drei neuen P3-Gates:** TaskProposalGate,
   ProtectedPathGate, BranchIsolationGate (alle Allow, vor jedem Apply
   geprüft) sowie ToolCapabilityGate/ToolScopeGate (Allow) und
   BuildEvidenceGate/TestEvidenceGate/RegressionGate (alle Allow, nach
   dem echten Build/Test).
5. **PhaseBlock → RepoWorkbody:** Accept-8 erfüllt (`AcceptOutcome::
   Accepted`), RepoWorkbody der Containerklasse `"repo"` real versiegelt,
   `loom_verify::verify() == Verdict::Valid`.
6. **Replay:** ein zweiter, unabhängiger Lauf über
   `cce_inference::replay::replay_response` (dieselbe aufgezeichnete
   Antwort, KEIN zweiter Live-Aufruf, S-A5/A6) ergab dieselbe
   PhaseBlock-Payload-Klasse und denselben Snapshot-Root.
7. **HumanConfirmationGate + echter `git commit`:** über
   `ToolGateway::run_git` (unverändert aus P2) mit aufgezeichneter
   Bestätigungsreferenz.
8. **Kein Merge, kein Push nach `main`.**

### Ergebnis

- **Build:** `cargo build -p cce-swe` → exit 0.
- **Test:** `cargo test -p cce-swe` → exit 0, alle 11 Tests grün
  (10 bestehende + der neue, Kanzel-erzeugte
  `apply_unified_diff_handles_multiple_hunks`) — unabhängig danach noch
  einmal separat nachgeprüft.
- **RepoWorkbody:** versiegelt, `loom_verify::verify() ==
  Verdict::Valid`.
- **Replay:** identische Ergebnisklasse bestätigt.
- **Branch:** `dogfood/p3-001`, ein einzelner Commit
  `93fab8a94a3e90992956a03a87b9448ef9568547` — genau 7 Zeilen in genau
  einer Datei (`cce/crates/cce-swe/src/model.rs`), ausschließlich
  innerhalb von `mod tests`:

```rust
    #[test]
    fn apply_unified_diff_handles_multiple_hunks() {
        let original = "line 1\nline 2\nline 3\nline 4\n";
        let diff = "@@ -1,4 +1,4 @@\n line 1\n-line 2\n+modified line 2\n line 3\n@@ -4,1 +4,1 @@\n-line 4\n+modified line 4\n";
        let patched = apply_unified_diff(original, diff).expect("Diff wendet an");
        assert_eq!(patched, "line 1\nmodified line 2\nline 3\nmodified line 4\n");
    }
```

- **Commit bestätigt:** ja — `confirmation_ref =
  "operator:auftraggeber;chat-freigabe:TaskProposal-001"`,
  aufgezeichnet über `HumanConfirmationGate`-Disziplin (dieselbe
  `ToolGateway::run_git`-Erzwingung wie in P2, hier am echten Repo
  erneut bewiesen).
- **Merge/Push nach `main`:** NICHT geschehen — der Branch
  `dogfood/p3-001` wurde zur Ansicht des Auftraggebers zusätzlich nach
  `origin/dogfood/p3-001` gepusht, bleibt aber unangetastet neben
  `main` stehen. Die Entscheidung über Merge/Verwerfen bleibt exklusiv
  beim Auftraggeber.

### Ehrlich dokumentiert: mehrere Anläufe waren nötig

Der Lauf gelang nicht beim ersten Versuch. Über rund ein Dutzend echte,
einzelne API-Aufrufe gegen `gpt-4o-mini` hinweg (jeweils dieselbe
freigegebene TaskProposal, derselbe Wunsch — zwischen den Versuchen
wurden ausschließlich die FORMAT-Anweisungen im System-Prompt
präzisiert, niemals der Testinhalt selbst vorgegeben oder die
Kanzel-Antwort von Hand nachgebessert) traten wiederholt folgende
echte, von den Gates korrekt gefangene Fehlerbilder auf:

- **Parsefehler bei `apply_unified_diff`:** die Kanzel-Antwort war nicht
  als gültiger Hunk lesbar (z. B. ein Leerzeichen VOR dem `+`/`-`-Präfix
  statt als erstes Zeichen) — dieser Fall bricht bereits VOR jedem
  Build/Test ab (fail-closed, kein stilles Umformen).
- **Build rot:** die Kanzel-Antwort war zwar als Hunk parsebar, ergab
  aber ungültigen Rust-Code — meist eine fehlende, doppelte oder
  fälschlich mit `#` statt reinem `+` versehene schließende Klammer der
  neu eingefügten Funktion. `BuildEvidenceGate` hielt korrekt.
- **Test rot:** Build gelang, aber die von der Kanzel selbst erfundenen
  Beispieldaten (ein Beispiel-Ausgangstext + ein Beispiel-Diff-String
  ALS Testdaten für `apply_unified_diff()`) waren in sich logisch
  inkonsistent — meist eine falsch berechnete Zeilennummer im zweiten
  Hunk-Kopf des selbst erfundenen Beispiels, gelegentlich ein Diff, der
  nicht zum eigenen Beispieltext passte. `TestEvidenceGate` hielt
  korrekt.

Ein Versuch erreichte bereits einen vollständig korrekten, real grünen
Build+Test-Stand (RepoWorkbody versiegelt, `verify == Valid`, Replay
bestätigt) — an dieser Stelle verhinderte ein eigener Harness-Fehler
(eine Pfad-Konvention beim `git add`-Aufruf: relativer statt absoluter
Pfad gegen den als Cwd genutzten absoluten Scope) nur noch den
allerletzten Schritt (den Commit selbst). Das zeigt: der verbleibende
Fehler zu diesem Zeitpunkt lag an meiner eigenen Orchestrierung, nicht
an der Kanzel oder an der Gate-Kette. Nach der Behebung dieses einen
Harness-Fehlers gelang ein weiterer vollständiger, realer Lauf INKLUSIVE
Commit — das ist der oben dokumentierte, tatsächlich committete Lauf.

Jeder rote Versuch wurde von den ECHTEN `BuildEvidenceGate`/
`TestEvidenceGate` korrekt gehalten (kein PhaseBlock, keine
Versiegelung) — genau die Sicherheitsgarantie, die Dokument 18/19
verlangen: eine Modellantwort mit einem echten Fehler kommt nie
ungeprüft durch. Die reale Datei wurde nach jedem nicht-erfolgreichen
Versuch mit `git checkout --` zurückgesetzt, bevor der nächste Versuch
begann — kein Zwischenstand blieb liegen.

## f) Dieser Bericht

s. oben.

## g) N-DOG-1..4 im Wächter

`conformance/dogfood/dogfood_catalog.rs` (4 Tests, dauerhaft im
Wächter, laufen hermetisch grün in der normalen CI):

- **N-DOG-1:** Task außerhalb der Whitelist-Klassen ⇒
  `TaskProposalGate` reject, `BlockedBeforeApply`, kein Werkzeug
  berührt.
- **N-DOG-2:** Zielpfad in der Schutzzone ⇒ `protected_path_violation`,
  reject VOR jedem Apply.
- **N-DOG-3:** Schreibversuch auf `main` ⇒ `BranchIsolationGate`
  reject, keine Ausnahme.
- **N-DOG-4:** Commit ohne aufgezeichnete Bestätigung ⇒ reject —
  derselbe `ToolGateway::run_git`-Pfad, der in R-DOG-1 real erfolgreich
  committete.

Details/Commit: `1129066`.

## CI GRÜN

`cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets --
-D warnings` (Default-Build UND `--features process`/`--features http`
einzeln geprüft für `cce-dogfood`/`cce-conformance`),
`python3 ci/check_acyclic.py` (57 Workspace-Crates, DAG sauber,
Tor-Trennung + Socket-Scan sauber — `cce-dogfood` bekam den benannten
`loom-verify`-Port, dieselbe Disziplin wie `cce-swe`/`cce-bridge`),
`bash ci/run_ci.sh` — alle grün, weiterhin netzfrei und ohne echte
Subprozesse im Default. Alle Alt-Zeugen (P1/P2/X4/Ring E5/…)
unangetastet.

## Ausgangs-Gate P3 (Dokument 19 §8) — erfüllt

- §2–§7 vollständig implementiert. ✓
- R-DOG-1 real erbracht und vollständig dokumentiert (dieser Bericht). ✓
- N-DOG-1..4 im Wächter, grün. ✓
- Alt-Zeugen unverändert. ✓
- Kein automatischer Merge stattgefunden. ✓

**DoD(P3) = 1** ⟺ Ausgangs-Gate erfüllt — erfüllt. Damit ist die
P3-Voraussetzung von K9 (CompetitiveDoD, Messlatte 17 §4) erfüllt.

## Residuen

Keine neuen Bau-Residuen. Zwei Betriebsbeobachtungen, offen benannt,
nicht blockierend:

- `apply_unified_diff` (P2, unverändert) ist bewusst streng
  (kein Fuzzy-Match) — das machte mehrere reale Kanzel-Anläufe nötig,
  bis eine formal und logisch korrekte Antwort kam. Das ist die
  gewünschte Sicherheitsdisziplin (lieber ein echter Reject als ein
  stillschweigend akzeptierter, fehlerhafter Patch), kein Baumangel.
- `fs_write` bleibt aus P2 bewusst eine In-Memory-Arbeitskopie ohne
  realen Diskschreibpfad; der R-DOG-1-Orchestrator materialisiert das
  bereits berechnete, bereits gate-geprüfte Ergebnis mechanisch auf die
  reale Datei. Ein genuiner realer `fs_write`-Diskpfad (falls je
  benötigt) wäre eine Erweiterung von `cce-toolgateway` — bewusst NICHT
  in P3 gebaut (Schutzzone, keine neue Kern-Logik).

## Nächster Schritt

P4 · Vergleichsläufe (Dokument 17 §3) — ein eigener, separater
nächster Schritt, NICHT Teil von P3. Spezifikation folgt vom
Auftraggeber.

Abweichungen: keine.
