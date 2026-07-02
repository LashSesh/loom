# CCE — DETAIL-SPEZIFIKATION S7: ARTEFAKT-AUSGABE JE DOMÄNE

**Fünfte D-Spec.** Wie das geschlossene, materialisierte Artefakt entnommen, exportiert, abgelegt und domänen-nativ verwendet wird — als tragbares Ding in der Welt, das seine Herkunft und seinen Abschluss mitträgt.

**Status:** Detail-Spezifikation v1.0, auf Bauverfassungs-Tiefe. Baut auf: BAUVERFASSUNG (Artifact, Certificate, content-addressing P10), S1 (DocArtifact, Zwei-Digest-Ansatz S1.7), S3 (Artefakt-Fläche S3.2.4, Persistenz S3.7), S6 (Nachvollziehbarkeit, deren Ausgabe-Gegenstück S7 ist).

---

## S7.0 — Einordnung & Leitsatz

Das Artefakt ist der **Zweck** der ganzen Apparatur: Der Operator wollte ein Dokument, jetzt braucht er es — zum Öffnen, Verwenden, Aufbewahren, Weitergeben. S7 macht das geschlossene Artefakt zu einem realen Ding, **ohne** die Garantien preiszugeben.

Leitsatz:

> *Ein ausgegebenes Artefakt ist Bytes, gebunden an ihre Herkunft: Es trägt den bestätigten Crystal, den Lauf, das Zertifikat und seine zwei Digests mit sich — und kann darum überall in der Welt auf seinen Abschluss zurückgeführt werden.*

Zwei Grundfesten:

- **Ein Artefakt ist nie nur eine Datei.** Es ist Bytes **plus** eine unfälschbare Herkunftskette (Crystal → Lauf → Zertifikat) und zwei Digests (§S7.2). Die Datei ohne diese Bindung ist bloß eine Kopie, kein zertifiziertes Artefakt.
- **Die Garantie reist mit, aber nur für die exakte Bedeutung.** Das Zertifikat gilt für **genau eine** kanonische Inhaltsklasse. Äußere Bearbeitung verändert die Klasse — und wird dadurch **nachweisbar** (§S7.6). Das System verhindert äußere Nutzung nicht; es macht sie *prüfbar*.

---

## S7.1 — Das Artefakt-Objekt (vollständig)

Ein `DocArtifact` (allgemein: ein Domänen-`Artifact`) besteht aus:

| Bestandteil | Inhalt |
|-------------|--------|
| **Bytes** | die gerenderte Datei (`.docx` primär, `.md` sekundär, `.pdf` tertiär-später) |
| **Byte-Digest** | Digest der exakten Bytes — identifiziert die **Datei** (§S7.2) |
| **Inhaltsklassen-Digest** | Digest der kanonischen Inhaltsklasse — identifiziert die **Bedeutung** (§S7.2) |
| **Herkunftskette** | Referenzen auf den bestätigten `DocCrystal`, den `RunDescriptor`, den Ledger-Eintrag |
| **Zertifikat** | das `Certificate` des Abschlusses (Bauverfassung VC10), unabhängig prüfbar |

Die Herkunftskette und das Zertifikat sind das, was ein zertifiziertes Artefakt von einer bloßen Datei unterscheidet.

---

## S7.2 — Das Zwei-Digest-Modell (der technische Kern)

Aus S1.7 hier vollständig ausgeführt. Das Artefakt trägt **zwei** getrennte Digests, weil Datei und Bedeutung verschiedene Dinge sind:

- **Byte-Digest** — Hash über die **exakten gerenderten Bytes**. Identifiziert die konkrete Datei. Ändert sich bei jeder kosmetischen Änderung (andere Schrift, anderer Seitenumbruch). Zweck: Datei-Identität, Dedup, Integrität der Ablage.
- **Inhaltsklassen-Digest** — Hash über die **kanonische Inhaltsklasse** (S1.6: gleiche Einheiten, gleicher Naht-Graph, gleiche Abdeckung, gleiche bedeutungstragende Reihenfolge). Identifiziert die Bedeutung. Ändert sich **nur**, wenn sich der semantische Kern ändert. Zweck: Äquivalenz, Replay, Zertifikats-Bindung.

**Die Schlüsselbeziehung:**

```
verschiedene Bytes, gleiche Bedeutung   ⇒  Byte-Digest verschieden,  Inhaltsklassen-Digest GLEICH
gleiche Bytes                            ⇒  beide gleich
gleiche Bytes, andere Bedeutung          ⇒  unmöglich (Bedeutung ist Funktion der Bytes)
verschiedene Bedeutung                   ⇒  Inhaltsklassen-Digest verschieden
```

Deshalb reproduziert **Replay die Inhaltsklasse, nicht die Bytes** (Bauverfassung INV-10, S1.6, S6.5): Zwei Läufe mit gleichem bestätigtem Crystal + RunDescriptor erzeugen dieselbe **Klasse** (gleicher Inhaltsklassen-Digest), auch wenn die gerenderten Bytes kosmetisch differieren könnten. Das **Zertifikat bindet an den Inhaltsklassen-Digest** — es bezeugt die Bedeutung, nicht eine kosmetische Fassung.

---

## S7.3 — Entnahme (Öffnen, Exportieren, Kopieren)

Die Artefakt-Fläche (S3.2.4) bietet drei Entnahme-Wege:

- **Öffnen (domänen-nativ).** Startet das Artefakt im nativen Programm: `.docx` in Word/LibreOffice, `.md` in Editor/Vorschau. Der Operator sieht und nutzt das fertige Dokument.
- **Exportieren.** Legt das Artefakt an einen vom Operator gewählten Ort — als Datei im gewünschten Exportformat (§S7.4), **mit** beigefügter Herkunft/Zertifikat (als Begleit-Metadaten), sodass die Bindung erhalten bleibt.
- **Kopieren.** Übernahme in den Workspace (content-adressiert, §S7.5) oder in die Zwischenablage.

---

## S7.4 — Exportformate & Formattreue

- **Formate (Dokument):** `.docx` (primär), `.md` (sekundär), `.pdf` (tertiär, zurückgestellt §S7.10-R1).
- **Treue-Garantie.** Ein Export in ein Format ändert die **kanonische Inhaltsklasse nicht** — er fügt nur formatspezifische Kosmetik hinzu (die die Kanonisierung ohnehin wegstreift). Der Inhaltsklassen-Digest bleibt gleich; das Zertifikat gilt weiter.
- **Formatverlust als sichtbares Residuum.** Kann ein Zielformat die volle Inhaltsklasse **nicht** tragen (z. B. ein Format, das eine bestimmte Naht-Beziehung nicht darstellen kann), wird das als **`format_loss`-Residuum sichtbar** gemacht (P4/V2) — nie still hingenommen. Der Operator entscheidet dann bewusst; das exportierte Artefakt in diesem Format ist als „inhaltsklassen-reduziert" gekennzeichnet und trägt **kein** volles Zertifikat.

Damit reicht die „kein stilles Residuum"-Disziplin bis in den Export: Bedeutung geht nie unbemerkt verloren.

---

## S7.5 — Content-adressierte Ablage (sync-fähig)

- **Ablage nach Byte-Digest.** Artefakte liegen im Workspace **content-adressiert nach ihrem Byte-Digest** (Bauverfassung P10, S3.7): unveränderlich, dedup-fähig, integritätsgesichert.
- **Herkunft im Ledger.** Die Kette Crystal → Lauf → Artefakt steht im append-only Ledger (S6.4); das Artefakt ist von seinem Lauf aus auffindbar und umgekehrt.
- **Auffinden & Auflisten.** Ein Artefakt ist über seinen Byte-Digest abrufbar; über den Inhaltsklassen-Digest sind **alle kosmetischen Fassungen derselben Bedeutung** auffindbar.
- **Sync-Bereitschaft.** Da inhaltsadressiert und append-only, ist die Ablage konfliktfrei synchronisierbar (R-Plan-5); die Sync-Ebene selbst ist S9. Der `PersistenceAdapter`-Trait (S3.1.2) trennt „lokal jetzt" von „synchronisiert später" hinter stabiler Schnittstelle.

---

## S7.6 — Re-Import & Erkennung äußerer Bearbeitung (die elegante Folge)

Ein exportiertes Artefakt kann **re-importiert** und über den Domänen-`reanalyze` (S1.3) zurück in einen DocCrystal gelesen werden. Daraus folgt unmittelbar eine Manipulations-Erkennung:

```
Re-Import eines Artefakts:
    reanalyze(Datei) → DocCrystal''
    Inhaltsklassen-Digest(DocCrystal'') == Zertifikats-Digest ?
        JA  → das Artefakt trägt exakt die zertifizierte Bedeutung (unverändert)
        NEIN → das Artefakt wurde AUSSERHALB bearbeitet; Bedeutung hat sich geändert;
               das Zertifikat gilt nicht mehr — sichtbar gemeldet
```

**Das ist eine Konsequenz, kein Zusatzmechanismus:** Weil das Zertifikat an die Inhaltsklasse bindet (§S7.2) und `reanalyze` die Inhaltsklasse zurückgewinnt (S1.3), ist jede äußere semantische Bearbeitung automatisch **nachweisbar**. Das System **verhindert** äußere Bearbeitung nicht (der Operator darf sein `.docx` in Word ändern) — aber es macht transparent, dass das Ergebnis dann **nicht mehr** das zertifizierte Artefakt ist. Wer das geänderte Dokument wieder schließen will, führt es erneut durch den Motor (neuer Wunsch/Crystal/Lauf/Zertifikat).

Rein kosmetische äußere Änderungen (andere Schrift) ändern die Inhaltsklasse **nicht** und lassen das Zertifikat gültig — genau die richtige Grenze.

---

## S7.7 — Domänen-native Verwendung (in der Tiefe)

- **Öffnen/Lesen/Drucken/Teilen.** Das `.docx`/`.md` verhält sich wie jedes Dokument: öffnen, lesen, drucken, per E-Mail teilen. Das System steht dem nicht im Weg.
- **Bearbeiten außerhalb.** Erlaubt, aber: verändert es die Bedeutung, bricht das Zertifikat nachweisbar (§S7.6). Das ist eine ehrliche Eigenschaft, keine Einschränkung.
- **Weitergabe mit Herkunft.** Beim Export können Herkunft und Zertifikat als Begleit-Metadaten mitgegeben werden, sodass ein Empfänger die Bedeutung unabhängig prüfen kann (Zertifikatsprüfung, S6.6) — das Artefakt trägt seinen Beweis in die Welt.

---

## S7.8 — Adapter-Parität für die Ausgabe

S7 ist die Ausgabe-Seite des `DomainAdapter`-Vertrags (S1.8, Punkte 5/10/11). **Jede** Domäne stellt für die Ausgabe **identisch** bereit:

- `materialize` — Gewebe → Artefakt-Bytes (nur Kosmetik, kein neuer Inhalt).
- die **zwei Digests** — Byte-Digest und Inhaltsklassen-Digest, nach demselben Muster.
- die **Herkunftskette** — Crystal → Lauf → Zertifikat.
- `export_formats` — die Formatliste der Domäne, mit `format_loss`-Meldung, wo ein Format die Inhaltsklasse nicht trägt.
- `native_open` — die domänen-native Öffnen-Aktion.
- die **Re-Import/Reanalyze**-Fähigkeit für die Manipulations-Erkennung.

`check_adapter_parity` (Systemlandkarte §5) prüft, dass jede Domäne diese Ausgabe-Teile vollständig hat. Keine Domäne gibt „anders" aus.

---

## S7.9 — Read-only-Garantien der Ausgabe (Verbote auf Ausgabeebene)

Abwesende Handhaben, die die Prohibitionen erzwingen:

- **kein „Herkunft entfernen"** — ein zertifiziertes Artefakt kann nicht von seiner Kette getrennt und dennoch als zertifiziert dargestellt werden.
- **kein „Zertifikat fälschen"** — Zertifikate sind an den Inhaltsklassen-Digest gebunden und unabhängig prüfbar; ein erfundenes Zertifikat fällt bei Prüfung durch.
- **kein stiller Bedeutungsverlust** — Formatverlust ist stets sichtbares Residuum (§S7.4, V2).
- **kein „unzertifiziert als zertifiziert"** — ein inhaltsklassen-reduziertes oder äußerlich bearbeitetes Artefakt wird nie als voll zertifiziert ausgegeben (§S7.4/S7.6).

---

## S7.10 — Sichtbare Residuen dieser Spezifikation

Kein stilles Loch:

- **R1 (PDF-Export).** Als Tertiärformat zurückgestellt; Treue-/`format_loss`-Regeln gelten sinngemäß, sobald aktiviert.
- **R2 (Export-Treue je Format).** Die genauen Grenzen, welche Inhaltsklassen-Elemente ein Format trägt, sind je Format beim Aktivieren zu fixieren; das Prinzip (Treue oder sichtbares `format_loss`) steht fest.
- **R3 (Re-Import-UX).** Die genaue Darstellung der Manipulations-Erkennung ist einem Gestaltungsdurchgang vorbehalten; der Mechanismus (§S7.6) steht fest.
- **R4 (Begleit-Metadaten-Format).** Das Format der mitexportierten Herkunft/Zertifikat ist offen (naheliegend: `PHC-Portable`, Bauverfassung); irrelevant für die Garantie.

---

## S7.11 — Abnahme (DoD dieser Ebene)

```
DoD(S7) = 1  ⟺
    Ein Operator kann jedes geschlossene Artefakt entnehmen: öffnen (domänen-nativ),
        exportieren (mit Herkunft/Zertifikat), kopieren
  ∧ jedes Artefakt trägt die unfälschbare Herkunftskette (Crystal → Lauf → Zertifikat)
        und BEIDE Digests (Byte-Digest = Datei, Inhaltsklassen-Digest = Bedeutung)
  ∧ Replay/Zertifikat binden an den Inhaltsklassen-Digest, nicht an die Bytes
  ∧ Export bewahrt die Inhaltsklasse ODER meldet Formatverlust als sichtbares `format_loss`-Residuum
        (nie stiller Bedeutungsverlust)
  ∧ Re-Import + reanalyze erkennt äußere Bearbeitung nachweisbar (Zertifikat gilt nur für die exakte Klasse)
  ∧ Ablage ist content-adressiert (Byte-Digest), unveränderlich, sync-fähig; Herkunft im Ledger
  ∧ jede Domäne gibt über die IDENTISCHE Ausgabe-Teileliste aus (§S7.8, Adapter-Parität)
  ∧ KEINE Handhabe entfernt Herkunft, fälscht Zertifikate, verliert Bedeutung still
        oder stellt Unzertifiziertes als zertifiziert dar (§S7.9)
  ∧ Engine-DoD, DoD(S3), DoD(S1), DoD(S4), DoD(S6) bleiben unberührt
```

---

## S7.12 — Anschluss

S7 schließt die Ausgabe-Ebene: Das Artefakt ist ein reales, brauchbares Ding, das seine Herkunft und seinen Abschluss mitträgt; Bedeutung und Datei sind sauber getrennt (zwei Digests); Bedeutungsverlust ist nie still; äußere Bearbeitung ist nachweisbar. Zusammen decken S4 (Eingang), S6 (Audit) und S7 (Ausgang) die gesamte sichtbare Reise ab.

**Nächste D-Spec (Systemlandkarte §6):** **S5 — Orchestrierung & Human-in-the-Loop** — Auftragssteuerung, Fortschritt, Pause/Fortsetzen, menschliche Gate-Freigaben in voller Tiefe (in S3.2.2/S3.5 skizziert, hier ausgearbeitet: Lauf-Lebenszyklus, HITL-Entscheidungspunkte, deterministische Wiederaufnahme). Danach S8, S9, S13, S2, S11, S12 — bis `ProduktDoD=1`.

*Ende der Detail-Spezifikation S7.*
