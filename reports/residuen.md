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
