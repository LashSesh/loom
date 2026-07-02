# CCE Loom — Operator-Handbuch

Diese Dokumentation steht unter der Verfassung des Produkts (S13.4):
sie verspricht nur, was das Produkt hält, und sie stimmt mit dem
tatsächlichen Verhalten überein. Wo das Produkt einen Fall ablehnt,
sagt dieses Handbuch, dass und warum.

## 1. Das mentale Modell

Wunsch → Crystal → Lauf → Artefakt. Sie beschreiben, was Sie brauchen;
die KI-Kanzel hilft, daraus einen prüfbaren Arbeitsauftrag (Crystal) zu
formen; der deterministische Motor fährt den Lauf; am Ende steht ein
geschlossenes, geprüftes, wiederholbares Artefakt.

- **Die KI ist Führer und Dolmetscher, niemals Richter.** Jede
  Kanzel-Ausgabe ist als „Interpretation, kein Motor-Urteil" markiert.
- **Residuen sind sichtbare Wahrheit.** Was offen ist, wird angezeigt —
  ein leeres Residuenfeld erscheint ausdrücklich als „geschlossen (∅)",
  es wird nie weggelassen.
- **Replay ist die unabhängige Prüfung.** Jeder Lauf lässt sich aus
  bestätigtem Crystal und Laufbeschreibung identisch wiederholen — die
  Wiederholung erzeugt dieselbe Klasse, nie eine neue KI-Antwort.

## 2. Die Reise, Schritt für Schritt

Öffnen → Wünschen → Bestätigen → Laufen → Prüfen → Entnehmen →
Ablegen/Wiederholen. Jeder Übergang ist eine benannte Naht (0–5), die
ihre Garantie überträgt; es gibt keinen Übergang, an dem eine Garantie
verloren geht. Die Bestätigungsgrenze liegt an Naht 1: davor hilft die
(nichtdeterministische) KI, danach arbeitet ausschließlich der
deterministische Motor.

## 3. Wünsche formulieren

Ein Wunsch ist wohlformbar, wenn er ein Ziel, erkennbare Randbedingungen
und ein Materialisierungsziel trägt („ein zweiseitiges Memo, drei
Projektrisiken, je Gegenmaßnahme, ohne Bewertungszahlen"). Die Kanzel
formt daraus einen Crystal und legt eine Annahmen-Liste vor; jede
Annahme ist als menschen- oder modellgeformt gekennzeichnet. Prüfen Sie
die Annahmen; bindend wird der Crystal erst durch Ihre Bestätigung und
die unabhängige Motor-Validierung.

## 4. Gates und Residuen lesen

Ein Gate-Urteil ist boolesch und begründet: grün oder rot, nie eine
Zahl. Ein Residuum ist ein offener Rest — mit Quelle, Inhalt und
Schweregrad. „geschlossen (∅)" heißt: das Feld wurde geprüft UND ist
leer. „blocking" heißt: dieser Lauf kann nicht abschließen, solange der
Rest besteht. Jede Anzeige lässt sich bis zu ihrem Motor-Artefakt
aufklappen (Wurzel-Rückführung).

## 5. Ermessens-Entscheidungen treffen

Wo der Korpus eine menschliche Entscheidung vorsieht, zeigt das Cockpit
den vollen Kontext (Residuum, Gegenhorizont, Nullmodelle). Ihre
Entscheidung wird aufgezeichnet und in den Ledger geschrieben. Harte
Gates sind nicht übersteuerbar: es gibt keinen
„Trotzdem durchlassen"-Knopf — rot heißt rot.

## 6. Artefakte entnehmen und verwenden

Beim Entnehmen erhält das Artefakt zwei Digests: einen für die Datei
(die konkreten Bytes) und einen für die Bedeutung (die kanonische
Klasse des Inhalts). Das Zertifikat bindet beide. Äußere Bearbeitung
der Datei bricht das Zertifikat nachweisbar: beim Re-Import wird die
Abweichung mit beiden Klassen benannt. Ein verlustiger Export (reiner
Klartext ohne Struktur-Anker) ist möglich, aber nur als bewusste
Entscheidung mit sichtbarem format_loss-Hinweis.

## 7. Replay: selbst nachprüfen

„Identisch wiederholen" reproduziert den Lauf aus bestätigtem Crystal
und Laufbeschreibung. Dass dieselbe Klasse herauskommt, beweist das
Ergebnis unabhängig — die Wiederholung fragt niemals ein Modell neu.

## 8. Wenn etwas nicht schließt

Ein benannter Nicht-Abschluss ist ein gültiges, ehrliches Ergebnis:
„ABGELEHNT" trägt den Grund, das geführte Residuum und bleibt selbst
wiederholbar. Kein Scheitern wird versteckt. Der Weg weiter ist ein
neuer oder geänderter Wunsch — nie das Übergehen eines Gates.

## 9. Bibliothek und Vorlagen

Die Saat-Bibliothek liegt der Installation bei (u. a. der
Drei-Risiken-Memo-Arbeitskörper als .loom-Datei). Der erste geführte
Lauf nutzt dieses Beispiel: Sie sehen schon im ersten Durchlauf ein
Residuenfeld „geschlossen (∅)", einen Gate-Report und den
Abschlussbeweis.

## 10. Einstellungen

Workspace-Ablage (lokal, content-adressiert, sync-fähig ausgelegt),
KI-Zugang (Provider-Deklaration ist nie Aktivierung; Aktivierung ist
eine bestätigungspflichtige Handlung und wird verbucht), Export-Ziele.
Ohne konfigurierten KI-Anbieter läuft das Produkt vollständig — die
Kanzel zeigt sich dann sichtbar degradiert.

## Was diese Doku bewusst nicht tut

Sie behauptet nichts jenseits der geprüften Reichweite des Korpus —
keine Aussagen über Physik oder offene mathematische Vermutungen als
Betriebsversprechen, keine Zahlen als Urteile, keine Funktionsreife
ohne Kennzeichnung.
