Etappe X3 — Ring E5: L9b Normic Memory (Ökosystem-Expansionskarte
§2/E5, S-E5 `15_L9B_NORMIC_MEMORY_SPEC.md`, öffnet und schließt R-1b)

Eingang: Etappe X2 (Ringe E2→E3→E4) abgeschlossen und angenommen.
Spec-Lieferung S-E5 gelesen und vollständig umgesetzt: ein Gedächtnis,
das ausschließlich aus Geschlossenem besteht — herkunftsgebunden,
gate-promoviert, revidierbar, sichtbar erodierend, strikt opt-in in
der Anwendung. Bauplan §10, Reihenfolge a→j, in genau dieser
Reihenfolge umgesetzt, drei Commits.

## Einheit a-f, h — cce-bridge Kern (Commit 7adcbc5)

Neues Crate `crates/cce-bridge` (kein Egress, nur benannte Ports:
`cce-core`, `loom-cites`, der `.loom`-Kern). Neue, explizit begründete
`ci/check_acyclic.py`-Ausnahme (S-E5 §10 autorisiert diese Ports
wörtlich, dieselbe Disziplin wie `cce-runner`/`cce-observe`).

**a) types.rs** — `NexusClass` (fünf Klassen v1), `Scope`, `NormStatus`,
`ProvenanceSet`, `NormCandidate`, `BridgeNorm` (mit `derives_cites` —
das ProvenanceSet ist die `external_citations`-Liste der Norm, S-E2a
wirkt wörtlich weiter), das vollständige Residuen-Vokabular (§8).

**b) provenance.rs** — Auflösung über den bestehenden
`CitationResolver`-Port: Mitglied muss Valid EXAKT (schließt
Quarantäne und `ValidWithResidues` aus, die wörtliche Spec-Lesart),
`claims.closed`, keine offene eigene `supports`/`derives`-Naht
(Quellen-Quiescence — die Zielseite jeder eigenen closure-relevanten
Naht muss selbst mindestens `ValidWithResidues` sein).

**c) distill.rs** — RD-gebundener Destillationslauf: ein
unvollständiger RD (S5.9) macht die Destillation verboten
(N-A6/N-NRM-7); `n_counter` ist strukturell nie verschweigbar
(= `known_counterexamples.len()`, kein freies Feld).

**d) gate.rs** — BridgeGate, sechs benannte Stufen in fester
Reihenfolge: ProvenanceGate, DiversityGate, CounterexampleGate
(HITL-Bestätigung über `HitlDecision`, dasselbe Muster wie S5-A5),
ConflictGate, ScopeGate (Fundament-Schutz — eine Pattern-Whitelist
schließt jede Lockerung strukturell aus), DistillationReplayGate.
`BridgeGateReport`s Felder und Konstruktoren sind PRIVAT — nur
`bridge_gate()` kann ein `Allow`-Verdikt erzeugen (PROD-INV-21
strukturell, nicht nur konventionell erzwungen).

**e) workbody.rs** — die Norm IST ein `.loom`-Workbody der neuen
Containerklasse `"norm"` (MANIFEST+CL+LEDGER+RESIDUE+EVIDENCE+
REPLAY_MANIFEST). `loom-format::PROFILES` (6→7) und
`loom-verify::required_kinds` additiv um `"norm"` erweitert. Nur ein
`Allow`-Verdikt darf hier ankommen.

**f) memory.rs** — `norms/`-Sektion additiv an die E4c-Klassen-
Registry angehängt (dieselbe additive Cv-Erweiterungsdisziplin wie
S-E2a I.4/X1c), `query(scope, class)`.

**h) lifecycle.rs** — automatische Erosion (ein ineligibles
ProvenanceSet-Mitglied → `deprecated`), gegateter Widerruf (siegelt
einen echten Widerrufs-Workbody, zitiert Norm+Gegenbelege via
`refers`), Lineage (`supersedes`-Kette).

`cce-core::RunDescriptor` gewinnt additiv `norm_profile: Vec<String>`
(§5: RD-Input, Replay-pflichtig). `loom-replay` gewinnt additiv
`replay_manifest_segment_with_norms`/`check_norm_profile`.

## Einheit g — Aktivierung im Runner (Commit 5aa5d1f)

`cce-runner` hängt jetzt von `cce-bridge` ab (Richtung
`cce-runner -> cce-bridge`, kein Zyklus). `Run` trägt
`activated_norms: Vec<BridgeNorm>` (vom Aufrufer bereits aufgelöst);
`Stage::Loom` prüft VOR dem Weben fail-closed, dass jede in
`rd.norm_profile` gelistete `norm_id` unter `activated_norms` als
`Active` auflösbar ist — sonst `norm_not_activated`
(N-NRM-6/PROD-INV-22). Eine Norm ohne RD-Eintrag wirkt nie, selbst
wenn sie dem Lauf bekannt ist (A3: strikt opt-in).

## Einheit i — Zeugen im Wächter (Commit 239c263)

Meilenstein **R-NRM-1**: `loom_conformance::build_first_active_norm`
siegelt DREI echte, geschlossene ("full", `claims_closed=true`)
Familien-Referenz-Cubes (D02/D03/D06 — dieselbe `Relation`-Kern-Naht-
Regel-FORM, verschiedene Nahtnamen "refers"/"derives"/"priced", echter
Motorpfad über `cce-runner`), destilliert daraus einen `NormCandidate`
(RD-gebunden), promoviert ihn durchs BridgeGate (Allow) und siegelt die
erste aktive Norm als `.loom`-Workbody. Verdikt `Valid`, `CitationGate`
grün (alle drei `derives`-Nähte lösen über den `SeedResolver`
auf — derselbe Port wie in Ring E2). Eine DRITTE `CitationResolver`-
Implementierung (`InMemoryResolver`, speicherbasiert) beweist erneut
Resolver-Agnostik.

Alle 12 Zeugen + PROD-INV-21..23 real und grün, gegen den ECHTEN
R-NRM-1-Kandidaten/die reale Norm (nicht gegen synthetische Fixtures):
`conformance/tests/e5_l9b_normic_memory.rs` (15 Tests) +
`crates/cce-runner/tests/g5_gate.rs` (4 neue Aktivierungs-Tests).

- R-NRM-1 — Meilenstein, s. oben.
- R-NRM-2 — Aktivierung: Lauf mit `norm_profile`, RD listet `norm_id`,
  Replay klassenidentisch (zwei unabhängige Läufe, gleiche Klasse).
- R-NRM-3 — Widerruf: echter Widerrufs-Workbody (`Valid­WithResidues`,
  zitiert Norm+Gegenbeleg via `refers`), Status `revoked`,
  Alt-Lauf-Reanalyse zeigt `norm_since_revoked`.
- R-NRM-4 — Erosion: ein invalid gewordenes Mitglied (Resolver liefert
  es nicht mehr auf) lässt die Norm automatisch auf `deprecated`
  fallen.
- N-NRM-1 — κ(=2) < κ_min(=3) ⇒ `insufficient_provenance`, Hold.
- N-NRM-2/PROD-INV-21 — ein Hold-Verdikt darf niemals einen
  Norm-Workbody erzeugen (`seal_norm` weist zurück).
- N-NRM-3/PROD-INV-23 — drei verschiedene lockernde Pattern
  ("Gate deaktivieren", "Capability-Lock entfernen",
  "Invariante aufheben") werden alle als `norm_scope_violation`
  rejected.
- N-NRM-4 — ein Kandidat mit Gegenbeispiel ohne aufgezeichnete
  HITL-Bestätigung wird rejected (`counterexample_unresolved`).
- N-NRM-5 — ein widersprechender Kandidat gegen eine bereits aktive
  Norm gleichen Scopes hält (`norm_conflict`).
- N-NRM-6/PROD-INV-22 — Anwendung ohne Aktivierung im Profil ⇒
  `norm_not_activated`.
- N-NRM-7 — Destillation mit unvollständigem RD ist verboten.
- N-NRM-8 — eine abweichende Reproduktion wird vom
  DistillationReplayGate erkannt (`distillation_replay_mismatch`).

`conformance::GUARD_PHASES += "L9B"`,
`FEATURE_PL += ("l9b_normic_memory", "PL2", ...)`.

## Ausgangs-Gate X3 — Prüfung gegen §10

**Alle 12 Zeugen korrekt:** grün, s. oben (Log-Auszug: 15/15 in
`e5_l9b_normic_memory.rs`, 4/4 neue Aktivierungs-Tests in
`g5_gate.rs`).

**Alt-Zeugen unverändert:** voller Workspace-Testlauf nach jeder
Einheit und final: 174 Testgruppen, 0 Fehlschläge. Alle 213 Domänen-
Referenz-Zeugen, R1–R8-Golden-Files, alle X1/E2/E3/E4-Zeugen
unangetastet (inkl. `e4c_class_registry.rs`, dessen Registry-
Erwartungen trotz additiv erweiterter `norms/`-Sektion unverändert
grün bleiben).

**PROD-INV-21..23 negativ-getestet:** grün (`n_nrm_2_and_prod_inv_21_
candidate_without_allow_cannot_seal`,
`n_nrm_3_and_prod_inv_23_scope_loosening_patterns_are_rejected`,
`n_nrm_6_and_prod_inv_22_application_without_activation_is_refused`).
PROD-INV-21 zusätzlich strukturell erzwungen (private Felder/
Konstruktoren in `BridgeGateReport`).

**CI GRUEN:** `cargo fmt --all`, `cargo clippy --workspace
--all-targets -- -D warnings`, `python3 ci/check_acyclic.py` (55
Workspace-Crates, DAG sauber), `bash ci/run_ci.sh` — alle grün.

**Kein neues Kern-Crate mit externen Abhängigkeiten:** `cce-bridge`
trägt keine externe Kiste (nur interne `cce-core`/`loom-*`-Ports).

**Ausgangs-Gate X3: ERFÜLLT.**

## Residuen

- R-Agent-13: das Pattern eines `NormCandidate` ist caller-geliefert,
  nicht algorithmisch aus HBM-Blueprints gemint (§7 nennt HBM nur als
  "optionale" Quelle; `cce-bridge` hängt bewusst nicht von `cce-hbm`
  ab). Details: `reports/residuen.md`.
- R-Agent-14: ScopeGate ist String-/Stichwort-basiert (v1), keine
  Semantikanalyse — bewusst konservativ. Details:
  `reports/residuen.md`.

## R-1b

**R-1b: GESCHLOSSEN** (im Agenten-eigenen Register
`reports/residuen.md` geführt — `spec/`/`cce-spec-repo/` bleibt
unangetastet, dessen internes Register wird nicht editiert).

## Nicht begonnen (auftragsgemäß)

Host-Leiste (GGUF/LLM, OS-Keyring-Live, macOS/Windows-Pakete,
GPU-Klickpfad) bleibt gesperrt. `spec/`/`cce-spec-repo/` unangetastet.

Abweichungen: keine.
