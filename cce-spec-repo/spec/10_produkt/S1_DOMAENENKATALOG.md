# CCE — DOMÄNEN-KATALOG (Erweiterung von S1)

**Der vollständige Domänen-Katalog.** 213 Leaf-Domänen in 16 Familien (A–P), jede mit ihrem Differentiator und ihrem Produkt-Level — vollständig, mit **sichtbar geführter Unfertigkeit** statt kaschierender Auslassung.

**Status:** Katalog v1.0, Erweiterung von S1. Baut auf: S1 (DomainAdapter-Vorlage S1.8, Dokument als L4-Referenz), Systemlandkarte §5 (Adapter-Parität), S8 (Bibliothek), S5/S13 (HITL/Governance — Grundlage des ProfessionalReviewGate).

---

## K.0 — Einordnung & Leitsatz

S1 hat die Dokument-Domäne vollständig instanziiert **und** die kanonische Teileliste (`DomainAdapter`, 11 Punkte) als Vorlage definiert. Dieser Katalog **erweitert S1 auf alle Domänen**: Er benennt jede Leaf-Domäne, gibt ihren Differentiator, und führt ihren Reifegrad als sichtbares Residuum.

Leitsatz:

> *Der Katalog ist vollständig, wenn jede Domäne benannt, differenziert und mit einem Produkt-Level versehen ist. Dass eine Domäne noch nicht L4-produktreif ist, ist kein Loch — es ist ein sichtbarer, geführter Reifegrad. Unfertigkeit wird strukturiert ausgewiesen, nie durch Weglassen kaschiert.*

Zwei Grundfesten:

- **Adapter-Parität trägt die Nicht-Redundanz.** Der Adapter-Kontrakt (die 8 Methoden `wish_schema … equivalent` + Doku-Slot + Bibliotheks-Slot + Test-Cubes) ist **für alle Domänen identisch** (S1.8). Er wird **einmal** festgehalten (§K.1) und **je Familie** spezialisiert (§K.4); jede Leaf-Domäne trägt nur ihren echten Differentiator.
- **Produkt-Level = geführte Unfertigkeit.** Jede Domäne trägt ein Level L0–L4 (§K.2). Der Katalog bringt jede Domäne auf mindestens **L1**; das Level sagt exakt, was noch fehlt — das ist der Vollständigkeits-Mechanismus.

---

## K.1 — Der invariante Adapter-Kontrakt (einmal, für alle)

**Jede** der 213 Domänen implementiert **exakt** den `DomainAdapter`-Vertrag aus S1.8 — dieselben 11 Punkte, dieselbe Disziplin:

1. `wish_schema` / 2. `validate_wish` — Wunsch-Grammatik + deterministische Validierung.
3. `to_canonical` — Instanziierung auf die abstrakten Motor-Objekte.
4. `encode` — Crystal → PHC-Paket (Domänen-Profil).
5. `loom` — Projektion → Gewebe (radiale Spindel, Nullanker markiert).
6. `materialize` — Gewebe → Artefakt (nur Kosmetik, kein neuer Inhalt).
7. `reanalyze` — Artefakt → Crystal' (Collect-Pfad, verlustfrei).
8. `canonicalize` / `equivalent` — das ≃ der Domäne (kanonische Inhaltsklasse, nie Byte).
9. `domain_gates` — auf G1–G7 aufgesetzt, boolesch, fail-closed, begründet, kein Score.
10. `residue_vocabulary` / `counter_horizon` — Domänen-Residuen + Gegenhorizont.
11. `reference_cube` / `negative_cubes` / native Ausgabe / Doku-Slot / Bibliotheks-Slot.

**Was pro Domäne variiert**, ist ausschließlich: der **Crystal-Typ** (was Unit/Seam sind), die **Artefakt-Typen**, die **Domänen-Gates**, das **Residuen-Vokabular/Counter-Horizon** und die **native Verwendung**. Genau diese Differentiatoren stehen in §K.4. Die 8 Methoden-*Signaturen* sind invariant; ihre familienspezifische Ausprägung steht im Familien-Profil (§K.4, je Familienkopf).

`check_adapter_parity` (Systemlandkarte §5) prüft maschinell, dass jede Domäne alle 11 Punkte vollständig hat.

---

## K.2 — Die Produkt-Level-Leiter L0–L4 (der Vollständigkeits-Mechanismus)

Jede Domäne trägt genau ein Level. Das Level ist die **strukturierte, sichtbare Form der Unfertigkeit**:

| Level | Bedeutung | Was vorhanden ist |
|-------|-----------|-------------------|
| **L0** | katalogisiert | ID + Zweck benannt |
| **L1** | differenziert | Crystal-Typ, Artefakt, Domänen-Gates, Residuen, Counter-Horizon spezifiziert |
| **L2** | Adapter instanziiert | die 8 Methoden konkret für die Domäne (über das Familien-Profil hinaus) |
| **L3** | test-gedeckt | Referenz-Cube + Negativ-Cubes + Bibliotheks-Slot vorhanden, CI-gewacht (S8.3) |
| **L4** | produktreif | Doku-Slot vollständig, im Cockpit bedienbar, `ProduktDoD(Domäne)=1` |

**Regel für Slots (statt je-Domäne-Spalte):** Referenz-Cube, Negativ-Cubes, Doku-Slot und Bibliotheks-Slot sind bei **L3+** vorhanden; bei **L0–L2** sind sie **zu autorisieren** — und genau das ist der sichtbare, geführte Rest.

**Ausgangszustand nach diesem Katalog:** Dokument-Bericht (D01) = **L4** (die S1-Referenz). **Alle übrigen 212 Domänen = L1** (hier differenziert). Die Menge der Domänen unter L4 **ist** das sichtbare Gesamt-Residuum des Katalogs (§K.7) — vollständig ausgewiesen, nicht kaschiert.

---

## K.3 — Das ProfessionalReviewGate (Familie P, hart)

Familie P (Regulated Advisory) trägt ein zusätzliches, **hartes** Gate, das die Governance (S13) und HITL (S5) für haftungs-/regulierungsrelevante Domänen verschärft:

- **Was:** Für regulierte Beratungs-Domänen (Recht, Medizin, Finanzen, Steuer, Ingenieur-Abnahme u. a.) gibt die CCE **niemals** ein zertifiziertes Artefakt allein aus. Ein **lizenzierter Fachmensch** muss das Artefakt prüfen und freigeben.
- **Hart, nicht übersteuerbar (S5.4/S13.3):** Weder der Operator noch die KI-Kanzel kann das ProfessionalReviewGate umgehen. Ohne aufgezeichnete Freigabe einer qualifizierten Person erreicht das Artefakt **nicht** `certified/closed` — es bleibt „Entwurf, nicht fachgeprüft".
- **Aufgezeichnet (S6.4):** Die Freigabe (wer, Qualifikation, Zeitpunkt) steht im Ledger.
- **Claim-Schranke verschärft (S13.4):** Der Entwurf trägt bis zur Freigabe den sichtbaren Hinweis, dass er **keine** fachliche Beratung ersetzt.

Das ProfessionalReviewGate macht den Fachmenschen-in-the-Loop zur **baulichen Pflicht** für diese Domänen — konsistent mit „die Maschine assistiert, der qualifizierte Mensch verantwortet".

---

## K.4 — Der Leaf-Domänen-Katalog (213 Domänen)

Je Familie: ein **Familien-Profil** (wie die 8 Methoden spezialisieren) und die **Leaf-Tabelle** (ID · Zweck | Crystal Unit·Seam | Artefakt | Kern-Gate | Kern-Residuum/Counter-Horizon | Level).

---

### A — Dokument/Text (D01–D15)

**Familien-Profil:** Unit = semantische Text-Einheit; Seam = Stütz-/Enthaltens-/Verweis-Beziehung. `materialize` → `.docx`/`.md`/`.pdf`; `reanalyze` liest Struktur+Einheiten zurück; `equivalent` = gleiche Einheiten/Naht-Graph/Abdeckung (S1.6). Das ist das S1-Referenzprofil.

| ID · Zweck | Crystal (Unit · Seam) | Artefakt | Kern-Gate | Kern-Residuum / Counter-Horizon | Level |
|---|---|---|---|---|---|
| **D01** Bericht/Memo | Aussage · Stütz-Naht | .docx/.md | Support, Coverage | unsupported_unit; unabgedeckt | **L4** |
| **D02** Vertrag/Contract | Klausel · Verweis-Naht | .docx/.pdf | Verweis-Konsistenz | dangling_clause; Widerspruch | L1 |
| **D03** Spezifikation/Requirements | Anforderung · Ableit-Naht | .md/.docx | Testbarkeit, Traceability | untestable_req; verwaist | L1 |
| **D04** Handbuch/Manual | Schritt · Reihenfolge-Naht | .md/.pdf | Reihenfolge-Vollständigkeit | gap_in_procedure | L1 |
| **D05** Brief/Korrespondenz | Absatz · Bezug-Naht | .docx | Anrede/Schluss-Vollständigkeit | missing_salutation | L1 |
| **D06** Angebot/Proposal | Position · Preis-Naht | .docx/.pdf | Positions-Deckung | orphan_line_item | L1 |
| **D07** Richtlinie/Policy | Regel · Geltungs-Naht | .md/.docx | Geltungsbereich definiert | unscoped_rule; Konflikt | L1 |
| **D08** Protokoll/Minutes | Punkt · Beschluss-Naht | .md | Beschluss-Zuordnung | actionless_item | L1 |
| **D09** Zusammenfassung/Abstract | Kernaussage · Quell-Naht | .md | Treue zur Quelle | invented_semantic; Auslassung | L1 |
| **D10** Pressemitteilung | Botschaft · Beleg-Naht | .docx | Fakten-Beleg | unsupported_claim | L1 |
| **D11** Lebenslauf/CV | Station · Zeit-Naht | .docx/.pdf | Chronologie-Konsistenz | timeline_gap | L1 |
| **D12** Whitepaper | These · Argument-Naht | .pdf/.md | Argument-Kette | broken_argument | L1 |
| **D13** FAQ | Frage · Antwort-Naht | .md | Frage-Antwort-Paarung | unanswered_question | L1 |
| **D14** Checkliste | Prüfpunkt · Ordnungs-Naht | .md | Eindeutigkeit je Punkt | ambiguous_item | L1 |
| **D15** Glossar | Begriff · Definitions-Naht | .md | Definitions-Vollständigkeit | undefined_term; Zirkel | L1 |

---

### B — Software Engineering (SWE01–SWE15)

**Familien-Profil:** Unit = Code-/Spec-Einheit (Funktion, Endpoint, Testfall); Seam = Aufruf-/Abhängigkeits-/Vertrags-Beziehung. `materialize` → Quell-/Konfig-Dateien; `reanalyze` parst zurück zu AST-/Struktur-Crystal; `equivalent` = gleiche semantische/verhaltensbezogene Klasse (nicht Formatierung).

| ID · Zweck | Crystal (Unit · Seam) | Artefakt | Kern-Gate | Kern-Residuum / Counter-Horizon | Level |
|---|---|---|---|---|---|
| **SWE01** Modul/Funktion | Funktion · Aufruf-Naht | Quellcode | Signatur-Konsistenz, Testbezug | untested_path; toter Code | L1 |
| **SWE02** API-Spezifikation | Endpoint · Schema-Naht | OpenAPI | Schema-Vollständigkeit | undefined_response | L1 |
| **SWE03** Test-Suite | Testfall · Abdeckungs-Naht | Testdateien | Abdeckung je Anforderung | uncovered_requirement | L1 |
| **SWE04** DB-Schema/Migration | Tabelle · FK-Naht | SQL/DDL | Referenz-Integrität | orphan_fk; Zyklus | L1 |
| **SWE05** CI/CD-Pipeline | Stufe · Abhängigkeits-Naht | YAML | Stufen-Reihenfolge, Fail-closed | missing_gate | L1 |
| **SWE06** Refactoring-Plan | Schritt · Erhalt-Naht | .md/Patch | Verhaltens-Erhalt | behavior_change | L1 |
| **SWE07** Architektur/ADR | Komponente · Schnittstellen-Naht | .md/Diagramm | Azyklizität | dependency_cycle | L1 |
| **SWE08** Bugfix/Patch | Änderung · Ursache-Naht | Diff | Regressions-Freiheit | regression; Ursache offen | L1 |
| **SWE09** Konfigurationsdatei | Schlüssel · Geltungs-Naht | Config | Vollständigkeit/Typ | missing_key; Konflikt | L1 |
| **SWE10** CLI-Tool | Kommando · Flag-Naht | Quellcode | Hilfe/Fehler-Vollständigkeit | undocumented_flag | L1 |
| **SWE11** Library/Package | Public-API · Vertrags-Naht | Paket | API-Stabilität, SemVer | breaking_change | L1 |
| **SWE12** Code-Review-Bericht | Befund · Bezug-Naht | .md | Befund-Belegbarkeit | unlocated_finding | L1 |
| **SWE13** SBOM/Dependency-Manifest | Abhängigkeit · Lizenz-Naht | SBOM | Lizenz-/CVE-Vollständigkeit | unknown_license | L1 |
| **SWE14** Infrastructure-as-Code | Ressource · Bezug-Naht | Terraform | Idempotenz, Drift-Freiheit | drift; orphan_resource | L1 |
| **SWE15** Regex/Grammar/Parser | Regel · Ableit-Naht | Grammatik | Eindeutigkeit/Terminierung | ambiguous_grammar | L1 |

---

### C — Daten/Analytics (DATA01–DATA12)

**Familien-Profil:** Unit = Datenfeld/Transformation/Metrik; Seam = Ableit-/Join-/Lineage-Beziehung. `materialize` → Query/Pipeline/Schema/Chart-Spec; `reanalyze` liest Lineage/Schema zurück; `equivalent` = gleiche Datenklasse/Transformationssemantik.

| ID · Zweck | Crystal (Unit · Seam) | Artefakt | Kern-Gate | Kern-Residuum / Counter-Horizon | Level |
|---|---|---|---|---|---|
| **DATA01** Datenbereinigung | Feld · Regel-Naht | Cleaning-Spec | Regel-Vollständigkeit | unhandled_null; stiller Verlust | L1 |
| **DATA02** SQL-Query | Klausel · Join-Naht | SQL | Join-Korrektheit | cartesian_blowup | L1 |
| **DATA03** ETL-Pipeline | Stufe · Lineage-Naht | Pipeline | Lineage-Lückenlosigkeit | broken_lineage | L1 |
| **DATA04** Dashboard-Spec | Widget · Metrik-Naht | Spec | Metrik-Herkunft | undefined_metric | L1 |
| **DATA05** Datenmodell | Entität · Beziehungs-Naht | Schema | Normalisierung/Integrität | orphan_entity | L1 |
| **DATA06** Statistische Analyse | Test · Annahme-Naht | Analyse-Spec | Annahmen-Prüfung | violated_assumption; p-hacking | L1 |
| **DATA07** Feature-Engineering | Feature · Quell-Naht | Spec | Leckage-Freiheit | data_leakage | L1 |
| **DATA08** Datenqualitäts-Regelwerk | Regel · Feld-Naht | Regelwerk | Deckung je Feld | unchecked_field | L1 |
| **DATA09** Visualisierung/Chart | Kanal · Daten-Naht | Chart-Spec | Kanal-Daten-Treue | misleading_scale | L1 |
| **DATA10** KPI-Definition | KPI · Formel-Naht | Definition | Berechenbarkeit/Eindeutig | ambiguous_kpi | L1 |
| **DATA11** Datenkatalog/Metadaten | Asset · Herkunfts-Naht | Katalog | Herkunfts-Vollständigkeit | undocumented_source | L1 |
| **DATA12** Sampling/Kohorte | Kriterium · Zugehörigkeits-Naht | Definition | Repräsentativität | selection_bias | L1 |

---

### D — Graph/Netzwerk/Struktur (GRA01–GRA12)

**Familien-Profil:** Unit = Knoten/Zustand; Seam = Kante/Übergang. `materialize` → Graph-Datei + Rendering; `reanalyze` liest Knoten/Kanten zurück; `equivalent` = Graph-Isomorphie der kanonischen Struktur.

| ID · Zweck | Crystal (Unit · Seam) | Artefakt | Kern-Gate | Kern-Residuum / Counter-Horizon | Level |
|---|---|---|---|---|---|
| **GRA01** Graph-Konstruktion | Knoten · Kante | Graph-Datei | Wohlgeformtheit | dangling_edge | L1 |
| **GRA02** Ontologie/Taxonomie | Begriff · is-a-Naht | OWL/Graph | Azyklizität (is-a) | cycle_in_hierarchy | L1 |
| **GRA03** Zustandsautomat (FSM) | Zustand · Übergang | FSM-Spec | Vollständigkeit/Determinismus | unreachable_state | L1 |
| **GRA04** Abhängigkeitsgraph | Einheit · Dependency | Graph | Azyklizität | dependency_cycle | L1 |
| **GRA05** ER-Modell | Entität · Relation | ER-Diagramm | Kardinalitäts-Konsistenz | undefined_cardinality | L1 |
| **GRA06** Workflow/DAG | Schritt · Fluss-Naht | DAG | Azyklizität, Erreichbarkeit | orphan_step | L1 |
| **GRA07** Mindmap/Konzeptnetz | Konzept · Assoziation | Mindmap | Zusammenhang | isolated_node | L1 |
| **GRA08** Netzwerk-Topologie | Knoten · Verbindung | Topologie | Konnektivität/Redundanz | single_point_of_failure | L1 |
| **GRA09** Baum/Hierarchie | Knoten · Eltern-Naht | Baum | Baum-Eigenschaft | multiple_parents | L1 |
| **GRA10** Petri-Netz | Stelle/Transition · Kante | Petri-Spec | Beschränktheit/Lebendigkeit | deadlock | L1 |
| **GRA11** Kausaldiagramm | Faktor · Kausal-Naht | DAG | Azyklizität, Konfounder | unmeasured_confounder | L1 |
| **GRA12** Routing/Pfad | Knoten · Kanten-Gewicht | Routing-Spec | Pfad-Existenz/Optimalität | unreachable_target | L1 |

---

### E — Mathematik/Formale Struktur (MATH01–MATH15)

**Familien-Profil:** Unit = Aussage/Definition/Schritt; Seam = Ableit-/Abhängigkeits-Beziehung. `materialize` → Beweis-/Struktur-Artefakt; `reanalyze` liest die Ableitungskette zurück; `equivalent` = gleiche logische Struktur/Ableitungsklasse. Gegenhorizont besonders wichtig (Nullmodelle, Gegenbeispiele).

| ID · Zweck | Crystal (Unit · Seam) | Artefakt | Kern-Gate | Kern-Residuum / Counter-Horizon | Level |
|---|---|---|---|---|---|
| **MATH01** Beweis | Schritt · Ableit-Naht | Beweis-Doc | Lückenlosigkeit | gap_in_proof; Gegenbeispiel | L1 |
| **MATH02** Gleichungssystem | Gleichung · Kopplungs-Naht | Modell | Lösbarkeit/Konsistenz | inconsistent_system | L1 |
| **MATH03** Optimierungsmodell | Variable · Constraint-Naht | Modell | Zulässigkeit/Beschränktheit | infeasible; unbounded | L1 |
| **MATH04** Algorithmus-Spec | Schritt · Kontrollfluss-Naht | Pseudocode | Terminierung/Korrektheit | nontermination | L1 |
| **MATH05** Formale Definition | Term · Abhängigkeits-Naht | Definition | Wohldefiniertheit | circular_definition | L1 |
| **MATH06** Theorem + Lemma-Kette | Satz · Nutzungs-Naht | Doc | Lemma-Abdeckung | unused_hypothesis | L1 |
| **MATH07** Kombinatorische Struktur | Objekt · Inzidenz-Naht | Struktur | Konsistenz der Zählung | double_counting | L1 |
| **MATH08** Wahrscheinlichkeitsmodell | Ereignis · Abhängigkeits-Naht | Modell | Normierung (Σ=1) | unnormalized | L1 |
| **MATH09** Logische Formel/SAT | Klausel · Variablen-Naht | CNF | Erfüllbarkeit/Konsistenz | contradiction | L1 |
| **MATH10** Typsystem/Kalkül | Regel · Prämissen-Naht | Kalkül | Subject Reduction/Progress | broken_preservation | L1 |
| **MATH11** Kategorielle Konstruktion | Objekt/Morphismus · Komposition | Diagramm | Kommutativität | noncommuting_diagram | L1 |
| **MATH12** Differentialgleichung | Term · Rand-Naht | Modell | Randbedingungs-Vollständig | ill_posed | L1 |
| **MATH13** Gruppentheorie-Struktur | Element · Operations-Naht | Struktur | Abgeschlossenheit/Axiome | not_closed | L1 |
| **MATH14** Numerisches Verfahren | Schritt · Fehler-Naht | Verfahren | Stabilität/Konvergenz | divergence | L1 |
| **MATH15** Beweis-Skizze | Kernidee · Lücken-Naht | Skizze | Lücken explizit markiert | hidden_gap | L1 |

---

### F — Projekt/Prozess/Workflow (PM01–PM15)

**Familien-Profil:** Unit = Aufgabe/Ereignis/Rolle; Seam = Abhängigkeits-/Zuordnungs-Beziehung. `materialize` → Plan/Diagramm/Register; `reanalyze` liest Abhängigkeiten zurück; `equivalent` = gleiche Ablauf-/Zuordnungsstruktur.

| ID · Zweck | Crystal (Unit · Seam) | Artefakt | Kern-Gate | Kern-Residuum / Counter-Horizon | Level |
|---|---|---|---|---|---|
| **PM01** Projektplan/Gantt | Aufgabe · Abhängigkeit | Gantt | Azyklizität/kritischer Pfad | dependency_cycle | L1 |
| **PM02** Prozess-Definition | Schritt · Fluss-Naht | BPMN | Erreichbarkeit/Ende | no_end_event | L1 |
| **PM03** Risikoregister | Risiko · Maßnahme-Naht | Register | Maßnahme je Risiko | unmitigated_risk | L1 |
| **PM04** Meilenstein-Plan | Meilenstein · Abhängigkeit | Plan | Erreichbarkeit | orphan_milestone | L1 |
| **PM05** Sprint-Backlog | Story · Abhängigkeit | Backlog | Schätzbarkeit/DoD | unestimated_story | L1 |
| **PM06** RACI-Matrix | Aufgabe · Rollen-Naht | Matrix | genau ein A je Aufgabe | missing_accountable | L1 |
| **PM07** Ressourcenplan | Ressource · Zuweisungs-Naht | Plan | Kapazitäts-Deckung | overallocation | L1 |
| **PM08** Statusbericht | Kennzahl · Bezug-Naht | Bericht | Bezug zu Plan | unbacked_status | L1 |
| **PM09** Retrospektive | Befund · Aktion-Naht | Doc | Aktion je Befund | actionless_finding | L1 |
| **PM10** Entscheidungsvorlage | Option · Kriterium-Naht | Vorlage | Kriterien-Deckung (kein Score-Gate) | undecided_criterion | L1 |
| **PM11** Change-Request | Änderung · Impact-Naht | CR | Impact-Vollständigkeit | unassessed_impact | L1 |
| **PM12** Abnahmekriterien/DoD | Kriterium · Prüf-Naht | DoD | Prüfbarkeit | untestable_criterion | L1 |
| **PM13** Eskalationspfad | Stufe · Übergabe-Naht | Pfad | Lückenlosigkeit | dead_end_escalation | L1 |
| **PM14** Kapazitätsplan | Einheit · Bedarf-Naht | Plan | Bedarf-Deckung | capacity_gap | L1 |
| **PM15** Roadmap | Initiative · Abhängigkeit | Roadmap | Reihenfolge-Konsistenz | orphan_initiative | L1 |

---

### G — Governance/Compliance/Audit (GOV01–GOV12)

**Familien-Profil:** Unit = Kontrolle/Anforderung/Nachweis; Seam = Mapping-/Beleg-Beziehung. `materialize` → Checkliste/Bericht/Matrix; `reanalyze` liest Mappings/Belege zurück; `equivalent` = gleiche Kontroll-Abdeckungsstruktur. Kein Score-als-Gate (V1) hier besonders kritisch.

| ID · Zweck | Crystal (Unit · Seam) | Artefakt | Kern-Gate | Kern-Residuum / Counter-Horizon | Level |
|---|---|---|---|---|---|
| **GOV01** Compliance-Checkliste | Anforderung · Nachweis-Naht | Checkliste | Nachweis je Anforderung | unevidenced_requirement | L1 |
| **GOV02** Audit-Bericht | Feststellung · Beleg-Naht | Bericht | Belegbarkeit | unbacked_finding | L1 |
| **GOV03** Kontroll-Matrix | Kontrolle · Risiko-Naht | Matrix | Risiko-Abdeckung | uncovered_risk | L1 |
| **GOV04** Control-Mapping | Anforderung · Kontroll-Naht | Mapping | Vollständigkeit des Mappings | unmapped_control | L1 |
| **GOV05** Risikobewertung | Risiko · Bewertungs-Naht | Bewertung | Methoden-Konsistenz (kein Score-Gate) | inconsistent_rating | L1 |
| **GOV06** DPIA | Verarbeitung · Schutz-Naht | DPIA | Verhältnismäßigkeit/Schutz | unmitigated_processing | L1 |
| **GOV07** Evidence-Paket | Nachweis · Anforderungs-Naht | Paket | Rückverfolgbarkeit | orphan_evidence | L1 |
| **GOV08** Gap-Analyse | Soll · Ist-Naht | Analyse | Soll-Ist-Vollständigkeit | unaddressed_gap | L1 |
| **GOV09** Attestierung | Aussage · Beleg-Naht | Attest | Belegbarkeit je Aussage | unsupported_attestation | L1 |
| **GOV10** Regelwerk-Konformität | Regel · Umsetzungs-Naht | Bericht | Umsetzung je Regel | unimplemented_rule | L1 |
| **GOV11** Incident-Report (Compliance) | Ereignis · Ursache-Naht | Report | Ursache/Maßnahme | rootless_incident | L1 |
| **GOV12** Zertifizierungs-Vorbereitung | Kriterium · Nachweis-Naht | Dossier | Kriterien-Deckung | uncovered_criterion | L1 |

---

### H — Security/Infra/Ops (OPS01–OPS15)

**Familien-Profil:** Unit = Asset/Bedrohung/Kontrolle/Schritt; Seam = Abwehr-/Abhängigkeits-Beziehung. `materialize` → Plan/Runbook/Regelwerk; `reanalyze` liest Abdeckung/Abhängigkeiten zurück; `equivalent` = gleiche Abwehr-/Ablaufstruktur. Fail-closed (V7) besonders kritisch.

| ID · Zweck | Crystal (Unit · Seam) | Artefakt | Kern-Gate | Kern-Residuum / Counter-Horizon | Level |
|---|---|---|---|---|---|
| **OPS01** Threat-Model | Bedrohung · Abwehr-Naht | Modell | Abwehr je Bedrohung | unmitigated_threat | L1 |
| **OPS02** Runbook/Playbook | Schritt · Bedingungs-Naht | Runbook | Lückenlosigkeit/Fail-closed | undefined_branch | L1 |
| **OPS03** Incident-Response-Plan | Phase · Übergabe-Naht | Plan | Phasen-Vollständigkeit | missing_containment | L1 |
| **OPS04** Härtungs-Checkliste | Maßnahme · Asset-Naht | Checkliste | Deckung je Asset | unhardened_asset | L1 |
| **OPS05** Netzwerk-Segmentierung | Zone · Grenz-Naht | Spec | Isolation/kein Boundary ohne Naht | flat_network | L1 |
| **OPS06** Zugriffskontrolle (RBAC) | Rolle · Recht-Naht | Matrix | Least-Privilege | excess_privilege | L1 |
| **OPS07** Monitoring/Alert-Regeln | Signal · Schwellen-Naht | Regelwerk | Deckung/kein Score-Gate | blind_spot | L1 |
| **OPS08** Disaster-Recovery-Plan | Schritt · RPO/RTO-Naht | Plan | RPO/RTO-Erfüllung | untested_recovery | L1 |
| **OPS09** Patch-Plan | Patch · Abhängigkeits-Naht | Plan | Deckung/Reihenfolge | unpatched_cve | L1 |
| **OPS10** Vulnerability-Bericht | Schwachstelle · Fix-Naht | Bericht | Fix/Belegbarkeit | unremediated_vuln | L1 |
| **OPS11** SLO/SLA-Definition | Ziel · Messungs-Naht | Definition | Messbarkeit | unmeasurable_slo | L1 |
| **OPS12** Skalierungsplan | Ressource · Last-Naht | Plan | Last-Deckung | scaling_gap | L1 |
| **OPS13** Backup-Strategie | Datensatz · Aufbewahrungs-Naht | Strategie | Deckung/Wiederherstellbar | unbacked_data | L1 |
| **OPS14** Secrets/Key-Management | Secret · Rotations-Naht | Plan | Rotation/kein Klartext | plaintext_secret | L1 |
| **OPS15** Deployment-Plan | Schritt · Rollback-Naht | Plan | Rollback-Fähigkeit | no_rollback | L1 |

---

### I — Produkt/Business/Markt (BUS01–BUS15)

**Familien-Profil:** Unit = Annahme/Segment/Ziel/Position; Seam = Begründungs-/Abhängigkeits-Beziehung. `materialize` → Case/Analyse/Plan/Canvas; `reanalyze` liest Begründungsstruktur zurück; `equivalent` = gleiche Argument-/Modellstruktur.

| ID · Zweck | Crystal (Unit · Seam) | Artefakt | Kern-Gate | Kern-Residuum / Counter-Horizon | Level |
|---|---|---|---|---|---|
| **BUS01** Business-Case | Annahme · Begründungs-Naht | Case | Belegte Annahmen | unbacked_assumption | L1 |
| **BUS02** Marktanalyse | Segment · Evidenz-Naht | Analyse | Evidenz je Aussage | unsourced_claim | L1 |
| **BUS03** Wettbewerbsanalyse | Wettbewerber · Vergleichs-Naht | Analyse | Vergleichbarkeit | apples_oranges | L1 |
| **BUS04** Produktanforderungen (PRD) | Anforderung · Nutzen-Naht | PRD | Nutzen/Testbarkeit | unjustified_feature | L1 |
| **BUS05** Go-to-Market-Plan | Maßnahme · Kanal-Naht | Plan | Kanal-Deckung | untargeted_segment | L1 |
| **BUS06** Preismodell | Stufe · Wert-Naht | Modell | Konsistenz/Deckungsbeitrag | negative_margin | L1 |
| **BUS07** Geschäftsmodell-Canvas | Baustein · Bezug-Naht | Canvas | Baustein-Vollständigkeit | missing_revenue_stream | L1 |
| **BUS08** SWOT-Analyse | Faktor · Ableit-Naht | SWOT | Ableitbarkeit zu Aktionen | actionless_factor | L1 |
| **BUS09** OKR-Set | Objective · KR-Naht | OKR | Messbarkeit der KRs | unmeasurable_kr | L1 |
| **BUS10** Investoren-Pitch | Aussage · Beleg-Naht | Pitch-Struktur | Beleg je Kernaussage | unbacked_claim | L1 |
| **BUS11** Value Proposition | Nutzen · Bedürfnis-Naht | Statement | Bedürfnis-Passung | value_gap | L1 |
| **BUS12** Kundensegmentierung | Segment · Kriterium-Naht | Modell | Trennschärfe | overlapping_segments | L1 |
| **BUS13** Umsatzprognose | Treiber · Formel-Naht | Modell-Struktur | Treiber-Konsistenz | unfounded_projection | L1 |
| **BUS14** Partnerschafts-Vorschlag | Beitrag · Gegenleistungs-Naht | Vorschlag | Ausgewogenheit | one_sided_deal | L1 |
| **BUS15** Produkt-Roadmap (Business) | Initiative · Wert-Naht | Roadmap | Wert/Reihenfolge | valueless_initiative | L1 |

---

### J — Wissen/Forschung/Quellen (KNOW01–KNOW15)

**Familien-Profil:** Unit = Aussage/Quelle/Hypothese; Seam = Beleg-/Zitat-/Ableit-Beziehung. `materialize` → Review/Antrag/Dossier; `reanalyze` liest Beleg-/Zitatnetz zurück; `equivalent` = gleiche Evidenz-/Argumentstruktur. Quellen-Treue und Nicht-Erfindung besonders kritisch.

| ID · Zweck | Crystal (Unit · Seam) | Artefakt | Kern-Gate | Kern-Residuum / Counter-Horizon | Level |
|---|---|---|---|---|---|
| **KNOW01** Literaturübersicht | Aussage · Quell-Naht | Review | Beleg je Aussage | uncited_claim; erfunden | L1 |
| **KNOW02** Forschungsantrag | Ziel · Methoden-Naht | Antrag | Methode je Ziel | methodless_aim | L1 |
| **KNOW03** Quellen-/Zitat-Netz | Quelle · Zitat-Naht | Zitatgraph | Auflösbarkeit der Zitate | dangling_citation | L1 |
| **KNOW04** Hypothesen-Set | Hypothese · Test-Naht | Set | Testbarkeit/Falsifizierbar | untestable_hypothesis | L1 |
| **KNOW05** Experiment-Design | Faktor · Kontroll-Naht | Design | Kontrolle/Validität | confounded_design | L1 |
| **KNOW06** Meta-Analyse | Studie · Gewicht-Naht | Struktur | Ein-/Ausschluss-Konsistenz | selection_bias | L1 |
| **KNOW07** Annotierte Bibliographie | Quelle · Annotations-Naht | Bibliographie | Annotation je Quelle | unannotated_source | L1 |
| **KNOW08** Wissensbasis-Artikel | Aussage · Beleg-Naht | Artikel | Belegbarkeit | unsupported_statement | L1 |
| **KNOW09** Systematic Review (PRISMA) | Studie · Fluss-Naht | Review | PRISMA-Fluss-Vollständig | unaccounted_exclusion | L1 |
| **KNOW10** Faktencheck-Dossier | Behauptung · Beleg-Naht | Dossier | Beleg/Gegenbeleg | one_sided_check | L1 |
| **KNOW11** Begriffs-Landkarte | Begriff · Beziehungs-Naht | Karte | Zusammenhang | isolated_concept | L1 |
| **KNOW12** Evidenz-Synthese | Befund · Quell-Naht | Synthese | Widerspruchs-Ausweisung | hidden_conflict | L1 |
| **KNOW13** Argumentationskarte | These · Prämissen-Naht | Karte | Prämissen-Vollständig | enthymeme_gap | L1 |
| **KNOW14** Datenquellen-Verzeichnis | Quelle · Herkunfts-Naht | Verzeichnis | Herkunft/Lizenz | unknown_provenance | L1 |
| **KNOW15** Forschungsbericht | Ergebnis · Methoden-Naht | Bericht | Methode↔Ergebnis-Bezug | unsupported_result | L1 |

---

### K — Bildung/Training (EDU01–EDU12)

**Familien-Profil:** Unit = Lernziel/Inhalt/Aufgabe; Seam = Voraussetzungs-/Bewertungs-Beziehung. `materialize` → Lehrplan/Lektion/Assessment; `reanalyze` liest Ziel-Inhalt-Bewertung zurück; `equivalent` = gleiche Lernziel-Abdeckungsstruktur.

| ID · Zweck | Crystal (Unit · Seam) | Artefakt | Kern-Gate | Kern-Residuum / Counter-Horizon | Level |
|---|---|---|---|---|---|
| **EDU01** Lehrplan/Curriculum | Lernziel · Voraussetzungs-Naht | Curriculum | Voraussetzungs-Konsistenz | prerequisite_gap | L1 |
| **EDU02** Lektion/Lesson Plan | Ziel · Aktivitäts-Naht | Plan | Ziel-Aktivitäts-Bezug | goalless_activity | L1 |
| **EDU03** Quiz/Assessment | Frage · Lernziel-Naht | Assessment | Ziel-Abdeckung | unaligned_question | L1 |
| **EDU04** Übungsaufgaben-Set | Aufgabe · Schwierigkeits-Naht | Set | Progression | difficulty_jump | L1 |
| **EDU05** Lernpfad | Modul · Abhängigkeits-Naht | Pfad | Erreichbarkeit | orphan_module | L1 |
| **EDU06** Rubrik/Bewertungsraster | Kriterium · Niveau-Naht | Rubrik | Trennschärfe (kein Score-Gate) | ambiguous_level | L1 |
| **EDU07** Kurs-Syllabus | Einheit · Ziel-Naht | Syllabus | Ziel-Deckung | uncovered_objective | L1 |
| **EDU08** Tutorial/Anleitung | Schritt · Reihenfolge-Naht | Tutorial | Nachvollziehbarkeit | gap_in_steps | L1 |
| **EDU09** Fallstudie (Teaching) | Element · Lernziel-Naht | Fallstudie | Ziel-Bezug | pointless_detail | L1 |
| **EDU10** Kompetenzmodell | Kompetenz · Ableit-Naht | Modell | Beobachtbarkeit | unobservable_competency | L1 |
| **EDU11** Flashcard-Set | Karte · Konzept-Naht | Set | Eindeutigkeit | ambiguous_card | L1 |
| **EDU12** Schulungshandbuch | Modul · Ziel-Naht | Handbuch | Ziel-Deckung | uncovered_topic | L1 |

---

### L — Kreativ/Medien/Design (CRE01–CRE15)

**Familien-Profil:** Unit = Element (Szene/Komponente/Motiv); Seam = Fluss-/Konsistenz-Beziehung. `materialize` → Outline/Script/Style-Guide/Spec; `reanalyze` liest Struktur/Konsistenz zurück; `equivalent` = gleiche Struktur-/Konsistenzklasse. **Urheberrecht-Gate** familienweit: das Artefakt ist **originale Struktur/Form**, nie Reproduktion geschützter Werke.

| ID · Zweck | Crystal (Unit · Seam) | Artefakt | Kern-Gate | Kern-Residuum / Counter-Horizon | Level |
|---|---|---|---|---|---|
| **CRE01** Story/Narrative | Szene · Kausal-Naht | Outline | Kausal-Kohärenz, Urheberrecht | plot_hole; Reproduktion | L1 |
| **CRE02** Drehbuch/Script | Beat · Fluss-Naht | Script | Struktur-Vollständigkeit | dangling_beat | L1 |
| **CRE03** Design-System/Style-Guide | Token · Nutzungs-Naht | Style-Guide | Token-Konsistenz | inconsistent_token | L1 |
| **CRE04** Wireframe/UX-Flow | Screen · Übergangs-Naht | Flow | Erreichbarkeit | dead_end_screen | L1 |
| **CRE05** Kampagnen-Konzept | Botschaft · Kanal-Naht | Konzept | Botschaft-Konsistenz | off_message | L1 |
| **CRE06** Moodboard-Spec | Motiv · Zusammenhangs-Naht | Spec | Kohärenz | incoherent_mood | L1 |
| **CRE07** Charakter/World-Building | Entität · Konsistenz-Naht | Kompendium | Konsistenz | contradiction | L1 |
| **CRE08** Content-Kalender | Beitrag · Termin-Naht | Kalender | Kadenz-Konsistenz | scheduling_gap | L1 |
| **CRE09** Slogan/Naming-Set | Kandidat · Kriterium-Naht | Set | Kriterien-Deckung, Urheberrecht | trademark_clash | L1 |
| **CRE10** Storyboard | Panel · Fluss-Naht | Storyboard | Kontinuität | continuity_break | L1 |
| **CRE11** Songtext-Struktur | Abschnitt · Reim/Fluss-Naht | Struktur (original) | Urheberrecht (keine Reproduktion) | reproduction; Formbruch | L1 |
| **CRE12** Gedicht-Form | Zeile · Metrum/Reim-Naht | Form (original) | Form-Konsistenz, Urheberrecht | broken_meter; Reproduktion | L1 |
| **CRE13** Podcast/Video-Skript | Segment · Übergangs-Naht | Skript | Fluss/Timing | abrupt_transition | L1 |
| **CRE14** Marken-Identität | Element · Konsistenz-Naht | Guide | Konsistenz | off_brand | L1 |
| **CRE15** Layout/Composition | Element · Ausrichtungs-Naht | Spec | Balance/Hierarchie | visual_imbalance | L1 |

---

### M — Hardware/CAD/Mechatronik/Fertigung (HW01–HW15)

**Familien-Profil:** Unit = Bauteil/Signal/Schritt; Seam = Verbindungs-/Toleranz-/Ablauf-Beziehung. `materialize` → BOM/Netlist/CAD-Spec/Plan; `reanalyze` liest Verbindungen/Toleranzen zurück; `equivalent` = gleiche physisch-funktionale Struktur. Sicherheits-Gates (FMEA) besonders kritisch.

| ID · Zweck | Crystal (Unit · Seam) | Artefakt | Kern-Gate | Kern-Residuum / Counter-Horizon | Level |
|---|---|---|---|---|---|
| **HW01** Stückliste (BOM) | Bauteil · Verbau-Naht | BOM | Vollständigkeit/Eindeutig | missing_part | L1 |
| **HW02** Schaltplan/Netlist | Bauteil · Netz-Naht | Netlist | Konnektivität/kein offener Pin | floating_pin | L1 |
| **HW03** CAD-Modell-Spec | Feature · Bezugs-Naht | CAD-Spec | Bemaßungs-Vollständig | underdimensioned | L1 |
| **HW04** Toleranz-/Passungs-Plan | Maß · Passungs-Naht | Plan | Toleranzkette | tolerance_stackup | L1 |
| **HW05** Fertigungsplan (Routing) | Schritt · Reihenfolge-Naht | Plan | Fertigbarkeit/Reihenfolge | infeasible_step | L1 |
| **HW06** Prüfplan (Inspection) | Merkmal · Prüf-Naht | Plan | Prüfung je kritisches Merkmal | uninspected_feature | L1 |
| **HW07** Mechatronik-Steuerung | Zustand · Übergang | Steuerungs-Spec | Determinismus/Sicherheit | unsafe_state | L1 |
| **HW08** PCB-Layout-Constraint | Netz · Regel-Naht | Constraints | Regel-Vollständigkeit (EMV) | violated_clearance | L1 |
| **HW09** Materialauswahl | Anforderung · Material-Naht | Auswahl | Anforderungs-Deckung | unmet_property | L1 |
| **HW10** Montageanleitung | Schritt · Reihenfolge-Naht | Anleitung | Reihenfolge/Werkzeug | impossible_assembly | L1 |
| **HW11** Wartungsplan | Aufgabe · Intervall-Naht | Plan | Deckung kritischer Teile | unmaintained_part | L1 |
| **HW12** Sicherheitsanalyse (FMEA) | Fehler · Maßnahme-Naht | FMEA | Maßnahme je kritischen Fehler | unmitigated_failure | L1 |
| **HW13** Energie-/Leistungsbudget | Verbraucher · Budget-Naht | Budget | Bilanz-Deckung | power_overrun | L1 |
| **HW14** 3D-Druck-Vorbereitung | Region · Stütz-Naht | Slicing-Spec | Druckbarkeit/Stützen | unsupported_overhang | L1 |
| **HW15** Kinematik-/Mechanik-Modell | Glied · Gelenk-Naht | Modell | Freiheitsgrad-Konsistenz | overconstrained | L1 |

---

### N — Kommunikation/CRM/Organisation (COM01–COM12)

**Familien-Profil:** Unit = Nachricht/Kontakt/Punkt; Seam = Adressats-/Bezug-Beziehung. `materialize` → E-Mail/Agenda/Modell/Plan; `reanalyze` liest Adressierung/Bezug zurück; `equivalent` = gleiche Kommunikations-/Organisationsstruktur.

| ID · Zweck | Crystal (Unit · Seam) | Artefakt | Kern-Gate | Kern-Residuum / Counter-Horizon | Level |
|---|---|---|---|---|---|
| **COM01** E-Mail/Nachricht | Absatz · Zweck-Naht | E-Mail | Zweck/Adressat klar | purposeless_message | L1 |
| **COM02** Meeting-Agenda | Punkt · Ziel-Naht | Agenda | Ziel/Zeit je Punkt | goalless_item | L1 |
| **COM03** CRM-Kontakt-Modell | Kontakt · Beziehungs-Naht | Modell | Beziehungs-Konsistenz | orphan_contact | L1 |
| **COM04** Kommunikationsplan | Botschaft · Zielgruppen-Naht | Plan | Zielgruppen-Deckung | unreached_audience | L1 |
| **COM05** Newsletter-Struktur | Sektion · Fluss-Naht | Struktur | Fluss/CTA | missing_cta | L1 |
| **COM06** Stakeholder-Map | Stakeholder · Interessen-Naht | Map | Deckung/Einfluss | unmapped_stakeholder | L1 |
| **COM07** Antwort-Vorlage | Baustein · Fall-Naht | Vorlage | Fall-Deckung | uncovered_case | L1 |
| **COM08** Eskalations-Kommunikation | Stufe · Empfänger-Naht | Nachricht | Empfänger-Vollständig | missed_recipient | L1 |
| **COM09** Onboarding-Kommunikation | Schritt · Zeit-Naht | Sequenz | Lückenlosigkeit | onboarding_gap | L1 |
| **COM10** Kundenfeedback-Synthese | Feedback · Thema-Naht | Synthese | Themen-Treue | lost_signal | L1 |
| **COM11** Interne Ankündigung | Aussage · Bezug-Naht | Ankündigung | Klarheit/Vollständig | ambiguous_notice | L1 |
| **COM12** Verteiler-/Org-Struktur | Einheit · Zuordnungs-Naht | Struktur | Zuordnungs-Vollständig | orphan_unit | L1 |

---

### O — Finanzen/Verwaltung/Planung (FIN01–FIN10)

**Familien-Profil:** Unit = Position/Konto/Treiber; Seam = Summen-/Ableit-Beziehung. `materialize` → Budget/Modell/Report; `reanalyze` liest Summen/Ableitungen zurück; `equivalent` = gleiche Rechenstruktur. Bilanz-Gates (Summen stimmen) besonders kritisch.

| ID · Zweck | Crystal (Unit · Seam) | Artefakt | Kern-Gate | Kern-Residuum / Counter-Horizon | Level |
|---|---|---|---|---|---|
| **FIN01** Budget-Plan | Position · Summen-Naht | Budget | Summen-Konsistenz | unbalanced_budget | L1 |
| **FIN02** Kostenaufstellung | Posten · Zuordnungs-Naht | Aufstellung | Zuordnungs-Vollständig | unassigned_cost | L1 |
| **FIN03** Rechnungs-Struktur | Position · Summen-Naht | Rechnung | Summen/Steuer-Konsistenz | miscalculated_total | L1 |
| **FIN04** Finanzmodell | Treiber · Formel-Naht | Modell-Struktur | Formel-Konsistenz | circular_reference | L1 |
| **FIN05** Forecast/Prognose | Treiber · Ableit-Naht | Forecast | Treiber-Belegbarkeit | unfounded_forecast | L1 |
| **FIN06** Ausgaben-Report | Ausgabe · Kategorie-Naht | Report | Kategorie-Deckung | uncategorized_expense | L1 |
| **FIN07** Investitionsrechnung | Cashflow · Diskont-Naht | ROI/NPV-Struktur | Methoden-Konsistenz | wrong_discounting | L1 |
| **FIN08** Liquiditätsplan | Zu-/Abfluss · Zeit-Naht | Plan | Zeitreihen-Konsistenz | liquidity_gap | L1 |
| **FIN09** Kostenstellen-Plan | Kostenstelle · Zuordnungs-Naht | Plan | Vollständige Zuordnung | orphan_cost_center | L1 |
| **FIN10** Reconciliation-Struktur | Buchung · Abgleich-Naht | Struktur | Abgleich-Vollständig | unreconciled_item | L1 |

---

### P — Regulated Advisory mit ProfessionalReviewGate (REG01–REG08)

**Familien-Profil:** Unit = Aussage/Empfehlung/Befund; Seam = Beleg-/Rechtsgrund-Beziehung. `materialize` → Entwurf-Dokument (**stets „Entwurf, nicht fachgeprüft"** bis zur Freigabe); `reanalyze` liest Belegstruktur zurück; `equivalent` = gleiche Empfehlungs-/Belegstruktur. **Alle** tragen zusätzlich das **harte ProfessionalReviewGate** (§K.3).

| ID · Zweck | Crystal (Unit · Seam) | Artefakt | Kern-Gate | Kern-Residuum / Counter-Horizon | Level |
|---|---|---|---|---|---|
| **REG01** Rechtsberatungs-Entwurf | Aussage · Rechtsgrund-Naht | Entwurf | **ProfessionalReviewGate (Jurist)** + Belegbarkeit | unreviewed; unbelegt | L1 |
| **REG02** Medizinische Information | Aussage · Evidenz-Naht | Entwurf | **ProfessionalReviewGate (Kliniker)** + Evidenz | unreviewed; unbelegt | L1 |
| **REG03** Finanz-/Anlageberatung | Empfehlung · Grundlagen-Naht | Entwurf | **ProfessionalReviewGate (Berater)** + Eignung | unreviewed; unsuitable | L1 |
| **REG04** Steuerberatung | Aussage · Norm-Naht | Entwurf | **ProfessionalReviewGate (Steuerprofi)** + Norm-Bezug | unreviewed; norm_gap | L1 |
| **REG05** Compliance-Gutachten | Befund · Norm-Naht | Entwurf | **ProfessionalReviewGate (Compliance)** + Belegbarkeit | unreviewed; unbacked | L1 |
| **REG06** Ingenieur-Abnahme | Aussage · Nachweis-Naht | Entwurf | **ProfessionalReviewGate (PE/Ing.)** + Nachweis | unreviewed; unproven | L1 |
| **REG07** Sicherheits-/Gefahrgut-Bewertung | Risiko · Maßnahme-Naht | Entwurf | **ProfessionalReviewGate (Sicherheitsexperte)** + Maßnahme | unreviewed; unmitigated | L1 |
| **REG08** Klinische/Pharma-Doku | Aussage · Regulatorik-Naht | Entwurf | **ProfessionalReviewGate (Regulatory/Med.)** + Konformität | unreviewed; noncompliant | L1 |

---

## K.5 — Adapter-Parität über den ganzen Katalog

- **Ein Vertrag, 213 Implementierungen.** Jede Leaf-Domäne implementiert exakt den `DomainAdapter` (S1.8). `check_adapter_parity` prüft je Domäne alle 11 Punkte.
- **Familien-Profile teilen die Methoden-Ausprägung.** Domänen einer Familie teilen die familienspezifische Ausprägung der 8 Methoden (§K.4-Familienköpfe); die Leaf-Ebene fügt nur Crystal-Typ/Gates/Residuen hinzu.
- **Kein halb gebautes Modell.** Eine Domäne, die auf L4 gehoben wird, muss alle 11 Punkte vollständig (inkl. Referenz-Cube, Negativ-Cubes, Doku-Slot, Bibliotheks-Slot) tragen — sonst bleibt sie sichtbar unter L4.

---

## K.6 — Sichtbare Residuen (Unfertigkeit strukturiert, nicht kaschiert)

Das **Gesamt-Residuum** des Katalogs, vollständig ausgewiesen:

- **Reifegrad-Verteilung:** D01 = **L4**; **alle übrigen 212 Domänen = L1**. Die Menge {alle Domänen unter L4} **ist** der geführte Rest — jede einzeln benannt, keine ausgelassen.
- **Was je Domäne unter L4 noch aussteht (nach §K.2):** L2 (8 Methoden konkret über das Familien-Profil hinaus), L3 (Referenz-/Negativ-Cubes + Bibliotheks-Slot, CI-gewacht), L4 (Doku-Slot vollständig, im Cockpit bedienbar, `ProduktDoD(Domäne)=1`).
- **Priorisierungs-Weiche (offen, zu bestätigen):** welche Domänen nach Dokument als Nächstes auf L4 gehoben werden. Empfehlung: je eine „Anker"-Domäne pro Familie zuerst (z. B. SWE01, DATA02, GRA01, MATH01, PM01, GOV01, OPS02, BUS01, KNOW01, EDU01, CRE01, HW01, COM01, FIN01, REG01), dann Breite.
- **Familienspezifische Zusatz-Gates (spezifiziert, Umsetzung je Domäne offen):** ProfessionalReviewGate (Familie P, §K.3); Urheberrecht-Gate (Familie L); Bilanz-Gate (Familie O); FMEA-Sicherheits-Gate (Familie M); Fail-closed-Betonung (Familie H).

Kein Loch: Jede der 213 Domänen ist benannt, differenziert (L1) und mit ihrem ausstehenden Reifepfad versehen.

---

## K.7 — Abnahme (DoD des Katalogs)

```
DoD(Domänen-Katalog) = 1  ⟺
    alle 213 Leaf-Domänen in 16 Familien sind benannt (ID + Zweck)
  ∧ jede Domäne ist mindestens L1: Crystal-Typ (Unit·Seam), Artefakt, Domänen-Gate,
        Residuum/Counter-Horizon spezifiziert (§K.4)
  ∧ jede Domäne implementiert denselben DomainAdapter-Kontrakt (S1.8); Ausprägung je Familien-Profil (§K.5)
  ∧ das Produkt-Level jeder Domäne ist gesetzt; Unfertigkeit ist als Level sichtbar geführt (§K.2/§K.6),
        nie durch Auslassen kaschiert
  ∧ Familie P trägt das harte ProfessionalReviewGate (§K.3); familienspezifische Zusatz-Gates sind benannt
  ∧ Dokument-Bericht (D01) ist L4 (S1-Referenz); der ausstehende Reifepfad aller übrigen ist benannt (§K.6)
  ∧ Engine-DoD, DoD(S1..S13) und S10 (Produkt-DoD Dokument) bleiben unberührt
```

---

## K.8 — Anschluss

Der Domänen-Katalog ist vollständig auf **L1** (alle 213 Domänen benannt, differenziert, adapter-paritätisch, mit sichtbarem Reifegrad) — die horizontale Achse ist damit **strukturell geschlossen**; das Heben einzelner Domänen auf L4 folgt der Priorisierungs-Weiche (§K.6), ohne Kernberührung (Adapter-Parität).

**Als Nächstes (Ihre zweite Achse):** **S14 — Kern-Erweiterungs-Protokoll** (vertikal) — `CoreExtension` mit closure-erhaltenden Verträgen, vom Regressionswächter validiert, azyklisch über Ports, Invarianten über alle Erweiterungen gehalten. Damit wäre auch die **vertikale** Achse spezifiziert und die **Plattform-DoD** vollständig auf Papier.

*Ende des Domänen-Katalogs (Erweiterung von S1).*
