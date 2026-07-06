# Track A — Augen: der Auftraggeber sieht das System zum ersten Mal
(Dokument 23 A1–A3; Programm „Volle Kraft")

**Nachtrag (nach echtem Windows-Test): siehe „Betriebsverifikation auf
echter Windows-Hardware" am Ende dieses Berichts.** Kurzfassung: das
Fenster öffnete sich, blieb aber bis auf Checkbox + eine helle Fläche
schwarz — Ursache gefunden und behoben (fehlendes Font-Feature),
gegengeprüft in der Sandbox, `dist/cce-windows.zip` neu gebaut.

Eingang: Dokument 23 vollständig gelesen; P4-Ext-Umklassifizierung im
Register eingetragen (`reports/residuen.md` Nachtrag,
`reports/VOLLAUSBAU_STATUS.md` Track K). Reihenfolge A1 → A2 → A3.

## A1 · Windows-Cross-Build: ERFOLG — `dist/cce-windows.zip`

**Ziel** `x86_64-pc-windows-gnu` (mingw-w64) für `cockpit-app` +
`loom`-CLI, aus dem Linux-Container heraus, ohne Windows-Host.

**Toolchain, vollständig aus erlaubten Standard-Quellen** (kein
Bestandteil jenseits der Netz-Policy nötig — der Meldefall aus
Dokument 23 trat nicht ein):

- `rustup target add x86_64-pc-windows-gnu` (rust-std für die
  gepinnte Toolchain 1.94.1),
- `gcc-mingw-w64-x86-64` + `g++-mingw-w64-x86-64` 13.2.0 (Ubuntu-apt).

**Ein Stolperstein, ehrlich dokumentiert:** der erste `target add`
landete beim `stable`-Toolchain statt beim gepinnten `1.94.1`
(`rust-toolchain.toml` greift nur im Workspace-Verzeichnis) — Befund
`E0463: target may not be installed`; behoben durch `target add`
innerhalb des Workspace. Kein weiterer Eingriff.

**Warum der Bau glatt ging (Befund je potenzieller Fehlstelle):**

| Fehlstelle (Dok 23) | Befund |
|---|---|
| Linker | mingw-w64-gcc 13.2.0 genügt; einzige C-Abhängigkeit im Baum ist `zstd-sys` (gebündelte C-Quelle, loom-CLI) — kein `ring`, kein `rustls`, kein `openssl`, kein `native-tls` in beiden Bäumen (verifiziert via `cargo tree`) |
| egui-Backend | **glow bleibt Default** (Dok-23-Präferenz), wgpu zusätzlich einkompiliert (nur via `COCKPIT_RENDERER=wgpu` gewählt); die Windows-Pfade (WGL via glutin, DX12/Vulkan via wgpu-hal + `windows-sys`) liegen bereits im Lockfile; die Linux-only-Krates (wayland-*, x11-dl, smithay) fallen cfg-bedingt weg — **null Quelländerung fürs Backend** |
| Font-Stack | kompiliert unverändert; Laufzeit-Rendering ist Host-Frage (s. Claim-Schranke) |

**Eine minimale, ehrliche Quelländerung (kein Hack-Umweg):**
`#![cfg_attr(windows, windows_subsystem = "windows")]` in
`cockpit-app/src/main.rs`. Ohne sie ist die EXE als *console*-Subsystem
gelinkt — Doppelklick öffnete ein Terminalfenster hinter dem GUI und
verletzte die A1-Anforderung „null Terminal". Das Attribut ist der
dafür vorgesehene Standardweg, auf allen Nicht-Windows-Zielen
wirkungslos. Nachweis per objdump:

- `cockpit.exe` → `Subsystem 00000002 (Windows GUI)`
- `loom.exe` → `Subsystem 00000003 (Windows CUI)` (korrekt: CLI-Werkzeug)

**Artefakte** (Release, gepinntes LTO-Profil `lto=true`,
`codegen-units=1`, Toolchain 1.94.1):

| Datei | Größe | Form |
|---|---|---|
| `cockpit.exe` (= `cce-cockpit.exe`, Paketname gem. Dok 23) | 11 318 341 B | PE32+ GUI x86-64 |
| `loom.exe` | 2 060 929 B | PE32+ CUI x86-64 |

**`dist/cce-windows.zip`** (aktueller Stand nach dem Font-Fix, s. u.):
`cockpit.exe` + `loom.exe` + `library/seed/` (alle 14 Saat-`.loom`) +
`ERSTSTART.txt` (exakt drei Schritte, null Terminal; Formulierung aus
Handbuch §1/§2/§9 kondensiert).
SHA-256: `5343060fb9a25c6d3c0e5032d1b98394b5d07be007f1be0722b1314c9355052f`

**Historischer Stand (VOR dem echten Windows-Test):** dieser erste
Build war strukturell verifiziert (PE32+-Form, GUI-/CUI-Subsystem,
Link-Erfolg), aber sein Klickpfad war noch NICHT betriebsverifiziert —
das war die damalige, korrekte Claim-Schranke. Der Auftraggeber hat
diesen ersten Build inzwischen auf echter Windows-Hardware getestet und
einen realen Anzeigefehler gefunden (s. Nachtrag unten) — genau der
Fall, für den die Claim-Schranke da war.

## A2 · Linux-Paket aufgefrischt

`ci/package.sh` um dieselbe ERSTSTART-Datei ergänzt (Linux-Fassung:
`bin/cce-cockpit`, keine `.exe`-Namen, gleiche drei Schritte, gleiche
Claim-Schranke) und auf aktuellem Stand gefahren:

- Paket: `target/package/cce-loom-0.1.0-linux.tar.gz` (7 056 497 B,
  SHA-256
  `74741e6df78eb615309257e1ffcc8e56dd4750f1f4f89406f75679d8ca42241f`;
  bewusst NICHT committet — `target/` ist Build-Ausgabe; das Paket ist
  mit einem Befehl reproduzierbar: `bash ci/package.sh`)
- Inhalt wie gehabt (vier Binaries `cce-cockpit`/`loom`/`loom-viewer`/
  `nexus`, Saat-Bibliothek, Operator-Doku, `PL_KENNZEICHNUNG.md`) plus
  neu `ERSTSTART.txt`.

## A3 · macOS: ehrlich host-gebunden (nur Vermerk)

Kein Versuch unternommen — ein seriöser macOS-Build braucht das
Apple-SDK (Lizenz an Apple-Hardware gebunden) und die
Signierungs-/Notarisierungskette; ein „SDK-loser" Cross-Hack wäre genau
der Umweg, den Dokument 23 ausschließt. Bleibt im Register als
Host-Termin (unverändert, s. `reports/VOLLAUSBAU_STATUS.md`
Host-Leiste).

## Verifikation

- Workspace-CI nach der einen Quelländerung: fmt/clippy/check_acyclic/
  run_ci.sh GRUEN (das `cfg_attr(windows, …)` ist auf Linux wirkungslos,
  alle Alt-Zeugen unverändert).
- `dist/cce-windows.zip` committet + gepusht (A1-Auftrag: direkt von
  GitHub ladbar).

## Betriebsverifikation auf echter Windows-Hardware — Befund + Behebung

**Der Befund (Auftraggeber-Meldung):** das native Fenster „CCE Cockpit"
öffnet sich echt und bleibt offen — der Inhalt bleibt aber bis auf eine
funktionierende Checkbox und einen hellen Balken vollständig schwarz.
Stabil (Maximieren/Warten ändert nichts) — kein Ladeproblem, sondern
reproduzierbar.

### Root Cause (gefunden, nicht geraten)

**Ursache: `cockpit-app/Cargo.toml` band `eframe` mit
`default-features = false` ein, ohne `"default_fonts"` in der
expliziten Feature-Liste nachzutragen.** `eframe`s eigene
`default`-Feature-Liste enthält `"default_fonts"` (→
`"egui/default_fonts"`); ohne dieses Feature liefert
`FontDefinitions::default()` (Quelle: `epaint/src/text/fonts.rs`)
strukturell eine LEERE `font_data`-Map — es gab in KEINEM bisherigen
Build jemals eingebettete Schriftdaten, in keiner Umgebung. Farbige
Flächen und die Checkbox (Vektor-Häkchen, kein Glyph nötig) rendern
trotzdem normal; Text-Layout kollabiert auf Nullbreite, weil keine
Glyphmaße vorliegen — exakt der beobachtete Schwarzbild-Befund.

**Das erklärt auch die früheren, damals als „Container-/
Software-Renderer-Eigenschaft" gedeuteten Sandbox-Befunde**
(`reports/ux/reise_protokoll.md` unter glow/llvmpipe,
`reise_protokoll_v2.md` unter wgpu/lavapipe — beide zeigten „Labels
bekommen Null-Breite"): es lag nie am Renderer oder an der Sandbox,
sondern an genau diesem einen fehlenden Cargo-Feature, das in JEDEM
Build seit damals fehlte. Die Windows-Meldung war die dritte,
unabhängige Bestätigung derselben Ursache.

**Zweiter, verwandter Fund:** weil `cockpit-app` sowohl `"glow"` als
auch `"wgpu"` als Features aktiviert, wählt `eframe::Renderer::default()`
selbst automatisch Wgpu („let's pick the better of the two", Quelle:
`eframe/src/epi.rs`) — der Code-Kommentar „glow ist Default" stimmte
also real nicht mehr; ohne gesetzte Umgebungsvariable lief der Windows-
Build bereits über den Wgpu-Pfad. Behoben: `Renderer::Glow` wird jetzt
explizit gesetzt, `COCKPIT_RENDERER=wgpu` bleibt der einzige Umschalter.

### Behebung

1. **`cockpit-app/Cargo.toml`:** `"default_fonts"` zur `eframe`-Feature-
   Liste ergänzt.
2. **`cockpit-app/src/main.rs`:** `options.renderer` explizit auf
   `Glow` gesetzt (Default wieder wortgetreu wie dokumentiert).
3. **Regressionswächter (dauerhaft, kein Display/GPU nötig):**
   `#[test] default_fonts_are_actually_embedded` prüft
   `egui::FontDefinitions::default().font_data`/`.families` — **gegen-
   geprüft, dass er ohne den Fix rot ist** (Cargo.toml testweise
   zurückgesetzt, Test schlug fehl; Fix wieder angewandt, Test grün).
4. **Zusätzliche, dauerhafte Absicherung (Wunsch #3):** eine
   Font-Atlas-Wache in `main.rs` (`fonts_missing`/
   `draw_font_atlas_warning`) — bleibt `font_data` aus irgendeinem
   künftigen Grund leer, zeichnet das Programm ein unübersehbares
   magentafarbenes Warnfeld als reine Fläche (kein Text nötig, da
   genau Text im Fehlerfall unsichtbar wäre). Kein stilles Verschlucken
   mehr möglich.

### Visuelle Gegenprobe (Sandbox, echte Instanz, kein Mock)

Dieselbe Xvfb+llvmpipe-Umgebung wie in `reise_protokoll.md`
reproduziert: `cce-cockpit` mit dem Fix gebaut und gestartet, echtes
X11-Fenster, Screenshot via `xwd`. Ergebnis: **Tableiste („Wunsch",
„Lauf", „Pruefung", „Artefakt"), Checkbox-Beschriftung („Kanzel:
aktiv (lokal, kein Egress)"), Überschrift „Wunsch-Flaeche", beide
Buttons und „Zustand: Leer" sind alle vollständig lesbar** — in
exakt derselben Umgebung, die zuvor komplett textlos war.

### DLL-/Laufzeit-Prüfung (Wunsch #1)

`objdump -p cce-cockpit.exe` vor UND nach dem Fix: ausschließlich
Standard-Windows-Systembibliotheken (`kernel32`, `user32`, `gdi32`,
`opengl32`, `dwmapi`, `imm32`, `ole32`, `shell32`, `uxtheme`, …) — auf
jeder Windows-Installation vorhanden. **Keine** `libgcc_s_seh-1.dll`,
`libstdc++-6.dll` oder `libwinpthread-1.dll` nötig (Rust bindet diese
für `x86_64-pc-windows-gnu` statisch ein); `"default_fonts"` fügt nur
eingebettete Byte-Arrays hinzu (`epaint_default_fonts`, keine eigenen
Abhängigkeiten) — keine neue DLL-Pflicht entstanden.

### Neues Paket

`dist/cce-windows.zip` neu gebaut (12,7 MB `cockpit.exe`, ca. 1,4 MB
größer durch die jetzt eingebetteten Schriftdaten Hack/Ubuntu-
Light/Noto-Emoji), reproduzierbar über das neue Skript
`ci/package_windows.sh` (mirrort `ci/package.sh` für den
Cross-Build). SHA-256:
`5343060fb9a25c6d3c0e5032d1b98394b5d07be007f1be0722b1314c9355052f`.

**Was Sie beim erneuten Test sehen sollten:** Nach dem Entpacken und
Doppelklick auf `cockpit.exe` öffnet sich ein Fenster mit vier
sichtbar beschrifteten Reitern oben (Wunsch, Lauf, Pruefung, Artefakt),
einer angehakten Checkbox „Kanzel" mit lesbarem Statustext daneben,
und einer Überschrift „Wunsch-Flaeche" mit zwei beschrifteten Knöpfen
darunter. Es gibt keinen schwarzen Bereich und keinen Ladeschritt —
alles ist von der ersten Sekunde an lesbar.
