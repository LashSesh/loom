# Etappe P5 — RepoIntelligence: das Repo beweisbar verstehen
(Dokument 23 Track B; Stand: gebaut bis zum benannten STOPP)

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

## STOPP (Dokument 23 Track B): Vorschlag für den echten Lauf R-RIG-1

**Vorschlag: der projekteigene `numkit`-Testling als Erstziel** — laut
Dokument 23 ausdrücklich zulässig und lizenzfrei unbedenklich; es ist
dasselbe Paket, das P4/P4-Ext als Vergleichsgegenstand nutzt (drei
Dateien: Cargo.toml, src/lib.rs mit `max_of`, tests/it.rs). Der
`#[ignore]`-Betriebs-Harness liegt bereit
(`crates/cce-repointel/tests/r_rig_betriebsverifikation.rs`):

```
cargo test -p cce-repointel --test r_rig_betriebsverifikation -- --ignored --nocapture
```

- materialisiert numkit nach /tmp und liest es mit ECHTEN fs-Reads;
- Commit-Achse ehrlich als `tree:<sha256 über sortierte Datei-Digests>`
  gekennzeichnet (numkit ist kein git-Repo; für ein echtes git-Ziel
  trüge hier der HEAD-SHA);
- fährt die volle Kette, verifiziert beide Körper, schreibt
  `source.loom` + `blueprint.loom` NUR nach /tmp und druckt
  packet_digest / blueprint_class / Regeln / offene Entscheidungen.

Ein externes MIT-Repo als ZWEITZIEL bleibt möglich (der Harness nimmt
`CCE_RIG_TARGET_DIR`/`CCE_RIG_REPO_ID`/`CCE_RIG_LICENSE`), aber laut
Dokument 23 nur nach ausdrücklicher Freigabe.

**Es wartet:** die Freigabe des Ziel-Repos. Bis dahin läuft kein
realer R-RIG-1.
