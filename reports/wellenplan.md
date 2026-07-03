# Wellenplan — Domänen-Vollausbau (Track A, W1–W16)

Abgeleitet aus dem Katalog (S1_DOMAENENKATALOG K.4 / catalog_data.rs)
nach Masterplan §2.5: Ordnung nach (a) struktureller Nähe zur
Referenzdomäne **D01**, (b) Wiederverwendungsgrad des Familien-Kerns,
(c) professionsgebundene Familie strukturell gleichrangig, PL4 dort
ausdrücklich review-gebunden. 16 Familien, 213 Leaf-Domänen.
Ziel je Domäne: **PL3** (Adapter 11/11 · 1 Referenz- + ≥2 Negativ-Cubes
· Domänen-Kerntest ≃ · Doku-Zeile). PL4 nur mit Nutzungs-/Review-Evidenz.

## Reihenfolge und Begründung

| Welle | Fam | Präfix | Domänen | n | Nähe/Reuse-Begründung |
|---|---|---|---|---|---|
| **W1** | A | D | D01–D15 | 15 | **Referenzfamilie** (D01 = PL4-Anker). Unit = semantische Text-Einheit, Seam = Stütz-/Verweis-Beziehung → der Doc-Family-Core ist zugleich die Basis vieler Folgefamilien. |
| **W2** | J | KNOW | KNOW01–15 | 15 | Text-Einheiten mit Quell-/Zitat-/Beleg-Nähten — höchste Doc-Core-Wiederverwendung. |
| **W3** | G | GOV | GOV01–12 | 12 | Regel-/Klausel-Einheiten, Geltungs-/Verweis-Nähte — doc-nah (wie D02/D07). |
| **W4** | N | COM | COM01–12 | 12 | Nachricht-/Kontakt-Einheiten, Bezug-Nähte — doc-nah (wie D05). |
| **W5** | F | PM | PM01–15 | 15 | Schritt-/Vorgang-Einheiten, Reihenfolge-/Abhängigkeits-Nähte (wie D04). |
| **W6** | K | EDU | EDU01–12 | 12 | Lern-Einheiten, Sequenz-/Voraussetzungs-Nähte — Prozess-nah. |
| **W7** | I | BUS | BUS01–15 | 15 | Aussage-/Positions-Einheiten, Beleg-Nähte — doc-/prozess-nah. |
| **W8** | B | SWE | SWE01–15 | 15 | Code-/Spec-Einheiten, Aufruf-/Vertrags-Nähte — eigener Struktur-Core (AST-Rückgewinnung), aber scharf definiert. |
| **W9** | C | DATA | DATA01–12 | 12 | Tabellen-/Kennzahl-Einheiten, Ableit-/Referenz-Nähte. |
| **W10** | D | GRA | GRA01–12 | 12 | Knoten-/Kanten-Einheiten — nutzt das Lattice-/Graph-Substrat direkt. |
| **W11** | E | MATH | MATH01–15 | 15 | Aussage-/Ableit-Einheiten, Beweis-Nähte — Claim-Schranke besonders wachsam. |
| **W12** | L | CRE | CRE01–15 | 15 | Motiv-/Szenen-Einheiten, Kompositions-Nähte. |
| **W13** | O | FIN | FIN01–10 | 10 | Position-/Saldo-Einheiten, Bilanz-Nähte. |
| **W14** | H | OPS | OPS01–15 | 15 | Control-/Asset-Einheiten, Abhängigkeits-Nähte (Safety-by-abstraction wie HBM S13-A3). |
| **W15** | M | HW | HW01–15 | 15 | Bauteil-/Baugruppen-Einheiten, Verbindungs-Nähte. |
| **W16** | P | REG | REG01–08 | 8 | Professionsgebunden: strukturell wie andere gebaut, **PL4 ausdrücklich an ProfessionalReviewGate gebunden** (Agent hebt nur bis PL3). Kapstein-Welle, da die Review-Gate-Mechanik zuvor nirgends nötig ist. |

Summe: 213 (15+15+12+12+15+12+15+15+12+12+15+15+10+15+15+8).

## Interleaving (Masterplan §3)

Track A ist die Hauptlast. Zwischen den Wellen schiebe ich je eine
Track-Einheit aus C/D/E/F ein und vermerke die Wahl im Wellenbericht.
Vorläufige Zuordnung (anpassbar):

- nach W1 → **D/P5** HttpTransport + Wikimedia live (Snapshot-fixiert)
- nach W2 → **C/P4** echtes Lokalmodell hinter LocalModelProvider
- nach W3 → **F/P6(c)** Ed25519 + OS-Keyring (nur CLI-Blatt)
- nach W4 → **E/P8** SCALE-2 „Dokumentenmappe" (Red(2)-Kerntest)
- nach W5 → **F** zstd-Transportprofil; nach W6 → **F** .docx-Export;
  weitere nach Bedarf.

Track G (macOS/Windows-Pakete) bleibt gesperrt bis Build-Hosts
existieren. L9b und Klonungs-Aktivierung bleiben außerhalb.

## Bau-Muster je Welle (Masterplan §2)

1. **Familien-Kern** unter `cce-materialize` (geteilte Crystal-Grammatik,
   Gate-Bausteine, Residuen-Grundvokabular) — Bibliothekscode, nie Kopie.
2. **Domänen als Spezialisierung** über dem Kern; nur die Differenz ist
   Eigencode; `check_adapter_parity` 11/11 Pflicht.
3. **Zeugen je Domäne:** 1 Referenz-Cube + ≥2 Negativ-Cubes (erwartetes
   Residuum) + Domänen-Kerntest (≃, closure_roundtrip-Muster) — der eine
   Wächter nimmt sie beim Registry-Eintritt automatisch auf.
4. **Katalog-Hebung** auf PL2 (Adapter+Zeugen) bzw. PL3 (+Kerntest+Doku).
