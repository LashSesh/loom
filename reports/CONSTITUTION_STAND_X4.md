# Constitution-Stand nach Etappe X4 (Dokument 16 §3e)

Acht getrennte DoD-Klassen (`08_TOTAL_CLOSURE_CONSTITUTION.md`), keine
erbt Grün von einer anderen. Stand nach Ring E1–E5 + R-CYC-1:

**K1 · BuildDoD — ERFÜLLT.** Die Formel aus `02_MASTER_DOD` (a–i) ist
quelltextgestützt erfüllt: G0–G12 vollständig gebaut, Vollausbau
213/213 Domänen, Etappen X1–X4 abgeschlossen. Zeuge: der gesamte
Bericht-Zug `reports/X1_bericht.md` … `reports/X4_bericht.md` +
`reports/VOLLAUSBAU_STATUS.md`.

**K2 · VerificationDoD — ERFÜLLT.** Jeder Claim ist EXECUTED durch
unabhängigen Lauf, nicht nur berichtet: 175 Testgruppen laufen bei
jedem CI-Durchlauf real (`bash ci/run_ci.sh`), inklusive R-CYC-1 (dem
einen Zeugen, der acht Stationen END-TO-END real durchläuft, nicht
simuliert) und der Replay-Identitätsproben an jeder Station. Zeuge:
`ci/run_ci.sh` GRUEN nach jeder Einheit dieser Session,
`python3 ci/check_acyclic.py` (55 Crates, DAG sauber).

**K3 · ExperienceDoD (UX) — UX-3+ (host-gebunden).** UX-4/5/6 bleiben
offen (Fenster erlebt/Nicht-Entwickler installiert/Betrieb gelebt) —
das sind Host-Termine (GPU-Klickpfad mit gerenderten Glyphen,
Drei-OS-Pakete), keine Bau-Residuen. Der wgpu-Klick-Durchlauf (Block 1,
`reports/ux/reise_protokoll_v2.md`) und die fünf LC-R5-Pflichtansichten
(Block 2) sind real erbracht, so weit ohne den Host-Termin möglich.

**K4 · OperationsDoD — teilerfüllt.** Lokal voll: echte Provider-Pfade
(LocalExtractiveModel, WikimediaAdapter live+Fixture), Update-Kanal
(`update_dod`) strukturell gebaut. Offen: Live-Onboarding und ein
realer Release-Durchlauf sind Betriebsentscheidungen (WO-4/5), keine
Bau-Residuen — sie bleiben es bewusst, bis der Auftraggeber sie
auslöst (P1/OpenAI-Provider, s. u., löst den GGUF-Teil der Host-Leiste
strukturell, sobald ein Schlüssel gesetzt ist).

**K5 · DomainDoD(D) — D01 = PL4, 212 × PL3.** Track A vollständig
(213/213, `reports/VOLLAUSBAU_STATUS.md`). PL4-Reifepfade (Betriebs-
evidenz je Domäne) bleiben ein spaeterer, laufender Prozess — kein
Bau-Residuum, sondern der ausdrücklich vorgesehene nächste Reifegrad.

**K6 · ScaleDoD(s) — SCALE-1..3 erfüllt.** MultiScaleClosure(1) aus dem
Grundbau, SCALE-2 (X1b, Dokumentenmappe), SCALE-3 (E2, Projektraum +
R-CYC-1 Station 3, MSC(1→2→3) real geschlossen). SCALE-4..8 bleiben
typisierte, ungebaute Stufen (Atlas D) — kein aktuelles Residuum,
sondern ausdrücklich zurückgestellter Ausbauhorizont.

**K7 · EcosystemDoD — Kern erfüllt.** Signatur-Registry-Vollform
(X1e), zstd/blake3-Transportprofile (X1c), CE-1 als erste echte
CoreExtension durch den S14-Pfad (`reports/CE1_beweiszug.md`, DoD
ERFÜLLT), `loom-sdk` + wasm32-Viewer (E4b), Klassen-Registry als
`.loom`-Katalog-Workbody + zweite `CitationResolver`-Implementierung
(E4c), L9b Normic Memory (E5/X3, R-1b GESCHLOSSEN). CDDL/MIME-
Registrierung und ein Fremdsprach-SDK bleiben offen (Atlas H) — echte,
noch nicht angeforderte Ausbauschritte, kein Baumangel.

**K8 · AssuranceDoD — teilerfüllt.** Strukturelle Verbote + Symbol-Scan
(`ci/check_acyclic.py`s Tor-Trennung/Socket-Scan) + Bomben-/Limit-Pfade
sind gebaut und laufen bei jedem CI-Durchlauf. Ein dediziertes
Fuzz-Harness (`cargo-fuzz` auf `decode_sealed`/`loom_canon::decode`)
und ein externes Parser-Review sind noch nicht angelegt — offen,
Atlas K, ein klar umrissener nächster Schritt.

## TotalClosure-Stand

`TotalClosure := K1 ∧ … ∧ K8` bleibt ein Horizont, kein Sprint-Ziel
(08, Verfassungsregel). Nach X4: **K1, K2, K7(Kern) erfüllt** ·
**K3, K5, K6 jeweils bis zur vorgesehenen Stufe erfüllt, Rest bewusst
zurückgestellt** · **K4, K8 teilerfüllt, mit benanntem, nicht
verborgenem Restweg**. Kein K-Grün ohne sichtbares Residuenregister der
Klasse (Verfassungsregel 3) — Gesamtregister: `reports/residuen.md`.

Mit X4 ist **die technologische Expansionsstufe vollständig**
(Dokument 16 §4). Danach verbleiben ausschließlich: die Host-Leiste
(GGUF-LLM, OS-Keyring-live, macOS/Windows, GPU-Klickpfad), die
PL4-Reifepfade, und Betriebsentscheidungen (Lizenz, Live-Onboarding,
Sync-Betrieb) — sowie, ab der Messlatte „Parity, then Surpass"
(Dokument 17), die neue Abnahme-Klasse **K9 · CompetitiveDoD** als
Fundament auf diesen acht Klassen, nicht als Ersatz.
