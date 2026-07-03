Track-Einheit: C/P4 — Intelligenz: echtes Lokalmodell (nach W2)
Eingang: W2 grün, CI grün.
Gebaut:
- `crates/cce-inference/src/providers/local_extractive.rs` —
  `LocalExtractiveModel`: ein ECHTES lokales Modell (kein Mock), das den
  allowed_context WIRKLICH verarbeitet: Satzsegmentierung → Term-Frequenz-
  Gewichtung (mit Stoppwort-Filter) → Satz-Ranking → extraktiver Entwurf.
  Provider-Klasse LocalModel (KEIN Egress, needs_egress=false),
  replay_policy=recorded, Manifest vollständig (C.4). Liegt ausschließlich
  unter providers/; Gateway/Gates/Zeugen UNVERÄNDERT.
- Zeugen: `r_inf_2b_real_local_model_recorded_replay_class_identical`
  (durch das unveränderte Gateway; Aufzeichnung == Response-Digest der
  Evidence; Live-Re-Inferenz identisch) + `offline_core_unbroken_with_
  real_local_model_present`. Modell-Unit-Tests belegen: Ausgabe hängt
  WIRKLICH vom Kontext ab (nicht-kanned) und ist deterministisch.
Abnahme (P4):
- Alle bestehenden Inference-Zeugen grün (jetzt 27 = 25 + 2 neue) = grün
- Neuer Zeuge r_inf_2b (Aufzeichnung+Einspielung klassenidentisch) = grün
- Offline-Modus unverändert grün (DisabledProvider degradiert; LocalModel
  fügt keinen Egress-Pfad hinzu) = grün
- Keine neuen Egress-Pfade, keine Gate-/Zeugen-Änderung; CI GRUEN
Residuen:
- **Echtes neuronales LLM (GGUF/llama.cpp)**: bewusst NICHT angebunden —
  bräuchte ein Modell-Artefakt (Multi-GB), einen C++-Build und Host-
  Ressourcen, würde CI sprengen und ist im Container unpraktisch. Das
  gelieferte Lokalmodell ist ehrlich als leichtgewichtig/regelbasiert
  (extraktiv) gekennzeichnet — kein LLM-Overclaim. Die GGUF-Anbindung
  bleibt Folgeschritt (Track C, IG-R-Reihe), hinter DEMSELBEN Gateway.
- Kanzel: das echte Modell steht hinter dem Gateway bereit; die
  cockpit-core-Kanzel formt weiter über den deterministischen Memo-Former
  — die Verdrahtung „Kanzel nutzt LocalExtractiveModel" ist ein kleiner
  Folgeschritt (Track B/C), berührt keine Gates.
Abweichungen: keine; spec/ unangetastet; egress weiter nur durch die drei Tore.
