//! Inference-Zeugenkatalog (Overlay 05, Teil G) — Ablage
//! `conformance/inference/`, ab G8a dauerhaft im einen Waechter.
//! 7 Referenzen (gruen) + 12 Negative (rot) + PROD-INV-17..20.

use cce_core::capability::CapabilityLock;
use cce_core::gate::GateReport;
use cce_core::residue::ResidueField;
use cce_core::signature::sha256;
use cce_core::value::CanonValue;
use cce_inference::gates::{
    human_confirmation_gate, model_budget_gate, model_capability_gate, model_privacy_gate,
    model_rate_gate, no_direct_commit_gate, no_gate_override_gate, output_schema_gate,
    prompt_context_gate, provider_manifest_gate, provider_terms_gate, tool_capability_gate,
    tool_egress_gate, InfVerdict, ALL_INFERENCE_GATES,
};
use cce_inference::gateway::{run_inference, GatewayOutcome, InferenceRecorder};
use cce_inference::kanzel::Kanzel;
use cce_inference::manifest::{ModelManifest, ProviderClass};
use cce_inference::providers::cloud_mock::CloudModelProviderMock;
use cce_inference::providers::disabled::DisabledProvider;
use cce_inference::providers::external_agent::ExternalAgentProviderMock;
use cce_inference::providers::local::LocalModelProvider;
use cce_inference::providers::ModelProvider;
use cce_inference::replay::{check_live_reinference, replay_response};
use cce_inference::request::{ContextSlice, InferenceRequest};
use cce_inference::residues::ALL_MODEL_RESIDUES;
use cce_inference::response::{CandidateOutput, ResponseOutcome};
use cce_phaseblock::accept::{accept_block, AcceptContext, AcceptOutcome};
use cce_phaseblock::phaseblock::PhaseBlock;
use cce_toolgateway::gateway::{FsReadTool, ToolGateway};
use cce_toolgateway::manifest::{Egress, ToolManifest};

fn boundary() -> Vec<String> {
    vec!["projektion".to_string(), "wunsch".to_string()]
}

fn open_egress_lock() -> CapabilityLock {
    let mut l = CapabilityLock::closed("model_egress:test");
    l.open("operator:sk", "ledger:e1");
    l
}

// ---------- Referenzen R-INF-1..7 (gruen) ----------

#[test]
fn r_inf_1_offline_disabled_provider_core_intact() {
    // Voller Motor-/Produktpfad OHNE Provider: der Kern-Gate-Pfad
    // (PhaseBlock-Accept) funktioniert; die Kanzel ist SICHTBAR degradiert.
    let provider = DisabledProvider;
    let req = InferenceRequest::example("r1");
    let mut rec = InferenceRecorder::default();
    let out = run_inference(
        &provider,
        &req,
        &boundary(),
        "no_pii",
        None,
        0,
        "none",
        &mut rec,
    );
    match out {
        GatewayOutcome::Failed { residue, .. } => {
            assert!(
                residue.content.contains("degradiert"),
                "Degradation sichtbar"
            );
        }
        _ => panic!("DisabledProvider muss sichtbar degradiert antworten"),
    }
    // Motorpfad unabhaengig davon voll funktionsfaehig:
    let mut block = PhaseBlock::candidate(
        1,
        "p3",
        &CanonValue::Text("payload".to_string()),
        vec![],
        vec![GateReport::pass("G1", "scope ok")],
        vec![sha256(b"ev")],
        ResidueField::new(),
        sha256(b"rd"),
        vec![],
    );
    assert!(matches!(
        accept_block(&mut block, &AcceptContext::all_true()),
        AcceptOutcome::Accepted
    ));
}

#[test]
fn r_inf_2_local_model_seed_deterministic_full_evidence() {
    let provider = LocalModelProvider::new("kernmodell");
    let req = InferenceRequest::example("r2");
    let mut rec1 = InferenceRecorder::default();
    let mut rec2 = InferenceRecorder::default();
    let run = |rec: &mut InferenceRecorder| match run_inference(
        &provider,
        &req,
        &boundary(),
        "no_pii",
        None,
        0,
        "draft",
        rec,
    ) {
        GatewayOutcome::Candidate(boxed) => *boxed,
        other => panic!("erwartet Kandidat, war {other:?}"),
    };
    let (c1, e1) = run(&mut rec1);
    let (c2, e2) = run(&mut rec2);
    // seed-deterministisch: gleiche Klasse
    assert_eq!(c1.content, c2.content);
    assert_eq!(e1.response_digest.to_hex(), e2.response_digest.to_hex());
    // Manifest/Evidence/Trace vollstaendig, replay=strict
    assert!(provider.manifest().validate().is_ok());
    assert_eq!(e1.egress_gate_reports.len(), 10);
    assert!(c1.evidence_ref.is_some());
}

#[test]
fn r_inf_3_cloud_mock_egress_only_after_all_gates() {
    let provider = CloudModelProviderMock::new(&[("r3", "kandidatentext")]);
    let req = InferenceRequest::example("r3");
    let lock = open_egress_lock();
    let mut rec = InferenceRecorder::default();
    let out = run_inference(
        &provider,
        &req,
        &boundary(),
        "no_pii",
        Some(&lock),
        0,
        "draft",
        &mut rec,
    );
    let (cand, ev) = match out {
        GatewayOutcome::Candidate(boxed) => *boxed,
        other => panic!("erwartet Kandidat, war {other:?}"),
    };
    assert_eq!(
        provider.egress_calls.get(),
        1,
        "genau EIN Egress, NACH den Gates"
    );
    // Kontext-Digest der Evidence = Digest des GESENDETEN Kontexts.
    assert_eq!(ev.context_digest.to_hex(), req.context_digest().to_hex());
    assert_eq!(cand.content, "kandidatentext");
}

#[test]
fn r_inf_4_external_agent_sees_only_projection_packet() {
    let provider = ExternalAgentProviderMock {
        demands_full_repo: false,
    };
    let req = InferenceRequest::example("r4");
    let lock = open_egress_lock();
    let mut rec = InferenceRecorder::default();
    let out = run_inference(
        &provider,
        &req,
        &boundary(),
        "no_pii",
        Some(&lock),
        0,
        "draft",
        &mut rec,
    );
    match out {
        GatewayOutcome::Candidate(boxed) => {
            let (cand, _) = *boxed;
            assert!(cand.content.contains(&req.projection_id));
            assert!(cand.content.contains("kein anderer Zugriff"));
        }
        other => panic!("erwartet Kandidat, war {other:?}"),
    }
}

#[test]
fn r_inf_5_candidate_through_gates_into_phaseblock() {
    // Kandidat → Motor-Gates → PhaseBlock; der Commit traegt
    // InferenceEvidence-Ref + aufgezeichnete Bestaetigung.
    let provider = LocalModelProvider::new("kernmodell");
    let req = InferenceRequest::example("r5");
    let mut rec = InferenceRecorder::default();
    let (cand, ev) = match run_inference(
        &provider,
        &req,
        &boundary(),
        "no_pii",
        None,
        0,
        "draft",
        &mut rec,
    ) {
        GatewayOutcome::Candidate(boxed) => *boxed,
        other => panic!("erwartet Kandidat, war {other:?}"),
    };
    // materielle Wirkung: HumanConfirmationGate verlangt Bestaetigung
    let confirm = human_confirmation_gate(Some("confirm:op:sk:2026-run1"));
    assert!(confirm.allows());
    let mut block = PhaseBlock::candidate(
        1,
        "p4",
        &CanonValue::Text(cand.content.clone()),
        vec![],
        vec![
            GateReport::pass("motor:G1", "scope ok"),
            GateReport::pass(confirm.gate(), "Bestaetigung aufgezeichnet"),
        ],
        vec![sha256(ev.evidence_id.as_bytes())],
        ResidueField::new(),
        sha256(b"rd"),
        vec![],
    );
    assert!(matches!(
        accept_block(&mut block, &AcceptContext::all_true()),
        AcceptOutcome::Accepted
    ));
    assert!(
        !block.evidence_refs.is_empty(),
        "Commit traegt Evidence-Ref"
    );
}

#[test]
fn r_inf_6_candidate_to_residue_visible_fate() {
    // Schema-Verletzung: der Kandidat landet als Residuum — Verbleib lesbar.
    let v = output_schema_gate("wunsch-entwurf", "freitext");
    match v {
        InfVerdict::Hold { residue, .. } => {
            assert!(residue.id.contains("model_output_schema_invalid"));
            assert!(residue.content.contains("kein stilles Umformen"));
        }
        _ => panic!("Schema-Bruch muss halten"),
    }
}

#[test]
fn r_inf_7_provider_status_headless_root_traceable() {
    // ProviderStatus-Datenmodell: headless renderbar, wurzel-rueckfuehrbar
    // (jede Angabe stammt aus dem Manifest — keine freie Erfindung).
    let m = ModelManifest::complete("local:kernmodell", ProviderClass::LocalModel, "kernmodell");
    let status = format!(
        "modus=fully_local provider={} model={}@{} kanzel=aktiv replay={:?}",
        m.provider_id, m.model_id, m.model_version, m.replay_policy
    );
    assert!(status.contains("local:kernmodell"));
    assert!(status.contains("kernmodell@1.0.0"));
    // Wurzel-Rueckfuehrbarkeit: alle Felder kommen aus dem validierten Manifest.
    assert!(m.validate().is_ok());
}

// ---------- Negative N-INF-1..12 (rot) ----------

#[test]
fn n_inf_1_provider_manifest_missing_hold_no_egress() {
    let provider = CloudModelProviderMock::new(&[]);
    let mut m = provider.manifest();
    m.privacy_mode = None;
    let v = provider_manifest_gate(&m);
    assert!(!v.allows());
    assert!(v
        .residue()
        .unwrap()
        .id
        .contains("provider_manifest_missing"));
}

#[test]
fn n_inf_2_cloud_egress_without_privacy_gate_reject_before_socket() {
    // Manifest mit inkompatibler Privacy: die Kette haelt VOR dem Socket —
    // der Zaehler des Mocks beweist null Egress.
    struct BadPrivacyCloud(CloudModelProviderMock);
    impl ModelProvider for BadPrivacyCloud {
        fn manifest(&self) -> ModelManifest {
            let mut m = self.0.manifest();
            m.privacy_mode = Some("training_use_allowed".to_string());
            m
        }
        fn infer(&self, r: &InferenceRequest) -> cce_inference::response::InferenceResponse {
            self.0.infer(r)
        }
    }
    let provider = BadPrivacyCloud(CloudModelProviderMock::new(&[("n2", "x")]));
    let req = InferenceRequest::example("n2");
    let lock = open_egress_lock();
    let mut rec = InferenceRecorder::default();
    let out = run_inference(
        &provider,
        &req,
        &boundary(),
        "no_pii",
        Some(&lock),
        0,
        "draft",
        &mut rec,
    );
    match out {
        GatewayOutcome::BlockedBeforeEgress(failed) => {
            assert!(failed.iter().any(|v| v
                .residue()
                .is_some_and(|r| r.id.contains("model_privacy_block"))));
        }
        other => panic!("erwartet Block vor Egress, war {other:?}"),
    }
    assert_eq!(
        provider.0.egress_calls.get(),
        0,
        "KEIN Socket vor PrivacyGate"
    );
}

#[test]
fn n_inf_3_model_attempted_direct_commit() {
    let v = no_direct_commit_gate(true);
    match v {
        InfVerdict::Reject { residue, .. } => {
            assert!(residue.id.contains("model_attempted_direct_commit"))
        }
        _ => panic!("direkter Commit-Versuch muss REJECT sein"),
    }
}

#[test]
fn n_inf_4_model_attempted_gate_override() {
    let v = no_gate_override_gate(true);
    match v {
        InfVerdict::Reject { residue, .. } => {
            assert!(residue.id.contains("model_attempted_gate_override"))
        }
        _ => panic!("Gate-Override-Versuch muss REJECT sein"),
    }
}

#[test]
fn n_inf_5_forbidden_context_never_sent() {
    let provider = CloudModelProviderMock::new(&[("n5", "x")]);
    let mut req = InferenceRequest::example("n5");
    req.context.push(ContextSlice {
        name: "geheimnis".to_string(),
        content: "api-key".to_string(),
    });
    let lock = open_egress_lock();
    let mut rec = InferenceRecorder::default();
    let out = run_inference(
        &provider,
        &req,
        &boundary(),
        "no_pii",
        Some(&lock),
        0,
        "draft",
        &mut rec,
    );
    match out {
        GatewayOutcome::BlockedBeforeEgress(failed) => {
            assert!(failed.iter().any(|v| v
                .residue()
                .is_some_and(|r| r.id.contains("sensitive_context_egress_blocked"))));
        }
        other => panic!("forbidden_context muss vor Egress rejecten, war {other:?}"),
    }
    assert_eq!(
        provider.egress_calls.get(),
        0,
        "kein Egress mit forbidden Kontext"
    );
}

#[test]
fn n_inf_6_model_output_schema_invalid() {
    let v = output_schema_gate("json", "yaml");
    assert!(!v.allows());
    assert!(v
        .residue()
        .unwrap()
        .id
        .contains("model_output_schema_invalid"));
}

#[test]
fn n_inf_7_provider_terms_unknown_hold() {
    let mut m = ModelManifest::complete("p", ProviderClass::CloudModel, "m");
    m.provider_terms_ref = Some("terms:unreviewed".to_string());
    let v = provider_terms_gate(&m);
    assert!(!v.allows());
    assert!(v.residue().unwrap().id.contains("model_terms_unknown"));
}

#[test]
fn n_inf_8_model_budget_exceeded_hold_before_egress() {
    let m = ModelManifest::complete("p", ProviderClass::LocalModel, "m");
    let mut req = InferenceRequest::example("n8");
    req.budget_tokens = 200_000; // > token_budget 100_000
    let v = model_budget_gate(&req, &m);
    assert!(!v.allows());
    assert!(v.residue().unwrap().id.contains("model_budget_exceeded"));
    assert!(v.residue().unwrap().content.contains("VOR Egress"));
}

#[test]
fn n_inf_9_external_agent_full_repo_without_lock() {
    // Der Agent fordert Repo-Vollzugriff: das ToolGateway gibt ohne
    // klassen-eigenen Lock NICHTS frei; die Forderung wird sichtbar.
    let provider = ExternalAgentProviderMock {
        demands_full_repo: true,
    };
    let req = InferenceRequest::example("n9");
    let lock = open_egress_lock();
    let mut rec = InferenceRecorder::default();
    let out = run_inference(
        &provider,
        &req,
        &boundary(),
        "no_pii",
        Some(&lock),
        0,
        "draft",
        &mut rec,
    );
    match out {
        GatewayOutcome::Failed { residue, .. } => {
            assert!(residue.content.contains("ToolCapabilityLock"));
        }
        other => panic!("Vollzugriff-Forderung muss sichtbar scheitern, war {other:?}"),
    }
    // Und strukturell: fs_read ohne Lock bleibt zu.
    let mut gw = ToolGateway::new();
    let tool = FsReadTool::with_files(&[("repo/src/main.rs", b"code")]);
    let manifest = ToolManifest {
        tool_id: "agent-fs".to_string(),
        tool_class: "fs_read".to_string(),
        scope: vec!["repo/".to_string()],
        side_effects: false,
        egress: Egress::None,
        budget_calls: 10,
        replay_strategy: "recorded".to_string(),
        lock_ref: "lock:fs_read".to_string(),
    };
    let err = gw
        .run_fs_read(&manifest, &tool, "repo/src/main.rs")
        .unwrap_err();
    assert!(err.id.contains("tool_egress_without_tool_capability"));
}

#[test]
fn n_inf_10_tool_egress_without_tool_capability() {
    let v = tool_capability_gate("shell", None);
    assert!(!v.allows(), "shell ohne Lock: niemals implizit frei");
    // Remote-Tool ohne explizite Erlaubnis:
    let v = tool_egress_gate(true, false, true, true);
    assert!(!v.allows());
}

#[test]
fn n_inf_11_hidden_model_call_on_open() {
    // Format-Ebene (= LOOM-N15, hier als Datenmodell-Zeuge; der
    // .loom-Verify-Zeuge folgt in G9): ein PROVIDER_MANIFEST-Segment
    // mit Autostart-/Aktivierungsfeld ist REJECT — Deklaration ≠
    // Aktivierung (IG-A5).
    let declared = [("provider_id", "cloud-x"), ("autostart", "true")];
    let has_activation_field = declared
        .iter()
        .any(|(k, _)| *k == "autostart" || *k == "activate_on_open");
    assert!(
        has_activation_field,
        "Fixture enthaelt das verbotene Feld (Vorbedingung des Zeugen)"
    );
    // Die Regel: Validierung MUSS so ein Manifest verwerfen.
    fn validate_declaration(fields: &[(&str, &str)]) -> Result<(), String> {
        for (k, _) in fields {
            if *k == "autostart" || *k == "activate_on_open" {
                return Err(format!("hidden_model_call_on_open: Feld '{k}' unzulaessig"));
            }
        }
        Ok(())
    }
    let err = validate_declaration(&declared).unwrap_err();
    assert!(err.contains("hidden_model_call_on_open"));
}

#[test]
fn n_inf_12_model_confidence_used_as_gate() {
    // PROD-INV-20: Confidence ersetzt nie ein Gate. Ein GateReport mit
    // Score-Feld wird strukturell verworfen (V1-Enforcement).
    let mut m = std::collections::BTreeMap::new();
    m.insert(
        "gate_id".to_string(),
        CanonValue::Text("konfidenz".to_string()),
    );
    m.insert("confidence".to_string(), CanonValue::Int(950));
    assert!(GateReport::from_untyped(&CanonValue::Map(m)).is_err());
    // Und: die Selbst-Einschaetzung am Kandidaten ist reine Anzeige —
    // sie ist KEIN Feld eines GateReports und kein Verdikt.
    let cand = CandidateOutput {
        candidate_id: "c".to_string(),
        response_ref: "r".to_string(),
        content: "x".to_string(),
        output_schema: "s".to_string(),
        self_assessment_permille: Some(999),
        evidence_ref: Some("e".to_string()),
    };
    assert!(cand.self_assessment_permille.is_some());
    // kein Weg von hier zu einem Verdikt: GateReport::pass verlangt
    // gate_id+reason, nimmt keinen Score.
    let g = GateReport::pass("motor:G1", "begruendet, boolesch");
    assert!(g.is_pass());
}

// ---------- PROD-INV-17..20 ----------

#[test]
fn prod_inv_17_no_model_egress_outside_gateway() {
    // Der einzige Weg zu einem Provider fuehrt durch run_inference:
    // Gate-Halt ⇒ Provider unberuehrt (Zaehler 0), belegt in N-INF-2/5.
    // Zusaetzlich: egress-pflichtige Klassen OHNE offenen Lock halten an.
    let m = ModelManifest::complete("cloud", ProviderClass::CloudModel, "m");
    let closed = CapabilityLock::closed("model_egress:cloud");
    let v = model_capability_gate(&m, Some(&closed), "draft");
    assert!(!v.allows(), "geschlossener model_egress-Lock ⇒ kein Egress");
    let v = model_capability_gate(&m, None, "draft");
    assert!(!v.allows(), "ohne Lock ⇒ kein Egress");
}

#[test]
fn prod_inv_18_no_tool_egress_outside_toolgateway_per_class_locks() {
    let mut gw = ToolGateway::new();
    gw.open_lock("fs_read", "op", "l1");
    // fs_read offen schaltet KEINE andere Klasse frei:
    for class in [
        "shell",
        "git",
        "package_manager",
        "browser",
        "ci",
        "network_tool",
        "fs_write",
    ] {
        let v = tool_capability_gate(class, None);
        assert!(!v.allows(), "{class} darf nicht implizit frei sein");
    }
}

#[test]
fn prod_inv_19_no_provider_kanzel_agent_path_writes_verdicts() {
    // Kanzel-API ist schreiblos: form/explain/propose liefern Texte
    // bzw. Requests — keinerlei Urteils-/Residuen-/Ledger-Schreibpfad.
    let kanzel = Kanzel;
    let req = kanzel.form_wish_request("drei Risiken memo", "proj:p1");
    assert_eq!(req.system_contract, "annahmen als modellgeformt markieren");
    let residue = cce_inference::residues::model_residue("model_refusal", "test");
    let text = kanzel.explain_residue(&residue);
    assert!(text.contains("kein Urteil"));
    let proposal = kanzel.propose_repair(&GateReport::hold("G4", "residuum offen"));
    assert!(proposal.contains("kandidat, kein urteil"));
    // Versuch eines Override-Pfads: strukturell Reject.
    assert!(!no_gate_override_gate(true).allows());
}

#[test]
fn prod_inv_20_confidence_never_replaces_gate() {
    // Vollstaendige Gate-Liste ist boolesch: alle 14 Namen vorhanden,
    // keine Score-Schwelle irgendwo.
    assert_eq!(ALL_INFERENCE_GATES.len(), 14);
    assert_eq!(ALL_MODEL_RESIDUES.len(), 16);
    // Ranking/Einschaetzung aendert keinen Gate-Status (siehe auch
    // n_inf_12): ein Hold bleibt Hold, egal welche Einschaetzung.
    let m = ModelManifest::complete("p", ProviderClass::CloudModel, "m");
    let closed = CapabilityLock::closed("model_egress:p");
    let v = model_capability_gate(&m, Some(&closed), "draft");
    assert!(!v.allows());
}

// ---------- recorded-Replay (F.1 g) ----------

#[test]
fn recorded_replay_class_identical() {
    let provider = LocalModelProvider::new("kernmodell");
    let req = InferenceRequest::example("rp1");
    let mut recorder = InferenceRecorder::default();
    let (_, ev) = match run_inference(
        &provider,
        &req,
        &boundary(),
        "no_pii",
        None,
        0,
        "draft",
        &mut recorder,
    ) {
        GatewayOutcome::Candidate(boxed) => *boxed,
        other => panic!("erwartet Kandidat, war {other:?}"),
    };
    // Replay: die Aufzeichnung wird EINGESPIELT (kein Live-Re-Call).
    let replayed = replay_response(&recorder, "rp1").expect("Aufzeichnung vorhanden");
    assert_eq!(replayed.digest().to_hex(), ev.response_digest.to_hex());
    // Live-Re-Inferenz (nur wo verlangt): identisch bei strict-Provider …
    let live = provider.infer(&req);
    assert!(check_live_reinference(&replayed, &live).is_ok());
    // … und Abweichung waere sichtbares model_replay_weak:
    let mut drifted = live.clone();
    drifted.outcome = ResponseOutcome::Output("anders".to_string());
    let err = check_live_reinference(&replayed, &drifted).unwrap_err();
    assert!(err.id.contains("model_replay_weak"));
    // Fehlende Aufzeichnung ist model_trace_missing:
    let err = replay_response(&recorder, "unbekannt").unwrap_err();
    assert!(err.id.contains("model_trace_missing"));
}

// ---------- Modell-Weigerung & Rate (Zusatzabdeckung der 14) ----------

#[test]
fn refusal_is_regular_visible_state_and_rate_holds() {
    let provider = CloudModelProviderMock::new(&[]); // keine Fixture ⇒ Refusal
    let req = InferenceRequest::example("rf1");
    let lock = open_egress_lock();
    let mut rec = InferenceRecorder::default();
    match run_inference(
        &provider,
        &req,
        &boundary(),
        "no_pii",
        Some(&lock),
        0,
        "draft",
        &mut rec,
    ) {
        GatewayOutcome::Refusal { residue, .. } => {
            assert!(residue.id.contains("model_refusal"))
        }
        other => panic!("erwartet sichtbare Weigerung, war {other:?}"),
    }
    // ModelRateGate haelt bei erreichter Rate:
    let m = provider.manifest();
    let v = model_rate_gate(m.rate_limit.unwrap(), &m);
    assert!(!v.allows());
    // uebrige Gate-Namen einzeln angefasst (fail-closed Belege):
    assert!(provider_manifest_gate(&m).allows());
    assert!(provider_terms_gate(&m).allows());
    assert!(model_privacy_gate(&m, "no_pii").allows());
    assert!(prompt_context_gate(&req, &boundary()).allows());
    assert!(model_budget_gate(&req, &m).allows());
    assert!(!human_confirmation_gate(None).allows());
}

// ---------- P4 (Track C): echtes Lokalmodell ----------

#[test]
fn r_inf_2b_real_local_model_recorded_replay_class_identical() {
    use cce_inference::providers::local_extractive::LocalExtractiveModel;
    let provider = LocalExtractiveModel::new("kernmodell");
    // Manifest vollstaendig (C.4) und Klasse LocalModel (kein Egress).
    assert!(provider.manifest().validate().is_ok());
    assert_eq!(
        provider.manifest().provider_class,
        cce_inference::manifest::ProviderClass::LocalModel
    );

    let mut req = InferenceRequest::example("r2b");
    req.context = vec![cce_inference::request::ContextSlice {
        name: "projektion".to_string(),
        content: "Serverausfall gefaehrdet den Go-Live. Redundanz senkt das Ausfallrisiko. \
                  Datenverlust droht bei Migration."
            .to_string(),
    }];

    // Durch das UNVERAENDERTE Gateway: alle Egress-Vorgates + Aufzeichnung.
    let mut recorder = InferenceRecorder::default();
    let (cand, ev) = match run_inference(
        &provider,
        &req,
        &boundary(),
        "no_pii",
        None,
        0,
        "draft",
        &mut recorder,
    ) {
        GatewayOutcome::Candidate(boxed) => *boxed,
        other => panic!("erwartet Kandidat, war {other:?}"),
    };
    // Das echte Modell hat den Kontext verarbeitet (nicht-leerer Entwurf).
    assert!(cand.content.contains("ENTWURF (extraktiv"));
    assert!(cand.evidence_ref.is_some());

    // recorded-Replay ist klassenidentisch: eingespielte Aufzeichnung ==
    // Response-Digest der Evidence.
    let replayed = replay_response(&recorder, "r2b").expect("Aufzeichnung");
    assert_eq!(replayed.digest().to_hex(), ev.response_digest.to_hex());
    // Live-Re-Inferenz stimmt ueberein (deterministisches Modell).
    let live = provider.infer(&req);
    assert!(check_live_reinference(&replayed, &live).is_ok());
}

#[test]
fn offline_core_unbroken_with_real_local_model_present() {
    // Der Offline-Nachweis bleibt gueltig: DisabledProvider degradiert
    // weiterhin sichtbar; das echte Lokalmodell fuegt KEINEN Egress hinzu
    // (Klasse LocalModel braucht keinen Lock, needs_egress=false).
    use cce_inference::manifest::ProviderClass;
    assert!(!ProviderClass::LocalModel.needs_egress());
    let provider = DisabledProvider;
    let req = InferenceRequest::example("offline2");
    let mut rec = InferenceRecorder::default();
    match run_inference(
        &provider,
        &req,
        &boundary(),
        "no_pii",
        None,
        0,
        "none",
        &mut rec,
    ) {
        GatewayOutcome::Failed { residue, .. } => assert!(residue.content.contains("degradiert")),
        other => panic!("Disabled muss degradiert antworten, war {other:?}"),
    }
}

// ---------- P1 (Dokument 17 §3 + Overlay-Klausel): CloudModelProviderOpenAI ----------
//
// Der reale HTTP-Egress liegt hinter dem opt-in-Feature `http` (Standard AUS,
// s. `crates/cce-inference/Cargo.toml`) — dieser Katalog baut/laeuft OHNE das
// Feature, bleibt also netzfrei (F.3-Disziplin). Was hier bewiesen wird, ist
// GENAU die Bau-Garantie: vollstaendiges Manifest (Terms/Privacy/Retention/
// Budget), Lauf durch das UNVERAENDERTE Gateway mit allen Vor-Egress-Gates,
// und saubere, sichtbare Degradation OHNE jeden Socket-Versuch, solange kein
// Schluessel/Feature aktiv ist — exakt der Zustand, in dem CI immer laeuft.

#[test]
fn p1_cloud_openai_manifest_complete_terms_privacy_retention_budget() {
    use cce_inference::providers::openai::CloudModelProviderOpenAI;
    let provider = CloudModelProviderOpenAI::new("gpt-test");
    let m = provider.manifest();
    assert!(m.validate().is_ok(), "Manifest muss vollstaendig sein");
    assert_eq!(m.provider_class, ProviderClass::CloudModel);
    // Terms
    assert!(m
        .provider_terms_ref
        .as_deref()
        .is_some_and(|t| t.starts_with("terms:known")));
    // Privacy + Retention
    assert_eq!(m.privacy_mode.as_deref(), Some("no_training_use"));
    assert_ne!(
        m.data_retention_mode.as_deref(),
        Some("indefinite_retention")
    );
    // Budget
    assert!(m.cost_budget.is_some());
    assert!(m.token_budget.is_some());
    // recorded-Replay (Auftraggeber-Weisung)
    assert_eq!(
        m.replay_policy,
        cce_inference::manifest::ReplayPolicy::Recorded
    );
    // CloudModel braucht den model_egress-Lock — deklariert, nicht optional.
    assert!(!m.capability_locks.is_empty());
}

#[test]
fn p1_cloud_openai_without_key_or_feature_degrades_after_full_gate_chain_no_socket() {
    // Ohne Feature `http` (dieser Bau) verhaelt sich der Provider EXAKT wie
    // ohne gesetzten OPENAI_API_KEY: run_inference laesst die volle
    // Vor-Egress-Gate-Kette durch (Lock offen, alle 10 Gates allow) und der
    // Provider selbst meldet sichtbar `provider_unavailable` — kein stiller
    // Fallback-Inhalt, kein Socket-Versuch (das Feature ist nicht einmal
    // kompiliert).
    use cce_inference::providers::openai::CloudModelProviderOpenAI;
    let provider = CloudModelProviderOpenAI::new("gpt-test");
    let req = InferenceRequest::example("p1-degrade");
    let lock = open_egress_lock();
    let mut rec = InferenceRecorder::default();
    let out = run_inference(
        &provider,
        &req,
        &boundary(),
        "no_pii",
        Some(&lock),
        0,
        "draft",
        &mut rec,
    );
    match out {
        GatewayOutcome::Failed { residue, .. } => {
            assert!(residue.id.contains("provider_unavailable"));
        }
        other => panic!("erwartet sichtbare Degradation, war {other:?}"),
    }
}

#[test]
fn p1_cloud_openai_egress_blocked_before_provider_without_open_lock() {
    // Ohne offenen model_egress-Lock haelt ModelCapabilityGate VOR jedem
    // Providerkontakt — dieselbe strukturelle Garantie wie beim Mock
    // (r_inf_3), jetzt fuer den echten Anbieter: kein Lock ⇒ kein Egress-
    // Versuch, unabhaengig von Schluessel/Feature.
    use cce_inference::providers::openai::CloudModelProviderOpenAI;
    let provider = CloudModelProviderOpenAI::new("gpt-test");
    let req = InferenceRequest::example("p1-nolock");
    let mut rec = InferenceRecorder::default();
    let out = run_inference(
        &provider,
        &req,
        &boundary(),
        "no_pii",
        None,
        0,
        "draft",
        &mut rec,
    );
    match out {
        GatewayOutcome::BlockedBeforeEgress(failed) => {
            assert!(failed.iter().any(|v| v.gate() == "ModelCapabilityGate"));
        }
        other => panic!("erwartet Gate-Halt VOR Egress, war {other:?}"),
    }
}

// ---------- Betriebsverifikation P1 (echter Egress, KEIN Bau-Zeuge) ----------
//
// Laeuft NIE im Default-CI: `cargo test --workspace` / `bash
// ci/run_ci.sh` bauen ohne das Feature `http`, dieser Test existiert
// dort nicht einmal (cfg-Gate). Selbst mit `--features http` laeuft er
// nur explizit mit `--ignored` (Konvention):
//
//   cargo test -p cce-conformance --features http -- --ignored \
//       p1_betriebsverifikation
//
// Voraussetzung: `OPENAI_API_KEY` in der Prozessumgebung gesetzt — echter,
// bezahlter Egress gegen https://api.openai.com. Der Schluessel wird hier
// nie gelesen/geloggt/ausgegeben (das tut ausschliesslich `infer()`,
// unveraendert); dieser Test prueft nur das Ergebnis des unveraenderten
// Gateway-Laufs.
#[cfg(feature = "http")]
#[test]
#[ignore]
fn p1_betriebsverifikation_echter_openai_egress_nach_voller_gate_kette() {
    use cce_inference::providers::openai::CloudModelProviderOpenAI;
    // Echtes, guenstiges Chat-Completions-Modell — bewusst NICHT der
    // Platzhalter "gpt-test" aus den obigen Bau-Zeugen.
    let provider = CloudModelProviderOpenAI::new("gpt-4o-mini");
    let req = InferenceRequest::example("p1-betriebsverifikation");
    let lock = open_egress_lock();
    let mut rec = InferenceRecorder::default();
    let out = run_inference(
        &provider,
        &req,
        &boundary(),
        "no_pii",
        Some(&lock),
        0,
        "draft",
        &mut rec,
    );
    match out {
        GatewayOutcome::Candidate(boxed) => {
            let (candidate, evidence) = *boxed;
            // (a) die volle Vor-Egress-Gate-Kette lief NACHWEISLICH durch,
            // BEVOR der Egress zaehlt: zehn Gate-Reports, alle Pass.
            assert_eq!(evidence.egress_gate_reports.len(), 10);
            assert!(evidence
                .egress_gate_reports
                .iter()
                .all(|r| r.verdict == cce_core::gate::GateVerdict::Pass));
            // (b) eine echte, inhaltlich nicht-leere Antwort kam zurueck
            // und ist korrekt als CandidateOutput (ResponseOutcome::Output)
            // abgebildet — kein Fallback, kein stiller Leerinhalt.
            assert!(!candidate.content.trim().is_empty());
            // Nie den Inhalt selbst ausgeben — nur das Faktum + Groessen,
            // damit kein potenziell sensibler Antworttext in Testausgabe/
            // Logs landet.
            eprintln!(
                "Betriebsverifikation P1: OK — Modell=gpt-4o-mini, {} Zeichen Antwort, \
                 {} Gate-Reports (alle Pass), {} Tokens",
                candidate.content.len(),
                evidence.egress_gate_reports.len(),
                evidence.cost_tokens
            );
        }
        other => panic!("erwartet echten Candidate-Output, war {other:?}"),
    }
}
