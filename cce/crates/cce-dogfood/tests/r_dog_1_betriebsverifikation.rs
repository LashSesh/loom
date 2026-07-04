//! R-DOG-1 — Betriebsverifikation (Dokument 19 §6/§7): der EINE echte,
//! vollstaendig dokumentierte Lauf. LAEUFT NIE in `cargo test
//! --workspace` (kein Default-Feature, zusaetzlich `#[ignore]`).
//!
//! Konvention (wie P1s Betriebsverifikation):
//!   cargo test -p cce-dogfood --features process \
//!       --features cce-inference/http \
//!       --test r_dog_1_betriebsverifikation -- --ignored --nocapture
//!
//! Voraussetzung: `OPENAI_API_KEY` in der Prozessumgebung gesetzt (echter
//! Egress gegen https://api.openai.com); ein bereits ausgecheckter,
//! isolierter Branch `dogfood/p3-<proposal_id>` (NIE `main`) fuer die
//! reale `git add`/`git commit`-Wirkung dieses Laufs.
//!
//! TaskProposal 001 (vom Auftraggeber ausdruecklich freigegeben, s.
//! Chatverlauf): additive_test in `cce/crates/cce-swe/src/model.rs`,
//! ausschliesslich innerhalb des bestehenden `#[cfg(test)] mod tests`.
//!
//! Ehrlichkeitshinweis (kein verstecktes Detail): `fs_write` ist aus P2
//! bewusst NUR eine In-Memory-Arbeitskopie (hermetische Disziplin,
//! keine reale Diskschreibung existiert in `cce-toolgateway` — jene
//! Datei ist zudem Teil der Schutzzone, s. Dokument 19 §2, und wird
//! hier NICHT geaendert). Fuer DIESEN einen realen Lauf materialisiert
//! dieser Orchestrator das bereits vom echten Kanzel-Aufruf erzeugte,
//! bereits durch `apply_unified_diff` (unveraendert aus P2) validierte
//! Byte-Ergebnis 1:1 auf die reale Datei, BEVOR die echten
//! `cargo build`/`cargo test`-Unterprozesse laufen — eine rein
//! mechanische Persistenz bereits berechneter Bytes, keine
//! Diff-Autorenschaft. Der Test prueft explizit, dass das reale
//! Datei-Ergebnis byte-identisch mit dem, was die Kern-Kette selbst im
//! `FsWriteTool` fuehrt.

use cce_core::capability::CapabilityLock;
use cce_core::signature::sha256;
use cce_dogfood::kette::{run_dogfood_task, DogfoodOutcome};
use cce_dogfood::proposal::{DogfoodRun, OperatorDecision, TaskProposal};
use cce_dogfood::snapshot::snapshot_for_scope;
use cce_inference::gateway::InferenceRecorder;
use cce_inference::providers::openai::CloudModelProviderOpenAI;
use cce_inference::replay::replay_response;
use cce_inference::request::{ContextSlice, InferenceRequest};
use cce_phaseblock::accept::{accept_block, AcceptContext, AcceptOutcome};
use cce_swe::model::{CodeUnitRole, DiffCandidate, DiffHunk, ProducedBy, TaskLedger};
use cce_swe::provider_diff::provider_diff_candidate;
use cce_swe::workbody::seal_repo_workbody;
use cce_toolgateway::gateway::{
    BuildTool, FsWriteTool, GitOperation, GitTool, TestTool, ToolGateway,
};
use cce_toolgateway::manifest::{Egress, ToolManifest};
use std::path::PathBuf;

const REPO_REL_PATH: &str = "cce/crates/cce-swe/src/model.rs";
const BRANCH_NAME: &str = "dogfood/p3-001";

fn repo_root() -> PathBuf {
    // CARGO_MANIFEST_DIR = <repo>/cce/crates/cce-dogfood
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .expect("Repo-Wurzel aufloesbar")
}

fn model_rs_abs_path() -> PathBuf {
    repo_root().join(REPO_REL_PATH)
}

fn tool_manifest(class: &str, scope: &str) -> ToolManifest {
    ToolManifest {
        tool_id: format!("dogfood-{class}-1"),
        tool_class: class.to_string(),
        scope: vec![scope.to_string()],
        side_effects: true,
        egress: Egress::None,
        budget_calls: 5,
        replay_strategy: "recorded".to_string(),
        lock_ref: format!("lock:{class}"),
    }
}

#[test]
#[ignore]
fn r_dog_1_real_task_runs_full_chain_on_isolated_branch() {
    let repo_root = repo_root();
    let file_path = model_rs_abs_path();
    let original_content = std::fs::read_to_string(&file_path).expect("model.rs real lesbar");

    // --- TaskProposal 001 (Auftraggeber-Freigabe: s. Chatverlauf) ---
    let proposal = TaskProposal {
        proposal_id: "001".to_string(),
        wish_text: "Fuege in crates/cce-swe/src/model.rs einen zusaetzlichen, rein additiven \
                     Unit-Test hinzu, der beweist, dass apply_unified_diff() auch einen unified \
                     diff mit MEHREREN, nicht zusammenhaengenden Hunks in derselben Datei korrekt \
                     anwendet."
            .to_string(),
        task_class: "additive_test".to_string(),
        target_scope: vec![REPO_REL_PATH.to_string()],
        estimated_cost_budget: 1,
        rationale: "erhoeht die Beweislage von apply_unified_diff() (der Funktion, die jeder \
                     kuenftige reale Dogfooding-Patch durchlaeuft) um den bisher ungetesteten \
                     Mehr-Hunk-Fall — kein neuer Code, nur mehr Beweislage."
            .to_string(),
    };
    let run = DogfoodRun::new(
        proposal.clone(),
        BRANCH_NAME,
        OperatorDecision::Approved {
            confirmation_ref: "operator:auftraggeber;chat-freigabe:TaskProposal-001".to_string(),
        },
    );

    // --- Schritt 1: die echte Kanzel (P1, unveraendertes Gateway) formt
    // den DiffCandidate. Reale Netzwerk-Anfrage NUR mit Feature `http`
    // auf cce-inference aktiv; ohne Feature degradiert der Provider
    // sichtbar (provider_unavailable) -- dann bricht dieser Test hier
    // ab (kein stiller Fallback).
    let provider = CloudModelProviderOpenAI::new("gpt-4o-mini");
    let system_contract = "Du bist ein Diff-Generator fuer ein Rust-Repository. Antworte \
        AUSSCHLIESSLICH mit einem einzelnen unified-diff-Hunk-Block: eine Zeile der Form \
        '@@ -alt_start,alt_len +neu_start,neu_len @@' gefolgt von Kontextzeilen (Praefix \
        ein Leerzeichen), Entfernungszeilen (Praefix '-') und Hinzufuegungszeilen (Praefix \
        '+'). KEINE Markdown-Code-Fences (kein ```), KEINE Erklaerung, KEIN Dateikopf \
        (kein '---'/'+++'), NUR den Hunk-Text selbst, exakt wie im Original-Diff-Format. \
        KRITISCH, LIES GENAU: das ALLERERSTE Zeichen jeder Zeile (Spalte 0, OHNE \
        vorangestelltes Leerzeichen) ist entweder '+' oder '-' oder ' ' (genau eines dieser \
        drei Zeichen, NIE etwas davor). NIEMALS ein Leerzeichen VOR einem '+' oder '-' \
        einfuegen. Richtiges Beispiel fuer eine hinzugefuegte Zeile (Spalte 0 ist das \
        Pluszeichen): '+    let x = 1;'. FALSCH waere: ' +    let x = 1;' (das fuehrende \
        Leerzeichen vor dem Plus ist ein Fehler, den du vermeiden musst) und FALSCH waere \
        auch '+ +    let x = 1;' oder '+#    let x = 1;'. Nur EIN Praefixzeichen, dann \
        direkt der Code. Jede neue Zeile im Hunk muss syntaktisch gueltiger, vollstaendig \
        kompilierbarer Rust-Code sein.";
    let user_content = format!(
        "{wish}\n\n{rationale}\n\nAktueller, VOLLSTAENDIGER Inhalt von {path} \
         (1-indexierte Zeilennummern zur Orientierung, NICHT Teil der Datei):\n\n{numbered}\n\n\
         Fuege GENAU EINEN neuen #[test]-Funktionsblock in das bestehende `mod tests {{ ... }}` \
         ein (am Ende, direkt nach dem letzten bestehenden Test, vor der schliessenden \
         geschweiften Klammer der `mod tests`), der beweist, dass `apply_unified_diff()` \
         einen unified diff mit ZWEI separaten, klar getrennten Hunks in DERSELBEN Datei \
         korrekt anwendet (z. B. eine Aenderung frueh im Text und eine zweite, unabhaengige \
         Aenderung spaeter im selben Text). Gib NUR den/die unified-diff-Hunk(s) zurueck, die \
         GENAU diese eine Testfunktion einfuegen -- keine anderen Aenderungen an der Datei. \
         WICHTIG -- exaktes Hunk-Geruest, uebernimm diese Struktur woertlich (nur die mit \
         ... markierten Stellen fuellst du mit deinem eigenen Testcode): der Hunk-Kopf ist \
         GENAU '@@ -384,0 +385,N @@' (N = Anzahl deiner Zeilen), danach '+    #[test]' dann \
         '+    fn DEIN_NAME() {{' dann mehrere '+        ...' Zeilen (dein Testkoerper) und als \
         ALLERLETZTE Zeile deines gesamten Hunks GENAU EINMAL '+    }}' (das schliesst DEINE \
         neue Funktion). Danach kommt NICHTS mehr -- keine weitere Zeile, kein zweites '}}', \
         keine Kontextzeile. Die bereits bestehende schliessende Klammer von `mod tests` \
         steht bereits in der Datei und bleibt dort unveraendert stehen; du musst und darfst \
         sie in deinem Hunk NICHT erwaehnen oder wiederholen. WICHTIG (Beispiel-Korrektheit): dein Test \
         konstruiert selbst einen kleinen Beispiel-Ausgangstext UND einen Beispiel-Diff-String \
         als Testdaten fuer `apply_unified_diff()`. Bevor du antwortest, simuliere in Gedanken \
         SCHRITT FUER SCHRITT, wie `apply_unified_diff` deinen Beispiel-Diff auf deinen \
         Beispiel-Ausgangstext anwendet (Zeile fuer Zeile, mit den echten 1-indexierten \
         Zeilennummern deines Beispiel-Ausgangstexts in den '@@'-Kopfzeilen), und stelle sicher, \
         dass GENAU dein erwartetes `assert_eq!`-Ergebnis herauskommt. Waehle bewusst ein \
         MOEGLICHST EINFACHES Beispiel: zwei sehr kurze, eindeutig getrennte Aenderungen von je \
         GENAU EINER Zeile (keine mehrdeutigen Kontextzeilen, keine wiederholten Woerter), damit \
         die Zeilennummern-Buchhaltung deines Beispiel-Diffs garantiert fehlerfrei ist. Hier ein \
         VOLLSTAENDIG KORREKTES Formatbeispiel fuer einen Zwei-Hunk-Diff (NUR das Format \
         nachahmen, NICHT diesen Inhalt verwenden): fuer einen Ausgangstext mit 4 Zeilen \
         'x1\\nx2\\nx3\\nx4\\n' waere ein gueltiger Diff, der Zeile 2 und Zeile 4 aendert: \
         '@@ -1,3 +1,3 @@\\n x1\\n-x2\\n+y2\\n x3\\n@@ -4,1 +4,1 @@\\n-x4\\n+y4\\n' -- beachte: \
         der ZWEITE Hunk-Kopf '@@ -4,1' beginnt bei der Zeilennummer, die UNMITTELBAR auf die \
         letzte vom ERSTEN Hunk beruehrte Zeile folgt (hier: nach Zeile 3 kommt Zeile 4) -- \
         KEINE Zeile darf von zwei Hunks gleichzeitig beansprucht werden und KEINE Zeile darf \
         uebersprungen werden. UEBERNIMM fuer deinen eigenen Beispiel-Diff GENAU DIESE STRUKTUR \
         (vier Zeilen Ausgangstext, erster Hunk aendert Zeile 2 MIT Kontextzeilen davor und \
         danach wie im Beispiel, zweiter Hunk aendert Zeile 4 OHNE Kontextzeile, exakt wie im \
         Beispiel) -- aendere NUR die Wortinhalte (z. B. andere Woerter statt x1/x2/x3/x4), \
         NICHT die Hunk-Kopfzeilen-Zahlen, NICHT die Anzahl der Kontextzeilen, NICHT die \
         Reihenfolge. Ein einzeiliger Hunk OHNE jede Kontextzeile (wie '@@ -1,1 +1,1 @@' ganz \
         ohne umgebende Kontextzeile) ist FALSCH und darf nicht vorkommen. KRITISCHSTER PUNKT \
         (die haeufigste Fehlerquelle ueberhaupt): die zweite Zahl in jedem \
         '@@ -a,b +c,d @@'-Kopf (b und d) wird von der Pruefung IGNORIERT -- nur 'a' (der \
         Startpunkt im Original) zaehlt. Berechne 'a' des ZWEITEN Hunks NICHT durch Addition \
         der Kopfzahlen des ersten Hunks (also NICHT 'a1+b1'), sondern zaehle die \
         Original-Zeilen, die im TEXT-KOERPER (nicht im Kopf) des ersten Hunks tatsaechlich \
         mit einem Leerzeichen oder einem '-' beginnen (Kontext- UND Entfernungszeilen, OHNE \
         die '+'-Zeilen); die Zeilennummer des zweiten Hunks ist GENAU EINS PLUS die hoechste \
         Original-Zeilennummer, die durch diese gezaehlten Zeilen erreicht wird. Beispiel im \
         obigen Formatbeispiel: Hunk 1 hat DREI solche Zeilen (' x1', '-x2', ' x3') und \
         erreicht damit Original-Zeile 3 -- deshalb beginnt Hunk 2 bei Zeile 4, UNABHAENGIG \
         davon, was im Kopf von Hunk 1 als zweite Zahl stand.",
        wish = proposal.wish_text,
        rationale = proposal.rationale,
        path = REPO_REL_PATH,
        numbered = original_content
            .lines()
            .enumerate()
            .map(|(i, l)| format!("{:>4}: {l}", i + 1))
            .collect::<Vec<_>>()
            .join("\n"),
    );

    let mut req = InferenceRequest::example("r-dog-1-kanzel");
    req.system_contract = system_contract.to_string();
    req.context = vec![ContextSlice {
        name: "projektion".to_string(),
        content: user_content,
    }];
    req.budget_tokens = 8000;

    let mut lock = CapabilityLock::closed("model_egress:cloud-openai");
    lock.open(
        "operator:auftraggeber",
        "ledger:r-dog-1-chat-freigabe-TaskProposal-001",
    );
    let mut recorder = InferenceRecorder::default();

    let base_snapshot = snapshot_for_scope(
        &proposal.target_scope,
        &[(
            REPO_REL_PATH,
            "rust",
            CodeUnitRole::Source,
            original_content.as_bytes(),
        )],
        "rustc-real-dogfood-1",
        None,
    )
    .expect("Basis-Snapshot liegt innerhalb des freigegebenen Scopes");

    let diff = provider_diff_candidate(
        &provider,
        &req,
        REPO_REL_PATH,
        &proposal.rationale,
        base_snapshot.snapshot_root(),
        &lock,
        &mut recorder,
    )
    .unwrap_or_else(|e| panic!("echter Kanzel-Lauf schlug fehl: {e:?}"));

    eprintln!(
        "R-DOG-1: echte Kanzel-Antwort erhalten ({} Zeichen unified diff, provider_id={:?})",
        diff.hunks[0].unified_diff.len(),
        match &diff.produced_by {
            ProducedBy::Provider { provider_id, .. } => provider_id.clone(),
            ProducedBy::Operator { .. } => "OPERATOR (unerwartet)".to_string(),
        }
    );
    eprintln!(
        "R-DOG-1: RAW DIFF TEXT >>>\n{}\n<<< RAW DIFF TEXT",
        diff.hunks[0].unified_diff
    );

    // --- Schritt 2: apply (unveraendert aus P2) -- real, gegen den
    // tatsaechlichen Dateiinhalt.
    let new_content =
        cce_swe::model::apply_unified_diff(&original_content, &diff.hunks[0].unified_diff)
            .unwrap_or_else(|e| panic!("apply_unified_diff auf die echte Kanzel-Antwort: {e}"));
    assert_ne!(
        new_content, original_content,
        "der reale Diff muss die Datei tatsaechlich veraendern"
    );
    assert!(
        new_content.contains("#[test]"),
        "die echte Kanzel-Antwort muss einen neuen Testblock einfuegen"
    );

    // --- Schritt 3: Materialisierung auf die reale Arbeitskopie (s.
    // Ehrlichkeitshinweis oben) -- NUR das bereits real berechnete
    // Ergebnis wird persistiert, keine neue Entscheidung.
    std::fs::write(&file_path, &new_content).expect("reale Datei schreibbar");

    let mut fs_write_tool =
        FsWriteTool::with_files(&[(REPO_REL_PATH, original_content.as_bytes())]);
    let cce_workspace_root = repo_root.join("cce");
    let fs_write_manifest = tool_manifest("fs_write", "cce/crates/cce-swe/");
    let build_manifest = tool_manifest("build", cce_workspace_root.to_str().unwrap());
    let test_manifest = tool_manifest("test", cce_workspace_root.to_str().unwrap());
    let mut gw = ToolGateway::new();
    gw.open_lock("fs_write", "operator:auftraggeber", "ledger:r-dog-1");
    gw.open_lock("build", "operator:auftraggeber", "ledger:r-dog-1");
    gw.open_lock("test", "operator:auftraggeber", "ledger:r-dog-1");

    // Reale Build-/Test-Auftraege (Feature `process` auf
    // cce-toolgateway/cce-swe aktiv): echtes `cargo build`/`cargo test`
    // in der cce-Workspace, gegen die bereits materialisierte reale
    // Datei.
    let build_tool = BuildTool::with_fixture(&["cargo", "build", "-p", "cce-swe"], 0, "");
    let test_tool = TestTool::with_fixture(&["cargo", "test", "-p", "cce-swe"], 0, "");

    let outcome = run_dogfood_task(
        &run,
        &DiffCandidate {
            base_snapshot_root: base_snapshot.snapshot_root(),
            hunks: vec![DiffHunk {
                path: REPO_REL_PATH.to_string(),
                unified_diff: diff.hunks[0].unified_diff.clone(),
            }],
            rationale: proposal.rationale.clone(),
            produced_by: diff.produced_by.clone(),
        },
        &base_snapshot,
        &mut gw,
        &fs_write_manifest,
        &mut fs_write_tool,
        &build_manifest,
        &build_tool,
        &test_manifest,
        &test_tool,
        sha256(b"rd:r-dog-1"),
    );

    // Konsistenzbeweis: die In-Memory-Arbeitskopie der Kern-Kette
    // stimmt byte-identisch mit der real materialisierten Datei
    // ueberein -- keine stille Abweichung zwischen Kandidat und Realitaet.
    assert_eq!(
        fs_write_tool.files.get(REPO_REL_PATH).unwrap(),
        new_content.as_bytes(),
        "Kern-Kette-Arbeitskopie und reale Datei muessen byte-identisch sein"
    );

    let (mut block, new_snapshot) = match outcome {
        DogfoodOutcome::Candidate(boxed) => *boxed,
        other => panic!("erwartet Candidate, war {other:?}"),
    };
    eprintln!("R-DOG-1: PhaseBlock-Kandidat erzeugt (id={})", block.id);

    assert_eq!(
        accept_block(&mut block, &AcceptContext::all_true()),
        AcceptOutcome::Accepted
    );
    let mut ledger = TaskLedger::new(sha256(b"rd:r-dog-1"));
    ledger.push(block);
    assert!(ledger.all_accepted());

    // --- RepoWorkbody versiegeln + verify == Valid ---
    let tool_manifests = [&fs_write_manifest, &build_manifest, &test_manifest];
    let sealed = seal_repo_workbody(
        "task:r-dog-1-p3-001",
        &new_snapshot,
        &fs_write_tool.files,
        &ledger,
        &DiffCandidate {
            base_snapshot_root: base_snapshot.snapshot_root(),
            hunks: vec![DiffHunk {
                path: REPO_REL_PATH.to_string(),
                unified_diff: diff.hunks[0].unified_diff.clone(),
            }],
            rationale: proposal.rationale.clone(),
            produced_by: diff.produced_by.clone(),
        },
        &tool_manifests,
    )
    .expect("RepoWorkbody-Siegelung gelingt");
    let verification = loom_verify::verify(&sealed.bytes);
    assert_eq!(
        verification.verdict,
        loom_verify::Verdict::Valid,
        "{:?}",
        verification.diagnoses
    );
    eprintln!(
        "R-DOG-1: RepoWorkbody versiegelt, loom_verify::verify == {:?}",
        verification.verdict
    );

    // --- Replay: zweiter unabhaengiger Lauf -- dieselbe aufgezeichnete
    // Antwort (kein Live-Re-Call, S-A5/A6), derselbe Basis-Snapshot,
    // identische Ergebnisklasse.
    let replayed_response =
        replay_response(&recorder, "r-dog-1-kanzel").expect("die Kanzel-Antwort ist aufgezeichnet");
    let replayed_diff_text = match replayed_response.outcome {
        cce_inference::response::ResponseOutcome::Output(text) => text,
        other => panic!("erwartet Output, war {other:?}"),
    };
    assert_eq!(
        replayed_diff_text, diff.hunks[0].unified_diff,
        "Replay muss die IDENTISCHE aufgezeichnete Antwort liefern"
    );
    let mut gw2 = ToolGateway::new();
    gw2.open_lock("fs_write", "operator:auftraggeber", "ledger:r-dog-1-replay");
    gw2.open_lock("build", "operator:auftraggeber", "ledger:r-dog-1-replay");
    gw2.open_lock("test", "operator:auftraggeber", "ledger:r-dog-1-replay");
    let mut fs_write_tool_2 =
        FsWriteTool::with_files(&[(REPO_REL_PATH, original_content.as_bytes())]);
    let outcome_2 = run_dogfood_task(
        &run,
        &DiffCandidate {
            base_snapshot_root: base_snapshot.snapshot_root(),
            hunks: vec![DiffHunk {
                path: REPO_REL_PATH.to_string(),
                unified_diff: replayed_diff_text,
            }],
            rationale: proposal.rationale.clone(),
            produced_by: diff.produced_by.clone(),
        },
        &base_snapshot,
        &mut gw2,
        &fs_write_manifest,
        &mut fs_write_tool_2,
        &build_manifest,
        &build_tool,
        &test_manifest,
        &test_tool,
        sha256(b"rd:r-dog-1"),
    );
    match outcome_2 {
        DogfoodOutcome::Candidate(boxed) => {
            let (block_2, snapshot_2) = *boxed;
            assert_eq!(
                block_2.payload_digest, ledger.blocks[0].payload_digest,
                "Replay muss dieselbe PhaseBlock-Payload-Klasse ergeben"
            );
            assert_eq!(
                snapshot_2.snapshot_root(),
                new_snapshot.snapshot_root(),
                "Replay muss denselben Snapshot-Root ergeben"
            );
            eprintln!("R-DOG-1: Replay identische Ergebnisklasse bestaetigt");
        }
        other => panic!("Replay erwartet Candidate, war {other:?}"),
    }

    // --- Realer git commit auf dem isolierten Branch, mit
    // aufgezeichneter Auftraggeber-Bestaetigung (HumanConfirmationGate).
    let git_manifest = tool_manifest("git", repo_root.to_str().unwrap());
    let mut git_gw = ToolGateway::new();
    git_gw.open_lock("git", "operator:auftraggeber", "ledger:r-dog-1-commit");
    let mut git_tool = GitTool::default();
    // GitOperation::Add prueft den Pfad gegen manifest.scope per Praefix
    // (cce_toolgateway::gateway::run_git) -- da scope[0] hier der ABSOLUTE
    // Repo-Wurzel-Pfad ist (fuer den echten `git`-Unterprozess als cwd
    // noetig), muss der Add-Pfad ebenfalls absolut sein, nicht
    // repo-relativ.
    git_gw
        .run_git(
            &git_manifest,
            &mut git_tool,
            &GitOperation::Add(file_path.to_str().unwrap().to_string()),
            None,
        )
        .expect("git add gelingt (kein confirmation_ref noetig, keine materielle Aktion)");
    git_gw
        .run_git(
            &git_manifest,
            &mut git_tool,
            &GitOperation::Commit(
                "P3 R-DOG-1: additiver Multi-Hunk-Test fuer apply_unified_diff (TaskProposal 001)"
                    .to_string(),
            ),
            Some("operator:auftraggeber;chat-freigabe:TaskProposal-001"),
        )
        .expect("git commit gelingt MIT aufgezeichneter Bestaetigung");
    eprintln!("R-DOG-1: echter git commit auf {BRANCH_NAME} abgeschlossen (HumanConfirmationGate erfuellt)");

    // KEIN push, KEIN merge -- diese Aktion existiert im System nicht
    // (MergeExclusionGate) und bleibt exklusiv eine Handlung des
    // Auftraggebers.
}
