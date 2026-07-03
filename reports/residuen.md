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
