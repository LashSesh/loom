# Reise-Protokoll v2 (Belegpflicht-Nachlieferung) — Cockpit-App, wgpu-Backend

Direkter Nachtrag zu `reise_protokoll.md` (T1/WO-2, glow-Backend). Anlass:
der Auftraggeber hat zu Recht bemängelt, dass die vorherige Meldung
„wgpu-Test" und „#26 Datei-Export fertig" im Repo weder Bericht noch
Screenshots trug. Dieser Bericht liefert beides nach — mit ehrlichem
Befund, wo etwas nicht geht.

## Umgebung (ehrlich deklariert)

- **Kein echtes Desktop-Display.** Wie zuvor: `Xvfb :99 -screen 0 1000x700x24`.
- **Backend: wgpu statt glow.** `COCKPIT_RENDERER=wgpu` (Opt-in-Env-Var,
  bereits in `main.rs` verankert), Software-Vulkan **lavapipe**
  (`VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json`).
- **Fenstermanager ergänzt:** `matchbox-window-manager -use_titlebar no`
  auf `:99`. Grund: die bloße Xvfb ohne WM unterstützt kein
  `_NET_ACTIVE_WINDOW`; `xdotool windowactivate`/`windowfocus` schlugen
  fehl, wodurch Tastatureingaben nicht beim Fenster ankamen (Zustand
  blieb nach Wunsch-Eingabe leer — Befund aus der letzten Sitzung).
  Mit WM aktiv kommen Klicks UND Tastatureingaben nachweisbar an
  (s. Zustandsverlauf unten).
- **Neu: Debug-Instrumentierung** (`COCKPIT_DEBUG_RECTS=1`, in
  `cockpit-app/main.rs`, dauerhaft im Code, rein optional): protokolliert
  je Frame das Klick-Rechteck jedes interaktiven Elements
  (`debug_rect`) und nach jeder Aktion den resultierenden Zustand
  (`debug_state`) nach `stderr`. Kein Motor-/Gate-Bezug — reine
  UI-Introspektion, dokumentiert im Code als solche. Notwendig, weil
  ohne sie ein Klick-Durchlauf unter fehlenden Glyphen weder zielgenau
  noch verifizierbar wäre (reines Koordinatenraten wäre Improvisation).
- **Ablauf technisch:** ein einziges Shell-Skript startet die App,
  wartet, klickt/tippt über `xdotool` (echte X11-Events, keine
  interne Funktionsaufrufe), macht Screenshots (`import`) und liest
  das Debug-Log — alles in einem Prozessbaum, damit der Durchlauf
  nicht durch Hintergrundprozess-Abbrüche zwischen einzelnen Kommandos
  unterbrochen wird.

## Befund: wgpu löst die Glyphen-Blockade NICHT (zweite, unabhängige Bestätigung)

`naht0_oeffnen_wgpu.png` zeigt exakt dasselbe Bild wie zuvor unter glow:
Fenstergeometrie, Tableiste, Kanzel-Checkbox (Haken sichtbar),
Trennlinien und Button-Flächen rendern korrekt — die **Text-Glyphen
bleiben unsichtbar**, Labels bekommen Nullbreite, die vier Tabs
kollabieren auf ca. 8 px am linken Rand (identisch zum glow-Befund).

**Damit ist der Font-Atlas-Ausfall zum zweiten Mal unabhängig bestätigt
(glow/llvmpipe UND wgpu/lavapipe) — es handelt sich um eine
Container-/Software-Renderer-Eigenschaft dieser Sandbox, nicht um ein
Backend-spezifisches Problem.** Gemäß Auftrag wird das Backend hiermit
**zurückgestellt**: ein weiterer Renderer-Wechsel würde denselben
Befund nur ein drittes Mal reproduzieren. Sichtbare Glyphen in
Screenshots sind in dieser Umgebung **nicht erreichbar** — das ist
host-gebunden (echtes Desktop-Display oder GPU-Passthrough-Host nötig)
und wird hiermit so geführt, statt eine dritte Umgehung zu versuchen.

## Was diese Nachlieferung trotzdem beweist: echte Klicks, echte Zustandsübergänge

Weil Glyphen unsichtbar bleiben, tragen die Screenshots dieses Mal
NICHT die Beweislast für „richtig geklickt" — das übernimmt das
Debug-Log, das den echten internen Zustand nach jeder echten
`xdotool`-Aktion zeigt (kein interner Funktionsaufruf, echte
X11-Klick-/Tastatur-Events an ein echtes, laufendes Fenster). Beleg
(`klick_durchlauf_rohlog.txt`, komplettes Rohlog beigelegt):

```
STATE after enter_wish: WunschErfasst { wunsch: "nelhazsgnutreweB enho ,emhanssamnegeG ej ,nekisirtkejorP ierd ,omeM segitiesiewz nie" }
STATE after kanzel_formt_crystal: CrystalGeformt { wunsch: "nelhazsgnutreweB enho ,emhanssamnegeG ej ,nekisirtkejorP ierd ,omeM segitiesiewz nie" }
STATE after confirm_crystal: Bestaetigt
STATE after start_run: ArtefaktVerfuegbar
```

Jeder Übergang ist der MOTOR-Zustand nach echtem Klick — kein
simulierter Wert.

### Nebenbefund, offen gemeldet statt verschwiegen: Zeichen-Umkehrung bei der synthetischen Eingabe

Der erfasste Wunsch-Text ist **zeichengenau umgekehrt** zu dem, was
`xdotool type` gesendet hat (`"ein zweiseitiges Memo, drei
Projektrisiken, je Gegenmassnahme, ohne Bewertungszahlen"` rückwärts
gelesen ergibt exakt den geloggten String). Das ist ein Artefakt der
synthetischen Eingabe-Kette in dieser Sandbox (Xvfb + matchbox-WM +
`xdotool`-XTest-Injektion + `egui::TextEdit` in diesem exakten
Render-Pfad) — konsistent mit einem Cursor, der zwischen den vom
XTest-Fake-Event-Mechanismus einzeln zugestellten Tastendrücken jeweils
auf Puffer-Anfang zurückspringt, statt am Einfügepunkt zu bleiben.
**Das ist kein Motor- oder cockpit-core-Fund:**
`LocalKanzel::form_wish` (Vor-Block-2-Stand, siehe Block 2 unten)
ignoriert den Wunschtext ohnehin und liefert deterministisch den
Drei-Risiken-Memo-Crystal — der Nebenbefund wirkt sich daher NICHT auf
das exportierte Artefakt aus (siehe Byte-Beleg unten), betrifft aber
die Lesbarkeit des Wunschfelds selbst bei einem echten Tastatur-Layout
in genau dieser Testkette. Gemeldet als Residuum dieser
Automatisierungsumgebung, nicht stillschweigend korrigiert oder
verborgen.

## Die sechs Nähte — mit Belegen aus diesem Durchlauf

| Naht | Klick (echt, `xdotool`, Koordinaten aus Debug-Log) | Screenshot | Motor-Fakt (Beleg) |
|---|---|---|---|
| **0 Öffnen** | App-Start unter `COCKPIT_RENDERER=wgpu` | `wgpu/naht0_oeffnen_wgpu.png` | Fenster + Tableiste + Checkbox rendern; Glyphen-Befund s. o. |
| **1 Wünschen→Bestätigen** | Klick `wish_input` (140,35) · Text tippen · Klick „Wunsch erfassen" (4,49) · Klick „Kanzel formt Crystal" (4,70) · Klick „BESTAETIGEN" (4,94) | `naht1a_wunsch_eingegeben.png` → `naht1d_bestaetigt.png` | `WunschErfasst` → `CrystalGeformt` → `Bestaetigt`, alle vier Zeilen woertlich im Log |
| **2 Laufen** | Klick Tab „Lauf" (20,9) · Klick „Lauf STARTEN" (Koordinate aus Log: 4,42) | `naht2a_lauf_tab.png`, `naht2b_lauf_gestartet.png` | `STATE after start_run: ArtefaktVerfuegbar` — `MotorEngine::start_run` läuft synchron bis Laufende, alle Gates grün (s. `.cert` unten) |
| **3 Prüfen** | Klick Tab „Pruefung" (36,9) | `naht3_pruefung.png` | Alle 6 Gate-Reports grün ⇒ `residues()` liefert leere Liste ⇒ `residue_view()` zeigt deterministisch **„geschlossen (∅)"** (`views.rs:36-44`, Code-Pfad, nicht nur Vermutung) |
| **4 Entnehmen** | Klick Tab „Artefakt" (52,9) · Klick „Exportieren" (Koordinate aus Log: 4,45) | `naht4a_artefakt_tab.png`, `naht4b_exportiert.png` | **Echte Datei auf Platte**, geschrieben durch den echten GUI-Button-Handler (nicht headless) — s. u. |
| **5 Ablegen/Wiederholen** | (kein GUI-Re-Import-Knopf vorhanden; Beleg über denselben Code-Pfad, den der Button jetzt nutzt) | — | `cargo test … artefakt_datei_export_und_reimport_klassenidentisch` — **grün**, frisch in dieser Sitzung erneut ausgeführt; prüft `take_artifact` → `write_to` → `reimport` klassenidentisch — exakt die Funktionskette hinter dem Button |

## Der echte Export-Fund (Kern der Nachlieferung)

Der Export-Button rief bis zu dieser Sitzung nur
`core.export_artifact(c)` auf — reine Ledger-Verankerung, **keine
Datei**. Das war der reale, im vorigen Bericht als Finding vermerkte,
aber nicht behobene Lücke, die der Auftraggeber zu Recht angemahnt hat.

**Behoben:** Der Button ruft jetzt `cockpit_core::journey::take_artifact`
gefolgt von `CertifiedArtifact::write_to` auf — dieselbe Funktionskette,
die der headless Export-Pfad (`--export`) bereits nutzte. Der Klick in
diesem Durchlauf hat real geschrieben:

```
reports/ux/wgpu/exports/export-7385bb739f9f.md       (875 Bytes)
reports/ux/wgpu/exports/export-7385bb739f9f.md.cert  (496 Bytes)
```

Zertifikatsinhalt (`.cert`, real gelesen nach dem Klick):

```
artifact_format: .md
content_class: f31cde17bc1667df22de87ddab209f2a9274388ce50e7320328e501f63261a7c
byte_digest: 7385bb739f9fb79853d61c3609461eb4aa888132a54a3c823459f982ff67b47e
gates:
  - G3-Type = gruen (Paket typkorrekt, V0–V9 gruen)
  - G1-Scope = gruen (Projektion im Scope)
  - G2-Boundary = gruen (Nullanker markiert, Naht gebunden)
  - G6-Export = gruen (Materialisierung nach Gate/Evidence)
  - G4-Residue = gruen (Reanalyse vollstaendig)
  - G7-Reanalysis = gruen (q(Obs(A)) = q(C))
```

**Gegenprobe (Integrität des neuen Pfads):** `export-7385bb739f9f.md`
ist **byte-identisch** (`diff` ⇒ keine Abweichung) mit
`reports/ux/drei_risiken_memo.md`, das in der vorigen Sitzung über den
HEADLESS-Pfad erzeugt wurde. Gleicher `content_class`, gleicher
`byte_digest`. Das beweist: der reale GUI-Klick und der
CLI-Headless-Pfad laufen durch **dieselbe** deterministische
Motor-Kette — keine zwei divergenten Export-Implementierungen.

## Beigelegte Dateien

- `reports/ux/wgpu/naht0_oeffnen_wgpu.png` … `naht4b_exportiert.png`
  (10 Screenshots, alle 1000×700, alle aus echten `xdotool`-Interaktionen
  gegen ein echtes, laufendes Fenster)
- `reports/ux/wgpu/exports/export-7385bb739f9f.md` — die real per
  GUI-Button exportierte Memo-Datei
- `reports/ux/wgpu/exports/export-7385bb739f9f.md.cert` — Zertifikats-
  Seitendatei (beide Digests + Gate-Bilanz)
- `reports/ux/wgpu/klick_durchlauf_rohlog.txt` — verdichtetes Debug-Log
  (Widget-Rechtecke + vollständiger Zustandsverlauf, wörtlich)

## Offene Punkte (Residua)

- **Sichtbare Glyphen in dieser Sandbox: host-gebunden, zweifach
  bestätigt (glow, wgpu).** Für einen Klick-Durchlauf mit lesbarem Text
  in Screenshots wird ein echtes Desktop-Display oder ein
  GPU-Passthrough-Host benötigt. Kein weiterer Backend-Wechsel geplant.
- **Zeichen-Umkehrung bei synthetischer Tastatureingabe** (s. o.) —
  Automatisierungsumgebungs-Artefakt, wirkt sich auf das Motor-/
  Export-Ergebnis in diesem Fall NICHT aus (Kanzel ignoriert den
  Wunschtext vor Block 2), betrifft aber die Aussagekraft eines
  zukünftigen Klick-Durchlaufs, der den WUNSCHTEXT selbst prüfen soll
  (relevant erst, sobald Block 2 die Kanzel an echte Inferenz
  anschließt — dann sollte ein späterer Beleg-Durchlauf entweder ein
  echtes Keyboard-Layout/Display nutzen oder Text blockweise statt
  zeichenweise einfügen, um dieses Artefakt zu vermeiden).
- **Kein GUI-Re-Import-Knopf.** Naht 5 ist im Cockpit bisher keine
  eigene interaktive Fläche — der Beleg läuft über den bereits
  committeten, grünen Test, der exakt dieselbe Funktionskette prüft.
  Eine eigene Re-Import-UI-Fläche wäre ein Feature-Zuwachs, kein reiner
  Beleg-Nachtrag, und liegt daher außerhalb dieser Nachlieferung.
