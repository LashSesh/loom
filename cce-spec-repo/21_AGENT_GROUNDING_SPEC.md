# 21 — AGENT-GROUNDING-SPEZIFIKATION (Regeln und offene Entscheidungen als geprüfte Objekte)

**Herkunft und Abgrenzung:** Diese Spezifikation ist eine eigenständige, für CCE neu gefasste Synthese — angeregt durch einen frei liegenden Entwurf für ein anderes Projekt des Auftraggebers, aber vollständig in CCE-eigener Sprache neu gedacht: keine übernommenen Fremdbegriffe, kein fremdes Koordinatensystem, keine Nennung der Quelle in Code, Kommentaren oder Doku. Erweitert P2 (SWE-Tiefe); Bauphase **P2-Ext**, vor P4-Ext (Dokument 22).

## §1 Das Problem, das gelöst wird

Ein Coding-Agent — CCE selbst oder ein Fremdwerkzeug — braucht Projektwissen: welche Regeln gelten, welche Entscheidungen noch offen sind, welche Werkzeuge existieren. Bisher lebt das entweder implizit im Code (Gates) oder gar nicht. Für SWE-Tiefe fehlt ein **geprüftes, wiederholbares Objekt**, das diese drei Dinge trägt — und das später als **einzige, gehashte Quelle** dient, aus der sowohl CCEs eigene Arbeit als auch eine abgeleitete Briefing-Datei für ein Fremdwerkzeug entstehen (Dokument 22).

## §2 Neue Objekte (Crate-Erweiterung `crates/cce-swe`, kein neues Kern-Crate)

**RuleAtom:** `(rule_id, scope [Pfad/Modul/Befehl], trigger, prescription, severity ∈ {advisory, required, blocking}, evidence_ref, gate_ref?, decay?)`. Eine Regel ohne `evidence_ref` (Repo-Beleg, Auftraggeber-Direktive oder vorherige akzeptierte Entscheidung) wird beim Kompilieren automatisch auf `advisory` herabgestuft — nie stillschweigend als `blocking` geführt (**RuleEvidenceGate**, neu).

**DecisionSlot:** `(decision_id, question, domain [zulässige Optionen], status ∈ {open, proposed, resolved, stale, superseded}, candidates[], evidence_ref?, resolved_as?, expires?)`. Eine offene architektonische Entscheidung wird NIE stillschweigend von einem Modell getroffen — sie bleibt `open` bis ein PhaseBlock sie explizit auflöst (aufgezeichnet wie jede HITL-Entscheidung, S5-A5-Disziplin: `resolved`-Übergang trägt RD-Ref + Evidence).

**GroundingPacket:** `(package_id, rules[RuleAtom], open_decisions[DecisionSlot], tool_surface[test/build/lint-Befehle], packet_digest)` — die kanonische, kompilierte Fassung; `packet_digest` ist ein Merkle über sortierte Regeln+Entscheidungen (deterministisch, unabhängig von Einfügereihenfolge — dieselbe Disziplin wie `RepoSnapshot::snapshot_root()`). **Kein Fließtext-Prompt ist die Quelle — nur dieses Objekt.** Abgeleitete, nicht-maßgebliche Export-Projektionen (menschenlesbare Kontextdatei für ein Fremdwerkzeug) entstehen als reine Funktion `export_context(packet) -> String` — der Export trägt einen sichtbaren Herkunftsverweis auf `packet_digest`, ist aber selbst nie Eingabequelle für irgendeine Prüfung.

## §3 Zwei neue Gates (Erweiterung der P2-Gate-Familie, dieselbe Form)

**DeltaBudgetGate:** ein DiffCandidate, dessen Zeilenumfang das im Auftrag deklarierte Budget überschreitet, hält — außer bei explizit aufgezeichneter Erhöhung des Budgets durch den Auftraggeber (materielle Aktion, HumanConfirmationGate). Verhindert unbemerkt riesige Änderungen. **ContextBudgetGate:** ein GroundingPacket, das die deklarierte Zeichen-/Token-Obergrenze überschreitet, hält mit `context_budget_exceeded` — kein stilles Kürzen sensibler Regeln.

## §4 Kompilierung und Wirkung

`compile_grounding(Π) -> GroundingPacket` liest Projektsubstrat (Dateien, Werkzeugoberfläche, bestehende Domänen-/Familien-Regeln aus dem Katalog, offene Residuen als Decision-Slot-Kandidaten) und erzeugt das Packet deterministisch — **kompiliert, nicht diktiert** (dieselbe Disziplin wie ProjectionPacket, S15/HBM: nie der rohe Vollkontext). Ein `GroundingPacket` wirkt in `run_swe_task`/`run_dogfood_task` als zusätzliche Vorprüfung: jeder DiffCandidate wird gegen `blocking`-RuleAtoms geprüft (`RuleComplianceGate`, neu — reject bei Verstoß, unabhängig vom Modell-Output).

## §5 Residuen-Vokabular

`rule_missing_evidence` (automatische Herabstufung, sichtbar) · `decision_left_open` (Lauf endet mit offenem Slot — Hold, kein stiller Default) · `context_budget_exceeded` · `delta_budget_exceeded` · `rule_violation` (blocking-Regel verletzt).

## §6 Zeugen

**R-GND-1** RuleAtom ohne Beleg wird automatisch zu `advisory` herabgestuft (sichtbar, nicht stillschweigend). **R-GND-2** DiffCandidate verletzt eine `blocking`-Regel ⇒ `RuleComplianceGate` reject, unabhängig vom sonstigen Gate-Ergebnis. **R-GND-3** GroundingPacket-Digest ist deterministisch (zwei Kompilierläufe, identisches Ergebnis, ordnungsunabhängig). **R-GND-4** eine offene DecisionSlot wird durch einen PhaseBlock aufgelöst, aufgezeichnet mit RD-Ref. **N-GND-1** Diff über Budget ohne Erhöhungs-Bestätigung ⇒ Hold. **N-GND-2** Packet über Kontextbudget ⇒ Hold, kein Kürzen.

## §7 DoD

**DoD(Agent-Grounding) = 1 ⟺** §2–§6 implementiert ∧ alle sechs Zeugen grün/rot korrekt ∧ Alt-Zeugen (P1–P4) unverändert ∧ kein neues Kern-Crate, keine externe Kiste in `cce-swe` ∧ **mindestens ein Nachweis, dass ein GroundingPacket eine reale Regelverletzung fängt, die die bisherige Gate-Familie allein nicht gefangen hätte** (der eigentliche Wertbeweis, analog zur P4-Kernthese).

*Anschluss: Dokument 22 (P4-Ext) nutzt `GroundingPacket` als die eine, gehashte Quelle, aus der beide Vergleichsarme — CCE und ein echtes Fremdwerkzeug — dieselben Regeln erhalten.*
