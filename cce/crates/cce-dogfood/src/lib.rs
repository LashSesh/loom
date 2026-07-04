//! cce-dogfood — P3/Dokument 19: Dogfooding. **Sicherheit zuerst, Beweis
//! zweitrangig, Automatik nirgends.** Verdrahtet AUSSCHLIESSLICH
//! bestehende P1/P2-Bausteine (`cce_swe::kette::run_swe_task`,
//! `cce_swe::workbody::seal_repo_workbody`,
//! `cce_toolgateway::gateway::ToolGateway::run_git`) hinter drei neuen
//! Vor-Gates + einer Schutzzonen-Pruefung — keine neue Kern-Logik,
//! kein neuer Egress-/Container-Pfad. Kein Pfad in diesem Crate
//! fuehrt je zu einem Merge nach `main` — diese Aktion existiert
//! schlicht nicht (`gates::merge_exclusion_gate`).

pub mod fence;
pub mod gates;
pub mod kette;
pub mod proposal;
pub mod residues;
pub mod snapshot;
