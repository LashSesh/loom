Phase: G9 .loom-Containerformat (loom/-Workspace, LBC-1)
Eingang erfüllt: ja (G8a-Gate grün, reports/G08a_bericht.md)
Gebaut (Reihenfolge B1–B12 aus LOOM-Standard Teil 12):
- B1 `loom-canon`: LOOM-CANON-1 — dCBOR-Encoder + STRIKTER Decoder
  (definite lengths, kürzeste Int-Kodierung, Map-Keys tstr/uint bytewise
  sortiert ohne Duplikate, Float-Verbot STRUKTURELL [das Wertemodell hat
  keinen Float-Konstruktor; Major-7-Floats werden vor dem Head-Parser
  abgefangen], Decimal-Fraction Tag 4 mit Integer-Mantisse, Tag-Whitelist
  {0,4}, Tiefenlimit, TrailingBytes-Reject); NFC als Teilmengen-Prüfung
  (kombinierende Diakritika U+0300–036F ⇒ TextNotNfc); CANON_RULES_TEXT
  für das CANON_DESC-Segment. K1–K6-Kerntests (gleiche Semantik ⇒ gleiche
  Bytes; Can∘Can=Can auf Formatebene).
- B2 `loom-format`: Präambel (16 B, PNG-artiges Magic 89"LOOM"0D0A1A0A,
  major/minor/flags/header_len LE, reservierte Bits MÜSSEN 0), Frame
  (kind u16, seg_flags, uncompressed/stored_len u64, Multihash 0x12 0x20
  über die UNKOMPRIMIERTE Payload; Digest-Prüfung VOR Deserialisierung;
  stored≠uncompressed = Bomben-Signatur N13), Footer (64 B, segtab_offset/
  len, core_root, LOOM_END), Kind-Registry 0x0000–0x0050 + Overlay-Kinds
  0x0060–0x0064, eigener SHA-256 (FIPS-Vektoren getestet) — der gesamte
  Reader-Pfad ist MOTORFREI.
- B3/B5 `loom-codec`: seal_canonical (canonical-stored: Segmente
  unkomprimiert, physisch in Tabellenordnung (kind,digest); Dedupe-Pflicht;
  Header-Pseudo-Eintrag 0x0000; SEGTAB EINTRAGSLOS), Merkle-core_root
  (Blatt H(0x00‖dCBOR(kind,digest,len)), Knoten H(0x01‖l‖r), promote-odd,
  Domain-Separation), decode_sealed (Tabellen-/Frame-/Digest-/Sortier-
  Konsistenz, SegtabSelfEntry-Reject), recompute_core_root.
- B4 `loom-verify`: L0 (Struktur+Root), L1 (Kanon+Schema, Manifest-13-
  Felder-Vertrag, unbekannte Kinds: preserve+sichtbar bzw.
  required_understand⇒quarantine), L2 (Profil-Pflichtsegmente,
  Referenzen auflösbar+azyklisch, EvidencePack-Pflicht je CSU,
  GateReport je Commit, residue_summary≡RESIDUE, Claims≤Beweislage,
  Score-als-Verdikt-Reject, Overlay-Regeln: 0x0060 ohne Autostart-Feld,
  0x0061 Manifest-Referenzen vorhanden, 0x0062 evidence-gebunden,
  0x0063 kein Kandidat als Commit, 0x0064 keine implizite Freigabe,
  network_request-Altform sichtbar-konservativ). Verdikte
  valid|valid_with_residues|quarantine|reject, nie still.
- B6 `loom-mount` (open=M1–M2 reine Verifikation, inspect, Mount-Modi;
  Quarantäne inspizierbar aber nie run-/exportfähig) + `loom-viewer`
  (Referenz-Viewer, 5 Pflichtansichten headless; **baut nachweislich
  motorfrei** — ci/check_acyclic prüft die transitive Viewer-Hülle auf
  cce-*/nexus-*/cockpit-Freiheit) + `loom-cli` (inspect/verify/ls).
- B7 R-Dateien über Motor-Ports: `loom-gate` (GateReports aus cce-core,
  read-execute), `loom-project` (PHC-Segment aus cce-phc
  LocalProjection), R2/R6-CSA-Inhalt klassenidentisch zu nexus-core-CSUs,
  R3-HBM-Inhalt aus cce-hbm extract_facets, R7-Inhalt = Drei-Risiken-Memo
  aus cce-materialize (kanonische Klasse transportiert).
- B8 `loom-runner`: RunnerContext mit CapabilityLocks (Deklaration ≠
  Aktivierung; capability_classes inkl. Aufspaltung source_acquisition/
  model_egress/tool_egress; network_request-Altform nie aktivierbar);
  run erzeugt CandidateOutput, NIE Commit; N8-Eskalation getestet.
- B9 `loom-replay`: REPLAY_MANIFEST-Vertrag + check_replay
  (Klassenvergleich, ManifestIncomplete benannt = N7).
- B10 `loom-export`: ExportGate (Quarantäne/Reject ⇒ kein Export;
  cc-by ohne Attribution ⇒ AttributionMissing, PROD-INV-16).
- B11 Migration: format_major-Bruch ohne Migrationspfad = Reject (N10);
  minor-additiv bleibt read-/seal-kompatibel (C5-Test).
- B12 `loom-conformance`: R1–R8-Builder, N1–N16, C0–C5, Golden Files.
- Saat-Bibliothek: `library/seed/drei_risiken_memo_workbody.loom` (=R7)
  und `minimal_inspect.loom` (=R1) als .loom ausgeprägt; golden-gen ist
  deterministisch (Doppellauf byte-identisch).
Ausgangs-Gate:
- Golden Files: encode∘decode byte-identisch im canonical-stored-Profil
  (tests/golden.rs golden_files_byte_identical + C0-Reseal-Test über
  alle 8 R-Dateien) = grün
- 8 Referenzen valid (R1–R7 + R8 aus Overlay Teil E), 16 Negative
  reject/quarantine am richtigen Prüfpunkt (N1–N14 + N15
  hidden_model_call_on_open + N16 provider_autostart_flag; 24 Zeugen
  gesamt gemäß Overlay) = grün
- C0–C5 grün (C0 byte-identisch, C1 permutierte Eingaben ⇒ gleiche
  core_root, C2 Verdikte, C3 open/inspect/verify seiteneffektfrei
  [kein FS-Write; kein Modell-/Tool-Egress möglich, da der Viewer-Pfad
  per CI-Regel an keinem Gateway-/Motor-Crate hängt], C4 Replay
  reproduziert Commit-Klasse, C5 minor-additiv kompatibel + N10 rot)
- Viewer-Target baut ohne cce-*-Abhängigkeit: ci/check_acyclic.py
  Reader-Prinzip-Regel (transitive Hülle von loom-viewer) = grün;
  Laufzeitnachweis am Golden R7 (viewer_renders_golden_r7_without_motor)
- Wächter: GUARD_PHASES += "G9"; conformance/tests/loom_guard.rs bindet
  die 8 Formatzeugen in denselben CI-Lauf = grün
- CI vollständig grün.
Residuen dieser Phase (sichtbar, blockieren nicht — LC-R-Familie):
- LC-R1 Signaturverfahren (SIGNATURE 0x0050 registriert, non_core
  erzwungen; Ed25519-Bindung + Schlüsselbund = S11/Betrieb).
- LC-R2 blake3-Zweitprofil nicht aktiviert (nur sha2-256).
- LC-R3 CDDL-Vollschemata: Feldverträge sind in loom-verify normativ
  implementiert; maschinenlesbare CDDL-Dateien unter schemas/ stehen aus.
- NFC: Teilmengen-Prüfung statt voller Unicode-Normalisierungstabellen
  (alle selbsterzeugten Inhalte sind ASCII/präkomponiert; dekomponierte
  Formen werden fail-closed verworfen, nie still normalisiert).
- Kompression (seg_flags bit0/zstd) nicht aktiviert: Reader weist
  komprimierte Segmente begründet ab (CompressionUnsupported) — zstd
  wäre die erste externe Abhängigkeit; canonical-stored ist ohnehin
  unkomprimiert. Transportprofil = Betriebsentscheidung.
Abweichungen von der Spec:
- loom-migrate ist kein eigenes Crate: N10/C5 leben in verify/codec
  (der Standard nennt loom-migrate nur im CLI-/Reihenfolge-Kontext;
  Migrationslauf als gegateter Lauf folgt mit der Workbench-Reise G11).
- L3 (Klassen-Replay) prüft über den Motor-Port des Aufrufers
  (loom-replay::check_replay + deterministische Reproduktion im Test) —
  der Standard verlangt Motor-Ports genau dafür (Teil 5.2).
