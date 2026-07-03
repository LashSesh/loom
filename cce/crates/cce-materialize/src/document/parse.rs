//! reanalyze-Parser: Markdown-Bytes → DocCrystal' — gewinnt den semantischen
//! Kern VOLLSTAENDIG zurueck (S1.3: "reanalyze verliert nichts"). Kosmetik
//! (Markup-Praefixe, Leerzeilen, Whitespace) wird weggestriffen.

use super::{encode_table, DocCrystal, DocUnit, TableCell, UnitType};

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

/// CE-1 (II.2): splittet eine Pipe-Tabellenzeile in Zellinhalte,
/// respektiert `\|` als escapetes Pipe-Zeichen (nicht als Trenner).
fn split_pipe_row(line: &str) -> Vec<String> {
    let trimmed = line.trim();
    let trimmed = trimmed.strip_prefix('|').unwrap_or(trimmed);
    let trimmed = trimmed.strip_suffix('|').unwrap_or(trimmed);
    let mut cells = Vec::new();
    let mut cur = String::new();
    let mut chars = trimmed.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\\' && chars.peek() == Some(&'|') {
            cur.push('|');
            chars.next();
        } else if c == '|' {
            cells.push(cur.trim().to_string());
            cur = String::new();
        } else {
            cur.push(c);
        }
    }
    cells.push(cur.trim().to_string());
    cells
}

/// CE-1: rekonstruiert eine Zelle aus ihrer Textform — Int/DecFrac
/// werden erkannt, sonst Text (mit `\|`→`|` entschaerft, s.
/// `split_pipe_row`, das bereits entschaerft hat).
fn parse_cell(s: &str) -> TableCell {
    if let Ok(n) = s.parse::<i64>() {
        return TableCell::Int(n);
    }
    if let Some((int_part, frac_part)) = s.split_once('.') {
        let int_digits = int_part.strip_prefix('-').unwrap_or(int_part);
        if !frac_part.is_empty()
            && frac_part.chars().all(|c| c.is_ascii_digit())
            && !int_digits.is_empty()
            && int_digits.chars().all(|c| c.is_ascii_digit())
        {
            let combined = format!("{int_part}{frac_part}");
            if let Ok(num) = combined.parse::<i64>() {
                return TableCell::DecFrac {
                    num,
                    scale: frac_part.len() as u32,
                };
            }
        }
    }
    TableCell::Text(s.to_string())
}

/// CE-1 (II.2): parst die Pipe-Tabellenzeilen (Kopfzeile, Trennzeile,
/// Datenzeilen) EINER Tabelle zurueck in `(header, rows)`.
fn parse_pipe_table(lines: &[&str]) -> Result<(Vec<String>, Vec<Vec<TableCell>>), String> {
    let mut it = lines.iter();
    let header_line = it.next().ok_or("Tabelle ohne Kopfzeile")?;
    let header = split_pipe_row(header_line);
    it.next(); // Trennzeile (| --- | --- |) — Struktur, kein Inhalt.
    let rows = it
        .map(|l| split_pipe_row(l).iter().map(|s| parse_cell(s)).collect())
        .collect();
    Ok((header, rows))
}

pub fn parse_markdown(bytes: &[u8]) -> Result<DocCrystal, String> {
    let text = std::str::from_utf8(bytes).map_err(|_| "kein UTF-8".to_string())?;
    let lines: Vec<&str> = text.lines().collect();
    let mut title = String::new();
    let mut ordering = "meaningful".to_string();
    let mut no_score_fields = false;
    let mut covers = Vec::new();
    let mut required_sections = Vec::new();
    let mut units: Vec<DocUnit> = Vec::new();
    let mut last_text_line: Option<String> = None;
    let mut i = 0usize;

    while i < lines.len() {
        let line = lines[i].trim_end();
        if line.trim().is_empty() {
            i += 1;
            continue;
        }
        if let Some(t) = line.strip_prefix("# ") {
            if title.is_empty() {
                title = t.to_string();
            }
            i += 1;
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
            i += 1;
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
            let id = get("id");
            if unit_type == UnitType::Table {
                // CE-1 (II.2): der Anker steht VOR der Kopfzeile — die
                // Tabellenzeilen folgen erst NACH diesem Anker (Look-ahead
                // statt `last_text_line`, anders als jeder andere Typ).
                i += 1;
                let mut table_lines: Vec<&str> = Vec::new();
                while i < lines.len() && lines[i].trim_start().starts_with('|') {
                    table_lines.push(lines[i]);
                    i += 1;
                }
                let (header, rows) =
                    parse_pipe_table(&table_lines).map_err(|e| format!("Tabelle {id}: {e}"))?;
                units.push(DocUnit {
                    id,
                    unit_type,
                    text: encode_table(&header, &rows),
                    seams,
                });
                continue;
            }
            let text = last_text_line
                .take()
                .ok_or_else(|| format!("Einheit {id} ohne Textzeile"))?;
            units.push(DocUnit {
                id,
                unit_type,
                text,
                seams,
            });
            i += 1;
            continue;
        }
        last_text_line = Some(strip_prefix(line).to_string());
        i += 1;
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
