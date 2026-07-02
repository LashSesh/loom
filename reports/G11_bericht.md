Phase: G11 Produktreise, Doku, Feinschluss
Eingang erfüllt: ja (G10-Gate grün, reports/G10_bericht.md)
Gebaut:
- End-to-End-Verkabelung der Reise (S2.2, Nähte 0–5):
  `cockpit-core/src/journey.rs` — Naht 4 (take_artifact: Artefakt mit
  ZWEI Digests [content_class = kanonische Klasse des bestätigten
  Crystals, byte_digest = Datei-Bytes] + Gate-Fakten als
  Zertifikatskern; nur aus ARTEFAKT_VERFÜGBAR, nur mit aufgezeichneter
  Bestätigung); Naht 5 (reimport: parst das entnommene Artefakt zurück
  und vergleicht Klassen — SameClass | CertificateBroken{certified,
  reimported} | Unparseable; replay_matches: Klasse reproduzierbar,
  Byte-Digest bindet die Datei).
- Fehl-Reisen enden lesbar (S2.5): „Wunsch nicht erfassbar" → Motor-
  Schema rot ⇒ zurück zu WUNSCH_ERFASST mit Grund (G10-Maschine);
  „Lauf abgelehnt" → ABGELEHNT mit Grund (G10); „Formatverlust beim
  Export" → export_lossy_plaintext: OHNE Bestätigung kein Export
  (format_loss-Residuum als Fehler), MIT Bestätigung Export + sichtbares
  format_loss-Residuum — nie ein stiller Verlust.
- Operator-Doku `docs/operator/` aus den S12-Slots:
  - handbuch.md: alle 10 S12.1-Abschnitte (mentales Modell, Reise,
    Wünsche formulieren, Gates/Residuen lesen, Ermessens-Entscheidungen,
    Artefakte entnehmen [zwei Digests in Klartext, Zertifikatsbruch bei
    äußerer Bearbeitung], Replay, benannter Nicht-Abschluss, Bibliothek/
    Saat, Einstellungen [Deklaration ≠ Aktivierung, Kanzel-Degradation])
    + „Was diese Doku bewusst nicht tut" (Claim-Schranke, S12.3/S12.5).
  - lesarten.md: die vier Klartext-Lesarten aus S12-A1 (Phase&BlueCube,
    Skala&RedCube, Ratchet&Frontier, HBM-Kandidat) + Nexus-Ausblick
    ausdrücklich ohne Funktionsversprechen; unter der Claim-Schranke
    (S12-A2).
Ausgangs-Gate:
- **Produkt-Kerntest grün** (conformance/tests/product_journey.rs::
  produkt_kerntest_ueber_sechs_naehte): Drei-Risiken-Memo-Wunsch →
  Kanzel formt (Annahmen als modellgeformt sichtbar) → Motor validiert →
  Bestätigung (Naht 1, aufgezeichnet) → Lauf mit festem RD (Naht 2) →
  Prüfung: alle Gates grün + Residuenfeld EXPLIZIT „geschlossen (∅)"
  (Naht 3) → Artefakt-Export mit zwei Digests + Zertifikat (Naht 4) →
  Re-Import klassenidentisch + Replay-Klasse reproduzierbar (Naht 5) —
  vollständig über die Cockpit-Zustandsmaschine, ohne Konsole.
- Zertifikatsbruch nachweisbar: extern editiertes Artefakt ⇒
  CertificateBroken mit BEIDEN Klassen benannt = grün.
- Fehl-Reisen: format_loss sichtbar + bestätigungspflichtig = grün;
  ABGELEHNT/abgewiesen mit Grund = grün.
- Doku=Verhalten-Stichproben = grün: „geschlossen (∅)" (Doku-Text ⇔
  residue_view-Verhalten), „kein Trotzdem durchlassen" (Doku ⇔
  force_through immer Fehler), Kanzel-Markierung + Degradation (Doku ⇔
  INTERPRETATION_MARKER/DegradedKanzel), vier Lesarten + Nexus ohne
  Funktionsversprechen vorhanden.
- Wächter: GUARD_PHASES += "G11" · CI vollständig grün.
Residuen dieser Phase: keine neuen. Hinweis: der Kerntest fährt die
Reise über cockpit-core (die eine Quelle der App-Oberfläche); der
Klick-Pfad in der egui-Shell rendert exakt diese Zustandsmaschine
(Display-Lauf = G12-Paketierungsnachweis).
Abweichungen von der Spec: keine.
