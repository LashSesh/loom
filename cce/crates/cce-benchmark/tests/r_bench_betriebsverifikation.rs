//! R-BENCH-1/2 — Betriebsverifikation (Dokument 20 §7): die ZWEI realen
//! Vergleichslaeufe. LAUEFT NIE in `cargo test --workspace` (kein
//! Default-Feature, zusaetzlich `#[ignore]`).
//!
//! Konvention:
//!   cargo test -p cce-benchmark --features process --features http \
//!       --test r_bench_betriebsverifikation -- --ignored --nocapture
//!
//! Voraussetzung: `OPENAI_API_KEY` gesetzt; `cargo`, `python3`, `true`
//! im PATH. Beide Arme nutzen DASSELBE Modell (gpt-4o-mini) — der
//! einzige Unterschied ist die Gate-/Evidence-/Replay-/Zertifizierungs-
//! Schicht (§1). Der ungegatete Arm laeuft ZUERST und isoliert
//! (`provider.infer()` direkt, an der Gate-Kette vorbei), wird
//! versiegelt; erst DANACH der CCE-Arm (`run_inference` + P2-Kette).
//!
//! Ehrlichkeitshinweis: das Modell liefert je Aufgabe den VOLLSTAENDIGEN
//! neuen Dateiinhalt (natuerlicher als ein Diff-Format fuer Prosa); der
//! Orchestrator wickelt ihn — fuer BEIDE Arme identisch — in einen
//! Ganzdatei-Ersetzungs-Diff, den die unveraenderte P2-Kette (CCE-Arm)
//! bzw. `apply_unified_diff` (Raw-Arm) anwendet. Reine mechanische
//! Einwicklung bereits modellierter Bytes, keine Loesungsvorgabe.

use cce_benchmark::assemble::assemble_benchmark;
use cce_benchmark::model::{
    BenchmarkTaskPackage, CceRunResult, ComparisonMatrix, RawRunResult, TaskClass,
};
use cce_core::capability::CapabilityLock;
use cce_core::signature::sha256;
use cce_inference::gateway::InferenceRecorder;
use cce_inference::providers::openai::CloudModelProviderOpenAI;
use cce_inference::providers::ModelProvider;
use cce_inference::request::{ContextSlice, InferenceRequest};
use cce_inference::response::ResponseOutcome;
use cce_phaseblock::accept::{accept_block, AcceptContext, AcceptOutcome};
use cce_swe::kette::{run_swe_task, SweOutcome};
use cce_swe::model::{CodeUnitRole, DiffCandidate, DiffHunk, RepoSnapshot, TaskLedger};
use cce_swe::provider_diff::provider_diff_candidate;
use cce_swe::workbody::seal_repo_workbody;
use cce_toolgateway::gateway::{BuildTool, FsWriteTool, TestTool, ToolGateway};
use cce_toolgateway::manifest::{Egress, ToolManifest};
use std::path::Path;
use std::process::Command;
use std::time::Instant;

const MODEL: &str = "gpt-4o-mini";
const MAX_ATTEMPTS: u32 = 6;

// ---------------------------------------------------------------------
// Die zwei freigegebenen Aufgabenpakete (Chat-Freigabe: "beide").
// ---------------------------------------------------------------------

fn coding_package() -> BenchmarkTaskPackage {
    BenchmarkTaskPackage {
        package_id: "r-bench-1-coding".to_string(),
        task_class: TaskClass::Coding,
        task_text: "In src/lib.rs gibt die Funktion max_of das MINIMUM statt das MAXIMUM \
             zurueck (der Vergleichsoperator ist vertauscht). Behebe den Fehler, sodass \
             `cargo test` gruen wird. Aendere nur src/lib.rs."
            .to_string(),
        starting_files: vec![
            (
                "Cargo.toml".to_string(),
                b"[package]\nname = \"numkit\"\nversion = \"0.1.0\"\nedition = \"2021\"\n".to_vec(),
            ),
            (
                "src/lib.rs".to_string(),
                b"pub fn max_of(v: &[i64]) -> i64 {\n    let mut m = v[0];\n    for &x in v {\n        if x < m {\n            m = x;\n        }\n    }\n    m\n}\n".to_vec(),
            ),
            (
                "tests/it.rs".to_string(),
                b"use numkit::max_of;\n#[test]\nfn finds_maximum() {\n    assert_eq!(max_of(&[3, 7, 2, 9, 4]), 9);\n    assert_eq!(max_of(&[-5, -1, -8]), -1);\n}\n".to_vec(),
            ),
        ],
        success_criteria: "cargo build exit 0 UND cargo test exit 0".to_string(),
        build_command: vec!["cargo".to_string(), "build".to_string()],
        test_command: vec!["cargo".to_string(), "test".to_string()],
        target_path: "src/lib.rs".to_string(),
    }
}

const CHECK_ANALYSIS_PY: &str = r#"import sys, re
try:
    text = open("analysis.md", encoding="utf-8").read()
except FileNotFoundError:
    print("analysis.md fehlt"); sys.exit(1)
for q in ["quelle_a", "quelle_b", "quelle_c"]:
    if q not in text:
        print(f"Quelle {q} nicht referenziert"); sys.exit(1)
risks = [l for l in text.splitlines() if l.strip().startswith("RISIKO:")]
if len(risks) < 2:
    print(f"nur {len(risks)} RISIKO-Zeilen"); sys.exit(1)
verdicts = re.findall(r"^Gesamtrisiko:\s*(NIEDRIG|MITTEL|HOCH)\s*$", text, re.M)
if len(verdicts) != 1:
    print(f"Gesamtrisiko-Zeile fehlerhaft ({len(verdicts)})"); sys.exit(1)
print("OK"); sys.exit(0)
"#;

fn document_package() -> BenchmarkTaskPackage {
    BenchmarkTaskPackage {
        package_id: "r-bench-2-document".to_string(),
        task_class: TaskClass::Document,
        task_text: "Lies quelle_a.txt, quelle_b.txt und quelle_c.txt (deren Inhalt unten steht) \
             und schreibe eine Risikoanalyse. Die Analyse MUSS: (1) jede der drei Quellen \
             namentlich referenzieren (die Zeichenketten 'quelle_a', 'quelle_b', 'quelle_c' \
             muessen im Text vorkommen); (2) mindestens ZWEI konkrete Risiken benennen, jedes \
             in einer eigenen Zeile, die mit 'RISIKO:' beginnt; (3) mit GENAU EINER Zeile der \
             Form 'Gesamtrisiko: NIEDRIG' oder 'Gesamtrisiko: MITTEL' oder 'Gesamtrisiko: HOCH' \
             abschliessen. Gib NUR den vollstaendigen Inhalt der Datei analysis.md zurueck."
            .to_string(),
        starting_files: vec![
            (
                "quelle_a.txt".to_string(),
                b"Interne Notiz A (quelle_a): Der geplante Rollout des Buchungssystems trifft \
                  auf eine Serverkapazitaet, die nur fuer 60 Prozent der erwarteten Spitzenlast \
                  ausgelegt ist. Eine Skalierung ist erst im naechsten Quartal budgetiert."
                    .to_vec(),
            ),
            (
                "quelle_b.txt".to_string(),
                b"Interne Notiz B (quelle_b): Der Liefertermin wurde um drei Wochen vorgezogen. \
                  Zwei der fuenf Testphasen entfallen dadurch. Das QA-Team hat schriftlich \
                  Bedenken angemeldet."
                    .to_vec(),
            ),
            (
                "quelle_c.txt".to_string(),
                b"Interne Notiz C (quelle_c): Eine externe Sicherheitspruefung steht noch aus. \
                  Der Zahlungsdienstleister verlangt sie vor Freigabe. Ohne Abnahme drohen \
                  Vertragsstrafen."
                    .to_vec(),
            ),
            ("analysis.md".to_string(), b"TODO\n".to_vec()),
            (
                "check_analysis.py".to_string(),
                CHECK_ANALYSIS_PY.as_bytes().to_vec(),
            ),
        ],
        success_criteria:
            "python3 check_analysis.py exit 0 (drei Quellen referenziert, >=2 RISIKO-Zeilen, \
             genau eine Gesamtrisiko-Zeile)"
                .to_string(),
        build_command: vec!["true".to_string()],
        test_command: vec!["python3".to_string(), "check_analysis.py".to_string()],
        target_path: "analysis.md".to_string(),
    }
}

// ---------------------------------------------------------------------
// Hilfsfunktionen (fuer BEIDE Arme identisch).
// ---------------------------------------------------------------------

fn role_for(path: &str) -> CodeUnitRole {
    if path.ends_with(".rs") {
        CodeUnitRole::Source
    } else if path.ends_with(".toml") {
        CodeUnitRole::Config
    } else {
        CodeUnitRole::Doc
    }
}

fn base_snapshot(pkg: &BenchmarkTaskPackage) -> RepoSnapshot {
    let owned: Vec<(String, String, CodeUnitRole, Vec<u8>)> = pkg
        .starting_files
        .iter()
        .map(|(p, c)| (p.clone(), "text".to_string(), role_for(p), c.clone()))
        .collect();
    let refs: Vec<(&str, &str, CodeUnitRole, &[u8])> = owned
        .iter()
        .map(|(p, l, r, c)| (p.as_str(), l.as_str(), *r, c.as_slice()))
        .collect();
    RepoSnapshot::from_files(&refs, "rustc-bench-1", None)
}

fn materialize(dir: &Path, pkg: &BenchmarkTaskPackage) {
    if dir.exists() {
        std::fs::remove_dir_all(dir).ok();
    }
    for (path, content) in &pkg.starting_files {
        let full = dir.join(path);
        if let Some(parent) = full.parent() {
            std::fs::create_dir_all(parent).expect("Verzeichnis anlegen");
        }
        std::fs::write(&full, content).expect("Startdatei schreiben");
    }
}

fn strip_fences(s: &str) -> String {
    let t = s.trim();
    if let Some(rest) = t.strip_prefix("```") {
        // erste Zeile (Sprach-Tag) verwerfen, schliessende Fence entfernen
        let after_lang = rest.split_once('\n').map(|x| x.1).unwrap_or("");
        let body = after_lang.trim_end();
        let body = body.strip_suffix("```").unwrap_or(body);
        return body.trim_end().to_string();
    }
    t.to_string()
}

/// Ganzdatei-Ersetzungs-Diff: ersetzt den gesamten `old`-Inhalt durch
/// `new`. Fuer BEIDE Arme identisch (nur der Orchestrator wickelt ein,
/// keine Loesungsvorgabe).
fn whole_file_replace_diff(old: &str, new: &str) -> String {
    let old_lines: Vec<&str> = old.strip_suffix('\n').unwrap_or(old).split('\n').collect();
    let new_lines: Vec<&str> = new.strip_suffix('\n').unwrap_or(new).split('\n').collect();
    let mut d = format!("@@ -1,{} +1,{} @@\n", old_lines.len(), new_lines.len());
    for l in &old_lines {
        d.push('-');
        d.push_str(l);
        d.push('\n');
    }
    for l in &new_lines {
        d.push('+');
        d.push_str(l);
        d.push('\n');
    }
    d
}

fn build_request(pkg: &BenchmarkTaskPackage, extra: &str, attempt: u32) -> InferenceRequest {
    let target_content = pkg
        .starting_files
        .iter()
        .find(|(p, _)| p == &pkg.target_path)
        .map(|(_, c)| String::from_utf8_lossy(c).into_owned())
        .unwrap_or_default();
    let sources: String = pkg
        .starting_files
        .iter()
        .filter(|(p, _)| p != &pkg.target_path && p != "check_analysis.py")
        .map(|(p, c)| format!("=== {p} ===\n{}", String::from_utf8_lossy(c)))
        .collect::<Vec<_>>()
        .join("\n\n");
    let mut req = InferenceRequest::example(&format!("{}-a{}", pkg.package_id, attempt));
    req.system_contract = "Du bist ein praeziser Software-/Dokumentations-Assistent. Antworte \
         AUSSCHLIESSLICH mit dem vollstaendigen, finalen Inhalt der Zieldatei — kein Markdown- \
         Code-Fence, keine Erklaerung davor oder danach, nur der reine Dateiinhalt."
        .to_string();
    req.context = vec![ContextSlice {
        name: "projektion".to_string(),
        content: format!(
            "{task}\n\nAusgangsmaterial:\n{sources}\n\nAktueller Inhalt von {target}:\n{tc}\n{extra}",
            task = pkg.task_text,
            sources = sources,
            target = pkg.target_path,
            tc = target_content,
            extra = extra
        ),
    }];
    req.budget_tokens = 8000;
    req
}

fn run_cmd(argv: &[String], cwd: &Path) -> (i32, String) {
    let out = Command::new(&argv[0])
        .args(&argv[1..])
        .current_dir(cwd)
        .output()
        .unwrap_or_else(|e| panic!("Prozess {argv:?} in {cwd:?}: {e}"));
    let mut log = String::from_utf8_lossy(&out.stdout).into_owned();
    log.push_str(&String::from_utf8_lossy(&out.stderr));
    (out.status.code().unwrap_or(-1), log)
}

fn tool_manifest(class: &str, scope: &str) -> ToolManifest {
    ToolManifest {
        tool_id: format!("bench-{class}"),
        tool_class: class.to_string(),
        scope: vec![scope.to_string()],
        side_effects: true,
        egress: Egress::None,
        budget_calls: 20,
        replay_strategy: "recorded".to_string(),
        lock_ref: format!("lock:{class}"),
    }
}

// ---------------------------------------------------------------------
// Der ungegatete Arm: provider.infer() DIREKT, dann beobachten.
// ---------------------------------------------------------------------

fn run_raw_arm(
    pkg: &BenchmarkTaskPackage,
    provider: &CloudModelProviderOpenAI,
    dir: &Path,
    submission_order: u64,
) -> RawRunResult {
    let start = Instant::now();
    let mut interventions = 0u32;
    let mut last_output = String::new();
    let mut extra = String::new();
    let mut build_pass = false;
    let mut test_pass = false;

    for attempt in 1..=MAX_ATTEMPTS {
        materialize(dir, pkg);
        let req = build_request(pkg, &extra, attempt);
        // UNGEGATET: direkter infer()-Aufruf, keine Gate-Kette, keine
        // Evidence, kein Replay, keine Zertifizierung.
        let resp = provider.infer(&req);
        let content = match resp.outcome {
            ResponseOutcome::Output(t) => strip_fences(&t),
            other => {
                interventions += 1;
                extra = format!("\nVorheriger Versuch lieferte keinen Inhalt ({other:?}).");
                continue;
            }
        };
        last_output = content.clone();
        let target = dir.join(&pkg.target_path);
        let final_content = if content.ends_with('\n') {
            content.clone()
        } else {
            format!("{content}\n")
        };
        std::fs::write(&target, &final_content).expect("Raw-Arm schreibt Zieldatei");

        let (bc, _blog) = run_cmd(&pkg.build_command, dir);
        build_pass = bc == 0;
        let (tc, tlog) = run_cmd(&pkg.test_command, dir);
        test_pass = tc == 0;
        if build_pass && test_pass {
            eprintln!(
                "RAW  [{}] Versuch {attempt}: build+test GRUEN",
                pkg.package_id
            );
            break;
        }
        interventions += 1;
        eprintln!(
            "RAW  [{}] Versuch {attempt}: build={build_pass} test={test_pass} — erneuter Anlauf",
            pkg.package_id
        );
        extra = "\nDein vorheriger Versuch hat build oder test NICHT bestanden. Liefere die \
             korrigierte, vollstaendige Zieldatei erneut."
            .to_string();
        let _ = &tlog;
    }

    RawRunResult::observed(
        &pkg.package_id,
        &pkg.digest_hex(),
        &sha256(last_output.as_bytes()).to_hex(),
        start.elapsed().as_millis() as u64,
        Some(build_pass),
        Some(test_pass),
        interventions,
        submission_order,
    )
}

// ---------------------------------------------------------------------
// Der CCE-Arm: run_inference (Gates) + unveraenderte P2-Kette.
// ---------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn run_cce_arm(
    pkg: &BenchmarkTaskPackage,
    provider: &CloudModelProviderOpenAI,
    dir: &Path,
    submission_order: u64,
) -> (CceRunResult, Vec<u8>) {
    let start = Instant::now();
    let base = base_snapshot(pkg);
    let base_root = base.snapshot_root();
    let old_content = pkg
        .starting_files
        .iter()
        .find(|(p, _)| p == &pkg.target_path)
        .map(|(_, c)| String::from_utf8_lossy(c).into_owned())
        .unwrap_or_default();

    let mut lock = CapabilityLock::closed("model_egress:cloud-openai");
    lock.open("operator:auftraggeber", "ledger:r-bench");
    let mut recorder = InferenceRecorder::default();

    let mut interventions = 0u32;
    let mut extra = String::new();
    let mut sealed_bytes: Vec<u8> = Vec::new();
    let mut gate_report_count = 0u32;
    let mut replay_confirmed = false;
    let mut output_digest = String::new();
    let mut build_pass = false;
    let mut test_pass = false;

    for attempt in 1..=MAX_ATTEMPTS {
        materialize(dir, pkg);
        let req = build_request(pkg, &extra, attempt);
        // GEGATET: run_inference (volle Vor-Egress-Gate-Kette) via
        // provider_diff_candidate — der Modell-Content kommt hier durch
        // dasselbe unveraenderte Gateway wie in P1/P3.
        let diff0 = match provider_diff_candidate(
            provider,
            &req,
            &pkg.target_path,
            "Benchmark-CCE-Arm",
            base_root,
            &lock,
            &mut recorder,
        ) {
            Ok(d) => d,
            Err(e) => {
                interventions += 1;
                extra = format!("\nGateway-Fehler, erneuter Anlauf: {e:?}");
                continue;
            }
        };
        let content = strip_fences(&diff0.hunks[0].unified_diff);
        let final_content = if content.ends_with('\n') {
            content.clone()
        } else {
            format!("{content}\n")
        };
        // Materialisierung auf Platte (damit die realen, GATE-bezeugten
        // Build-/Test-Unterprozesse der P2-Kette die Datei sehen).
        std::fs::write(dir.join(&pkg.target_path), &final_content)
            .expect("CCE-Arm schreibt Zieldatei");

        let wrapped = whole_file_replace_diff(&old_content, &final_content);
        let real_diff = DiffCandidate {
            base_snapshot_root: base_root,
            hunks: vec![DiffHunk {
                path: pkg.target_path.clone(),
                unified_diff: wrapped.clone(),
            }],
            rationale: "Benchmark-CCE-Arm".to_string(),
            produced_by: diff0.produced_by.clone(),
        };

        let mut gw = ToolGateway::new();
        gw.open_lock("fs_write", "op", "l");
        gw.open_lock("build", "op", "l");
        gw.open_lock("test", "op", "l");
        let mut fs_write_tool =
            FsWriteTool::with_files(&[(pkg.target_path.as_str(), old_content.as_bytes())]);
        let fs_write_manifest = tool_manifest("fs_write", &pkg.target_path);
        let build_manifest = tool_manifest("build", dir.to_str().unwrap());
        let test_manifest = tool_manifest("test", dir.to_str().unwrap());
        let build_tool = BuildTool {
            argv: pkg.build_command.clone(),
            fixture_exit_code: 0,
            fixture_log: String::new(),
        };
        let test_tool = TestTool {
            argv: pkg.test_command.clone(),
            fixture_exit_code: 0,
            fixture_log: String::new(),
        };

        let outcome = run_swe_task(
            &real_diff,
            &base,
            &mut gw,
            &fs_write_manifest,
            &mut fs_write_tool,
            &build_manifest,
            &build_tool,
            &test_manifest,
            &test_tool,
            false,
            sha256(format!("rd:{}", pkg.package_id).as_bytes()),
        );

        match outcome {
            SweOutcome::Candidate(boxed) => {
                let (mut block, new_snapshot) = *boxed;
                gate_report_count = block.gate_reports.len() as u32;
                build_pass = true;
                test_pass = true;
                output_digest = sha256(final_content.as_bytes()).to_hex();

                assert_eq!(
                    accept_block(&mut block, &AcceptContext::all_true()),
                    AcceptOutcome::Accepted
                );
                let mut ledger = TaskLedger::new(sha256(b"rd:r-bench"));
                ledger.push(block.clone());

                let tms = [&fs_write_manifest, &build_manifest, &test_manifest];
                let sealed = seal_repo_workbody(
                    &format!("task:{}", pkg.package_id),
                    &new_snapshot,
                    &fs_write_tool.files,
                    &ledger,
                    &real_diff,
                    &tms,
                )
                .expect("RepoWorkbody-Siegelung gelingt");
                assert_eq!(
                    loom_verify::verify(&sealed.bytes).verdict,
                    loom_verify::Verdict::Valid
                );
                sealed_bytes = sealed.bytes.clone();

                // Replay: zweiter unabhaengiger Lauf (deterministischer
                // Apply derselben aufgezeichneten Diff) — gleiche
                // Payload-Klasse ⇒ replay_confirmed.
                let mut gw2 = ToolGateway::new();
                gw2.open_lock("fs_write", "op", "l");
                gw2.open_lock("build", "op", "l");
                gw2.open_lock("test", "op", "l");
                let mut fsw2 =
                    FsWriteTool::with_files(&[(pkg.target_path.as_str(), old_content.as_bytes())]);
                let outcome2 = run_swe_task(
                    &real_diff,
                    &base,
                    &mut gw2,
                    &fs_write_manifest,
                    &mut fsw2,
                    &build_manifest,
                    &build_tool,
                    &test_manifest,
                    &test_tool,
                    false,
                    sha256(format!("rd:{}", pkg.package_id).as_bytes()),
                );
                if let SweOutcome::Candidate(b2) = outcome2 {
                    replay_confirmed = b2.0.payload_digest == block.payload_digest;
                }
                eprintln!(
                    "CCE  [{}] Versuch {attempt}: Candidate, verify Valid, replay={replay_confirmed}",
                    pkg.package_id
                );
                break;
            }
            other => {
                interventions += 1;
                eprintln!(
                    "CCE  [{}] Versuch {attempt}: Hold/Block ({other:?}) — erneuter Anlauf",
                    pkg.package_id
                );
                extra = "\nDein vorheriger Versuch hat build oder test NICHT bestanden. Liefere \
                     die korrigierte, vollstaendige Zieldatei erneut."
                    .to_string();
            }
        }
    }

    let cce = CceRunResult::certified(
        &pkg.package_id,
        &pkg.digest_hex(),
        &output_digest,
        start.elapsed().as_millis() as u64,
        Some(build_pass),
        Some(test_pass),
        interventions,
        submission_order,
        &hex34_or_placeholder(&sealed_bytes),
        gate_report_count,
        replay_confirmed,
    );
    (cce, sealed_bytes)
}

/// Der repo_workbody_ref = core_root (hex) des CCE-Arm-RepoWorkbody.
/// Wir dekodieren ihn aus dem versiegelten Container.
fn hex34_or_placeholder(sealed_bytes: &[u8]) -> String {
    match loom_codec::decode_sealed(sealed_bytes) {
        Ok(dec) => dec
            .footer
            .core_root
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect(),
        Err(_) => "00".repeat(34),
    }
}

fn print_matrix(matrix: &ComparisonMatrix) {
    eprintln!(
        "\n===== D1–D6-Matrix: {} ({}) =====",
        matrix.package_id,
        matrix.task_class.as_str()
    );
    for row in &matrix.rows {
        eprintln!("  {}", row.dimension);
        eprintln!("    RAW: [{}] {}", row.raw.verdict, row.raw.beleg);
        eprintln!("    CCE: [{}] {}", row.cce.verdict, row.cce.beleg);
    }
    eprintln!("=================================================\n");
}

fn run_one_package(pkg: &BenchmarkTaskPackage, provider: &CloudModelProviderOpenAI) {
    let base = std::env::temp_dir().join(format!("cce-bench-{}", pkg.package_id));
    let raw_dir = base.join("raw");
    let cce_dir = base.join("cce");

    // FAIRNESS (§2): der ungegatete Arm ZUERST und isoliert
    // (submission_order 1), versiegelt, ERST DANACH der CCE-Arm
    // (submission_order 2). Getrennte Arbeitsverzeichnisse — keine Seite
    // sieht die Antwort der anderen.
    eprintln!("\n### {} — Raw-Arm (ungegatet, zuerst) ###", pkg.package_id);
    let raw = run_raw_arm(pkg, provider, &raw_dir, 1);
    eprintln!("### {} — CCE-Arm (gegatet, danach) ###", pkg.package_id);
    let (cce, _sealed) = run_cce_arm(pkg, provider, &cce_dir, 2);

    // Zusammenbau: FairnessGate + ComparisonSealGate + Matrix + Siegel.
    let assembled = assemble_benchmark(pkg, &raw, &cce)
        .unwrap_or_else(|e| panic!("Benchmark-Zusammenbau fuer {}: {e:?}", pkg.package_id));
    assert_eq!(
        loom_verify::verify(&assembled.sealed.bytes).verdict,
        loom_verify::Verdict::Valid,
        "Benchmark-Container muss Valid sein"
    );
    print_matrix(&assembled.matrix);

    // Kern-Nachweis D4 (Aufgabenparitaet): der CCE-Arm besteht die
    // success_criteria (Parität genuegt laut Messlatte 17 §4).
    assert!(
        cce.criteria_met(),
        "CCE-Arm muss die success_criteria bestehen (D4-Paritaet)"
    );
    eprintln!(
        "{}: RAW criteria_met={} ({} Eingriffe, {} ms) | CCE criteria_met={} ({} Eingriffe, {} ms, {} GateReports, replay={})",
        pkg.package_id,
        raw.criteria_met(),
        raw.human_interventions_count,
        raw.wall_time_ms,
        cce.criteria_met(),
        cce.human_interventions_count,
        cce.wall_time_ms,
        cce.gate_report_count,
        cce.replay_confirmed,
    );

    // Ergebnis-.loom fuer die Dokumentation ablegen (nicht ins Repo).
    let out = base.join(format!("{}-benchmark.loom", pkg.package_id));
    std::fs::write(&out, &assembled.sealed.bytes).ok();
    eprintln!("versiegelter Benchmark-Workbody: {}", out.display());
}

#[test]
#[ignore]
fn r_bench_both_real_comparison_runs() {
    let provider = CloudModelProviderOpenAI::new(MODEL);
    run_one_package(&coding_package(), &provider);
    run_one_package(&document_package(), &provider);
}
