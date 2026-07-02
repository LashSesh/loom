# Residuenregister des Bau-Agenten (fortlaufend, sichtbar — 04_AGENT_AUFTRAG §3)

R-Agent-1: Ablageort der Spezifikationen
Fundstelle: 00_START_HIER §5 (Zielstruktur: `spec/` neben `cce/`)
Frage/Konflikt: Das übergebene Repository enthält die Spezifikationen im ZIP-entpackten
Verzeichnis `cce-spec-repo/spec/` statt in einem Wurzel-`spec/`. Verschieben/Kopieren würde
physische Duplikate der read-only Spezifikationen erzeugen.
Vorläufige Behandlung: konservative Auslegung — `cce-spec-repo/spec/` gilt als DER read-only
Spec-Baum; keine Datei wird bewegt oder editiert; CI prüft Integrität an Ort und Stelle.
Blockiert: nichts
