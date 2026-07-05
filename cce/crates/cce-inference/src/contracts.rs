//! Kanzel-Prompt-Bibliothek (IG-R4, Dokument 23 C2): VERSIONIERTE
//! `system_contract`-Vorlagen je Einsatz — als GEPRUEFTE Assets:
//! jede Vorlage traegt Version + deterministischen Digest, die
//! Bibliothek einen Gesamt-Digest; der Waechter pinnt ihn byte-genau
//! (dieselbe Disziplin wie `seed_file_matches_builder` bei den
//! .loom-Seeds — keine stille Aenderung einer Vorlage moeglich).
//!
//! Drei Einsaetze (C2 woertlich): Wunsch-Formung, Erklaerung,
//! Reparaturvorschlag. `wunsch_formung@1.0.0` ist BYTE-IDENTISCH zu
//! dem bisher inline gefuehrten Vertragstext — die Bibliothek
//! kodifiziert den Ist-Stand, sie aendert kein Verhalten (Alt-Zeugen
//! unberuehrt). Erklaerung/Reparaturvorschlag liefern heute reine
//! Texte OHNE Gateway-Request; ihre Vertraege sind die designierten
//! `system_contract`s fuer den Moment, in dem diese Fluesse durchs
//! Gateway gehen.

use cce_core::signature::{sha256, Digest};

/// Eine versionierte Vertragsvorlage — reine Daten, kein Verhalten.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SystemContract {
    pub contract_id: &'static str,
    /// Einsatz: "wunsch_formung" | "erklaerung" | "reparaturvorschlag".
    pub use_case: &'static str,
    pub version: &'static str,
    pub text: &'static str,
}

impl SystemContract {
    /// Deterministischer Digest der Vorlage (id/use_case/version/text,
    /// 0x1f-getrennt).
    pub fn digest(&self) -> Digest {
        let mut buf = Vec::new();
        for part in [self.contract_id, self.use_case, self.version, self.text] {
            buf.extend_from_slice(part.as_bytes());
            buf.push(0x1f);
        }
        sha256(&buf)
    }
}

/// Die Bibliothek v1 — exakt drei Vorlagen (C2).
pub const KANZEL_CONTRACTS: [SystemContract; 3] = [
    SystemContract {
        contract_id: "kanzel:wunsch_formung",
        use_case: "wunsch_formung",
        version: "1.0.0",
        // BYTE-IDENTISCH zum bisherigen Inline-Vertrag der Kanzel
        // (form_wish_request) — kein Verhaltenswechsel.
        text: "annahmen als modellgeformt markieren",
    },
    SystemContract {
        contract_id: "kanzel:erklaerung",
        use_case: "erklaerung",
        version: "1.0.0",
        text: "erklaerung ist einschaetzung, kein urteil; residuen bleiben sichtbar, \
               bis der motor sie schliesst; keine zahlen als urteile",
    },
    SystemContract {
        contract_id: "kanzel:reparaturvorschlag",
        use_case: "reparaturvorschlag",
        version: "1.0.0",
        text: "vorschlag ist kandidat, kein urteil; umsetzung nur via motor-gates \
               und bestaetigung; keine aktion aus der kanzel heraus",
    },
];

/// Nachschlag je Einsatz — `None` fuer unbekannte Einsaetze (kein
/// stiller Default).
pub fn system_contract_for(use_case: &str) -> Option<&'static SystemContract> {
    KANZEL_CONTRACTS.iter().find(|c| c.use_case == use_case)
}

/// Gesamt-Digest der Bibliothek: Merkle ueber die SORTIERTEN
/// Vorlagen-Digests — der eine Wert, den der Waechter pinnt.
pub fn library_digest() -> Digest {
    let mut digests: Vec<String> = KANZEL_CONTRACTS
        .iter()
        .map(|c| c.digest().to_hex())
        .collect();
    digests.sort();
    let mut buf = Vec::new();
    for d in digests {
        buf.extend_from_slice(d.as_bytes());
        buf.push(0x1e);
    }
    sha256(&buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_contracts_versioned_and_distinct() {
        assert_eq!(KANZEL_CONTRACTS.len(), 3);
        for c in &KANZEL_CONTRACTS {
            assert_eq!(c.version, "1.0.0");
            assert!(!c.text.is_empty());
        }
        let mut ids: Vec<&str> = KANZEL_CONTRACTS.iter().map(|c| c.contract_id).collect();
        ids.dedup();
        assert_eq!(ids.len(), 3);
    }

    #[test]
    fn lookup_by_use_case_no_silent_default() {
        assert!(system_contract_for("wunsch_formung").is_some());
        assert!(system_contract_for("erklaerung").is_some());
        assert!(system_contract_for("reparaturvorschlag").is_some());
        assert!(system_contract_for("unbekannt").is_none());
    }

    #[test]
    fn library_digest_is_deterministic() {
        assert_eq!(library_digest(), library_digest());
    }
}
