//! cce-benchmark — P4/Dokument 20: Vergleichslaeufe. Blatt-Crate, keine
//! neue Kern-Logik. Isoliert exakt die Variable, um die es geht: kostet
//! die Beweispflicht-Schicht (Gate/Evidence/Replay/Zertifizierung)
//! etwas, oder ist sie kostenlos (oder sogar ein Gewinn)? Beide Arme —
//! ungegatet und CCE — nutzen dasselbe zugrundeliegende Modell; der
//! einzige Unterschied ist, ob die Gates dazwischenliegen. Der CCE-Arm
//! bindet den UNVERAENDERTEN `cce-swe`-Kern (P2/P3, generalisiert auf ein
//! beliebiges Zielpaket statt CCEs eigenen Baum). Der Benchmark-
//! Vergleich selbst wird ein zertifizierter `.loom`-Koerper der Klasse
//! `"benchmark"`.

pub mod assemble;
pub mod gates;
pub mod matrix;
pub mod model;
pub mod residues;
pub mod workbody;
