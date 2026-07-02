# CCE — DETAIL-SPEZIFIKATION S11: AUSLIEFERUNG, PAKETIERUNG & BETRIEB

**Elfte D-Spec.** Wie das fertige Produkt als installierbare native App ausgeliefert, konfiguriert, aktualisiert und betrieben wird — der gesamte Lebenszyklus konsolenfrei.

**Status:** Detail-Spezifikation v1.0, auf Bauverfassungs-Tiefe. Baut auf: R-Plan-1 (native Desktop-App), S3.1.1 (reine Rust-App), S7.2 (Zwei-Digest, Zertifikat), S8.3 (Regressionswächter/CI), S9 (Persistenz/CAS), S13 (Governance).

---

## S11.0 — Einordnung & Leitsatz

S11 macht aus dem spezifizierten System ein **auslieferbares, betreibbares Produkt**: installiert wie jedes native Programm, konfiguriert ohne Konsole, sicher aktualisiert, zuverlässig betrieben. Das ist, was es „an jemanden übergebbar" macht statt „ein Repo, das man kompiliert".

Leitsatz:

> *Der gesamte Lebenszyklus — installieren, konfigurieren, aktualisieren, betreiben — geschieht ohne Konsole. Und jede Aktualisierung steht unter derselben Definition von Fertig wie der Bau selbst: Sie wird nur ausgeliefert, wenn der Kerntest und die Bibliotheks-Cubes grün sind.*

Zwei Grundfesten:

- **Konsolenfrei über den ganzen Lebenszyklus.** `cargo`, CLI und Terminal sind reine **Bauwerkzeuge des Coding-Agenten**; dem Operator wird an **keinem** Lebenszyklus-Schritt eine Konsole zugemutet (R-Plan-1).
- **Updates unter derselben DoD.** Eine Aktualisierung wird nur ausgeliefert, wenn `DoD(cce)=1` (Bauverfassung §8.7) gilt — insbesondere der Kerntest grün und die Bibliotheks-Cubes (S8.3) grün. Das erweitert „kein 'ah, da fehlt noch was' vor der Auslieferung" auf den Update-Kanal.

---

## S11.1 — Das Auslieferungs-Artefakt

- **Eine native App je Plattform** (Linux/macOS/Windows), aus dem Rust-Monorepo (`cce/`) kompiliert.
- **Selbst-enthaltend.** Die App bündelt: den **Motor** (`cce-*`-Crates), das **Cockpit** (native GUI, S3.1.1), die **lokale Persistenz** (CAS, S9) und die **Saat-Bibliothek** (Referenz-/Negativ-Cubes, S8).
- **Kein Server, kein Browser, keine Konsole.** Ein natives Binär/App-Bundle; keine Laufzeit-Abhängigkeit von einem Terminal oder einem separaten Dienst.
- **Für Einzelnutzer selbst-genügsam** (R-Plan-4): alles Nötige liegt in der App bzw. im lokalen Workspace.

---

## S11.2 — Installation (konsolenfrei)

- **Nativer Installer je Plattform** (z. B. `.dmg`/`.app`, `.msi`/`.exe`, `.deb`/AppImage). Doppelklick, wie jede App.
- **Selbst-Initialisierung beim ersten Start.** Die App legt Workspace, CAS und Ledger beim ersten Lauf an (S9) — **keine** Terminal-Befehle, **keine** manuell editierten Konfigurationsdateien.
- **Saat-Bibliothek sofort aktiv.** Nach Installation ist die Dokument-Referenz-Cube-/Negativ-Cube-Menge vorhanden (S11.6) — der Regressionswächter (S8.3) ist ab Installation aktiv.

---

## S11.3 — Konfiguration (konsolenfrei)

- **Alles in-App.** Konfiguration geschieht in einer **Einstellungs-Fläche** im Cockpit, nicht in Konsole-editierten Dateien.
- **Konfigurierbar:** Workspace-Ort (S9), der LLM-Endpunkt/-Zugang der KI-Kanzel (S3.10-R4), Export-Voreinstellungen (S7), Sync-Einstellungen (sobald Sync kommt, S9.6).
- **Sicher gespeichert.** Sensible Konfiguration (LLM-Zugangsdaten) wird **sicher** abgelegt (OS-Schlüsselbund/Keychain), **nie** im Klartext im Ledger oder in einer Konfigurationsdatei (§S11.9).
- **Persistiert im Workspace.** Nicht-sensible Konfiguration lebt im Workspace (S9).

---

## S11.4 — Aktualisierung (sicher, konsolenfrei, unter der DoD)

- **In-App-Update.** Auf Update prüfen → herunterladen → anwenden → neu starten, wie jede native App. Keine Konsole.
- **Unter derselben DoD ausgeliefert.** Ein Update wird nur veröffentlicht, wenn `DoD(cce)=1` (Bauverfassung §8.7): Kerntest grün, **alle Referenz-Cubes grün**, **alle Negativ-Cubes rot** (S8.3). Ein Update, das den Abschluss bräche oder eine Prohibition schwächte, ist **nicht auslieferbar** — der Regressionswächter fängt es vor der Auslieferung.
- **Bricht keine vergangenen Artefakte.** Weil Artefakte content-adressiert sind und Zertifikate an den **Inhaltsklassen-Digest** binden (S7.2), invalidiert ein App-Update **keine** vergangenen Artefakte — sie bleiben prüfbar.
- **Orphaniert keinen Workspace.** Das CAS-Format ist stabil/versioniert; eine aktualisierte App liest **alte Workspaces** (Rückwärtskompatibilität des Speichers ist Pflicht, §S11.7).

---

## S11.5 — Betrieb, Offline-Fähigkeit & Determinismus (starke Zuverlässigkeitseigenschaft)

- **Nativer Prozess.** Deterministischer Motor, lokaler CAS, das Cockpit-GUI. Kein Hintergrund-Server.
- **Der Kern läuft vollständig offline und deterministisch.** Motor, Abschluss, Replay, Inspektion, Artefakt-Ausgabe und Persistenz brauchen **kein** Netz.
- **Nur die KI-Kanzel ruft hinaus** (zu einem LLM) — und sie liegt **außerhalb** des Abschlusspfads (S13.2). Daraus folgt eine wichtige Garantie:

```
Kein Internet ⇒ man kann WEITERHIN:
    bestätigte Kristalle deterministisch laufen lassen,
    inspizieren, replayen, Artefakte entnehmen.
Man verliert NUR: die KI-Hilfe beim FORMEN neuer Wünsche.
```

Die **Garantien hängen nie am Netz.** Selbst die Abwesenheit der KI blockiert einen bestätigten Crystal nicht — der Motor arbeitet ohne sie; die KI hilft nur beim *Formen* (S4). Das ehrt die Determinismus-Disziplin (P9): der replaybare Kern ist netzunabhängig.

---

## S11.6 — Paketierung der Saat-Bibliothek

- Die ausgelieferte App bündelt die **Saat-Bibliothek** (S8): den Dokument-Referenz-Cube (Drei-Risiken-Memo) + die Negativ-Cubes (je Prohibition/Residuentyp einer).
- Damit hat eine frische Installation ein **funktionierendes, bewiesenes Repertoire** ab Werk, und der Regressionswächter ist ab Installation aktiv.

---

## S11.7 — Versionierung & Kompatibilität

- **App-Version aufgezeichnet.** Artefakte/Ledger verzeichnen die **App-Version**, die sie erzeugte (Herkunft, S7.1).
- **Stabiles, versioniertes CAS-Format.** Eine aktualisierte App liest ältere Workspaces; ein Update **orphaniert nie** einen bestehenden Workspace.
- **Rückwärtskompatibilität ist Pflicht.** Formatänderungen sind migrierend, nie zerstörend (konsistent mit S9: alte Versionen bleiben erhalten).

---

## S11.8 — Adapter-Parität für die Auslieferung

Die Assets **jeder** Domäne (Bibliothek, Adapter) werden im **selben** Paket ausgeliefert; eine neue Domäne ändert den Auslieferungs-Mechanismus **nicht** — sie fügt nur ihren Bibliotheks-Slot (S8.5) und ihren Adapter (S1.8) hinzu. Ein Paket, ein Installer, ein Update-Kanal für alle Domänen.

---

## S11.9 — Read-only-/Betriebs-Garantien (Verbote auf Auslieferungsebene)

Abwesende Handhaben:

- **kein Konsolen-Zwang** an irgendeinem Lebenszyklus-Schritt (Installieren/Konfigurieren/Aktualisieren/Betreiben).
- **kein Update außerhalb der DoD** — ein Update, das Kerntest oder Bibliotheks-Cubes bräche, ist nicht auslieferbar (§S11.4).
- **kein Invalidieren vergangener Artefakte/Zertifikate** durch ein Update (§S11.4).
- **kein Orphanieren eines Workspace** durch ein Update (§S11.7).
- **keine Klartext-Zugangsdaten** — sensible Konfiguration nur im OS-Schlüsselbund (§S11.3).

---

## S11.10 — Sichtbare Residuen dieser Spezifikation

Kein stilles Loch:

- **R1 (Plattform-Ziele & Installer-Technik).** Konkrete Bündelung der reinen Rust-GUI je Plattform ist beim Bau zu wählen; das Prinzip (nativ, selbst-enthaltend, konsolenfrei) steht fest.
- **R2 (Update-Kanal).** Der konkrete Auslieferungskanal (Update-Server/Signatur) ist zu wählen; die DoD-Bindung des Updates steht fest.
- **R3 (Zugangsdaten-Speicher).** Der konkrete Schlüsselbund-Mechanismus je Plattform ist zu wählen; „nie Klartext" steht fest.
- **R4 (Code-Signatur).** Signierung/Notarisierung je Plattform ist zu fixieren; irrelevant für die Kern-Garantien.

---

## S11.11 — Abnahme (DoD dieser Ebene)

```
DoD(S11) = 1  ⟺
    Das Produkt ist eine native, installierbare App je Plattform, selbst-enthaltend (Motor+Cockpit+CAS+Saat-Bibliothek)
  ∧ Installieren / Konfigurieren / Aktualisieren / Betreiben sind ALLE konsolenfrei (§S11.2/3/4/5)
  ∧ der Kern läuft vollständig offline und deterministisch; nur die KI-Kanzel braucht Netz,
        und ihre Abwesenheit blockiert keinen bestätigten Lauf (§S11.5)
  ∧ Updates stehen unter derselben DoD: Kerntest + Bibliotheks-Cubes grün, sonst nicht auslieferbar (§S11.4, S8.3)
  ∧ Updates invalidieren keine vergangenen Artefakte/Zertifikate und orphanieren keinen Workspace (§S11.4/S11.7)
  ∧ die Saat-Bibliothek ist im Paket; der Regressionswächter ist ab Installation aktiv (§S11.6)
  ∧ sensible Konfiguration (LLM-Zugang) sicher im OS-Schlüsselbund, nie Klartext (§S11.3/S11.9)
  ∧ ein Paket/Installer/Update-Kanal für alle Domänen (§S11.8, Adapter-Parität)
  ∧ Engine-DoD und DoD(S1/S2/S3/S4/S5/S6/S7/S8/S9/S13) bleiben unberührt
```

---

## S11.12 — Anschluss

S11 schließt die Auslieferungs-Ebene: Das Produkt ist eine native, selbst-enthaltende, konsolenfrei installierbare/konfigurierbare/aktualisierbare App; der Kern läuft offline und deterministisch; Updates stehen unter derselben DoD wie der Bau und brechen nie Vergangenes; die Saat-Bibliothek und der Regressionswächter sind ab Installation da.

**Nächste D-Spec (Systemlandkarte §6):** **S12 — Operator-Dokumentation & Onboarding** — das Handbuch fürs *Benutzen* (nicht fürs Bauen): wie man einen Wunsch formuliert, läuft, Gates/Residuen liest, Artefakte entnimmt, replayt; Vermittlung des mentalen Modells (S2.6), unter der Claim-Schranke (S13.4). Danach die **Produktabnahme S10** — der Schlussstein, der `ProduktDoD=1` über alle Ebenen prüft.

*Ende der Detail-Spezifikation S11.*
