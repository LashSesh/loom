//! cce-docx-export — .docx-Export hinter demselben materialize-Vertrag
//! (S1.8 Punkt 5: Gewebe -> Artefakt, "materialize erfindet nichts")
//! wie `document::render::render_markdown` — nur die Ausgabeform ist
//! anders. Schliesst S1.10-R1 ("Bibliothekswahl offen"): die Wahl ist
//! getroffen (zip + handgeschriebenes Minimal-OOXML), NUR in diesem
//! Blatt-Crate — cce-materialize selbst bekommt keine neue Abhaengigkeit.
//!
//! Verlustform (Muster aus cockpit-core::journey::export_lossy_plaintext):
//! .docx traegt KEINE Struktur-Anker (`<!--cce:unit ...-->`) — anders als
//! .md ist das kein verlustfreier Reanalyse-Pfad. `format_loss_residue()`
//! macht das sichtbar, statt es zu verschweigen (S2.5/S7.4).

use cce_materialize::document::DocWeave;
use std::io::{Read, Write};

pub struct DocxArtifact {
    pub bytes: Vec<u8>,
    pub format: &'static str,
}

impl DocxArtifact {
    /// Der Verlust ist STRUKTURELL (keine Anker in .docx moeglich in
    /// dieser einfachen, lesbaren Form) — immer vorhanden, nie bedingt
    /// weggelassen.
    pub fn format_loss_residue() -> cce_core::residue::Residue {
        cce_core::residue::Residue::new(
            "s1.10:docx_format_loss",
            "docx_export",
            cce_core::residue::ResidueKind::named("format_loss"),
            cce_core::residue::Severity::Warning,
            ".docx traegt keine Struktur-Anker (<!--cce:unit...-->) — \
             Re-Import aus .docx kann die Kristall-Klasse NICHT rekonstruieren; \
             .md bleibt der verlustfreie Reanalyse-Pfad (bewusste \
             Operator-Entscheidung, S2.5).",
        )
    }
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn xml_unescape(s: &str) -> String {
    s.replace("&quot;", "\"")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
}

#[derive(Debug)]
pub enum RoundtripError {
    NotAZip(String),
    NoDocumentXml(String),
}

/// "docx-Roundtrip-Klasse" (X1d-Exit-Zeuge): liest `word/document.xml`
/// aus den .docx-Bytes zurueck und extrahiert jeden `<w:t>`-Textlauf in
/// Dokumentreihenfolge. KEIN Struktur-Reimport (keine Anker, s.
/// `format_loss_residue`) — beweist Inhalts-Roundtrip auf Textebene:
/// dieselben Woerter kommen zurueck, die `render_docx` hineingelegt hat.
pub fn extract_paragraph_texts(docx_bytes: &[u8]) -> Result<Vec<String>, RoundtripError> {
    let cursor = std::io::Cursor::new(docx_bytes);
    let mut zip =
        zip::ZipArchive::new(cursor).map_err(|e| RoundtripError::NotAZip(e.to_string()))?;
    let mut xml = String::new();
    zip.by_name("word/document.xml")
        .map_err(|e| RoundtripError::NoDocumentXml(e.to_string()))?
        .read_to_string(&mut xml)
        .map_err(|e| RoundtripError::NoDocumentXml(e.to_string()))?;

    let mut texts = Vec::new();
    let mut rest = xml.as_str();
    while let Some(start) = rest.find("<w:t") {
        let after_tag = &rest[start..];
        let Some(gt) = after_tag.find('>') else {
            break;
        };
        let content_start = start + gt + 1;
        let Some(end_rel) = rest[content_start..].find("</w:t>") else {
            break;
        };
        let content_end = content_start + end_rel;
        texts.push(xml_unescape(&rest[content_start..content_end]));
        rest = &rest[content_end + "</w:t>".len()..];
    }
    Ok(texts)
}

fn paragraph_xml(text: &str, heading: bool) -> String {
    let style = if heading {
        "<w:pPr><w:pStyle w:val=\"Heading1\"/></w:pPr>"
    } else {
        ""
    };
    format!(
        "<w:p>{style}<w:r><w:t xml:space=\"preserve\">{}</w:t></w:r></w:p>",
        xml_escape(text)
    )
}

/// Rendert dieselbe `DocWeave`-Eingabe wie `render_markdown` — NUR die
/// Ausgabeform (.docx statt .md) ist anders; kein neuer Inhalt entsteht
/// (S1.3 "materialize erfindet nichts").
pub fn render_docx(w: &DocWeave) -> DocxArtifact {
    let mut body = paragraph_xml(&w.title, true);
    for b in &w.weave.blocks {
        let heading = b.unit_type == "section";
        body.push_str(&paragraph_xml(&b.text, heading));
    }
    let document_xml = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\
         <w:document xmlns:w=\"http://schemas.openxmlformats.org/wordprocessingml/2006/main\">\
         <w:body>{body}<w:sectPr/></w:body></w:document>"
    );
    DocxArtifact {
        bytes: build_zip(&document_xml),
        format: ".docx",
    }
}

const CONTENT_TYPES: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\
<Types xmlns=\"http://schemas.openxmlformats.org/package/2006/content-types\">\
<Default Extension=\"rels\" ContentType=\"application/vnd.openxmlformats-package.relationships+xml\"/>\
<Default Extension=\"xml\" ContentType=\"application/xml\"/>\
<Override PartName=\"/word/document.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml\"/>\
</Types>";

const ROOT_RELS: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\
<Relationships xmlns=\"http://schemas.openxmlformats.org/package/2006/relationships\">\
<Relationship Id=\"rId1\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument\" Target=\"word/document.xml\"/>\
</Relationships>";

fn build_zip(document_xml: &str) -> Vec<u8> {
    let mut buf = Vec::new();
    {
        let mut zip = zip::ZipWriter::new(std::io::Cursor::new(&mut buf));
        let opts = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);
        zip.start_file("[Content_Types].xml", opts)
            .expect("zip start_file");
        zip.write_all(CONTENT_TYPES.as_bytes()).expect("zip write");
        zip.start_file("_rels/.rels", opts).expect("zip start_file");
        zip.write_all(ROOT_RELS.as_bytes()).expect("zip write");
        zip.start_file("word/document.xml", opts)
            .expect("zip start_file");
        zip.write_all(document_xml.as_bytes()).expect("zip write");
        zip.finish().expect("zip finish");
    }
    buf
}

#[cfg(test)]
mod tests {
    use super::*;
    use cce_materialize::adapter::DomainAdapter;
    use cce_materialize::document::assets::three_risks_memo;
    use cce_materialize::document::DocumentAdapter;
    use cce_phc::projection_calc::project;

    /// Derselbe Pfad wie cce-runner intern (encode -> project -> loom) —
    /// keine handgebaute Projektion, sondern die echte Motor-Kette.
    fn weave_from(crystal: &cce_materialize::document::DocCrystal) -> DocWeave {
        let adapter = DocumentAdapter;
        let pkg = adapter.encode(crystal);
        let proj = project(&pkg, "proj:materialize").expect("projizieren");
        adapter.loom(&proj).expect("weben")
    }

    #[test]
    fn docx_is_a_valid_zip_with_the_expected_parts() {
        let crystal = three_risks_memo();
        let weave = weave_from(&crystal);
        let artifact = render_docx(&weave);
        assert_eq!(artifact.format, ".docx");
        assert!(!artifact.bytes.is_empty());

        let cursor = std::io::Cursor::new(&artifact.bytes);
        let mut zip = zip::ZipArchive::new(cursor).expect("gueltiges ZIP");
        let names: Vec<String> = (0..zip.len())
            .map(|i| zip.by_index(i).unwrap().name().to_string())
            .collect();
        assert!(names.contains(&"[Content_Types].xml".to_string()));
        assert!(names.contains(&"_rels/.rels".to_string()));
        assert!(names.contains(&"word/document.xml".to_string()));

        let mut doc_xml = String::new();
        zip.by_name("word/document.xml")
            .unwrap()
            .read_to_string(&mut doc_xml)
            .unwrap();
        // Der echte Memo-Inhalt muss lesbar im Dokument stehen — kein
        // leeres Geruest.
        assert!(doc_xml.contains("Serverausfall"));
        assert!(doc_xml.contains("Failover-Cluster"));
    }

    #[test]
    fn docx_rendering_is_deterministic() {
        let crystal = three_risks_memo();
        let weave = weave_from(&crystal);
        let a = render_docx(&weave);
        let b = render_docx(&weave);
        assert_eq!(a.bytes, b.bytes);
    }

    #[test]
    fn xml_special_characters_are_escaped() {
        let escaped = xml_escape("A & B <tag> \"quote\"");
        assert_eq!(escaped, "A &amp; B &lt;tag&gt; &quot;quote&quot;");
    }

    #[test]
    fn format_loss_residue_is_always_present_and_named() {
        let r = DocxArtifact::format_loss_residue();
        assert_eq!(r.kind, cce_core::residue::ResidueKind::named("format_loss"));
        assert!(r.content.contains(".docx"));
    }

    /// "docx-Roundtrip-Klasse" (X1d-Exit-Zeuge): Inhalts-Roundtrip auf
    /// Textebene — dieselben Woerter, dieselbe Reihenfolge wie im
    /// Gewebe, OHNE Struktur-Anker (die sind bewusst verloren, s.
    /// format_loss_residue). Kein Byte-Vergleich zu .md noetig — der
    /// Vergleich ist gegen die Quelle (DocWeave), nicht gegen das
    /// andere Format.
    #[test]
    fn docx_roundtrip_recovers_the_same_text_content() {
        let crystal = three_risks_memo();
        let weave = weave_from(&crystal);
        let artifact = render_docx(&weave);

        let recovered = extract_paragraph_texts(&artifact.bytes).expect("Roundtrip");
        let mut expected = vec![weave.title.clone()];
        expected.extend(weave.weave.blocks.iter().map(|b| b.text.clone()));
        assert_eq!(recovered, expected);
    }

    #[test]
    fn docx_roundtrip_fails_closed_on_non_docx_bytes() {
        assert!(matches!(
            extract_paragraph_texts(b"kein zip"),
            Err(RoundtripError::NotAZip(_))
        ));
    }
}
