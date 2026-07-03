//! Zeuge (Block 2, Track C): die Kanzel-Formung nutzt nachweislich den
//! echten LocalExtractiveModel-Pfad ueber das unveraenderte
//! InferenceGateway — kein Stub, kein fester String. DegradedKanzel
//! bleibt unberuehrt (Kanzel-Aus-Pfad).

use cockpit_core::kanzel::{DegradedKanzel, KanzelPort, LocalKanzel, INTERPRETATION_MARKER};

#[test]
fn kanzel_formation_uses_real_model_not_canned_string() {
    let k = LocalKanzel;
    let (_c1, i1) = k
        .form_wish("Serverausfall gefaehrdet den Betrieb. Redundanz senkt das Ausfallrisiko.")
        .unwrap();
    let (_c2, i2) = k
        .form_wish("Ein voellig anderer Text ueber Lieferketten und Zweitlieferanten.")
        .unwrap();
    // Verschiedener Wunsch ⇒ verschiedene Interpretation: kein fester String,
    // die Ausgabe ist eine echte Funktion des Kontexts (wie
    // LocalExtractiveModel::draft selbst beweist).
    assert_ne!(
        i1.text, i2.text,
        "Kanzel-Interpretation darf nicht kanoniert/fest sein"
    );
    // Beide tragen den echten Provider-/Evidence-Fussabdruck des Gateways —
    // das ist der Unterschied zu einem Stub (der keine Provider-Herkunft
    // haette).
    assert!(
        i1.text.contains("local-extractive:kernmodell"),
        "kein Modellpfad-Beleg: {}",
        i1.text
    );
    assert!(
        i1.text.contains("Beleg iev:"),
        "keine Gateway-Evidence: {}",
        i1.text
    );
    assert!(i2.text.contains("local-extractive:kernmodell"));
    // COCK-INV-4 bleibt unveraendert: jede Ausgabe markiert.
    assert_eq!(i1.marker, INTERPRETATION_MARKER);
    assert_eq!(i2.marker, INTERPRETATION_MARKER);
}

#[test]
fn kanzel_model_text_actually_reflects_context_terms() {
    // Nicht nur "verschieden", sondern INHALTLICH aus dem Kontext:
    // dasselbe Kernkonzept wie im LocalExtractiveModel-Zeugen selbst.
    let k = LocalKanzel;
    let (_c, i) = k
        .form_wish(
            "Redundanz senkt das Ausfallrisiko deutlich. Redundanz ist zentral fuer den Betrieb.",
        )
        .unwrap();
    assert!(
        i.text.to_lowercase().contains("redundanz"),
        "Modelltext soll das dominante Kontext-Konzept extrahieren: {}",
        i.text
    );
}

#[test]
fn degraded_kanzel_path_untouched_no_gateway_access() {
    // Kanzel-Aus liefert weiterhin None — kein Motor-/Gateway-Zugriff,
    // sichtbar degradiert, Cockpit bleibt voll bedienbar (COCK-INV-Kern).
    assert!(DegradedKanzel.form_wish("beliebiger wunsch").is_none());
    assert_eq!(
        DegradedKanzel.status(),
        "degradiert (kein Anbieter konfiguriert) — Kern voll funktionsfaehig"
    );
}
