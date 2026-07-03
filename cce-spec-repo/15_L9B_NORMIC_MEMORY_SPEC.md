# 15 — S-E5: L9b NORMIC MEMORY (Vollspezifikation, öffnet R-1b)

**Der Paradigmakern:** ein Gedächtnis, das ausschließlich aus Geschlossenem besteht. Wiederkehrende, bewährte Regelmäßigkeiten über viele zertifizierte Arbeitskörper werden zu **Normen** destilliert — herkunftsgebunden, gate-promoviert, revidierbar, sichtbar erodierend, und in der Anwendung strikt opt-in. Eine Norm dekretiert nie Wahrheit; sie trägt ihre Beweisbasis mit sich. **Kein neues Tor:** L9b hat keinerlei Egress — „Bridge" ist die Brücke zwischen Arbeiten und Gedächtnis, nicht zum Netz. Normativer Overlay in der etablierten Autoritätsordnung; Bauphase = Etappe X3 (Ring E5).

## §1 Leitsätze

**N-A1 (Herkunftspflicht):** Jede Norm existiert nur über einer expliziten Menge geschlossener, verifizierter Arbeitskörper (ProvenanceSet) — adressiert über Inhaltsklassen, nie über Pfade. **N-A2 (Statistik ordnet, Gate+Mensch entscheiden):** Support-/Gegenbeispiel-Zahlen sind Ordnungs-, nie Entscheidungsgrößen; Promotion trotz bekannter Gegenbeispiele ist HITL-pflichtig. **N-A3 (Opt-in-Wirkung):** Eine Norm wirkt nur, wenn ein Auftrag sie ausdrücklich aktiviert; dann wird sie echtes, fail-closed Gate *dieses Laufs* und Replay-Input. Nie automatische Verschärfung, nie automatische Lockerung. **N-A4 (Fundament-Schutz):** Normen können Prüfungen *hinzufügen*, niemals Invarianten, Verbote, Tore oder bestehende Gates lockern (Pattern-Whitelist, §4). **N-A5 (Sichtbare Erosion):** Verliert die Beweisbasis ihre Gültigkeit, degradiert die Norm automatisch und sichtbar — ein Gedächtnis, das mit seinen Beweisen altert, statt sie zu überleben. **N-A6 (Keine versteckte Destillation):** Quiescence ist Default; Destillation läuft nur als expliziter, RD-gebundener Lauf — nie als Hintergrundprozess.

## §2 Objektmodell

**ProvenanceSet:** deduplizierte, sortierte Menge von `core_root`s; Auflösung über den CitationResolver-Port (SeedResolver und Klassen-Registry aus E4c); jedes Mitglied MUSS `Valid` und geschlossen sein; Mitglieder unter Quarantäne oder mit offenen `supports`-cites sind unzulässig (Quellen-Quiescence). Parameter je NexusClass: Mindestkardinalität `κ_min` (Default 3) und `diversity_rule` (Default: ≥ 2 verschiedene Domänen ODER ≥ 2 verschiedene Erzeugungsläufe).

**NexusClass** (Typ der Norm, abschließende Aufzählung v1): `structural_rule` (DomainRule-förmige Strukturregel) · `seam_pattern` (wiederkehrendes Naht-Muster) · `closure_profile` (empfohlene Zusatz-Gate-Konfiguration) · `vocabulary_norm` (Terminologie-/Residuen-Benennungsnorm) · `process_norm` (PhaseLadder-/Reihenfolge-Muster). Jede Klasse definiert Anwendungsort, Prüfform, Widerrufswirkung. Erweiterung der Aufzählung nur über den S14-Pfad.

**NormCandidate:** `(pattern, nexus_class, provenance_set, n_support, n_counter, known_counterexamples[core_root+Kurzgrund], scope ∈ {domain:<id>, family:<id>, global}, distillation_rd)`. Quellen: Destillationslauf über Registry-Bestand und/oder HBM-Blueprints als Pattern-Lieferant. **Gegenbeispiele sind Pflichtfeld** — ein Kandidat, der Gegenbeispiele verschweigt, ist ungültig. Kandidat ≠ Norm; das Kandidaten-Commit-Verbot gilt unverändert.

**BridgeNorm** (die promovierte Norm): `(norm_id = content_class des Norm-Workbody, nexus_class, pattern, provenance_set, known_counterexamples, scope, status ∈ {active, deprecated, revoked}, promotion_evidence, lineage {supersedes?})`. **Speicherform: die Norm IST ein `.loom`-Workbody** der neuen Containerklasse `norm` — damit erbt sie alles Bestehende: verify, Signatur, Transport, Registry-Eintrag, und vor allem: **ihr ProvenanceSet ist ihre `external_citations`-Liste mit `cite_kind=derives`** (S-E2a wirkt wörtlich weiter; CitationGate prüft die Herkunft). Pflichtsegmente: MANIFEST (+Norm-Felder, additiv/minor), CL (Pattern als Regelstruktur), EVIDENCE (BridgeGate-Report, Statistik, ggf. HITL-PhaseBlock-Ref), LEDGER, RESIDUE, REPLAY_MANIFEST (Destillations-RD).

**Normic Memory:** die `norms/`-Sektion der Klassen-Registry (E4c-Katalog-Workbody, additiv erweitert): Index nach `scope × nexus_class × status`, inklusive Widerrufs- und Lineage-Führung. API (lesend): `query(scope, class) → [BridgeNorm]`; Anwendung ausschließlich über Aktivierung (§5).

## §3 BridgeGate — die Promotionsprüfung (fail-closed, Verdikt allow|hold|reject)

(1) **ProvenanceGate:** alle Mitglieder aufgelöst, `Valid`, geschlossen, quarantänefrei; `|set| ≥ κ_min` — sonst `insufficient_provenance`/Hold. (2) **DiversityGate:** `diversity_rule` erfüllt — sonst `diversity_unmet`/Hold. (3) **CounterexampleGate:** `n_counter` vollständig geführt; bei `n_counter > 0` ist Promotion **nur** mit aufgezeichneter Operator-Bestätigung zulässig (HumanConfirmationGate; die Gegenbeispiele werden `known_counterexamples` der Norm) — verschwiegene Gegenbeispiele ⇒ reject. (4) **ConflictGate:** kein Widerspruch zu aktiver Norm gleichen Scopes — sonst `norm_conflict`, beide sichtbar, Hold. (5) **ScopeGate (Fundament-Schutz):** Pattern-Typ ∈ Whitelist der NexusClass; jedes Pattern, das ein bestehendes Gate, Verbot, Tor oder eine Invariante lockern, ersetzen oder bedingt aussetzen würde ⇒ `norm_scope_violation`, reject. (6) **DistillationReplayGate:** gleicher Registry-Snapshot + gleiches RD ⇒ klassenidentischer Kandidat — sonst `distillation_replay_mismatch`, rot.

## §4 Pattern-Whitelist (abschließend v1)

Zulässig sind ausschließlich **additive Prüf- und Vorschlagsformen**: DomainRule-Instanzen der sechs bestehenden Regeltypen · Naht-Muster-Empfehlungen (`seam_pattern`) · Zusatz-Gate-Profile, die nur verschärfen (`closure_profile`) · Benennungs-/Terminologie-Normen · Reihenfolge-Muster. Ausgeschlossen per Konstruktion: alles mit Wirkung auf Egress, Capability-Locks, Verdikt-Schreibwege, Score-Semantik, Fundamentinvarianten, bestehende Gate-Schwellen nach unten.

## §5 Aktivierung, Wirkung, Replay

Ein Auftrag/eine Capsule kann ein `norm_profile` führen: explizite Liste aktivierter `norm_id`s. Nur dann: die Pattern wirken als **zusätzliche, fail-closed Gates dieses Laufs**; der RD listet die `norm_id`s als Inputs (Replay verlangt dieselben Norm-Klassen); jede norm-geformte Kanzel-/Workbench-Hilfe trägt die Markierung `norm-geformt:<norm_id>`. Anwendung ohne Aktivierung ⇒ `norm_not_activated`, PROD-INV-Verstoß. **PROD-INV-Erweiterung:** **PROD-INV-21** keine Promotion außer durch BridgeGate mit ProvenanceSet · **PROD-INV-22** keine Norm-Wirkung ohne explizite Aktivierung im RD · **PROD-INV-23** Normen lockern nie (ScopeGate-Whitelist strukturell).

## §6 Erosion, Widerruf, Lineage

**Erosion (automatisch, sichtbar):** Wird ein ProvenanceSet-Mitglied invalid, widerrufen oder quarantänisiert, fällt die Norm bei der nächsten Registry-Pflege auf `deprecated` mit Residuum `provenance_erosion` — nie stiller Fortbestand. **Widerruf (gegateter Lauf):** `revoke(norm_id, Begründung, Gegenbelege)` erzeugt einen Widerrufs-Workbody (zitiert die Norm + Belege), Status `revoked`, Registry führt Widerrufe sichtbar. **Klassenstabilität:** Läufe, die eine später widerrufene Norm aktiviert hatten, bleiben klassenstabil (der RD hält die damalige Norm-Klasse); ihre Reanalyse zeigt das Informations-Residuum `norm_since_revoked`. **Lineage:** Ersatz-Normen führen `supersedes`; die Kette ist im Memory abfragbar.

## §7 Abgrenzung und Kopplung zu HBM

HBM mined **Strukturen** (Blueprints = wiederverwendbare Bauformen, Kandidatengewinnung aus Korpora). L9b destilliert **Normen** (Regeln mit Geltungsanspruch und Herkunftspflicht aus *geschlossenen* Arbeiten). Kopplung: HBM-Blueprints dürfen NormCandidates das Pattern liefern — die Promotion läuft ausschließlich durch das BridgeGate; kein HBM-Pfad erzeugt direkt eine aktive Norm.

## §8 Residuen-Vokabular (vollständig, jedes mit Zeuge)

`insufficient_provenance · diversity_unmet · counterexample_unresolved · norm_conflict · norm_scope_violation · provenance_erosion · norm_since_revoked · distillation_replay_mismatch · unresolved_norm_citation · norm_not_activated`.

## §9 Zeugen (Wächter-Eintritt mit X3)

**R-NRM-1** Destillation über ≥3 geschlossene Familien-Referenz-Cubes ⇒ Kandidat ⇒ BridgeGate ⇒ aktive Norm als `.loom` (`verify` Valid; `derives`-cites auf die Herkunft, CitationGate grün). **R-NRM-2** Aktivierung: Lauf mit `norm_profile` — Pattern wirkt als Gate, RD listet norm_id, Replay klassenidentisch. **R-NRM-3** Widerruf: Widerrufs-Workbody, Status revoked, Alt-Lauf-Reanalyse zeigt `norm_since_revoked`. **R-NRM-4** Erosion: Mitglied wird invalid ⇒ Norm automatisch `deprecated` + `provenance_erosion`. **N-NRM-1** κ < κ_min ⇒ Hold. **N-NRM-2** Kandidat versucht Wirkung ohne Promotion ⇒ reject. **N-NRM-3** Pattern will Gate lockern ⇒ `norm_scope_violation` reject. **N-NRM-4** verschwiegene Gegenbeispiele ⇒ reject. **N-NRM-5** Norm-Konflikt ⇒ beide Hold, sichtbar. **N-NRM-6** Anwendung ohne Aktivierung ⇒ `norm_not_activated`. **N-NRM-7** Destillation ohne RD (Hintergrund-Simulation) ⇒ verboten/reject. **N-NRM-8** Replay-Mismatch der Destillation ⇒ rot.

## §10 Bauplan Etappe X3 (Ring E5) und DoD

Crate `crates/cce-bridge` (kein Egress; Ports: cce-core, loom-cites/Resolver, Klassen-Registry, cce-runner [Aktivierungs-Hook], cce-hbm [optionale Pattern-Quelle]). Reihenfolge: a) Typen+Residuen → b) ProvenanceSet+Auflösung → c) Destillationslauf (RD-gebunden) → d) BridgeGate (6 Stufen, HITL-Pfad als aufgezeichneter Eingang) → e) Norm-Workbody-Bau (Containerklasse `norm`, derives-cites) → f) Memory/Registry-Sektion + query → g) Aktivierung im Runner (`norm_profile`, RD-Input) → h) Erosion+Widerruf+Lineage → i) Zeugen R-NRM-1..4/N-NRM-1..8 in den Wächter → j) Doku-Zeile + Status/Register. **Ausgangs-Gate X3:** alle 12 Zeugen korrekt · Alt-Zeugen unverändert · PROD-INV-21..23 negativ-getestet · CI GRUEN · kein neues Kern-Crate mit externen Abhängigkeiten. **DoD(L9b)=1 ⟺** §1–§9 implementiert ∧ Gate X3 grün ∧ R-1b im Register als GESCHLOSSEN geführt.

*Damit ist jeder Ring der Karte spezifiziert. Nach X3 besitzt das System das letzte Organ: ein Gedächtnis, das nur behält, was bewiesen wurde — und vergisst, was seine Beweise verliert.*
