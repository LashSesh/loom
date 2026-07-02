//! materialize: DocWeave → Markdown-Bytes. Fuegt NUR Kosmetik hinzu
//! (Ueberschriften-Markup, Leerzeilen) — jede semantische Information stammt
//! aus dem Gewebe (S1.3: "materialize erfindet nichts"). Struktur-Anker als
//! unsichtbare Kommentare tragen die Rueckgewinnbarkeit (verlustfrei).

use super::DocWeave;

/// Kosmetisches Praefix je Einheitstyp (reine Darstellung).
fn prefix(unit_type: &str) -> &'static str {
    match unit_type {
        "section" => "## ",
        "risk" => "### Risiko: ",
        "countermeasure" => "- Gegenmassnahme: ",
        "claim" => "",
        "support" => "> Beleg: ",
        "definition" => "**Definition:** ",
        "step" => "1. ",
        _ => "",
    }
}

pub fn render_markdown(w: &DocWeave) -> String {
    let mut out = String::new();
    out.push_str(&format!("# {}\n\n", w.title));
    out.push_str(&format!(
        "<!--cce:doc ordering={};no_score_fields={};covers={};sections={}-->\n\n",
        w.weave.ordering,
        w.no_score_fields,
        w.covers.join("|"),
        w.required_sections.join("|"),
    ));
    for b in &w.weave.blocks {
        out.push_str(prefix(&b.unit_type));
        out.push_str(&b.text);
        out.push('\n');
        let seams = b
            .seams
            .iter()
            .map(|(k, t)| format!("{k}->{t}"))
            .collect::<Vec<_>>()
            .join(",");
        out.push_str(&format!(
            "<!--cce:unit id={};type={};seams={}-->\n\n",
            b.unit_id, b.unit_type, seams
        ));
    }
    out
}
