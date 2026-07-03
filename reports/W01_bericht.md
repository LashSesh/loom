Welle: W1 — Familie A (Dokument/Text, D02–D15)
Eingang erfüllt: ja (Wellenplan committet; CI grün vor Welle)
Gebaut (Masterplan §2):
- **Familien-Kern** `crates/cce-materialize/src/family_a.rs`: ein
  geteilter, datengetriebener Kern. Die gesamte Motorik (encode / loom /
  materialize / reanalyze / to_canonical / canonicalize / equivalent)
  DELEGIERT an den bestehenden `DocumentAdapter` (D01) — KEINE Kopie;
  D01 bleibt unberührt als PL4-Anker. Je Domäne kommt nur hinzu:
  (a) die Kern-Naht-REGEL (`DomainRule`: Relation · AcyclicRelation ·
  ChainedRelation · UniqueSubjects · OrderedSteps · StructuralPresence),
  (b) das Kern-Residuum aus dem Katalog, (c) Referenz-/Negativ-Cubes.
  `DocDomainAdapter` implementiert den vollen DomainAdapter-Vertrag
  (11 Punkte) über dem Kern; `domain_core_gate` setzt die Regel
  fail-closed durch.
- **14 Domänen** `crates/cce-materialize/src/family_a_domains.rs` als
  reine Profil-Spezialisierung: D02 Vertrag (Verweis→dangling_clause),
  D03 Spezifikation (Ableit→untestable_req), D04 Handbuch (Reihenfolge→
  gap_in_procedure), D05 Brief (Anrede/Schluss→missing_salutation),
  D06 Angebot (Preis→orphan_line_item), D07 Richtlinie (Geltung→
  unscoped_rule), D08 Protokoll (Beschluss→actionless_item),
  D09 Zusammenfassung (Quelle→invented_semantic), D10 Pressemitteilung
  (Beleg→unsupported_claim), D11 Lebenslauf (Zeit→timeline_gap),
  D12 Whitepaper (Argument→broken_argument), D13 FAQ (Antwort→
  unanswered_question), D14 Checkliste (Eindeutigkeit→ambiguous_item),
  D15 Glossar (Definition/Zirkel→undefined_term).
- **Zeugen** `conformance/tests/family_a_catalog.rs` (im einen Wächter):
  je Domäne 1 Referenz-Cube + ≥2 Negativ-Cubes (28 Negative gesamt).
- **Katalog-Hebung**: catalog_data.rs D02–D15 von PL1 auf **PL3**;
  `WITNESSED_DOMAINS` (D01–D15) als die eine Wahrheit, an der PL2/PL3
  hängt; `feature_maturity_overclaim` bleibt leer (Katalog- UND
  conformance-Ebene). Doku-Zeile: `docs/operator/domaenen/familie_a.md`.
Ausgangs-Gate W1 (§2, vollständig):
- Alle 14 Domänen `check_adapter_parity` 11/11 grün
  (w1_all_domains_parity_11_of_11).
- Alle Referenz-Cubes schließen: voller Motorpfad + alle Gates (inkl.
  Domänen-Kern-Gate) grün (w1_reference_cubes_close_all_gates_green).
- Alle Negativ-Cubes rot mit ERWARTETEM Kern-Residuum
  (w1_negative_cubes_trigger_expected_core_residue).
- Alle Domänen-Kerntests ≃ (Reanalyze∘Materialize∘LOOM∘Project∘PHC = id;
  w1_domain_kerntest_reanalyze_is_identity).
- CI GRUEN; GUARD_PHASES += "W1"; Register/Katalog committet.
Interleaving-Wahl (§3): nach W1 folgt eine Track-Einheit **D/P5**
(HttpTransport + Wikimedia live, Snapshot-fixiert) gemäß Wellenplan.
Residuen dieser Welle:
- PL4 für D02–D15 bleibt offen (verlangt echte Nutzungs-Evidenz;
  Anti-Overclaim). Agent hebt strukturell nur bis PL3.
- Die Kern-Naht-Regeln generalisieren die S1-Referenzprofile auf das
  geteilte D01-Unit-Vokabular (Subjekt = neutraler Typ ohne Stütz-
  Pflicht, um Kollision mit dem geerbten Support-Gate zu vermeiden);
  domänenspezifische Unit-Namen (Klausel, Anforderung …) sind
  Darstellungssache, die geprüfte Struktur ist die Naht-Regel.
Abweichungen von der Spec: keine; spec/ unangetastet.
