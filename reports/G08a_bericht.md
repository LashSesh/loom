Phase: G8a InferenceGateway & ToolGateway (Overlay 05, Teile B–G)
Eingang erfüllt: ja (G8-Gate grün, reports/G08_bericht.md)
Gebaut (Reihenfolge a–j gemäß F.1):
- a) `cce-inference`: ModelManifest (alle C.4-Pflichtfelder; fehlend ⇒
  provider_manifest_missing, Provider bleibt PASSIV — IG-A5 Deklaration ≠
  Aktivierung; egress-pflichtige Klassen brauchen deklarierte
  capability_locks), 5 Provider-Klassen (ProviderClass mit needs_egress()),
  ReplayPolicy {recorded (Default), strict, weak}; InferenceRequest (C.5,
  allowed/forbidden_context mit Prüfvorrang, context_digest, RD-Ref, Seed);
  InferenceResponse (C.6, Refusal = regulärer sichtbarer Zustand);
  CandidateOutput (C.7: strukturell KEIN Commit — kein Status-, Verdikt-
  oder Ledger-Feld; self_assessment nur Anzeige); InferenceEvidence (C.7:
  Request-/Kontext-/Response-Digest, Manifest-/Projektions-/RD-Ref,
  Egress-Gate-Reports, Kosten); 16 ModelResidues (C.9, ALL_MODEL_RESIDUES;
  model_refusal/model_replay_weak = Warning, Rest Blocking).
- b) Gate-Kette: alle 14 Pflichtgates (C.8) einzeln als Funktionen,
  boolesch/fail-closed/begründet, Vier-Wege-Verdikt; im Gateway laufen
  die 10 Egress-Vorgates STRIKT VOR dem Provider-Aufruf (PromptContext
  mit forbidden-Prüfvorrang + Boundary-Check), OutputSchemaGate typisiert
  NACH Egress; ToolCapability-/ToolEgressGate gehören dem ToolGateway,
  HumanConfirmationGate dem materiellen Aktionspfad. Kontext-Overflow ⇒
  Hold, KEIN stilles Kürzen.
- c) DisabledProvider/DegradedMode: antwortet IMMER sichtbar degradiert;
  Offline-Nachweis: Motorpfad (PhaseBlock-Accept) läuft ohne Provider
  vollständig (R-INF-1).
- d) LocalModelProvider-Echtpfad: deterministischer Stub (splitmix64-Schritt
  auf RD-Seed + Kontext-Digest), replay_policy=strict deklariert und
  ehrlich erfüllt (R-INF-2).
- e) CloudModelProviderMock hinter ModelCapabilityGate (model_egress-Lock
  via cce-core::capability); Egress-Zähler als Beweismittel. KEIN realer
  Cloud-Anbieter verdrahtet (F.3: Betrieb).
- f) ExternalAgentProviderMock: erhält per Port-Signatur NUR den Request
  (ProjectionPacket-Kontext); Repo-Vollzugriffs-Forderung wird sichtbarer
  Fehlerpfad, nie still erfüllt.
- g) recorded-Replay: InferenceRecorder zeichnet auf; replay_response
  spielt EIN (kein Live-Re-Call); Live-Abweichung = model_replay_weak
  (sichtbar); fehlende Aufzeichnung = model_trace_missing.
- h) `cce-toolgateway`: ToolManifest (C.10, 9 Tool-Klassen), ToolGateway
  mit JE KLASSE eigenem ToolCapabilityLock (open_lock öffnet genau eine
  Klasse; Operator+Ledger-Ref pflichtig), fs_read-Referenz-Tool gegen
  eingefrorene Sicht (Scope-, Budget-, Klassen-Prüfung, Aufzeichnung);
  Remote nur hinter ToolEgressGate (im Bau ausschließlich Verweigerungs-
  pfad getestet). Tor-Trennung im Crate-Graph: cce-toolgateway hängt
  weder an nexus-* noch an cce-inference.
- i) 19 Zeugen unter `conformance/inference/inference_catalog.rs`
  (Spez-Ablageort, als [[test]]-Pfad eingebunden) — im einen Wächter.
- j) Schreiblose Kanzel-API (C.12): Kanzel::form_wish_request /
  explain_residue / propose_repair — Rückgabetypen sind Texte bzw.
  InferenceRequests; es existiert kein Parameter/Rückgabetyp für
  Urteils-/Residuen-/Ledger-Schreibzugriff (für G10 bereit).
Ausgangs-Gate G8a:
- Kein Modell-/Tool-Socket außerhalb der Gateways: (1) Dependency-Scan —
  ci/check_acyclic.py prüft jetzt IG-A1-Tor-Trennung (cce-inference/
  cce-toolgateway ↮ nexus-*, Gateways nicht verschmolzen); (2) Symbol-Scan —
  Socket-/HTTP-Primitive (TcpStream/UdpSocket/reqwest/hyper/curl/ureq)
  außerhalb providers/ bzw. nexus-fetch ⇒ CI-Fail; (3) Laufzeit-Negativ:
  N-INF-2/N-INF-5 (Egress-Zähler = 0 bei Gate-Halt) = grün
- 14 Gates einzeln getestet, fail-closed (inference_catalog +
  Crate-Unit-Tests; ALL_INFERENCE_GATES = 14) = grün
- Offline Core Mode grün (R-INF-1: Motor + Produktpfad ohne Provider,
  Kanzel sichtbar degradiert) = grün
- recorded-Replay klassenidentisch (recorded_replay_class_identical:
  Aufzeichnung eingespielt, Digest-gleich; Drift ⇒ model_replay_weak) = grün
- 7 R-INF grün + 12 N-INF rot im Wächter (25 Tests inkl. Zusatzabdeckung
  Refusal/Rate; GUARD_PHASES += "G8a") = grün
- PROD-INV-17..20 negativ-getestet (prod_inv_17..20) = grün
- Deklaration ≠ Aktivierung nachgewiesen (Manifest-Validierung ohne
  Aktivierung; N-INF-11 Autostart-Feld-Reject; Locks default zu) = grün
- CI vollständig grün (fmt, clippy -D warnings, INV-11 + Tor-Trennung +
  Socket-Scan, alle Tests).
Residuen dieser Phase: keine neuen offenen. Gemäß F.3 sind NICHT
Bestandteil des Baus (Betrieb, im Abschlussbericht §3 zu führen):
Live-Cloud-Provider-Vollintegration (Mocks + LocalModel-Echtpfad
erfüllen die Bau-Pflicht) und ExternalAgent-Produktivbetrieb
(implementiert + mock-getestet).
Abweichungen von der Spec:
- N-INF-11 (`hidden_model_call_on_open`) ist hier als Datenmodell-Zeuge
  umgesetzt (Autostart-/Aktivierungsfeld ⇒ Reject); der Format-Zeuge auf
  .loom-Verify-Ebene (= LOOM-N15, Kind 0x0060 L2-Regel) folgt planmäßig
  in G9 mit der Kind-Registry 0x0060–0x0064 (Teil E).
- Die 14 Gates verteilen sich architektonisch: 10 Egress-Vorgates +
  OutputSchemaGate im InferenceGateway, ToolCapability-/ToolEgressGate im
  ToolGateway, HumanConfirmationGate am materiellen Aktionspfad — gemäß
  C.8-Zuständigkeiten; alle 14 einzeln getestet.
