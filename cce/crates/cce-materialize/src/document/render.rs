//! materialize: DocWeave → Markdown-Bytes. Fuegt NUR Kosmetik hinzu
//! (Ueberschriften-Markup, Leerzeilen) — jede semantische Information stammt
//! aus dem Gewebe (S1.3: "materialize erfindet nichts"). Struktur-Anker als
//! unsichtbare Kommentare tragen die Rueckgewinnbarkeit (verlustfrei).

use super::{decode_table, DocWeave, TableCell};

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

/// CE-1: Zellinhalt pipe-escaped (II.2) — `|` wird zu `\|`, damit die
/// Markdown-Pipe-Tabelle nicht faelschlich weitere Spalten liest.
fn cell_display(c: &TableCell) -> String {
    match c {
        TableCell::Text(s) => s.replace('|', "\\|"),
        TableCell::Int(n) => n.to_string(),
        TableCell::DecFrac { num, scale } => format_decfrac(*num, *scale),
    }
}

/// num * 10^-scale als Dezimaldarstellung, ohne Float (K3).
fn format_decfrac(num: i64, scale: u32) -> String {
    if scale == 0 {
        return num.to_string();
    }
    let sign = if num < 0 { "-" } else { "" };
    let digits = num.unsigned_abs().to_string();
    let scale = scale as usize;
    let padded = if digits.len() <= scale {
        format!("{}{digits}", "0".repeat(scale - digits.len() + 1))
    } else {
        digits
    };
    let split_at = padded.len() - scale;
    format!("{sign}{}.{}", &padded[..split_at], &padded[split_at..])
}

/// CE-1: kanonische Pipe-Tabellen-Serialisierung — feste Spaltenordnung
/// (Header-Ordnung), Zeilen in Dokumentordnung (K1/K5, nie sortiert).
fn render_pipe_table(header: &[String], rows: &[Vec<TableCell>]) -> String {
    let mut out = format!("| {} |\n", header.join(" | "));
    out.push_str(&format!(
        "| {} |\n",
        header.iter().map(|_| "---").collect::<Vec<_>>().join(" | ")
    ));
    for row in rows {
        let cells: Vec<String> = row.iter().map(cell_display).collect();
        out.push_str(&format!("| {} |\n", cells.join(" | ")));
    }
    out
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
        let seams = b
            .seams
            .iter()
            .map(|(k, t)| format!("{k}->{t}"))
            .collect::<Vec<_>>()
            .join(",");
        if b.unit_type == "table" {
            // CE-1 (II.2): der Anker steht VOR der Kopfzeile — anders als
            // bei jedem anderen Einheitstyp (dort nach dem Text).
            out.push_str(&format!(
                "<!--cce:unit id={};type={};seams={}-->\n",
                b.unit_id, b.unit_type, seams
            ));
            match decode_table(&b.text) {
                Some(t) => out.push_str(&render_pipe_table(&t.header, &t.rows)),
                None => out.push_str("<!--cce:table-undecodable-->\n"),
            }
            out.push('\n');
        } else {
            out.push_str(prefix(&b.unit_type));
            out.push_str(&b.text);
            out.push('\n');
            out.push_str(&format!(
                "<!--cce:unit id={};type={};seams={}-->\n\n",
                b.unit_id, b.unit_type, seams
            ));
        }
    }
    out
}
