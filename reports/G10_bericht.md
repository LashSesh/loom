Phase: G10 Cockpit (natives Desktop, KI-Kanzel-Ports)
Eingang erfüllt: ja (G9-Gate grün, reports/G09_bericht.md)
Gebaut:
- `cockpit-core` (keine Motor-Logik, einziger Vermittler GUI↔Kanzel↔
  Motor↔Persistenz):
  - `engine.rs`: EnginePort-Trait (validate_crystal, submit, run_to_end,
    status, gate_reports, residues, ledger, artifact, replay_class) +
    MotorEngine-Referenzimplementierung über cce-runner (Dokument-Domäne,
    R-Plan-2). Der Replay-Pfad hat TYPSYSTEMISCH keinen Kanzel-/LLM-
    Parameter (COCK-INV-5 architektonisch).
  - `state.rs`: deterministische Zustandsmaschine S3.3 (LEER →
    WUNSCH_ERFASST → CRYSTAL_GEFORMT → BESTÄTIGT ═Bestätigungsgrenze═ →
    LÄUFT → AN_GATE | ABGELEHNT | GESCHLOSSEN → ARTEFAKT_VERFÜGBAR);
    Motor-Schema rot ⇒ zurück zu WUNSCH_ERFASST MIT Grund; ABGELEHNT =
    gültiger ehrlicher Endzustand; jede materielle Aktion (Crystal
    bestätigen, Lauf starten, Export) verlangt eine AUFGEZEICHNETE
    Confirmation{operator, action, statement}; `force_through()`
    existiert nur als sprechender Verbotstest und liefert IMMER
    NoOverridePath (V7: kein „Trotzdem durchlassen").
  - `kanzel.rs`: KanzelPort (form_wish/explain_gate/status) — jede
    Ausgabe trägt den festen Marker "Interpretation, kein Motor-Urteil"
    im Typ (COCK-INV-4); DegradedKanzel (Kanzel-Aus: form ⇒ None,
    sichtbar degradiert, Cockpit voll bedienbar); LocalKanzel
    (Referenzpfad, formt den Drei-Risiken-Memo-Crystal). Strukturell
    schreiblos: alle Methoden nehmen &self + Motor-Fakten, liefern Texte.
  - `persistence.rs`: PersistenceAdapter (content-adressiert, S3.7,
    sync-fähig ausgelegt) über cce-store.
  - `views.rs`: ViewItem mit PFLICHT-source_ref (Wurzel-Rückführung);
    GateReport-Ansicht (rot nie ohne Begründung), Residuen-Ansicht
    (leer ⇒ EXPLIZIT „geschlossen (∅)"), die sechs Ansichten
    (PhaseBlock, Frontier+Sync [Desync blockierend sichtbar], Ratchet,
    Blue/Red+WrapStability, HBM-Candidate-Board mit konstantem Label
    „ordnet, entscheidet nicht" [nicht abschaltbar], CSA-Quellen-Sicht)
    + Overlay S3-A7..A9 (ProviderStatus aus dem Manifest, Datenabfluss
    gesendet/blockiert je Request mit Grund, CandidateOutput-Sicht mit
    Verbleib und Confirmation-Status; Einschätzung stets als
    „Einschätzung, kein Urteil").
- `cockpit-app`: native egui-Desktop-App (eframe 0.35, glow/x11/wayland;
  Toolkit-Entscheidung S3.10-R1 damit gemäß S3.1.1-Empfehlung gefällt) —
  vier Flächen Wunsch/Lauf/Prüfung/Artefakt als Tabs; Kanzel-Schalter
  (aktiv/degradiert sichtbar); Bestätigungs-Buttons als explizit
  benannte materielle Aktionen; Prüf-Fläche OHNE Override-Knopf;
  Binary `cce-cockpit` baut nativ.
Ausgangs-Gate:
- COCK-INV-1..6 als Tests (cockpit/cockpit-core/tests/cock_inv.rs):
  1 kein GUI-Pfad erzeugt/ändert Gate-Urteile (+ Score-als-Gate
  strukturell abgewiesen), 2 kein Residuum verborgen (leer explizit
  „geschlossen (∅)"), 3 materielle Aktionen nur mit aufgezeichneter
  Bestätigung (3 Confirmations im Pfad), 4 Kanzel stets als
  Interpretation markiert, 5 Replay aus bestätigtem Crystal+RD
  klassenidentisch (zwei unabhängige Läufe, gleiche Klasse; kein
  LLM-Parameter im Pfad), 6 Ranking-Spalte mit festem Label, Status nur
  vom Gate = grün. Zusätzlich COCK-INV-7 (Provider-Aktivierung nur via
  CapabilityLock.open mit Operator+Ledger-Ref) und COCK-INV-8
  (Confidence nie als Verdikt gerendert) = grün. „Kein Trotzdem
  durchlassen" (no_force_through_button_exists) = grün.
- Cockpit startet nativ: Binary cce-cockpit kompiliert für das Ziel-OS
  (Linux x11/wayland). Der Fensterstart selbst ist in der headless
  CI-Umgebung nicht ausführbar — Laufzeitnachweis auf einem Display ist
  Betriebs-/Paketierungsschritt (G12); die App enthält keinerlei
  Konsolen-Pfad.
- Alle Anzeigen wurzel-rückführbar (views_are_root_traceable_sample:
  jedes ViewItem trägt source_ref auf Motor-Artefakte) = grün
- Kanzel-Aus-Modus voll funktionsfähig (kanzel_off_mode_fully_functional:
  ganzer Pfad bis ARTEFAKT_VERFÜGBAR mit DegradedKanzel) = grün
- Voller Bedienpfad Wunsch→Artefakt über die Zustandsmaschine
  (full_journey_reaches_artifact) = grün
- Wächter: GUARD_PHASES += "G10" · CI vollständig grün.
Residuen dieser Phase:
- S3.10-R1 (Toolkit) ist entschieden (egui); das GUI-FEINDESIGN der
  fünf Inspektionsansichten bleibt LC-R5 (Anschluss der .loom-Viewer-
  Pflichtansichten an die Cockpit-Oberfläche).
- Erste externe Abhängigkeit des Workspaces (eframe/egui-Stack),
  ausschließlich im GUI-Blatt-Crate cockpit-app — Motor, nexus, loom
  und cockpit-core bleiben dependency-frei; die Determinismus-Grenze
  (S3.1.3) verläuft unterhalb der App.
Abweichungen von der Spec: keine inhaltlichen. eframe-0.35-API nutzt
App::ui statt des älteren App::update — reine Toolkit-Anpassung.
