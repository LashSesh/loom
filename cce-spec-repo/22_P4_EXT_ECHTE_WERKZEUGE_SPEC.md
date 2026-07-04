# 22 — P4-EXT: ECHTER VERGLEICH GEGEN CURSOR / GITHUB COPILOT / BOLT

**Der Schritt, der die ursprüngliche Behauptung wörtlich einlöst.** P4 bewies: die Beweispflicht kostet nichts gegenüber demselben ungegateten Modell. P4-Ext beweist: CCE hält stand gegen die tatsächlich benannten, am Markt befindlichen Werkzeuge. Normativer Overlay, direkter Anschluss an P4 und Dokument 21 (Agent-Grounding). Bauphase **P4-Ext**.

## §1 Design-Entscheidungen (autonom getroffen)

**Dieselben zwei Aufgabenklassen, jetzt gegen einen dritten Arm-Typ:** `ExternalToolResult` — ein Werkzeug, das CCE nicht kontrolliert und dessen internen Prozess CCE nicht beobachten kann. CCE zeichnet nur, was **beobachtbar** ist: Ausgangszustand (identisch, per `task_package_digest`), Endzustand (Diff/Datei), ob `success_criteria` erfüllt sind (**mechanisch geprüft**, wie in P4), Wanduhrzeit, Zahl der nötigen Nachbesserungen durch den Menschen. Kein Versuch, das Fremdwerkzeug zu zertifizieren oder sein Inneres zu bewerten — nur die Tatsachen.

**Der Mensch führt den Fremdarm aus, nicht CCE.** CCE kann Cursor/Copilot/Bolt nicht selbst bedienen. Der Auftraggeber (oder wer immer die Lizenz bedient) führt die Aufgabe im jeweiligen Werkzeug real aus und liefert das Ergebnis (Enddatei-Inhalt, Zeit, Eingriffszahl) an CCE zurück — strukturell wie eine aufgezeichnete HITL-Beobachtung (S5-A5-Muster): ein bestimmter, extern erzeugter, aber sauber typisierter Eingang.

**Fairness über Regeln, nicht nur Dateien (Dokument 21):** Beide Seiten erhalten **dasselbe GroundingPacket** — CCE nativ, das Fremdwerkzeug als abgeleitete, für dessen Format passende Exportdatei (`.cursorrules`-artig für Cursor, äquivalent für Copilot/Bolt je nach deren Konventionen). `packet_digest` wird Teil von `task_package_digest` — ein Regel-Unterschied zwischen den Armen ist damit strukturell ausgeschlossen, nicht nur behauptet.

## §2 Objektmodell-Erweiterung (über P4 hinaus, gleiches Crate `cce-benchmark`)

**ExternalToolResult:** `(package_id, tool_name, tool_version?, output_content, wall_time, human_interventions_count, criteria_met, evidence_present=false, observer_note)`. `tool_name` ist ein freies Textfeld (kein Enum) — CCE stellt keine Behauptung über das Werkzeug auf, außer dem, was beobachtet wurde. **ThreeArmComparisonMatrix:** die D1–D6-Zeilen jetzt für Raw / CCE / ExternalTool nebeneinander je Aufgabenklasse.

## §3 Protokoll (erweitert Dokument 20 §2)

Reihenfolge unverändert streng: Aufgabenpaket + GroundingPacket zuerst gehasht und eingefroren → **Fremdwerkzeug-Arm zuerst und isoliert** (der Mensch arbeitet ohne Kenntnis, was CCE später liefert) → Ergebnis versiegelt → **danach** CCE-Arm (kann nicht durch das Fremdergebnis beeinflusst sein, da es zeitlich vorher feststeht) → **danach** Matrixbau. `FairnessGate` erweitert: prüft zusätzlich `packet_digest`-Gleichheit zwischen allen Armen.

## §4 Zeugen

**R-BENCH-EXT-1** (Coding, gegen Cursor ODER Copilot ODER Bolt — Auftraggeber wählt): identisches `numkit`-artiges Bug-Fix-Paket + identisches GroundingPacket, drei Arme, `ThreeArmComparisonMatrix` vollständig, Benchmark-Workbody `verify == Valid`. **R-BENCH-EXT-2** (Dokument/Recherche, falls das Werkzeug das unterstützt — sonst als Residuum `task_class_unsupported_by_tool` sichtbar geführt, nicht erzwungen). **N-BENCH-EXT-1** unterschiedlicher `packet_digest` zwischen Armen ⇒ `FairnessGate` reject.

## §5 Betriebsverifikation (wie P1/P3/P4 — keine hermetische CI-Pflicht für den externen Arm)

`reports/P4_ext_bericht.md`: je Arm Aufgabentext, GroundingPacket-Export (was das Fremdwerkzeug tatsächlich erhielt), Ergebnis, Zeit, Eingriffe, ausgefüllte Drei-Arm-Matrix. Objektmodell/Gates (§2–§3) hermetisch testbar mit einem synthetischen `ExternalToolResult`-Fixture.

## §6 DoD

```
DoD(P4-Ext) = 1 ⟺
    §2–§5 implementiert ∧ Alt-Zeugen (inkl. P4, Dokument 21) unverändert
  ∧ mindestens EIN echter Lauf gegen mindestens EIN namentlich benanntes
        Werkzeug (Cursor, Copilot oder Bolt) real durchgeführt und mit
        identischem GroundingPacket-Digest belegt
  ∧ Drei-Arm-Matrix vollständig ∧ D4/D5 mindestens Parität ∧
        D1/D2/D3/D6 kategorisch (wie P4)
```
**DoD(P4-Ext)=1 ⟹ K9 vollständig im ursprünglichen, wörtlichen Sinn des Auftraggebers erfüllt** — nicht mehr nur gegen ein ungegatetes Modell, sondern gegen das tatsächlich benannte Vergleichsfeld.

**Voraussetzung, die nur der Auftraggeber schaffen kann:** Zugang zu mindestens einem der drei Werkzeuge (Konto/Lizenz) und die Bereitschaft, den Fremdarm einmal real selbst zu bedienen — CCE kann diesen Teil strukturell nicht automatisieren.
