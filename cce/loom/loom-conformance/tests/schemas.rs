//! LC-R3-Wache: die CDDL-Zweitform unter loom/schemas/ bleibt
//! deckungsgleich mit den im Code gefuehrten Vertraegen.

#[test]
fn every_contracted_kind_has_a_cddl_file() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../schemas");
    let expected = [
        "common.cddl",
        "header.cddl",
        "segtab.cddl",
        "manifest.cddl",
        "canon-desc.cddl",
        "gate-reports.cddl",
        "residue.cddl",
        "csa-nsb.cddl",
        "ledger.cddl",
        "replay-manifest.cddl",
        "provider-manifest.cddl",
        "inference-profile.cddl",
        "inference-trace.cddl",
        "candidate-outputs.cddl",
        "tool-profile.cddl",
    ];
    for f in expected {
        let p = root.join(f);
        let text = std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("{p:?}: {e}"));
        assert!(!text.trim().is_empty(), "{f} leer");
    }
    // Die 13 Manifest-Pflichtfelder stehen wortgleich in der CDDL:
    let manifest = std::fs::read_to_string(root.join("manifest.cddl")).unwrap();
    for field in loom_verify::MANIFEST_REQUIRED_FIELDS {
        assert!(
            manifest.contains(&format!("\"{field}\"")),
            "Manifest-CDDL ohne Feld {field}"
        );
    }
    // Die Overlay-Verbotskeys stehen in den jeweiligen Schemata:
    let pm = std::fs::read_to_string(root.join("provider-manifest.cddl")).unwrap();
    for bad in ["autostart", "activate_on_open", "hidden_model_call"] {
        assert!(pm.contains(bad), "provider-manifest.cddl nennt {bad} nicht");
    }
}
