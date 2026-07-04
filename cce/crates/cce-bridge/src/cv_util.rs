//! Kleiner geteilter Cv-Zugriffshelfer (dieselbe Handvoll Zeilen wie
//! `loom-conformance::cv_get` und `loom-cites`s interne Getter) — kein
//! generischer Cv-Zugriffspfad existiert in `loom-canon` selbst, jede
//! Kern-Bibliothek schreibt sich ihren eigenen minimalen Getter.

use loom_canon::Cv;

pub(crate) fn cv_get<'a>(map: &'a Cv, key: &str) -> Option<&'a Cv> {
    if let Cv::Map(entries) = map {
        entries.iter().find_map(|(k, v)| match k {
            Cv::Text(s) if s == key => Some(v),
            _ => None,
        })
    } else {
        None
    }
}
