Ring E2 — Verbund & Skalen (Ökosystem-Expansionskarte §2/E2 +
S-E2a Teil I, Dokument 14)

Eingang: Etappe X1 (Ring E1) abgeschlossen und angenommen. Spec-Lieferung
S-E2a (`cce-spec-repo/14_CITES_UND_COREEXTENSION_SPEC.md`, Teil I)
gelesen und normativ umgesetzt. Bau-Reihenfolge laut Dokument 14: I.1–I.7
(inkl. R-CIT-3/SCALE-3) — vollständig in vier Einheiten (Commits
`ac7a3f2`, `9efa24b`, `b3b54f6`, `f5163bd`) umgesetzt.

## Einheit 1 — Format-/Verify-Grundlage (Commit ac7a3f2)

`loom-verify`: neue hermetische L2-Prüfung `manifest_citation_mismatch`
(N-CIT-5) — MANIFEST.external_citations muss exakt der Menge der
tatsächlichen `cites`-Ziele im CL_SUBSTRATE entsprechen. Rein
byte-lokal, kein Resolver nötig — die eigentliche Auflösung externer
Ziele lebt bewusst in einem eigenen Crate (Einheit 2), da `loom-verify`
motorfrei-hermetisch bleiben muss (LOOM Teil 6).

`cce-core::RunDescriptor`: additives Feld `input_digests` (I.5) +
Builder `with_input()` — zitierte Ziel-Digests werden Teil der
RD-Klasse. `loom-replay`: additive `replay_manifest_segment_with_inputs`
+ `check_replay_inputs` (bestehende Funktionen unverändert).

## Einheit 2 — loom-cites: CitationResolver-Port, SeedResolver,
CitationGate (Commit 9efa24b)

Neues Crate `loom-cites` (keine externen Abhängigkeiten; hängt nur an
loom-canon/format/codec/verify/mount + cce-materialize über denselben
Motor-Port wie `loom-conformance`, INV-11 unverletzt).

- `CiteEntry`/`CiteKind` (supports/refers/derives) + Cv-Codec fürs
  CL_SUBSTRATE-Feld `"cites"` (additiv).
- `CitationResolver`-Port (I.2): `resolve(core_root) ->
  Option<VerifiedTarget>`, entkoppelt E2 von E4 (die Klassen-Registry
  aus E4c wird später eine zweite Implementierung).
- `SeedResolver`: durchsucht konfigurierte Verzeichnisse, verifiziert
  jeden Kandidaten L0–L2 UND prüft den Digest gegen den angefragten
  `core_root` gegen, bevor er akzeptiert wird. Der Einheiten-Index für
  `target_unit_ref`-Prüfungen wird best-effort aus den in X1(a)
  eingebetteten Artefakt-Bytes rekonstruiert
  (`loom_mount::extract_artifact` + `cce-materialize::parse_markdown`).
- `citation_gate()` (I.3): die fünf Prüfschritte einzeln benannt
  (`UnresolvedCitation`, `CitationTargetInvalid`,
  `CitationClassMismatch`, `CitationUnitMissing`,
  `CircularSupportCitation`) + `closure_pass` (nur supports/derives sind
  closure-relevant). Verteidigung in der Tiefe: dem Resolver wird nie
  blind vertraut (core_root-Gegenprüfung, dieselbe Disziplin wie
  `loom_mount::extract_artifact`, X1a). Zyklenerkennung über
  Workbody-Grenzen per DFS + Besuchsmenge.

15 Unit-Tests (Mock-Resolver deckt alle fünf Gate-Schritte deterministisch
ab, inkl. eines zweistufigen Zyklus über zwei Container); SeedResolver
zusätzlich real gegen `library/seed/kristall_wikimedia_workbody.loom`
geprüft.

## Einheit 3 — R-CIT-1/2 + N-CIT-1/5 auf dem echten Welt-Kristall
(Commit b3b54f6)

`manifest_declare_external_citations` (additiv, dieselbe Disziplin wie
`manifest_declare_hash_profiles`, X1c). Das erste Memo, das sich real
über Workbody-Grenzen hinweg auf einen anderen zertifizierten Container
stützt: `citing_memo_welt_kristall` + `seal_citing_memo_welt_kristall`,
mit zwei `cites`-Einträgen auf den echten Welt-Kristall aus Block 3 —
R-CIT-1 (unit `d1`, `supports`, kein `target_unit_ref`) und R-CIT-2
(unit `d2`, `derives`, `target_unit_ref` auf die Definitionseinheit `d1`
DES Welt-Kristalls). Seed: `library/seed/citing_memo_welt_kristall.loom`.

Neuer Konformitätstest `conformance/tests/e2_cites.rs` (4 Tests, gegen
die echten Container): R-CIT-1/2 lösen über `SeedResolver` gegen das
reale `library/seed/` beide grün auf (`closure_pass = true`); N-CIT-1
(unaufgelöstes Ziel) bleibt transportierbar/`Valid`, aber
`closure_pass = false`; N-CIT-5 (MANIFEST.external_citations manipuliert
über `decode_sealed` → Feld leeren → `seal_canonical` neu versiegeln)
⇒ `reject` mit `manifest_citation_mismatch`.

## Einheit 4 — SCALE-3 „Projektraum" (I.6), Zeuge R-CIT-3 (Commit
f5163bd)

Neues Modul `cce-materialize::scale3_project`: `Scale3Project`/
`ProjectEntry` (Zellen = SCALE-2-Mappen/Quellen-Workbodies/
Blueprint-Kristalle, referenziert per `core_root`) + Nähte
(`contains`/`cites`/`precedes`). `materialize_index`/`parse_index` +
`project_seams_valid` spiegeln exakt das SCALE-2-Muster, bewusst OHNE
Resolver-Abhängigkeit (INV-11) — die Schließungsprüfung lebt im
Aufrufer.

„Mappe B" bündelt das zitierende Memo aus Einheit 3 als Kind statt
eines neutralen Kurzhinweises — macht die SCALE-3-Naht „Mappe B cites
Welt-Kristall" zu einer echten, überprüfbaren Tatsache statt eines
Labels. Blueprint-Zelle: struktureller Platzhalter (R3-Muster, echte
HBM-Facetten über `cce-hbm`) — die reale, aus dem Eigenkorpus
zertifizierte Blueprint-Kette liefert Etappe X2/E3. Zwei neue
Seed-Dateien (`scale2_projektmappe_b.loom`, `blueprint_reference_cube.loom`).

`scale3_adapter()` (mirrors `scale2_adapter`): 8/8-Parity grün.
MSC(1→2→3) über die bereits skalen-agnostische `cce-spiral::s15`-
Maschinerie — KEIN Core-Code geändert (`close_red`/
`multi_scale_closure` funktionieren unverändert für Skala 3).
Reanalyze-Kerntest (≃) grün. Schließungs-Zeuge: alle vier Zellen real
über `SeedResolver` auflösbar und nicht rot; UND mindestens ein in
Mappe B gebündeltes Kind zitiert (`CitationGate`, Gate-Pass) tatsächlich
den echten Welt-Kristall-`core_root` — keine Behauptung ohne Gegenprobe.

## Ausgangs-Gate E2 — Prüfung gegen Karte §2/E2 + Dokument 14

**Exit-Zeugen (Karte): „cites-Gate grün/rot (fehlendes Ziel,
Klassen-Mismatch) · Red(3)-Kerntest ≃ · MSC(1→2→3)":**
1. cites-Gate grün/rot — grün (15 Unit-Tests in loom-cites decken alle
   fünf Ausgänge deterministisch ab; die reale Kette in
   `e2_cites.rs`/`e2_scale3_project.rs` bestätigt es zusätzlich auf
   echten Containern).
2. Red(3)-Kerntest ≃ — grün (`red3_kerntest_project_seams_valid_and_
   reanalyze_identical`).
3. MSC(1→2→3) — grün (`multi_scale_closure_1_to_3_green`,
   `scale3_adapter_parity_8_of_8`).

**Vollständige Zeugenliste (Dokument 14 §I.7):** R-CIT-1 ✓ (`e2_cites.rs`),
R-CIT-2 ✓ (`e2_cites.rs`), R-CIT-3 ✓ (`e2_scale3_project.rs`), N-CIT-1 ✓
(`e2_cites.rs`, real), N-CIT-2 ✓ (`loom-cites::gate::tests`, Mock —
Resolver „lügt", Verteidigung in der Tiefe greift), N-CIT-3 ✓
(`loom-cites::gate::tests`, Mock — zweistufiger Zyklus über zwei
Container), N-CIT-4 ✓ (`loom-cites::gate::tests`, Mock — Ziel `Reject`),
N-CIT-5 ✓ (`e2_cites.rs`, real).

**Alle Alt-Zeugen unverändert:** voller Workspace-Testlauf nach jeder
Einheit und final: 161 Testgruppen, 0 Fehlschläge. R1–R8-Golden-Files,
alle X1-Zeugen (a–e), alle COCK-INV-Tests, CSA-Katalog, alle 213
Domänen-Referenz-Zeugen unangetastet und grün.

**CI GRUEN:** `cargo fmt --all`, `cargo clippy --workspace --all-targets
-- -D warnings`, `python3 ci/check_acyclic.py` (53 Workspace-Crates,
DAG, Schichten sauber), `bash ci/run_ci.sh` (Spec-Integrität + fmt-check
+ clippy + Azyklik + volle Testsuite) — alle grün.

**Kein neues Kern-Crate mit externen Abhängigkeiten:** `loom-cites` hat
KEINE externen Abhängigkeiten (nur first-party loom-*/cce-materialize).
`cce-materialize::scale3_project` fügt keine neue Abhängigkeit hinzu.
Dieselbe Disziplin wie in Ring E1.

**Ausgangs-Gate E2: ERFÜLLT.**

## Residuen

- Die Blueprint-Zelle für SCALE-3 (R-CIT-3) ist ein struktureller
  Platzhalter (R3-Muster) — die REALE, aus dem Eigenkorpus zertifizierte
  Blueprint-Kette ist ausdrücklich Sache von Ring E3 (als solche
  benannt, nicht als Endresultat behauptet).
- I.5 (Replay-Inputs) ist additiv umgesetzt (`RunDescriptor::input_digests`,
  `replay_manifest_segment_with_inputs`, `check_replay_inputs`) und
  unit-getestet, aber noch nicht in einen vollständigen End-to-End-Replay-
  Lauf über einen zitierenden Workbody eingebunden — dafür braucht es
  einen echten zweiten Motor-Lauf gegen dieselbe RD-Klasse, der in dieser
  Etappe nicht gefordert war (I.7 listet dafür keinen eigenen Zeugen).
  Sichtbar geführt, kein Verstecken.

## Nicht begonnen (auftragsgemäß)

E3 (HBM produktiv), E4 (CE-1, loom-sdk/wasm-Viewer, Klassen-Registry),
E5 (L9b Normic Memory) — laut Bau-Reihenfolge in Dokument 14 folgt jetzt
E3. Host-Leiste unverändert gesperrt. `spec/` unangetastet.

Abweichungen: keine.
