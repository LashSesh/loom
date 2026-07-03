# Vollausbau-Status (eine Seite, Klartext)

Stand: nach Welle **W1**. CI: GRUEN. `feature_maturity_overclaim`: leer.

## Domänen (Track A) — 213 gesamt

| Familie | Präfix | fertig/gesamt | PL-Verteilung |
|---|---|---|---|
| A Dokument/Text | D | **15/15** | D01 = PL4 · D02–D15 = PL3 |
| J Wissen/Forschung | KNOW | **15/15** | KNOW01–15 = PL3 |
| G Governance | GOV | **12/12** | GOV01–12 = PL3 |
| N Kommunikation | COM | 0/12 | PL1 |
| F Projekt/Prozess | PM | 0/15 | PL1 |
| K Bildung | EDU | 0/12 | PL1 |
| I Produkt/Business | BUS | 0/15 | PL1 |
| B Software Eng. | SWE | 0/15 | PL1 |
| C Daten/Analytics | DATA | 0/12 | PL1 |
| D Graph/Netzwerk | GRA | 0/12 | PL1 |
| E Mathematik | MATH | 0/15 | PL1 |
| L Kreativ/Medien | CRE | 0/15 | PL1 |
| O Finanzen | FIN | 0/10 | PL1 |
| H Security/Ops | OPS | 0/15 | PL1 |
| M Hardware/CAD | HW | 0/15 | PL1 |
| P Regulated Advisory | REG | 0/8 | PL1 (PL4 review-gebunden) |

**Summe fertig (≥PL3): 42/213.** Nächste Welle: **W4 = Familie N
(Kommunikation/CRM, COM01–12).**

## Parallele Tracks

| Track | Stand |
|---|---|
| A Domänen | W1–W3 fertig (Familien A, J, G, PL3); W4 als Nächstes |
| B Erlebbarkeit | wgpu getestet (Software-GL blockt Text → Host nötig); #26 Datei-Export **fertig**; GUI-Feindesign offen |
| C Intelligenz | P4 **fertig**: echtes lokales Extraktiv-Modell (recorded, kein Egress), Zeuge grün; GGUF/LLM offen |
| D Weltzugang | P5 **fertig**: HttpTransport (feature `http`) + Wikimedia live, Fixture-fixiert, Zeuge grün; JSON-CSU-Extraktor offen |
| E Skalen | P8 (SCALE-2) — geplant nach W4 |
| F Härtung | P6(a/b/c) **fertig**: Ed25519-Signatur ueber core_root (loom-cli sign/verify-sig), OS-Keyring feature-gated; Registry-Vollform offen |
| G Pakete | gesperrt (Build-Hosts fehlen) |

## Nächster Schritt

Welle **W4 = Familie N (Kommunikation/CRM, COM01–12)**; danach
Interleaving-Einheit **E/P8** (SCALE-2 Dokumentenmappe).
