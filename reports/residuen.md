# Residuenregister des Bau-Agenten (fortlaufend, sichtbar — 04_AGENT_AUFTRAG §3)

R-Agent-1: Ablageort der Spezifikationen
Fundstelle: 00_START_HIER §5 (Zielstruktur: `spec/` neben `cce/`)
Frage/Konflikt: Das übergebene Repository enthält die Spezifikationen im ZIP-entpackten
Verzeichnis `cce-spec-repo/spec/` statt in einem Wurzel-`spec/`. Verschieben/Kopieren würde
physische Duplikate der read-only Spezifikationen erzeugen.
Vorläufige Behandlung: konservative Auslegung — `cce-spec-repo/spec/` gilt als DER read-only
Spec-Baum; keine Datei wird bewegt oder editiert; CI prüft Integrität an Ort und Stelle.
Blockiert: nichts

R-Agent-2: Abnahmekatalog-Nummern K1–K18 / C1–C14 (Zuordnung rekonstruiert)
Fundstelle: Bauverfassung Teil 7.5 / Teil 9 Phasen C+D; 01_MASTER_BUILD G1
Frage/Konflikt: Die Spezifikationen fordern die Kataloge CL K1–K18 und CCC C1–C14 als
Testsuiten, listen die Einzelpunkte aber nicht (sie verweisen auf den nicht beiliegenden
Quell-Korpus). Verankert sind einzelne Nummern (CL K12 Residuen-Sichtbarkeit, CCC C13
Score/Gate) und alle inhaltlichen Eigenschaften (Hüllen-Gesetze, Kontraktion, Chordalität,
RIP, Verklebung, Zwei-Sweep, Bi-Temporalität, Kristall-Protokoll, Replay).
Vorläufige Behandlung: konservative Auslegung — alle verankerten Eigenschaften als Tests
materialisiert und fortlaufend nummeriert; die zwei explizit verankerten Nummern (K12, C13)
liegen auf ihren Spec-Positionen. Zuordnungstabelle in den Testdateien dokumentiert.
Blockiert: nichts

R-Agent-3: PhaseBlock-10-Tupel — Feldzuordnung
Fundstelle: REBASE_KONSOLIDIERUNG §1.1 ("PhaseBlock (10-Tupel)"), 01_MASTER_BUILD G2
Frage/Konflikt: Das 10-Tupel wird gefordert, die zehn Felder werden im Spec-Repo nicht
einzeln aufgezählt (Quelle: Strukturpause, nicht beiliegend). Die Accept-8-Kriterien
benennen die Pflichtinhalte (Typed/Boundary/Seam/Gate/Evidence/Residue/Replay/Reanalysis).
Vorläufige Behandlung: konservative Zuordnung — (id, scale, phase, inputs[HITL],
payload_digest, gate_reports, evidence_refs, residue_field, rd_ref, parent_refs);
deckt alle Accept-8-Prüfgegenstände ab, keines erfunden.
Blockiert: nichts

R-Agent-4: HyperDAG-Kantentypen — Zählweise
Fundstelle: REBASE_KONSOLIDIERUNG §1.1 / S15.5 (Formel: E_dep, E_seam, E_phase,
E_scale, E_commit) vs. 01_MASTER_BUILD G2 ("HyperDAG H mit 6 Kantentypen")
Frage/Konflikt: Die normative Formel nennt fünf Kantenmengen; die konsolidierte
Bauordnung spricht von sechs Kantentypen.
Vorläufige Behandlung: Implementierung folgt der normativen Formel (5 Kantentypen,
Autoritätsordnung: Spec über Wurzeldokument-Wortlaut); kein sechster Typ erfunden.
Blockiert: nichts

R-Agent-5: Nummernzuordnung V0–V9, LOOM-Abnahme (10), TAT P1–P7
Fundstelle: Bauverfassung Teil 7.5 (Kataloge), PHC §17/§18, LOOM/TAT-Verweise
Frage/Konflikt: Wie bei K/C-Katalogen (R-Agent-2) verweisen die geforderten Test-
kataloge auf den nicht beiliegenden Quell-Korpus; die Einzelnummern sind dort.
Vorläufige Behandlung: konservative Materialisierung aller im Spec-Repo verankerten
Eigenschaften (Loader-Kette, Themenfelder aus Teil 6.1-Testzeilen, TAT-Normalform),
fortlaufend nummeriert und in den Testdateien dokumentiert.
Blockiert: nichts

---

ENDSTAND (G12-Gesamtabnahme): R-Agent-1..5 bleiben dokumentiert wie oben;
keines blockiert. Das vollständige End-Residuenregister der Abnahme
(inkl. 02_MASTER_DOD §3 + F.3-Ergänzungen + Betriebs-/Umgebungsgrenzen)
steht in reports/ABSCHLUSSBERICHT.md §3/§4.

---

## Nachträge Vollausbau / Belegpflicht-UX (T1/WO-2 + Block 1)

R-Agent-6: Font-Atlas rendert in dieser Container-Sandbox nicht — host-gebunden
Fundstelle: reports/ux/reise_protokoll.md (glow/llvmpipe), reports/ux/reise_protokoll_v2.md (wgpu/lavapipe)
Frage/Konflikt: Der egui-Font-Atlas rasterisiert unter BEIDEN getesteten
Software-Renderern dieses Containers nicht (Labels/Buttons erhalten
Nullbreite); Fenstergeometrie, Klicks und Zustandsübergänge sind davon
unabhängig nachweislich echt und korrekt.
Vorläufige Behandlung: zweifach unabhängig bestätigt (glow UND wgpu) ⇒
als host-gebundene Umgebungsgrenze geführt, nicht weiter durch
Backend-Wechsel „umgangen". Sichtbare Glyphen in Screenshots brauchen
ein echtes Desktop-Display oder einen GPU-Passthrough-Host.
Blockiert: nichts (Klick-Durchlauf bleibt über Debug-Instrumentierung
verifizierbar, s. reise_protokoll_v2.md).

R-Agent-7: Zeichen-Umkehrung bei synthetischer xdotool-Texteingabe
Fundstelle: reports/ux/reise_protokoll_v2.md, klick_durchlauf_rohlog.txt
Frage/Konflikt: `xdotool type` liefert den Wunschtext in dieser exakten
Kette (Xvfb + matchbox-WM + XTest-Injektion + egui::TextEdit)
zeichengenau umgekehrt an die App aus — ein Automatisierungs-/
Test-Harness-Artefakt, kein Motor- oder cockpit-core-Fund.
Vorläufige Behandlung: offen gemeldet, nicht verborgen; wirkt sich in
diesem Durchlauf nicht auf das Ergebnis aus, da `LocalKanzel::form_wish`
(Vor-Block-2-Stand) den Wunschtext ohnehin ignoriert. Relevant für
künftige Beleg-Durchläufe NACH Block 2 (echte Kanzel-Inferenz liest den
Wunschtext) — dort entweder echtes Keyboard-Layout/Display nutzen oder
blockweises statt zeichenweises Einfügen.
Blockiert: nichts (betrifft nur die Testkette, nicht das Produkt).

---

## Nachträge Etappe X1 (Ring E1, Ökosystem-Expansionskarte §2/E1)

R-Agent-8: LibreOffice in dieser Sandbox generell konvertierungsunfähig
Fundstelle: reports/X1_bericht.md (Einheit d, .docx-Export)
Frage/Konflikt: `libreoffice --headless --convert-to` scheitert an JEDER
Datei in diesem Container — auch an einem reinen `.txt`→`.pdf` und an
einer unveränderten, extern erzeugten python-docx-Referenzdatei. Kein
docx-spezifischer Fehler, sondern eine kaputte Headless-Office-
Installation dieser Sandbox.
Vorläufige Behandlung: als Umgebungsgrenze geführt, nicht weiter
„umgangen" durch Backend-Wechsel; `python-docx` (unabhängiger,
weitverbreiteter OOXML-Parser) als reale Gegenprobe für den
docx-Roundtrip-Zeugen genutzt, ad hoc installiert, NICHT als
Repo-Abhängigkeit übernommen.
Blockiert: nichts (der Zeuge selbst — Inhalts-Roundtrip auf
Textebene — ist über python-docx real erbracht).

---

## Geschlossene Residuen (Etappe X1)

S1.10-R1 (docx-Bibliothekswahl offen, zuerst G03_bericht.md,
wiederholt in VOLLAUSBAU_STATUS.md): GESCHLOSSEN durch Etappe X1(d) —
neues Blatt-Crate `cce-docx-export`, Wahl getroffen (zip +
handgeschriebenes Minimal-OOXML), Zeuge grün, Belege in
reports/X1_bericht.md.

Signatur-Registry-Vollform (Mehrfachsignaturen, ProfessionalReviewGate-
Freigaben der Familie P — zuerst als Folgeschritt in
reports/F_P6c_bericht.md geführt): GESCHLOSSEN durch Etappe X1(e) —
additive Mehrfachsignaturen mit Rollenfeld (Autor/Prüfer/ReviewGate),
`verify_sig_all` prüft die ganze Kette unabhängig grün/rot. Der
`ROLE_REVIEW_GATE`-Slot ist eine strukturelle Signaturposition, KEIN
Ersatz für echte menschliche Prüfung — das bleibt außerhalb der
Möglichkeiten dieses Agenten und wird hier ausdrücklich nicht
behauptet.

---

## Nachträge Etappe X2 (Ring E2, S-E2a Teil I)

R-Agent-9: Blueprint-Zelle für SCALE-3 (R-CIT-3) ist ein struktureller
Platzhalter
Fundstelle: reports/E2_bericht.md (Einheit 4, SCALE-3 „Projektraum")
Frage/Konflikt: R-CIT-3 verlangt eine Projekt-Zelle vom Typ
„Blueprint-Kristall". Die REALE, aus dem Eigenkorpus zertifizierte
Blueprint-Kette liefert erst Ring E3 (HBM produktiv) — der laut
Bau-Reihenfolge in Dokument 14 NACH E2 kommt. Fuer den SCALE-3-Zeugen
wurde daher das strukturell gleichwertige R3-Muster (echte HBM-Facetten
ueber cce-hbm, hbm-Profil-Container) als Zellen-Platzhalter
wiederverwendet.
Vorläufige Behandlung: offen benannt (nicht als E3-Ergebnis
ausgegeben); die SCALE-3-Struktur/MSC(1→2→3)/Schliessungspruefung ist
davon unabhaengig vollstaendig und real — nur die MATERIELLE Reife der
Blueprint-Zelle folgt in Ring E3.
Blockiert: nichts (E3 ist der naechste geplante Schritt).

R-Agent-10: I.5 (Replay-Inputs) additiv umgesetzt, aber ohne
End-to-End-Replay-Lauf
Fundstelle: reports/E2_bericht.md (Residuen)
Frage/Konflikt: `RunDescriptor::input_digests` +
`replay_manifest_segment_with_inputs` + `check_replay_inputs` sind
gebaut und unit-getestet, aber Dokument 14 §I.7 fordert dafuer keinen
eigenen Zeugen — ein vollstaendiger zweiter Motor-Lauf gegen dieselbe
RD-Klasse (echter Replay-Beweis mit Input-Digests) wurde daher nicht
gebaut, um keinen ungeforderten Umfang zu erfinden.
Vorläufige Behandlung: sichtbar gefuehrt, nicht verborgen; die
Mechanik ist bereit fuer den ersten echten Anwendungsfall, sobald einer
gefordert wird.
Blockiert: nichts.

---

## Geschlossene Residuen (Etappe X2/Ring E2)

Keine — Etappe X2/Ring E2 schliesst keine vorher gefuehrten Residuen,
sondern eroeffnet zwei neue (R-Agent-9, R-Agent-10, s. oben).

---

## Geschlossene Residuen (Ring E3)

R-Agent-9 (Blueprint-Zelle fuer SCALE-3 war ein struktureller
Platzhalter, s. oben): GESCHLOSSEN durch Ring E3 — der reale,
eigenkorpus-zertifizierte Blueprint-Kristall existiert jetzt
(`library/seed/blueprint_eigenkorpus.loom`, Details:
`reports/E3_bericht.md`). Der SCALE-3-Zeuge aus Ring E2 selbst wurde
bewusst NICHT rueckwirkend umgeschrieben (Ring-Grenzen bleiben
historisch stabil); der neue Blueprint steht fuer kuenftige Verwendung
real zur Verfuegung.

R-Agent-10 (I.5 Replay-Inputs additiv umgesetzt, ohne End-to-End-
Anwendungsfall, s. oben) bleibt unveraendert offen gefuehrt — Ring E3
betraf HBM, nicht die cites-Replay-Mechanik.

---

## Nachträge Ring E4 (S-E4a Teil II + Dokument 14 E4b/E4c)

R-Agent-11: `wasm-bindgen-cli`/`playwright` sind lokale Sandbox-
Werkzeuge, nicht Teil des versionierten Rust-Workspace
Fundstelle: reports/E4_bericht.md (Einheit b, loom-sdk/wasm-Viewer)
Frage/Konflikt: der Headless-Browser-Zeuge ("wasm-Viewer verifiziert
R1/N-Dateien") braucht `wasm-bindgen-cli` (per `cargo install`) und
`playwright`+Chromium (per `npm install`, Browser-Binary bereits in
der Sandbox vorhanden) — beides Werkzeuge ausserhalb des Cargo-
Workspaces und von `ci/run_ci.sh`.
Vorläufige Behandlung: real ausgefuehrt und das Ergebnis im Bericht
dokumentiert (kein Mock); Build-Skript (`build_wasm.sh`) + Quelltext
(`web/index.html`, `tests/browser_witness.js`) versioniert, Build-
Ausgabe (`web/pkg/`) und Node-Abhaengigkeiten bewusst NICHT versioniert
(.gitignore) — regenerierbar. Derselbe Umgang wie bei den UX-Klick-
Durchlaeufen (Block 1): ein manuell ausgefuehrter, dokumentierter
Nachweis statt eines automatisierten CI-Schritts.
Blockiert: nichts (der Zeuge selbst ist real erbracht).

R-Agent-12: Klassen-Registry fuehrt das Signaturen-Feld sichtbar, aber
derzeit stets leer
Fundstelle: reports/E4_bericht.md (Einheit c, Klassen-Registry)
Frage/Konflikt: keiner der aktuell committeten Seed-Container traegt
ein SIGNATURE-Segment (der Ed25519-Pfad, P6c/X1e, wurde bisher nur
manuell ueber die CLI auf temporaeren Kopien vorgefuehrt). Die
Registry haette fuer eine echte Rollenliste eine neue Kern-
Abhaengigkeit auf `loom-cli` (ed25519-dalek/zstd/blake3) gebraucht —
nur fuer ein stets leeres Feld.
Vorläufige Behandlung: `signatures: Vec::new()` explizit und
kommentiert, keine erfundene Kern-Abhaengigkeit. Sobald ein signierter
Seed existiert, liest man die Rollen direkt aus dessen SIGNATURE-
Frames (dieselbe Cv-Struktur wie `loom_cli::sign::verify_sig_all`).
Blockiert: nichts.

---

## Geschlossene Residuen (Ring E4)

Keine neuen Schliessungen — Ring E4 eroeffnet zwei neue Residuen
(R-Agent-11, R-Agent-12, s. oben). R-Agent-9/10 aus den Vorringen
bleiben wie dort gefuehrt.

---

## Etappe X2 (Ringe E2→E3→E4) — Gesamtabschluss

Alle acht Zeugen aus Dokument 14 §I.7 (cites-Naht), alle vier HBM-
Exit-Zeugen (Ring E3), und alle drei E4-Exit-Zeugen (CE-1, wasm-Viewer,
Klassen-Registry) sind gruen und real erbracht — keine Behauptung ohne
Gegenprobe. Sechs neue Residuen (R-Agent-9..12 minus die durch R-Agent-9
geschlossene) sind sichtbar gefuehrt, keines blockiert den naechsten
Schritt. Etappe X2 ist damit vollstaendig abgeschlossen; nur Ring E5
(L9b Normic Memory) bleibt gesperrt bis zur Spec-Lieferung S-E5.

---

## Nachträge Etappe X3/Ring E5 (S-E5, L9b Normic Memory)

R-Agent-13: das Pattern eines NormCandidate ist caller-geliefert, nicht
algorithmisch aus HBM-Blueprints gemint
Fundstelle: reports/X3_bericht.md (Einheit c, Destillationslauf)
Frage/Konflikt: S-E5 §7 nennt HBM-Blueprints als "optionale
Pattern-Quelle" fuer NormCandidates. Diese Bauphase liest §7 so, dass
L9b selbst NIE Pattern mined — es gatet/promoviert nur (dieselbe
Trennung wie "Kandidat ≠ Norm"). `cce_bridge::distill::distill()`
nimmt das Pattern deshalb als Parameter entgegen, `cce-bridge` haengt
bewusst NICHT von `cce-hbm` ab (keine neue Kopplung fuer eine
Faehigkeit, die architektonisch bei der Integrationsschicht liegt).
Vorläufige Behandlung: der Meilenstein R-NRM-1 authored das Pattern
direkt in `loom_conformance::build_first_active_norm` (textuell
beschreibt es die real beobachtete, wiederkehrende Relation-Kern-
Naht-Regel ueber D02/D03/D06) statt es aus einem echten HBM-Blueprint-
Lauf zu extrahieren. Eine echte HBM→L9b-Pattern-Pipeline (z. B. ein
zertifizierter Blueprint als Pattern-Textquelle) waere ein natuerlicher
Folgeauftrag, sobald ein Anwendungsfall danach verlangt.
Blockiert: nichts — §7 verlangt nur, dass HBM-Pfade NIE direkt eine
aktive Norm erzeugen (kein HBM-Code wurde dafuer geaendert).

R-Agent-14: ScopeGate ist String-/Stichwort-basiert (v1), keine
Semantikanalyse
Fundstelle: reports/X3_bericht.md (Einheit d, BridgeGate Stufe 5)
Frage/Konflikt: §4 verlangt, dass ein Pattern, das ein Gate/Verbot/Tor/
eine Invariante lockern wuerde, strukturell ausgeschlossen ist. Eine
vollstaendige Semantikanalyse von freiem Fliesstext ist aus dem Bau
heraus nicht leistbar; `violates_scope_whitelist` scannt stattdessen
eine feste Stichwort-Liste (z. B. "gate deaktivieren", "capability_lock
entfernen").
Vorläufige Behandlung: bewusst konservativ (lieber ein falsches Reject
als ein durchgerutschtes Lockerungs-Pattern) — dokumentiert als v1,
nicht als vollstaendige Loesung. Erweiterung der Stichwort-Liste (oder
ein strukturierteres Pattern-Format statt freiem Text) ist ein
natuerlicher spaeterer Schritt, kein aktueller Blocker (alle drei
N-NRM-3/PROD-INV-23-Zeugen sind mit der aktuellen Liste real gruen).
Blockiert: nichts.

---

## Geschlossene Residuen (Ring E5)

Keine der Vorring-Residuen (R-Agent-9..12) wird durch Ring E5
geschlossen — sie betreffen andere Bauteile (SCALE-3-Blueprint,
Replay-Inputs, wasm-Werkzeuge, Signaturen). Ring E5 eroeffnet zwei neue
(R-Agent-13, R-Agent-14, s. oben).

---

## R-1b — GESCHLOSSEN (Etappe X3/Ring E5)

**R-1b (Nexus-Bridge L9b, `cce-spec-repo/07_TOTAL_RESIDUE_TO_CLOSURE_
REGISTER.md` #11): GESCHLOSSEN.** Mit S-E5 (`15_L9B_NORMIC_MEMORY_
SPEC.md`) und Ring E5 (Etappe X3) ist L9b vollstaendig gebaut: Crate
`cce-bridge` (ProvenanceSet, RD-gebundene Destillation, sechsstufiges
BridgeGate, Norm-Workbody der Containerklasse `"norm"`, `norms/`-
Registry-Sektion, Aktivierungs-Hook im Runner, Erosion/Widerruf/
Lineage). Meilenstein R-NRM-1 real erbracht: die erste aktive Norm,
destilliert aus drei echten, geschlossenen Familien-Referenz-Cubes,
als zertifizierter `.loom` mit beweisbarer, CitationGate-gruener
Herkunft. Alle 12 Zeugen (R-NRM-1..4, N-NRM-1..8) + PROD-INV-21..23 real
und gruen (`conformance/tests/e5_l9b_normic_memory.rs`,
`crates/cce-runner/tests/g5_gate.rs`). Ausgangs-Gate X3 erfuellt
(Details: `reports/X3_bericht.md`). Dieses vorherige "port_only"-
Register-Eintrag (Typ-Stubs, keine Wirkung) ist damit durch einen
echten, gateten, replay-pflichtigen Bau ersetzt — `spec/` selbst bleibt
unangetastet (das Register dort wird NICHT editiert, s. R-Agent-1).

---

## Etappe X4 (Dokument 16 §2) — R-Agent-13/14 GESCHLOSSEN

**R-Agent-13 (Pattern-Herkunft war caller-geliefert): GESCHLOSSEN.**
Neuer typisierter Extraktor `cce_bridge::extract::blueprint_to_pattern
(&BlueprintCandidate) -> Option<Pattern>` liest echte HBM-Blueprint-
Facetten (die reale `"invariant: {id}|regel={Name}|naht={seam}"`-Form)
und leitet daraus NUR eine der sechs whitelisted `DomainRuleForm`-Formen
ab — nicht abbildbare Blueprints liefern `None`, nie ein geratenes
Pattern (sichtbares Residuum `pattern_extraction_unsupported` beim
Aufrufer). `build_first_active_norm` (R-NRM-1) UND `build_r_cyc_1`
(R-CYC-1, Station 5) destillieren ihr Pattern jetzt beide aus einem
echten, kleinen HBM-Lauf statt aus Freitext. Details: `reports/
X4_bericht.md`.

**R-Agent-14 (ScopeGate war String-/Stichwort-basiert): GESCHLOSSEN.**
`NormCandidate`/`BridgeNorm.pattern` ist jetzt eine typisierte
`Pattern`-Repräsentation (`StructuralRule`/`SeamPattern`/
`ClosureProfile`/`VocabularyNorm`/`ProcessNorm`) — die Fundament-
Schutz-Whitelist (§4) ist damit eine KONSTRUKTIONS-, keine
Texteigenschaft: vier der fünf Formen können eine Lockerung typisch
gar nicht mehr ausdrücken; die einzige verbleibende, genuin
gefährliche Form (`ClosureProfile`, Zusatz-Gates) wird strukturell
gegen die bestehenden Fundament-Gate-IDs geprüft (Kaperungsversuch ⇒
`norm_scope_violation`). N-NRM-3/PROD-INV-23 auf die typisierte Form
umgestellt, bleibt rot. Details: `reports/X4_bericht.md`.

---

## Etappe X4 (Dokument 16 §3c) — Register-Gesamtstand

Konsolidierung JEDER jemals vergebenen Residuen-/Zeugen-Nummer über die
gesamte Bauzeit, mit Endstatus. Dieses Register (`reports/
residuen.md`) ist das AGENTEN-EIGENE, fortlaufend gepflegte Register —
`cce-spec-repo/07_TOTAL_RESIDUE_TO_CLOSURE_REGISTER.md` (das Register
DES Auftraggebers, #1–25) bleibt unangetastet (`spec/`-Disziplin,
R-Agent-1); wo sein "Ist"-Text seit seinem Stand (03.07., vor Block 1-3
und X1-X4) inzwischen überholt ist, wird das hier vermerkt, OHNE die
Datei selbst zu ändern.

### R-Agent-N (dieses Register)

| # | Thema | Endstatus |
|---|---|---|
| R-Agent-1 | Ablageort der Spezifikationen (`cce-spec-repo/spec/`) | OFFEN — dauerhafte Auslegung, blockiert nichts |
| R-Agent-2 | K1–K18/C1–C14-Katalognummern rekonstruiert | OFFEN — dauerhafte Auslegung |
| R-Agent-3 | PhaseBlock-10-Tupel-Feldzuordnung rekonstruiert | OFFEN — dauerhafte Auslegung |
| R-Agent-4 | HyperDAG-Kantentyp-Anzahl (Formel 5 vs. Fliesstext 6) | OFFEN — Spec-vor-Fliesstext-Autoritaet angewandt |
| R-Agent-5 | V0–V9/LOOM-Abnahme(10)/TAT-P1–P7 rekonstruiert | OFFEN — dauerhafte Auslegung |
| R-Agent-6 | egui-Font-Atlas rendert nicht (Sandbox-Grenze) | OFFEN — Host-Termin (GPU-Klickpfad) |
| R-Agent-7 | `xdotool`-Zeichenumkehr-Artefakt der Testkette | OFFEN — harmlos, Testketten-Artefakt |
| R-Agent-8 | LibreOffice-Headless-Konvertierung in der Sandbox kaputt | OFFEN — Umgebungsgrenze, `python-docx` als Gegenbeweis |
| R-Agent-9 | SCALE-3-Blueprint-Zelle war struktureller Platzhalter | **GESCHLOSSEN** (Ring E3) |
| R-Agent-10 | I.5 Replay-Inputs additiv, kein Use-Case | OFFEN — kein Use-Case bisher |
| R-Agent-11 | `wasm-bindgen-cli`/`playwright` sind lokale Sandbox-Werkzeuge | OFFEN — bewusst, regenerierbar |
| R-Agent-12 | Klassen-Registry-Signaturenfeld stets leer | OFFEN — kein signierter Seed existiert |
| R-Agent-13 | Pattern-Herkunft war caller-geliefert | **GESCHLOSSEN** (Etappe X4 §2a) |
| R-Agent-14 | ScopeGate war String-basiert | **GESCHLOSSEN** (Etappe X4 §2b) |
| R-Agent-15 | Keine dedizierte P2-Spezifikationsdatei auffindbar | **GESCHLOSSEN** (Dokument 18 nachgeliefert, 2026-07-04) |

Separat in diesem Register geschlossen: **S1.10-R1** (docx-Bibliothek,
GESCHLOSSEN X1d), **Signatur-Registry-Vollform** (GESCHLOSSEN X1e),
**R-1b** (Nexus-Bridge L9b, GESCHLOSSEN X3/Ring E5, s. oben).

### Register 07 (`cce-spec-repo/07_TOTAL_RESIDUE_TO_CLOSURE_REGISTER.md`, #1–25) — Stand-Abgleich

Der Registerbefund dort lautet wörtlich: "25 Einträge, 0 blockieren die
Bau-DoD, genau 1 (Lizenz) blockiert eine etwaige Veröffentlichung." Das
gilt unveraendert. Seither (Block 1-3, X1-X4) GESCHLOSSEN, obwohl die
Datei selbst (unangetastet) noch den alten Stand zeigt:

| # | Residuum | Registerstand ("Ist") | Tatsaechlicher Stand nach X1-X4 |
|---|---|---|---|
| 11 | Nexus-Bridge L9b | "port_only" | **GESCHLOSSEN** (Ring E5/X3, R-1b) |
| 14 | Signatur-Registry Ed25519+Keyring | "SIGNATURE-Kind registriert" | Mehrfachsignaturen-Vollform **GESCHLOSSEN** (X1e); OS-Keyring-Live-Test bleibt offen (Host-Termin) |
| 15 | blake3-Zweitprofil | "nur sha2-256" | **GESCHLOSSEN** (X1c, CLI-Blatt-Feature) |
| 16 | CDDL unter `schemas/` | "Vertraege in loom-verify" | **GESCHLOSSEN** (15 CDDL-Dateien extrahiert, `reports/P6_haertung_bericht.md`) |
| 17 | GUI-Feindesign 5 Ansichten | "Kernansichten da" | **GESCHLOSSEN** (Block 2, alle 5 Pflichtansichten verdrahtet) |
| 18 | zstd-Transportprofil | "CompressionUnsupported" | **GESCHLOSSEN** (X1c, CLI-Blatt-Feature) |
| 20 | .docx-Export | ".md verlustarm" | **GESCHLOSSEN** (X1d, = S1.10-R1) |
| 25 | Fuzzing/Threat-Model | "Symbol-Scan + Limits" | TEIL-geschlossen: deterministischer CI-Fuzz-Harness + Threat-Model geliefert (`reports/P6_haertung_bericht.md`); coverage-gesteuertes nightly `cargo-fuzz` bleibt Betriebsschritt (K8 teilerfuellt) |

Alle uebrigen 17 Eintraege (#1-10, 12, 13, 19, 21-24) bleiben wie im
Register 07 gefuehrt unveraendert OFFEN (Host-/Betriebstermine,
PL4-Reifepfade, Lizenzentscheidung, redaktionelle Amendment-
Einarbeitung — keiner blockiert den Bau).

### LC-R\* (LOOM-Container-Residuen)

| Nummer | Thema | Endstatus |
|---|---|---|
| LC-R1 | Signatur-Registry (Einzelsignatur -> Mehrfachsignatur-Vollform) | **GESCHLOSSEN** (X1e); OS-Keyring-Live-Test offen (Host) |
| LC-R2 | blake3-Zweitprofil | **GESCHLOSSEN** (X1c) |
| LC-R3 | CDDL-Schemadateien unter `schemas/` | **GESCHLOSSEN** (P6-Haertung) |
| LC-R5 | GUI-Feindesign, 5 Pflichtansichten | **GESCHLOSSEN** (Block 2) |

(LC-R4 kommt an keiner Stelle im Repo vor.)

### CSA-R\*, IG-R\*, S15-R\* (unveraendert offen, Betriebs-/Reifepfade)

| Nummer | Thema | Endstatus |
|---|---|---|
| CSA-R3 | ConnectorAdapter-OAuth-Vollform (= Register-07 #7, = R-16) | OFFEN — Betriebsschritt |
| IG-R1 | Live-Cloud-Provider (= Register-07 #4) | **GESCHLOSSEN** (P1, `CloudModelProviderOpenAI` real gebaut hinter unveraendertem Gateway); realer Betrieb bleibt WO-4/5-Entscheidung (Feature `http` + `OPENAI_API_KEY` sind im Bau AUS/nicht gesetzt) |
| IG-R3 | ExternalAgent produktiv (= Register-07 #5) | OFFEN — Betriebsschritt |
| IG-R4 | Kanzel-Prompt-Bibliothek | OFFEN — Reifepfad |
| S15-R1 | SCALE-2..8-Kerntests (= Register-07 #10, = R-10) | TEILWEISE GESCHLOSSEN — SCALE-1..3 real geschlossen (X1b, E2, R-CYC-1 Station 3); SCALE-4..8 bleiben typisierte, ungebaute Stufen |

### Weitere bare R-N/R-NN (aus der Bauzeit vor diesem Fortsetzungs-Abschnitt)

| Nummer | Thema | Endstatus |
|---|---|---|
| R-1b | Nexus-Bridge L9b | **GESCHLOSSEN** (s. oben) |
| R-6 | MEF-1 Payload-Byte-Kodierung | **GESCHLOSSEN** (`reports/G02_bericht.md`) |
| R-7 | DispersionProfile-Default (Dyadic) | **GESCHLOSSEN** (`reports/G04_bericht.md`) |
| R-8 | HBM Score-Kalibrierung (= Register-07 #21) | OFFEN — Betriebs-Kalibrierlauf |
| R-9 | Amendment-Einarbeitung in S-Dateien (= Register-07 #22) | OFFEN, BEWUSST DAUERHAFT — redaktioneller Auftraggeber-Schritt, vom Agenten nie ausgefuehrt |
| R-10 | SCALE-2..8-Kerntests | s. S15-R1 oben |
| R-13 | Klonungs-Lock (BoundedOperatorSpecialization) | OFFEN, BEWUSST — Lock scharf, Negativzeuge bleibt rot (Betriebsentscheidung) |
| R-16 | ConnectorAdapter-OAuth | s. CSA-R3 oben |
| Repo-R1 | Fehlende LICENSE/NOTICE-Datei (= Register-07 #23) | OFFEN — blockiert nur Veroeffentlichung, nicht den Bau |

---

## Etappe P1 (Dokument 17 §3, Overlay-Klausel: OpenAI statt Anthropic)

**IG-R1 (Live-Cloud-Provider) GESCHLOSSEN.** `CloudModelProviderOpenAI`
(`crates/cce-inference/src/providers/openai.rs`) implementiert
`ModelProvider` real, läuft ausschließlich durch das unveränderte
`run_inference()` (alle zehn Vor-Egress-Gates unverändert aus G8a),
trägt ein vollständiges Manifest (Terms/Privacy/Retention/Budget,
`replay_policy=Recorded`, `capability_locks` für `model_egress`). Der
reale HTTP-Pfad (`ureq`+`serde_json`) liegt hinter dem opt-in-Feature
`http` (Standard AUS, `cce-inference/Cargo.toml`) — CI baut/testet ohne
dieses Feature und bleibt damit strukturell netzfrei; `OPENAI_API_KEY`
wird ausschließlich zur Laufzeit aus der Prozessumgebung gelesen, nie
gespeichert/geloggt/committet. Ohne Feature ODER ohne Schlüssel
degradiert der Provider sichtbar (`provider_unavailable`), kein
Socket-Versuch — dieselbe Disziplin wie `DisabledProvider`. Details,
inkl. der Anleitung, wo/wie der Schlüssel einzutragen ist: `reports/
P1_bericht.md`.

Kein neues Residuum durch P1 selbst. Die reale Erprobung gegen die
tatsächliche OpenAI-API bleibt ein Betriebsschritt (Feature + Schlüssel
aktivieren) — explizit vorgemerkt, nicht blockierend für P1's eigenen
Auftragsumfang (Dokument 17 §3: Manifest + Gates + recorded-Replay real
gebaut).

---

## Nachtrag Etappe P1 — Betriebsverifikation (2026-07-04)

**IG-R1-Zusatz:** die "reale Erprobung"-Klausel aus dem Eintrag oben ist
jetzt einmalig real erbracht (ein manueller, `#[ignore]`-markierter
Smoke-Test, echtes Modell `gpt-4o-mini`, Details `reports/
P1_bericht.md` §„Betriebsverifikation"). Das ändert IG-R1s Endstatus
nicht (bereits GESCHLOSSEN durch den P1-Bau selbst) — es dokumentiert
nur, dass der zuvor vorgemerkte Betriebsschritt inzwischen einmal
tatsächlich durchgeführt wurde. Der Bau-Default (`cargo test
--workspace`, `bash ci/run_ci.sh`) bleibt unverändert netzfrei; Feature
`http` bleibt AUS.

---

## Nachtrag P2-Anschluss (2026-07-04) — R-Agent-15

R-Agent-15: keine dedizierte P2-Spezifikationsdatei in `cce-spec-repo/`
auffindbar
Fundstelle: Auftrag verweist auf Dokument 17 §3 ("`17 MESSLATTE PARITY
SURPASS.md`"); durchsucht wurden alle Dateien unter `cce-spec-repo/`
(Top-Level 00–17 + `spec/00_kern`..`spec/40_format`) nach "P2",
"SWE-Tiefe", "PL3", "Repo-Workbody", "ToolCapabilityLock".
Frage/Konflikt: Dokument 17 §3 enthält nur EINEN Satz zu P2 ("die
Software-Familie von PL3-Struktur auf Arbeitsreife: Repo-Workbody
(Code-Einheiten, Diffs als Artefakte, Build-/Test-Läufe als Evidence),
Tool-Klassen fs/git/build/test real unter ToolCapabilityLocks") — keine
eigene P2-Spec-Datei mit Feldern/Zeugen/Abnahmekriterien wie bei den
anderen Etappen (vgl. S-E5 für Ring E5, Dokument 16 für X4). Ohne
Vorgabe zu Feldern/genauen Testkriterien/Abgrenzung gegenüber dem
bereits bestehenden `cce-toolgateway` (der laut P1_bericht/vorherigen
Berichten bereits fs/git/build/test-Tool-Klassen SPEZIFIZIERT+GETESTET
hat, aber laut Auftrag jetzt "real gefahren" werden sollen) wäre jede
konkrete Ausgestaltung geraten.
Vorläufige Behandlung: NICHT eigenständig spezifiziert — Bau gestoppt,
Rückfrage beim Auftraggeber gestellt (s. Chatverlauf), bevor P2 begonnen
wird.
Blockiert: den Beginn von P2, bis Rückfrage beantwortet ist.

**GESCHLOSSEN (2026-07-04):** Dokument 18 (`cce-spec-repo/18 P2 SWE
TIEFE SPEC.md`) wurde nachgeliefert — vollständige P2-Spezifikation
(§1-§9, Objektmodell, fünf Werkzeugklassen, sechs Werkzeug-Gates,
Kern-Kette, Zeugen R-SWE-1..5/N-SWE-1..8, Bauplan a→h). P2-Bau folgt
ab jetzt exakt diesem Dokument.

---

## Etappe P2 (Dokument 18) — SWE-Tiefe

Crate `crates/cce-swe` real gebaut: Objektmodell (RepoWorkbody/
RepoSnapshot/DiffCandidate/BuildRun/TestRun/TaskLedger), fünf
Werkzeugklassen (fs_read bereits aus P1-Aera, fs_write/git/build/test
neu) real im `ToolGateway` (`crates/cce-toolgateway`), sechs
Werkzeug-Gates (vier neu, drei aus `cce_inference::gates`
wiederverwendet), die Kern-Kette DiffCandidate→apply(dry-run)→
BuildRun→TestRun→Gates→PhaseBlock, eine provider-erzeugte Diff (OpenAI
hinter dem unveränderten Gateway, recorded), Replay-Determinismus, 14
Zeugen (R-SWE-1..5/N-SWE-1..8 + Typ-Erreichbarkeit) dauerhaft im
Wächter (`conformance/swe/swe_catalog.rs`). RepoWorkbody ist eine neue
Containerklasse `"repo"` (additiv in `loom_format::PROFILES`,
`required_kinds` in `loom-verify` additiv erweitert) — kein neues
Segment nötig. Reale Prozess-Aufrufe (git/build/test) liegen NUR unter
dem neuen opt-in-Feature `process` (Standard AUS, dieselbe Disziplin
wie `http`); CI bleibt hermetisch. Details: `reports/P2_bericht.md`.

Kein neues Bau-Residuum durch P2 selbst. Zwei Betriebsschritte
ausdrücklich vorgemerkt (kein Blocker): (1) eine reale, manuelle
Betriebsverifikation des Features `process` gegen ein echtes
Referenz-Repository (echte `cargo build`/`cargo test`/`git commit`) —
analog zu P1s Betriebsverifikation gegen die echte OpenAI-API; (2) eine
echte, live gegen die tatsächliche OpenAI-API laufende Diff-Erzeugung
(statt der hermetischen `RecordedOpenAiDiffProvider`-Fixture, Feature
`http` + `OPENAI_API_KEY`, unverändert aus P1).

---

## Etappe P3 (Dokument 19) — Dogfooding

Crate `crates/cce-dogfood` real gebaut: TaskProposal + ProtectedPathFence
+ drei neue Gates (TaskProposalGate/ProtectedPathGate/BranchIsolationGate,
plus strukturelles MergeExclusionGate), Kern-Kette verdrahtet über die
UNVERÄNDERTEN P2-Bausteine. **R-DOG-1 real erbracht** am eigenen
Repository (Branch `dogfood/p3-001`, ungemergt): echte Kanzel → Diff →
echtes Build/Test → PhaseBlock → RepoWorkbody `verify == Valid` → Replay
→ realer `git commit` mit aufgezeichneter Bestätigung. N-DOG-1..4
hermetisch im Wächter. Kein automatischer Merge nach `main` (bleibt
exklusiv Auftraggeber-Handlung). Details: `reports/P3_dogfooding_bericht.md`.

Kein neues Bau-Residuum durch P3 selbst. Ehrlich dokumentiert: R-DOG-1
brauchte mehrere echte Kanzel-Anläufe (Format-/Selbstkonsistenzfehler des
Modells, alle korrekt von BuildEvidenceGate/TestEvidenceGate gefangen) +
einen eigenen Harness-Fehler — vollständige Historie im Bericht.

## Etappe P4 (Dokument 20) — Vergleichsläufe (K9 erfüllt)

Crate `crates/cce-benchmark` real gebaut: FairnessGate +
ComparisonSealGate, Container-Klasse `"benchmark"` (additiv in
`loom_format::PROFILES` + `loom-verify`, unveränderte „repo"-Regel exakt
erhalten), D1–D6-Matrixbau. **R-BENCH-1 (Coding) + R-BENCH-2 (Dokument)
real gefahren:** ungegatetes gpt-4o-mini gegen CCE, dasselbe Modell —
**D4/D5 Parität** in beiden Klassen (beide criteria_met, 0 Eingriffe;
die Beweispflicht kostet an Aufgabenqualität nichts), **D1/D2/D3/D6
kategorisch** nur beim CCE-Arm (der ungegatete Arm hat strukturell keinen
Evidence-/Replay-/Governance-/Audit-Mechanismus). Beide
Benchmark-Workbodies `verify == Valid`. N-BENCH-1/2 hermetisch im Wächter.
**K9 · CompetitiveDoD erfüllt.** Details:
`reports/P4_vergleichslaeufe_bericht.md`.

Kein neues Bau-Residuum durch P4 selbst. Ausdrücklich benannt (kein
Blocker, nicht Teil der P4-DoD): ein realer Vergleich gegen
Cursor/Copilot/Bolt selbst (bräuchte deren Konten/Lizenzen —
`cce-benchmark` nimmt jeden weiteren Arm ohne neue Architektur auf);
breitere Aufgaben-Stichproben zur statistischen Absicherung.

## Etappen P2-Ext (Dokument 21) + P4-Ext (Dokument 22) — Nachtrag

**P2-Ext · Agent-Grounding: GESCHLOSSEN (gebaut + bezeugt).**
`cce-swe/grounding` (RuleAtom/DecisionSlot/GroundingPacket/
compile_grounding/export_context), drei neue Gates (DeltaBudget/
ContextBudget/RuleCompliance), fünf neue Residuen. Zeugen R-GND-1..4,
N-GND-1..2 hermetisch im Wächter, plus Wert-Beweis: ein Diff, den die
P2-Gates grün durchlassen, fängt `run_grounded_swe_task` via
RuleComplianceGate (`conformance/grounding/grounding_catalog.rs`).
`run_swe_task` selbst unverändert (Alt-Zeugen unberührt).

**P4-Ext · Dritter Arm (echte Werkzeuge): Bausteine GESCHLOSSEN,
echter Lauf UMKLASSIFIZIERT.** ExternalToolResult,
ThreeArmComparisonMatrix, `three_arm_fairness_gate`
(packet_digest-Gleichheit über alle Arme; packet_digest per
`grounded_task_package_digest` strukturell in den task_package_digest
eingefaltet), Drei-Arm-Workbody (echte Obermenge des
Zwei-Arm-Containers, `verify == Valid`). Zeugen R-BENCH-EXT-STRUCT,
N-BENCH-EXT-1 (+ Digest-Variante), R-BENCH-EXT-2-Residuumpfad
hermetisch im Wächter; `#[ignore]`-Export-Schritt
(`r_bench_ext_export.rs`) liefert das GroundingPacket samt Digests
für einen Fremdarm.

| Nummer | Gegenstand | Endstatus |
|---|---|---|
| R-BENCH-EXT-1/2 (realer Lauf) | echter Vergleich gegen Cursor/Copilot/Bolt mit identischem GroundingPacket-Digest | OFFEN, OPTIONAL — Auftraggeber-Entscheidung (Dokument 23): zurückgestellt, Bedienlast läge beim Menschen; jederzeit nachholbar, blockiert nichts. K9 gilt in der P4-Fassung; der wörtliche Marktvergleich bleibt sichtbar offen — kein Overclaim |

---

**Keine verwaiste Nummer:** jede oben gelistete Nummer hat einen
Endstatus (GESCHLOSSEN/TEILWEISE GESCHLOSSEN/OFFEN+Grund) und einen
Fundstellen-Verweis. Kein Eintrag blockiert den Bau-DoD; Repo-R1
(Lizenz) blockiert ausschliesslich eine etwaige externe
Veroeffentlichung, wie im Register 07 selbst vermerkt.
