# Vollausbau-Status (eine Seite, Klartext)

Stand: Track A **W16 VOLLSTAENDIG (213/213 PL3)** + **Block 1
(Belegpflicht UX), Block 2 (Kanzel↔Modell + GUI-Feindesign), Block 3
(JSON→CSU-Extraktor + erstes Welt-Crystal) UND Etappe X1 (Ring E1 der
Ökosystem-Expansionskarte, VOLLSTAENDIG) abgeschlossen**. CI: GRUEN.
`feature_maturity_overclaim`: leer.

`cce-spec-repo/13_OEKOSYSTEM_EXPANSIONSKARTE.md` ist der Arbeitsplan
ÜBER diesem Status/dem Masterplan (Ringe E1→E2→(E3∥E4)→E5). E1
(`reports/X1_bericht.md`), E2 (`reports/E2_bericht.md`), E3
(`reports/E3_bericht.md`) UND jetzt **E4** (CE-1 Tabellen-Zellentyp +
loom-sdk/wasm-Viewer + Klassen-Registry; Details + alle Exit-Zeugen in
`reports/E4_bericht.md`) sind geschlossen. **Damit ist Etappe X2
(Ringe E2→E3→E4) vollständig abgeschlossen.** Nur E5 bleibt offen —
gesperrt bis S-E5 (Spec-Lieferung steht laut Karte §4 aus, „der
Auftraggeber" liefert sie vorab).

## Domänen (Track A) — 213 gesamt

| Familie | Präfix | fertig/gesamt | PL-Verteilung |
|---|---|---|---|
| A Dokument/Text | D | **15/15** | D01 = PL4 · D02–D15 = PL3 |
| J Wissen/Forschung | KNOW | **15/15** | KNOW01–15 = PL3 |
| G Governance | GOV | **12/12** | GOV01–12 = PL3 |
| N Kommunikation | COM | **12/12** | COM01–12 = PL3 |
| F Projekt/Prozess | PM | **15/15** | PM01–15 = PL3 |
| K Bildung | EDU | **12/12** | EDU01–12 = PL3 |
| I Produkt/Business | BUS | **15/15** | BUS01–15 = PL3 |
| B Software Eng. | SWE | **15/15** | SWE01–15 = PL3 |
| C Daten/Analytics | DATA | **12/12** | DATA01–12 = PL3 |
| D Graph/Netzwerk | GRA | **12/12** | GRA01–12 = PL3 |
| E Mathematik | MATH | **15/15** | MATH01–15 = PL3 |
| L Kreativ/Medien | CRE | **15/15** | CRE01–15 = PL3 |
| O Finanzen | FIN | **10/10** | FIN01–10 = PL3 |
| H Security/Ops | OPS | **15/15** | OPS01–15 = PL3 |
| M Hardware/CAD | HW | **15/15** | HW01–15 = PL3 |
| P Regulated Advisory | REG | **8/8** | REG01–08 = PL3 (PL4 review-gebunden) |

**Summe fertig (≥PL3): 213/213 — Track A VOLLSTAENDIG.** D01=PL4, 212×PL3.

## Parallele Tracks

| Track | Stand |
|---|---|
| A Domänen | **W1–W16 fertig: 213/213 auf PL3** (alle 16 Familien) |
| B Erlebbarkeit | **Block 1 nachgeliefert:** echter wgpu-Klick-Durchlauf (6 Nähte, debug-instrumentiert, `reports/ux/reise_protokoll_v2.md`) + #26 Datei-Export jetzt ECHT über den GUI-Button verifiziert (byte-identisch zum Headless-Pfad); Font-Atlas-Befund zweifach bestätigt host-gebunden (glow+wgpu), Backend zurückgestellt. **GUI-Feindesign (LC-R5) fertig:** alle 5 Pflichtansichten (Manifest/Segmentliste+Digest/Residuen+Verdikt/Gate-Reports/Ledger) im bestehenden Pruef-Tab verdrahtet, Zeuge grün |
| C Intelligenz | P4 **fertig**: echtes lokales Extraktiv-Modell (recorded, kein Egress), Zeuge grün. **Kanzel-Verdrahtung fertig:** LocalKanzel ruft real durch das unveränderte InferenceGateway (LocalExtractiveModel), Annahmen-Text modellgeformt + Provider-/Evidence-Beleg, DegradedKanzel unberührt, COCK-INV-1..8 unverändert grün; GGUF/LLM offen (host-gebunden) |
| D Weltzugang | P5 **fertig**: HttpTransport (feature `http`) + Wikimedia live, Fixture-fixiert, Zeuge grün. **JSON→CSU-Extraktor fertig:** echter JSON-Decoder (nexus-decode, keine externe Kiste), WikimediaAdapter liest die echte MediaWiki-Antwort, erstes Welt-Crystal `library/seed/kristall_wikimedia_workbody.loom` zertifiziert (`loom verify` ⇒ Valid, 0 Residuen), Attribution real transportiert |
| E Skalen | P8 **fertig**: SCALE-2 Dokumentenmappe (Red(2)-Kerntest, MSC 1→2, Adapter 8/8, PL2). **X1(b) fertig:** Mappe buendelt beide Kind-Memos PHYSISCH (je ein CAS_BLOB), Extraktion liefert 2 unabhaengig valide Einzel-Workbodies. **X2/E2 fertig:** SCALE-3 „Projektraum" (2 Mappen + Welt-Kristall + Blueprint-Zelle), MSC(1→2→3) gruen, Schliessung real per SeedResolver+CitationGate nachgewiesen |
| F Härtung | P6(a/b/c) **fertig**: Ed25519-Signatur ueber core_root (loom-cli sign/verify-sig), OS-Keyring feature-gated. **X1(e) fertig:** Signatur-Registry-Vollform — additive Mehrfachsignaturen (Autor/Pruefer/ReviewGate), `verify-sig --all` prueft die ganze Kette unabhaengig gruen/rot |
| G Pakete | gesperrt (Build-Hosts fehlen) |

## Block-Auftrag (aktuell)

1. **Block 1 — Belegpflicht UX: ABGESCHLOSSEN.** Echter Klick-Durchlauf
   unter wgpu (matchbox-WM fuer Fokus, COCKPIT_DEBUG_RECTS=1 fuer
   verifizierbare Koordinaten/Zustaende, da Glyphen host-bedingt nicht
   rendern), reise_protokoll_v2.md, echte per Button exportierte
   `.md`+`.cert`. Zwei ehrliche Nebenbefunde offen gemeldet (nicht
   verborgen): Glyphen-Rendering host-gebunden (R-Agent-6), xdotool-
   Zeichenumkehr-Artefakt der Testkette (R-Agent-7) — siehe
   reports/residuen.md.
2. **Block 2 — Kanzel↔LocalExtractiveModel + GUI-Feindesign: ABGESCHLOSSEN.**
   Kanzel-Formung laeuft real durch `run_inference` (LocalExtractiveModel,
   egress_lock=None da kein Egress noetig); Annahmen-Interpretation traegt
   Provider-Herkunft (`local-extractive:kernmodell`) + Gateway-Evidence-ID
   als Beleg, Crystal-Referenzstruktur unveraendert. GUI-Feindesign: die
   fuenf LC-R5-Pflichtansichten (Manifest/Segmentliste+Digest-Status/
   Residuen+Verdikt/Gate-Reports/Ledger-PhaseBlocks) rendern jetzt im
   Pruef-Tab aus echten Motor-/Ledger-Fakten. Zeugen: `kanzel_model_witness.rs`
   (3 Tests) + `lc_r5_inspection_views.rs` (3 Tests), alle 13 COCK-INV-Tests
   unveraendert gruen (keine Testdatei-Aenderung noetig), voller
   Workspace-Testlauf gruen, clippy sauber.
3. **Block 3 — JSON→CSU-Extraktor Wikimedia + erstes Welt-Crystal: ABGESCHLOSSEN.**
   `nexus-decode::decode_json` (selbst gefuehrter Parser, keine externe
   Kiste — dieselbe Disziplin wie beim Ed25519-Pfad), `WikimediaAdapter`
   liest jetzt die echte MediaWiki-JSON-Antwort statt des alten
   kv-Zeilen-Platzhalters. Milestone: `library/seed/kristall_wikimedia_workbody.loom`
   — Quellenzelle + Attribution sind der ECHTE, eingefrorene Wikipedia-
   Auszug „Kristall" (CC BY-SA 4.0), materialisiert ueber den echten
   Motor (cce-runner), `loom verify` ⇒ Valid/0 Residuen, deterministisch.
   Inspect-Beleg: `reports/welt_crystal_wikimedia.md`. Zeugen: 6 (nexus-decode)
   + 3 (nexus-adapter-wikimedia) + 4 (loom-conformance) neu, 19 CSA-Katalog-
   Zeugen weiterhin gruen (2 auf realistische JSON-Bytes umgestellt).
   Alle CSA-Gates/disallowed_actions unveraendert.

4. **Etappe X1 — Ring E1 der Ökosystem-Expansionskarte: ABGESCHLOSSEN.**
   Fünf Einheiten a→e, je mit eigenem Zeugen und Commit: (a) Artefakt-
   Bytes im Container (CAS_BLOB) + `loom extract`/`extract-children`,
   byte-identisch zu `materialize()`. (b) SCALE-2-Mappe bündelt zwei
   echte Kind-Memos physisch, Extraktion liefert 2 unabhängig valide
   Einzel-Workbodies. (c) zstd-Ganzdatei-Transport (klassenidentisch zu
   stored, canonical-stored bleibt Golden-Referenz) + blake3-
   Zweitprofil, beide nur im CLI-Blatt. (d) `.docx`-Export über ein
   neues Blatt-Crate `cce-docx-export` hinter demselben
   materialize-Vertrag wie `.md`, schließt S1.10-R1. (e) Signatur-
   Registry-Vollform: additive Mehrfachsignaturen (Autor/Prüfer/
   ReviewGate), `verify-sig --all` prüft die Kette unabhängig
   grün/rot. Alle fünf Karten-Exit-Zeugen grün, alle Alt-Zeugen
   unverändert (154 Testgruppen/418 Tests, 0 Fehlschläge), CI GRUEN,
   kein neues Kern-Crate mit externen Abhängigkeiten. Details:
   `reports/X1_bericht.md`.

5. **Etappe X2/Ring E2 — Verbund & Skalen: ABGESCHLOSSEN.** Die
   `cites`-Naht (S-E2a Teil I) vollstaendig: neues Crate `loom-cites`
   (CitationResolver-Port, SeedResolver, CitationGate mit fuenf
   benannten Pruefschritten), neue hermetische L2-Pruefung
   `manifest_citation_mismatch` in loom-verify, additive
   Replay-Input-Deklaration (`RunDescriptor::input_digests`). Das erste
   Memo, das sich real ueber Workbody-Grenzen hinweg auf den echten
   Welt-Kristall stuetzt (R-CIT-1/2). SCALE-3 „Projektraum" (I.6):
   neues Modul `cce-materialize::scale3_project`, MSC(1→2→3) gruen ueber
   die bereits skalen-agnostische cce-spiral::s15-Maschinerie (kein
   Core-Code geaendert), Schliessung real durch SeedResolver +
   CitationGate auf den echten Containern nachgewiesen (R-CIT-3). Alle
   acht Zeugen aus Dokument 14 §I.7 (R-CIT-1..3, N-CIT-1..5) gruen.
   Details: `reports/E2_bericht.md`.

6. **Etappe X2/Ring E3 — Selbstbezug: HBM produktiv: ABGESCHLOSSEN.**
   Keine neue Spec-Lieferung noetig (Dokument 14: HBM ist bereits
   vollspezifiziert). Die unveraenderte `cce-hbm`-Pipeline laeuft ueber
   einen REALEN Eigenkorpus (213 Familien-Referenzprofile + Katalog-
   Kern-Gates/-Residuen, projiziert in domaenen-eindeutige Facet-Zeilen)
   und zertifiziert mindestens einen Blueprint-Kristall — die
   „Vollprojektion" C6, die ALLE Facetten traegt (das im Karten-Beispiel
   genannte „Struktur-Muster wiederkehrender Naht-Regeln ueber
   Familien"), real materialisiert als
   `library/seed/blueprint_eigenkorpus.loom`. Replay klassenidentisch
   (zwei Laeufe, gleiche zertifizierte Klassen, gleicher Ledger-Head);
   R-13-Klonungs-Lock unangetastet, Negativzeuge weiterhin rot. Schliesst
   R-Agent-9 (Blueprint-Zelle war struktureller Platzhalter in E2).
   Details: `reports/E3_bericht.md`.

7. **Etappe X2/Ring E4 — Erweiterbarkeit & SDK: ABGESCHLOSSEN.** CE-1
   Tabellen-Zellentyp (die erste echte CoreExtension durch den
   S14-Pfad, `reports/CE1_beweiszug.md`, DoD ERFÜLLT) → `loom-sdk`
   (schmale, motorfreie Fassade open/inspect/verify/extract/replay) +
   wasm32-Build des Viewers, Headless-Browser-Zeuge real gegen R1
   (Valid) und eine beschädigte Kopie (Reject) ausgeführt → Klassen-
   Registry (`.loom`-Katalog-Workbody, zweite `CitationResolver`-
   Implementierung — löst dieselbe R-CIT-1/2-Szene aus Ring E2 genauso
   grün auf wie der `SeedResolver`). Details: `reports/E4_bericht.md`.

Damit sind alle drei angeordneten Bloecke UND **Etappe X2 (Ringe
E2→E3→E4) vollständig** abgeschlossen. Track A bleibt vollständig
(213/213 PL3). `.docx`-Export, zstd-Transportprofil, blake3-Zweitprofil,
SCALE-2-Vollmaterialisierung, die `cites`-Naht/SCALE-3, HBM auf
Eigenkorpus, CE-1/loom-sdk/wasm-Viewer UND die Klassen-Registry sind
jetzt GESCHLOSSEN. Offen bleiben: PL3→PL4-Reifepfade sowie Ring E5 der
Ökosystem-Expansionskarte (L9b Normic Memory — gesperrt bis S-E5, Karte
§4). Host-gebunden gesammelt und bewusst nicht weiter umgangen: GGUF-/
LLM-Anbindung, OS-Keyring-Live-Test, macOS/Windows-Pakete (Track G),
GPU-Klickpfad mit gerenderten Glyphen — je einen Build-/Desktop-/
GPU-Host, bis dahin gesperrt.