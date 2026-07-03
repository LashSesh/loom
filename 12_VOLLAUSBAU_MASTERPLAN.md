# 12 — VOLLAUSBAU-MASTERPLAN (Domänen 213/213 + alle offenen Spektren, ab sofort)

**Auftraggeber-Direktive:** Der Ausbau des vollen Spektrums — insbesondere aller 213 Katalog-Domänen — beginnt **unmittelbar**, nicht nachgelagert. Dieses Dokument ist der persistente Plan dafür: abbruchfest über viele Agent-Sitzungen (Kontingent-Disziplin gilt: kleiner Commit je Einheit, Bericht je Welle), qualitätsgesichert über den EINEN Wächter, ehrlich in der Reifegrad-Sprache (PL). Es ergänzt 09_POST_IMPLEMENTATION_TOTAL_ROADMAP und ersetzt deren serielle Lesart durch **parallele Tracks**.

## §1 Zielbild und die eine ehrliche Grenze

**Ziel:** Alle 213 Domänen auf **PL3** („strukturell vollausgebaut und maschinell bewiesen": Adapter 11/11, Zeugen, Kerntest, Doku-Zeile) — durch Agentenarbeit vollständig erreichbar. **PL4** („nutzungsreif bewiesen") verlangt per Verfassung echte Nutzungs-Evidenz; bei professionsgebundenen Domänen zusätzlich das ProfessionalReviewGate (menschliche Fachprüfung). Das ist keine Verzögerung, sondern die Anti-Overclaim-Regel des Systems selbst: Agenten können Struktur beweisen, nicht Berufszulassung ersetzen. Jede Domäne trägt ihr PL sichtbar; `feature_maturity_overclaim` bleibt leer.

## §2 Bauprinzip: Familien-Wellen statt 212 Einzelbauten

Die Registry (catalog_data.rs) trägt bereits je Domäne den Kernvertrag (Zweck/Crystal/Artefakt/Kern-Gate/Kern-Residuum). Der Ausbau vertieft diese Zeile zum vollen Adapter — **familienweise**:

1. **Familien-Kern zuerst:** Je Familie (16, gemäß S1_DOMAENENKATALOG K.4) wird ein gemeinsamer Familien-Adapter-Kern gebaut (geteilte Crystal-Grammatik, geteilte Gate-Bausteine, geteiltes Residuen-Grundvokabular) — als Bibliothekscode unter cce-materialize, NIE als Kopie.
2. **Domänen als Spezialisierung:** Jede Leaf-Domäne implementiert den DomainAdapter-Vertrag (11 Punkte, `check_adapter_parity` 11/11 Pflicht) über dem Familien-Kern; nur ihre Differenz ist Eigencode.
3. **Zeugenpflicht je Domäne:** 1 Referenz-Cube (realistisch, geschlossen) + ≥2 Negativ-Cubes mit ERWARTETEM Residuum + Domänen-Kerntest (geschlossener Pfad ≃, nach closure_roundtrip-Muster). Der Wächter nimmt sie beim Registry-Eintritt automatisch und dauerhaft auf.
4. **Katalog-Hebung:** Registry-Eintrag der Domäne auf PL2 (Adapter+Zeugen) bzw. PL3 (+Kerntest+Doku-Zeile in docs/operator/domaenen/) mit Evidence-Pfad; PL4 nur mit Nutzungs-/Review-Evidenz.
5. **Wellen-Reihenfolge:** vom Agenten aus dem Katalog abgeleitet nach (a) struktureller Nähe zur Referenzdomäne D01, (b) Wiederverwendungsgrad des Familien-Kerns, (c) professionsgebundene Familie strukturell gleichrangig, PL4 dort ausdrücklich review-gebunden. Die gewählte Reihenfolge wird VOR Welle 1 als `reports/wellenplan.md` festgeschrieben (Auftraggeber kann umpriorisieren).

**Exit-Gate je Welle W<nn>:** alle Domänen der Familie parity 11/11 · alle Zeugen grün/rot korrekt im Wächter · alle Kerntests grün · CI GRUEN · `reports/W<nn>_bericht.md` (Domänenliste, PL-Stand, Besonderheiten, Residuen) · Register-/Katalog-Update committet. Kein Wellen-Übergang bei rotem Gate.

## §3 Parallele Tracks (alle starten, soweit nicht host-/freigabe-blockiert)

| Track | Inhalt | Startbedingung |
|---|---|---|
| **A · Domänen-Vollausbau** | §2, W1–W16 bis 213/213 PL3 | sofort |
| **B · Erlebbarkeit** | WO-2-Abschluss (wgpu-Test), #26 Datei-Export, danach GUI-Feindesign (LC-R5), Erststart-Führung | läuft; Rest sofort |
| **C · Intelligenz** | P4: echtes Lokalmodell hinter LocalModelProvider (recorded); Kanzel-Prompt-Bibliothek (IG-R4) | freigegeben (unten) |
| **D · Weltzugang** | P5: HttpTransport + Wikimedia live (Snapshot-fixiert); danach weitere official_api-Quellen; OAuth-Connector als Folgeschritt | freigegeben (unten) |
| **E · Skalen** | P8: SCALE-2 „Dokumentenmappe" (Red(2)-Kerntest, MSC 1→2); danach SCALE-3-Entwurf | sofort |
| **F · Härtung/Ökosystem** | P6(c) Ed25519+OS-Keyring (nur CLI-Blatt), zstd-Transportprofil, blake3-Profil, .docx-Export (S1.10-R1), MIME-Doku | freigegeben (unten) |
| **G · Pakete** | P3 macOS/Windows | wartet auf Build-Hosts (einziger externer Blocker) |

**Interleaving-Regel:** Track A ist die Hauptlast; der Agent darf B–F nach eigenem Ermessen zwischen Wellen einschieben (z. B. 1 Track-Einheit je abgeschlossener Welle), meldet die Wahl im Wellenbericht. L9b (Bridge) und Klonungs-Aktivierung bleiben AUSSERHALB dieses Plans (eigener Zyklus bzw. bewusste Betriebsentscheidung).

## §4 Unverändert unantastbar

spec/ read-only · Score nie Gate · fail-closed · Egress nur durch die drei Tore (neue Quellen NUR als CSA-Adapter mit Manifest+Gates; neue Modelle NUR als Provider hinter dem Gateway) · jede Unfertigkeit sichtbar · Wächter wächst, wird nie beschnitten · bei Unklarheit Residuum statt Raten · Berichtsformate wie eingeführt.

## §5 Fortschrittsanzeige für den Auftraggeber (code-frei lesbar)

Je Welle pflegt der Agent `reports/VOLLAUSBAU_STATUS.md`: eine Tabelle „Familie · Domänen fertig/gesamt · PL-Verteilung · Tracks-Stand · nächste Welle" — maximal eine Seite, Klartext. Das ist die eine Datei, an der der Auftraggeber jederzeit abliest, wo das Projekt steht.
