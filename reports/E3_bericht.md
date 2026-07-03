Ring E3 — Selbstbezug: HBM produktiv (Ökosystem-Expansionskarte §2/E3)

Eingang: Ring E2 abgeschlossen und angenommen. Dokument 14 stellt fest:
„Ring E3 (HBM auf Eigenkorpus) braucht keine neue Norm — HBM ist
vollspezifiziert; der Eigenkorpus ist gewöhnlicher Ingest." Keine
Spec-Lieferung nötig; Bau direkt aus der bestehenden, unveränderten
`cce-hbm`-Pipeline (Commit `7f8828f`).

## Was gebaut wurde

`eigenkorpus_ingest_lines()` (loom-conformance): projiziert die REALEN
213 Familien-Referenzprofile — `DocProfile.rule`, die tatsächliche im
Code lebende Naht-Regel jeder Domäne (`Relation`/`AcyclicRelation`/
`ChainedRelation`/`UniqueSubjects`/`OrderedSteps`/`StructuralPresence`),
gesammelt über alle 16 `family_*_domains::all_profiles()` — plus die
Katalog-Kern-Gates/-Residuen (`CATALOG`, 213 Einträge) in das
bestehende HBM-Facet-Vokabular (`invariant`/`gate`/`constraint`,
`cce_hbm::facet::FACET_TYPES`). Jede Zeile trägt die Domänen-Kennung,
damit kein `ExclusionGate`-Hold durch redundante Struktur entsteht
(639 Fact-Zeilen: 213 Regel- + 213 Gate- + 213 Residuum-Fakten).

`build_eigenkorpus_mining_input()` speist das durch die UNVERÄNDERTE
`cce-hbm::pipeline::run_pipeline`. Gefundener und dokumentierter
Parameter-Unterschied zum kleinen Demo-Korpus: `theta_d=0` statt `=1` —
`Score_D` wächst quadratisch mit der Facet-Anzahl (bestehende Formel in
`score.rs`, keine Änderung), bei über 600 Facetten in der
„Vollprojektion" C6 unterschreitet der Rohscore real den Schwellwert 1.
θ_D bleibt reine Vorauswahl, nie Abnahme (Doku-Kommentar in `score.rs`
selbst); `0` lässt alle Kandidaten zu den Gates durch, ohne die
Gate-Entscheidung zu verändern.

`build_blueprint_eigenkorpus()`: der ECHTE, aus dem Eigenkorpus
zertifizierte Blueprint-Kristall — ein `hbm`-Profil-Container mit den
tatsächlichen Facetten/Kandidaten/zertifizierten Klassen des Laufs
(kein Platzhaltertext wie `build_r3`/`hbm_content_kristall`). Seed:
`library/seed/blueprint_eigenkorpus.loom` (70 KB, `loom verify` ⇒
`Valid`, 0 Residuen).

## Zeugen (`conformance/tests/e3_hbm_eigenkorpus.rs`, 5 Tests)

- `eigenkorpus_end_to_end_certifies_at_least_one_blueprint`: der
  Eigenkorpus-Lauf zertifiziert mindestens einen Blueprint;
  Ledger-Commits == Anzahl zertifizierter Klassen; `verify_ledger` Ok
  (HBM-18-Disziplin).
- `full_projection_candidate_covers_the_entire_eigenkorpus`: die
  „Vollprojektion" C6 trägt ALLE Facetten des Eigenkorpus (keine
  Teilmenge) UND ist zertifiziert — das im Karten-Beispiel genannte
  „Struktur-Muster wiederkehrender Naht-Regeln über Familien" ist damit
  real nachgewiesen, nicht behauptet.
- `eigenkorpus_replay_is_class_identical`: zwei Läufe desselben
  `MiningInput` zertifizieren dieselben Klassen, enden im selben
  Ledger-Head (HBM-20-Disziplin, hier auf dem realen Eigenkorpus statt
  dem kleinen Demo-Korpus).
- `r13_cloning_lock_stays_closed_negative_witness_remains_red`:
  `cce_hbm::specialization` wurde in diesem Ring NICHT angefasst — der
  Negativzeuge (`unbounded_cloning`, Meldungstext enthält „R-13") bleibt
  rot, 0 Nachkommen.
- `blueprint_eigenkorpus_seed_is_valid_and_matches_builder`: Seed-Datei
  byte-identisch zum Generator, `Valid`.

## Ausgangs-Gate E3 — Prüfung gegen Karte §2/E3

**Exit-Zeugen (Karte): „HBM-End-to-End auf Eigenkorpus, Blueprint-
Kristall zertifiziert, Replay klassenidentisch, Klonungs-Lock-
Negativzeuge weiterhin rot":**
1. HBM-End-to-End auf Eigenkorpus — grün.
2. Blueprint-Kristall zertifiziert — grün (mindestens die C6-
   Vollprojektion, real materialisiert als `.loom`-Container).
3. Replay klassenidentisch — grün.
4. Klonungs-Lock-Negativzeuge weiterhin rot — grün (unverändert,
   zusätzlich frisch bestätigt).

**Alle Alt-Zeugen unverändert:** voller Workspace-Testlauf: 163
Testgruppen, 0 Fehlschläge. Alle bestehenden HBM-Katalog-Zeugen
(HBM-01…20, kleiner Demo-Korpus) unangetastet und grün; `cce-hbm`
selbst wurde in KEINER Zeile verändert — nur ein neuer Aufrufer
(loom-conformance) mit realen Eingabedaten.

**CI GRUEN:** `cargo fmt --all`, `cargo clippy --workspace --all-targets
-- -D warnings`, `python3 ci/check_acyclic.py` (53 Workspace-Crates),
`bash ci/run_ci.sh` — alle grün.

**Kein neues Kern-Crate mit externen Abhängigkeiten:** keine neue Crate
angelegt; keine externe Kiste hinzugefügt.

**Ausgangs-Gate E3: ERFÜLLT.**

## Residuen

- R-Agent-9 (aus `reports/E2_bericht.md`: Blueprint-Zelle für SCALE-3
  war ein struktureller Platzhalter) — GESCHLOSSEN: der reale,
  eigenkorpus-zertifizierte Blueprint-Kristall existiert jetzt
  (`library/seed/blueprint_eigenkorpus.loom`). Der SCALE-3-Zeuge aus
  Ring E2 selbst wurde bewusst NICHT rückwirkend umgeschrieben (Ring-
  Grenzen bleiben historisch stabil, „Alt-Zeugen unverändert" gilt auch
  für abgeschlossene Ringe); der neue Blueprint steht für künftige
  Verwendung (z. B. eine SCALE-3-Auffrischung oder die Klassen-Registry
  in E4c) real zur Verfügung.
- Die Facet-Vokabular-Zuordnung (invariant/gate/constraint) ist eine
  bewusste, dokumentierte Übersetzungsentscheidung (kein im Katalog
  selbst vorgegebenes Mapping) — jede Wahl ist im Code kommentiert und
  nachvollziehbar, keine stille Interpretation.

## Nicht begonnen (auftragsgemäß)

E4 (CE-1 Tabellen-Zellentyp, loom-sdk/wasm-Viewer, Klassen-Registry),
E5 (L9b Normic Memory) — laut Bau-Reihenfolge in Dokument 14 folgt jetzt
E4, beginnend mit CE-1 nach Teil II. Host-Leiste unverändert gesperrt.
`spec/` unangetastet.

Abweichungen: keine.
