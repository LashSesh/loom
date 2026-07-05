//! Die Versiegelung (Dokument 23 Track B): ZWEI Container.
//!
//! 1. Der Quell-Container (bestehende Klasse `"source"`): das
//!    NexusSourceBundle des Einzugs (CSA_NSB + EVIDENCE) — die
//!    Quell-Evidence als eigenstaendig pruefbarer Koerper.
//! 2. Der Bauplan-Container (NEUE Klasse `"blueprint"`, additiv wie
//!    seinerzeit `"repo"`/`"benchmark"`): Blueprint-Kristall(e) +
//!    Regeln + offene Entscheidungen + `cites` AUF den
//!    Quell-Container — `verify == Valid`, replay-identisch.
//!
//! Offene Entscheidungen stehen SICHTBAR im RESIDUE-Segment
//! (`repointel_decision_left_open`, Warning) — sichtbare Wahrheit,
//! kein leeres Feld durch Weglassen.

use crate::distill::DistilledRepo;
use crate::ingest::{IngestedRepo, RepoIntelInput};
use cce_core::signature::{sha256, Digest};
use cce_swe::grounding::{DecisionSlot, GroundingPacket, RuleAtom};
use loom_canon::Cv;
use loom_cites::{cites_field, external_citations_hex, CiteEntry, CiteKind};
use loom_codec::{seal_canonical, PackError, Sealed, Segment};
use loom_format::{
    KIND_CANON_DESC, KIND_CL_SUBSTRATE, KIND_CSA_NSB, KIND_EVIDENCE, KIND_HBM, KIND_LEDGER,
    KIND_MANIFEST, KIND_REPLAY_MANIFEST, KIND_RESIDUE,
};

#[derive(Debug)]
pub enum WorkbodyError {
    Pack(PackError),
}

fn seg(kind: u16, v: &Cv) -> Segment {
    Segment::canonical(kind, v).expect("kanonisches Segment")
}

fn canon_desc_segment() -> Segment {
    Segment {
        kind: KIND_CANON_DESC,
        seg_flags: 0,
        payload: Cv::Text(loom_canon::CANON_RULES_TEXT.into())
            .encode()
            .unwrap(),
        deps: vec![],
    }
}

/// Manifest-Basis. `closed`: nur ein Koerper MIT Abschlussbeweis im
/// LEDGER darf `claims.closed=true` tragen (L2 `claim_over_evidence`);
/// der Quell-Container traegt ehrlich `false`. `residue_kinds`: die
/// Manifest-Zusammenfassung MUSS der RESIDUE-Segment-Zaehlung
/// entsprechen (L2 `residue_summary_mismatch`, N6).
fn manifest_base(
    title: &str,
    class: &str,
    closed: bool,
    residue_kinds: &[String],
    external: Vec<String>,
    origin: Cv,
) -> Cv {
    Cv::map(vec![
        ("title", Cv::Text(title.to_string())),
        ("container_class", Cv::Text(class.to_string())),
        (
            "domain_refs",
            Cv::Array(vec![Cv::Text("repointelligence".into())]),
        ),
        ("scale", Cv::Uint(1)),
        ("pl_level", Cv::Text("PL2".into())),
        ("claims", Cv::map(vec![("closed", Cv::Bool(closed))])),
        ("origin", origin),
        (
            "profiles_required",
            Cv::Array(vec![Cv::Text(class.to_string())]),
        ),
        ("profiles_optional", Cv::Array(vec![])),
        (
            "residue_summary",
            Cv::map(vec![
                ("count", Cv::Uint(residue_kinds.len() as u64)),
                (
                    "kinds",
                    Cv::Array(residue_kinds.iter().map(|k| Cv::Text(k.clone())).collect()),
                ),
            ]),
        ),
        (
            "capability_declarations",
            Cv::Array(vec![Cv::Text("read_segment".into())]),
        ),
        ("license_summary", Cv::Text("cc0".into())),
        (
            "created",
            Cv::Tag(0, Box::new(Cv::Text("2026-07-05T00:00:00Z".into()))),
        ),
        (
            "external_citations",
            Cv::Array(external.into_iter().map(Cv::Text).collect()),
        ),
    ])
}

// ---------------- 1. Quell-Container (Klasse "source") ----------------

/// Siegelt das NexusSourceBundle des Einzugs als `"source"`-Workbody:
/// CSA_NSB (csu_uids + evidence_for je CSU, N5) + EVIDENCE (die
/// EvidencePacks im Detail).
pub fn seal_source_workbody(
    input: &RepoIntelInput,
    ingested: &IngestedRepo,
) -> Result<Sealed, WorkbodyError> {
    let manifest = manifest_base(
        &format!("Quell-Evidence {} @ {}", input.repo_id, input.commit_sha),
        "source",
        // Kein LEDGER im Quell-Container ⇒ ehrlich NICHT "closed".
        false,
        &[],
        vec![],
        Cv::map(vec![
            ("tool", Cv::Text("cce-repointel-0.1".into())),
            ("adapter", Cv::Text("git_repository".into())),
            ("repo", Cv::Text(input.repo_id.clone())),
            ("commit", Cv::Text(input.commit_sha.clone())),
            ("observation", Cv::Text("structural_kv_v1".into())),
            ("declared_license", Cv::Text(input.declared_license.clone())),
        ]),
    );

    let csu_uids: Vec<Cv> = ingested
        .nsb
        .csu_set
        .iter()
        .map(|c| Cv::Text(c.uid.clone()))
        .collect();
    let evidence_for: Vec<Cv> = ingested
        .nsb
        .evidence_packs
        .iter()
        .map(|p| Cv::Text(p.record_id.clone()))
        .collect();
    let nsb_cv = Cv::map(vec![
        ("bundle_id", Cv::Text(ingested.nsb.bundle_id.clone())),
        ("run_id", Cv::Text(ingested.nsb.run_id.clone())),
        ("csu_uids", Cv::Array(csu_uids)),
        ("evidence_for", Cv::Array(evidence_for)),
        ("ledger_head", Cv::Text(ingested.ledger_head.to_hex())),
        (
            "export_contract",
            Cv::Text(ingested.nsb.export_contract.clone()),
        ),
    ]);

    let packs: Vec<Cv> = ingested
        .nsb
        .evidence_packs
        .iter()
        .map(|p| {
            Cv::map(vec![
                ("evidence_id", Cv::Text(p.evidence_id.clone())),
                ("record_id", Cv::Text(p.record_id.clone())),
                ("locator", Cv::Text(p.locator.clone())),
                (
                    "transform_path",
                    Cv::Array(
                        p.transform_path
                            .iter()
                            .map(|t| Cv::Text(t.clone()))
                            .collect(),
                    ),
                ),
                ("license", Cv::Text(p.license.clone())),
                (
                    "attribution",
                    match &p.attribution {
                        Some(a) => Cv::Text(a.clone()),
                        None => Cv::Null,
                    },
                ),
                ("raw_hash", Cv::Text(p.raw_hash.to_hex())),
                ("csu_class", Cv::Text(p.csu_class.to_hex())),
            ])
        })
        .collect();
    let evidence_cv = Cv::map(vec![
        ("evidence_packs", Cv::Array(packs)),
        (
            "adapter_cites",
            Cv::Array(
                ingested
                    .adapter_cites
                    .iter()
                    .map(|c| Cv::Text(c.clone()))
                    .collect(),
            ),
        ),
    ]);

    seal_canonical(
        "source",
        &["source"],
        &[
            seg(KIND_MANIFEST, &manifest),
            canon_desc_segment(),
            seg(KIND_CSA_NSB, &nsb_cv),
            seg(KIND_EVIDENCE, &evidence_cv),
        ],
    )
    .map_err(WorkbodyError::Pack)
}

// --------------- 2. Bauplan-Container (Klasse "blueprint") ---------------

/// Die Replay-Klasse des Bauplans: Digest ueber packet_digest +
/// SORTIERTE zertifizierte Blueprint-Klassen + Quell-core_root —
/// deterministisch; zwei Laeufe derselben Eingabe ergeben dieselbe
/// Klasse (die `check_replay`-Achse).
pub fn blueprint_class_digest(
    packet: &GroundingPacket,
    certified: &[(String, Digest)],
    source_root_hex: &str,
) -> Digest {
    let mut certs: Vec<String> = certified
        .iter()
        .map(|(id, d)| format!("{id}\u{1f}{}", d.to_hex()))
        .collect();
    certs.sort();
    let mut buf = Vec::new();
    buf.extend_from_slice(packet.digest_hex().as_bytes());
    buf.push(0x1e);
    for c in certs {
        buf.extend_from_slice(c.as_bytes());
        buf.push(0x1e);
    }
    buf.extend_from_slice(source_root_hex.as_bytes());
    sha256(&buf)
}

fn rule_cv(r: &RuleAtom) -> Cv {
    Cv::map(vec![
        ("rule_id", Cv::Text(r.rule_id.clone())),
        ("scope", Cv::Text(r.scope.clone())),
        ("trigger", Cv::Text(r.trigger.clone())),
        ("prescription", Cv::Text(r.prescription.clone())),
        ("severity", Cv::Text(r.severity.as_str().to_string())),
        (
            "evidence_ref",
            match &r.evidence_ref {
                Some(e) => Cv::Text(e.clone()),
                None => Cv::Null,
            },
        ),
        (
            "gate_ref",
            match &r.gate_ref {
                Some(g) => Cv::Text(g.clone()),
                None => Cv::Null,
            },
        ),
    ])
}

fn decision_cv(d: &DecisionSlot) -> Cv {
    Cv::map(vec![
        ("decision_id", Cv::Text(d.decision_id.clone())),
        ("question", Cv::Text(d.question.clone())),
        ("status", Cv::Text(d.status.as_str().to_string())),
        (
            "domain",
            Cv::Array(d.domain.iter().map(|x| Cv::Text(x.clone())).collect()),
        ),
    ])
}

/// Siegelt den Bauplan-Workbody (Klasse `"blueprint"`). `cites` zeigt
/// auf den Quell-Container (`Derives`) — die Quell-Evidence ist damit
/// strukturell gebunden, nicht nur behauptet.
pub fn seal_blueprint_workbody(
    input: &RepoIntelInput,
    ingested: &IngestedRepo,
    distilled: &DistilledRepo,
    source_root_hex: &str,
) -> Result<Sealed, WorkbodyError> {
    let packet = &distilled.grounding.packet;
    let class_digest =
        blueprint_class_digest(packet, &distilled.outcome.certified, source_root_hex);

    let source_cite = CiteEntry {
        unit_id: "quell_evidence".to_string(),
        target_core_root_hex: source_root_hex.to_string(),
        target_unit_ref: None,
        cite_kind: CiteKind::Derives,
    };
    let external = external_citations_hex(std::slice::from_ref(&source_cite));

    // RESIDUE zuerst (offene Entscheidungen SICHTBAR, Warnings), damit
    // die Manifest-Zusammenfassung der Segment-Zaehlung entspricht (N6).
    let mut residue_entries: Vec<(String, String)> = packet
        .open_decisions
        .iter()
        .filter(|d| {
            matches!(
                d.status,
                cce_swe::grounding::DecisionStatus::Open
                    | cce_swe::grounding::DecisionStatus::Proposed
            )
        })
        .map(|d| {
            (
                format!("rig:repointel_decision_left_open:{}", d.decision_id),
                d.question.clone(),
            )
        })
        .collect();
    residue_entries.extend(
        distilled
            .grounding
            .downgrades
            .iter()
            .map(|r| (r.id.clone(), r.content.clone())),
    );
    let residue_kinds: Vec<String> = residue_entries.iter().map(|(id, _)| id.clone()).collect();

    let manifest = manifest_base(
        &format!(
            "Bauplan (RepoIntelligence): {} @ {}",
            input.repo_id, input.commit_sha
        ),
        "blueprint",
        // LEDGER traegt closure_proof=true (Destillation abgeschlossen).
        true,
        &residue_kinds,
        external,
        Cv::map(vec![
            ("tool", Cv::Text("cce-repointel-0.1".into())),
            ("repo", Cv::Text(input.repo_id.clone())),
            ("commit", Cv::Text(input.commit_sha.clone())),
            ("observation", Cv::Text("structural_kv_v1".into())),
            ("packet_digest", Cv::Text(packet.digest_hex())),
            ("blueprint_class", Cv::Text(class_digest.to_hex())),
        ]),
    );

    // CL_SUBSTRATE: Grounding + zertifizierte Blueprints + cites.
    let cl = Cv::map(vec![
        ("cubes", Cv::Array(vec![])),
        ("constraints", Cv::Array(vec![])),
        ("cites", cites_field(&[source_cite])),
        (
            "grounding_packet",
            Cv::map(vec![
                ("package_id", Cv::Text(packet.package_id.clone())),
                ("packet_digest", Cv::Text(packet.digest_hex())),
                (
                    "rules",
                    Cv::Array(packet.rules.iter().map(rule_cv).collect()),
                ),
                (
                    "open_decisions",
                    Cv::Array(packet.open_decisions.iter().map(decision_cv).collect()),
                ),
                (
                    "tool_surface",
                    Cv::Array(
                        packet
                            .tool_surface
                            .iter()
                            .map(|t| Cv::Text(t.clone()))
                            .collect(),
                    ),
                ),
            ]),
        ),
        (
            "certified_blueprints",
            Cv::Array(
                distilled
                    .outcome
                    .certified
                    .iter()
                    .map(|(id, d)| {
                        Cv::map(vec![
                            ("candidate_id", Cv::Text(id.clone())),
                            ("crystal_class", Cv::Text(d.to_hex())),
                        ])
                    })
                    .collect(),
            ),
        ),
    ]);

    // HBM: Facetten + Skelett + Ranking (dieselbe Form wie der
    // Eigenkorpus-Blueprint-Seed).
    let hbm = Cv::map(vec![
        (
            "facets",
            Cv::Array(
                distilled
                    .facets
                    .iter()
                    .map(|f| {
                        Cv::map(vec![
                            ("facet_type", Cv::Text(f.facet_type.clone())),
                            ("scope", Cv::Text(f.scope.clone())),
                            (
                                "evidence",
                                match &f.evidence {
                                    Some(e) => Cv::Text(e.clone()),
                                    None => Cv::Null,
                                },
                            ),
                        ])
                    })
                    .collect(),
            ),
        ),
        (
            "skeleton",
            Cv::Text(match distilled.outcome.treewidth {
                Some(w) => format!("jt:treewidth={w}"),
                None => "jt:none".to_string(),
            }),
        ),
        (
            "ranking",
            Cv::Array(
                distilled
                    .outcome
                    .ranking
                    .iter()
                    .map(|(id, s)| {
                        Cv::map(vec![
                            ("candidate_id", Cv::Text(id.clone())),
                            ("score", Cv::Uint(*s)),
                        ])
                    })
                    .collect(),
            ),
        ),
    ]);

    // EVIDENCE: die Quell-Bindung (CSU/Ledger/Beobachtung).
    let evidence = Cv::map(vec![
        (
            "source_bundle",
            Cv::map(vec![
                ("bundle_id", Cv::Text(ingested.nsb.bundle_id.clone())),
                ("ledger_head", Cv::Text(ingested.ledger_head.to_hex())),
                (
                    "observation_locator",
                    Cv::Text(ingested.observation_locator.clone()),
                ),
                (
                    "csu_uids",
                    Cv::Array(
                        ingested
                            .nsb
                            .csu_set
                            .iter()
                            .map(|c| Cv::Text(c.uid.clone()))
                            .collect(),
                    ),
                ),
            ]),
        ),
        (
            "facet_lines",
            Cv::Array(
                distilled
                    .lines
                    .iter()
                    .map(|l| Cv::Text(l.clone()))
                    .collect(),
            ),
        ),
    ]);

    let ledger = Cv::map(vec![
        (
            "commits",
            Cv::Array(vec![Cv::Text("blueprint:sealed".into())]),
        ),
        (
            "gate_reports_for",
            Cv::Array(vec![Cv::Text("blueprint:sealed".into())]),
        ),
        ("closure_proof", Cv::Bool(true)),
        (
            "hdag_projection",
            Cv::Text("n/a: Bauplan ist ein Destillations-Meta-Koerper, kein Motorlauf".into()),
        ),
    ]);

    // RESIDUE-Segment aus denselben Eintraegen wie die Manifest-
    // Zusammenfassung (N6-Gleichstand strukturell garantiert).
    let residues = Cv::map(vec![(
        "residues",
        Cv::Array(
            residue_entries
                .iter()
                .map(|(id, detail)| {
                    Cv::map(vec![
                        ("id", Cv::Text(id.clone())),
                        ("severity", Cv::Text("warning".into())),
                        ("detail", Cv::Text(detail.clone())),
                    ])
                })
                .collect(),
        ),
    )]);

    // REPLAY_MANIFEST: RD = die deterministische Beobachtung; die
    // Klasse ist `blueprint_class_digest` (zwei Laeufe ⇒ identisch).
    // INPUT-GEBUNDEN (I.5, schliesst R-Agent-10 mit dem ersten realen
    // Use-Case): `input_digests` traegt die raw_hashes der
    // Beobachtungs-Bytes — der Replay-Vertrag benennt damit EXAKT die
    // Eingabe, aus der der Bauplan destilliert wurde.
    let rd_hex = sha256(
        format!(
            "repointel\u{1f}{}\u{1f}{}\u{1f}structural_kv_v1",
            input.repo_id, input.commit_sha
        )
        .as_bytes(),
    )
    .to_hex();
    let input_digests: Vec<String> = ingested
        .nsb
        .evidence_packs
        .iter()
        .map(|p| p.raw_hash.to_hex())
        .collect();
    let input_refs: Vec<&str> = input_digests.iter().map(String::as_str).collect();
    let replay = loom_replay::replay_manifest_segment_with_inputs(
        &rd_hex,
        0,
        &class_digest.to_hex(),
        &input_refs,
    );

    seal_canonical(
        "blueprint",
        &["blueprint"],
        &[
            seg(KIND_MANIFEST, &manifest),
            canon_desc_segment(),
            seg(KIND_CL_SUBSTRATE, &cl),
            seg(KIND_HBM, &hbm),
            seg(KIND_LEDGER, &ledger),
            seg(KIND_RESIDUE, &residues),
            seg(KIND_EVIDENCE, &evidence),
            seg(KIND_REPLAY_MANIFEST, &replay),
        ],
    )
    .map_err(WorkbodyError::Pack)
}
