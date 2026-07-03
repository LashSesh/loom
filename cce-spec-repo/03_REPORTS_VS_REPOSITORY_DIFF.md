# 03 — REPORTS vs. REPOSITORY DIFF (behauptet ↔ vorgefunden)

**Methode:** Jeder zentrale Report-Claim wurde gegen den Code-Index geprüft. Kategorien: **A** deckungsgleich (Code gesehen) · **B** deckungsgleich mit dokumentierter, begründeter Abweichung von der Spec (im Report selbst offengelegt) · **C** nur Report (Laufzeit) · **D** Diskrepanz.

## Kategorie A — deckungsgleich, quelltextbelegt (Auswahl der geprüften Claims)

51-Crate-Workspace · Magic/Footer/Frame/SEGTAB/Merkle exakt nach Standard · Kind-Registry inkl. 0x0060–0x0064 · MANIFEST 13 Pflichtfelder · Profil-Pflichtsegmente · ApprovedFetchPlan versiegelt + fetch nur damit · disallowed_actions-Reject-Endzustand (prod_inv_14 inkl. „Endzustand"-Assert) · CountingTransport-0-Beweis · ETag-Differenzabruf · 14 Inference-Gates/16 Residuen als len()-Asserts · Egress genau 1 nach Gates · recorded-Replay spielt ein, model_replay_weak/model_trace_missing benannt · Kanzel schreiblos (beide Ebenen) + INTERPRETATION_MARKER + DegradedKanzel · COCK-INV-1..5-Testquelltext inkl. 3 aufgezeichneter Confirmations · force_through ⇒ immer Fehler · Produkt-Kerntest über 6 Nähte inkl. „geschlossen (∅)"-Assert, Zwei-Digest-Ungleichheit, Re-Import SameClass · Motor-Kerntest inkl. q(Obs(A))=q(C) · Doku=Verhalten-Tests lesen docs/operator/ · GUARD_PHASES G0..G12 · FEATURE_PL 13 · update_dod fail-closed · package.sh-Inhalt · check_acyclic-Regeln (Tore, Reader, Symbol-Scan).

## Kategorie B — Abweichungen von der Spec, im Report offengelegt und begründet (kein stilles Residuum)

1. `spec/` liegt unter `cce-spec-repo/spec/` (R-Agent-1; ZIP-Layout des Auftraggebers; keine Kopie/Bewegung).
2. K/C/V/L/P-Katalognummern rekonstruiert (R-Agent-2/5; verankerte Nummern auf Position).
3. PhaseBlock-10-Tupel-Felder konservativ zugeordnet (R-Agent-3).
4. HyperDAG **5** Kantentypen nach normativer Formel; „6" im Master-Build-Wortlaut als R-Agent-4 geführt.
5. `SourceAdapter::acquire` synchron statt async-Skizze (P9-Determinismus, portidentisch).
6. cce-conformance als deklarierte Wächter-Ausnahme oberhalb der Schichten in check_acyclic.
7. loom-migrate kein eigenes Crate (N10/C5 in verify/codec; CLI-Kontext).
8. N-INF-11 zweistufig (Datenmodell-Zeuge in G8a, Format-Zeuge N15 in G9) — planmäßig.
9. eframe 0.35 `App::ui`-API · 10. zstd nicht aktiviert (CompressionUnsupported statt stiller Annahme) · 11. NFC als Teilmengen-Prüfung fail-closed · 12. CDDL-Dateien offen (Feldverträge in loom-verify normativ implementiert) — alle vier als sichtbare Residuen geführt.

## Kategorie C — nur Laufzeit-Report (Code-Struktur konsistent, Ausführung NOT_EXECUTED)

CI grün gesamt · Testzahlen · Paket 4,2 MB · offline_core_proof-Lauf · golden-gen-Doppellauf · Integritäts-24/24 · „Binary baut für Ziel-OS". → WO-1 (Dok 02 §4).

## Kategorie D — echte Diskrepanzen

**Keine gefunden.** Einzige Zahlen-Irritation (51/48) ist zeitlich erklärt (Dok 02 §3). Kein Report-Claim widerspricht gesichtetem Code; kein gesichteter Code widerspricht einem Report.

**Diff-Verdikt:** Die Berichte sind eine getreue Projektion des Repositorys. Vertrauensstufe der Berichtslage: hoch; verbleibende Prüfschuld: ausschließlich Ausführung (WO-1).
