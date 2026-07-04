//! Etappe X3/E5 (Karte §2/E5, S-E5: L9b Normic Memory) — Ring E5,
//! Residuum R-1b. Alle 12 Zeugen (R-NRM-1..4, N-NRM-1..8) + PROD-INV-21..23.
//!
//! R-NRM-1 ist der Meilenstein: die erste aktive Norm, destilliert aus
//! DREI echten, geschlossenen Familien-Referenz-Cubes (D02/D03/D06 —
//! dieselbe `Relation`-Kern-Naht-Regel-FORM, verschiedene Nahtnamen),
//! als zertifizierter `.loom` mit beweisbarer Herkunft
//! (`build_first_active_norm`, `loom-conformance`).

use cce_bridge::activation::{activate, ActivationError, NormProfile};
use cce_bridge::distill::{distill, DistillError, DistillationInput};
use cce_bridge::gate::{bridge_gate, BridgeGateContext};
use cce_bridge::lifecycle::{check_erosion, revoke};
use cce_bridge::memory::{parse_norms_section, query};
use cce_bridge::provenance::ProvenanceError;
use cce_bridge::types::{residue, CounterExample, NexusClass, NormStatus, Scope};
use cce_bridge::{BridgeNorm, BridgeVerdict};
use cce_core::replay::RunDescriptor;
use cce_core::signature::sha256;
use cce_runner::runner::{Run, RunStatus};
use loom_cites::{citation_gate, hex34, SeedResolver};
use loom_conformance::{build_class_registry, build_first_active_norm, InMemoryResolver};

fn seed_dir() -> std::path::PathBuf {
    std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../library/seed"))
}

// ---------------------------------------------------------------------
// R-NRM-1 — Meilenstein: erste aktive Norm, echte Herkunft, CitationGate
// gruen.
// ---------------------------------------------------------------------

#[test]
fn r_nrm_1_first_active_norm_is_valid_with_green_citation_gate() {
    let (norm, sealed, report, members) = build_first_active_norm();
    assert_eq!(report.verdict(), BridgeVerdict::Allow);
    assert_eq!(norm.status, NormStatus::Active);
    assert_eq!(norm.provenance_set.len(), 3);

    let verification = loom_verify::verify(&sealed.bytes);
    assert_eq!(
        verification.verdict,
        loom_verify::Verdict::Valid,
        "{:?}",
        verification.diagnoses
    );

    // Herkunft ist beweisbar: die derives-cites zeigen GENAU auf die drei
    // Familien-Referenz-Cubes, ueber den SeedResolver aufgeloest (echter
    // Verzeichnis-Scan ueber die committeten Seed-Dateien).
    let resolver = SeedResolver {
        search_dirs: vec![seed_dir()],
    };
    let gate = citation_gate(&sealed.bytes, &resolver);
    assert_eq!(gate.entries.len(), 3);
    assert!(
        gate.entries
            .iter()
            .all(|e| e.outcome == loom_cites::CitationOutcome::Ok),
        "alle drei derives-Nähte muessen gruen aufloesen: {:?}",
        gate.entries
    );
    assert!(
        gate.closure_pass,
        "CitationGate: closure_pass muss gruen sein"
    );

    // Die drei Herkunfts-Container sind selbst je "full"/claims_closed.
    for m in &members {
        let v = loom_verify::verify(&m.bytes);
        assert_eq!(v.verdict, loom_verify::Verdict::Valid);
    }
}

#[test]
fn seed_files_match_builders() {
    use cce_materialize::family_a_domains::{d02, d03, d06};
    use loom_conformance::build_family_reference_workbody;

    for (name, profile) in [("d02", d02()), ("d03", d03()), ("d06", d06())] {
        let built = build_family_reference_workbody(&profile);
        let on_disk = std::fs::read(seed_dir().join(format!("family_ref_{name}.loom")))
            .expect("Seed-Datei muss vorhanden sein");
        assert_eq!(built.bytes, on_disk, "family_ref_{name}.loom weicht ab");
    }

    let (_, sealed, _, _) = build_first_active_norm();
    let on_disk = std::fs::read(seed_dir().join("norm_relation_regel.loom"))
        .expect("Seed-Datei muss vorhanden sein");
    assert_eq!(sealed.bytes, on_disk);
}

/// §10(f): die `norms/`-Sektion der Klassen-Registry traegt die reale
/// Norm, `query` findet sie ueber Scope x NexusClass.
#[test]
fn normic_memory_section_is_queryable_from_the_real_registry() {
    let registry = build_class_registry();
    let dec = loom_codec::decode_sealed(&registry.bytes).unwrap();
    let doc = dec
        .frames
        .iter()
        .find_map(|(e, f)| {
            if e.kind == loom_format::KIND_DOC {
                loom_canon::decode(&f.payload).ok()
            } else {
                None
            }
        })
        .expect("Registry traegt ein KIND_DOC");
    let norms = parse_norms_section(&doc);
    assert_eq!(norms.len(), 1);
    let hits = query(&norms, &Scope::Global, Some(NexusClass::StructuralRule));
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].status, NormStatus::Active);
}

// ---------------------------------------------------------------------
// R-NRM-2 — Aktivierung: Lauf mit norm_profile, Replay klassenidentisch.
// ---------------------------------------------------------------------

#[test]
fn r_nrm_2_activation_via_norm_profile_is_replay_stable() {
    use cce_materialize::adapter::DomainAdapter;
    use cce_materialize::document::DocumentAdapter;

    let (norm, _sealed, _report, _members) = build_first_active_norm();
    let adapter = DocumentAdapter;
    let crystal = adapter.reference_cube();
    let rd = RunDescriptor::new(adapter.canonicalize(&crystal).0, "document", 42)
        .with_norm(norm.norm_id.clone());

    let mut first = Run::submit(crystal.clone(), rd.clone()).unwrap();
    first.activated_norms.push(norm.clone());
    first.run_to_end(None).unwrap();
    assert_eq!(first.status, RunStatus::Closed);
    assert!(first.rd.norm_profile.contains(&norm.norm_id));

    let mut second = Run::submit(crystal, rd).unwrap();
    second.activated_norms.push(norm);
    second.run_to_end(None).unwrap();
    assert_eq!(second.status, RunStatus::Closed);
    assert_eq!(first.result_class(), second.result_class());
}

// ---------------------------------------------------------------------
// R-NRM-3 — Widerruf: Widerrufs-Workbody, Status revoked,
// Alt-Lauf-Reanalyse zeigt norm_since_revoked.
// ---------------------------------------------------------------------

#[test]
fn r_nrm_3_revocation_workbody_and_reanalysis_shows_norm_since_revoked() {
    use cce_materialize::adapter::DomainAdapter;
    use cce_materialize::document::DocumentAdapter;

    let (norm, sealed, _report, _members) = build_first_active_norm();
    let norm_core_root_hex = hex34(&sealed.core_root);

    let (revoked, sealed_revocation) = revoke(
        &norm,
        &norm_core_root_hex,
        "Gegenbeispiel in einer vierten Domaene entdeckt",
        vec![CounterExample {
            core_root_hex: "ff".repeat(34),
            reason: "Subjekt ohne aufloesbare Naht, aber Regel greift nicht".to_string(),
        }],
    )
    .expect("Widerruf siegelt");
    assert_eq!(revoked.status, NormStatus::Revoked);
    let verification = loom_verify::verify(&sealed_revocation.bytes);
    assert_eq!(
        verification.verdict,
        loom_verify::Verdict::ValidWithResidues,
        "{:?}",
        verification.diagnoses
    );

    // Alt-Lauf-Reanalyse: derselbe RD (dieselbe RD-Klasse, Klassenstabilitaet)
    // aktiviert jetzt die WIDERRUFENE Kopie -> norm_since_revoked, kein
    // stiller Fortbestand.
    let adapter = DocumentAdapter;
    let crystal = adapter.reference_cube();
    let rd = RunDescriptor::new(adapter.canonicalize(&crystal).0, "document", 42)
        .with_norm(norm.norm_id.clone());
    let mut reanalysis = Run::submit(crystal, rd).unwrap();
    reanalysis.activated_norms.push(revoked);
    reanalysis.run_to_end(None).unwrap();
    match reanalysis.status {
        RunStatus::Rejected { reason } => {
            assert!(reason.contains(residue::NORM_SINCE_REVOKED), "{reason}")
        }
        other => panic!("Reanalyse einer widerrufenen Norm haette rejecten muessen: {other:?}"),
    }
}

// ---------------------------------------------------------------------
// R-NRM-4 — Erosion: Mitglied wird invalid ⇒ Norm automatisch deprecated.
// ---------------------------------------------------------------------

#[test]
fn r_nrm_4_member_invalidation_triggers_automatic_erosion() {
    let (norm, _sealed, _report, members) = build_first_active_norm();
    // Nur zwei der drei Mitglieder bleiben aufloesbar — das dritte ist
    // aus Resolver-Sicht "verschwunden" (invalid/widerrufen/quarantaenisiert
    // fuehren alle zum selben Resolver-Ergebnis: nicht mehr eligibel).
    let resolver = InMemoryResolver::from_sealed_subset(&members, &[0, 1]);
    let deprecated = check_erosion(&norm, &resolver).expect("Erosion muss erkannt werden");
    assert_eq!(deprecated.status, NormStatus::Deprecated);
}

// ---------------------------------------------------------------------
// N-NRM-1 — κ < κ_min ⇒ Hold (insufficient_provenance).
// ---------------------------------------------------------------------

#[test]
fn n_nrm_1_below_kappa_min_is_insufficient_provenance() {
    let (_norm, _sealed, _report, members) = build_first_active_norm();
    let resolver = InMemoryResolver::from_sealed_subset(&members, &[0, 1]);
    let roots: Vec<String> = [0usize, 1]
        .iter()
        .map(|&i| hex34(&members[i].core_root))
        .collect();
    let rd = RunDescriptor::new(sha256(b"insufficient-provenance"), "document", 1);
    let input = DistillationInput {
        pattern: "p".to_string(),
        nexus_class: NexusClass::StructuralRule,
        candidate_roots: roots,
        n_support: 2,
        known_counterexamples: vec![],
        scope: Scope::Global,
        kappa_min: 3,
    };
    let err = distill(input, &resolver, &rd).unwrap_err();
    assert_eq!(
        err,
        DistillError::Provenance(ProvenanceError::InsufficientProvenance {
            resolved: 2,
            kappa_min: 3
        })
    );
}

// ---------------------------------------------------------------------
// N-NRM-2 — Kandidat versucht Wirkung ohne Promotion ⇒ reject
// (== PROD-INV-21: keine Promotion ausser durch BridgeGate).
// ---------------------------------------------------------------------

#[test]
fn n_nrm_2_and_prod_inv_21_candidate_without_allow_cannot_seal() {
    use cce_bridge::workbody::{seal_norm, WorkbodyError};

    let (_norm, _sealed, _report, members) = build_first_active_norm();
    let resolver = InMemoryResolver::from_sealed(&members);
    let roots: Vec<String> = members.iter().map(|m| hex34(&m.core_root)).collect();
    // distill() selbst mit kappa_min=1 lassen (liefert einen Kandidaten),
    // aber das BridgeGate MIT dem echten kappa_min=3 pruefen — genau die
    // Trennung, die PROD-INV-21 verlangt: nur ein durchlaufenes Gate
    // entscheidet, nie der Kandidat selbst.
    let rd = RunDescriptor::new(sha256(b"no-promotion-without-gate"), "document", 1);
    let input = DistillationInput {
        pattern: "p".to_string(),
        nexus_class: NexusClass::StructuralRule,
        candidate_roots: roots[..1].to_vec(),
        n_support: 1,
        known_counterexamples: vec![],
        scope: Scope::Global,
        kappa_min: 1,
    };
    let candidate = distill(input, &resolver, &rd).expect("Destillation mit kappa_min=1 gelingt");

    let domains: Vec<String> = vec![];
    let ctx = BridgeGateContext::new(&domains, &[]);
    let report = bridge_gate(&candidate, &ctx);
    assert_eq!(report.verdict(), BridgeVerdict::Hold);

    match seal_norm(&candidate, &report, &[]) {
        Err(WorkbodyError::NotAllowed(BridgeVerdict::Hold)) => {}
        Err(other) => panic!("falscher Fehler: {other:?}"),
        Ok(_) => panic!("ein Hold-Verdikt darf NIEMALS einen Norm-Workbody erzeugen"),
    }
}

// ---------------------------------------------------------------------
// N-NRM-3 — Pattern will Gate lockern ⇒ norm_scope_violation reject
// (== PROD-INV-23: Normen lockern nie).
// ---------------------------------------------------------------------

#[test]
fn n_nrm_3_and_prod_inv_23_scope_loosening_patterns_are_rejected() {
    let (_norm, _sealed, _report, members) = build_first_active_norm();
    let resolver = InMemoryResolver::from_sealed(&members);
    let roots: Vec<String> = members.iter().map(|m| hex34(&m.core_root)).collect();

    for offending_pattern in [
        "Gate deaktivieren fuer PL2-Laeufe",
        "Capability-Lock entfernen bei aktivem norm_profile",
        "Invariante aufheben, sobald drei Gegenbeispiele vorliegen",
    ] {
        let rd = RunDescriptor::new(sha256(offending_pattern.as_bytes()), "document", 1);
        let input = DistillationInput {
            pattern: offending_pattern.to_string(),
            nexus_class: NexusClass::StructuralRule,
            candidate_roots: roots.clone(),
            n_support: 3,
            known_counterexamples: vec![],
            scope: Scope::Global,
            kappa_min: 3,
        };
        let candidate = distill(input, &resolver, &rd).expect("Destillation gelingt");
        let domains = vec!["dom:a".to_string(), "dom:b".to_string()];
        let ctx = BridgeGateContext::new(&domains, &[]);
        let report = bridge_gate(&candidate, &ctx);
        assert_eq!(
            report.verdict(),
            BridgeVerdict::Reject,
            "Pattern {offending_pattern:?} haette rejecten muessen"
        );
        assert_eq!(report.residue(), Some(residue::NORM_SCOPE_VIOLATION));
    }
}

// ---------------------------------------------------------------------
// N-NRM-4 — verschwiegene Gegenbeispiele ⇒ reject.
// ---------------------------------------------------------------------

#[test]
fn n_nrm_4_counterexamples_without_hitl_confirmation_are_rejected() {
    let (_norm, _sealed, _report, members) = build_first_active_norm();
    let resolver = InMemoryResolver::from_sealed(&members);
    let roots: Vec<String> = members.iter().map(|m| hex34(&m.core_root)).collect();

    let rd = RunDescriptor::new(sha256(b"counterexamples-unconfirmed"), "document", 1);
    let input = DistillationInput {
        pattern: "p".to_string(),
        nexus_class: NexusClass::StructuralRule,
        candidate_roots: roots,
        n_support: 3,
        known_counterexamples: vec![CounterExample {
            core_root_hex: "aa".repeat(34),
            reason: "abweichende Domaene".to_string(),
        }],
        scope: Scope::Global,
        kappa_min: 3,
    };
    let candidate = distill(input, &resolver, &rd).expect("Destillation gelingt");
    assert_eq!(candidate.n_counter, 1);

    let domains = vec!["dom:a".to_string(), "dom:b".to_string()];
    let ctx = BridgeGateContext::new(&domains, &[]);
    let report = bridge_gate(&candidate, &ctx);
    assert_eq!(report.verdict(), BridgeVerdict::Reject);
    assert_eq!(report.residue(), Some(residue::COUNTEREXAMPLE_UNRESOLVED));
}

// ---------------------------------------------------------------------
// N-NRM-5 — Norm-Konflikt ⇒ beide Hold, sichtbar.
// ---------------------------------------------------------------------

#[test]
fn n_nrm_5_conflicting_active_norm_of_same_scope_holds() {
    let (existing_norm, _sealed, _report, members) = build_first_active_norm();
    let resolver = InMemoryResolver::from_sealed(&members);
    let roots: Vec<String> = members.iter().map(|m| hex34(&m.core_root)).collect();

    let rd = RunDescriptor::new(sha256(b"conflicting-candidate"), "document", 1);
    let input = DistillationInput {
        pattern: "eine ANDERE Regel-Behauptung, gleicher globaler Scope".to_string(),
        nexus_class: NexusClass::StructuralRule,
        candidate_roots: roots,
        n_support: 3,
        known_counterexamples: vec![],
        scope: Scope::Global,
        kappa_min: 3,
    };
    let candidate = distill(input, &resolver, &rd).expect("Destillation gelingt");
    let domains = vec!["dom:a".to_string(), "dom:b".to_string()];
    let existing = vec![existing_norm];
    let mut ctx = BridgeGateContext::new(&domains, &[]);
    ctx.existing_active_norms = &existing;
    let report = bridge_gate(&candidate, &ctx);
    assert_eq!(report.verdict(), BridgeVerdict::Hold);
    assert_eq!(report.residue(), Some(residue::NORM_CONFLICT));
}

// ---------------------------------------------------------------------
// N-NRM-6 — Anwendung ohne Aktivierung ⇒ norm_not_activated
// (== PROD-INV-22: keine Norm-Wirkung ohne explizite Aktivierung im RD).
// ---------------------------------------------------------------------

#[test]
fn n_nrm_6_and_prod_inv_22_application_without_activation_is_refused() {
    let (norm, _sealed, _report, _members) = build_first_active_norm();
    // Leeres Profil: die Norm ist real und Active, aber nicht aktiviert.
    let err = activate(&norm, &NormProfile::default()).unwrap_err();
    assert_eq!(err, ActivationError::NotActivated);
    assert_eq!(err.residue(), residue::NORM_NOT_ACTIVATED);
}

// ---------------------------------------------------------------------
// N-NRM-7 — Destillation ohne RD (Hintergrund-Simulation) ⇒ verboten.
// ---------------------------------------------------------------------

#[test]
fn n_nrm_7_distillation_without_a_complete_rd_is_forbidden() {
    let (_norm, _sealed, _report, members) = build_first_active_norm();
    let resolver = InMemoryResolver::from_sealed(&members);
    let roots: Vec<String> = members.iter().map(|m| hex34(&m.core_root)).collect();

    let mut incomplete_rd = RunDescriptor::new(sha256(b"no-background-run"), "document", 1);
    incomplete_rd.domain = String::new();
    let input = DistillationInput {
        pattern: "p".to_string(),
        nexus_class: NexusClass::StructuralRule,
        candidate_roots: roots,
        n_support: 3,
        known_counterexamples: vec![],
        scope: Scope::Global,
        kappa_min: 3,
    };
    let err = distill(input, &resolver, &incomplete_rd).unwrap_err();
    assert_eq!(err, DistillError::IncompleteRunDescriptor("domain"));
}

// ---------------------------------------------------------------------
// N-NRM-8 — Replay-Mismatch der Destillation ⇒ rot.
// ---------------------------------------------------------------------

#[test]
fn n_nrm_8_distillation_replay_mismatch_is_rejected() {
    let (_norm, _sealed, _report, members) = build_first_active_norm();
    let resolver = InMemoryResolver::from_sealed(&members);
    let roots: Vec<String> = members.iter().map(|m| hex34(&m.core_root)).collect();

    let rd = RunDescriptor::new(sha256(b"replay-check"), "document", 1);
    let make_input = || DistillationInput {
        pattern: "identisches Pattern".to_string(),
        nexus_class: NexusClass::StructuralRule,
        candidate_roots: roots.clone(),
        n_support: 3,
        known_counterexamples: vec![],
        scope: Scope::Global,
        kappa_min: 3,
    };
    // Gleicher Snapshot + gleiches RD ⇒ klassenidentischer Kandidat.
    let a = distill(make_input(), &resolver, &rd).unwrap();
    let b = distill(make_input(), &resolver, &rd).unwrap();
    assert_eq!(
        a, b,
        "gleicher Snapshot + gleiches RD muss denselben Kandidaten ergeben"
    );

    // Eine abweichende "Reproduktion" (simuliert einen manipulierten
    // zweiten Destillationslauf) wird vom DistillationReplayGate erkannt.
    let mut mismatched = b.clone();
    mismatched.pattern = "abweichendes Pattern".to_string();
    let domains = vec!["dom:a".to_string(), "dom:b".to_string()];
    let mut ctx = BridgeGateContext::new(&domains, &[]);
    ctx.replay_reproduction = Some(&mismatched);
    let report = bridge_gate(&a, &ctx);
    assert_eq!(report.verdict(), BridgeVerdict::Reject);
    assert_eq!(
        report.residue(),
        Some(residue::DISTILLATION_REPLAY_MISMATCH)
    );
}

// ---------------------------------------------------------------------
// Alt-Zeugen: eine promovierte Norm bleibt ein normales `.loom`
// (extrahierbar, verifizierbar) — keine Sonderbehandlung im Reader.
// ---------------------------------------------------------------------

#[test]
fn norm_workbody_extracts_its_pattern_via_the_generic_reader_path() {
    let (norm, sealed, _report, _members) = build_first_active_norm();
    let dec = loom_codec::decode_sealed(&sealed.bytes).unwrap();
    let cl = dec
        .frames
        .iter()
        .find_map(|(e, f)| {
            if e.kind == loom_format::KIND_CL_SUBSTRATE {
                loom_canon::decode(&f.payload).ok()
            } else {
                None
            }
        })
        .expect("Norm-Workbody traegt CL_SUBSTRATE");
    let cites = loom_cites::parse_cites_from_cl_substrate(&cl);
    assert_eq!(cites.len(), norm.provenance_set.len());
    assert!(cites
        .iter()
        .all(|c| c.cite_kind == loom_cites::CiteKind::Derives));
}

#[allow(dead_code)]
fn assert_bridge_norm_type(_: &BridgeNorm) {}
