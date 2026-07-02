//! nexus-fetch — der EINZIGE Netzpfad der CSA (CSA.4/CSA.10).
//! `fetch()` akzeptiert AUSSCHLIESSLICH einen `ApprovedFetchPlan`
//! (versiegelt von nexus-policy): NoFetchBeforePolicyGate ist damit
//! ARCHITEKTONISCH, nicht konventionell (PROD-INV-13).
//! RateBudgetGate (7/15) + Caching/Differenzabruf (ETag/Cursor als
//! Ref-Zustand, S9-A5).

use nexus_core::objects::RawObservation;
use nexus_core::residues::csa_residue;
use nexus_core::verdict::Verdict;
use nexus_policy::ApprovedFetchPlan;
use std::collections::BTreeMap;

/// Transport: liefert Bytes fuer geplante Endpunkte. Im Bau ausschliesslich
/// Snapshot-/Fixture-Transporte (deterministisch, netzfrei); ein realer
/// HTTP-Transport wuerde DENSELBEN versiegelten Pfad benutzen.
pub trait Transport {
    fn get(&self, endpoint: &str) -> Result<(Vec<u8>, Option<String>), String>;
}

/// Snapshot-Transport: fixe Zuordnung Endpoint → (Bytes, ETag).
#[derive(Debug, Default)]
pub struct SnapshotTransport {
    pub responses: BTreeMap<String, (Vec<u8>, Option<String>)>,
    pub snapshot_id: String,
}

impl Transport for SnapshotTransport {
    fn get(&self, endpoint: &str) -> Result<(Vec<u8>, Option<String>), String> {
        self.responses
            .get(endpoint)
            .cloned()
            .ok_or_else(|| format!("Endpoint {endpoint} nicht im Snapshot"))
    }
}

/// Cursor-/Cache-Zustand (veraenderlicher Ref ueber unveraenderlichen
/// Objekten, S9-A5): ETags + Cursor je Endpoint.
#[derive(Debug, Default)]
pub struct FetchCache {
    pub etags: BTreeMap<String, String>,
    pub cursors: BTreeMap<String, String>,
}

/// 7/15 RateBudgetGate: Quellen- und Globalbudget.
pub fn rate_budget_gate(planned_requests: u32, budget: u32) -> Verdict {
    if planned_requests <= budget {
        Verdict::Allow {
            gate: "RateBudgetGate".into(),
            reason: format!("{planned_requests} ≤ Budget {budget}"),
        }
    } else {
        Verdict::Hold {
            gate: "RateBudgetGate".into(),
            residue: Box::new(csa_residue(
                "rate_budget_exceeded",
                &format!("{planned_requests} Requests > Budget {budget} — Hold, keine Umgehung"),
            )),
        }
    }
}

/// DER Abruf: nur mit versiegeltem Plan. Differenzabruf: unveraenderte
/// ETags werden uebersprungen (Cache-Pflicht, CSA.10).
pub fn fetch(
    approved: &ApprovedFetchPlan,
    transport: &dyn Transport,
    cache: &mut FetchCache,
    budget: u32,
) -> Result<Vec<RawObservation>, Verdict> {
    let planned = approved
        .plan
        .operations
        .iter()
        .map(|o| o.cost_requests)
        .sum::<u32>();
    let rate = rate_budget_gate(planned, budget);
    if !rate.allows() {
        return Err(rate);
    }
    let mut out = Vec::new();
    for op in &approved.plan.operations {
        let (bytes, etag) = transport
            .get(&op.endpoint_template)
            .map_err(|e| Verdict::Hold {
                gate: "FetchGate".into(),
                residue: Box::new(csa_residue("schema_unparseable", &e)),
            })?;
        if let Some(etag) = &etag {
            if cache.etags.get(&op.endpoint_template) == Some(etag) {
                continue; // Differenzabruf: unveraendert.
            }
            cache
                .etags
                .insert(op.endpoint_template.clone(), etag.clone());
        }
        out.push(RawObservation {
            locator: op.endpoint_template.clone(),
            bytes,
            fetched_via: approved.plan.adapter_id.clone(),
            snapshot_id: approved.policy_snapshot.clone(),
        });
    }
    Ok(out)
}
