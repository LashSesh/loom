# 05 — RUNTIME & UX REALITY AUDIT (Was kann ein Mensch heute wirklich erleben?)

**UX-Realitätsskala:** UX-0 nichts startbar · UX-1 nur Tests · UX-2 CLI für Entwickler · UX-3 CLI + baubares GUI ohne Display-Nachweis · UX-4 GUI nachweislich startbar, geführte Referenzreise · UX-5 installierbar für Nicht-Entwickler (alle Ziel-OS), Onboarding · UX-6 Betriebsreife (Provider live, Sync, Update-Kanal gelebt).

## Ist-Einstufung: **UX-3** (mit starkem UX-4-Fundament)

**Was real vorhanden ist (quell-/strukturbelegt):** vollständige Bedienlogik als deterministische Zustandsmaschine (LEER→…→ARTEFAKT_VERFÜGBAR) mit Bestätigungsgrenze, aufgezeichneten Confirmations, ehrlichen Endzuständen — **und der komplette Klickpfad existiert als getesteter Code** (drive_to_artifact, full_journey_reaches_artifact, produkt_kerntest_ueber_sechs_naehte). Eine native egui-App (cockpit-app) rendert exakt diese Maschine; Binary-Build REPORTED. CLI-Werkzeuge: loom (inspect/verify/ls), loom-viewer (motorfrei), nexus (Referenzlauf), golden-gen. Saat-Workbodies als .loom. Operator-Doku, deren Aussagen von Tests gegen das Verhalten geprüft werden.

**Was heute NIEMAND erlebt hat (ehrlich):** (1) **Kein Mensch hat das Cockpit-Fenster gesehen** — headless CI kann kein Display; der egui-Pfad ist ungerendert-ungeklickt (Report sagt das selbst). (2) **Kein Nicht-Entwickler kann es installieren:** es gibt ein Linux-tar.gz mit Binaries (REPORTED), aber keinen Installer, kein macOS/Windows-Artefakt, keine Erststart-Führung außerhalb der Doku. (3) **Keine echte KI antwortet:** Kanzel läuft gegen LocalModel-Stub/Mocks — produktiv fühlt sich das erst mit einem echten lokalen Modell oder Provider an (per DoD bewusst Betrieb). (4) **Kein echtes Netz:** CSA nur Snapshot/Fixture (derselbe versiegelte Pfad, aber nie gegen eine reale API gefahren).

## Die vier UX-Lücken als präzise Arbeitspakete

**WO-2 Display-Nachweis (UX-3→UX-4):** auf einem Linux-Desktop: `cargo run -p cockpit-app` → Screenshot der vier Flächen; die Referenzreise einmal DURCHKLICKEN (Wunsch→Bestätigen→Lauf→Prüfung „geschlossen (∅)"→Export→Re-Import) und je Naht einen Screenshot ablegen (`reports/ux/`). Abnahme: Reise ohne Konsole erlebt, Artefakt-Datei real entnommen.
**WO-3 Paket-Vervollständigung (UX-4→UX-5):** macOS-/Windows-Build-Hosts, `ci/package.sh` dort ausführen; Erststart-README ins Paket (3 Schritte: entpacken, Binary starten, Saat-Reise); optional .desktop/.app/.msi-Hüllen. Abnahme: frische Maschine, Nicht-Entwickler erreicht das Artefakt.
**WO-4 Erstes echtes Modell (Kanzel fühlbar):** ein lokales GGUF/ollama-artiges Modell als LocalModelProvider-Implementierung hinter DEMSELBEN Gateway (replay_policy=recorded); Abnahme: form_wish liefert modellgeformte Annahmen, R-INF-2-Klasse bleibt grün, Offline-Modus unverändert.
**WO-5 Erste echte Quelle (CSA live):** ein realer HTTP-Transport (implementiert das Transport-Trait) hinter unverändertem approve_fetch; Wikimedia-Referenzlauf gegen die echte API mit festem Snapshot-Fixieren; Abnahme: ref_2-Klasse identisch, Attribution transportiert, Rate-Budget respektiert.

## Ehrliches UX-Verdikt

Die *Architektur* der Bedienbarkeit ist außergewöhnlich fertig (die Reise ist Code, nicht Absicht); die *Erfahrbarkeit* hängt an vier klar geschnittenen, kleinen Betriebsschritten. Nichts davon erfordert neue Architektur — alles dockt an existierende Ports (Transport-Trait, ModelProvider-Trait, package.sh) an.
