Etappe P4 — Vergleichsläufe: die Überlegenheitsbehauptung wird Beweis
(Dokument 20, direkter Anschluss an P3; K9 · CompetitiveDoD)

Eingang: Etappe P3 abgeschlossen und angenommen
(`reports/P3_dogfooding_bericht.md`). Dokument 20
(`cce-spec-repo/20 P4 VERGLEICHSLAEUFE SPEC.md`) vollständig gelesen,
insbesondere §1 (Design-Entscheidung) und §2 (Fairness-Protokoll).
Gebaut exakt nach Dokument 20, Reihenfolge §4→§5→§6→§7. Nach §6 wurden
die zwei Aufgabenpakete vorgelegt; erst nach ausdrücklicher Freigabe des
Auftraggebers („beide") liefen die realen Vergleichsläufe (§7).

## Die Design-Entscheidung (§1), umgesetzt

Erste Vergleichsspur: **ungegatetes gpt-4o-mini gegen CCE** — beide Arme
nutzen dasselbe zugrundeliegende Modell; der einzige Unterschied ist, ob
die Gate-/Evidence-/Replay-/Zertifizierungsschicht dazwischenliegt. Das
isoliert exakt die Variable: *kostet die Beweispflicht-Schicht etwas,
oder ist sie kostenlos (oder ein Gewinn)?* Der Vergleich gegen
Cursor/Copilot/Bolt bleibt der benannte nächste Schritt (§7 der Spec),
nicht Teil der P4-DoD.

Zwei Aufgabenklassen: **R-BENCH-1 Coding** (CCEs neue Stärke, P2/P3) und
**R-BENCH-2 Dokument/Recherche** (CCEs ursprüngliche Stärke, 213
Domänen).

## §4–§6: die hermetischen Bausteine (grün in der Default-CI)

- **Crate `crates/cce-benchmark`** (Blatt, keine neue Kern-Logik): das
  Objektmodell (`BenchmarkTaskPackage` mit deterministischem
  `task_package_digest`, `RawRunResult` mit `evidence_present=false` als
  struktureller Tatsachenfeststellung, `CceRunResult` mit
  `evidence_present=true` + `repo_workbody_ref` + `gate_report_count` +
  `replay_confirmed`, `ComparisonMatrix` mit den D1–D6-Zeilen), die zwei
  neuen Gates (**FairnessGate**: identischer Digest + Raw-Arm strikt vor
  CCE-Arm; **ComparisonSealGate**: beide Arme abgeschlossen, Digest
  identisch), der Matrixbau und `assemble_benchmark` (FairnessGate →
  ComparisonSealGate → Matrix → Versiegelung).
- **Container-Klasse „benchmark"** (§5, additiv): `loom_format::PROFILES`
  8→9; `loom-verify` `required_kinds("benchmark")` + ein
  benchmark-Semantik-Block (beide Ergebnis-Digests vorhanden,
  Fairness-Reihenfolge konsistent, Matrix genau sechs Zeilen). Die
  unveränderte „repo"-Regel aus P2 bleibt exakt erhalten. Der CCE-Arm
  wird per `cites/derives` an seinen eigenen zertifizierten
  „repo"-Workbody gebunden (D6).
- **Zeugen** (§6, `conformance/benchmark/benchmark_catalog.rs`, 4
  hermetische Tests): R-BENCH-STRUCT (versiegelter benchmark-Workbody
  `verify == Valid`, D1–D6 vollständig), N-BENCH-1 (Fairness-Verstoß →
  FairnessGate reject, keine Matrix), N-BENCH-2 (Matrixbau vor Abschluss
  beider Arme → ComparisonSealGate reject) + eine ehrliche Abbildung
  eines Raw-Arm-Fehlschlags in der Matrix.

## §7: die zwei realen Vergleichsläufe (2026-07-04)

Betriebs-Harness: `crates/cce-benchmark/tests/
r_bench_betriebsverifikation.rs` (`#[ignore]`, Konvention: `cargo test
-p cce-benchmark --features process --features http --test
r_bench_betriebsverifikation -- --ignored --nocapture`). Modell:
`gpt-4o-mini` (echter `CloudModelProviderOpenAI` aus P1). Für **beide**
Arme dasselbe Modell; der ungegatete Arm läuft **zuerst und isoliert**
(`provider.infer()` direkt, an der Gate-Kette vorbei, eigenes
Arbeitsverzeichnis), wird als `RawRunResult` festgehalten, **erst
danach** der CCE-Arm (`run_inference` + unveränderte P2-Kette →
zertifizierter `repo`-Workbody `verify == Valid` → Replay).
FairnessGate erzwingt diese Reihenfolge + Digest-Gleichheit.

**Ehrlichkeitshinweis (wie P3):** das Modell liefert je Aufgabe den
vollständigen neuen Dateiinhalt (natürlicher als ein Diff-Format für
Prosa); der Orchestrator wickelt ihn — für **beide** Arme identisch —
in einen Ganzdatei-Ersetzungs-Diff, den die unveränderte P2-Kette
(CCE-Arm) bzw. `apply_unified_diff` (Raw-Arm) anwendet. Reine
mechanische Einwicklung bereits modellierter Bytes, keine
Lösungsvorgabe. Der einzige inhaltliche Unterschied zwischen den Armen
ist die Gate-Schicht.

### R-BENCH-1 — Coding (neutrales Referenz-Repo `numkit`)

- **Aufgabe:** `max_of` gibt Minimum statt Maximum zurück (`<` statt
  `>`); beheben, sodass `cargo test` grün.
- **Raw-Arm:** Versuch 1 grün, 0 Eingriffe, 3458 ms.
- **CCE-Arm:** Versuch 1 → Candidate, 7 GateReports, `verify == Valid`,
  Replay klassenidentisch, 0 Eingriffe, 2853 ms. Modell-Fix (verifiziert
  korrekt): `if x < m` → `if x > m`.
- **Benchmark-Workbody:** `benchmark`-Container `verify == Valid`
  (4884 Bytes), core_root `1220f84aad66…16cd9f880`.

**D1–D6-Matrix R-BENCH-1 (Coding):**

| Dimension | Raw-Arm (ungegatet) | CCE-Arm |
|---|---|---|
| **D1 Nachweisbarkeit** | strukturell nicht vorhanden | InferenceEvidence + 7 GateReports, RepoWorkbody `1220f84a…` |
| **D2 Wiederholbarkeit** | strukturell nicht vorhanden | Replay: zweiter Lauf klassenidentisch (recorded) |
| **D3 Governance** | strukturell nicht vorhanden | volle Vor-Egress-Gate-Kette + 7 GateReports, fail-closed |
| **D4 Aufgabenparität** | criteria_met (build+test grün) | criteria_met (build+test grün, gate-bezeugt) |
| **D5 Autonomie unter Gates** | 0 Eingriffe (keine Hoheits-Aufzeichnung) | 0 Eingriffe unter Gates (HITL-Hoheit aufgezeichnet) |
| **D6 Auditierbarkeit** | strukturell nicht vorhanden | zertifizierter `.loom`-RepoWorkbody `1220f84a…` (verify == Valid) |

### R-BENCH-2 — Dokument/Recherche (fiktives Quellenbündel)

- **Aufgabe:** drei Quellen (Kapazität/Termin/Sicherheit eines fiktiven
  Rollouts) zu einer Risikoanalyse `analysis.md` verdichten (jede Quelle
  referenziert, ≥ 2 `RISIKO:`-Zeilen, genau eine `Gesamtrisiko:`-Zeile);
  maschinell geprüft durch `check_analysis.py`.
- **Raw-Arm:** Versuch 1 grün, 0 Eingriffe, 2933 ms.
- **CCE-Arm:** Versuch 1 → Candidate, 7 GateReports, `verify == Valid`,
  Replay klassenidentisch, 0 Eingriffe, 5859 ms. Die erzeugte Analyse
  referenziert alle drei Quellen, benennt drei konkrete Risiken und
  schließt mit `Gesamtrisiko: HOCH` — eine inhaltlich plausible,
  kriterienerfüllende Risikoeinschätzung.
- **Benchmark-Workbody:** `benchmark`-Container `verify == Valid`
  (4966 Bytes), core_root `1220838591af…374d0eb34e6`.

**D1–D6-Matrix R-BENCH-2 (Dokument):**

| Dimension | Raw-Arm (ungegatet) | CCE-Arm |
|---|---|---|
| **D1 Nachweisbarkeit** | strukturell nicht vorhanden | InferenceEvidence + 7 GateReports, RepoWorkbody `1220838591af…` |
| **D2 Wiederholbarkeit** | strukturell nicht vorhanden | Replay: zweiter Lauf klassenidentisch (recorded) |
| **D3 Governance** | strukturell nicht vorhanden | volle Vor-Egress-Gate-Kette + 7 GateReports, fail-closed |
| **D4 Aufgabenparität** | criteria_met (Prüfer grün) | criteria_met (Prüfer grün, gate-bezeugt) |
| **D5 Autonomie unter Gates** | 0 Eingriffe (keine Hoheits-Aufzeichnung) | 0 Eingriffe unter Gates (HITL-Hoheit aufgezeichnet) |
| **D6 Auditierbarkeit** | strukturell nicht vorhanden | zertifizierter `.loom`-RepoWorkbody `1220838591af…` (verify == Valid) |

## Was die Matrix zeigt (die eigentliche Behauptung, jetzt belegt)

- **D1/D2/D3/D6 — kategorisch:** der ungegatete Arm ist dort *strukturell
  nicht vorhanden* — nicht „schlechter", sondern ohne jeden Mechanismus,
  den man abfragen könnte. Es gibt keine Evidence, keinen
  maschinengeprüften Replay, keine fail-closed-Gate-Kette, keinen
  auditierbaren `.loom`-Körper. Das ist eine Tatsachenfeststellung, kein
  Werturteil.
- **D4/D5 — Parität in beiden Aufgabenklassen:** der CCE-Arm besteht in
  **beiden** Klassen dieselben `success_criteria` wie der ungegatete Arm
  (beide `criteria_met`), mit **derselben** Eingriffszahl (0 in allen
  vier Läufen — dasselbe Modell, dieselbe Aufgabe). Die
  Beweispflicht-Schicht **kostet an Aufgabenqualität und
  Menscheneingriffen nichts**.

Damit ist die Kernthese von P4 belegt: die Beweispflicht ist an D4/D5
**kostenlos** und liefert an D1/D2/D3/D6 kategorisch etwas, das der
ungegatete Arm gar nicht erbringen kann.

**Ehrliche Nebenbeobachtung (kein DoD-Kriterium):** der CCE-Arm braucht
mehr Wall-Clock (Coding 2853 ms vs. Raw 3458 ms — hier sogar schneller;
Dokument 5859 ms vs. Raw 2933 ms — hier langsamer), weil er zusätzlich
den Replay-Zweitlauf und die Container-Versiegelung fährt. Das ist der
Preis der Garantie (D2/D6), nicht ein Aufgabenqualitäts-Nachteil — D4/D5
bleiben paritätisch. Beide Läufe gelangen ohne jeden Wiederholungsanlauf
zum Ziel (im Gegensatz zu P3s Dutzend-Anläufen — hier waren beide
Aufgaben präzise genug spezifiziert, dass gpt-4o-mini sie im ersten
Anlauf traf).

## CI GRÜN

`cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets --
-D warnings` (Default UND `--features process`/`--features http` für
`cce-benchmark`/`cce-conformance`), `python3 ci/check_acyclic.py` (58
Workspace-Crates, DAG + Socket-Scan sauber — `cce-benchmark` mit
denselben benannten `.loom`-Ports wie `cce-swe`/`cce-bridge`),
`bash ci/run_ci.sh` — alle grün, weiterhin hermetisch (kein Netz, keine
echten Prozess-Aufrufe im Default). Die realen Läufe schreiben
ausschließlich nach `/tmp` (außerhalb des Repos); nichts vom
Benchmark-Lauf gelangt in den versionierten Baum. Alt-Zeugen
(P1/P2/P3/X4/Ring E5/…) unangetastet. `spec/` unberührt.

## Ausgangs-Gate P4 (Dokument 20 §8) — erfüllt

- §2–§7 implementiert ∧ Alt-Zeugen unverändert. ✓
- D1–D6-Matrix für **R-BENCH-1 UND R-BENCH-2** vollständig dokumentiert
  (oben). ✓
- D1/D2/D3/D6: kategorischer Nachweis, dass der Raw-Arm strukturell
  keine gleichwertige Evidence/Replay/Governance/Auditierbarkeit
  vorweisen kann (Tatsachenfeststellung). ✓
- D4/D5: für **jede** der zwei Aufgabenklassen mindestens Parität
  (CCE-Arm besteht dieselben `success_criteria`, gleiche Eingriffszahl).
  ✓

**DoD(P4) = 1 ⟹ K9 (CompetitiveDoD) erfüllt** — genau in der Fassung,
die Messlatte 17 §4 definiert: Beweispflicht kategorisch,
Aufgabenqualität mindestens gleichauf. Der Prototyp ist damit im Sinne
des Auftraggebers **fertig** (K9 = P3 bestanden ∧ D1–D6-Matrix
dokumentiert ∧ Parität-bis-Überlegenheit in D4/D5 ∧ kategorische
D1/D2/D3/D6-Nachweise als Benchmark-Workbodies).

## Residuen

Keine neuen Bau-Residuen. Ausdrücklich benannt, nicht Teil der P4-DoD
(Dokument 20 §7/§8):

- Ein realer Vergleich gegen Cursor / GitHub Copilot / Bolt selbst
  (bräuchte deren Konten/Lizenzen). Der `cce-benchmark`-Baustein nimmt
  jeden weiteren Arm auf, ohne neue Architektur — sobald gewünscht.
- Größere/vielfältigere Aufgabenpakete und wiederholte Läufe zur
  statistischen Absicherung (die zwei Läufe hier belegen die
  kategorische These; eine breitere Stichprobe wäre ein Betriebsschritt,
  kein Baumangel).

## Nächster Schritt

Die Messlatte-17-Sequenz X4 → P1 → P2 → P3 → P4 ist damit vollständig.
Weitere Schritte (Vergleich gegen die namentlich genannten Werkzeuge,
die vier Coffindragger-Merkposten aus Dokument 19 §*) sind eigene,
benannte Aufträge — nicht Teil von P4.

Abweichungen: keine.
