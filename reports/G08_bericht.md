Phase: G8 Source-Akquisition (CSA, nexus/-Workspace)
Eingang erfüllt: ja (G7-Gate grün, reports/G07_bericht.md)
Gebaut:
- `nexus-core`: SourceHorizon (HS-9-Tupel, S13-A4), TaskSpec (scale_target=1,
  data_policy no_pii), CandidateSource (passiv bis PolicyGate), RawObservation
  (content-adressiert), CSU (uid/kind/payload/schema/Qualität (ψ,ρ,ω) in
  Promille/provenance/license/source_hash/residues/domain_facet, Canonicalize),
  EvidencePack (mit Attribution-Feld, PROD-INV-16), NexusSourceBundle
  (every_csu_has_evidence, PROD-INV-15), SourceRunDescriptor mit HART
  VERDRAHTETER disallowed_actions-Liste (6 Einträge: captcha_bypass,
  paywall_bypass, auth_circumvention, bot_protection_evasion,
  rate_limit_evasion, terms_violation — kein Konstruktor ohne die Liste);
  18 CSA-Residuen (ALL_CSA_RESIDUES) + Vier-Wege-Verdikt
  (allow|hold|reject|quarantine), boolesch-begründet.
- `nexus-policy`: **ApprovedFetchPlan als VERSIEGELTER Typ** (privates
  _sealed-Feld, einziger Konstruktor `approve_fetch()`); Gates 1–6/15
  (SourceHorizon, SourcePolicy [Manifest 9/9], Access [disallowed_action ⇒
  Reject-ENDZUSTAND], RobotsTerms, License [cc-by ⇒ Attributionspflicht],
  Privacy). NoFetchBeforePolicyGate ist damit ARCHITEKTONISCH: der Netzpfad
  akzeptiert ausschließlich diesen Typ (PROD-INV-13).
- `nexus-fetch`: einziger Abrufpfad `fetch(&ApprovedFetchPlan, …)`;
  Transport-Trait mit SnapshotTransport (deterministisch, netzfrei);
  RateBudgetGate (7/15); Differenzabruf über ETag/Cursor (FetchCache, S9-A5).
- `nexus-adapter`: AdapterManifest (9 Pflichtfelder, fehlend ⇒
  manifest_missing, Quelle bleibt passiv); SourceAdapter-Portvertrag
  (8 Methoden) + check_source_adapter_parity (CSA.16).
- `nexus-ingress` (Discovery ohne Abruf, exploration_out_of_horizon sichtbar),
  `nexus-decode` (deklarierte Decoder, Schlüsselvalidierung, fail-closed
  schema_unparseable), `nexus-normalize` (CSU-UID = Content-Adresse der
  kanonischen Payload ⇒ Dedup-Grundlage), `nexus-store` (CAS + Snapshot-Index,
  replay-stabil), `nexus-validate` (Gates 8–10: Schema, Qualität mit
  Achsen-Minima τ, Provenienz; rank_d ganzzahlig; Dedup per kanonischer
  Klasse), `nexus-evidence` (Gate 11: EP-Pflicht + Attributionspflicht),
  `nexus-ledger` (Ledger = CommitProjection des Source-Runs, Gate 12 Replay),
  `nexus-export` (Gates 13–15: HBM-Import, PHC-Projektion, Export mit
  Lizenz-/Attribution-Transport).
- `nexus-cell`: Akquisitionszellen als Workbench-Capsules; Source-Ratchet
  advance(R_src)=1 ⟺ Gate=Pass ∧ Evidence=1 ∧ Residue sichtbar; Zelle nur
  aus versiegeltem ApprovedFetchPlan konstruierbar; CSA-INV-2 (kein
  Policy-Drift): unveränderlicher Policy-Schnappschuss ohne Mutationspfad;
  Dissolution konsumiert die Zelle (nur Protokoll bleibt).
- 3 Referenz-Adapter: LocalCorpusAdapter (local_corpus),
  Wikimedia-OfficialAPIAdapter (official_api, cc-by-sa MIT
  Attribution-Transport), Git-RepositoryAdapter (git_repository,
  Provenienz = Commit-SHA+Pfad, Replay = fixer Commit).
- `nexus-cli`: dünner Treiber (Referenzlauf über die volle Kette,
  Gate-Bilanz; kein eigener Netzpfad).
Ausgangs-Gate:
- CSA-Zeugen: 5 Referenzen grün (conformance/tests/csa_catalog.rs:
  ref_1_local_corpus_full_chain_green, ref_2_wikimedia_official_api_with_
  attribution, ref_3_git_repository_commit_bound, ref_4_feed_differential_
  fetch_via_etag, ref_5_hbm_import_green) = grün
- 8 Negative rot: neg_1_policy_blocked_fetch_no_socket_before_policy_gate
  (Zähl-Transport beweist: 0 Zugriffe vor PolicyGate), neg_2_rate_limit_
  violation, neg_3_evidence_missing, neg_4_license_incompatible,
  neg_5_replay_drift, neg_6_score_as_gate_attempt (strukturell, V1),
  neg_7_source_unknown, neg_8_html_scope_leak (Discovery-Verwurf +
  Decoder-fail-closed) = grün
- PROD-INV-13..16 als Negativ-Tests: prod_inv_13 (versiegelter Typ),
  prod_inv_14 (disallowed_action ⇒ Reject-Endzustand + RD trägt volle
  Verbotsliste), prod_inv_15 (Export ohne EP blockiert), prod_inv_16
  (cc-by ohne Attribution blockiert) = grün
- Replay: fixer Snapshot ⇒ gleiche IDs/Hashes/Ledgerpfade
  (replay_fixed_snapshot_same_ids_hashes_ledger_paths) = grün
- Wächter um CSA-Zeugen erweitert (GUARD_PHASES += "G8"; cce-conformance
  hängt jetzt an allen nexus-Crates) = grün
- CI vollständig grün (Spec-Integrität 24/24, fmt, clippy -D warnings,
  INV-11-Schichtung, alle Tests).
Residuen dieser Phase: keine neuen. Kein realer Netz-Transport verdrahtet —
alle Läufe gegen Snapshot-/Fixture-Transporte; ein realer HTTP-Transport
würde DENSELBEN versiegelten Pfad benutzen (Betriebs-/Onboarding-Schritt,
kein Bauschritt; deckungsgleich mit Overlay-05-Linie „Anbieter-Onboarding
= Betrieb").
Abweichungen von der Spec:
- `SourceAdapter::acquire` ist SYNCHRON statt der `async fn`-Skizze der
  Spec: P9-Determinismus + keine Runtime-Abhängigkeit (der Bau ist
  netzfrei); die Signatur bleibt portidentisch (Beobachtungen rein,
  Beobachtungen raus). Begründung dokumentiert, kein Verhaltensunterschied
  im Snapshot-Betrieb.
- ci/check_acyclic.py: cce-conformance als Wächter-Ausnahme oberhalb aller
  Schichten eingetragen (die Zeugensuite wächst per Spez mit jeder Phase —
  sie ist Konsument der Prüflinge, kein Kern-Crate).
