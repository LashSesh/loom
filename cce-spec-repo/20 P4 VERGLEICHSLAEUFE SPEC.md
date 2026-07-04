# 20 — P4-SPEZIFIKATION: VERGLEICHSLÄUFE (K9 vollständig, „mehrfach übertroffen" wird Beweis statt Behauptung)

**Der letzte Schritt zur eigentlichen Behauptung.** P3 hat bewiesen: CCE kann an sich selbst arbeiten. P4 beweist das Eigentliche — dass die Beweispflicht-Architektur nicht nur nichts kostet, sondern etwas liefert, das kein Vergleichssystem liefern kann, **ohne** bei der reinen Aufgabenqualität schlechter zu sein. Normativer Overlay, direkter Anschluss an P3. Bezugsformel unverändert aus Messlatte 17 §2/§4.

## §1 Design-Entscheidung (autonom getroffen, kurz begründet)

**Erste konkrete Vergleichsspur: ungegatetes Claude Code gegen CCE — nicht sofort Cursor/Copilot/Bolt.** Begründung: Das isoliert exakt die Variable, um die es geht — *kostet die Beweispflicht-Schicht etwas, oder ist sie kostenlos (oder sogar ein Gewinn)?* Beide Arme nutzen dasselbe zugrundeliegende Modell; der einzige Unterschied ist, ob Gate/Evidence/Replay/Zertifizierung dazwischenliegen. Das ist der wissenschaftlich sauberste Vergleich, den man ohne fremde Lizenzen/Konten sofort fahren kann. **Der Vergleich gegen die namentlich genannten Werkzeuge bleibt ausdrücklich der nächste, benannte Schritt** (§7) — nicht Teil der P4-DoD, aber nicht vergessen.

**Zwei Aufgabenklassen, nicht nur Coding:** Messlatte 17 nennt „Coding-/Dokument-/Rechercheaufgaben" explizit. R-BENCH-1 prüft Coding (CCEs neue Stärke, P2/P3). R-BENCH-2 prüft Dokumentarbeit (CCEs ursprüngliche Stärke, 213 Domänen, D01) — hier tritt CCE auf eigenem Boden gegen ein Allzweckwerkzeug an, das dafür nicht gebaut ist.

## §2 Fairness-Protokoll (unverhandelbar)

Beide Arme erhalten **exakt denselben** Aufgabentext und Ausgangszustand, gehasht vor Beginn (`task_package_digest`). Der ungegatete Arm läuft **zuerst und vollständig isoliert** — sein Ergebnis wird versiegelt und **erst danach** dem CCE-Arm-Ergebnis gegenübergestellt; keine Seite sieht die Antwort der anderen vor der eigenen Abgabe (`FairnessGate`, geprüft über Zeitstempel + Ergebnis-Digest-Reihenfolge). Weder Testinhalt noch Lösungshinweise werden zwischen den Läufen weitergegeben — dieselbe Disziplin, die schon in P3s Dutzend-Anläufen galt (nur Formatpräzisierung, nie Inhaltsvorgabe).

## §3 Objektmodell

**BenchmarkTaskPackage:** `(package_id, task_text, starting_files[Pfad→Inhalt], success_criteria, task_package_digest)` — für R-BENCH-1 ein kleines, neutrales Referenz-Repo (nicht CCEs eigenes!); für R-BENCH-2 ein kleines Dokumentenbündel + Aufgabentext (z. B. Analyse dreier Quellen zu einer Risikoeinschätzung — bewusst strukturähnlich zu D01, aber neuer Inhalt, um Auswendiglernen auszuschließen). **RawRunResult:** `(package_id, arm="raw", output_digest, wall_time, build_pass?, test_pass?, human_interventions_count, evidence_present=false)` — das Ergebnis des ungegateten Arms; „evidence_present=false" ist eine strukturelle Tatsachenfeststellung, keine Wertung: es gibt dort schlicht keinen Gate-/Replay-/Zertifizierungsmechanismus, den man abfragen könnte. **CceRunResult:** dieselben Felder + `repo_workbody_ref` (P2/P3-Kette, generalisiert auf beliebiges Zielpaket statt nur CCEs eigenen Baum) + volle InferenceEvidence/GateReport-Kette. **ComparisonMatrix:** die D1–D6-Zeilen je Aufgabenklasse, mit Belegverweis je Zelle (Digest, Testname oder „strukturell nicht vorhanden" für den Raw-Arm bei D1/D2/D3/D6).

## §4 Neue Bausteine (Crate `cce-benchmark`, Blatt, keine neue Kern-Logik)

`crates/cce-benchmark`: generalisiert `RepoSnapshot`-Targeting auf ein beliebiges deklariertes `BenchmarkTaskPackage` (nicht CCEs eigenen Baum — kein `ProtectedPathFence`-Bezug zu CCE selbst nötig, da nie CCEs eigenes Repo betroffen ist); bindet den **unveränderten** `cce-swe`/`cce-dogfood`-Kern für den CCE-Arm; nimmt `RawRunResult` als reines Beobachtungsprotokoll entgegen (CCE kann den Raw-Lauf nicht selbst zertifizieren — nur die BEOBACHTBAREN Fakten aufzeichnen: baute es, testete es, wie lange, wie oft musste nachgeholfen werden). **FairnessGate** (§2, strukturell) + **ComparisonSealGate** (beide Arme abgeschlossen, `task_package_digest` auf beiden Seiten identisch, bevor die Matrix gebaut wird).

## §5 Container-Klasse „benchmark" (additiv, minor)

Ein `.loom` der neuen Klasse `benchmark` bindet: `BenchmarkTaskPackage`, `RawRunResult`, `CceRunResult` (inkl. dessen eigenem „repo"-Workbody via `cites`/`derives`), `ComparisonMatrix`. `loom verify` prüft: beide Ergebnis-Digests vorhanden, Fairness-Zeitstempel konsistent, Matrix-Zeilen vollständig. Kein neues Segment nötig (Pflichtfelder in MANIFEST + CANDIDATE_OUTPUTS/EVIDENCE wie gehabt).

## §6 Zeugen

**R-BENCH-1** (Coding): identische kleine Bug-Fix-/Feature-Aufgabe in einem neutralen Referenz-Repo, beide Arme, versiegelter `benchmark`-Workbody, `verify == Valid`. **R-BENCH-2** (Dokument/Recherche): identische Analyseaufgabe über ein kleines Quellenbündel, beide Arme, ebenso versiegelt. **N-BENCH-1** Fairness-Verstoß (CCE-Arm startet mit abweichendem `task_package_digest`) ⇒ `FairnessGate` reject, keine Matrix gebaut. **N-BENCH-2** Matrixbau vor Abschluss beider Arme ⇒ `ComparisonSealGate` reject.

## §7 Betriebsverifikation (wie P1/P3 — kein hermetischer CI-Zeuge)

R-BENCH-1/2 sind reale, einmalig (oder auf Abruf) durchgeführte, vollständig dokumentierte Läufe — `reports/P4_vergleichslaeufe_bericht.md` hält je Arm: Aufgabentext, Ergebnis, Bauzeit, Testausgang, Eingriffe, und die ausgefüllte D1–D6-Matrix. **Objekte/Gates/Container-Klasse selbst** (§3–§5) sind hermetisch testbar und laufen grün in der normalen CI.

## §8 DoD — exakt die K9-Formel aus Messlatte 17 §4, nicht verschärft

```
DoD(P4) = 1 ⟺
    §2–§7 implementiert ∧ Alt-Zeugen unverändert
  ∧ D1–D6-Matrix für R-BENCH-1 UND R-BENCH-2 vollständig dokumentiert
  ∧ D1/D2/D3/D6: kategorischer Nachweis, dass der Raw-Arm strukturell
        keine gleichwertige Evidence/Replay/Governance/Auditierbarkeit
        vorweisen kann (Tatsachenfeststellung, kein Werturteil)
  ∧ D4/D5: für JEDE der zwei Aufgabenklassen mindestens PARITÄT
        (CCE-Ergebnis besteht dieselben success_criteria) — Überlegenheit
        wo tatsächlich beobachtet, aber Parität genügt laut Messlatte
```
**DoD(P4)=1 ⟹ K9 (CompetitiveDoD) erfüllt** — genau in der Fassung, die Messlatte 17 selbst definiert: Beweispflicht kategorisch, Aufgabenqualität mindestens gleichauf.

**Explizit NICHT Teil der P4-DoD (benannt, nicht vergessen):** ein realer Vergleich gegen Cursor/GitHub Copilot/Bolt selbst (bräuchte deren Konten/Lizenzen) — sobald gewünscht, derselbe `cce-benchmark`-Baustein nimmt jeden weiteren Arm auf, ohne neue Architektur.
