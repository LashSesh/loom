# 19 — P3-SPEZIFIKATION: DOGFOODING (Der Prototyp-Kerntest, K9)

**Der Moment, auf den alles zulief:** CCE arbeitet zum ersten Mal an seinem **eigenen, echten** Repository — nicht an einer harmlosen Testkopie (P2), sondern am Repo, das gerade Ihr Lebenswerk trägt. Deshalb ist diese Spezifikation vorsichtiger geschnitten als jede vorherige: **Sicherheit zuerst, Beweis zweitrangig, Automatik nirgends.** Normativer Overlay; Bauphase P3, direkt nach P2. Zieldefinition unverändert aus Messlatte 17 §3: *„CCE führt einen echten, kleinen Bauauftrag am eigenen Repository aus — Wunsch → Frontier-Kanzel formt → Plan → ToolGateway-Ausführung → Gates → PhaseBlocks → Replay → zertifizierter Ergebnis-Workbody."*

## §1 Die entscheidende Klarstellung: Was Dogfooding NICHT ist

**P3 ist nicht „der Coding-Agent bearbeitet weiter das Repo".** Das wäre gewöhnliche Entwicklungsarbeit — genau das, was bisher geschah, um CCE zu bauen. Dogfooding heißt: **das gebaute System CCE selbst** — über seine eigene Wunschstrecke (S4), seine eigene Kanzel (P1, echtes OpenAI), sein eigenes `cce-swe` (P2), sein eigenes ToolGateway — erzeugt den Diff, durchläuft die eigenen Gates, versiegelt sich selbst als RepoWorkbody. Der äußere Coding-Agent **verdrahtet und beobachtet** diesen Lauf; er schreibt die Diff nicht selbst. Wird das vermischt, ist P3 nicht erfüllt, auch wenn am Ende ein grüner Test steht — genau die Verwechslung, vor der dieses ganze Projekt seit Tag eins schützt (Score/Selbstauskunft ≠ Beweis).

## §2 Schutzzone (unverhandelbar, vor jedem Lauf geprüft)

**Verbotene Pfade** (ProtectedPathFence, harte Negativliste): `spec/`, `cce-spec-repo/` · alle Gate-tragenden Kern-Crates (`cce-core`, `cce-lattice`, `cce-ccc`, `cce-phaseblock`, `cce-crystal`, `cce-kernel`, `cce-spiral`, `cce-hbm`, `cce-bridge`) · alle Gate-Definitionen in `cce-inference`/`cce-toolgateway` · `loom-format`/`loom-codec`/`loom-verify` · `ci/` selbst · jede `Cargo.toml` auf Workspace-Ebene. **Erlaubte Zielklasse für den ERSTEN Lauf** (Task-Klassen-Whitelist, niedrigstes Risiko): Dokumentations-Ergänzung in einem Blatt-Crate · zusätzlicher, rein additiver Unit-Test in einem bereits grünen Modul · ein einzelner, isolierter Lint-Fix (`clippy`-Hinweis) in einem Blatt-Crate. **Kein** struktureller Umbau, keine Gate-Änderung, keine Abhängigkeitsänderung im ersten Lauf.

## §3 Objektmodell-Ergänzung (schlank, über P2)

**TaskProposal:** `(proposal_id, wish_text, task_class ∈ {doc, additive_test, lint_fix}, target_scope [Pfade], estimated_cost_budget, rationale)` — das Ergebnis der Wunschstrecke/Kanzel, **bevor** irgendein Werkzeug läuft. **ProtectedPathFence:** eine geprüfte Funktion `path_allowed(scope) -> bool` gegen §2; jeder Treffer außerhalb ⇒ `protected_path_violation`, harter Reject vor jedem Apply. **DogfoodRun:** bindet TaskProposal + TaskLedger (aus P2) + Branch-Namen (`dogfood/p3-<proposal_id>`, niemals `main`) + die Auftraggeber-Freigabe des Vorschlags selbst.

## §4 Die Kette, real (Erweiterung der P2-Kern-Kette um zwei Vorstufen und eine Nachstufe)

```
Wunsch (Auftraggeber, frei formuliert)
  → Kanzel formt via echtem OpenAI-Provider (P1, unverändertes Gateway) → TaskProposal
  → TaskProposalGate: Task-Klasse ∈ Whitelist ∧ target_scope ⊆ erlaubt ∧ Budget gesetzt
  → **AUFTRAGGEBER-FREIGABE DES VORSCHLAGS** (expliziter Stopp — kein Autolauf ab hier)
  → DiffCandidate (cce-swe, ggf. erneut modellgestützt) → apply auf isoliertem Branch/Arbeitskopie
  → BuildRun + TestRun ECHT (Feature `process` AN — kein Fixture-Ersatz für diesen Lauf)
  → alle P2-Gates + ProtectedPathGate + BranchIsolationGate
  → PhaseBlock → RepoWorkbody (Containerklasse „repo") versiegelt, `loom verify` = Valid
  → Replay: zweiter unabhängiger Lauf, identische Ergebnisklasse
  → HumanConfirmationGate für den lokalen `git commit` auf dem Dogfood-Branch (aufgezeichnet)
  → **KEIN automatischer Merge/Push nach `main`** — das bleibt exklusiv eine Handlung des Auftraggebers
```

## §5 Neue Gates (zusätzlich zu allen P1/P2-Gates, die unverändert gelten)

**TaskProposalGate** (Klasse+Scope+Budget vor jeder Ausführung geprüft) · **ProtectedPathGate** (§2-Fence, reject vor Apply) · **BranchIsolationGate** (jede Schreib-/Commit-Operation nur auf `dogfood/p3-*`; ein Versuch auf `main` ⇒ harter Reject, keine Ausnahme) · **MergeExclusionGate** (strukturell: kein CCE-Pfad enthält einen Merge-in-main-Aufruf — diese Aktion existiert im System schlicht nicht).

## §6 Betriebsverifikation statt CI-Zeuge (der entscheidende Unterschied zu P1/P2)

P3s Hauptbeweis kann **nicht** hermetisch in der grünen Standard-CI liegen — der Witz ist ja gerade „echt, nicht Fixture". Er ist daher, wie P1s reale OpenAI-Probe, ein **einmalig (oder auf Abruf) real ausgeführter, vollständig dokumentierter Lauf**: beide Features (`http` UND `process`) explizit an, `#[ignore]`-gebunden, nie Teil von `cargo test --workspace`. Ergebnis wird als `reports/P3_dogfooding_bericht.md` festgehalten — inklusive: welcher Wunsch, welche TaskProposal, welcher Diff, welches Buildergebnis, welcher Testausgang, welches Gate-Protokoll, welcher Branch, ob der Commit bestätigt wurde. **Kein Wert wird geraten oder aus einer früheren Fixture übernommen** — das ist der ganze Sinn dieser Etappe.

## §7 Zeugen

**R-DOG-1** (Hauptzeuge, Betriebsverifikation): ein echter, kleiner, whitelisted Task auf dem echten Repo durchläuft die volle Kette aus §4 bis zum versiegelten, `Valid`-geprüften RepoWorkbody auf einem isolierten Branch; Replay klassenidentisch. **R-DOG-2:** dieselbe Kette mit einer TaskProposal, die vom Auftraggeber **abgelehnt** wird — Lauf endet sauber vor jeder Werkzeug-Ausführung, sichtbar dokumentiert. **N-DOG-1** Task außerhalb der Whitelist-Klassen ⇒ `TaskProposalGate` reject. **N-DOG-2** Zielpfad in der Schutzzone ⇒ `protected_path_violation`, reject vor Apply. **N-DOG-3** Schreibversuch auf `main` ⇒ `BranchIsolationGate` reject. **N-DOG-4** Commit ohne aufgezeichnete Auftraggeber-Bestätigung ⇒ reject (P2-Disziplin, hier am echten Repo erneut bewiesen).

## §8 Bauplan und DoD

Reihenfolge: a) TaskProposal-Objekt + ProtectedPathFence + Whitelist → b) TaskProposalGate + BranchIsolationGate + MergeExclusionGate → c) RepoSnapshot auf eine **klar benannte, kleine** reale Zielstelle beschränken (keine Vollspiegelung des 56-Crate-Workspace) → d) die Kette §4 verdrahten (bestehende P1/P2-Bausteine, keine neue Kern-Logik) → e) EINMAL real ausführen, mit Ihrer expliziten Freigabe des konkreten Vorschlags → f) `reports/P3_dogfooding_bericht.md` schreiben → g) N-DOG-1..4 als hermetische Tests in den Wächter (die dürfen und sollen grün in der normalen CI laufen — nur R-DOG-1 selbst ist der reale, separate Beweis).

**Ausgangs-Gate P3:** §2–§7 vollständig ∧ R-DOG-1 real erbracht und dokumentiert ∧ N-DOG-1..4 im Wächter ∧ Alt-Zeugen unverändert ∧ kein automatischer Merge stattgefunden. **DoD(P3) = 1 ⟺** Ausgangs-Gate erfüllt. Damit ist die P3-Voraussetzung von K9 (CompetitiveDoD, Messlatte 17 §4) erfüllt — **P4 (Vergleichsläufe gegen Claude Code/Cursor) ist ein eigener, separater nächster Schritt, nicht Teil von P3.**

*Vorgemerkt, nicht vergessen: die vier Coffindragger-Ideen (reichere Sucharten, deterministischer Zweit-Orakel-Typ, Fremd-Repo-Bauplan-Destillation, Instanzen-Föderation) bleiben unbearbeitete, benannte Merkposten für die Zeit nach P3/P4.*
