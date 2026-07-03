Etappe X1 — Ring E1 vollständig (Ökosystem-Expansionskarte §2/E1 + §3)

Eingang: 13_OEKOSYSTEM_EXPANSIONSKARTE.md aus origin/main synchronisiert
(Commit 070289d, einzeln cherry-gepickt — kein Merge der 51 divergenten
Commits). Auftrag: Ring E1 „Selbstenthaltung & Format-Vollreife",
Einheiten a→e, je mit Zeugen zuerst benannt, dann gebaut, kleiner Commit
je Einheit. E2–E5 nicht begonnen (Spec-Stücke stehen aus); Host-Leiste
unverändert gesperrt; spec/ unangetastet.

## Einheit (a) — Artefakt-Bytes im Container

Commit 535b28b. `loom-mount::extract`: neues Modul, liest den
deklarierten `byte_digest` aus dem ARTIFACT-Segment, findet den
CAS_BLOB-Frame mit genau diesem sha256 und liefert die Bytes erst nach
Digest-Gegenprobe — kein blindes Vertrauen auf Positions-/
Reihenfolge-Zufall. `all_cas_blobs()` liefert alle eingebetteten Blobs
(Grundlage für (a) und (b)). `loom-cli`: neue Kommandos `extract`
(ein Artefakt) und `extract-children` (mehrere, mit
Einzelverifikation). `build_welt_kristall_wikimedia()` trägt jetzt
einen CAS_BLOB-Frame mit den echten materialisierten Bytes neben dem
bestehenden ARTIFACT-Digest-Segment — schließt das im
Welt-Crystal-Bericht sichtbar geführte Muster-Residuum („Container
trägt nur Digest-Metadaten"). R1–R8 (Golden Files) bewusst
unverändert gelassen.

Zeuge (`x1a_artifact_bytes_in_container.rs`, 3 Tests): `extract() ==
materialize()`-Bytes, unabhängig aus der echten Fixture nachgerechnet;
Re-Import aus Container == Re-Import aus Datei == ursprüngliche
Crystal-Klasse (kein `semantic_loss`); fail-closed auf Containern ohne
ARTIFACT (R1) bzw. ohne CAS_BLOB (R7).

## Einheit (b) — SCALE-2-Vollmaterialisierung

Commit 5930892. `build_scale2_folder_full()`: zwei echte, heterogene
Dokument-Crystals (`three_risks_memo` + ein neuer Kurzhinweis
„Nahtstabilität", keine Klone) werden je einzeln real materialisiert
(cce-runner) und als eigenständige „workcell"-Arbeitskörper versiegelt
(`seal_document_workbody`). Die Mappe bündelt beide physisch — je ein
CAS_BLOB pro Kind. `cce-materialize::scale2_folder` bleibt unverändert:
Domänen-Logik und physisches Bündeln sind sauber getrennt.

Zeuge (`x1b_scale2_folder_full.rs`, 5 Tests): Mappen-Container selbst
`Valid`; `all_cas_blobs` liefert genau 2 Kinder, jedes für sich `Valid`
UND selbst wieder `extract_artifact`-fähig (rekursiv konsistent); die
im DOC-Segment deklarierten `child_core_root`-Werte entsprechen den
tatsächlich extrahierten (Mengenvergleich, ordnungsunabhängig);
Determinismus; Seed==Builder. Seed:
`library/seed/scale2_projektmappe_full.loom` (Generator
`scale2-folder-gen`, mehrfacher Lauf byte-identisch).

Gefundener und behobener Fehler unterwegs: `claims_closed=true` an
`manifest_cv` für „workcell"-Profil-Container führte zu
`claim_over_evidence`-Ablehnung (kein LEDGER-Segment vorhanden) —
korrigiert auf `false`.

## Einheit (c) — zstd-Transportprofil + blake3-Zweitprofil

Commit eb09529. `loom-cli::transport`: Ganzdatei-zstd-Kompression
(Stufe 19) bewusst AUSSERHALB des Frame-Formats (kein
`SEG_FLAG_COMPRESSED`-Pfad, keine Änderung an `loom-format`/
`loom-codec`) — Entpacken liefert byte-identisch die Eingabe zurück,
die kanonisch-gespeicherte Form bleibt Golden-Referenz. Neue
CLI-Kommandos `pack-zstd`/`unpack-zstd`. `loom-cli::hashprofile`:
blake3-Zweitprofil, additiv zu den sha2-256-Frame-Digests (die bleiben
unverändert, hand-geführt, core-only). Neues CLI-Kommando
`hash-profile`. Deklaration im MANIFEST über die neue, rein additive
Hilfsfunktion `manifest_declare_hash_profiles`. Beide externen Kisten
(`zstd`, `blake3`) leben ausschließlich im CLI-Blatt.

Zeuge (`x1c_transport_and_hash_profiles.rs`, 3 Tests): zstd-Roundtrip
klassenidentisch zu stored auf DREI echten, committeten
Arbeitskörpern (Welt-Crystal, SCALE-2-Mappe, Drei-Risiken-Memo) —
gleicher `core_root`, gleiches Verdikt; manipulierte zstd-Bytes
fail-closed abgewiesen; blake3-Deklaration lesbar, unabhängig
reproduzierbar, kontextabhängig. Blake3-Testvektoren wurden gegen die
echte `blake3`-Kiste in einer Probe-Crate nachgerechnet (nicht aus dem
Gedächtnis übernommen) — ein anfänglicher Tippfehler im erwarteten
Digest wurde dabei gefangen.

## Einheit (d) — .docx-Export hinter dem materialize-Vertrag

Commit 43a101b. Neues Blatt-Crate `cce-docx-export`: `render_docx(&DocWeave)
-> DocxArtifact` nimmt DIESELBE Eingabe wie `render_markdown` — nur die
Ausgabeform ist anders. Schließt S1.10-R1 („Bibliothekswahl offen"):
`zip` + handgeschriebenes Minimal-OOXML (drei Teile:
`[Content_Types].xml`, `_rels/.rels`, `word/document.xml`),
ausschließlich in diesem Blatt-Crate — `cce-materialize` selbst bekommt
keine neue Abhängigkeit, nur eine erweiterte `export_formats()`-
Deklaration (`.md`, `.docx`). Verlustform sichtbar gemacht:
`format_loss_residue()` benennt, dass `.docx` keine Struktur-Anker
trägt und daher KEIN verlustfreier Reanalyse-Pfad ist wie `.md`.

Zeuge (`cce-docx-export`, 6 Tests): gültiges ZIP mit den erwarteten
Teilen + echtem Memo-Inhalt lesbar; Determinismus; XML-Escaping;
`format_loss`-Residuum immer vorhanden; „docx-Roundtrip-Klasse"
(`extract_paragraph_texts` liefert dieselben Textläufe wie das
Original-Gewebe, Inhalts- nicht Struktur-Ebene); fail-closed auf
Nicht-ZIP-Bytes. Real erprobt mit `python-docx` (unabhängiger,
weitverbreiteter OOXML-Parser): erfolgreich geöffnet, alle 8 Absätze
wortgleich extrahiert. Ehrlicher Nebenbefund: LibreOffice ist in dieser
Sandbox selbst für ein reines `.txt`→`.pdf` defekt (kein
docx-spezifisches Problem, bestätigt an einer unveränderten
python-docx-Referenzdatei) — daher `python-docx` als reale Gegenprobe
genutzt statt eines kaputten Headless-Office.

## Einheit (e) — Signatur-Registry-Vollform

Commit 2c9e021. `sign()` akkumuliert jetzt additiv statt vorherige
SIGNATURE-Segmente zu ersetzen; jede Signatur trägt eine Rolle
(`ROLE_AUTHOR`, `ROLE_REVIEWER`, `ROLE_REVIEW_GATE` — letztere der
Familie-P-Slot für menschengebundene PL4-Prüfung, OHNE zu behaupten der
Agent leiste diese Prüfung selbst). `verify_sig()` bleibt für
Abwärtskompatibilität unverändert (erste gefundene Signatur); neu
`verify_sig_all()` prüft die gesamte Kette und meldet jede Signatur
unabhängig grün/rot, ohne bei der ersten ungültigen abzubrechen. CLI:
`sign` nimmt optional eine Rolle als 4. Argument (Default `author`);
`verify-sig --all` listet jede Rolle mit Gültigkeit + `public_key`.

Manuell end-to-end mit dem echten Binary gegen den realen
Welt-Kristall-Seed verifiziert: 3er-Kette (Autor→Prüfer→ReviewGate),
`core_root` unverändert, Container bleibt `Valid`, alle drei
Signaturen gültig. Zeuge (`x1e_signature_registry.rs`, 2 Tests) +
`loom-cli`-Unit-Tests (10 gesamt, 3 neu): dieselbe Kette automatisiert
plus eine gezielt manipulierte Signatur — genau 1 von 2 wird rot, die
andere bleibt grün (Manipulation über `decode_sealed` → Cv-Feld ändern
→ `seal_canonical` neu versiegeln, nicht per rohem Byte-Flip, da das
die Frame-Digest-Prüfung vorzeitig auslösen würde statt die
Ed25519-Prüfung selbst zu testen).

## Ausgangs-Gate X1 — Prüfung gegen §3

**Fünf Exit-Zeugen (Karte §2/E1 + §3):**
1. `extract == materialize`-Bytes — grün (Einheit a, 3 Tests).
2. Mappe → n valide Kinder — grün (Einheit b, 5 Tests, n=2).
3. zstd-Container klassenidentisch zu stored — grün (Einheit c, 3 Tests,
   3 reale Arbeitskörper).
4. docx-Roundtrip-Klasse — grün (Einheit d, 6 Tests, Inhalts-Roundtrip
   + python-docx-Gegenprobe).
5. Mehrfachsignatur-Kette grün/rot — grün (Einheit e, 2 Konformitäts-
   + 3 neue Unit-Tests: additive Kette UND gezielt manipulierte
   Einzelsignatur je einzeln korrekt gemeldet).

**Alle Alt-Zeugen unverändert:** voller Workspace-Testlauf nach jeder
Einheit und final: 154 Testgruppen, 418 Einzeltests, 0 fehlgeschlagen.
R1–R8-Golden-Files, alle COCK-INV-Tests, CSA-Katalog, alle 213
Domänen-Referenz-Zeugen unangetastet und grün.

**CI GRUEN:** `cargo fmt --all` sauber, `cargo clippy --workspace
--all-targets -- -D warnings` sauber, `bash ci/run_ci.sh` (Spec-
Integritätsprüfung + fmt --check + clippy + `check_acyclic.py` +
volle Testsuite) grün.

**Kein neues Kern-Crate mit externen Abhängigkeiten:** `zstd` und
`blake3` ausschließlich in `loom-cli` (Blatt); `zip` ausschließlich in
`cce-docx-export` (neues, eigenes Blatt-Crate — kein bestehendes
Kern-Crate wurde erweitert). `loom-format`, `loom-codec`, `loom-canon`,
`cce-materialize`, `cce-core` bleiben ohne neue externe Abhängigkeiten.
Dieselbe Disziplin wie beim Ed25519-Pfad (P6c).

**Ausgangs-Gate X1: ERFÜLLT.**

## Residuen (Register nachgeführt, siehe reports/residuen.md)

- S1.10-R1 (Bibliothekswahl `.docx` offen) — GESCHLOSSEN durch Einheit (d).
- Signatur-Registry-Vollform (aus F_P6c_bericht.md als Folgeschritt
  geführt) — GESCHLOSSEN durch Einheit (e). Der `ROLE_REVIEW_GATE`-Slot
  ist eine strukturelle Signaturposition für die Familie-P-Regel
  (menschengebundene PL4-Freigabe); er ersetzt keine echte menschliche
  Prüfung — das bleibt ausdrücklich außerhalb der Möglichkeiten dieses
  Agenten.
- Neu gemeldet: LibreOffice ist in dieser Sandbox generell (nicht nur
  für `.docx`) funktionsunfähig für Konvertierungen — dokumentiert als
  Umgebungsgrenze, umgangen durch `python-docx` als unabhängige
  Gegenprobe (siehe R-Agent-8 in residuen.md).
- `SEG_FLAG_COMPRESSED` (Frame-interne Kompression) bleibt weiterhin
  nicht implementiert — bewusst nicht angefasst; zstd wurde als
  Ganzdatei-Transport-Wrapper AUSSERHALB des Frame-Formats gelöst, was
  die Karte in §2/E1(c) explizit so vorsieht („canonical-stored bleibt
  Golden-Referenz").

## Nicht begonnen (auftragsgemäß)

E2 (Verbund & Skalen), E3 (Selbstbezug: HBM produktiv), E4
(Erweiterbarkeit & SDK), E5 (Gedächtnis: L9b Normic Memory) — Spec-
Stücke (S-E2a, S-E4a, S-E5) stehen laut Karte §4 noch aus. Host-Leiste
(GGUF/LLM, OS-Keyring-Live, macOS/Windows-Pakete, GPU-Klickpfad)
unverändert gesperrt. `spec/` unangetastet.

Abweichungen: keine.
