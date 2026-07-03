# Vollausbau-Status (eine Seite, Klartext)

Stand: Track A **W16 VOLLSTAENDIG (213/213 PL3)** + **Block 1
(Belegpflicht UX) UND Block 2 (Kanzel↔Modell + GUI-Feindesign)
abgeschlossen**. CI: GRUEN. `feature_maturity_overclaim`: leer.

## Domänen (Track A) — 213 gesamt

| Familie | Präfix | fertig/gesamt | PL-Verteilung |
|---|---|---|---|
| A Dokument/Text | D | **15/15** | D01 = PL4 · D02–D15 = PL3 |
| J Wissen/Forschung | KNOW | **15/15** | KNOW01–15 = PL3 |
| G Governance | GOV | **12/12** | GOV01–12 = PL3 |
| N Kommunikation | COM | **12/12** | COM01–12 = PL3 |
| F Projekt/Prozess | PM | **15/15** | PM01–15 = PL3 |
| K Bildung | EDU | **12/12** | EDU01–12 = PL3 |
| I Produkt/Business | BUS | **15/15** | BUS01–15 = PL3 |
| B Software Eng. | SWE | **15/15** | SWE01–15 = PL3 |
| C Daten/Analytics | DATA | **12/12** | DATA01–12 = PL3 |
| D Graph/Netzwerk | GRA | **12/12** | GRA01–12 = PL3 |
| E Mathematik | MATH | **15/15** | MATH01–15 = PL3 |
| L Kreativ/Medien | CRE | **15/15** | CRE01–15 = PL3 |
| O Finanzen | FIN | **10/10** | FIN01–10 = PL3 |
| H Security/Ops | OPS | **15/15** | OPS01–15 = PL3 |
| M Hardware/CAD | HW | **15/15** | HW01–15 = PL3 |
| P Regulated Advisory | REG | **8/8** | REG01–08 = PL3 (PL4 review-gebunden) |

**Summe fertig (≥PL3): 213/213 — Track A VOLLSTAENDIG.** D01=PL4, 212×PL3.

## Parallele Tracks

| Track | Stand |
|---|---|
| A Domänen | **W1–W16 fertig: 213/213 auf PL3** (alle 16 Familien) |
| B Erlebbarkeit | **Block 1 nachgeliefert:** echter wgpu-Klick-Durchlauf (6 Nähte, debug-instrumentiert, `reports/ux/reise_protokoll_v2.md`) + #26 Datei-Export jetzt ECHT über den GUI-Button verifiziert (byte-identisch zum Headless-Pfad); Font-Atlas-Befund zweifach bestätigt host-gebunden (glow+wgpu), Backend zurückgestellt. **GUI-Feindesign (LC-R5) fertig:** alle 5 Pflichtansichten (Manifest/Segmentliste+Digest/Residuen+Verdikt/Gate-Reports/Ledger) im bestehenden Pruef-Tab verdrahtet, Zeuge grün |
| C Intelligenz | P4 **fertig**: echtes lokales Extraktiv-Modell (recorded, kein Egress), Zeuge grün. **Kanzel-Verdrahtung fertig:** LocalKanzel ruft real durch das unveränderte InferenceGateway (LocalExtractiveModel), Annahmen-Text modellgeformt + Provider-/Evidence-Beleg, DegradedKanzel unberührt, COCK-INV-1..8 unverändert grün; GGUF/LLM offen (host-gebunden) |
| D Weltzugang | P5 **fertig**: HttpTransport (feature `http`) + Wikimedia live, Fixture-fixiert, Zeuge grün; JSON→CSU-Extraktor offen (→ Block 3) |
| E Skalen | P8 **fertig**: SCALE-2 Dokumentenmappe (Red(2)-Kerntest, MSC 1→2, Adapter 8/8, PL2); SCALE-3 offen |
| F Härtung | P6(a/b/c) **fertig**: Ed25519-Signatur ueber core_root (loom-cli sign/verify-sig), OS-Keyring feature-gated; Registry-Vollform offen |
| G Pakete | gesperrt (Build-Hosts fehlen) |

## Block-Auftrag (aktuell)

1. **Block 1 — Belegpflicht UX: ABGESCHLOSSEN.** Echter Klick-Durchlauf
   unter wgpu (matchbox-WM fuer Fokus, COCKPIT_DEBUG_RECTS=1 fuer
   verifizierbare Koordinaten/Zustaende, da Glyphen host-bedingt nicht
   rendern), reise_protokoll_v2.md, echte per Button exportierte
   `.md`+`.cert`. Zwei ehrliche Nebenbefunde offen gemeldet (nicht
   verborgen): Glyphen-Rendering host-gebunden (R-Agent-6), xdotool-
   Zeichenumkehr-Artefakt der Testkette (R-Agent-7) — siehe
   reports/residuen.md.
2. **Block 2 — Kanzel↔LocalExtractiveModel + GUI-Feindesign: ABGESCHLOSSEN.**
   Kanzel-Formung laeuft real durch `run_inference` (LocalExtractiveModel,
   egress_lock=None da kein Egress noetig); Annahmen-Interpretation traegt
   Provider-Herkunft (`local-extractive:kernmodell`) + Gateway-Evidence-ID
   als Beleg, Crystal-Referenzstruktur unveraendert. GUI-Feindesign: die
   fuenf LC-R5-Pflichtansichten (Manifest/Segmentliste+Digest-Status/
   Residuen+Verdikt/Gate-Reports/Ledger-PhaseBlocks) rendern jetzt im
   Pruef-Tab aus echten Motor-/Ledger-Fakten. Zeugen: `kanzel_model_witness.rs`
   (3 Tests) + `lc_r5_inspection_views.rs` (3 Tests), alle 13 COCK-INV-Tests
   unveraendert gruen (keine Testdatei-Aenderung noetig), voller
   Workspace-Testlauf gruen, clippy sauber.
3. **Block 3 — JSON→CSU-Extraktor Wikimedia + erstes Welt-Crystal: NÄCHSTER SCHRITT.**

Track A bleibt vollständig (213/213 PL3). Restliche Track-Reste (alle
sichtbar im Register): zstd-Transportprofil, .docx-Export,
blake3-Zweitprofil, SCALE-3-Entwurf, PL3→PL4-Reifepfade. Track G
(macOS/Windows-Pakete) bleibt gesperrt bis Build-Hosts existieren.