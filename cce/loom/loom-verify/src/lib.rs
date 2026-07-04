//! loom-verify — Verifikationsstufen L0–L2 (LOOM-Standard Teil 5.2),
//! MOTORFREI (Reader-Prinzip: nur CBOR + SHA-256). L3 (Klassen-Replay)
//! lebt in loom-replay hinter Motor-Ports.
//!
//! Verdikte: valid | valid_with_residues | quarantine | reject —
//! nie stilles Teilergebnis; jeder Fehlpunkt ist benannt.

use loom_canon::Cv;
use loom_codec::{decode_sealed, recompute_core_root, Decoded};
use loom_format::{
    KIND_CANDIDATE_OUTPUTS, KIND_CANON_DESC, KIND_CL_SUBSTRATE, KIND_CSA_NSB, KIND_EVIDENCE,
    KIND_GATE_REPORTS, KIND_HBM, KIND_INFERENCE_PROFILE, KIND_INFERENCE_TRACE, KIND_LEDGER,
    KIND_MANIFEST, KIND_PHC, KIND_PROVIDER_MANIFEST, KIND_REPLAY_MANIFEST, KIND_RESIDUE,
    KIND_RUNTIME_PROFILE, KIND_TOOL_PROFILE, SEG_FLAG_REQUIRED_UNDERSTAND,
};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict {
    Valid,
    ValidWithResidues,
    Quarantine,
    Reject,
}

#[derive(Debug, Clone)]
pub struct Diagnosis {
    pub level: String,
    pub point: String,
    pub detail: String,
}

#[derive(Debug)]
pub struct VerificationReport {
    pub verdict: Verdict,
    pub diagnoses: Vec<Diagnosis>,
    /// unbekannte Kinds ohne required_understand: preserve + SICHTBAR.
    pub preserved_unknown: Vec<u16>,
    pub residue_count: u64,
}

fn diag(level: &str, point: &str, detail: &str) -> Diagnosis {
    Diagnosis {
        level: level.into(),
        point: point.into(),
        detail: detail.into(),
    }
}

const KNOWN_KINDS: [u16; 26] = [
    0x0000, 0x0001, 0x0002, 0x0003, 0x0004, 0x0010, 0x0011, 0x0012, 0x0013, 0x0014, 0x0015, 0x0016,
    0x0017, 0x0018, 0x0020, 0x0021, 0x0030, 0x0031, 0x0032, 0x0033, 0x0050, 0x0060, 0x0061, 0x0062,
    0x0063, 0x0064,
];

fn get<'a>(map: &'a Cv, key: &str) -> Option<&'a Cv> {
    if let Cv::Map(entries) = map {
        entries.iter().find_map(|(k, v)| match k {
            Cv::Text(s) if s == key => Some(v),
            _ => None,
        })
    } else {
        None
    }
}

fn as_text(v: &Cv) -> Option<&str> {
    match v {
        Cv::Text(s) => Some(s),
        _ => None,
    }
}

/// Pflichtsegmente je Profil (Teil 3.4).
fn required_kinds(profile: &str) -> Vec<u16> {
    let mut base = vec![KIND_MANIFEST, KIND_CANON_DESC];
    match profile {
        "inspection" => {}
        "workcell" => base.extend([
            KIND_CL_SUBSTRATE,
            KIND_PHC,
            KIND_RUNTIME_PROFILE,
            KIND_GATE_REPORTS,
        ]),
        "source" => base.extend([KIND_CSA_NSB, KIND_EVIDENCE]),
        "hbm" => base.extend([KIND_HBM, KIND_EVIDENCE]),
        "runtime" => base.extend([KIND_LEDGER, KIND_REPLAY_MANIFEST]),
        // S-E5 §10(e): die Norm-Speicherform braucht CL (Pattern),
        // EVIDENCE (BridgeGate-Report), LEDGER, RESIDUE und
        // REPLAY_MANIFEST (Destillations-RD) — kein Motor-Workcell-Pfad
        // (kein PHC/GATE_REPORTS/RUNTIME_PROFILE), da eine Norm nie durch
        // den Motor laeuft, sondern durchs BridgeGate promoviert wird.
        "norm" => base.extend([
            KIND_CL_SUBSTRATE,
            KIND_LEDGER,
            KIND_RESIDUE,
            KIND_EVIDENCE,
            KIND_REPLAY_MANIFEST,
        ]),
        "full" => base.extend([
            KIND_CL_SUBSTRATE,
            KIND_PHC,
            KIND_LEDGER,
            KIND_RESIDUE,
            KIND_GATE_REPORTS,
            KIND_EVIDENCE,
            KIND_REPLAY_MANIFEST,
            KIND_RUNTIME_PROFILE,
            KIND_CSA_NSB,
            KIND_HBM,
        ]),
        // Dokument 18 §6 (P2): die RepoWorkbody-Speicherform braucht CL
        // (cites/Snapshot-Beschreibung), LEDGER (TaskLedger), RESIDUE,
        // EVIDENCE (ToolEvidence: Build-/TestRun), REPLAY_MANIFEST
        // (SWE-A4-Replay-Vertrag), CANDIDATE_OUTPUTS (der DiffCandidate
        // selbst, C.7 — nie ein Commit) und TOOL_PROFILE (die
        // verwendeten Werkzeug-Deklarationen, Deklaration ≠ Aktivierung)
        // — dieselbe Disziplin wie "norm", zusaetzlich um die
        // SWE-spezifischen Kinds erweitert.
        "repo" => base.extend([
            KIND_CL_SUBSTRATE,
            KIND_LEDGER,
            KIND_RESIDUE,
            KIND_EVIDENCE,
            KIND_REPLAY_MANIFEST,
            KIND_CANDIDATE_OUTPUTS,
            KIND_TOOL_PROFILE,
        ]),
        _ => {}
    }
    base
}

pub const MANIFEST_REQUIRED_FIELDS: [&str; 13] = [
    "title",
    "container_class",
    "domain_refs",
    "scale",
    "pl_level",
    "claims",
    "origin",
    "profiles_required",
    "profiles_optional",
    "residue_summary",
    "capability_declarations",
    "license_summary",
    "created",
];

/// L0: Struktur (Praeambel/Footer/Frames/Digests/SEGTAB/Root).
pub fn verify_l0(bytes: &[u8]) -> Result<Decoded, Vec<Diagnosis>> {
    let dec = decode_sealed(bytes).map_err(|e| vec![diag("L0", "structure", &format!("{e:?}"))])?;
    if recompute_core_root(&dec.segtab) != dec.footer.core_root {
        return Err(vec![diag("L0", "core_root", "Merkle-Root-Mismatch (N4)")]);
    }
    Ok(dec)
}

/// L0–L2 in einem Lauf; Abbruch je Stufe mit begruendeten Fehlpunkten.
pub fn verify(bytes: &[u8]) -> VerificationReport {
    // ---- L0 ----
    let dec = match verify_l0(bytes) {
        Ok(d) => d,
        Err(diagnoses) => {
            return VerificationReport {
                verdict: Verdict::Reject,
                diagnoses,
                preserved_unknown: vec![],
                residue_count: 0,
            }
        }
    };
    let mut diagnoses: Vec<Diagnosis> = Vec::new();
    let mut preserved_unknown = Vec::new();
    let mut quarantine = false;

    // Payloads je Kind dekodieren (L1: Kanon + Schema).
    let mut by_kind: BTreeMap<u16, Vec<Cv>> = BTreeMap::new();
    for (entry, frame) in &dec.frames {
        if !KNOWN_KINDS.contains(&entry.kind)
            && !(loom_format::KIND_EXT_MIN..=loom_format::KIND_EXT_MAX).contains(&entry.kind)
        {
            if entry.seg_flags & SEG_FLAG_REQUIRED_UNDERSTAND != 0 {
                quarantine = true;
                diagnoses.push(diag(
                    "L1",
                    "unknown_kind_required",
                    &format!(
                        "Kind 0x{:04x} mit required_understand — quarantine",
                        entry.kind
                    ),
                ));
            } else {
                preserved_unknown.push(entry.kind);
                diagnoses.push(diag(
                    "L1",
                    "unknown_kind_preserved",
                    &format!("Kind 0x{:04x} unbekannt — preserve, sichtbar", entry.kind),
                ));
            }
            continue;
        }
        match loom_canon::decode(&frame.payload) {
            Ok(v) => by_kind.entry(entry.kind).or_default().push(v),
            Err(e) => {
                return VerificationReport {
                    verdict: Verdict::Reject,
                    diagnoses: vec![diag(
                        "L1",
                        "canon",
                        &format!("Kind 0x{:04x}: dCBOR-Verstoss {e:?} (N3)", entry.kind),
                    )],
                    preserved_unknown,
                    residue_count: 0,
                }
            }
        }
    }

    // ---- L1: Manifest-Vertrag ----
    let manifest = match by_kind.get(&KIND_MANIFEST).and_then(|v| v.first()) {
        Some(m) => m.clone(),
        None => {
            return VerificationReport {
                verdict: Verdict::Reject,
                diagnoses: vec![diag("L1", "manifest", "MANIFEST fehlt (N2)")],
                preserved_unknown,
                residue_count: 0,
            }
        }
    };
    for field in MANIFEST_REQUIRED_FIELDS {
        if get(&manifest, field).is_none() {
            diagnoses.push(diag(
                "L1",
                "manifest_field",
                &format!("Pflichtfeld '{field}' fehlt"),
            ));
        }
    }
    if diagnoses.iter().any(|d| d.point == "manifest_field") {
        return VerificationReport {
            verdict: Verdict::Reject,
            diagnoses,
            preserved_unknown,
            residue_count: 0,
        };
    }

    // ---- L2: Profil-Pflichtsegmente ----
    let mut reject = false;
    if let Some(Cv::Array(profiles)) = get(&manifest, "profiles_required") {
        for p in profiles {
            if let Some(name) = as_text(p) {
                if !loom_format::PROFILES.contains(&name) {
                    reject = true;
                    diagnoses.push(diag(
                        "L2",
                        "profile_unknown",
                        &format!("Profil '{name}' unbekannt (N1) — kein stilles Teilverstehen"),
                    ));
                    continue;
                }
                for k in required_kinds(name) {
                    if !by_kind.contains_key(&k) {
                        reject = true;
                        diagnoses.push(diag(
                            "L2",
                            "required_segment",
                            &format!("Profil {name}: Pflichtsegment 0x{k:04x} fehlt (N2)"),
                        ));
                    }
                }
            }
        }
    }

    // ---- L2: Referenzen aufloesbar + azyklisch (N11) ----
    let digests: BTreeSet<[u8; 34]> = dec.segtab.iter().map(|e| e.digest).collect();
    for e in &dec.segtab {
        for d in &e.deps {
            if !digests.contains(d) {
                reject = true;
                diagnoses.push(diag(
                    "L2",
                    "dep_unresolved",
                    &format!("Kind 0x{:04x}: dep nicht aufloesbar (N11)", e.kind),
                ));
            }
        }
    }
    // Zyklen ueber deps (Digest-Graph):
    {
        let idx: BTreeMap<[u8; 34], usize> = dec
            .segtab
            .iter()
            .enumerate()
            .map(|(i, e)| (e.digest, i))
            .collect();
        let mut resolved: BTreeSet<usize> = BTreeSet::new();
        let mut changed = true;
        while changed {
            changed = false;
            for (i, e) in dec.segtab.iter().enumerate() {
                if resolved.contains(&i) {
                    continue;
                }
                if e.deps
                    .iter()
                    .all(|d| idx.get(d).map(|j| resolved.contains(j)).unwrap_or(true))
                {
                    resolved.insert(i);
                    changed = true;
                }
            }
        }
        if resolved.len() != dec.segtab.len() {
            reject = true;
            diagnoses.push(diag("L2", "dep_cycle", "zyklische Referenzstruktur (N11)"));
        }
    }

    // ---- L2: EvidencePack-Pflicht je CSU in CSA_NSB (N5) ----
    if let Some(nsbs) = by_kind.get(&KIND_CSA_NSB) {
        for nsb in nsbs {
            let csus: Vec<&str> = match get(nsb, "csu_uids") {
                Some(Cv::Array(items)) => items.iter().filter_map(as_text).collect(),
                _ => vec![],
            };
            let eps: BTreeSet<&str> = match get(nsb, "evidence_for") {
                Some(Cv::Array(items)) => items.iter().filter_map(as_text).collect(),
                _ => BTreeSet::new(),
            };
            for c in csus {
                if !eps.contains(c) {
                    reject = true;
                    diagnoses.push(diag(
                        "L2",
                        "evidence_missing",
                        &format!("CSU {c} ohne EvidencePack (N5)"),
                    ));
                }
            }
        }
    }

    // ---- L2: GateReport je Ledger-Commit (N6) + Score-Verdikt (N14) ----
    if let Some(ledgers) = by_kind.get(&KIND_LEDGER) {
        for ledger in ledgers {
            let commits: Vec<&str> = match get(ledger, "commits") {
                Some(Cv::Array(items)) => items.iter().filter_map(as_text).collect(),
                _ => vec![],
            };
            let gated: BTreeSet<&str> = match get(ledger, "gate_reports_for") {
                Some(Cv::Array(items)) => items.iter().filter_map(as_text).collect(),
                _ => BTreeSet::new(),
            };
            for c in commits {
                if !gated.contains(c) {
                    reject = true;
                    diagnoses.push(diag(
                        "L2",
                        "gate_report_missing",
                        &format!("Commit {c} ohne GateReport (N6)"),
                    ));
                }
            }
        }
    }
    if let Some(grs) = by_kind.get(&KIND_GATE_REPORTS) {
        for gr in grs {
            if let Cv::Map(_) = gr {
                if let Some(Cv::Array(reports)) = get(gr, "reports") {
                    for r in reports {
                        for score_key in ["score", "confidence", "ranking"] {
                            if get(r, score_key).is_some() {
                                reject = true;
                                diagnoses.push(diag(
                                    "L2",
                                    "score_as_verdict",
                                    &format!("GATE_REPORTS mit '{score_key}'-Feld (N14)"),
                                ));
                            }
                        }
                        if get(r, "verdict").is_none() {
                            reject = true;
                            diagnoses.push(diag(
                                "L2",
                                "verdict_missing",
                                "GateReport ohne boolesches Verdikt (N14-Klasse)",
                            ));
                        }
                    }
                }
            }
        }
    }

    // ---- L2: residue_summary ≡ RESIDUE (N6) ----
    let mut residue_count: u64 = 0;
    let mut residue_kinds: BTreeSet<String> = BTreeSet::new();
    if let Some(residues) = by_kind.get(&KIND_RESIDUE) {
        for seg in residues {
            if let Some(Cv::Array(items)) = get(seg, "residues") {
                residue_count += items.len() as u64;
                for it in items {
                    if let Some(Cv::Text(k)) = get(it, "kind") {
                        residue_kinds.insert(k.clone());
                    }
                }
            }
        }
    }
    if let Some(summary) = get(&manifest, "residue_summary") {
        let declared_count = match get(summary, "count") {
            Some(Cv::Uint(n)) => *n,
            _ => 0,
        };
        if declared_count != residue_count {
            reject = true;
            diagnoses.push(diag(
                "L2",
                "residue_summary_mismatch",
                &format!(
                    "residue_summary.count={declared_count} ≠ RESIDUE-Segment {residue_count} (N6)"
                ),
            ));
        }
    }

    // ---- L2: Claims ≤ Beweislage ----
    if let Some(claims) = get(&manifest, "claims") {
        let claims_closed = matches!(get(claims, "closed"), Some(Cv::Bool(true)))
            || matches!(get(claims, "certified"), Some(Cv::Bool(true)));
        if claims_closed {
            let ledger_has_proof = by_kind
                .get(&KIND_LEDGER)
                .map(|ls| {
                    ls.iter()
                        .any(|l| matches!(get(l, "closure_proof"), Some(Cv::Bool(true))))
                })
                .unwrap_or(false);
            if !ledger_has_proof {
                reject = true;
                diagnoses.push(diag(
                    "L2",
                    "claim_over_evidence",
                    "claims.closed/certified ohne gruenen Abschlussbeweis im LEDGER",
                ));
            }
        }
    }

    // ---- L2 (S-E2a I.4, N-CIT-5): MANIFEST.external_citations ≡ die
    // tatsaechlichen cites-Ziele im CL_SUBSTRATE. Rein hermetisch (nur
    // Bytes dieses EINEN Containers) — die eigentliche Aufloesung
    // externer Ziele braucht einen Resolver und lebt in loom-cites.
    {
        let mut actual_roots: BTreeSet<String> = BTreeSet::new();
        if let Some(cls) = by_kind.get(&KIND_CL_SUBSTRATE) {
            for cl in cls {
                if let Some(Cv::Array(cites)) = get(cl, "cites") {
                    for c in cites {
                        if let Some(Cv::Text(root)) = get(c, "target_core_root") {
                            actual_roots.insert(root.clone());
                        }
                    }
                }
            }
        }
        let declared_roots: BTreeSet<String> = match get(&manifest, "external_citations") {
            Some(Cv::Array(items)) => items.iter().filter_map(as_text).map(String::from).collect(),
            _ => BTreeSet::new(),
        };
        if declared_roots != actual_roots {
            reject = true;
            diagnoses.push(diag(
                "L2",
                "manifest_citation_mismatch",
                "MANIFEST.external_citations stimmt nicht mit den tatsaechlichen cites-Zielen im CL_SUBSTRATE ueberein (N-CIT-5)",
            ));
        }
    }

    // ---- L2 Overlay 05 Teil E: Inference-/Tool-Kinds ----
    // 0x0060: kein Autostart-/Aktivierungsfeld zulaessig (N16/N15).
    if let Some(pms) = by_kind.get(&KIND_PROVIDER_MANIFEST) {
        for pm in pms {
            if let Some(Cv::Array(providers)) = get(pm, "providers") {
                for p in providers {
                    for bad in ["autostart", "activate_on_open", "auto_activate"] {
                        if get(p, bad).is_some() {
                            reject = true;
                            diagnoses.push(diag("L2", "provider_autostart_flag", &format!("PROVIDER_MANIFEST mit '{bad}' (N16) — Deklaration ≠ Aktivierung")));
                        }
                    }
                }
            }
            for bad in ["on_open", "hidden_model_call"] {
                if get(pm, bad).is_some() {
                    reject = true;
                    diagnoses.push(diag(
                        "L2",
                        "hidden_model_call_on_open",
                        &format!("PROVIDER_MANIFEST mit '{bad}' (N15)"),
                    ));
                }
            }
        }
    }
    // 0x0061: referenzierte Manifeste muessen vorhanden sein.
    if let Some(profiles) = by_kind.get(&KIND_INFERENCE_PROFILE) {
        let declared: BTreeSet<String> = by_kind
            .get(&KIND_PROVIDER_MANIFEST)
            .map(|pms| {
                pms.iter()
                    .filter_map(|pm| match get(pm, "providers") {
                        Some(Cv::Array(ps)) => Some(
                            ps.iter()
                                .filter_map(|p| get(p, "provider_id").and_then(as_text))
                                .map(|s| s.to_string())
                                .collect::<Vec<_>>(),
                        ),
                        _ => None,
                    })
                    .flatten()
                    .collect()
            })
            .unwrap_or_default();
        for prof in profiles {
            if let Some(Cv::Array(refs)) = get(prof, "provider_refs") {
                for r in refs {
                    if let Some(id) = as_text(r) {
                        if !declared.contains(id) {
                            reject = true;
                            diagnoses.push(diag(
                                "L2",
                                "inference_profile_dangling",
                                &format!(
                                    "INFERENCE_PROFILE referenziert unbekannten Provider '{id}'"
                                ),
                            ));
                        }
                    }
                }
            }
        }
        if !by_kind.contains_key(&KIND_PROVIDER_MANIFEST) {
            reject = true;
            diagnoses.push(diag(
                "L2",
                "provider_manifest_required",
                "0x0061 vorhanden ⇒ 0x0060 Pflicht",
            ));
        }
    }
    // 0x0062: jede Response evidence-gebunden.
    if let Some(traces) = by_kind.get(&KIND_INFERENCE_TRACE) {
        for t in traces {
            if let Some(Cv::Array(resps)) = get(t, "responses") {
                for r in resps {
                    if get(r, "evidence_ref").is_none() {
                        reject = true;
                        diagnoses.push(diag(
                            "L2",
                            "trace_unbound",
                            "INFERENCE_TRACE-Response ohne evidence_ref",
                        ));
                    }
                }
            }
        }
    }
    // 0x0063: Evidence-Ref je Output; keiner als Commit markiert.
    if let Some(cands) = by_kind.get(&KIND_CANDIDATE_OUTPUTS) {
        for c in cands {
            if let Some(Cv::Array(outputs)) = get(c, "outputs") {
                for o in outputs {
                    if get(o, "evidence_ref").is_none() {
                        reject = true;
                        diagnoses.push(diag(
                            "L2",
                            "candidate_unbound",
                            "CandidateOutput ohne evidence_ref",
                        ));
                    }
                    if matches!(get(o, "is_commit"), Some(Cv::Bool(true))) {
                        reject = true;
                        diagnoses.push(diag(
                            "L2",
                            "candidate_marked_commit",
                            "CandidateOutput als Commit markiert — nie zulaessig (C.7)",
                        ));
                    }
                }
            }
        }
    }
    // 0x0064: keine implizite Freigabe.
    if let Some(tools) = by_kind.get(&KIND_TOOL_PROFILE) {
        for t in tools {
            if let Some(Cv::Array(entries)) = get(t, "tools") {
                for e in entries {
                    if matches!(get(e, "enabled"), Some(Cv::Bool(true)))
                        || matches!(get(e, "implicit_grant"), Some(Cv::Bool(true)))
                    {
                        reject = true;
                        diagnoses.push(diag(
                            "L2",
                            "tool_implicit_grant",
                            "TOOL_PROFILE mit impliziter Freigabe — Deklaration ≠ Aktivierung",
                        ));
                    }
                }
            }
        }
    }
    // Capability-Aufspaltung: `network_request` allein ist Altform —
    // konservativ: keine der drei Egress-Capabilities aktivierbar,
    // sichtbar als Diagnose (read-kompatibel, kein Reject).
    if let Some(Cv::Array(caps)) = get(&manifest, "capability_declarations") {
        for c in caps {
            if as_text(c) == Some("network_request") {
                diagnoses.push(diag(
                    "L2",
                    "capability_legacy",
                    "Altform 'network_request': keine der drei Egress-Capabilities ohne Neu-Deklaration aktivierbar (sichtbar, read-kompatibel)",
                ));
            }
        }
    }

    let verdict = if reject {
        Verdict::Reject
    } else if quarantine {
        Verdict::Quarantine
    } else if residue_count > 0 || !preserved_unknown.is_empty() {
        Verdict::ValidWithResidues
    } else {
        Verdict::Valid
    };
    VerificationReport {
        verdict,
        diagnoses,
        preserved_unknown,
        residue_count,
    }
}
