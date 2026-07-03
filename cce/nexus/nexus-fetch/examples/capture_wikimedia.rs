//! Live-Capture (P5): holt EINMAL die offizielle Wikimedia-API ueber den
//! versiegelten Pfad und friert die Antwort als Fixture ein. Aufruf:
//!   cargo run -p nexus-fetch --features http --example capture_wikimedia -- <out.json>
//! Danach laeuft der committete Zeuge deterministisch gegen die Fixture.
#[cfg(feature = "http")]
fn main() {
    use nexus_adapter::manifest::AdapterManifest;
    use nexus_adapter::port::{FetchPlan, PlannedOp};
    use nexus_core::objects::{SourceHorizon, TaskSpec};
    use nexus_fetch::{fetch, FetchCache, HttpTransport};
    use nexus_policy::{approve_fetch, SourceDeclaration};

    let out = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "wikimedia.json".to_string());
    let endpoint = "https://de.wikipedia.org/w/api.php?action=query&prop=extracts&exintro=1&explaintext=1&titles=Kristall&format=json&redirects=1";

    let hs = SourceHorizon::example_local();
    let task = TaskSpec::new("wikimedia-capture", "scope:docs");
    let manifest = AdapterManifest::complete("wikimedia-api", "official_api", "cc-by-sa-4.0");
    let decl = SourceDeclaration {
        adapter_class: "official_api".to_string(),
        access_method: "api_key_configured".to_string(),
        robots_or_terms: Some("permitted".to_string()),
        license: Some("cc-by-sa-4.0".to_string()),
        contains_pii: false,
        requested_actions: vec![],
    };
    let plan = FetchPlan {
        plan_id: "plan:wikimedia:live".to_string(),
        adapter_id: "wikimedia-api".to_string(),
        operations: vec![PlannedOp {
            method: "get".to_string(),
            endpoint_template: endpoint.to_string(),
            expected_status: 200,
            cost_requests: 1,
        }],
        gates: vec!["rate_budget_gate".to_string()],
        cache_etag: None,
        cache_cursor: None,
        backoff_policy: "exponential".to_string(),
    };
    // VERSIEGELTER Pfad: erst approve_fetch (alle Policy-Gates), dann fetch.
    let approved = approve_fetch(&hs, &task, &manifest, &decl, plan).expect("Policy gruen");
    let transport = HttpTransport::new("cce-loom-csa/0.1 (contact: operator)");
    let mut cache = FetchCache::default();
    let raws = fetch(&approved, &transport, &mut cache, 8).expect("Fetch");
    let raw = raws.into_iter().next().expect("eine Beobachtung");
    std::fs::write(&out, &raw.bytes).expect("Fixture schreiben");
    println!("eingefroren: {} ({} Bytes)", out, raw.bytes.len());
}

#[cfg(not(feature = "http"))]
fn main() {
    eprintln!("baue mit --features http");
}
