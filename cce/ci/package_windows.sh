#!/usr/bin/env bash
# ci/package_windows.sh — Windows-Cross-Build-Paket (Dokument 23 A1).
# Cross-kompiliert cockpit-app + loom-CLI fuer x86_64-pc-windows-gnu
# (mingw-w64) aus dem Linux-Build-Host heraus; erzeugt dist/cce-windows.zip
# mit genau den in A1 benannten Inhalten: cockpit.exe + loom.exe +
# library/seed/*.loom + ERSTSTART.txt (drei Schritte, null Terminal).
#
# Voraussetzung (aus Standard-Paketquellen, kein Sonderzugriff):
#   rustup target add x86_64-pc-windows-gnu
#   apt-get install -y gcc-mingw-w64-x86-64 g++-mingw-w64-x86-64
set -euo pipefail
CCE_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$CCE_ROOT"

TARGET=x86_64-pc-windows-gnu
# dist/ liegt eine Ebene ueber dem cce-Workspace (Repo-Wurzel, neben
# cce-spec-repo/ und reports/) — dieselbe Stelle, an der das erste
# Windows-Paket committet wurde.
OUT="$CCE_ROOT/../dist/cce-windows"

echo "== Cross-Release-Build ($TARGET) =="
CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER=x86_64-w64-mingw32-gcc \
    cargo build --release --target "$TARGET" -p cockpit-app -p loom-cli

echo "== Paketbaum =="
rm -rf "$OUT"
mkdir -p "$OUT/library/seed"
cp "target/$TARGET/release/cce-cockpit.exe" "$OUT/cockpit.exe"
cp "target/$TARGET/release/loom.exe" "$OUT/loom.exe"
cp library/seed/*.loom "$OUT/library/seed/"
cat > "$OUT/ERSTSTART.txt" <<'STARTEOF'
CCE LOOM — ERSTSTART (Windows)
==============================

Drei Schritte, kein Terminal:

  1. Diese ZIP-Datei entpacken (Rechtsklick > "Alle extrahieren...").

  2. Doppelklick auf  cockpit.exe

  3. Die Saat-Reise fahren:
     Wunsch -> Lauf -> Pruef -> Artefakt.
     Sie beschreiben im Reiter "Wunsch", was Sie brauchen; die
     KI-Kanzel hilft, daraus einen pruefbaren Arbeitsauftrag zu
     formen (sie ist Fuehrer und Dolmetscher, niemals Richter).
     Nach Ihrer Bestaetigung faehrt der deterministische Motor den
     Lauf. Im Reiter "Pruef" sehen Sie schon im ersten Durchlauf
     ein Residuenfeld "geschlossen (0)", einen Gate-Report und den
     Abschlussbeweis; im Reiter "Artefakt" entnehmen Sie das
     Ergebnis. Jeder Lauf laesst sich identisch wiederholen
     (Replay) — die Wiederholung erzeugt dieselbe Klasse, nie eine
     neue KI-Antwort.

Was Sie sehen sollten: ein Fenster mit vier Reitern oben (Wunsch,
Lauf, Pruefung, Artefakt), lesbarer Beschriftung auf allen Knoepfen
und Feldern, und einer aktivierten Checkbox "Kanzel" mit Statustext
daneben. Alles ist von Anfang an lesbar — es gibt keinen Ladeschritt
und keinen Schwarzbild-Zustand.

Was sonst noch in diesem Paket liegt:

  loom.exe            Pruef-Werkzeug fuer .loom-Arbeitskoerper
                      (fuer Fortgeschrittene; der Erststart braucht
                      es nicht).
  library\seed\       Die Saat-Bibliothek: mitgelieferte, bereits
                      versiegelte .loom-Arbeitskoerper (u. a. das
                      Drei-Risiken-Memo als erstes gefuehrtes
                      Beispiel).

Ehrliche Reichweite (keine Uebertreibung):

  Ohne konfigurierten KI-Anbieter laeuft das Produkt vollstaendig —
  die Kanzel zeigt sich dann sichtbar degradiert (Fuehrung ohne
  Cloud-Intelligenz). Dieses Paket behauptet nichts jenseits der
  gepruefteren Reichweite des Korpus: die Dokument-Domaene D01
  (Drei-Risiken-Memo-Reise) ist Produkt-Kerntest-gruen (PL4); die
  uebrigen Katalog-Domaenen sind Registry-Staende ohne Vollausbau.
  Dieses Windows-Paket ist ein Cross-Build; sein Klickpfad wurde
  auf echter Windows-Hardware getestet und ein dabei gefundener
  Anzeigefehler (fehlende Schrift) behoben (Programm 23, Nachtrag).
STARTEOF

echo "== Archiv =="
DIST_DIR="$CCE_ROOT/../dist"
rm -f "$DIST_DIR/cce-windows.zip"
(cd "$DIST_DIR" && zip -r -9 cce-windows.zip cce-windows >/dev/null)
echo "PAKET: $DIST_DIR/cce-windows.zip"
sha256sum "$DIST_DIR/cce-windows.zip"
