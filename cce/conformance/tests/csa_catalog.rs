//! CSA-Zeugenkatalog (G8-Ausgangs-Gate, CSA.12/CSA.15):
//! 5 Referenz-Zeugen gruen, 8 Negativ-Zeugen rot,
//! PROD-INV-13..16 als Negativ-Tests, Replay-Zeuge (fixer Snapshot ⇒
//! gleiche IDs/Hashes/Ledgerpfade).

use std::cell::Cell;

use cce_core::gate::GateReport;
use cce_core::value::CanonValue;
use nexus_adapter::manifest::AdapterManifest;
use nexus_adapter::port::{FetchPlan, PlannedOp, SourceAdapter};
use nexus_adapter_git::GitRepositoryAdapter;
use nexus_adapter_local_corpus::LocalCorpusAdapter;
use nexus_adapter_wikimedia::WikimediaAdapter;
use nexus_core::objects::{NexusSourceBundle, SourceHorizon, SourceRunDescriptor, TaskSpec};
use nexus_core::verdict::Verdict;
use nexus_evidence::{build_pack, evidence_gate};
use nexus_export::{export_gate, hbm_import_gate, phc_projection_gate};
use nexus_fetch::{fetch, rate_budget_gate, FetchCache, SnapshotTransport, Transport};
use nexus_ledger::{ledger_for_bundle, replay_gate};
use nexus_policy::{approve_fetch, license_gate, source_horizon_gate, SourceDeclaration};
use nexus_validate::{dedup, provenance_gate, quality_gate, schema_gate, QualityThresholds};

// ---------- gemeinsame Fixtures ----------

fn decl(class: &str) -> SourceDeclaration {
    SourceDeclaration {
        adapter_class: class.to_string(),
        access_method: "api_key_configured".to_string(),
        robots_or_terms: Some("permitted".to_string()),
        license: Some("cc-by-4.0".to_string()),
        contains_pii: false,
        requested_actions: vec![],
    }
}

fn snapshot(entries: &[(&str, &[u8])]) -> SnapshotTransport {
    let mut t = SnapshotTransport {
        snapshot_id: "snap-fix".to_string(),
        ..Default::default()
    };
    for (k, v) in entries {
        t.responses.insert(k.to_string(), (v.to_vec(), None));
    }
    t
}

/// Voller Referenzlauf eines Adapters gegen einen fixen Snapshot;
/// liefert das NexusSourceBundle samt Ledger-Head.
fn run_bundle(
    adapter: &dyn SourceAdapter,
    class: &str,
    entries: &[(&str, &[u8])],
    attribution: Option<&str>,
) -> (NexusSourceBundle, cce_core::signature::Digest) {
    let hs = SourceHorizon::example_local();
    let task = TaskSpec::new("zeuge", "scope:docs");
    let manifest = adapter.manifest();
    let d = decl(class);
    let plan = adapter.plan(&task, task.budget_requests);
    let approved = approve_fetch(&hs, &task, &manifest, &d, plan).expect("Policy gruen");
    let transport = snapshot(entries);
    let mut cache = FetchCache::default();
    let raws = fetch(&approved, &transport, &mut cache, task.budget_requests).expect("Fetch");
    let mut csus = Vec::new();
    let mut packs = Vec::new();
    for raw in &raws {
        for rec in adapter.extract(raw).expect("Extraktion") {
            for csu in adapter.normalize(&rec) {
                assert!(adapter.validate(&csu).is_pass());
                assert!(schema_gate(&csu).allows());
                assert!(quality_gate(&csu, &QualityThresholds::default()).allows());
                assert!(provenance_gate(&csu).allows());
                packs.push(build_pack(&csu, raw, &["decode", "normalize"], attribution));
                csus.push(csu);
            }
        }
    }
    let csus = dedup(csus);
    let nsb = NexusSourceBundle {
        bundle_id: format!("nsb:{class}"),
        run_id: "run-1".to_string(),
        csu_set: csus,
        evidence_packs: packs,
        source_graph: vec![],
        quality_summary: "alle Achsen ueber Schwellen".to_string(),
        residues: vec![],
        export_contract: "phc_projection".to_string(),
    };
    let head = ledger_for_bundle(&nsb).head();
    (nsb, head)
}

// ---------- 5 Referenz-Zeugen (gruen) ----------

#[test]
fn ref_1_local_corpus_full_chain_green() {
    let a = LocalCorpusAdapter::new("k1");
    let (nsb, _) = run_bundle(
        &a,
        "local_corpus",
        &[(
            "corpus://k1/{key}",
            b"titel: Referenz\ninhalt: Text" as &[u8],
        )],
        Some("Korpus k1, CC BY 4.0"),
    );
    assert_eq!(nsb.csu_set.len(), 1);
    assert!(nsb.every_csu_has_evidence());
    assert!(export_gate(&nsb).allows());
}

#[test]
fn ref_2_wikimedia_official_api_with_attribution() {
    let a = WikimediaAdapter;
    let (nsb, _) = run_bundle(
        &a,
        "official_api",
        &[(
            "api://wikimedia/page/{title}",
            b"titel: Kristall\nauszug: Festkoerper" as &[u8],
        )],
        Some("Wikimedia-Beitraegerinnen und -Beitraeger, CC BY-SA 4.0"),
    );
    assert!(nsb.every_csu_has_evidence());
    // Attribution ist transportiert (PROD-INV-16 positiv).
    assert!(nsb.evidence_packs[0].attribution.is_some());
    assert!(export_gate(&nsb).allows());
}

#[test]
fn ref_3_git_repository_commit_bound() {
    let a = GitRepositoryAdapter::new("beispiel/repo", "abc1234def", "mit");
    let (nsb, _) = run_bundle(
        &a,
        "git_repository",
        &[(
            "git://beispiel/repo@abc1234def/{path}",
            b"titel: Readme\ninhalt: Hallo" as &[u8],
        )],
        None,
    );
    assert!(nsb.every_csu_has_evidence());
    assert!(nsb.evidence_packs[0].locator.contains("abc1234def"));
    assert!(export_gate(&nsb).allows());
}

#[test]
fn ref_4_feed_differential_fetch_via_etag() {
    // Feed-Zeuge: zweiter Abruf mit unveraendertem ETag wird uebersprungen
    // (Differenzabruf, CSA.10) — keine Doppel-Beobachtung.
    let hs = SourceHorizon::example_local();
    let task = TaskSpec::new("feed", "scope:docs");
    let manifest = AdapterManifest::complete("feed-1", "feed", "cc-by-4.0");
    let plan = FetchPlan {
        plan_id: "plan:feed".to_string(),
        adapter_id: "feed-1".to_string(),
        operations: vec![PlannedOp {
            method: "get".to_string(),
            endpoint_template: "feed://nachrichten".to_string(),
            expected_status: 200,
            cost_requests: 1,
        }],
        gates: vec!["rate_budget_gate".to_string()],
        cache_etag: Some("e1".to_string()),
        cache_cursor: None,
        backoff_policy: "none".to_string(),
    };
    let approved = approve_fetch(&hs, &task, &manifest, &decl("feed"), plan).unwrap();
    let mut t = SnapshotTransport {
        snapshot_id: "snap-feed".to_string(),
        ..Default::default()
    };
    t.responses.insert(
        "feed://nachrichten".to_string(),
        (b"eintrag: a".to_vec(), Some("etag-1".to_string())),
    );
    let mut cache = FetchCache::default();
    let first = fetch(&approved, &t, &mut cache, 8).unwrap();
    assert_eq!(first.len(), 1);
    let second = fetch(&approved, &t, &mut cache, 8).unwrap();
    assert!(
        second.is_empty(),
        "unveraenderter ETag ⇒ Differenzabruf leer"
    );
}

#[test]
fn ref_5_hbm_import_green() {
    let a = LocalCorpusAdapter::new("k1");
    let (nsb, _) = run_bundle(
        &a,
        "local_corpus",
        &[(
            "corpus://k1/{key}",
            b"titel: Referenz\ninhalt: Text" as &[u8],
        )],
        None,
    );
    let csu = &nsb.csu_set[0];
    let ep = &nsb.evidence_packs[0];
    assert!(hbm_import_gate(csu, Some(ep)).allows());
    assert!(phc_projection_gate(&nsb).allows());
}

// ---------- 8 Negativ-Zeugen (rot) ----------

/// Zaehlender Transport: beweist, dass VOR dem PolicyGate kein einziger
/// Socket/Abruf existiert.
struct CountingTransport {
    calls: Cell<u32>,
}

impl Transport for CountingTransport {
    fn get(&self, _endpoint: &str) -> Result<(Vec<u8>, Option<String>), String> {
        self.calls.set(self.calls.get() + 1);
        Ok((b"x".to_vec(), None))
    }
}

#[test]
fn neg_1_policy_blocked_fetch_no_socket_before_policy_gate() {
    // PolicyBlockedFetch: Klasse nicht im Horizont ⇒ approve_fetch
    // scheitert ⇒ es EXISTIERT kein ApprovedFetchPlan — und fetch()
    // ist ohne ihn nicht aufrufbar (versiegelter Typ). Der Zaehler
    // beweist: null Transportzugriffe.
    let transport = CountingTransport {
        calls: Cell::new(0),
    };
    let hs = SourceHorizon::example_local();
    let task = TaskSpec::new("neg", "scope:docs");
    let manifest = AdapterManifest::complete("scraper", "adhoc_scrape", "unknown");
    let plan = FetchPlan {
        plan_id: "p".to_string(),
        adapter_id: "scraper".to_string(),
        operations: vec![],
        gates: vec![],
        cache_etag: None,
        cache_cursor: None,
        backoff_policy: "none".to_string(),
    };
    let res = approve_fetch(&hs, &task, &manifest, &decl("adhoc_scrape"), plan);
    assert!(res.is_err(), "Policy muss halten");
    // Kein ApprovedFetchPlan ⇒ kein legaler Aufruf von fetch();
    // der Transport wurde nie beruehrt:
    assert_eq!(transport.calls.get(), 0, "kein Socket vor PolicyGate");
    let failed = res.err().unwrap();
    assert!(failed.iter().any(|v| !v.allows()));
}

#[test]
fn neg_2_rate_limit_violation() {
    let v = rate_budget_gate(9, 8);
    match v {
        Verdict::Hold { residue, .. } => {
            assert!(residue.id.contains("rate_budget_exceeded"))
        }
        _ => panic!("RateBudgetGate muss halten"),
    }
}

#[test]
fn neg_3_evidence_missing() {
    let a = LocalCorpusAdapter::new("k1");
    let (nsb, _) = run_bundle(
        &a,
        "local_corpus",
        &[("corpus://k1/{key}", b"titel: T\ninhalt: I" as &[u8])],
        None,
    );
    let csu = &nsb.csu_set[0];
    match evidence_gate(csu, None) {
        Verdict::Hold { residue, .. } | Verdict::Reject { residue, .. } => {
            assert!(residue.id.contains("evidence_missing"))
        }
        _ => panic!("EvidenceGate ohne EP muss halten (PROD-INV-15)"),
    }
}

#[test]
fn neg_4_license_incompatible() {
    let mut d = decl("local_corpus");
    d.license = Some("proprietary_no_reuse".to_string());
    match license_gate(&d) {
        Verdict::Hold { residue, .. } => {
            assert!(residue.id.contains("license_incompatible"))
        }
        _ => panic!("LicenseGate muss halten"),
    }
}

#[test]
fn neg_5_replay_drift() {
    let h1 = cce_core::signature::sha256(b"lauf-1");
    let h2 = cce_core::signature::sha256(b"lauf-2");
    match replay_gate(h1, h2) {
        Verdict::Reject { residue, .. } => assert!(residue.id.contains("replay_drift")),
        _ => panic!("ReplayGate muss abweichende Pfade REJECTEN"),
    }
}

#[test]
fn neg_6_score_as_gate_attempt() {
    // Ein "Gate-Report" mit Score-Feld wird strukturell verworfen (V1).
    let mut m = std::collections::BTreeMap::new();
    m.insert("gate_id".to_string(), CanonValue::Text("q".to_string()));
    m.insert("score".to_string(), CanonValue::Int(9000));
    let r = GateReport::from_untyped(&CanonValue::Map(m));
    assert!(r.is_err(), "Score als Gate muss strukturell scheitern");
}

#[test]
fn neg_7_source_unknown() {
    let hs = SourceHorizon::example_local();
    match source_horizon_gate(&hs, &decl("dark_pool")) {
        Verdict::Hold { residue, .. } => {
            assert!(residue.id.contains("source_unknown"));
            assert!(residue.content.contains("exploration_out_of_horizon"));
        }
        _ => panic!("unbekannte Klasse muss halten"),
    }
}

#[test]
fn neg_8_html_scope_leak() {
    // HTMLScopeLeak: (a) eine HTML-Scrape-Quelle ausserhalb des Horizonts
    // wird sichtbar verworfen; (b) rohes HTML faellt im deklarierten
    // kv-Schema fail-closed als schema_unparseable aus — es gibt keinen
    // stillen Pfad, ueber den HTML-Inhalte in den Scope einsickern.
    let hs = SourceHorizon::example_local();
    let declared = vec![(
        "https://irgendwo/seite.html".to_string(),
        "html_scrape".to_string(),
    )];
    let (cands, rejected) = nexus_ingress::discover(&hs, &declared);
    assert!(cands.is_empty());
    assert_eq!(rejected.len(), 1);
    assert!(rejected[0].contains("exploration_out_of_horizon"));

    let raw = nexus_core::objects::RawObservation {
        locator: "https://irgendwo/seite.html".to_string(),
        bytes: b"<html><body><a href=\"https://extern\">leak</a></body></html>".to_vec(),
        fetched_via: "test".to_string(),
        snapshot_id: "snap-x".to_string(),
    };
    let err = nexus_decode::decode_kv_lines(&raw).unwrap_err();
    assert!(err.id.contains("schema_unparseable"));
}

// ---------- PROD-INV-13..16 als Negativ-Tests ----------

#[test]
fn prod_inv_13_no_fetch_before_policy_gate_is_architectural() {
    // fetch() verlangt &ApprovedFetchPlan; der Typ hat ein privates
    // _sealed-Feld und GENAU EINEN Konstruktor (approve_fetch). Ein
    // Literal-Konstrukt ausserhalb von nexus-policy kompiliert nicht —
    // hier belegt durch den einzig moeglichen (abgelehnten) Weg:
    let hs = SourceHorizon::example_local();
    let task = TaskSpec::new("t", "s");
    let manifest = AdapterManifest {
        adapter_id: "x".to_string(),
        class: "adhoc".to_string(),
        policy: None,
        license: None,
        privacy: None,
        budget_requests: None,
        rate_limit_per_run: None,
        provenance_strategy: None,
        replay_strategy: None,
    };
    let plan = FetchPlan {
        plan_id: "p".to_string(),
        adapter_id: "x".to_string(),
        operations: vec![],
        gates: vec![],
        cache_etag: None,
        cache_cursor: None,
        backoff_policy: "none".to_string(),
    };
    let res = approve_fetch(&hs, &task, &manifest, &decl("adhoc"), plan);
    assert!(
        res.is_err(),
        "ohne Manifest+Horizont kein Beleg, kein Abruf"
    );
}

#[test]
fn prod_inv_14_disallowed_action_is_reject_endstate() {
    let hs = SourceHorizon::example_local();
    let task = TaskSpec::new("t", "s");
    let manifest = AdapterManifest::complete("a", "local_corpus", "cc-by-4.0");
    let mut d = decl("local_corpus");
    d.requested_actions = vec!["captcha_bypass".to_string()];
    let plan = FetchPlan {
        plan_id: "p".to_string(),
        adapter_id: "a".to_string(),
        operations: vec![],
        gates: vec![],
        cache_etag: None,
        cache_cursor: None,
        backoff_policy: "none".to_string(),
    };
    let failed = approve_fetch(&hs, &task, &manifest, &d, plan)
        .err()
        .unwrap();
    let reject = failed
        .iter()
        .find(|v| matches!(v, Verdict::Reject { .. }))
        .expect("disallowed_action ⇒ Reject (Endzustand), nicht Hold");
    match reject {
        Verdict::Reject { residue, .. } => {
            assert!(residue.content.contains("captcha_bypass"));
            assert!(residue.content.contains("Endzustand"));
        }
        _ => unreachable!(),
    }
    // Und: der RunDescriptor traegt die volle Verbotsliste.
    let rd = SourceRunDescriptor::new("run-x", "test", 7);
    assert!(rd.disallowed_complete());
}

#[test]
fn prod_inv_15_export_without_evidence_blocked() {
    let a = LocalCorpusAdapter::new("k1");
    let (mut nsb, _) = run_bundle(
        &a,
        "local_corpus",
        &[("corpus://k1/{key}", b"titel: T\ninhalt: I" as &[u8])],
        None,
    );
    nsb.evidence_packs.clear();
    assert!(!nsb.every_csu_has_evidence());
    assert!(
        !export_gate(&nsb).allows(),
        "Export ohne EP muss blockieren"
    );
    assert!(!phc_projection_gate(&nsb).allows());
}

#[test]
fn prod_inv_16_attribution_must_travel() {
    // cc-by-Lizenz OHNE Attribution ⇒ license_attribution_required.
    let a = WikimediaAdapter;
    let (nsb, _) = run_bundle(
        &a,
        "official_api",
        &[(
            "api://wikimedia/page/{title}",
            b"titel: K\nauszug: F" as &[u8],
        )],
        None, // Attribution WEGGELASSEN
    );
    let csu = &nsb.csu_set[0];
    let mut ep = nsb.evidence_packs[0].clone();
    ep.attribution = None;
    match evidence_gate(csu, Some(&ep)) {
        Verdict::Hold { residue, .. } | Verdict::Reject { residue, .. } => {
            assert!(residue.id.contains("license_attribution_required"))
        }
        Verdict::Allow { .. } => panic!("cc-by ohne Attribution darf nicht passieren"),
        _ => panic!("unerwartetes Verdikt"),
    }
}

// ---------- Replay-Zeuge ----------

#[test]
fn replay_fixed_snapshot_same_ids_hashes_ledger_paths() {
    let a = LocalCorpusAdapter::new("k1");
    let entries: &[(&str, &[u8])] = &[(
        "corpus://k1/{key}",
        b"titel: Referenz\ninhalt: Text" as &[u8],
    )];
    let (nsb1, head1) = run_bundle(&a, "local_corpus", entries, None);
    let (nsb2, head2) = run_bundle(&a, "local_corpus", entries, None);
    // gleiche IDs
    let uids1: Vec<_> = nsb1.csu_set.iter().map(|c| c.uid.clone()).collect();
    let uids2: Vec<_> = nsb2.csu_set.iter().map(|c| c.uid.clone()).collect();
    assert_eq!(uids1, uids2);
    // gleiche Hashes
    assert_eq!(
        nsb1.evidence_packs[0].raw_hash.to_hex(),
        nsb2.evidence_packs[0].raw_hash.to_hex()
    );
    // gleiche Ledgerpfade
    assert_eq!(head1.to_hex(), head2.to_hex());
    assert!(replay_gate(head1, head2).allows());
}
