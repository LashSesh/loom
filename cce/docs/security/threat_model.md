# Threat-Model CCE/Loom (Register #25 — Kurzform, 2 Seiten)

Geltung: Bau-Stand 0.1.0. Dieses Dokument beschreibt Angreifermodelle
gegen die GEBAUTEN Flächen und ordnet jeder Klasse die existierende,
testbewachte Gegenmaßnahme zu. Es behauptet keine Sicherheit jenseits
der Beweislage; Restrisiken stehen am Ende mit Register-Nummer.

## 1. Schutzgüter

1. **Urteilsintegrität** — Gate-Verdikte, Residuen, Ledger: niemand
   außer dem Motor schreibt sie (V1/V2/V7; PROD-INV-19).
2. **Replay-Garantie** — bestätigter Crystal + RD reproduzieren die
   Commit-Klasse; keine nichtdeterministische Komponente im Pfad.
3. **Egress-Kontrolle** — Daten verlassen das System nur durch die
   drei Tore (CSA / InferenceGateway / ToolGateway), je gate- und
   lock-pflichtig (Egress-Vierteilung, Overlay 05 Teil B).
4. **Container-Authentizität** — core_root bindet Inhaltsklasse;
   file_byte_digest bindet die Datei (Zwei-Digest-Modell).
5. **Operator-Souveränität** — jede materielle Aktion trägt eine
   aufgezeichnete Bestätigung (COCK-INV-3).

## 2. Vertrauensgrenzen

- **T1 Fremddatei → Reader:** `.loom`-Dateien beliebiger Herkunft
  treffen auf loom-format/canon/verify (motorfreier Pfad).
- **T2 Quelle → Motor:** externe Inhalte treffen auf die CSA-Kette.
- **T3 Modell → Motor:** Provider-Antworten treffen auf das
  InferenceGateway (CandidateOutputs).
- **T4 Werkzeug → System:** Tool-Ausführung über das ToolGateway.
- **T5 GUI/Kanzel → Kern:** Bedienschale und LLM-Erzähler gegenüber
  Zustandsmaschine und Motor-Fakten.

## 3. Angreifermodelle und Gegenmaßnahmen

### A. Bösartige .loom-Datei (T1)
Vektoren: Parser-Crash/UB, Dekompressionsbombe, Second-Preimage über
Baumformen, Tabellen-/Offset-Lügen, versteckte Ausführung, Zyklen.
Gegenmaßnahmen (alle testbewacht):
- Digest-Prüfung VOR jeder Deserialisierung (Frame::decode; N4-Zeuge).
- stored≠uncompressed ⇒ Reject vor Allokation (N13); Kompression im
  Bau deaktiviert (CompressionUnsupported, sichtbar).
- Merkle mit Domain-Separation 0x00/0x01 + promote-odd (keine
  Duplikations-Ambiguität); SEGTAB eintragslos, Footer bindet Tabelle.
- Referenzen nur per Digest; Zyklen ⇒ N11-Reject; Tiefenlimit im
  Canon-Decoder; TrailingBytes-Reject.
- Kein Hook/Autostart im Format; open/inspect/verify seiteneffektfrei
  (C3-Zeuge); Autostart-Felder ⇒ N15/N16-Reject.
- **Fuzz-Nachweis:** deterministischer Harness
  (loom-conformance/tests/fuzz_smoke.rs) — ≈20 000 seeded Mutationen
  über Canon/Frame/Footer/Container: kein Panic, nur getypte Verdikte.

### B. Bösartige/kompromittierte Quelle (T2)
Vektoren: Scope-Ausbruch, Policy-Umgehung, Evidenz-Fälschung, Flutung.
Gegenmaßnahmen: versiegelter ApprovedFetchPlan (kein Socket vor
PolicyGate — Zähl-Transport-Zeuge); DISALLOWED_ACTIONS hart verdrahtet
(Reject-Endzustand); RateBudgetGate; EvidencePack-Pflicht je CSU (N5);
HTMLScopeLeak-Zeuge (Decoder-Schlüsselvalidierung + Horizont-Verwurf);
Ledger-Replay-Gate gegen Driftverschleierung.

### C. Bösartiges Modell / Prompt-Injection (T3)
Vektoren: Kandidat als Commit ausgeben, Gate-Override, Kontext-
Exfiltration, Confidence-als-Urteil, stiller Retry.
Gegenmaßnahmen: CandidateOutput ohne Status-/Verdikt-/Ledger-Feld
(strukturell); NoDirectCommit-/NoGateOverrideGate (Reject-Zeugen);
PromptContextGate mit forbidden-Prüfvorrang VOR Egress (Zähler-Zeuge:
0 Egress bei Halt); Refusal = regulärer sichtbarer Zustand;
COCK-INV-8 (Einschätzung nie Verdikt); Kanzel-API schreiblos.

### D. Werkzeug-Eskalation (T4)
Vektoren: implizite Freigabe, Scope-/Budget-Ausbruch, Sammel-Locks.
Gegenmaßnahmen: je Klasse EIGENER ToolCapabilityLock (Öffnung nur mit
Operator+Ledger-Ref); Scope-Präfix-Prüfung; Aufruf-Budget;
Aufzeichnungspflicht; Remote nur hinter ToolEgressGate (im Bau nur
Verweigerungspfad); N-INF-9/10-Zeugen.

### E. UI-/Kanzel-Manipulation (T5)
Vektoren: rotes Gate grün rendern, Residuum verbergen, Aktion ohne
Bestätigung, „Trotzdem durchlassen".
Gegenmaßnahmen: Datenfluss Motor→Core→Anzeige (Kanzel nur Leser);
ViewItems ohne Urteils-Setter, Pflicht-source_ref; leeres Feld wird
als „geschlossen (∅)" gerendert; force_through existiert nur als
immer-fehlschlagender Verbotstest; COCK-INV-1..8-Zeugen.

### F. Lieferketten-/Build-Angriffe
Ist-Stand: Motor/nexus/loom/cockpit-core sind dependency-frei (eigener
SHA-256, eigenes dCBOR); einzige externe Kette ist der egui-Stack im
GUI-Blatt cockpit-app. CI erzwingt Tor-Trennung im Crate-Graph,
motorfreie Viewer-Hülle und einen Socket-Symbol-Scan; Toolchain ist
auf 1.94.1 gepinnt; Golden Files machen Format-Drift sichtbar.

## 4. Restrisiken (offen, Register-geführt)

- **Signaturen nicht betriebsgebunden** (#14): SIGNATURE-Kind existiert,
  Ed25519+Schlüsselbund offen ⇒ Authentizität einer Datei stützt sich
  bis dahin auf core_root-Abgleich über einen Zweitkanal.
- **zstd-Profil** (#18): bei Aktivierung gilt N13-Disziplin erneut
  (Dekompressionsschranke VOR Allokation nachzuweisen).
- **NFC-Teilmenge** (#19): dekomponierte Formen werden fail-closed
  verworfen — kein Sicherheitsloch, aber eine Interop-Grenze.
- **egui-Lieferkette** (cockpit-app): extern, ungepinnt auf Audit-Ebene;
  Determinismus-Grenze liegt unterhalb der App.
- **Coverage-geleitetes Fuzzing** (#25-Rest): cargo-fuzz/libFuzzer
  (nightly) als Betriebsschritt; der deterministische Harness ist der
  CI-Kern, nicht der Ersatz.

## 5. Die drei Leit-Szenarien mit Datei:Zeile (P6(b) aus Audit-Doc 10)

| Szenario | Verhindernder Mechanismus | Datei:Zeile |
|---|---|---|
| **Angreifer öffnet fremde .loom** | Frame-Digest-Prüfung VOR Deserialisierung (Mismatch ⇒ Reject) | `loom/loom-format/src/lib.rs:198` |
| | Bomben-Schranke stored≠uncompressed vor Allokation (N13) | `loom/loom-format/src/lib.rs:183` |
| | Merkle-Root-Abgleich Tabelle↔Footer (N4) | `loom/loom-verify/src/lib.rs:124` |
| **Bösartiges Provider-Manifest** | Autostart-Feld ⇒ N16-Reject („Deklaration ≠ Aktivierung") | `loom/loom-verify/src/lib.rs:434` |
| | on_open/hidden_model_call ⇒ N15-Reject | `loom/loom-verify/src/lib.rs:444` |
| | Laufzeitseite: unvollständiges Manifest ⇒ Provider bleibt passiv | `crates/cce-inference/src/manifest.rs:69` |
| **Manipulierter Snapshot** | Replay-Gate: abweichende Ledger-Köpfe ⇒ replay_drift-REJECT | `nexus/nexus-ledger/src/lib.rs:30` |
| | Hash-Ketten-Prüfung des Ledgers selbst | `crates/cce-core/src/ledger.rs:139` |

(Zeilennummern Stand Commit dieser Datei; die zugehörigen Zeugen
n4/n13/n15/n16, neg_5_replay_drift und n_inf_1 halten die Mechanismen
unabhängig von Zeilendrift grün.)
