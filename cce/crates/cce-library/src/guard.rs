//! DER Regressionswaechter (S8.3): bei JEDEM Lauf — jede Referenz bleibt
//! gruen (schliesst weiterhin), jede Negative bleibt rot (wird weiterhin
//! mit dem ERWARTETEN Grund abgelehnt). Bricht eines davon: BAU ROT,
//! vor der Auslieferung. Derselbe Waechter schuetzt Domaenen, Updates,
//! CoreExtensions (S14.4), CSA- (S8-A5) und .loom-Zeugen — sie treten
//! phasenweise in diese Pruefung ein.

use crate::registry::{evaluate, AssetKind, Registry};

#[derive(Debug)]
pub enum GuardOutcome {
    Green {
        witnesses_checked: usize,
    },
    /// Bau rot: benannte Brueche.
    Broken(Vec<String>),
}

/// Prueft ALLE Zeugen der Registry gegen den Motor.
pub fn run_guard(registry: &Registry) -> GuardOutcome {
    let mut breaks = Vec::new();
    let mut checked = 0usize;
    for asset in registry.assets() {
        if asset.retired {
            continue;
        }
        let reports = evaluate(&asset.crystal);
        let all_pass = !reports.is_empty() && reports.iter().all(|r| r.is_pass());
        match asset.kind {
            AssetKind::ReferenceCube | AssetKind::Template | AssetKind::Fragment => {
                checked += 1;
                if !all_pass {
                    breaks.push(format!(
                        "REFERENZ GEBROCHEN: {} schliesst nicht mehr ({})",
                        asset.name,
                        reports
                            .iter()
                            .filter(|r| !r.is_pass())
                            .map(|r| r.gate_id.clone())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ));
                }
            }
            AssetKind::NegativeCube => {
                checked += 1;
                let expected = asset
                    .expected_residue
                    .as_ref()
                    .map(|k| k.as_str().to_string())
                    .unwrap_or_default();
                let rejected_correctly = reports
                    .iter()
                    .any(|r| !r.is_pass() && r.reason.contains(&expected));
                if !rejected_correctly {
                    breaks.push(format!(
                        "NEGATIV-CUBE ENTSCHAERFT: {} wird nicht mehr mit '{}' abgelehnt \
                         — eine Prohibition wurde geschwaecht",
                        asset.name, expected
                    ));
                }
            }
            _ => {}
        }
    }
    if breaks.is_empty() {
        GuardOutcome::Green {
            witnesses_checked: checked,
        }
    } else {
        GuardOutcome::Broken(breaks)
    }
}
