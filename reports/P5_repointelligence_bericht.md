# Etappe P5 — RepoIntelligence: das Repo beweisbar verstehen
(Dokument 23 Track B; Stand: ABGESCHLOSSEN — R-RIG-1 real erbracht)

Eingang: Track A abgeschlossen (`reports/A_augen_bericht.md`).
Dokument 23 Track B wörtlich: CCE liest ein *fremdes* Repo über die
**bestehenden CSA-Adapter** (Lizenz-/Policy-Gates unverändert scharf),
destilliert dessen Strukturmuster über die **bestehende HBM-Kette**,
kompiliert daraus ein **GroundingPacket** (Dokument 21) und versiegelt
das Ganze als zertifizierten **Bauplan-Workbody** mit `cites` auf die
Quell-Evidence — `verify == Valid`, replay-identisch.

## Was gebaut wurde (alles hermetisch GRUEN in der Default-CI)

**Crate `crates/cce-repointel`** (Blatt, reine Komposition; KEINE
cce-inference-/cce-toolgateway-Kante — P5 braucht weder Modell- noch
Werkzeug-Egress; Tor-Trennung bleibt trivial wahr):

- **`observe.rs` — `structural_kv_v1`:** die deterministische
  Struktur-Beobachtung. Repo-Dateien → EIN `kv_lines`-Dokument
  beobachteter Tatsachen (je Datei: Pfad, **SHA-256 der rohen Bytes**
  als Bindung an den fixen Stand, Zeilenzahl; je .rs-Datei
  konservative Stichwort-Zählungen: pub-fn / #[test] / unsafe /
  .unwrap() / panic!; aus Cargo.toml die Edition; repo-weit:
  LICENSE-Datei vorhanden?). Sortierte Dateifolge — Eingabereihenfolge
  ändert nichts. Dieselbe dokumentierte Stichwort-Disziplin wie das
  RuleComplianceGate (Dokument 21): Zählung, keine Semantikanalyse.
- **`ingest.rs`:** der Einzug WOERTLICH über die bestehende CSA-Kette,
  Reihenfolge exakt wie der CSA-Referenzzeuge: `GitRepositoryAdapter`
  (unverändert) → preflight → plan → **`approve_fetch`** (die sechs
  Policy-Gates, fail-closed, versiegelter Plan — u. a. das
  unveränderte `license_gate`) → **`fetch` nur mit versiegeltem
  Plan** → extract (`decode_kv_lines`) → normalize → validate +
  schema/quality/provenance → **EvidencePack je CSU** →
  `NexusSourceBundle` → Ledger-Head. Kein Socket, kein Bypass.
- **`distill.rs`:** CSU-Tatsachen → Facetten-Zeilen
  (MiningProfile-Vokabular: entity/measure/boundary_contract/gate/
  constraint/invariant/risk) → **unveränderte HBM-Kette**
  (`run_pipeline`) → zertifizierte Blueprint-Kandidaten. Daneben
  mechanische Ableitung: repo-weit beobachtete Invarianten
  (unsafe-frei, unwrap-frei in src/, panic-frei in src/) werden
  **belegte blocking-RuleAtoms** (`evidence_ref` → CSU-Feld,
  `gate_ref: RuleComplianceGate`); offene Punkte (keine LICENSE-Datei,
  beobachtete unwrap-Altlast) werden **offene DecisionSlots** —
  `compile_grounding` (Dokument 21, unverändert) kompiliert das
  GroundingPacket.
- **`workbody.rs`:** ZWEI Körper. (1) Quell-Container, bestehende
  Klasse **`"source"`** (CSA_NSB mit csu_uids+evidence_for je CSU +
  EVIDENCE mit den EvidencePacks; ehrlich `claims.closed=false` — kein
  Ledger, kein Abschlussanspruch). (2) Bauplan-Container, NEUE Klasse
  **`"blueprint"`** (additiv in `loom_format::PROFILES` — jetzt 10 —
  und `loom-verify::required_kinds`: CL_SUBSTRATE + HBM + LEDGER +
  RESIDUE + EVIDENCE + REPLAY_MANIFEST): GroundingPacket + zertifizierte
  Blueprints + **`cites` auf den Quell-Container** (Derives;
  `external_citations` == CL-cites, N-CIT-5-Disziplin), Facetten/
  Skelett/Ranking im HBM-Segment, offene Entscheidungen SICHTBAR im
  RESIDUE-Segment (Warning; Manifest-`residue_summary` ==
  Segment-Zählung, N6), REPLAY_MANIFEST mit deterministischer
  Destillations-RD und `blueprint_class_digest` als Klasse.
- **`kette.rs` — `run_repo_intelligence`:** EIN Aufruf, fail-closed an
  jeder Naht, mit eingebauter **Replay-Selbstprüfung**: ein zweiter
  voller Lauf derselben Eingabe MUSS dieselbe Bauplan-Klasse ergeben,
  sonst `repointel_replay_divergent`.
- **`residues.rs`:** fünf sichtbare Residuen (`rig:`-Namensraum):
  policy_blocked, empty_ingest, no_certified_blueprint,
  decision_left_open (Warning), replay_divergent.

## Zeugen (Wächter `conformance/repointel/`, Default-CI)

| Zeuge | Aussage |
|---|---|
| R-RIG-STRUCT | volle Kette am Fixture: Quell-Container `verify == Valid`; Bauplan `ValidWithResidues` (die offene LICENSE-Frage steht SICHTBAR im RESIDUE-Segment — ehrliches Verdikt, kein leeres Feld durch Weglassen); Regeln alle belegt, keine Herabstufung; ≥1 zertifizierter Blueprint |
| R-RIG-REPLAY | zwei volle Läufe ⇒ identische Bauplan-Klasse, identischer packet_digest, **byte-klassenidentische core_roots** |
| N-RIG-1 | `proprietary_no_reuse` ⇒ das UNVERÄNDERTE CSA-`license_gate` hält VOR jeder Destillation — kein Bundle, kein Bauplan, fail-closed |
| N-RIG-2 | offene Entscheidung (numkit ohne LICENSE) sichtbar als Warning-Residuum, blockiert die Siegelung nicht |
| Wert-Beweis | der destillierte Bauplan ist NUTZBAR: sein GroundingPacket fängt via `rule_compliance_gate` (Dokument 21) einen Diff, der die am Commit nachgewiesene Repo-Invariante bricht (`.unwrap()` in src/) — „versteht es beweisbar" |

Dazu 11 Unit-Zeugen im Crate (Beobachtung deterministisch +
bytes-gebunden + strikt kv-konform; Einzug mit EvidencePack je CSU;
Destillation deterministisch; Kette replay-identisch).

## Verifikation

`cargo fmt --check` · `cargo clippy --workspace --all-targets`
(Default UND `--features process --features http`) · `python3
ci/check_acyclic.py` (**59** Workspace-Crates, DAG, Schichten sauber —
`cce-repointel` mit benannten CSA-/loom-Ports eingetragen, dieselbe
Ausnahme-Disziplin wie cce-bridge/cce-swe/cce-benchmark) ·
`bash ci/run_ci.sh` — alles GRUEN, Alt-Zeugen unverändert.

## R-RIG-1 — der eine reale Lauf (nach STOPP-Freigabe: „Ja" → numkit)

Nach dem benannten STOPP wurde **numkit** als Erstziel vorgeschlagen
und vom Auftraggeber ausdrücklich freigegeben. Der Lauf
(`cargo test -p cce-repointel --test r_rig_betriebsverifikation --
--ignored --nocapture`):

| Feststellung | Wert |
|---|---|
| Ziel | `/tmp/cce-rig-1-numkit` — 3 Dateien, ECHTE fs-Reads (kein Fixture-Bypass) |
| Commit-Achse | `tree:f2b51faa2e626864d05c8b7126d1ce58ca8f7ad7ecb7027147b727101a1e2fc0` (ehrlich als Baum-Digest gekennzeichnet — numkit ist kein git-Repo; ein git-Ziel trüge den HEAD-SHA) |
| deklarierte Lizenz | `cc0` — CSA-Policy grün |
| Quell-Container | `verify == Valid` |
| Bauplan-Container | `verify == ValidWithResidues` — das EHRLICHE Verdikt: die offene LICENSE-Frage steht sichtbar im RESIDUE-Segment |
| `packet_digest` | `f24b3d01c712b07e7bb985bf5951b10323f6978b218ad9eac06c093aa810a99d` |
| `blueprint_class` | `de4fcef9ea34deb6a32a7db0dbc8a57c1adb47b4d295b14f231222c1ef098a81` |
| Regeln | 3, ALLE belegt (keep-unsafe-free, keep-unwrap-free-src, keep-panic-free-src — evidence_ref auf die commit-gebundenen CSU-Felder), keine Herabstufung |
| offene Entscheidungen | 1 — `license-clarification` (keine LICENSE-Datei beobachtet), sichtbar als `rig:repointel_decision_left_open` |
| zertifizierte Blueprints | 31 |
| Artefakte | `source.loom` (3 156 B) + `blueprint.loom` (13 020 B), NUR nach `/tmp/cce-rig-1-out` |

**Replay-Beweis über Prozessgrenzen:** ein ZWEITER, unabhängiger
Harness-Lauf (neuer Prozess, neue Materialisierung, neue fs-Reads)
ergab **identischen `packet_digest` UND identische `blueprint_class`**
— zusätzlich zur ketteninternen Replay-Selbstprüfung, die in jedem
Lauf einen vollständigen Zweitdurchlauf erzwingt.

**Gegenprobe mit dem ausgelieferten Werkzeug:** die frisch gebaute
`loom`-CLI verifiziert `/tmp/cce-rig-1-out/blueprint.loom` zu
`ValidWithResidues`. Ehrlicher Nebenbefund: ein VERALTETES
`loom`-Binary (gebaut vor der Klassen-Einführung) weist denselben
Container mit `profile_unknown` ZURÜCK — genau das gewollte
fail-closed-Verhalten alter Prüfer gegenüber neuen Klassen, kein
stilles Teilverstehen.

**Damit ist die Track-B-Aussage real erbracht:** CCE hat ein fremdes
Repo über die unveränderten CSA-Tore gelesen, sein Strukturverständnis
über die unveränderte HBM-Kette destilliert, daraus ein belegtes,
sofort nutzbares Regelwerk (Dokument 21) kompiliert und beides als
prüfbare, replay-identische `.loom`-Körper versiegelt — inklusive der
sichtbaren Wahrheit über das, was OFFEN bleibt. Ein externes MIT-Repo
als Zweitziel bleibt möglich (Harness nimmt `CCE_RIG_TARGET_DIR`),
laut Dokument 23 nur nach Freigabe.
