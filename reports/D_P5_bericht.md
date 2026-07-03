Track-Einheit: D/P5 — Weltzugang: HttpTransport + Wikimedia live (nach W1)
Eingang: W1 grün, CI grün; Netzkonnektivität zur offiziellen API geprüft
(HTTP 200 über den Agent-Proxy).
Gebaut:
- `HttpTransport` in nexus-fetch (GET + ETag, Proxy-/User-Agent-bewusst),
  NUR unter opt-in-Feature `http` (standardmäßig AUS) — CI baut ureq nicht,
  bleibt netzfrei und deterministisch. Er ist eine gewöhnliche
  `Transport`-Impl und läuft AUSSCHLIESSLICH über `fetch(&ApprovedFetchPlan,…)`:
  der versiegelte Pfad, approve_fetch/Gates/Adapter UNVERÄNDERT.
- `examples/capture_wikimedia.rs` (feature-gegated): holt EINMAL die
  offizielle Wikimedia-API über den vollen Policy-Pfad
  (approve_fetch → fetch) und friert die Antwort ein.
- Fixture `conformance/fixtures/wikimedia_kristall.json` (716 B, echte
  Antwort zu „Kristall", CC BY-SA).
- Zeuge `ref_2b_live_snapshot_frozen` (conformance/tests/csa_catalog.rs):
  die eingefrorene Antwort fließt deterministisch durch den unveränderten
  versiegelten Pfad — gleiche Bytes, gleicher Content-Digest über zwei
  Läufe, Bytes = Fixture, echte Wikimedia-Nutzlast, disallowed_actions
  vollständig. Kein Netz in CI.
Abnahme (P5):
- ref_2 live = Fixture-Klasse (Fetch-Ebene, deterministisch) = grün
- disallowed_actions unberührt (Zeuge prüft disallowed_complete) = grün
- Rate-Budget respektiert (fetch mit Budget 8, 1 Request; Live-Capture-Log
  „eingefroren: 716 Bytes") = grün
- neuer Zeuge ref_2b_live_snapshot_frozen im Wächter = grün
- versiegelter Pfad/Gates/Adapter unverändert (nur additive Transport-Impl
  hinter Feature) = grün · CI GRUEN
Residuen:
- **JSON→CSU-Extraktor für official_api**: der WikimediaAdapter dekodiert
  per kv-Zeilen (D01-Referenzprofil); echte API-Antworten sind JSON. Der
  Zeuge beweist daher die SEALED-FETCH-Ebene (Beobachtungsklasse stabil),
  nicht die volle CSU-Normalisierung — ein JSON-Extraktor wäre eine
  Adapter-Änderung (außerhalb P5-Scope „Adapter unverändert") und ist als
  Folgeschritt offen (Track D, official_api-Ausbau).
- Live-Abruf ist bewusst NICHT in CI (Feature-off): Reproduzierbarkeit vor
  Netz-Abhängigkeit; erneuter Live-Lauf via `--features http` möglich.
Abweichungen: keine; spec/ unangetastet; egress weiter nur durch das
CSA-Tor hinter dem versiegelten Plan.
