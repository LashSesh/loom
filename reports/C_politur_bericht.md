# Track C — Politur (Dokument 23 C1–C4)
(eigenes Ermessen, klein, ohne Host-Abhängigkeit — alles hermetisch
GRUEN in der Default-CI)

## C1 · Kanzel-Zweitmanifest `gpt-4o`

Bisher gab es KEINE Modell-Auswahl-Naht: das Modell war ein
hart kodierter String je Aufrufstelle, die Manifest-Budgets die
generischen `ModelManifest::complete()`-Defaults. Jetzt (alles im
bestehenden `CloudModelProviderOpenAI`, keine neue Architektur, hinter
dem unveränderten Gateway):

- **`gpt_4o_mini()`** — der DEFAULT, P1-Standardbudgets unverändert.
- **`gpt_4o()`** — das Zweitmanifest: `manifest()` setzt für `gpt-4o`
  das KONSERVATIVE Budget (cost 500 / token 50 000 / rate 5 — jeweils
  die Hälfte des Defaults; stärkeres Modell, engere Leine), VOR-Egress
  durchgesetzt vom unveränderten `model_budget_gate`/`model_rate_gate`.
- **`from_env_default()`** — die Umschalt-Naht: NUR
  `CCE_KANZEL_MODEL=gpt-4o` schaltet um; ungesetzt UND jeder unbekannte
  Wert ergibt den Default (kein freier Modell-String aus der Umgebung —
  nur manifestierte Provider).

Zeugen: `c1_gpt_4o_second_manifest_complete_with_conservative_budget`
(vollständig, Budgets strikt unter dem Default, gleiche Terms-/
Privacy-/Replay-Disziplin) + `c1_env_switch_defaults_to_mini_only_
known_value_switches`.

## C2 · Kanzel-Prompt-Bibliothek (IG-R4 → GESCHLOSSEN)

`cce-inference/src/contracts.rs`: drei VERSIONIERTE
`system_contract`-Vorlagen je Einsatz —

| Einsatz | contract_id | v |
|---|---|---|
| Wunsch-Formung | `kanzel:wunsch_formung` | 1.0.0 |
| Erklärung | `kanzel:erklaerung` | 1.0.0 |
| Reparaturvorschlag | `kanzel:reparaturvorschlag` | 1.0.0 |

Als GEPRÜFTE Assets: jede Vorlage trägt einen deterministischen
Digest, die Bibliothek einen Gesamt-Digest, und der Wächter PINNT ihn
byte-genau (`6ffabe47…c138`) — eine stille Änderung irgendeiner
Vorlage bricht den Zeugen sichtbar; bewusste Änderungen erhöhen die
Version und pinnen neu (derselbe Zwei-Schritt wie bei den
.loom-Seeds). Die Kanzel bezieht den Wunsch-Formungs-Vertrag AUS der
Bibliothek; `wunsch_formung@1.0.0` ist BYTE-IDENTISCH zum bisherigen
Inline-Text — kein Verhaltenswechsel, Alt-Zeugen unberührt.
Erklärung/Reparaturvorschlag erzeugen heute reine Texte ohne
Gateway-Request; ihre Verträge sind die designierten
`system_contract`s für den Moment, in dem diese Flüsse durchs Gateway
gehen (ehrlich dokumentiert, kein Overclaim).

Zeuge: `c2_kanzel_prompt_library_is_checked_asset_and_wired`.

## C3 · Register-Konsolidierung

- `reports/residuen.md`: Abschnitt „Programm Volle Kraft (Dokument 23)
  — Track-Stände" mit Endstatus je Track (A1/A2 GESCHLOSSEN, A3
  Host-Termin, B bis STOPP, C1–C4 GESCHLOSSEN); IG-R4 → GESCHLOSSEN;
  R-Agent-10 → GESCHLOSSEN; P4-Ext-Umklassifizierung war bereits
  eingetragen (offen/optional/blockiert nichts).
- `reports/VOLLAUSBAU_STATUS.md`: Titel + Etappen-Tabelle (A/P5/C) +
  Track-Zeilen (C Intelligenz, G Pakete) + Host-Leiste (Windows
  teilaufgelöst: Cross-Build existiert, nur der Klickpfad-Beweis
  bleibt Host-Termin).

## C4 · Zwei kleinste Registerpunkte (Wahl begründet)

Gewählt, weil beide host-frei und netz-frei sind, je genau eine
Einheit umfassen und unmittelbar die NEUEN Bausteine härten:

1. **R-Agent-10 (Replay-Inputs ohne Use-Case) → GESCHLOSSEN.** Der
   P5-Bauplan-Workbody nutzt jetzt
   `replay_manifest_segment_with_inputs`: die `raw_hash`es der
   Beobachtungs-Bytes stehen als `input_digests` im Replay-Vertrag —
   der Bauplan benennt exakt die Eingabe seiner Destillation. Erster
   realer Use-Case des I.5-Felds.
2. **Digest-Pinning der C2-Bibliothek.** Der Wächter prüfte zunächst
   nur Determinismus; jetzt pinnt er den konkreten Asset-Digest —
   erst damit ist „geprüftes Asset" wörtlich wahr.

Bewusst NICHT angefasst: Repo-R1 (LICENSE — Lizenzentscheidung des
Auftraggebers), R-9 (bewusst dauerhaft, Auftraggeber-Schritt), R-13
(bewusste Betriebsentscheidung), R-Agent-6/7/8 (Host-/Umgebungsgrenzen),
R-8 (Betriebs-Kalibrierlauf braucht Auftraggeber-Gewichtsentscheid).

## Verifikation

fmt/clippy (Default + `process`+`http`)/check_acyclic/run_ci.sh GRUEN;
Alt-Zeugen unverändert (33 Inference-Zeugen inkl. der 3 neuen, alle
Kataloge grün).
