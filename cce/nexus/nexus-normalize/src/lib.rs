//! nexus-normalize — ExtractedRecord → CSU (CSA.5): Kanonisierung,
//! UID = ContentAddress der kanonischen Bytes, Provenienz-Bindung.

use cce_core::canonical::Canonicalize;
use nexus_adapter::port::ExtractedRecord;
use nexus_core::objects::Csu;

/// Baut die CSU aus dem extrahierten Datensatz. Die UID ist die
/// Content-Adresse der KANONISCHEN Payload — gleiche kanonische Klasse
/// ⇒ gleiche UID (Dedup-Grundlage, CSA.5).
pub fn normalize_record(
    record: &ExtractedRecord,
    kind: &str,
    schema: &str,
    quality: (u16, u16, u16),
    license: &str,
    source_hash: cce_core::signature::Digest,
    domain_facet: &str,
) -> Csu {
    let payload = record.fields.normalize();
    let mut csu = Csu {
        uid: String::new(),
        kind: kind.to_string(),
        payload,
        schema: schema.to_string(),
        quality,
        provenance: record.locator.clone(),
        license: license.to_string(),
        source_hash,
        residues: Vec::new(),
        domain_facet: domain_facet.to_string(),
    };
    csu.uid = format!("csu:{}", csu.canonical_class().0.to_hex());
    csu
}

#[cfg(test)]
mod tests {
    use super::*;
    use cce_core::value::CanonValue;
    use std::collections::BTreeMap;

    fn rec(locator: &str, v: &str) -> ExtractedRecord {
        let mut m = BTreeMap::new();
        m.insert("titel".to_string(), CanonValue::Text(v.to_string()));
        ExtractedRecord {
            record_id: "r1".to_string(),
            locator: locator.to_string(),
            fields: CanonValue::Map(m),
        }
    }

    #[test]
    fn same_canonical_payload_same_uid() {
        let a = normalize_record(
            &rec("l://1", "A"),
            "doc",
            "kv",
            (900, 900, 900),
            "cc-by-4.0",
            cce_core::signature::sha256(b"h1"),
            "docs",
        );
        let b = normalize_record(
            &rec("l://2", "A"),
            "doc",
            "kv",
            (900, 900, 900),
            "cc-by-4.0",
            cce_core::signature::sha256(b"h1"),
            "docs",
        );
        let c = normalize_record(
            &rec("l://3", "B"),
            "doc",
            "kv",
            (900, 900, 900),
            "cc-by-4.0",
            cce_core::signature::sha256(b"h1"),
            "docs",
        );
        assert_eq!(a.uid, b.uid);
        assert_ne!(a.uid, c.uid);
    }
}
