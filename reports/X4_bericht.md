Etappe X4 — Paradigma-Vollzyklus-Kerntest (R-CYC-1) und Konsolidierung
(Dokument 16, Ökosystem-Expansionskarte)

Eingang: Etappe X3 (Ring E5, L9b Normic Memory, R-1b GESCHLOSSEN)
abgeschlossen und angenommen. Spec-Lieferung Dokument 16 gelesen und
vollständig umgesetzt: der eine Beweis, dass alle Organe ein Kreislauf
sind — Welt → Arbeit → Verbund → Selbstbezug → Gedächtnis →
Rückwirkung → sichtbare Erosion → Replay des Ganzen. Reihenfolge §2 →
§1 → §3, wie angeordnet, vier Commits.

## §2(a) — R-Agent-13 geschlossen: Blueprint→Pattern-Brücke

Neues Modul `cce_bridge::pattern`: `Pattern` ist jetzt eine typisierte
Repräsentation (`StructuralRule(DomainRuleForm)`/`SeamPattern`/
`ClosureProfile`/`VocabularyNorm`/`ProcessNorm`) statt freiem String.
`NormCandidate`/`BridgeNorm.pattern: String` → `pattern: Pattern`; die
`NexusClass` ist kein eigenes Feld mehr, sondern `pattern.class()` —
zwei Felder, die auseinanderlaufen könnten, sind strukturell
ausgeschlossen.

Neuer Extraktor `cce_bridge::extract::blueprint_to_pattern
(&BlueprintCandidate) -> Option<Pattern>`: liest echte
`cce-hbm`-Blueprint-Facetten (die reale `"invariant: {id}|regel=
{Name}|naht={seam}"`-Form, die `loom_conformance::domain_rule_fact`
bereits erzeugt) und leitet daraus NUR eine der sechs whitelisted
`DomainRuleForm`-Formen ab; nicht abbildbare Blueprints liefern `None`
statt eines geratenen Patterns (sichtbares Residuum
`pattern_extraction_unsupported` beim Aufrufer). `cce-bridge` hängt
jetzt zusätzlich von `cce-hbm` ab (S-E5 §10 nennt sie explizit als
optionale Pattern-Quelle; reine cce→cce-Kante, kein Zyklus, keine neue
`ALLOWED_CORE_TO_OUTER`-Ausnahme nötig).

`build_first_active_norm` (R-NRM-1, Ring E5) destilliert das Pattern
jetzt selbst aus einem echten kleinen HBM-Lauf
(`build_norm_source_blueprint`) über D02/D03/D06 statt es als Freitext
zu erfinden — schließt R-Agent-13 auch rückwirkend für den bestehenden
Meilenstein.

## §2(b) — R-Agent-14 geschlossen: ScopeGate v2

Die Fundament-Schutz-Whitelist (§4) ist jetzt eine KONSTRUKTIONS-,
keine Texteigenschaft: `StructuralRule`/`SeamPattern`/`VocabularyNorm`/
`ProcessNorm` können per Typ gar nicht erst etwas Lockerndes
ausdrücken. Einzige verbleibende, genuin gefährliche Form ist
`ClosureProfile` (Zusatz-Gates) — `gate.rs`s ScopeGate weist jede
`added_gate_id` zurück, die einen bestehenden Fundament-Gate-Namen
trägt (Kaperung statt echter Neuheit). N-NRM-3/PROD-INV-23 auf die
typisierte Form umgestellt (ein `ClosureProfile`, das einen
Fundament-Gate-Namen kapert), bleibt rot — für JEDES der real
existierenden Fundament-Gates einzeln geprüft.

Zusätzlich strukturell verschärft (nicht Teil des Auftrags, aber beim
Refactoring entdeckt): `BridgeGateReport`s Felder/Konstruktoren waren
bereits privat (PROD-INV-21 aus Ring E5) — diese Disziplin blieb beim
Pattern-Umbau erhalten.

## §1 — R-CYC-1: der Paradigma-Vollzyklus-Zeuge

Acht Stationen, real durchlaufen, in EINEM Zeugen
(`conformance/tests/x4_r_cyc_1.rs`, Builder in `loom-conformance`):

1. **Quelle** — das bestehende, CSA-getragene Welt-Crystal
   (`build_welt_kristall_wikimedia`).
2. **Arbeit** — ein NEUES, echtes "full"-Memo (LEDGER
   `closure_proof=true`, NICHT das bloß transportierbare
   `workcell`-Profil aus Ring E2) mit `supports`/`derives`-cites auf
   den Welt-Kristall (`build_cyc1_memo_workbody`); CitationGate grün
   über den echten `SeedResolver`.
3. **Verbund** — das Memo in einer geschlossenen SCALE-2-Mappe
   (`build_cyc1_folder_workbody`), die Mappe in einem SCALE-3-
   Projektraum (`build_cyc1_project`, `cce_materialize::scale3_
   project`); Red(3) geschlossen (MSC(1→2→3), dieselbe Spiral-
   Kinematik wie R-CIT-3, kein Code geändert); die deklarierte
   `cites`-Naht Mappe→Welt ist durch ein reales, gate-passendes Kind
   gedeckt (dieselbe Disziplin wie `e2_scale3_project.rs`).
4. **Selbstbezug** — ein echter, kleiner HBM-Lauf über den
   tatsächlichen Projektraum-Bestand (die reale Relation-Struktur des
   Memos: `"invariant: r-cyc-1-memo|regel=Relation|naht=supports"`),
   `build_cyc1_source_blueprint`; Replay klassenidentisch (zwei
   unabhängige Läufe, gleiche zertifizierte Klasse).
5. **Gedächtnis** — `blueprint_to_pattern` liefert das Pattern;
   ProvenanceSet = Memo + Mappe (Station 2–3) + eine bestehende
   Familien-Referenz-Cube (D02, bis κ_min=3); BridgeGate ALLOW; aktive
   Norm mit `derives`-cites, CitationGate grün über alle drei
   Mitglieder.
6. **Rückwirkung** — ein Lauf mit `norm_profile=[norm_id]` schließt
   einen weiteren Workbody unter der Norm; der RD listet sie
   (`RunDescriptor::norm_profile`); zwei unabhängige Läufe sind
   klassenidentisch.
7. **Erosionsprobe (zerstörungsfrei)** — ein Resolver OHNE ein
   Herkunfts-Mitglied (Test-Kopie, kein Datei-/Registry-Zugriff) lässt
   die Norm auf `deprecated` fallen (`check_erosion`); das Original
   bleibt in-memory unverändert `Active` — keine Produktiv-Registry
   wurde je berührt.
8. **Replay des Ganzen** — ein zweiter, unabhängiger Gesamtaufbau
   (`build_r_cyc_1()` erneut) ist an JEDER Station klassenidentisch:
   Welt-Wurzel, Memo-`core_root`, Mappen-`core_root`,
   Projekt-Äquivalenz, HBM-Blueprint-Klasse, `norm_id`,
   Norm-Workbody-`core_root`.

`loom_codec::Sealed` trägt jetzt additiv `#[derive(Debug, Clone)]`
(nötig, um denselben versiegelten Körper sowohl als eigenständiges
Stations-Ergebnis als auch als ProvenanceSet-Mitglied zu führen — reine
Ableitung, keine Verhaltensänderung).

`conformance::GUARD_PHASES += "X4"`, `FEATURE_PL += r_cyc_1_vollzyklus`
(PL2).

## §3 — Konsolidierung der Buchführung

**(c) Register-Gesamtstand:** `reports/residuen.md` konsolidiert jede
jemals vergebene Nummer (R-Agent-1..14, Register 07 #1–25, LC-R1/2/3/5,
CSA-R3, IG-R1/3/4, S15-R1, sowie R-1b/R-6/R-7/R-8/R-9/R-10/R-13/R-16/
Repo-R1) mit Endstatus GESCHLOSSEN/TEILWEISE GESCHLOSSEN/OFFEN+Pfad —
keine verwaiste Nummer. Acht Zeilen im (unangetasteten) Register 07 des
Auftraggebers sind seit dessen letztem Stand (vor Block 1–3/X1–X4)
inzwischen überholt — explizit vermerkt, ohne die Datei selbst zu
editieren (`spec/`-Disziplin, R-Agent-1).

**(d) VOLLAUSBAU_STATUS final:** auf eine echte Einseiten-Fassung
verdichtet — Ringe E1–E5 + X4/R-CYC-1 kompakt mit Bericht-Verweisen,
Host-Leiste, PL4-Reifepfade, Ausblick auf Dokument 17.

**(e) Constitution-K-Stände:** `reports/CONSTITUTION_STAND_X4.md` (neu)
— K1/K2 erfüllt, K3=UX-3+ (host-gebunden), K4 teilerfüllt, K5=D01-PL4+
212×PL3, K6=SCALE-1..3, K7=Kern erfüllt (CE-1, SDK, wasm, Registry),
K8 teilerfüllt (Fuzz+Threat-Model geliefert, externes Review offen) —
genau wie Dokument 16 §3(e) vorgegeben.

## Ausgangs-Gate X4 — Prüfung gegen §4

**R-CYC-1 grün:** ja, real, alle acht Stationen in einem Testlauf
(`conformance/tests/x4_r_cyc_1.rs::r_cyc_1_the_full_paradigm_cycle_
closes`).

**R-Agent-13/14 geschlossen (Zeugen umgestellt, N-NRM-3 rot):** ja —
typisierte `Pattern`-Form durchgängig, ScopeGate v2 aktiv,
`n_nrm_3_and_prod_inv_23_scope_loosening_patterns_are_rejected` prüft
JEDES Fundament-Gate einzeln und bleibt rot.

**Alt-Zeugen unverändert:** voller Workspace-Testlauf nach jeder
Einheit und final: 175 Testgruppen, 0 Fehlschläge (vorher 174 nach
Ring E5 + `x4_r_cyc_1` neu). Alle 213 Domänen-Referenz-Zeugen,
R1–R8-Golden-Files, alle X1/E2/E3/E4/E5-Zeugen unangetastet
(insbesondere `e5_l9b_normic_memory.rs`, 15/15 weiterhin grün trotz
des Pattern-Typumbaus).

**CI GRUEN:** `cargo fmt --all -- --check`, `cargo clippy --workspace
--all-targets -- -D warnings`, `python3 ci/check_acyclic.py` (55
Workspace-Crates, DAG sauber), `bash ci/run_ci.sh` — alle grün.

**(c)–(e) committet:** ja (`reports/residuen.md`,
`reports/VOLLAUSBAU_STATUS.md`, `reports/CONSTITUTION_STAND_X4.md`).

**Ausgangs-Gate X4: ERFÜLLT.**

## Residuen

Keine neuen. R-Agent-13/14 sind geschlossen (s. §2 oben); alle übrigen
Residuen bleiben wie im konsolidierten Register geführt (`reports/
residuen.md`) — keines blockiert den Bau-DoD.

## Nicht begonnen / weiterhin gesperrt

Host-Leiste (GGUF/LLM, OS-Keyring-Live, macOS/Windows-Pakete,
GPU-Klickpfad) bleibt gesperrt — durch P1 (unmittelbar folgend) wird
der GGUF-Teil strukturell teilaufgelöst (ein echter CloudModelProvider
löst den Bedarf an einem lokalen LLM-Host für "Frontier-Intelligenz").
`spec/`/`cce-spec-repo/` unangetastet.

**Damit ist die technologische Expansionsstufe (Dokument 16 §4)
vollständig.** Auftrag X4 abgeschlossen; direkter Anschluss an P1 (kein
neuer Prompt nötig, wie angeordnet).

Abweichungen: keine.
