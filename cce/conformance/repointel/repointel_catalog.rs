//! RepoIntelligence-Zeugenkatalog (Dokument 23 Track B, P5) — Ablage
//! `conformance/repointel/`, dauerhaft im einen Waechter. Die volle
//! Kette (CSA-Einzug → HBM-Destillation → GroundingPacket →
//! Bauplan-Workbody) ist hermetisch und laeuft gruen in der normalen
//! CI. Der EINE reale Lauf (R-RIG-1, gegen das freigegebene Ziel-Repo)
//! lebt separat als `#[ignore]`-Betriebs-Harness in
//! `crates/cce-repointel/tests/` und folgt erst nach der
//! STOPP-Freigabe (Repo-Wahl durch den Auftraggeber).

use cce_repointel::ingest::{ingest_repo, RepoIntelInput};
use cce_repointel::kette::{run_repo_intelligence, RepoIntelError};
use cce_repointel::observe::RepoFile;

fn numkit_fixture(license: &str) -> RepoIntelInput {
    RepoIntelInput {
        repo_id: "beispiel/numkit".to_string(),
        commit_sha: "abc1234def5678".to_string(),
        declared_license: license.to_string(),
        files: vec![
            RepoFile::new(
                "Cargo.toml",
                b"[package]\nname = \"numkit\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
            ),
            RepoFile::new(
                "src/lib.rs",
                b"pub fn max_of(v: &[i64]) -> i64 {\n    let mut m = v[0];\n    for &x in v {\n        if x > m {\n            m = x;\n        }\n    }\n    m\n}\n",
            ),
            RepoFile::new(
                "tests/it.rs",
                b"use numkit::max_of;\n\n#[test]\nfn finds_maximum() {\n    assert_eq!(max_of(&[1, 9, 3]), 9);\n}\n",
            ),
        ],
    }
}

/// R-RIG-STRUCT: die volle P5-Kette am Fixture-Repo — BEIDE Koerper
/// (`source` + `blueprint`) sind `verify == Valid`; der Bauplan traegt
/// belegte Regeln (keine Herabstufung), zertifizierte
/// Blueprint-Kandidaten und den cite auf die Quell-Evidence.
#[test]
fn r_rig_struct_full_chain_seals_valid_source_and_blueprint() {
    let out = run_repo_intelligence(&numkit_fixture("mit")).expect("Kette gruen");

    // Quell-Container: Klasse "source", verify == Valid.
    let vs = loom_verify::verify(&out.source_sealed.bytes);
    assert_eq!(
        vs.verdict,
        loom_verify::Verdict::Valid,
        "source: {:?}",
        vs.diagnoses
    );
    // Bauplan: Klasse "blueprint". numkit traegt KEINE LICENSE-Datei ⇒
    // eine offene Entscheidung steht SICHTBAR im RESIDUE-Segment, das
    // ehrliche Verdikt ist deshalb ValidWithResidues (nie Reject, nie
    // ein leeres Residuenfeld durch Weglassen).
    let vb = loom_verify::verify(&out.blueprint_sealed.bytes);
    assert_eq!(
        vb.verdict,
        loom_verify::Verdict::ValidWithResidues,
        "blueprint: {:?}",
        vb.diagnoses
    );

    // Regeln sind belegt (evidence_ref), nichts herabgestuft.
    assert!(!out.packet.rules.is_empty());
    assert!(out.packet.rules.iter().all(|r| r.evidence_ref.is_some()));
    assert!(!out
        .visible_residues
        .iter()
        .any(|r| r.id.contains("rule_missing_evidence")));
    // Mindestens ein zertifizierter Blueprint-Kandidat.
    assert!(!out.distilled.outcome.certified.is_empty());
}

/// R-RIG-REPLAY: zwei volle Laeufe derselben Eingabe ⇒ identische
/// Bauplan-Klasse UND identischer packet_digest (replay-identisch;
/// die Kette selbst prueft das zusaetzlich intern, fail-closed).
#[test]
fn r_rig_replay_two_runs_identical_class() {
    let a = run_repo_intelligence(&numkit_fixture("mit")).expect("Lauf 1");
    let b = run_repo_intelligence(&numkit_fixture("mit")).expect("Lauf 2");
    assert_eq!(a.blueprint_class_hex, b.blueprint_class_hex);
    assert_eq!(a.packet.digest_hex(), b.packet.digest_hex());
    // Auch die versiegelten Bytes sind klassenidentisch (deterministische
    // Segmente, fixes created-Datum — dieselbe Disziplin wie die Seeds).
    assert_eq!(a.blueprint_sealed.core_root, b.blueprint_sealed.core_root);
}

/// N-RIG-1: proprietaere Lizenz ⇒ die CSA-Policy haelt VOR jeder
/// Destillation — kein Bundle, kein Bauplan, fail-closed. Die
/// Lizenz-/Policy-Gates sind WOERTLICH die bestehenden (nexus-policy,
/// unveraendert).
#[test]
fn n_rig_1_proprietary_license_blocks_before_distillation() {
    // Direkt am Einzug:
    let e = ingest_repo(&numkit_fixture("proprietary_no_reuse")).unwrap_err();
    assert!(e.id.contains("repointel_policy_blocked"));

    // Und an der vollen Kette: kein Koerper entsteht.
    match run_repo_intelligence(&numkit_fixture("proprietary_no_reuse")) {
        Err(RepoIntelError::Residue(r)) => {
            assert!(r.id.contains("repointel_policy_blocked"));
        }
        Err(other) => panic!("erwartet Policy-Residuum, war {other:?}"),
        Ok(_) => panic!("proprietaere Quelle darf NIE einen Bauplan ergeben"),
    }
}

/// N-RIG-2: eine offene architektonische Frage (numkit ohne
/// LICENSE-Datei) wird als SICHTBARES `repointel_decision_left_open`
/// gefuehrt (Warning) — sie blockiert die Siegelung nicht, sie
/// verschwindet aber auch nicht (sichtbare Wahrheit, Dokument 21).
#[test]
fn n_rig_2_open_decision_visible_not_blocking() {
    let out = run_repo_intelligence(&numkit_fixture("mit")).expect("Kette gruen");
    // Offene Entscheidung im Packet ...
    assert!(out.packet.has_open_decisions());
    // ... als sichtbares Warning-Residuum ...
    assert!(out
        .visible_residues
        .iter()
        .any(|r| r.id.contains("repointel_decision_left_open")));
    // ... und der Bauplan ist TROTZDEM gueltig versiegelt — als
    // ValidWithResidues (Warning sichtbar, kein Reject, kein Blocker).
    let vb = loom_verify::verify(&out.blueprint_sealed.bytes);
    assert_eq!(vb.verdict, loom_verify::Verdict::ValidWithResidues);
}

/// Wert-Beweis (Dokument 23 Track B: „versteht es beweisbar"): der
/// destillierte Bauplan ist NUTZBAR — sein GroundingPacket, an
/// `run_grounded_swe_task`-Vorpruefung (Dokument 21) angelegt, faengt
/// einen Diff, der die am Commit nachgewiesene Repo-Invariante bricht
/// (unwrap-frei in src/), OHNE dass ein Werkzeug angefasst wird.
#[test]
fn value_proof_blueprint_packet_catches_invariant_breaking_diff() {
    use cce_core::signature::sha256;
    use cce_swe::gates::rule_compliance_gate;
    use cce_swe::model::{DiffCandidate, DiffHunk, ProducedBy};

    let out = run_repo_intelligence(&numkit_fixture("mit")).expect("Kette gruen");

    // Ein Diff, der in src/ ein .unwrap() einfuehrt — bricht die
    // destillierte blocking-Regel `keep-unwrap-free-src`.
    let diff = DiffCandidate {
        base_snapshot_root: sha256(b"base"),
        hunks: vec![DiffHunk {
            path: "src/lib.rs".to_string(),
            unified_diff: "@@ -1,1 +1,2 @@\n fn f() {}\n+    let y = a.unwrap();\n".to_string(),
        }],
        rationale: "fuegt unwrap hinzu".to_string(),
        produced_by: ProducedBy::Operator {
            operator: "op".to_string(),
        },
    };
    let verdict = rule_compliance_gate(&diff, &out.packet);
    assert!(!verdict.allows(), "Bauplan-Regel muss den Diff fangen");
    assert!(verdict.residue().unwrap().id.contains("rule_violation"));
}
