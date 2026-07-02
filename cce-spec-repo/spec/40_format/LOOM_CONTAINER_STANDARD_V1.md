# LOOM WORKBODY CONTAINER STANDARD v1.0 (.loom)

**Das eigene, mountbare Dateiformat für closure-zertifizierte KI-Arbeitskörper.** Kein JSON mit Hypercube-Label, kein Ordnerlayout, kein Promptpaket: `.loom` ist die physische Außenform der CCE — eine Datei, die wie ein Dokument geöffnet, wie ein Container gemountet und wie ein replayfähiger Arbeitskörper ausgeführt werden kann.

**Status:** Vollstandard v1.0, normativ, auf Byte-Ebene implementierbar. Arbeitet die Rahmenverfassung „LOOM Container Codec Standard v0.1" vollständig aus (Phasen F1–F7). Rollen unverändert: **Constraint Lattice = Substrat, PHC = interner Codec/Projektionskörper, LOOM = generative Auswebung und Runtime-Klasse, `.loom` = physische Außenform.** Horizontformel: `.loom = Container(CL, PHC, LOOM_profile, CSA_packs, HBM_blueprints, Ledger, Replay)`.

**Schlüsselwörter** MUSS/DARF NICHT/SOLL sind normativ.

---

# TEIL 1 — Physische Formatentscheidung und Begründung

**Entscheidung: `.loom` ist ein segmentierter Binary-Container („LBC-1") — Hybrid aus eigenem Byte-Rahmen, deterministischem CBOR für alle signaturrelevanten Segmente, content-adressierten Segmenten mit Merkle-Root und Footer-Index für Random Access.** Das ist die fünfte in v0.1 zugelassene Designfamilie („kleiner Header + Segmenttabelle + CAS-Blobs + kanonische Meta-Segmente"), final ausgeprägt.

Begründung entlang der neun unverhandelbaren Auswahlkriterien (v0.1 §9.2):

1. **Deterministic canonical bytes:** dCBOR (RFC 8949 §4.2.1 Core Deterministic Encoding) + LOOM-CANON-1-Zusatzregeln (Teil 4) liefern „gleiche Semantik ⇒ gleiche Bytes" formatnativ — ohne die Fragilität von ZIP (Timestamps, Eintragsreihenfolge, Kompressorvarianz) und ohne den Versionsdrift von SQLite-Pagelayouts.
2. **Random Access:** Segmenttabelle mit absoluten Offsets, über einen fixen 64-Byte-Footer auffindbar — O(1)-Sprung zu jedem Segment.
3. **Streaming-Inspection:** feste 16-Byte-Präambel, Header und MANIFEST stehen physisch vorn; Frames sind längen- und digest-präfixiert — ein Viewer liest Präambel+Header+Manifest und weiß, was er hat, ohne die Datei zu Ende zu lesen.
4. **Content-addressed + Root-Digest:** jedes Segment trägt einen Multihash über seine unkomprimierte kanonische Payload; ein Merkle-`core_root` bindet alle Core-Segmente; die Datei selbst ist CAS-fähig über ihren Byte-Digest — das **Zwei-Digest-Modell** (S7) auf Containerebene.
5. **Versionierung/Profilverhandlung:** Major/Minor in der Präambel, `required_profiles`/`optional_profiles` im Header, additive Kind-Registry.
6. **Negative Files scheitern gezielt:** jede Prüfstufe (Präambel, Frame, Digest, Tabelle, Root, Kanon, Schema, Semantik) ist ein isolierter, testbarer Fehlpunkt.
7. **Unabhängiger Viewer:** L0–L2-Validierung benötigt nur CBOR-Decoder + SHA-256 — zwei ubiquitäre Bausteine; Reader-Prinzip erfüllt.
8. **Runner mountet ohne Raten:** registrierte Kinds + normative Segmentverträge (Teil 3).
9. **Workbench schreibt ohne Replay-Bruch:** Draft-Journal (append-only) → Seal; `core_root` hängt nur von der logischen Tabelle ab, nicht von physischer Schreibreihenfolge.

**Verworfene Alternativen (mit Grund):** *ZIP/TAR-Innenordnung* — fremdes Magic (kein eigenes Format), Kanonisierung dauerfragil, Pfadnamens-Angriffsklasse (Zip-Slip), Tooling verführt zum Ordner-Denken; bleibt als Debug-Exportprofil erlaubt (`loom export --profile debug-tree`), nie als Kernform. *SQLite/DuckDB* — exzellenter Random Access, aber Byte-Determinismus über Bibliotheksversionen praktisch nicht garantierbar (Pages, Freelists, Vacuum) → man bräuchte ohnehin eine kanonische Exportform = zweite Wahrheit. *Reines IPLD/CAR* — CAS-Idee richtig, aber CARv1 ohne Index, Ökosystem-Kopplung an CID/IPFS unnötig; wir übernehmen Merkle+Multihash ohne die Abhängigkeit. *JSON-only* — per Rahmenverfassung ausgeschlossen; ein JSON-only-Pseudocontainer ist Negativdatei N12.

---

# TEIL 2 — Präambel, Magic, MIME, Version, Profile

## 2.1 Namensraum

Extension **`.loom`** · Formatname **LOOM Workbody Container** · MIME **`application/vnd.cce.loom`** · Formatklasse **LBC-1** (LOOM Binary Container, Fassung 1).

## 2.2 Präambel (Bytes 0–15, fix)

```
Offset  Länge  Inhalt
0       9      Magic: 89 4C 4F 4F 4D 0D 0A 1A 0A   (\x89 "LOOM" CR LF SUB LF)
9       1      format_major = 0x01
10      1      format_minor = 0x00
11      1      flags: bit0 sealed · bit1 has_compressed_segments · bit2–7 = 0 (MUSS 0)
12      4      header_len (u32, little-endian)
```

Das PNG-artige Magic schützt gegen 7-Bit-Kanäle (0x89), Zeilenenden-Konversion (CR LF … LF) und DOS-`type`-Abbruch (0x1A). Alle Mehrbyte-Zahlen im Rahmen sind **little-endian**.

## 2.3 Header (direkt nach Präambel, `header_len` Bytes, dCBOR-Map)

Pflichtfelder: `format_profile:"LBC-1"` · `digest_alg` (Multihash-Code; Kern: `0x12` sha2-256) · `canon_id:"loom-canon-1"` · `canon_rules_digest` (Digest des CANON_DESC-Segments) · `container_class ∈ {inspection, workcell, source, hbm, runtime, full}` · `profiles_required[]` · `profiles_optional[]` · `min_reader_version {major,minor}`. Der Header ist signaturrelevant: sein Digest geht als Pseudo-Eintrag `kind=0x0000` in die Segmenttabelle und damit in `core_root`.

## 2.4 Footer (letzte 64 Bytes, nur bei `sealed=1`)

```
[0..8)    segtab_offset   u64 LE   (absoluter Dateioffset des SEGTAB-Frames)
[8..16)   segtab_stored_len u64 LE
[16..50)  core_root       Multihash (0x12 0x20 + 32 Byte SHA-256)
[50..56)  reserved = 0
[56..64)  End-Magic: 4C 4F 4F 4D 5F 45 4E 44   ("LOOM_END")
```

Eine Datei ohne gültigen Footer bei `sealed=1` ist ungültig (N9-Klasse). `sealed=0` (Draft/Journal) hat keinen Footer und ist **nur workspace-intern** gültig — nicht transport-, export- oder importfähig.

## 2.5 Profile

`Profile ∈ {inspection, workcell, source, hbm, runtime, full}` (v0.1 §9.1). Pflichtsegmente je Profil in §3.4. Profilverhandlung: Ein Reader MUSS `profiles_required` vollständig verstehen oder mit begründetem `reject`/`quarantine` enden — nie stilles Teilverstehen.

---

# TEIL 3 — Segmentmodell und Segmentverträge

## 3.1 Frame-Format (jedes Segment auf Platte)

```
[0..2)    kind              u16 LE   (Registry §3.3)
[2..4)    seg_flags         u16 LE   bit0 compressed_zstd · bit1 non_core · bit2 required_understand
[4..12)   uncompressed_len  u64 LE
[12..20)  stored_len        u64 LE
[20..54)  payload_digest    Multihash(34B) über die UNKOMPRIMIERTE kanonische Payload
[54..)    stored payload    (stored_len Bytes; zstd-19-nodict falls bit0)
```

Signaturrelevant ist **immer** die unkomprimierte kanonische Payload; Kompression ist reine Transportschicht. Reader MUSS `uncompressed_len` als hartes Dekompressionslimit durchsetzen (N13).

## 3.2 Segmenttabelle (SEGTAB, kind 0x0002, dCBOR)

Autoritative logische Sicht: Array von Einträgen `(kind, digest, uncompressed_len, offset, stored_len, seg_flags, deps[digest])`, **sortiert nach (kind, digest)**; enthält den Header-Pseudo-Eintrag `kind=0x0000`, aber **keinen Selbsteintrag** — der eigene Digest kann nicht Inhalt der Tabelle sein; Auffindung und Bindung der Tabelle leistet der Footer (§2.4). **Referenzen zwischen Segmenten erfolgen ausschließlich per `digest`** (`loom:seg:sha256:<hex>`), nie per Offset — Offsets sind Tabellen-Privileg (CAS-Tauglichkeit). Duplikate (gleicher Digest) sind zulässig genau einmal physisch (Dedupe-Pflicht beim Seal); zyklische `deps` sind ungültig (N11).

## 3.3 Kind-Registry (u16)

| Kind | Name | Inhalt (Kurzvertrag) |
|---|---|---|
| 0x0000 | HEADER (Pseudo) | §2.3 |
| 0x0001 | MANIFEST | §3.5 |
| 0x0002 | SEGTAB | §3.2 |
| 0x0003 | CANON_DESC | vollständiger Regeltext LOOM-CANON-1: Sortierung, Zahl-/Text-/Zeit-Regeln, Tag-Whitelist, Tie-Breaks (Teil 4) |
| 0x0004 | TYPE_REGISTRY | Typen, Rollen, Domain-/Scale-/Source-Bindungen; verweist auf Katalog-IDs (S1) |
| 0x0010 | CL_SUBSTRATE | Cube, Constraints, Entfaltungsgraph, chordales Skelett, Junction Tree, Separatoren, Gate-Definitionen |
| 0x0011 | PHC | Achsen, Zelladressen, Workcells, Projektionen, Exportprofile — PHC-Pakete als interner Codec; Kristalle liegen hier |
| 0x0012 | SPIRAL_RATCHET | Spiraladressen, WrapPolicy, DispersionProfile, BlueCube/RedCube-Status, Phase-/ScaleRatchets |
| 0x0013 | LEDGER | PhaseBlocks (10-Tupel), Commits, GateReport-Refs, SourceRunHyperDAG-Projektion; `Ledger = CommitProjection(H)` |
| 0x0014 | RESIDUE | sichtbare Residuen, Counter-Horizon, Hold/Reject/Quarantine-Zustände — Pflicht, sobald irgendein Residuum existiert |
| 0x0015 | GATE_REPORTS | boolesche, begründete Verdikte; **kein Score-Feld als Verdikt** (N-Klasse „score_as_gate") |
| 0x0016 | EVIDENCE | EvidencePacks, MEF-/Proof-Payloads, Transformationspfade |
| 0x0017 | REPLAY_MANIFEST | RD, Seeds, Versionen, Snapshot-Refs, Tie-Breaks, Prüfpfade, Reanalysevertrag |
| 0x0018 | RUNTIME_PROFILE | erlaubte Ops, Toolverträge, deklarierte CapabilityLocks, Workcell-Mount-Profile |
| 0x0020 | CSA_NSB | SourceHorizon-Beleg, SourceAdapter-Manifestbelege, NexusSourceBundles: CSUs + EvidencePacks + source_graph + Residuen — **einziger** Weg externer Daten (§8) |
| 0x0021 | HBM | Facets, Cube/HDAG, Skeleton/JunctionTree, BlueprintCandidates, GateReports, Crystals, Replay — ephemere Suchpfade bleiben draußen |
| 0x0030 | ARTIFACT | materialisierte SCALE-1-Artefakte (oder Refs auf CAS_BLOB) mit Zwei-Digest-Bindung (S7) |
| 0x0031 | CAS_BLOB | rohe content-adressierte Nutzdaten (RawObservations, große Payloads) — immer von EVIDENCE/CSA referenziert, nie „nackt normativ" |
| 0x0032 | DOC | Operator-Doku-Slot (S12), Claim-Schranke gilt |
| 0x0033 | LIBRARY_WITNESS | eingebettete Referenz-/Negativ-Zeugen oder Refs (S8) |
| 0x0050 | SIGNATURE | detached Signaturen über `core_root`; `non_core` erzwungen (kann nicht Teil des Signierten sein) |
| 0x7000–0x7FFF | EXT | Erweiterungen; `non_core` erzwungen — **Extensions ändern Core-Hashes nicht** (K6) |

Unbekannter Kind: ohne `required_understand` → **preserve** (mitkopieren) + sichtbar im Inspektionsbericht; mit `required_understand` → `quarantine`.

## 3.4 Pflichtsegmente je Profil

Alle Profile: MANIFEST, SEGTAB, CANON_DESC (+RESIDUE, sobald Residuen existieren). **inspection:** nichts weiter. **workcell:** + CL_SUBSTRATE, PHC, RUNTIME_PROFILE, GATE_REPORTS. **source:** + CSA_NSB, EVIDENCE. **hbm:** + HBM, EVIDENCE. **runtime:** + LEDGER, REPLAY_MANIFEST. **full:** alle Kern-Kinds 0x0001–0x0021 (0x0030–0x0033 nach Inhalt).

## 3.5 MANIFEST-Vertrag (Pflichtfelder)

`title` · `container_class` · `domain_refs[]` (Katalog-IDs) · `scale` (SCALE-n) · `pl_level` (PL0–PL4, Anti-Overclaim) · `claims{}` (unter Claim-Schranke; „certified/closed" nur mit grünem Abschlussbeweis im LEDGER) · `origin{parent_core_root?, tool, rd_digest}` · `profiles_required[]/optional[]` · `residue_summary{count, kinds[]}` (MUSS mit RESIDUE konsistent sein — N6-Klasse) · `capability_declarations[]` (deklarativ, §8/§9) · `license_summary` · `created` (Evidence-Zeitfeld).

---

# TEIL 4 — Canonicalization, Hash, CAS, Root-Digest

## 4.1 LOOM-CANON-1

Alle signaturrelevanten Segmente MÜSSEN dCBOR nach RFC 8949 §4.2.1 sein, verschärft: definite lengths only; kürzeste Integer-Kodierung; Map-Keys nur `tstr`/`uint`, bytewise aufsteigend sortiert, keine Duplikate; **keine Floats** — Zahlen als Integer oder Decimal-Fraction (Tag 4, Integer-Mantisse) [K3]; Text NFC-normalisiert; Tag-Whitelist {0 (Zeit, nur in Evidence-Feldern), 4}; **keine Wall-Clock in signaturrelevanten Pfaden** außer als deklariertes Evidence-Feld; Seeds/Tie-Breaks ausschließlich im REPLAY_MANIFEST (RD-gebunden); Randomness nur seeded [K4]. Ergebnis: gleiche Semantik ⇒ gleiche kanonische Bytes [K5]; deterministisch sortiert [K1]; IDs content-addressed [K2]; Extensions ändern Core-Hashes nicht [K6].

## 4.2 Digests und Merkle-Root

Digest-Familie: **Multihash**; Kernprofil `sha2-256` (0x12, Länge 0x20) — korpuskonsistent (`sha256:`-IDs in RD/FetchPlan). Registrierte Zusatzprofile (z. B. blake3) sind zulässig, MÜSSEN im Header deklariert sein; ein Container nutzt genau **eine** Digest-Familie.

```
Blatt_i  = H(0x00 ‖ dCBOR(kind_i, digest_i, uncompressed_len_i))     (alle SEGTAB-Einträge mit non_core=0;
                                                                      SEGTAB selbst ist eintragslos, §3.2)
Knoten   = H(0x01 ‖ links ‖ rechts)        (ungerades Blatt wird unverändert hochgezogen)
core_root = Wurzel
```

Domain-Separation (0x00/0x01) verhindert Second-Preimage über Baumformen; „promote odd" (kein Duplizieren) verhindert die Duplikations-Ambiguität. **Identitätsformel** (v0.1 §10.2 erfüllt): `id(file.loom) = core_root` erreicht Header, Manifest und alle Core-Segmente inkl. ReplayManifest — und die Segmenttabelle als deren **erzeugende Funktion**: `core_root` ist die Faltung ihrer Einträge, jede Tabellenmanipulation ändert die Wurzel; der Footer bindet Offset/Länge der Tabelle.

## 4.3 Zwei-Digest-Modell (Containerebene)

**`core_root`** = Inhaltsklassen-Digest des Arbeitskörpers (Zertifikate, CAS-Identität der Klasse, Signaturen binden hieran). **`file_byte_digest`** = SHA-256 über die gesamten Dateibytes (Transport-/CAS-Schlüssel der konkreten Datei; steht naturgemäß außerhalb der Datei). Kompression und physische Reihenfolge dürfen variieren, ohne die Klasse zu ändern; das Byte-Konformanzprofil **`canonical-stored`** (alle Segmente unkomprimiert, physisch in Tabellenordnung, via `loom seal --canonical`) macht zusätzlich die Dateibytes deterministisch — Grundlage der Golden-Files.

## 4.4 CAS-Integration (S9)

Segmente sind CAS-Objekte (`loom:seg:sha256:<hex>`); der Container ist ein CAS-Objekt (`file_byte_digest`); die Klasse ist ein CAS-Anker (`core_root`). Der S9-CAS adressiert alle drei; `verify_hdag_projection` gilt für das LEDGER-Segment unverändert.

---

# TEIL 5 — Loader, Mount, Projection, Gate, Replay, Export

## 5.1 Loader-Phasen (normativ, aus v0.1 M1–M10)

M1 **Recognize** (Präambel, Version, Profilverhandlung) → M2 **Index** (Footer, SEGTAB, `core_root`) → M3 **Verify** (Frame-Digests, Tabellen-Konsistenz, Merkle-Root, CANON_DESC, Pflichtsegmente) → M4 **Reconstruct** (CL, PHC-Profil, Workcell-Index) → M5 **Bind Evidence** (EvidencePacks, CSA-Bundles, HBM-Facets, Ledgerpfade) → M6 **Project** (lokale Workcells) → M7 **Activate Runtime** (CapabilityLocks setzen) → M8 **Run/Inspect** → M9 **Commit** (jede Änderung als neuer PhaseBlock/Version, nie still in-place) → M10 **Replay/Reanalyze** (gegen Crystal-Klasse und ReplayManifest). Viewer endet bei M5; Mount-RO bei M6; nur Runner/Workbench erreichen M7+.

**Öffnen ist reine Verifikation + Deserialisierung.** Es gibt keinen Hook-, Autostart- oder Makro-Mechanismus im Format (§9.1).

## 5.2 Verifikationsstufen und Verdikte

**L0** Struktur (Präambel/Footer/Frames/Digests/SEGTAB/Root) · **L1** Kanon+Schema (dCBOR-Konformität, Segmentvertrag je Kind) · **L2** Semantik (Profil-Pflichtsegmente; Referenzen auflösbar + azyklisch; **EvidencePack-Pflicht für jede CSU in CSA_NSB**; GateReport je Ledger-Commit; `residue_summary` ≡ RESIDUE; Claims ≤ Beweislage) · **L3** Klassen-Replay (Reanalyze ≃ Crystal; benötigt Motor-Ports). Verdikte: `valid | valid_with_residues | quarantine | reject` — nie stilles Teilergebnis. Ein Viewer erreicht L0–L2 **ohne Projektwissen** (Reader-Prinzip: `Openable(file.loom) ⇒ ∃ Viewer, der ohne Projektwissen validieren kann`).

## 5.3 Mount-Vertrag

`Mount(file.loom) = { π_v(Σ) | v ∈ V_workcell, Gate(π_v) = ready }` — der Container ist ein **mountbarer Tesseract**: er liefert getypte, lokale, hinreichende, gate-fähige Projektionen; er legt den globalen Horizont **nie** als chaotischen Vollprompt frei. Modi: **inspect** (read-only Struktur/Evidence/Gates/Residuen/Ledger; keine Workcell-Ausführung) · **mount-ro** (Workcells projizieren, keine Commits) · **run** (Ausführung unter CapabilityLocks; erzeugt Evidence/GateReports/PhaseBlocks) · **workbench** (erzeugen, erweitern, reweben, neue Versionen) · **repair/quarantine** (isolierte Analyse; kein Export-, Run- oder Import-Pfad).

## 5.4 API-Mindestvertrag (normativ)

```
open(path) -> LoomHandle                          // M1–M2, KEINE Ausführung
inspect(handle) -> InspectionReport               // Manifest, Segmente, Residuen, Claims
verify(handle, level=L2) -> VerificationReport    // Verdikt + begründete Fehlpunkte
mount(handle, mode, profile) -> MountedWorkbody
project(mount, workcell_id) -> LocalProjection    // ProjectionPacket-konform (S13/HBM)
run(mount, workcell_id, input) -> RunReport       // CandidateOutput, KEIN Commit
commit(mount, run_report) -> PhaseBlock | ResidueReport   // Gate+Evidence+Replay-pflichtig
replay(handle, rd) -> ReplayReport                // gleiche Commit-Klasse ⇒ Pass
export(handle, sink_profile) -> ExportBundle      // ExportGate: Lizenz-/Attribution-Transport
```

`run` erzeugt **nie** direkt Wirklichkeit: `CandidateOutput —Gate,Evidence,Replay→ PhaseBlock | ResidueReport`. Die Gate-API ist read-execute: Gates lesen Containerinhalt und urteilen; **kein** API-Pfad schreibt Verdikte von außen.

---

# TEIL 6 — Viewer, Runner, Workbench, SDK (getrennte Rollen)

| Rolle | Aufgabe | Grenzen |
|---|---|---|
| **LOOM Viewer** | öffnen, L0–L2 validieren, inspizieren; Manifest, Evidence, Residuen, Gates, Ledger anzeigen; Export **ohne** Ausführung | keine Capabilities, keine Netzaktivität, kein Run, kein Commit |
| **LOOM Runner** | mounten, Workcells unter CapabilityLocks ausführen, Agenten/Tools lokal binden, Gates prüfen, PhaseBlocks schreiben (Draft→Seal als neue Version) | Netz **nur** via CSA-Kette; keine Lock-Umgehung; kein in-place-Write |
| **LOOM Workbench** | Container erzeugen (`pack`), PHC/LOOM/CSA/HBM-Segmente authoren, reweben, migrieren, versionieren, releasefähig machen (S15-Bindung: `.loom` ist das Austausch-/Persistenzformat der WorkbenchCapsule/Multicube-Snapshots) | jede Änderung = neue Version/PhaseBlock; Claims unter Beweislage |
| **LOOM SDK** | Bibliotheks-Oberfläche = §5.4 für Fremdanwendungen (parse, validate, inspect, mount, run, export, replay) | erzwingt dieselben Verdikte/Locks; keine „unsafe open"-Abkürzung |
| **Conformance Suite** | Referenz-/Negativdateien + Testmatrix (Teil 10) | CI-gebunden an den S8-Regressionswächter |

**Minimal-GUI-Inspektionspfad (normativ):** Jeder grafische Viewer MUSS mindestens fünf read-only-Ansichten aus denselben `InspectionReport`-/`VerificationReport`-Daten der SDK-API rendern: (1) Manifest (Klasse, PL, Claims, Profile), (2) Segmentliste mit Digest-Status, (3) Residuen + Verdikt, (4) Gate-Reports, (5) Ledger/PhaseBlocks — ohne Ausführung, ohne Netz, ohne Schreibpfad. Das Feindesign ist Residuum LC-R5.

---

# TEIL 7 — Kompatibilität und Migration

**Kompatibilitätsklassen** (v0.1 §11.1, normativ): *Read-compatible* (neuer Viewer liest alte Datei) · *Run-compatible* (neuer Runner führt alte Workcells aus oder hält begründet) · *Replay-compatible* (neue Runtime reproduziert alte Commit-Klasse) · *Write-compatible* (neue Workbench migriert alte Datei) · *Seal-compatible* (signierte Segmente bleiben unverändert prüfbar).

**Regeln:** `format_minor`-Erhöhung = additiv (neue Kinds/Felder; alte Reader: preserve+sichtbar) — Read/Replay/Seal-kompatibel garantiert. `format_major`-Erhöhung = Strukturbruch, **nur** mit Migrationspfad. **Migration ist ein gegateter Lauf:** `Migrate(loom_v) → loom_{v+1}` mit Evidence, MigrationLedger, altem und neuem `core_root` und **Reanalysebericht** (Klassenvergleich; Klassenwechsel MUSS begründet und sichtbar sein). Alte Datei bleibt unangetastet (append-only-Denken auf Dateiebene); `origin.parent_core_root` verkettet die Provenienz. Eine alte Version ohne deklarierte Migration ist N10.

---

# TEIL 8 — CSA-, HBM- und Quell-Segmente (externe Welt)

Externe Daten erreichen den Container **ausschließlich** über die Akquisitionskette: `Source → CSA → CSU + EvidencePack → NexusSourceBundle → PHC/HBM` — gespeichert im CSA_NSB-Segment mit Transformationspfaden, Lizenz-/Policy-Zuständen und Residuen. Der Container speichert **keine rohen Webbehauptungen als Strukturwahrheit**; eine CSU ohne EvidencePack ist Negativfall N5, ein CAS_BLOB ohne Evidence-Referenz ist nicht-normativ. HBM-Segmente transportieren nur das Persistierungswürdige: `HBM_seg = (Facets, Cube/HDAG, Skeleton, Candidates, Gates, Crystals, Replay)` — ephemere Suchpfade und Zellen verschwinden (Dissolution). **Ein Container beschreibt Netzquellen, führt aber nie Akquise aus:** neue Akquisitionen sind neue CSA-Läufe (SourceHorizon + PolicyGate + CapabilityLock + Ledger), keine impliziten Containeraktionen.

---

# TEIL 9 — Security-, Quarantine- und Capability-Modell

## 9.1 Keine versteckte Ausführung

Das Format kennt **keine** Hooks, Makros, Autostarts oder eingebetteten Interpreter-Direktiven. `open`/`inspect`/`verify` sind seiteneffektfrei. Ein Container, der Ausführung beim Öffnen beansprucht, ist Negativfall (reject). Ausführung beginnt erst im Runner/Workbench-Modus nach Capability-/Policy-/Gate-Prüfung.

## 9.2 Parser-Härtung (Pflichten des Readers)

Harte Limits: `uncompressed_len` als Dekompressionsschranke (Bombe ⇒ N13, reject vor Allokation); `max_segments`, `max_frame_len`, `max_deps_depth` als deklarierte Reader-Limits; keine Pfad-Semantik im Format (logische IDs statt Dateinamen ⇒ keine Traversal-Klasse); Digest-Prüfung **vor** semantischer Deserialisierung; azyklische `deps` erzwungen.

## 9.3 CapabilityLocks

`capability_classes = { read_segment, inspect_evidence, project_workcell, run_local_tool, write_phaseblock, export_artifact, network_request, memory_commit, source_acquisition }`. Der Container **deklariert** (MANIFEST/RUNTIME_PROFILE), welche Operationen vorgesehen sind; die **Runtime entscheidet** anhand des lokalen Umfelds (S13-Locks), welche tatsächlich aktiv werden — Deklaration ist nie Aktivierung. `network_request` und `source_acquisition` sind **immer** CSA-gebunden (PROD-INV-13/14 gelten im Format). Eine Workcell, die eine nicht deklarierte Capability fordert, ist N8 (capability_escalation).

## 9.4 Quarantine

Defekte, unvollständige oder policy-kritische Container MÜSSEN in Quarantäne öffenbar sein: inspizierbar (L0–L1, so weit möglich), aber **nicht** exportierbar, **nicht** mount-run-fähig, **nicht** als Crystal importierbar. Quarantäne ist ein sichtbares Verdikt mit Begründung, kein stiller Zustand. Der einzige Weg hinaus ist **Repair**: gezielte Korrektur im Repair-Modus, erneute Gate-Prüfung, neues Verdikt — **kein Exportpfad ohne neue Gates**.

## 9.5 Signaturen (optional, empfohlen)

SIGNATURE-Segment (0x0050, non_core): detached Signaturen über `core_root` (+ optional `file_byte_digest`), Mehrfachsignaturen zulässig (Autor, Prüfer, ProfessionalReviewGate-Freigaben der Familie P). Signaturverfahren ist Registry-Sache (Empfehlung: Ed25519); Schlüsselverwaltung liegt außerhalb des Formats (OS-Schlüsselbund, S11).

---

# TEIL 10 — Reference-Files, Negative-Files, Conformance-Suite, CLI

## 10.1 Pflicht-Referenzdateien (alle im `canonical-stored`-Profil als Golden Files)

**R1** Minimaler Inspect-Container (Manifest, SEGTAB, CANON_DESC, leeres-aber-valides CL-Segment) · **R2** LocalCorpus-Container (lokale Quelle → CSU/EvidencePack → NSB → PHC-Projektion) · **R3** HBM-Blueprint-Container (Facets, Skeleton/JT, Candidate, Gate, Crystal) · **R4** Workcell-Container (mehrere lokale Projektionen, Allowed Ops, GateReports) · **R5** Replay-Container (Ledger vorhanden, reproduzierbare Commit-Klasse) · **R6** Source-Container (CSA-Segmente mit SourceHorizon-Beleg und EvidencePacks) · **R7** Full-Workbody-Container (CL + PHC + Workcells + HBM + CSA + Ledger; der Drei-Risiken-Memo-Referenz-Cube aus S1 als Inhalt).

## 10.2 Pflicht-Negativdateien

**N1** falsches Magic / Profilkonflikt · **N2** fehlendes Pflichtsegment · **N3** nichtkanonische Map-Sortierung (dCBOR-Verstoß) · **N4** Root-Hash-Mismatch (manipuliertes Segment) · **N5** EvidencePack fehlt für externe CSU · **N6** GateReport fehlt für Commit / `residue_summary` widerspricht RESIDUE · **N7** ReplayManifest unvollständig · **N8** CapabilityLock verletzt (nicht deklarierte Capability gefordert) · **N9** SourceHorizon überschritten (CSA-Segment außerhalb HS) / sealed ohne gültigen Footer · **N10** alte Version ohne deklarierte Migration · **N11** zyklische oder ungültige Referenzstruktur · **N12** JSON-only-Pseudocontainer ohne Loader-/Mount-Vertrag · **N13** Dekompressionsbombe (stored≪declared-Verstoß) · **N14** Score-als-Verdikt im GATE_REPORTS-Segment.

Jede N-Datei MUSS am dafür definierten Prüfpunkt scheitern (gezieltes Scheitern), mit maschinenlesbarer Diagnose.

## 10.3 Conformance-Ebenen

**C0** Byte/Struktur: `decode(R_i)` fehlerfrei; `encode(decode(R_i))` byte-identisch im canonical-stored-Profil · **C1** Kanonisierung: permutierte semantisch gleiche Eingaben ⇒ identische `core_root` · **C2** Verdikte: alle R → `valid`/`valid_with_residues` korrekt, alle N → `reject`/`quarantine` am richtigen Prüfpunkt · **C3** API-Verhalten: `open/inspect/verify` seiteneffektfrei; Viewer ohne Netz-Capability; mount-ro schreibt nie · **C4** Replay: R5 reproduziert dieselbe Commit-Klasse · **C5** Migration/Profile: Minor-additive Datei bleibt Read/Replay/Seal-kompatibel; N10 scheitert. Die Suite ist CI-gebunden: derselbe S8-Regressionswächter, der Domänen, Updates, CoreExtensions und CSA-Zeugen wacht, wacht die 7+14 Formatzeugen.

## 10.4 Mindest-CLI

```
loom inspect file.loom                      loom verify file.loom [--level L0..L3]
loom mount file.loom --mode read-only       loom ls file.loom [seg]      loom cat file.loom <seg-digest>
loom run file.loom --workcell <id>          loom replay file.loom
loom export file.loom --profile <profile>   loom pack <workspace> --out project.loom
loom seal draft.loom [--canonical]          loom migrate old.loom --to <version>
loom conformance ./test-suite
```

---

# TEIL 11 — Repository-/Crate-Blueprint (Referenz-Codec)

```
cce-loom/                        # Rust-Workspace, azyklisch, Ports zu cce-* Crates
  crates/loom-format             # Präambel/Footer/Frames/Kind-Registry, Typen
  crates/loom-canon              # LOOM-CANON-1: dCBOR-Encoder/-Prüfer, NFC, DecFrac
  crates/loom-codec              # encode/decode/pack/seal (journal & canonical)
  crates/loom-verify             # L0–L3, Verdikte, Diagnosen, Merkle
  crates/loom-mount              # Mount-Modi, virtuelle Sicht, Limits (§9.2)
  crates/loom-project            # Workcell-Projektionen; Ports: cce-phc, cce-lattice
  crates/loom-gate               # Gate-Ausführung über Containerinhalt; Port: cce-core
  crates/loom-replay             # RD-Replay, Klassenvergleich; Port: cce-runner
  crates/loom-export             # ExportBundle, Lizenz-/Attribution-Transport; Port: cce-materialize
  crates/loom-viewer             # Referenz-Viewer (CLI/TUI), L0–L2 ohne Projektwissen
  crates/loom-runner             # Runner mit CapabilityLocks; Ports: cce-nexus-acquisition (CSA), cce-hbm
  crates/loom-cli                # Befehle aus §10.4
  crates/loom-conformance        # R1–R7, N1–N14, C0–C5-Matrix
  golden/                        # canonical-stored Golden Files
  schemas/                       # CDDL je Segment-Kind
```

Abhängigkeitsrichtung: `loom-*` nutzt `cce-*` nur über deklarierte Ports (INV-11 azyklisch); kein `cce-*`-Crate hängt von `loom-*` ab außer `cce-runner`/`cce-observe` über den SDK-Port.

---

# TEIL 12 — Build-Handoff für den Coding-Agenten

**Reihenfolge (jede Stufe mit grünen Tests vor der nächsten):** B1 `loom-canon` (+C1-Kern) → B2 `loom-format` (Präambel/Frames/Footer) → B3 `loom-codec` (pack/seal; R1 erzeugen) → B4 `loom-verify` L0–L1 (+N1/N3/N4/N13/N14) → B5 SEGTAB/Merkle/`core_root` (+C0/C1 voll) → B6 `loom-mount` + `loom-viewer` (L2; Reader-Prinzip-Nachweis: Viewer-Build ohne cce-Motor) → B7 R2–R7 über Motor-Ports (cce-phc/cce-lattice/cce-nexus-acquisition/cce-hbm) → B8 `loom-runner` (CapabilityLocks; N8/N9) → B9 `loom-replay` (C4) → B10 `loom-export` (ExportGate, Attribution) → B11 `loom-migrate` (C5, N10) → B12 volle Conformance + CI-Bindung an den S8-Wächter.

**Unantastbar:** F1–F6, INV-1..14, V1–V10, PROD-INV-9..16; Score nie Gate (N14 wacht); keine versteckte Ausführung; Netz nur via CSA; Draft nie transportfähig; Claims nie über Beweislage; alle Residuen sichtbar. **Keine neuen Architekturentscheidungen nötig:** Byte-Layout (Teil 2/3), Kanon (Teil 4), Verdikte (Teil 5), Rollen (Teil 6), Zeugen (Teil 10) und Reihenfolge (oben) sind vollständig fixiert; verbleibende Wahlfreiheiten sind als Residuen ausgewiesen (Teil 13) und blockieren den Bau nicht.

---

# TEIL 13 — DoD, sichtbare Residuen, Anschluss

## 13.1 Abnahme

```
DoD(LOOM_CONTAINER_STANDARD) = 1 ⟺
    .loom als eigener Containerstandard gesetzt (Fileformat/Codec/Container/Runtime/Workbench getrennt, Teil 0–6)
  ∧ finale physische Kodierung gewählt und begründet (LBC-1, Teil 1)
  ∧ Magic/Header/Version/Manifest/Segmenttabelle/Root-Digest/Profile konkret (Teil 2–4)
  ∧ alle Pflichtsegmente + Segmentverträge definiert; CL/PHC/CSA/HBM/Spiral-Ratchet/S15 segmentfähig (Teil 3, 8)
  ∧ Canonicalization/Hashing/CAS/core_root/Zwei-Digest-Modell definiert (Teil 4)
  ∧ Loader/Mount/Projection/Gate/Replay/Export normativ (Teil 5); keine versteckte Ausführung; Netz nur via CSA (Teil 8–9)
  ∧ Viewer/Runner/Workbench/SDK/Conformance getrennt spezifiziert (Teil 6, 10)
  ∧ Kompatibilitätsklassen + gegatete Migration (Teil 7)
  ∧ Security/Quarantine/Capability-Modell (Teil 9); Reference-Files (7) + Negative-Files (14) (Teil 10)
  ∧ Conformance-Suite + Mindest-CLI + Crate-Blueprint + Build-Handoff (Teil 10–12)
  ∧ JSON-only-Pseudocontainer ausgeschlossen (N12)
  ∧ ein Coding-Agent kann Referenz-Codec, Viewer, Runner und Conformance-Suite ohne neue
    Architekturentscheidung implementieren (Teil 12)
```

Damit sind zugleich alle 15 DoD-Punkte der Rahmenverfassung v0.1 §15 erfüllt.

## 13.2 Sichtbare Residuen

**LC-R1** Signaturverfahren-Registry (Empfehlung Ed25519; Festlegung + Schlüsselbund-Bindung beim Bau, mit S11) · **LC-R2** blake3-Zweitprofil (registriert, optional; Aktivierung offen) · **LC-R3** CDDL-Vollschemata je Kind (`schemas/`; Feldlisten hier normativ, maschinenlesbare CDDL = Bauphase B4) · **LC-R4** Kind-Registry-Governance (neue Kinds nur via S14-CoreExtension-Pfad; Registerführung beim Bau) · **LC-R5** GUI-Feindesign des Inspektionspfads (Mindestvertrag mit fünf Pflichtansichten normativ in Teil 6; grafische Vollform = S3-Cockpit-Anschluss, später).

## 13.3 Anschluss

**Build-Closure-Matrix:** neue Stufe **h = LoomContainerDoD** (= DoD dieses Standards, auf Papier **1**); `PlatformFoundationDoD = c ∧ d ∧ e ∧ g ∧ h`. **Landkarte:** `.loom` ist die physische Außenform quer zu L2 (LOOM/Workbody), L9a (CSA-Bundles), L10 (WorkbenchCapsule-Austausch) und L11 (S7-Artefakt-Transport, S11-Auslieferung von Saat-Bibliothek und Zeugen als `.loom`). **S8:** die 7 Referenz- und 14 Negativdateien treten als Bibliothekszeugen ein, unter demselben Wächter.

**Schlussformel:** *Constraint Lattice gibt die Semantik. PHC gibt die interne Projektionsform. LOOM gibt die generative Auswebung. `.loom` gibt dem Ganzen einen Körper: eine Datei, die nicht nur gelesen, sondern als closure-zertifizierter Arbeitsraum gemountet werden kann.*

*Ende des LOOM Workbody Container Standard v1.0.*
