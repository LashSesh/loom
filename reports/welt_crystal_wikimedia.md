# Erstes Welt-Crystal — Inspect-Beleg (Vollausbau Block 3)

Milestone-Nachweis: der JSON→CSU-Extraktor für die `official_api`-Klasse
(Wikimedia-Adapter) normalisiert die eingefrorene, ECHTE Wikimedia-
Antwort (`conformance/fixtures/wikimedia_kristall.json`) zu einer
vollwertigen CSU mit EvidencePack + CC-BY-SA-Attribution — und daraus
entsteht der ERSTE zertifizierte Arbeitskörper mit echter Weltquelle:
`library/seed/kristall_wikimedia_workbody.loom`.

## Was neu gebaut wurde

1. **`nexus-decode::decode_json`** — ein kleiner, selbst geführter
   rekursiver JSON-Decoder (Bytes → `CanonValue`), ohne externe Kiste
   (dieselbe Disziplin wie beim Ed25519-Pfad: fremde Crates bleiben im
   CLI-Blattcrate). Deckt Objekte/Arrays/Strings (inkl. `\uXXXX`- und
   Surrogatpaar-Escapes)/Ganzzahlen/einfache Dezimalbrüche/Bool/Null;
   Exponentialschreibweise ist bewusst `schema_unparseable` statt einer
   stillen Fehlrundung. 6 Zeugen, davon einer direkt gegen die echte,
   eingefrorene Fixture.
2. **`WikimediaAdapter::extract`** ruft jetzt `decode_json` auf (vorher
   `decode_kv_lines` — ein Platzhalterformat, das die echte MediaWiki-
   Antwort nie hätte lesen können) und navigiert `query.pages.<id>` —
   die numerische Seiten-ID ist bei MediaWiki nicht vorhersagbar, die
   Navigation läuft daher über den (bei Einzeltitel-Abfrage einzigen)
   Karteneintrag, nicht über eine geratene ID.
3. **`WikimediaAdapter::validate`** prüft jetzt zusätzlich, dass
   `title`/`extract` als Text in der Payload vorhanden sind (echtes
   Schema statt nur Lizenzprüfung).
4. **`build_welt_kristall_wikimedia()`** (`loom-conformance`): baut die
   reale `DocCrystal` — Quellenzelle (`d1`, Definition) ist der ECHTE
   Wikimedia-Auszug, `a1` (Support) trägt die echte Attribution-Zeile
   des Adapters — lässt sie über den echten Motor (`cce-runner`)
   laufen und versiegelt ECHTE Digests (kein Platzhalter wie
   `"sha256:artifact-bytes"` in den R1–R8-Golden-Files).

## Der zertifizierte Arbeitskörper

`library/seed/kristall_wikimedia_workbody.loom` — 4831 Bytes,
deterministisch (zweifach erzeugt, byte-identisch).

**`loom inspect` (echter Aufruf, `loom-cli`, motorfrei):**
```
segmente: [0, 1, 3, 16, 17, 19, 20, 21, 22, 23, 24, 32, 33, 48, 50]
core_root: 1220b3f33b6b4f7a25cf372251346da62ed14e44153723657cf5daa9fb42c79cd9ab
verdikt: Valid · residuen: 0
```

**`loom verify`:** `verdikt: Valid`

Segmente (Klartext): HEADER, MANIFEST, CANON_DESC, CL_SUBSTRATE, PHC,
LEDGER, RESIDUE, GATE_REPORTS, EVIDENCE, REPLAY_MANIFEST,
RUNTIME_PROFILE, CSA_NSB, HBM, ARTIFACT, DOC — 15/15, alle vom
Profil "full" (wie R7) verlangt, keines fehlt.

## Die reale Quellenzelle (Beleg, nicht Behauptung)

**CSU** (`WikimediaAdapter::normalize`, echt aus der Fixture berechnet):
```
uid:      csu:011d1442255dae091b8a36aec84eda452d3691c63075aee3a9e6ce99876c2242
license:  cc-by-sa-4.0
quality:  (850, 900, 950)  (ψ, ρ, ω — Promille, reine Anzeige, kein Gate-Ersatz)
```

**Attribution** (`WikimediaAdapter::cite`, PROD-INV-16, real transportiert
im EVIDENCE-Segment):
```
Wikimedia-Beitraegerinnen und -Beitraeger · https://de.wikipedia.org/w/api.php?action=query&prop=extracts&exintro=1&explaintext=1&titles=Kristall&format=json&redirects=1 · Lizenz cc-by-sa-4.0 · hash 853db8073617953f457cb4f185c338c6c9d39745179a7a86f916a691b1f9cdb1
```

**Der echte Auszug** (564 Zeichen, wortwörtlich aus der eingefrorenen
MediaWiki-Antwort, nur zeilenkollabiert — s. Residuum unten):

> Ein Kristall ist ein Festkörper, dessen Bausteine – z. B. Atome,
> Ionen oder Moleküle – regelmäßig in einer Kristallstruktur angeordnet
> sind. Bekannte kristalline Materialien sind Kochsalz, Zucker,
> Minerale und Schnee – aber auch die Metalle. Aufgrund der
> regelmäßigen Anordnung der Teilchen spricht man auch von ihrer
> Fernordnung im Kristall, die mathematisch durch die
> Translationssymmetrie beschrieben werden kann. Die Wissenschaft von
> den Eigenschaften und Formen der Kristalle ist die Kristallographie.
> Eng verwandt sind die Metallographie und die Mineralogie.

**Crystal `content_class`:**
`b332d4e0b8f8e3d444ba775aa50630898936b5d195d0264f1e57770b0758b62e`
— unabhängig im Zeugen `seed_carries_the_real_wikimedia_extract_and_attribution`
gegen den committeten Container nachgerechnet (grün).

**Die DocCrystal-Struktur** (`kristall_memo_from_wikimedia`):

| Einheit | Typ | Naht | Inhalt |
|---|---|---|---|
| `s1` | Section | — | „Quelle: Wikipedia-Artikel „Kristall" (CC BY-SA 4.0)" |
| `d1` | Definition | `refers→s1` | der obige, echte Wikimedia-Auszug |
| `a1` | Support | `supports→d1` | die obige, echte Attribution-Zeile |

Alle 7 Dokument-Gates grün (Coverage, Support, NoScore, Structure,
NonContradiction, Seam, RoundTrip) — bewiesen durch den echten
Motor-Lauf (`Run::submit`+`run_to_end`), nicht nur behauptet.

## Zeugen (grün, frisch in dieser Sitzung ausgeführt)

- `nexus-decode`: 6 Tests, davon `json_decodes_the_real_frozen_wikimedia_fixture`
  direkt gegen die echte Fixture.
- `nexus-adapter-wikimedia`: 3 Tests, davon
  `real_frozen_wikimedia_fixture_becomes_a_real_csu` das Milestone-Kernstück.
- `loom-conformance` (`welt_kristall_wikimedia.rs`): 4 Tests —
  Determinismus, Seed==Builder, `loom_verify` Valid/0 Residuen, und der
  Inhalts-/Attributions-Beleg samt unabhängiger Klassen-Nachrechnung.
- `conformance` (`csa_catalog.rs`): 19 Tests weiterhin grün (2 Zeugen
  auf realistische MediaWiki-JSON-Bytes umgestellt, da die alten
  kv-Zeilen-Platzhalterbytes mit dem echten Extraktor kein gültiges
  JSON mehr sind — Absicht, nicht Regression).
- Voller Workspace-Testlauf: 147/147 grün, `cargo fmt --check` +
  `clippy -D warnings` sauber, `ci/run_ci.sh` grün (inkl. Azyklik-Check
  für die neuen Pfad-Abhängigkeiten).

## Offene Punkte (Residua, ehrlich geführt)

- **Container trägt Digest-Metadaten, keine Roh-Artefakt-Bytes** — wie
  bei R1–R8 (`build_r7` etc.) enthält der Container Manifest/Evidence/
  Doc-Metadaten + Zwei-Digest-Paar, nicht die materialisierten
  `.md`-Bytes selbst als eigenes Segment. Das ist das etablierte Muster
  dieses Bau-Zyklus (kein neuer Präzedenzfall), nicht eine Abkürzung
  für dieses Milestone. Der lesbare Text steht oben transkribiert +
  im Zeugen `seed_carries_the_real_wikimedia_extract_and_attribution`
  gegen den echten Adapterpfad geprüft.
- **Zeilenkollaps des Auszugs**: `document/parse.rs` ist zeilenbasiert
  (ein Einheitstext = genau eine Zeile vor dem `<!--cce:unit...-->`-
  Anker); ein eingebettetes `\n` im Original-Auszug (zwei Sätze über
  zwei MediaWiki-Absätze) würde beim Reanalyze sonst den ersten Absatz
  verlieren — deshalb wird `\n` durch ein Leerzeichen ersetzt, bevor der
  Text zur Einheit wird. Wortlaut sonst unverändert, keine Kürzung.
- **`official_api`-Klasse jetzt JSON-fähig, andere Adapter noch kv-Zeilen**:
  `local_corpus`/`git_repository` nutzen weiterhin `decode_kv_lines` —
  das ist für diese Klassen weiterhin das deklarierte, passende Format
  (Korpus-/Repo-Metadaten), keine Lücke dieses Auftrags.
