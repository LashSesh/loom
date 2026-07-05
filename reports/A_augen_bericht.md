# Track A — Augen: der Auftraggeber sieht das System zum ersten Mal
(Dokument 23 A1–A3; Programm „Volle Kraft")

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

**`dist/cce-windows.zip`** (5 666 815 B, committet — direkt von GitHub
ladbar): `cockpit.exe` + `loom.exe` + `library/seed/` (alle 14
Saat-`.loom`) + `ERSTSTART.txt` (exakt drei Schritte, null Terminal;
Formulierung aus Handbuch §1/§2/§9 kondensiert, mit ausdrücklicher
Claim-Schranke).
SHA-256: `da9efb2be96f617c5e3713503e9468464856738ab437c3f63cdf499343a4d0f7`

**Claim-Schranke (steht wörtlich in der ERSTSTART.txt):** dies ist ein
Cross-Build; der Klickpfad auf echter Windows-Hardware ist damit NICHT
betriebsverifiziert (im Container existiert kein Windows/wine). Was
strukturell verifiziert ist: PE32+-Form, GUI-/CUI-Subsystem, statischer
Link-Erfolg beider Binaries samt beider Renderer-Backends. Der erste
Doppelklick eines Menschen auf Windows bleibt der ausstehende
Betriebsbeweis — sichtbar, kein Overclaim.

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
