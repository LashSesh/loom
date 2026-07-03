//! Wikimedia-OfficialAPIAdapter — Referenzadapter Klasse `official_api`
//! (CSA.15): offizielle API, deklarierte Rate, CC-BY-SA MIT
//! ATTRIBUTIONS-TRANSPORT (PROD-INV-16). Der Netzpfad laeuft in
//! nexus-fetch hinter dem PolicyGate — hier nur Plan/Dekodierung.

use cce_core::gate::GateReport;
use cce_core::value::CanonValue;
use nexus_adapter::manifest::AdapterManifest;
use nexus_adapter::port::{ExtractedRecord, FetchPlan, PlannedOp, SourceAdapter};
use nexus_core::objects::{Csu, RawObservation, TaskSpec};
use nexus_core::residues::csa_residue;
use nexus_decode::decode_json;
use nexus_normalize::normalize_record;

pub struct WikimediaAdapter;

pub const WIKIMEDIA_LICENSE: &str = "cc-by-sa-4.0";

impl SourceAdapter for WikimediaAdapter {
    fn manifest(&self) -> AdapterManifest {
        let mut m = AdapterManifest::complete("wikimedia-api", "official_api", WIKIMEDIA_LICENSE);
        m.policy = Some("official_api_terms".to_string());
        m.rate_limit_per_run = Some(2);
        m
    }

    fn preflight(&self, task: &TaskSpec) -> GateReport {
        if task.budget_requests == 0 {
            GateReport::hold("preflight:wikimedia", "Budget 0 — kein Abruf planbar")
        } else {
            GateReport::pass("preflight:wikimedia", "offizielle API, Budget vorhanden")
        }
    }

    fn plan(&self, task: &TaskSpec, budget_requests: u32) -> FetchPlan {
        FetchPlan {
            plan_id: format!("plan:wikimedia:{}", task.scope),
            adapter_id: "wikimedia-api".to_string(),
            operations: vec![PlannedOp {
                method: "get".to_string(),
                endpoint_template: "api://wikimedia/page/{title}".to_string(),
                expected_status: 200,
                cost_requests: budget_requests.min(2),
            }],
            gates: vec![
                "rate_budget_gate".to_string(),
                "schema_gate".to_string(),
                "quality_gate".to_string(),
                "evidence_gate".to_string(),
            ],
            cache_etag: Some("etag:wikimedia".to_string()),
            cache_cursor: None,
            backoff_policy: "exponential".to_string(),
        }
    }

    fn acquire(&self, raw: &[RawObservation]) -> Vec<RawObservation> {
        raw.to_vec()
    }

    /// Extrahiert aus der ECHTEN MediaWiki-Action-API-JSON-Antwort
    /// (`action=query&prop=extracts&...&format=json`): dekodiert das
    /// volle JSON und navigiert `query.pages.<Seiten-ID>` — bei einer
    /// Einzeltitel-Abfrage genau EIN Eintrag, dessen numerischer
    /// Schluessel nicht vorhersagbar ist (MediaWiki vergibt echte
    /// Seiten-IDs). Der komplette Seiteneintrag (title/pageid/extract/…)
    /// wandert unveraendert weiter — nichts wird stillschweigend verworfen.
    fn extract(&self, raw: &RawObservation) -> Result<Vec<ExtractedRecord>, String> {
        let root = decode_json(raw).map_err(|r| r.id.clone())?;
        let page = root
            .get("query")
            .and_then(|q| q.get("pages"))
            .and_then(|pages| match pages {
                CanonValue::Map(m) => m.values().next(),
                _ => None,
            })
            .ok_or_else(|| {
                csa_residue(
                    "schema_unparseable",
                    &format!("{}: query.pages fehlt oder ist leer", raw.locator),
                )
                .id
            })?;
        if !matches!(page, CanonValue::Map(_)) {
            return Err(csa_residue(
                "schema_unparseable",
                &format!("{}: Seiteneintrag ist keine Map", raw.locator),
            )
            .id);
        }
        Ok(vec![ExtractedRecord {
            record_id: format!("rec:{}", raw.digest().to_hex()),
            locator: raw.locator.clone(),
            fields: page.clone(),
        }])
    }

    fn normalize(&self, record: &ExtractedRecord) -> Vec<Csu> {
        vec![normalize_record(
            record,
            "wiki_page",
            "mediawiki_extract_json",
            (850, 900, 950),
            WIKIMEDIA_LICENSE,
            cce_core::signature::sha256(record.locator.as_bytes()),
            "encyclopedia",
        )]
    }

    fn validate(&self, unit: &Csu) -> GateReport {
        if !unit.license.starts_with("cc-by-sa") {
            return GateReport::hold("validate:wikimedia", "Lizenzangabe fehlt");
        }
        let has_title = matches!(unit.payload.get("title"), Some(CanonValue::Text(_)));
        let has_extract = matches!(unit.payload.get("extract"), Some(CanonValue::Text(_)));
        if !has_title || !has_extract {
            return GateReport::hold(
                "validate:wikimedia",
                "erwartetes Schema (title/extract) fehlt in der Payload",
            );
        }
        GateReport::pass("validate:wikimedia", "Lizenz + CSU wohlgeformt")
    }

    /// Attribution ist hier PFLICHT-Transportgut (PROD-INV-16).
    fn cite(&self, unit: &Csu) -> Vec<String> {
        vec![format!(
            "Wikimedia-Beitraegerinnen und -Beitraeger · {} · Lizenz {} · hash {}",
            unit.provenance,
            unit.license,
            unit.source_hash.to_hex()
        )]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nexus_adapter::port::check_source_adapter_parity;

    /// Echtes MediaWiki-Action-API-Antwortschema (`query.pages.<id>`),
    /// nicht das alte kv-Zeilen-Platzhalterformat — die numerische
    /// Seiten-ID ist bewusst eine andere als in der eingefrorenen
    /// Kristall-Fixture, um zu zeigen, dass die Navigation ueber
    /// `.values().next()` nicht auf eine feste ID angewiesen ist.
    fn synthetic_mediawiki_json(title: &str, extract: &str) -> Vec<u8> {
        format!(
            r#"{{"batchcomplete":"","query":{{"pages":{{"999":{{"pageid":999,"ns":0,"title":"{title}","extract":"{extract}"}}}}}}}}"#
        )
        .into_bytes()
    }

    #[test]
    fn parity_and_attribution_present() {
        let a = WikimediaAdapter;
        assert!(check_source_adapter_parity(&a).is_pass());
        let raw = RawObservation {
            locator: "api://wikimedia/page/Kristall".to_string(),
            bytes: synthetic_mediawiki_json("Kristall", "Festkoerper"),
            fetched_via: "snapshot".to_string(),
            snapshot_id: "snap-w1".to_string(),
        };
        let recs = a.extract(&raw).unwrap();
        let csu = &a.normalize(&recs[0])[0];
        assert!(a.validate(csu).is_pass());
        let cites = a.cite(csu);
        assert!(
            cites[0].contains("cc-by-sa"),
            "Attribution muss Lizenz transportieren"
        );
        assert!(cites[0].contains("Beitraeger"));
    }

    #[test]
    fn extract_fails_closed_on_non_json_and_missing_pages() {
        let a = WikimediaAdapter;
        let not_json = RawObservation {
            locator: "api://wikimedia/page/X".to_string(),
            bytes: b"titel: Kristall\nauszug: Festkoerper".to_vec(),
            fetched_via: "snapshot".to_string(),
            snapshot_id: "snap-w2".to_string(),
        };
        assert!(
            a.extract(&not_json)
                .unwrap_err()
                .contains("schema_unparseable"),
            "kv-Zeilen sind kein gueltiges JSON mehr — muss fail-closed sein"
        );
        let no_pages = RawObservation {
            locator: "api://wikimedia/page/X".to_string(),
            bytes: br#"{"query":{"pages":{}}}"#.to_vec(),
            fetched_via: "snapshot".to_string(),
            snapshot_id: "snap-w3".to_string(),
        };
        assert!(a
            .extract(&no_pages)
            .unwrap_err()
            .contains("schema_unparseable"));
    }

    /// Der Milestone-Zeuge: die tatsaechliche, eingefrorene Wikimedia-
    /// Antwort (conformance/fixtures/wikimedia_kristall.json) wird zur
    /// vollwertigen CSU — mit echtem Titel/Auszug aus der echten Quelle,
    /// nicht nur der Fetch-Byte-Stabilitaet (das war P5/Track D).
    #[test]
    fn real_frozen_wikimedia_fixture_becomes_a_real_csu() {
        let a = WikimediaAdapter;
        let fixture = include_bytes!("../../../../conformance/fixtures/wikimedia_kristall.json");
        let raw = RawObservation {
            locator:
                "https://de.wikipedia.org/w/api.php?action=query&prop=extracts&titles=Kristall&format=json"
                    .to_string(),
            bytes: fixture.to_vec(),
            fetched_via: "snapshot-frozen".to_string(),
            snapshot_id: "snap-wikimedia-frozen".to_string(),
        };
        let recs = a
            .extract(&raw)
            .expect("echte JSON-Antwort muss extrahieren");
        assert_eq!(recs.len(), 1);
        let csu = &a.normalize(&recs[0])[0];
        assert!(a.validate(csu).is_pass(), "{:?}", a.validate(csu));
        assert_eq!(
            csu.payload.get("title"),
            Some(&CanonValue::Text("Kristall".to_string()))
        );
        let extract = match csu.payload.get("extract") {
            Some(CanonValue::Text(t)) => t.clone(),
            other => panic!("extract muss Text sein: {other:?}"),
        };
        assert!(extract.contains("Kristallographie"), "{extract}");
        assert_eq!(csu.license, WIKIMEDIA_LICENSE);
        let cites = a.cite(csu);
        assert!(cites[0].contains("cc-by-sa-4.0"));
    }
}
