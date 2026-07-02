//! reanalyze-Parser: Markdown-Bytes → DocCrystal' — gewinnt den semantischen
//! Kern VOLLSTAENDIG zurueck (S1.3: "reanalyze verliert nichts"). Kosmetik
//! (Markup-Praefixe, Leerzeilen, Whitespace) wird weggestriffen.

use super::{DocCrystal, DocUnit, UnitType};

/// Entfernt das kosmetische Markup-Praefix einer Textzeile.
fn strip_prefix(line: &str) -> &str {
    for p in [
        "## ",
        "### Risiko: ",
        "- Gegenmassnahme: ",
        "> Beleg: ",
        "**Definition:** ",
        "1. ",
    ] {
        if let Some(rest) = line.strip_prefix(p) {
            return rest;
        }
    }
    line
}

fn parse_kv(marker: &str) -> Vec<(String, String)> {
    marker
        .split(';')
        .filter_map(|kv| {
            let mut it = kv.splitn(2, '=');
            Some((it.next()?.trim().to_string(), it.next()?.trim().to_string()))
        })
        .collect()
}

pub fn parse_markdown(bytes: &[u8]) -> Result<DocCrystal, String> {
    let text = std::str::from_utf8(bytes).map_err(|_| "kein UTF-8".to_string())?;
    let mut title = String::new();
    let mut ordering = "meaningful".to_string();
    let mut no_score_fields = false;
    let mut covers = Vec::new();
    let mut required_sections = Vec::new();
    let mut units: Vec<DocUnit> = Vec::new();
    let mut last_text_line: Option<String> = None;

    for raw_line in text.lines() {
        let line = raw_line.trim_end();
        if line.trim().is_empty() {
            continue;
        }
        if let Some(t) = line.strip_prefix("# ") {
            if title.is_empty() {
                title = t.to_string();
            }
            continue;
        }
        if let Some(body) = line
            .strip_prefix("<!--cce:doc ")
            .and_then(|s| s.strip_suffix("-->"))
        {
            for (k, v) in parse_kv(body) {
                match k.as_str() {
                    "ordering" => ordering = v,
                    "no_score_fields" => no_score_fields = v == "true",
                    "covers" => {
                        covers = v
                            .split('|')
                            .filter(|s| !s.is_empty())
                            .map(String::from)
                            .collect()
                    }
                    "sections" => {
                        required_sections = v
                            .split('|')
                            .filter(|s| !s.is_empty())
                            .map(String::from)
                            .collect()
                    }
                    _ => {}
                }
            }
            continue;
        }
        if let Some(body) = line
            .strip_prefix("<!--cce:unit ")
            .and_then(|s| s.strip_suffix("-->"))
        {
            let kvs = parse_kv(body);
            let get = |k: &str| {
                kvs.iter()
                    .find(|(key, _)| key == k)
                    .map(|(_, v)| v.clone())
                    .unwrap_or_default()
            };
            let unit_type = UnitType::parse(&get("type"))
                .ok_or_else(|| format!("unbekannter Einheitstyp: {}", get("type")))?;
            let seams = get("seams")
                .split(',')
                .filter(|s| !s.is_empty())
                .filter_map(|s| {
                    let mut it = s.splitn(2, "->");
                    Some((it.next()?.to_string(), it.next()?.to_string()))
                })
                .collect();
            let text = last_text_line
                .take()
                .ok_or_else(|| format!("Einheit {} ohne Textzeile", get("id")))?;
            units.push(DocUnit {
                id: get("id"),
                unit_type,
                text,
                seams,
            });
            continue;
        }
        last_text_line = Some(strip_prefix(line).to_string());
    }
    if title.is_empty() {
        return Err("kein Titel (# …) gefunden".to_string());
    }
    Ok(DocCrystal {
        title,
        units,
        covers,
        required_sections,
        no_score_fields,
        ordering,
    })
}
