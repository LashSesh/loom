# L9b — Normic Memory (Containerklasse `norm`)

Der Paradigmakern: ein Gedächtnis, das ausschließlich aus Geschlossenem
besteht (S-E5, Etappe X3/Ring E5 der Ökosystem-Expansionskarte, öffnet
Residuum R-1b). Wiederkehrende, bewährte Regelmäßigkeiten über viele
zertifizierte Arbeitskörper werden zu Normen destilliert —
herkunftsgebunden, gate-promoviert, revidierbar, sichtbar erodierend,
und in der Anwendung strikt opt-in.

## Was neu ist

Ein neues Crate `cce-bridge` (kein Egress, nur benannte Ports: den
bestehenden `CitationResolver` aus `loom-cites`, den `.loom`-Kern, und
`cce-core` für `RunDescriptor`/`HitlDecision`) trägt das gesamte L9b:

- **ProvenanceSet** — eine deduplizierte, sortierte Menge von
  `core_root`s, über den bestehenden `CitationResolver`-Port aufgelöst
  (`SeedResolver`/Klassen-Registry-`RegistryResolver`, unverändert aus
  E2/E4c). Jedes Mitglied muss `Valid` (exakt) und geschlossen sein;
  eigene offene `supports`/`derives`-Nähte machen ein Mitglied
  unzulässig (Quellen-Quiescence).
- **NormCandidate** — nie automatisch aus HBM-Blueprints erzeugt; das
  Pattern liefert der Aufrufer (HBM/Registry-Bestand als
  Pattern-Lieferant), L9b gatet/promoviert nur. Ein RD-gebundener
  Destillationslauf (`cce_bridge::distill`) verlangt zwingend einen
  vollständigen `RunDescriptor` — keine Hintergrund-Destillation.
- **BridgeGate** (`cce_bridge::gate`) — sechs Stufen, fail-closed,
  Verdikt `allow | hold | reject`: ProvenanceGate, DiversityGate,
  CounterexampleGate (HITL-Bestätigung über `HitlDecision`, dasselbe
  Muster wie S5-A5), ConflictGate, ScopeGate (Fundament-Schutz — eine
  Pattern-Whitelist schließt jede Lockerung von Gates/Invarianten/
  Capability-Locks/Egress/Verdikt-Semantik strukturell aus),
  DistillationReplayGate.
- **Norm-Workbody** — die Norm IST ein `.loom`-Workbody der neuen
  Containerklasse `"norm"` (additiv in `loom-format::PROFILES` und
  `loom-verify::required_kinds`). Ihr ProvenanceSet ist ihre
  `external_citations`-Liste mit `cite_kind=derives` — S-E2a wirkt
  wörtlich weiter, `CitationGate` prüft die Herkunft.
- **Normic Memory** — additive `norms/`-Sektion der Klassen-Registry
  (E4c-Katalog-Workbody): Index über Scope × NexusClass × Status,
  `query(scope, class)`.
- **Aktivierung** — `RunDescriptor` trägt additiv `norm_profile:
  Vec<String>` (die aktivierten `norm_id`s, RD-Input, Replay-pflichtig).
  `cce-runner` prüft vor dem Weben fail-closed, dass jede gelistete
  `norm_id` unter einer vom Aufrufer aufgelösten, `Active` Norm steht
  — sonst `norm_not_activated`. Eine Norm, die nicht im Profil steht,
  wirkt nie, selbst wenn sie dem Lauf bekannt ist (strikt opt-in).
- **Erosion/Widerruf/Lineage** (`cce_bridge::lifecycle`) — ein
  invalid/quarantänisiertes ProvenanceSet-Mitglied lässt die Norm
  automatisch auf `deprecated` fallen; `revoke()` siegelt einen echten
  Widerrufs-Workbody (zitiert Norm + Gegenbelege); Alt-Läufe bleiben
  klassenstabil, ihre Reanalyse zeigt `norm_since_revoked`;
  Ersatznormen führen `supersedes`.

## Wo es lebt

`crates/cce-bridge` — Abhängigkeitsrichtung `cce-runner -> cce-bridge`
(kein Zyklus, `cce-bridge` kennt `cce-runner` nicht). `cce-bridge`
selbst hängt zusätzlich von `loom-cites`/`loom-canon`/`loom-format`/
`loom-codec`/`loom-verify`/`loom-replay` ab — eine explizit benannte
Ausnahme in `ci/check_acyclic.py` (S-E5 §10 autorisiert diese Ports
wörtlich, dieselbe Disziplin wie `cce-runner`/`cce-observe`).

## Fundament-Schutz

Eine Norm kann Prüfungen HINZUFÜGEN, niemals Invarianten, Verbote, Tore
oder bestehende Gates lockern. Das ist strukturell erzwungen: die
Pattern-Whitelist im ScopeGate weist jedes Pattern ab, das eine
Wirkung auf Egress, Capability-Locks, Verdikt-Schreibwege,
Score-Semantik oder Gate-Schwellen-nach-unten beansprucht
(`norm_scope_violation`, reject). Und: nur ein echter `bridge_gate()`-
Durchlauf kann ein `Allow`-Verdikt erzeugen — `BridgeGateReport`s
Felder und Konstruktoren sind privat, kein Aufrufer kann sich eines
verschaffen, ohne das Gate wirklich zu durchlaufen (PROD-INV-21).

## Kompatibilität

Additiv, minor: `norm_profile` ist bei jedem bestehenden RD leer (kein
Lauf aktiviert je Normen implizit); Alt-Läufe/Alt-Container bleiben
klassenstabil und unangetastet.

## Zeugen

- **R-NRM-1** (Meilenstein) — die erste aktive Norm: drei echte,
  geschlossene Familien-Referenz-Cubes (D02/D03/D06 — dieselbe
  `Relation`-Kern-Naht-Regel-Form, verschiedene Nahtnamen/Domänen)
  werden durch den echten Motor gesiegelt, zu einem Kandidaten
  destilliert (RD-gebunden), durchs BridgeGate promoviert (Allow) und
  als `.loom`-Workbody der Containerklasse `"norm"` gesiegelt —
  Verdikt `Valid`, `CitationGate` grün (alle drei `derives`-Nähte
  auflösend über den `SeedResolver`).
- **R-NRM-2** — Aktivierung: ein Lauf mit `norm_profile` wendet die
  Norm als Gate an, Replay bleibt klassenidentisch.
- **R-NRM-3** — Widerruf: ein echter Widerrufs-Workbody, Status
  `revoked`, Alt-Lauf-Reanalyse zeigt `norm_since_revoked`.
- **R-NRM-4** — Erosion: ein invalid gewordenes Mitglied lässt die
  Norm automatisch auf `deprecated` fallen.
- **N-NRM-1..8** — κ < κ_min (Hold) · Kandidat ohne Promotion (Reject,
  = PROD-INV-21) · lockerndes Pattern (Reject, = PROD-INV-23) ·
  verschwiegene/unbestätigte Gegenbeispiele (Reject) · Norm-Konflikt
  (Hold) · Anwendung ohne Aktivierung (Reject, = PROD-INV-22) ·
  Destillation ohne vollständigen RD (verboten) · Replay-Mismatch der
  Destillation (Reject).

Alle 12 Zeugen + PROD-INV-21..23:
`conformance/tests/e5_l9b_normic_memory.rs`; Aktivierungs-Hook-Zeugen
zusätzlich in `crates/cce-runner/tests/g5_gate.rs`.
