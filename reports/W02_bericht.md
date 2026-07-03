Welle: W2 — Familie J (Wissen/Forschung/Quellen, KNOW01–15)
Eingang erfüllt: ja (W1 grün, D/P5 grün, CI grün)
Gebaut (Masterplan §2):
- **Kein neuer Familien-Kern nötig**: Familie J bildet 1:1 auf den in W1
  gebauten `family_a`-Kern ab (Text/Naht-Grammatik) — 14 Relation-Regeln
  + 1 Ketten-Regel (KNOW09 PRISMA-Fluss). Das ist die im Wellenplan §2.5-b
  begründete maximale Kern-Wiederverwendung; der `relation_domain!`-Builder
  wurde crate-weit teilbar gemacht (pub(crate) use), sonst KEIN neuer
  Motor-Code.
- **15 Domänen** `crates/cce-materialize/src/family_j_domains.rs` als reine
  Profil-Daten (Kern-Naht + Kern-Residuum aus S1 K.4): KNOW01 uncited_claim,
  KNOW02 methodless_aim, KNOW03 dangling_citation, KNOW04
  untestable_hypothesis, KNOW05 confounded_design, KNOW06 selection_bias,
  KNOW07 unannotated_source, KNOW08 unsupported_statement, KNOW09
  unaccounted_exclusion (Kette), KNOW10 one_sided_check, KNOW11
  isolated_concept, KNOW12 hidden_conflict, KNOW13 enthymeme_gap, KNOW14
  unknown_provenance, KNOW15 unsupported_result.
- **Zeugen** `conformance/tests/family_j_catalog.rs` (im einen Wächter):
  Parität 11/11, Referenz schließt, ≥2 Negative je Domäne (30 gesamt),
  Domänen-Kerntest ≃.
- **Katalog-Hebung** KNOW01–15 PL1→PL3; WITNESSED_DOMAINS auf 30 erweitert;
  overclaim leer. Doku `docs/operator/domaenen/familie_j.md`.
Ausgangs-Gate W2 (§2, vollständig):
- Parität 11/11 alle 15 (w2_all_domains_parity_11_of_11) = grün
- Referenz-Cubes schließen, alle Gates grün (w2_reference_cubes…) = grün
- Negative rot mit erwartetem Kern-Residuum (w2_negative_cubes…) = grün
- Domänen-Kerntests ≃ (w2_domain_kerntest…) = grün
- CI GRUEN; GUARD_PHASES += "W2"; Katalog/Register committet.
Interleaving-Wahl (§3): nach W2 folgt **C/P4** (echtes Lokalmodell hinter
LocalModelProvider, recorded) gemäß Wellenplan.
Residuen: PL4 für KNOW01–15 offen (Nutzungs-/Review-Evidenz).
Abweichungen: keine; spec/ unangetastet.
