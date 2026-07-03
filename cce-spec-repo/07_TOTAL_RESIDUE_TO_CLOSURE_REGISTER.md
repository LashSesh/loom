# 07 — TOTAL RESIDUE→CLOSURE REGISTER (jedes offene Ding, ein Schließweg)

Schema je Eintrag: **Ursprung · Ist · Schließung · blockiert?**

| # | Residuum | Ursprung | Ist | Schließweg | blockiert Bau-DoD? |
|---|---|---|---|---|---|
| 1 | Laufzeit-Grün unbestätigt (hier) | dieses Audit | REPORTED | **WO-1** Verifikationslauf | nein (Bestätigungsschuld) |
| 2 | Cockpit-Display-Nachweis | ABSCHLUSS §3.11 | Binary REPORTED, Fenster nie gesehen | **WO-2** | nein (Betrieb) — blockiert UX-4 |
| 3 | macOS/Windows-Pakete | §3.11 | Pipeline da, Hosts fehlen | **WO-3** | nein — blockiert UX-5 |
| 4 | Live-Cloud-Provider | F.3/IG-R1 | Mock + Stub-Echtpfad | **WO-4** + Provider-Onboarding je Manifest | nein (per DoD) |
| 5 | ExternalAgent produktiv | F.3 | mock-getestet | Agent-Wire-Format (IG-R3) + Pilot | nein |
| 6 | CSA-Live-Transport | G8-Bericht | Snapshot-only | **WO-5** | nein |
| 7 | ConnectorAdapter-OAuth | R-16/CSA-R3 | Portvertrag | OAuth-Flow im Betriebsprofil, Secrets in OS-Schlüsselbund | nein |
| 8 | generic_html/static_web | CSA-Weiche | HTMLScopeLeak-Zeuge wacht | zuletzt, mit Sonderregeln + eigenem Lock — oder bewusst nie | nein |
| 9 | 212 Domänen > PL1 | DoD §3.1 | Registry-Slots | Domänen-Wellen via K.6 (Atlas C) | nein |
| 10 | SCALE-2..8-Kerntests | §3.2/R-10 | Strukturen | je Stufe Red(s)-Kerntest (Atlas D) | nein |
| 11 | Nexus-Bridge L9b | §3.3/R-1b | port_only | eigener Spez-+Bauzyklus (Atlas G) | nein |
| 12 | Klonungs-Aktivierung | §3.4/R-13 | implementiert, Lock zu | bewusste Betriebsentscheidung + Lock-Zeremonie | nein |
| 13 | Sync-Mehrgerät | §3.6 | ausgelegt | Sync-Transport (Atlas I) | nein |
| 14 | Signatur-Registry Ed25519+Keyring | LC-R1/§3.8 | SIGNATURE-Kind registriert | Betriebsbindung (Atlas H) | nein |
| 15 | blake3-Zweitprofil | LC-R2 | nur sha2-256 | optionales Profil aktivieren | nein |
| 16 | CDDL unter schemas/ | LC-R3/§3.11 | Verträge in loom-verify | CDDL aus Code extrahieren (mechanisch) | nein |
| 17 | GUI-Feindesign 5 Ansichten | LC-R5 | Kernansichten da | Design-Pass nach WO-2 | nein |
| 18 | zstd-Transportprofil | G9-Bericht | CompressionUnsupported (sichtbar) | zstd-19-nodict als Betriebsprofil, Klassen-Digest unberührt | nein |
| 19 | NFC-Vollnormalisierung | G9-Bericht | Teilmengen-fail-closed | Unicode-Tabellen einbetten ODER fail-closed als Politik festschreiben | nein |
| 20 | .docx-Export | S1.10-R1 | .md verlustarm | docx-Backend hinter identischem materialize-Vertrag | nein |
| 21 | R-8 Score-Kalibrierung | HBM | RD-Parameter | Betriebs-Kalibrierlauf, RD-dokumentiert | nein |
| 22 | R-9 Amendment-Einarbeitung | spec-Disziplin | Overlays gelten | redaktioneller Auftraggeber-Schritt (nie Agent) | nein |
| 23 | Lizenz/NOTICE (Repo-R1) | Inventar | „SEE-REPO-ROOT" ohne Datei | Auftraggeber-Entscheidung + LICENSE-Datei | **ja für Veröffentlichung** (nicht für Bau) |
| 24 | R-Agent-1..5 | residuen.md | dokumentierte Auslegungen | bei nächster Spec-Redaktion kanonisieren (Nummern/Felder festschreiben) | nein |
| 25 | Fuzzing/Threat-Model | Atlas K (neu) | Symbol-Scan + Limits | cargo-fuzz-Harness + 2-Seiten-Threat-Model | nein |

**Registerbefund:** 25 Einträge, **0 blockieren die Bau-DoD**, genau 1 (Lizenz) blockiert eine etwaige Veröffentlichung. Jeder Eintrag hat einen konkreten, kleinen ersten Schritt.
