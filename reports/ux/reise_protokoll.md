# Reise-Protokoll (T1/WO-2) — Cockpit-App, echte Instanz

## Umgebung (ehrlich deklariert)

- **Kein echtes Desktop-Display.** `DISPLAY` war leer.
- **Virtueller Framebuffer eingerichtet:** `Xvfb :99 -screen 0 1280x900x24`
  (echter X-Server, keine Mock-/Simulationsschicht).
- **Software-GL:** Mesa **llvmpipe** (OpenGL 4.5 Compat, LLVM 20.1.2);
  softpipe getestet → nur Schwarzbild, verworfen.
- **Reale App-Instanz:** `cargo run -p cockpit-app` gebaut, dann
  `./target/debug/cce-cockpit` unter `DISPLAY=:99` gestartet — echtes
  X-Fenster „CCE Cockpit" (Window-ID 0x200003), Geometrie 800×600,
  reagiert auf echte `xdotool`-Klicks.

## Was WIRKLICH rendert — und was nicht

**Rendert real** (Screenshots beiliegend): Fenster, Tab-Leiste,
Kanzel-Checkbox (Haken sichtbar), Buttons als Flächen, aktiver Tab
(blau), Layout-Trennlinien. **Der Klick reagiert nachweislich:** die
beiden Screenshots `instanz_klick_kanzel_aus.png` /
`instanz_klick_kanzel_an.png` zeigen denselben Klick auf die
Kanzel-Checkbox mit umgeschaltetem Haken — eine echte, interaktive
Instanz.

**Rendert NICHT:** die **Text-Glyphen**. Unter der Software-GL dieses
Containers lädt der egui-Font-Atlas nicht; Labels bekommen Null-Breite,
wodurch die Tab-/Button-Beschriftungen unsichtbar sind UND die
Bedienelemente links zusammenfallen (im Screenshot: alle vier Tabs
kollabieren auf ~8 px am linken Rand).

## Folge für den Klick-Durchlauf (Blockade, nicht improvisiert)

Weil die Beschriftungen unsichtbar sind und die Kontrollflächen
überlappen, lassen sich die einzelnen Schaltflächen der Sequenz
(Wunsch erfassen → Kanzel formt → BESTÄTIGEN → Tab Lauf → Lauf starten
→ Tab Prüfung → Tab Artefakt → Export) **per Sicht nicht zuverlässig
und verifizierbar treffen**. Ein blindes Klicken nach Koordinaten wäre
geraten, nicht bewiesen — das wäre Improvisation. Gemäß Auftrag
(„klar als Blockade melden statt zu improvisieren") wird der
vollständige, verifizierbare Sechs-Nähte-Klick-Durchlauf **in dieser
Umgebung als blockiert** gemeldet. Blockade-Ursache ist die
Container-Software-GL (Font-Atlas), nicht die App: dieselbe
Zustandsmaschine ist durch `conformance/tests/product_journey.rs` real
und grün bewiesen.

## Die sechs Nähte — Substanz aus derselben Zustandsmaschine

Die GUI ist eine dünne Schale um `cockpit_core` (EnginePort +
CockpitState). Was jeder Klick auslösen WÜRDE, ist deterministisch und
im grünen Test `produkt_kerntest_ueber_sechs_naehte` bewiesen:

| Naht | Klick in der GUI | Motor-Fakt dahinter (bewiesen) |
|---|---|---|
| **0 Öffnen** | App startet, Tab „Wunsch" | `CockpitState::Leer`; Zustandsmaschine deterministisch (S3.3) |
| **1 Wünschen → Bestätigen** | Wunsch tippen · „Kanzel formt" · „BESTÄTIGEN" | Kanzel-Interpretation (Marker „Interpretation, kein Motor-Urteil"), Motor-Schema-Validierung grün, `confirm_crystal` mit AUFGEZEICHNETER Confirmation ⇒ Bestätigungsgrenze, `Bestaetigt` |
| **2 Laufen** | Tab „Lauf" · „Lauf starten" | fester RunDescriptor, `start_run` mit Confirmation ⇒ `Laeuft` → Motor fährt encode→…→equivalent |
| **3 Prüfen** | Tab „Prüfung" | alle Gate-Reports grün; Residuenfeld **„geschlossen (∅)"** (residue_view leer ⇒ genau dieser String); kein „Trotzdem durchlassen" |
| **4 Entnehmen** | Tab „Artefakt" · „Exportieren" | `ArtefaktVerfuegbar`; Zwei-Digest-Zertifikat: content_class ≠ byte_digest |
| **5 Ablegen/Wiederholen** | (Re-Import-Kontrolle) | Re-Import **klassenidentisch** (SameClass); Fremdedition ⇒ CertificateBroken; Replay reproduziert die Klasse |

## Das Artefakt (Memo) selbst

**Beigelegt:** `artefakt_memo_workbody.loom` — der zertifizierte
Drei-Risiken-Memo-Arbeitskörper (= R7-Saat, `verdikt: Valid`,
core_root `12203429fc78…`, 0 Residuen; Inspect-Beleg
`artefakt_memo_inspect.txt`).

**Inhalt** (deterministisch aus dem committeten
`document/assets.rs` + `render.rs`, byte-genau das, was `materialize`
erzeugt — hier lesbar transkribiert, nicht simuliert):

```markdown
# Drei-Risiken-Memo

## Risiken
### Risiko: Serverausfall gefaehrdet den Go-Live-Termin
- Gegenmassnahme: Failover-Cluster mit automatischem Umschalten vorbereiten
### Risiko: Datenverlust bei Migration der Altbestaende
- Gegenmassnahme: Vollstaendige Sicherung und Probelauf der Migration
### Risiko: Lieferverzug der externen Komponenten
- Gegenmassnahme: Zweitlieferant qualifizieren und Puffer einplanen
```

(Die byte-genaue Form mit den unsichtbaren `<!--cce:unit…-->`-Struktur-
Ankern liegt im `.loom`; drei Risiken, je eine Gegenmaßnahme-Naht
`supports`, keine Bewertungszahlen — deckt V1.)

## Offene Punkte (Residua, für deine Entscheidung)

- **UX-GL:** Der egui-Font-Atlas rendert unter der Container-Software-GL
  nicht. Für einen sichtbaren Klick-Durchlauf braucht es entweder ein
  echtes Desktop-Display / einen GPU-fähigen Host **oder** die Freigabe,
  den Renderer-Backend-Feature-Schalter (glow→wgpu) zu testen — Letzteres
  ist eine Cargo.toml-Änderung und daher hier bewusst NICHT vorgenommen.
- **Memo als `.md` im Browser:** Die GUI hat KEINEN Datei-Export-Pfad
  (der Export-Button verankert nur im Ledger, schreibt keine Datei) —
  ein reales Finding. Ein GitHub-anzeigbares `.md` bräuchte eine winzige,
  rein lesende Render-Hilfe; das ist eine Code-Änderung außerhalb
  „trivialer Startfehler" und wartet auf deine Freigabe. Bis dahin liegt
  das Memo als zertifiziertes `.loom` + transkribierter Klartext (oben)
  bei.
