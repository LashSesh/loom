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
