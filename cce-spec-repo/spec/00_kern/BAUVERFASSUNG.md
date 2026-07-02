# BAUVERFASSUNG — Domänenagnostische LOOM/PHC/Crystal/Hypercube-Engine

**Kanonischer Titel der Engine:** Crystalline Closure Engine (CCE)
**Status:** Geschlossene Bauverfassung v1.0 — vollständig, nicht prototypisch, nicht minimal.
**Grundlage:** Vollständige Ausführung der MASTER-MATRIX (der Master-Normalform des Korpus) über den gesamten hochgeladenen Korpus.
**Zweck:** Ein Folgeagent (Coding-Agent) kann aus diesem Dokument direkt ein einziges zusammenhängendes Repository aus einem Guss planen und bauen.

> **Identifikation der MASTER-MATRIX.** Der Korpus hinterlegt seine Ausführungsmatrix an zwei Stellen deckungsgleich: als *Minimaler Leitvertrag* (Korpus-Metaarchitektur) und als *Master-Normalform* (MERKABA). Beide lauten inhaltlich identisch:
>
> **Beobachten → Kristallisieren → Kodieren → Projizieren → Materialisieren → Replay → Re-Analyse.**
>
> Diese Bauverfassung führt genau diese Matrix vollständig über alle 13 Korpusressourcen aus und schließt sie zu einer einzigen Architektur, die auf die unten stehende Abschlussformel hin konstruiert ist.

---

## TEIL 0 — DIE ABSCHLUSSFORMEL ALS BAUACHSE

Alle Abschnitte dieser Verfassung sind auf **eine** Formel hin konstruiert. Sie ist die Fitness-Funktion des gesamten Baus, das einzige globale Abnahmekriterium und die Definition von „fertig".

```
Reanalyze( Materialize( LOOM( Project( PHC( Crystal ) ) ) ) )  ≃  Crystal
```

### 0.1 Operatorweise Dekodierung (verbindlich)

Jeder Name der Formel ist ein Korpusoperator mit fester Signatur. Nichts ist Metapher.

| Formelterm | Korpusoperator | Eingang → Ausgang | Herkunft |
|---|---|---|---|
| `Crystal` | Der closure-zertifizierte Kern `C` | — | CCC (Kristall als Token), BCIK (`Fix(T) ∩ Int(H⁻_Σ) ∩ Closed ∩ Pass(G) ∩ τ ∩ Replay ∩ PathInv`) |
| `PHC(·)` | `Encode` | `Crystal → PHC-Paket P` | Projektiver Hypercube-Codec (`PHC = (M,T,A,C,W,P,G,R,L,E)`) |
| `Project(·)` | `Decode ∘ Project` | `P → {lokale Workcell-Projektionen πc}` | Constraint Lattice (Chameleon-Projektion), PHC (Projektionskalkül) |
| `LOOM(·)` | `Weave ∘ Execute ∘ Gate ∘ Commit` (Distribute-Sweep) | `{πc} → committed Weave-Lauf` | LOOM (generative Inversion, Radialspindel) |
| `Materialize(·)` | `Emit` | `Weave-Lauf → Artefakt A` | LOOM Artefaktklassen, PHC Materialisierungsprofile, MERKABA Trumpet |
| `Reanalyze(·)` | `Obs` = **Collect-Sweep** | `A → Obs(A)` | Panoptikum + Instrumentenoptik + TAT + QLOGIC + DZ |
| `≃` | Gleichheit **modulo deklariertem Quotienten** `q` auf Kristallklassen | — | CCC (Quotientenabstieg), PHC (`q(Obs(A)) = q(C_P)`), CL (Replay-Identität) |

### 0.2 Der Abschluss ist der Collect∘Distribute-Rundlauf

Die Abschlussformel ist **nicht** ein neuer Mechanismus. Sie ist der geschlossene Rundlauf der zwei Sweeps des Crystalline Closure Calculus:

```
Crystal ──[ Distribute-Sweep ]──▶ Artefakt A ──[ Collect-Sweep = Reanalyze ]──▶ Obs(A) ≃ Crystal
          Project · LOOM · Materialize            Reanalyze (Obs)
          (zentrifugale Auswebung, → außen)        (zentripetale Sedimentation, → innen)
```

- **Distribute** (`Project ∘ LOOM ∘ Materialize`) ist die abwickelnde Spule (CCC §16, LOOM §7): der Kristall wird als Weave, Workcell-Sequenz und Artefakt ausgespannt.
- **Reanalyze** (`Obs`) ist die aufwickelnde Spule (CCC §15, LOOM §6): das erzeugte Artefakt sedimentiert erneut zum Kristall.
- Gekoppelt sind beide über **denselben Nullanker** `Z0` und dieselbe Kanonisierung `Can` / Signatur `σ`. Der Nullpunkt wird nie traversiert; die Inversion wird durch **Boundary, Mandorla-Seam, Residuum, Replay** regularisiert (LOOM §4, PHC §9.2, BCIK QSNA).

### 0.3 Das Abschluss-Theorem (globales Abnahmekriterium)

Sei `C` ein Root-Crystal, `P = Encode(C)`, `A = Emit(LOOM(Project(P)))`. Die Engine ist **geschlossen bezüglich `C`** genau dann, wenn:

```
ClosedCCE(C) = 1
  ⟺  Can(P) ist stabil (Root-Hash stimmt)                         [PHC V2]
  ∧  Pass(G_all)(A) = 1        (alle Pflichtgates fail-closed)     [PHC §12.2, BCIK §7]
  ∧  Replay(A) = 1             (byte-identisch mod Can)            [CL Satz 10.6, CCC Satz 19.5]
  ∧  Visible(Res(A)) = 1       (Residuum sichtbar, nicht absorbiert)[BCIK A4, CL K12]
  ∧  BCIK(A) = 1               (Integritätskern erfüllt)          [BCIK §9]
  ∧  q(Obs(A)) = q(C)          (Reanalyse fällt auf Kristallklasse)[PHC §16, LOOM §7]
```

Dies ist zugleich die Konjunktion der drei Korpus-Akzeptanzformeln, die deckungsgleich sind:

- LOOM: `AcceptLOOM(C,A) = 1 ⟺ BCIK(A) ∧ Obs(A) ≃ C ∧ Res(A) sichtbar`
- PHC: `ClosedPHC(P,A) = 1 ⟺ Can(P) ∧ Gate(A) ∧ Replay(A) ∧ Visible(Res(A)) ∧ q(Obs(A)) = q(C_P)`
- CCC: `Kristall ⟺ Closed ∧ QSR-Stabil ∧ Gate ∧ Replay ∧ ResidueVisible`

**Konsequenz für den Bau:** Jedes Modul, jedes Schema, jeder Test und jedes Gate in dieser Verfassung existiert nur, um `ClosedCCE(C) = 1` überprüfbar, reproduzierbar und domänenagnostisch herstellbar zu machen. Ein Modul, das nicht an diese Formel anschließt, ist **nicht** Teil der Engine.

### 0.4 Verbotsachse (aus dem Korpus, hart)

Diese Verbote gelten global und werden in Teil 8 als Abnahmebedingungen operationalisiert:

1. Keine Metrik/kein Score ersetzt jemals ein Gate (CL §7.2, BCIK §26, PHC §12).
2. Kein Residuum verschwindet ohne Report (BCIK A4, PHC `SilentResidueAbsorption`).
3. Kein stiller Durchgang durch den singulären Nullpunkt; jede Nullnähe erzeugt Boundary/Residuum/Trace (LOOM §5, PHC §9.2, BCIK QSNA).
4. Keine Pseudo-Bijektion: Gleichheit gilt nur relativ zu `Can`/Quotient, nie auf Rohdaten (LOOM §4).
5. Keine Prompt-Regression: eine Workcell wird nie als Textprompt zurückgegeben (LOOM §18).
6. Kein Repo/keine Genealogie als Ganzes importiert — nur Rollen, Invarianten, Verträge, Gates, Datenmodelle (Metaarchitektur §7).
7. Keine Reduktion auf Software: Software ist ein Domänenspezialfall `Dom = software` unter vielen (Metaarchitektur §6).
8. Keine Claim-Überdehnung: keine Physik, keine RH, keine Hilbert-Pólya-Struktur, `π`/`ζ` sind keine Operatoren (BCIK A8).


---

## TEIL 1 — VOLLSTÄNDIGES BAUSTEINREGISTER

Das Register führt jeden Baustein der Engine mit **Typ**, **Rolle**, **Herkunft** und **Anschluss an die Abschlussformel**. Es ist erschöpfend: alle Objekte, Operatoren, Gates, Invarianten, Zeitrollen und Zertifikate des Korpus sind erfasst. Bausteine sind in sechs Familien gruppiert.

### 1.1 Fundamentbausteine (Grundstock: BCIK · Constraint Lattice · CCC)

| ID | Baustein | Typ | Rolle | Herkunft | Formelanschluss |
|---|---|---|---|---|---|
| F-01 | `Can` Kanonisierung | idempotenter, deterministischer Operator | Identität aller Klassen; `Can∘Can=Can` | BCIK A1, CCC §1, CL §7.3 | Bindet `PHC`, `≃`, `Replay` |
| F-02 | `σ` Signatur | Abbildung, `σ=σ∘Can`, auf Träger vollständig | Content-Addressing, Kollisionsfreiheit | CCC §1, CL §15, BCIK A2 | Kristallidentität in `≃` |
| F-03 | `R` Reflexion / Involution | `R=R*, R²=I` | erzeugt akzeptierten Pol / Residualpol | CCC §2, BCIK §5, MERKABA R | Basis von `Closed` |
| F-04 | `B, B⁻` Randprojektoren | `B=½(I+R), B⁻=½(I−R)` | `B` akzeptiert, `B⁻` Residualpol | CCC §2, BCIK §8, CL §7.4 | `Closed ⟺ B⁻x=0` |
| F-05 | Tripolare Faser `Γ(x)=(p⁺,p⁻,m)` | Tupel je Skelettknoten | lokale Kern-Faser, `m=Ξ(p⁺,p⁻)` Naht | CCC §3, MERKABA §104 | lokaler `Closed`-Anteil |
| F-06 | Closure `B⁻x=0` | Prädikat | Zentralgesetz | CCC §6, BCIK A4 | Kern von `ClosedCCE` |
| F-07 | Constraint Cube `𝒞` (Grade G0–G3) | typisierter Produktraum | globaler Möglichkeitsraum | CL §3 | Trägt `Project` |
| F-08 | Propagationshülle `Γ_𝒦` | extensiv/monoton/idempotent | Kollaps zum kleinsten Fixpunkt | CL §3.2, CCC §15, BCIK §33 | Collect-Kern |
| F-09 | Feasible Set `ℱ` / Kandidaten `Cand_i` | Down-Set / monoton schrumpfend | zulässige Komplettierungen | CL §2, §3 | Collect-Kontraktion |
| F-10 | Entfaltungsgraph `U` (DAG) = Junction Tree | endlicher DAG / Cliquenbaum mit RIP | azyklische Materialisierungsordnung | CL §4, CCC §8 | Reihenfolge in `LOOM` |
| F-11 | Separator `S_ij = C_i ∩ C_j` = Naht | Kante des Junction Tree | Kopplungsdatum; = Mandorla-Seam | CCC §8, §11 | Kristall-Übergabe |
| F-12 | Chordalität / PEO | Grapheigenschaft / Eliminationsordnung | = Adäquatheit; lokal⇒global | CCC §7, §9, CL §5.2 | garantiert `q(Obs)=q(C)` |
| F-13 | Boundary Contract | (interface, assump, oblig, verif, expiry) | kontrollierte offene Abhängigkeit | CL §4.3, CCC §10 | Fill-in / SCC-Kappung |
| F-14 | Collapse-Zertifikat | `(Γ,H,Vol,Gate,Ev,ReplayHash,mode)` | Nachweis akzeptierten Kollapszustands | CL §7.4 | Beleg für Collect |
| F-15 | Baumweite `tw(G)` | Kostenmaß | struktureller Exponent (`d ∼ tw+1`) | CCC §10, CL §10.3 | Kosten von `Project`/`LOOM` |
| F-16 | Nicht-Erreichbarkeit (Grenze) | Satz | wo endliche Sweeps `Fix(R)` nicht erreichen | CCC §6.5, §10.3, BCIK §35 | definiert Nicht-Closure |

### 1.2 Integritäts- und Kernbausteine (BCIK)

| ID | Baustein | Typ | Rolle | Herkunft | Formelanschluss |
|---|---|---|---|---|---|
| I-01 | `BCIK(K)` Zentralgesetz | Prädikat: `Fix(T)∩Int(H⁻_Σ)∩Closed∩Pass(G)∩τ∩Replay∩PathInv` | härtester Kern; Materialisierungsschranke | BCIK §9 | Konjunkt in `ClosedCCE` |
| I-02 | Getypter Kern `K` | `(X,Σ,T,Can,R,B,B⁻,G,τ,ρ,Π,Replay,PathInv)` | Objekttyp jedes Kristallkandidaten | BCIK §5 | Objekt von `Materialize` |
| I-03 | Umgestülpte Boundary `Int(H⁻_Σ)` | abgeschlossener Horizont, Inneres ≠ ∅ | operative Boundary | BCIK §5.2 | Gate `G_boundary` |
| I-04 | Gate-Familie `G` | `G_typed∧G_boundary∧G_closure∧G_residue∧G_evidence∧G_trace∧G_replay∧G_path∧G_scope` | fail-closed Konjunktion | BCIK §7 | `Pass(G_all)` |
| I-05 | Residualoperator `ρ=B⁻` | Pflichtdiagnostik | sichtbares Residuum | BCIK §8 | `Visible(Res(A))` |
| I-06 | Trace `τ` / Replay | Ableitungspfad / determ. Re-Run | Auditierbarkeit | BCIK A6 | `Replay(A)` |
| I-07 | Pfadinvarianz `PathInv` | `γ1∼γ2 ⇒ Can(Kγ1)=Can(Kγ2)` | ordnungsunabhängige Reduktion | BCIK A7, §37 | Robustheit von `≃` |
| I-08 | H-/F-Instanz + Import-Gate | zwei Kategorien + Reflektor `L` | gehärtet vs. Forschung | BCIK §10, §30 | Admission-Controller |
| I-09 | QSNA | singularitätsähnlich, `Sing=0`, gate/trace/replay-erhaltend | ersetzt echte Singularitäten | BCIK §15 | regularisierter Nullkern |
| I-10 | Peristaltischer Zyklus | `K→ExCal→G→Sed→Assim→Renew` | Wachstum erfordert Residuum | BCIK §16 | Collect-Sedimentation |
| I-11 | BCIK-Maschine `(x,Σ,ℱ,ℓ,g)` + Regeln | Kleinschritt-Semantik (Canon/ExCal/Seam/Sediment/Assim/Commit/Hold) | Ausführungsmodell | BCIK §36 | Laufzeitkern |
| I-12 | Subject Reduction / Progress | Metatheoreme | „well-typed kernels don't go wrong" | BCIK §39 | Sicherheit von `Commit` |
| I-13 | Verfeinerung `⊑` + VC1–VC10 | Vorwärtssimulation + Beweisverpflichtungen | Spec→Software erhält Closure | BCIK §40, §46 | Bauvertrag (Teil 8) |
| I-14 | Monodromie-Ratchet `(θ,g)` | gerichtete Akkumulation | zyklische Wiederkehr ohne Rücklauf | BCIK §14 | Replay-/Filtrations-Index |

### 1.3 Generative Bausteine (LOOM · PHC · MERKABA)

| ID | Baustein | Typ | Rolle | Herkunft | Formelanschluss |
|---|---|---|---|---|---|
| G-01 | Dyadische Radialspindel | 4π-Orientierungsapparat, 2 Dyadenringe, Nullanker `Z0` | Bewegungsform der generativen Inversion | LOOM §3 | trägt Distribute |
| G-02 | Nullanker `Z0` / Mandorla-Kern `M` | prätemporaler Ordnungsanker / Umschlagzone | koppelt Collect↔Distribute orientierungsinvers | LOOM §5 | Regularisierung von `≃` |
| G-03 | Collect (zentripetal) | Apertur→Projektion→Triangulation→Sediment→Gate→Crystal | Analyse-Sweep | LOOM §6 | = `Reanalyze`-Baustein |
| G-04 | Distribute (zentrifugal) | Read→Weave→Project→Execute→Gate→Commit→Reanalyze | Generierungs-Sweep | LOOM §7 | = `LOOM(·)` |
| G-05 | Weave `W` | gerichtete, teilgeordnete Workcell-Sequenz | Strickkörper | LOOM §9, PHC §3.4 | Kern von `LOOM` |
| G-06 | Workcell `ω=(c,πc,Kc,Oc,Gc,Rc,Qc)` | kleinste agentisch ausführbare Einheit | gekapselte Projektion | PHC §3.3, LOOM §13 | Einheit von `Project` |
| G-07 | Nadelapertur (Radfenster) `Needle=(a,n,p,g)` | bidirektionaler Projektions-/Fadenpunkt | Apertur (Analyse) / Nadel (Generierung) | LOOM §8, PIO Radfenster | koppelt Obs↔LOOM |
| G-08 | Hypercube-Strickung `W_{n,k}=⊔ W_{n+1,2k+ε}` | dyadische Zellteilung | Verfeinerung bis lokal sicher | LOOM §9, DZ §2 | Granularität von `Project` |
| G-09 | PHC-Paket `PHC=(M,T,A,C,W,P,G,R,L,E)` | portable Arbeitsraumdatei | Transportkörper des Kristalls | PHC §1 | = `PHC(·)` |
| G-10 | PHC-Manifest `M` + Kanonisierung | (version, codec_id, status, domain_mode, root_crystal, canon) | Identität/Version/Root-Hash | PHC §5, §6 | `Can(P)` |
| G-11 | Achsen `A` (semantic/topological/temporal/control/evidence/material) | Freiheitsgrade | Achsenmodell | PHC §7 | Cube-Aufbau |
| G-12 | Zelle `cell(a1=v1,…)` + Zelladresse | adressierte Schnittmenge, `phc://…` | zelladressiertes Substrat | PHC §8 | adressiert `Project` |
| G-13 | Seam (Boundary-Equalizer) | deklarierte Kopplung, `residue_policy` | Mandorla-Naht im Codec | PHC §9 | regularisiert Nullkern |
| G-14 | Projektionsprofil `πc:H→Hc` | kontrollierte Reduktion + `max_tokens`, `allowed_ops` | No-Horizon-Leakage | PHC §10, CL §5.1 | Kern von `Project` |
| G-15 | Gate-Matrix `G` (scope/boundary/type/residue/replay/export/reanalysis) | fail-closed Prädikate | Pflichtgates | PHC §12.2 | `Pass(G_all)` |
| G-16 | Residuum `R` + Counter-Horizon `CH` | sichtbarer Rest / Nullmodelle/Gegendeutungen | Integrität, verhindert Selbstbestätigung | PHC §13 | `Visible(Res)` |
| G-17 | Ledger `L` (append-only) + RunDescriptor | Trace aller Events / Replay-Parameter | Auditbasis | PHC §14 | `Replay(A)` |
| G-18 | Export/Materialisierungsprofil `E` | (kind, format, entry_weave, gate_chain, reobserve) | Artefaktregeln | PHC §15 | = `Materialize(·)` |
| G-19 | Reanalyse/Closure `q(Obs(A))=q(C_P)` | Quotientenvergleich | Rückfall auf Kristallklasse | PHC §16 | = `≃` |
| G-20 | PHC-Profile CORE/LOOM/MERKABA | Konformitätsprofile | Etablierungsstufen | PHC §26–28 | Konformität |
| G-21 | MERKABA `M=(Z,S,X,H,Γ,R,B,B⁻,Ω,P,Θ,Q,I,Δ,G,C,τ,ρ,L,V)` | monolithischer Operatorapparat | Gesamtumlauf-Closure | MERKABA §1 | orchestriert alles |
| G-22 | Gesamtgleichung `F=Commit∘C∘G∘Δ∘I∘Q∘Θ∘P∘Can` | Operatorkomposition | ein Lauf `E=F(x)` | MERKABA §96 | monolithischer Distribute |
| G-23 | Organ `O=(X_O,Y_O,α_O,β_O,σ_O,γ_O)` + organische Komposition | getypter Transformator | Modul = Organ | MERKABA §100 | Modul-Vertrag |
| G-24 | 12+1-Struktur, 4π/720-Closure, `Z4×Z3≅Z12` | Fenster-/Limiter-Modell | chromatische Emergenz | MERKABA, DZ §1 | Orientierungsabschluss |

### 1.4 Beobachtende Bausteine (Collect / Reanalyze: DZ · QLOGIC · TAT · TP · PIO)

| ID | Baustein | Typ | Rolle | Herkunft | Formelanschluss |
|---|---|---|---|---|---|
| O-01 | Dyadisches Zellsubstrat `D=(X,p,µ,D,F,A,ch,λ,Θ,Θb,C,K,N,U,Π,Γ,Σsig,L,∂,B,R,Q,T,ρ)` | messbare Zellbasis | Untergrund aller Beobachtung | DZ §1 | diskretisiert `Obs` |
| O-02 | Dyadische Zelle `W_{n,k}`, `ch_ε(n,k)=(n+1,2k+ε)`, Filtration `F_n` | adressierbare Zelle + Verfeinerung | Messauflösung | DZ §2 | Zellgranularität |
| O-03 | Zellzustand `λ_n(W)=(m,ψ,ρ,ω,r,τ,g)` | lokales Gewichts-/Spektralfeld | Zellmessung | DZ §3 | Marker-Träger |
| O-04 | QLOGIC `(X,Can,HQ,B,Ω,SG,Ξ,PoR,ΦU,ΦV,C,M,T,ρ,IR,Sup)` | Spektral-Operatorlogik | Resonanz/Gate/Kondensation | QLOGIC §1 | Spektralseite von `Obs` |
| O-05 | Spektralregister `(R,F,T,S,E)` + Q-Zustand `q=(s,ϕ,κ,r)` | Matrix / Vierer | spektrale Belegung | QLOGIC §4, §5 | Signatur der Beobachtung |
| O-06 | Proof-of-Resonance `PoR` + Doppelkick `ΦU,ΦV` | Gate-Familie + orthogonale Kicks | „Gate vor Emission" | QLOGIC §1 | Gate im Collect |
| O-07 | TAT-System `T=(X,Z0,M,R,Θ,W,C,K,Π,Γ,∂,Can,σ,L)` | Horizontkalkül | Attraktorkarte | TAT §2 | Kern von `Reanalyze` |
| O-08 | Marker `m` / Response-Schnitt / Attractor-Kandidat | typisierter partieller Beobachtungsoperator | Marker-Response-Triangulation | TAT §2 | Triangulation |
| O-09 | Horizont / Gegenhorizont | orientierte Regionen | sichtbar vs. blockiert | TAT §4, TP §9 | Counter-Horizon |
| O-10 | MatrixCrystal `=(A,H,Tri,Gates,Residues,NullModels,Trace,ReplayHash)` | geschlossene Horizontkarte | Ausgabe des Collect | TAT §10 | = `Obs(A)`-Kristall |
| O-11 | `TAT = MatrixCrystal∘Gate∘Triangulate∘Horizon∘Respond∘Mark∘Embed` | Funktionskette | Collect-Normalform | TAT §1 | = `Reanalyze`-Kette |
| O-12 | Topologisches Panoptikum `P=(NC,X,Θ,Θb,Γ,W,H,Π,Tri,Lµ,M,Σ,G,B,T,ρ,C)` | Beobachtungsinstrument | Horizontlandschaft, nicht-sing. Kondensation | TP §1 | Sichtapparat von `Obs` |
| O-13 | NullCenter `NC` + Nullprojektion + Hypertorus + 720-Closure | zustandsloser Referenzpunkt | prätemporaler Anker | TP §4, §5 | = `Z0` im Collect |
| O-14 | PhaseSpaceWindow `W` + PhaseRays + Möbius-tripolarer Lift `Lµ` | phasische Trägerstrahlen + Mikro-Lift | orientierte Fensterung | TP §7, §8 | Fenster = Radfenster |
| O-15 | Instrumentenoptik `I=(X,D,Apt,Lens,Split,Track,Dump,Wave,Pulse,Gate,Trace,Replay)` | kybernetische Optik | Sicht-/Durchlassschicht | PIO §1 | Aperturschicht von `Obs` |
| O-16 | Optische Normalform `X0→Apt→Lens→Split→Track→Dump→Wave→Pulse→Gate→XC` | zyklische Operatorkette | Beobachtungspfad | PIO §2 | Collect-Pipeline |
| O-17 | Panoptische Binnenpupille + Radfenster (bidirektional) | sphärische 720-Aperturmonade | Apertur (in) / Nadel (out) | PIO §1, LOOM §21 | koppelt Obs↔LOOM |
| O-18 | Axiome: Apertur vor Fokus · Überschuss sichtbar · Nachführung erhält Fixpunkt | Beobachtungsaxiome | Rohmasse gebremst, Residuum geführt, Drift ≠ Ankerverschiebung | PIO §2 | Gate/Residuum/Track |

### 1.5 Diagnostische Bausteine (Omnipolar-Neutralisierung)

Omnipolar wird **nicht** als Modul importiert, sondern als **Extraktionsschicht** (Metaarchitektur §7, Omnipolar-Diagnostik). Import erfolgt nur als Rollen:

| ID | Extrahierte Rolle | Ziel im Bau | Herkunft |
|---|---|---|---|
| D-01 | Attraktortriangulation | → TAT-Modul (`cce-observe`) | Omnipolar-Normalform |
| D-02 | Wish-to-Diamond / Zielattraktor | → normalisierte Wunsch-Form `W=(X,H,K,G,Res,Π,τ,Replay,Goal,Materialize)` | Metaarchitektur §6 |
| D-03 | Nullmodell-Gates / Falsifikation | → Counter-Horizon + `diagnostics{nullmodels,falsification,proof_horizon}` | PHC §29.3 |
| D-04 | Sichtbare Residuen / keine stille Löschung | → globaler Residuenvertrag | BCIK A4 |
| D-05 | Proof-Horizon | → optionale Horizon-Schicht im PHC | PHC §29.3 |
| D-06 | MatrixCrystal-Ausgang | → `MatrixCrystal` (O-10) | TAT §10 |
| D-07 | bijektive Kanäle / inverse Weave | → LOOM Nadelapertur-Bidirektionalität | LOOM §8 |

**Ausdrücklich NICHT importiert** (bleiben im F-Raum): `SAT/AWDC/BOL/BCL/Diamond` als finale Namen, graph-only-Beschränkung, theorem-mining als einziges Ziel.

### 1.6 Genealogische Vorläufer (nur Rollenimport)

Babylon Compiler und Barbara sind **nicht** hochgeladene Eigenwerke, aber vom Korpus als technische Vorläufer benannt. Import ausschließlich als Rolle/Vertrag:

| ID | Vorläufer | Importierte Rolle | NICHT importiert |
|---|---|---|---|
| P-01 | Babylon | deterministische Pipeline `Input→Can→Embed/Index→Crystal→Gate→Ledger→Emit`; Crystal-Bundle; Evidence | Frontend-Namen, Compilerbindung als Paradigma |
| P-02 | Barbara | Forward/Inverse-Dualität `Observation→Topologie`, `Constraints→Materialisierung` | PSE-Abhängigkeit, Sprache als einzige Generierungsdomäne |


---

## TEIL 2 — ROLLENMATRIX ALLER DOKUMENTE

Jede hochgeladene Korpusressource erhält eine eindeutige Architekturrolle, eine Schicht-Zuordnung (Grundstock / beobachtender Halbraum / generativer Halbraum / monolithische Closure), einen Import-Kern und einen F-Rest (was **nicht** kanonisch übernommen wird). Die Struktur ist nicht linear: **ein Grundstock, zwei Halbräume, ein Monolithkörper** (Metaarchitektur §3).

### 2.1 Schichtenkarte (verbindliche Topologie)

```
                         ┌─────────────────────────────────────────────┐
                         │   MERKABA — monolithische Closure & Synthese │   (Monolithkörper)
                         └───────────────▲──────────────▲──────────────┘
                                         │              │
        BEOBACHTENDER HALBRAUM (Collect) │              │ GENERATIVER HALBRAUM (Distribute)
        kondensiert  → Crystal           │              │ expandiert Crystal → Artefakt
        ┌────────────────────────────────┴──┐        ┌──┴─────────────────────────────────┐
        │ TP · PIO · TAT · QLOGIC · DZ       │        │ (Babylon · Barbara) · LOOM · PHC   │
        └────────────────────────────────┬──┘        └──┬─────────────────────────────────┘
                                         │              │
                         ┌───────────────┴──────────────┴──────────────┐
                         │  GRUNDSTOCK                                  │
                         │  BCIK (Integrität/Gate/Residuum/Replay)      │
                         │  Constraint Lattice (Hypercube-Substrat)     │
                         │  CCC (chordales Skelett, Faser, 2 Sweeps)    │
                         └──────────────────────────────────────────────┘
                Omnipolar-Diagnostik = Extraktionsschicht (kein Modul), zwischen TAT/LOOM/Proof-Horizon
```

### 2.2 Vollständige Rollenmatrix

| # | Dokument | Schicht | Kanonische Rolle | Import-Kern (übernommen) | F-Rest (nicht kanonisch) | Ziel-Modul(e) |
|---|---|---|---|---|---|---|
| 1 | **Korpus-Metaarchitektur v0.2** | Meta | Geländerkarte, Rollenordnung, LOOM-Korrektur | Halbraum-Topologie, Antichimären-Importregeln, Minimaler Leitvertrag, normalisierte Wunsch-Form | — (ist selbst die Karte) | Governance / `spec/` / Teil 8 |
| 2 | **Omnipolar-Diagnostik v0.1** | Diagnostik | Neutralisierung des SAT-MASTER-Harness → Extraktionsschicht | Attraktortriangulation, Nullmodell-Gates, Proof-Horizon, sichtbare Residuen, MatrixCrystal, inverse Weave, bijektive Kanäle | SAT/AWDC/BOL/BCL/Diamond als Namen, graph-only, theorem-mining-Monokultur | verteilt in `cce-observe`, `cce-phc.diagnostics` |
| 3 | **LOOM Generative Inversion v0.2** | Generativ | Semantik der generativen Inversion (Distribute-Sweep) | Radialspindel, Nullanker/Mandorla, Collect/Distribute, Nadelaperturen, Weave, Workcell-DAG, 10-Punkte-Vertrag, Akzeptanzformel, Pseudocode | physikalische Bilder (Gyrosphäre, Wurmloch, Kugelwelle) | `cce-loom` |
| 4 | **Projektiver Hypercube-Codec v0.1** | Generativ | Transportkörper / Dateiformat (Encode/Decode/Project) | `PHC=(M,T,A,C,W,P,G,R,L,E)`, Kanonisierung, Zelladressen, Projektionskalkül, Gate-Matrix, Residuen/Counter-Horizon, Ledger/Replay, Materialisierungsprofile, Loader-Algorithmus, Validierungsphasen V0–V9, Profile CORE/LOOM/MERKABA | — (ist selbst normativ) | `cce-phc`, `schemas/` |
| 5 | **MERKABA v1.4** | Monolith | Monolithische Operatorarchitektur; Gesamtumlauf-Closure & Emission | `M=(Z,…,V)`, Gesamtgleichung `F`, Organ-Kalkül, organische Komposition, 12+1/4π/720-Struktur, Master-Normalform, Emissionsklasse | esoterische Herkunftsnamen als Semantik (Gabriel/Ophanim/… bleiben Rollenlabels) | `cce-merkaba` |
| 6 | **Boundary-Closed Invariant Kernels** | Grundstock | Integritätskern + Realisierungsbrücken (Kern→Software) | `BCIK`-Zentralgesetz, getypter Kern, Gate-Familien, Residuen-/Replay-Vertrag, H/F+Import-Gate, BCIK-Maschine, Erhaltung/Fortschritt, Verfeinerung, VC1–VC10, Anschlusslandkarte | RH/Hilbert-Pólya/Physik/`π,ζ`-als-Operator (nur F/D3), Zeta-Vollständigkeit | `cce-core`, Teil 8 (VC), gesamte Test-Schicht |
| 7 | **Constraint Lattice** | Grundstock | Hypercube-Substrat + Kollaps + lokale Projektion (Collect-Substrat) | `M=(𝒞,𝒦,𝒩,U,Π,Γ,Σsig,L)`, vier Cube-Grade, Propagationshülle, Entfaltung/Traversierung, Adäquatheit, Gate/Evidence/Replay, Materialisierung, Export-Funktor, K1–K18 Abnahme | — | `cce-lattice` |
| 8 | **Crystalline Closure Calculus** | Grundstock | Vereinheitlichendes Kalkül: Faser + Skelett + zwei Sweeps + Kristall-Token | Kanonisierung/Signatur, Randprojektor, tripolare Faser, Wickel-Transformation, chordales Skelett, Junction Tree/RIP, Verklebung, Zwei-Pass-Schema, Bi-Temporalität, Kristall-Protokoll, C1–C14 | Monodromie-Klassifikation, Seam-Konsistenz global (nur offene Probleme) | `cce-ccc` (verbindet alle) |
| 9 | **Dyadisches Zellsubstrat v1.1** | Beobachtend | Messbare Zellbasis (Untergrund aller Beobachtung/Projektion) | `D=(X,…,ρ)`, dyadische Zellen/Filtration, Zellzustand, Hypercube-Constraint-Substrat, chromatische Makrofenster `Z4×Z3≅Z12`, Randprojektor/Closure, 12+1-Kompatibilität | — | `cce-observe.substrate`, teilt Cube mit `cce-lattice` |
| 10 | **Topologisches Panoptikum v1.0** | Beobachtend | Beobachtungsinstrument: Horizontlandschaft, nicht-singuläre Kondensation | `P=(NC,…,C)`, NullCenter/Nullprojektion, Hypertorus/720-Closure, PhaseRays/Windows, Möbius-tripolarer Lift, Mandorla-Seam/Brane, Gate-Boundary-Trace-Replay | — | `cce-observe.panoptikum` |
| 11 | **Panoptische Instrumentenoptik v1.1** | Beobachtend | Sicht-/Durchlassschicht (Aperturkörper, Radfenster) | `I=(X,D,Apt,Lens,Split,Track,Dump,Wave,Pulse,Gate,Trace,Replay)`, optische Normalform, Binnenpupille, Radfenster (bidirektional), Axiome (Apertur vor Fokus u. a.) | optische Metaphorik als Physik | `cce-observe.optics`; Radfenster-Bridge zu `cce-loom` |
| 12 | **Topologische Attraktortriangulation (tat) v1.0** | Beobachtend | Horizontkalkül → Attraktorkarte (`Reanalyze`-Kern) | `T=(X,Z0,…,L)`, Marker/Response/Attractor, Horizont/Gegenhorizont, Triangulation, Constraint-Lattice-Kern, Gate-System, Mandorla-Seam/Pullback, MatrixCrystal, `TAT=MatrixCrystal∘…∘Embed` | — | `cce-observe.tat` (Orchestrator des Collect) |
| 13 | **QLOGIC v1.0** | Beobachtend | Spektral-Operatorlogik (Resonanz/Gate/Kondensation/Selbstspiegel) | `QLOGIC=(X,Can,HQ,B,Ω,SG,Ξ,PoR,ΦU,ΦV,C,M,T,ρ,IR,Sup)`, Hybridraum/Spektralregister, Q-Zustände, PoR-Gate, Doppelkick, Kondensation, „Spektral vor symbolisch", „Gate vor Emission" | — | `cce-observe.qlogic` |

### 2.3 Rollenkonflikte, die die Verfassung auflöst

1. **LOOM ist kein Repo.** LOOM ist die *Rolle* der generativen Inversion. Babylon/Barbara/Omnipolar sind Vorläufer, nicht LOOM (Metaarchitektur §1). → `cce-loom` implementiert die Rolle; Vorläufer liefern nur Verträge.
2. **PHC ≠ LOOM.** LOOM ist der Kalkül, PHC der Transportkörper (PHC §2). → getrennte Crates `cce-loom` / `cce-phc`; PHC-LOOM-0.1-Profil verbindet sie.
3. **MatrixCrystal (TAT) vs. Crystal (CCC/BCIK).** MatrixCrystal ist die *geschlossene Karte eines Collect-Laufs*; ein Crystal ist die *Klasse* `[Can(c)]_σ`. MatrixCrystal ⊂ Crystal-Familie mit Horizontfeldern. → gemeinsamer Typ `Crystal`, Untertyp `MatrixCrystal` (Teil 3).
4. **NullCenter (TP) = Nullanker `Z0` (LOOM/CCC) = Uranker `Π0` (BCIK).** Alle drei: prätemporale Referenz, `A∩X=∅`, nicht traversierbar. → ein Typ `NullAnchor` (Teil 3), boundary-regularisiert.
5. **Radfenster (PIO) = Nadelapertur (LOOM).** Dasselbe lokale Fenster wirkt beobachtend als Apertur, generativ als Nadel. → ein bidirektionaler Typ `WheelWindow` (Teil 3), der Collect und Distribute koppelt — **das ist der mechanische Grund, warum `Obs` und `LOOM` denselben Apparat bilden und `≃` geschlossen ist.**

---

## TEIL 3 — KANONISCHES OBJEKTMODELL

Ein einziges, domänenagnostisches Objektmodell trägt beide Halbräume und den Monolith. Alle Objekte sind getypt, tragen einen Lebenszyklus und schließen an die Abschlussformel an. Notation: Feldlisten sind normativ; `⟂` bezeichnet Pflichtfelder.

### 3.1 Identitäts- und Fundamentobjekte

**`CanonicalState`** — kanonisierter Träger `(X, Can)`, `Can∘Can=Can`.
Felder: `⟂raw`, `⟂canon` (= `Can(raw)`), `⟂signature` (`σ`, auf Träger vollständig), `provenance`.
Lebenszyklus: `raw → canon → signed`. Invariante: `σ = σ∘Can`; Kollisionen werden als Residuum protokolliert (BCIK A2).

**`NullAnchor`** — prätemporale Referenz `Z0`/`NC`/`Π0`.
Felder: `⟂id`, `⟂kind ∈ {reference}`, `space_tag` (`A`, mit `A∩X=∅`).
Invariante: **nicht traversierbar**; jede Nullnähe erzeugt `BoundaryTrace`+`Residue` (LOOM §5, BCIK §15). Kein Systemzustand.

**`TripolarFiber`** — `Γ(x)=(p⁺,p⁻,m)`.
Felder: `⟂accepted` (`p⁺=Bx`), `⟂residual` (`p⁻=B⁻x`), `⟂seam` (`m=Ξ(p⁺,p⁻)`).
Invariante: `B+B⁻=I`, `BB⁻=0`. Closed ⟺ `residual=0` mit ausgewiesenem (dann leerem) Residuum.

**`Reflection`** — `R=R*`, `R²=I`, mit `B=½(I+R)`, `B⁻=½(I−R)`. Trägt akzeptierten Pol / Residualpol.

### 3.2 Substrat- und Skelettobjekte

**`Cube`** (Constraint Lattice + Dyadisches Zellsubstrat, vereinheitlicht).
Felder: `⟂dimensions[]` (`Dimension=(id, Domain∈{Finite|Interval|Approx})`), `⟂couplings[]` (`(Di,Dj,kind)`), `⟂grade ∈ {G0,G1,G2,G3}`, `constraints[]` (`𝒦`), `norms[]` (`𝒩`, mit Fitness/Scope/Provenance), `measure µ`, `dyadic_basis` (Zelladressen `(n,k)`, `ch_ε(n,k)=(n+1,2k+ε)`, Filtration `F_n`).
Grade: G0 blank · G1 +Constraints (`ℱ_𝒦`) · G2 +Norm/Feld/Gradient · G3 +Entfaltung/Projektion/Gate/Ledger (ausführbar).

**`Cell`** — adressierte lokale Schnittmenge von Achsenwerten.
Felder: `⟂id`, `⟂address` (`phc://<codec_id>/cell/<axis=value>/…`), `⟂axis_values{}`, `fiber{claims,constraints,artifacts,residue}`, `boundary{imports,exports,seams}`, `⟂signature`, `status ∈ {empty,open,filled,locked,gated,closed}`.

**`Separator` / `Seam`** — `S_ij = C_i ∩ C_j`, zugleich Mandorla-Naht.
Felder: `⟂id`, `⟂kind` (z. B. `boundary_equalizer`), `⟂cells[]`, `⟂direction ∈ {inbound,outbound,bidirectional}`, `⟂rule` (z. B. `same_crystal_class_under_declared_quotient`), `⟂gate`, `⟂residue_policy=visible`, `replay_anchor`.
Invariante (Verklebung): lokale Closure überträgt sich auf Separator (CCC Lemma 12.2).

**`JunctionTree`** — Cliquenbaum mit Running-Intersection-Property; = Entfaltungsgraph `U`.
Felder: `⟂cliques[]` (maximale Cliquen = Subcubes), `⟂edges[]` (tragen `Separator`), `⟂root`, `treewidth`. Invariante: chordal ⟺ PEO existiert ⟺ Adäquatheit (lokal⇒global).

**`BoundaryContract`** — `(interface, assumptions, obligations, verification, expiry)`. Kontrollierte Fill-in-Kante / SCC-Kappung mit Verfallsdatum.

### 3.3 Beobachtungsobjekte (Collect / Reanalyze)

**`Marker`** — typisierter partieller Beobachtungsoperator (TAT §2, PIO §3).
Felder: `⟂id`, `⟂type`, `⟂query` (gerichtete Frage an `X`), `aperture_ref` (`WheelWindow`), `scope`.

**`Response` / `ResponseCut`** — gemessener/rekonstruierter Schnitt zu `(m, beobachtete Response)`. Felder: `⟂marker`, `⟂cut`, `evidence`, `t` (tri-temporal).

**`Horizon` / `CounterHorizon`** — orientierte Regionen sichtbar/latent/blockiert bzw. Nullmodelle/Gegendeutungen. Verhindert Selbstbestätigung.

**`SpectralRegister`** — `(R,F,T,S,E)=(Relation,Frequenz,Topologie,Symmetrie,Entropie)` + `QState q=(s,ϕ,κ,r)` (QLOGIC). Trägt die spektrale Signatur der Beobachtung („spektral vor symbolisch").

**`PhaseSpaceWindow` / `WheelWindow`** — **der Doppelbaustein**.
Felder: `⟂id`, `⟂cell`, `⟂aperture` (`a: X→[0,1]`, Collect-Rolle), `⟂needle` (`n: C→Thread`, Distribute-Rolle), `⟂projection_profile`, `⟂gate`, `direction ∈ {inbound,outbound,bidirectional}`.
Invariante: beobachtend Apertur, generativ Nadel — **koppelt Obs↔LOOM** (Teil 2.3.5).

### 3.4 Kristall-Objekte (das Übergabe-Token)

**`Crystal`** — closure-zertifizierte **Klasse** `[Can(c)]_σ`, nie Repräsentant.
Felder (⟂ alle): `id` (content-addressed `crystal:<hash>`), `signature σ`, `type_status`, `provenance/observation_ref`, `boundary_state`, `residue_state`, `replay_contract`, `closure_certificate`.
Zentralgesetz: `Crystal ⟺ Closed ∧ QSR-Stabil ∧ Gate ∧ Replay ∧ ResidueVisible` (CCC §18).

**`MatrixCrystal`** ⊂ `Crystal` — geschlossene Horizontkarte eines Collect-Laufs (TAT §10).
Zusatzfelder: `⟂attractor_map A`, `⟂horizons H`, `⟂triangulation Tri`, `⟂gates`, `⟂residues`, `⟂null_models`, `⟂trace`, `⟂replay_hash`.
Dies ist der Typ, den `Reanalyze` erzeugt: `Obs(A) : MatrixCrystal`.

**`Monolith`** — irreversibles, append-only Commit-Ereignis `Append(ℓ_k, π(K_k))` eines Kristalls; = MERKABA-Monolith bei Gesamt-Closure. Unveränderlich, signiert, im Ledger.

### 3.5 Generative Objekte (Distribute)

**`PHCPackage`** — `PHC=(M,T,A,C,W,P,G,R,L,E)` (siehe Teil 7.1 für Schema). Portable Arbeitsraumdatei; Transportkörper des Kristalls.

**`Workcell`** — `ω=(c, πc, Kc, Oc, Gc, Rc, Qc)`; kleinste agentisch ausführbare Einheit.
Felder: `⟂id`, `⟂cell`, `⟂projection`, `⟂intent`, `⟂input_contract{required,forbidden}`, `⟂allowed_operations[]`, `⟂output_contract{type,format,must_emit,must_not_emit}`, `⟂gate_chain[]`, `⟂residue_policy`, `⟂replay`, `status ∈ {pending,projected,running,emitted,gate_failed,residue_open,sealed,closed}`.
Invariante: kein globaler Horizont; nur lokale Projektion (No-Horizon-Leakage).

**`Weave`** — gerichtete, teilgeordnete Workcell-Sequenz `W={τj}` mit Separatoren+Gates; endlicher Replaypfad.

**`Thread`** — Folge von Maschen/Workcells `τ=(z1,…,zk)`.

**`NeedleAperture`** — `Needle=(a,n,p,g)`; = `WheelWindow` in Distribute-Rolle. Führt Arbeitsfaden aus dem Kristall in eine Workcell-Sequenz.

**`RadialSpindle`** — `Rad=(Fin, Fout, Z0, Σ, M, Replay)`; 4π-Orientierungsrahmen. `Distribute ≃ I∘Collect∘I` mit boundary-regularisierter Inversion `I`.

### 3.6 Kontroll-, Beweis- und Ledger-Objekte

**`Gate`** — fail-closed Prädikat, **kein Score**.
Felder: `⟂id`, `⟂kind ∈ {admission,topological,type,integrity,determinism,export,reanalysis}`, `⟂rule`, `⟂failure=fail_closed`. Semantik: `Gate(x)=1` ⟺ alle Pflichtbedingungen; sonst `Hold` (Diagnose, nie `Fire`).

**`GateReport`** — Ergebnis einer Gate-Chain: `{gate_id, result ∈ {pass,hold}, evidence_ref}`.

**`Evidence`** — content-addressed Nachweisobjekt (Gate-Eingang).

**`Residue`** — sichtbarer Rest.
Felder: `⟂id`, `⟂origin`, `⟂kind ∈ {unresolved_constraint,excluded_material,failed_gate,open_question}`, `⟂severity ∈ {info,warning,blocking}`, `⟂content`, `⟂visible_to[]`, `⟂resolution_status ∈ {open,absorbed_by_gate,closed_by_evidence,deferred}`.

**`RunDescriptor`** — `RD=(version, Can, σ, G, tolerances, operators, seed, input, domain, exportprofile)`. Grundlage von Replay; keine Wall-Clock, keine ungeseedete Zufälligkeit.

**`Ledger`** — append-only Folge von Events/Commits (`decode,project,execute,gate,residue,commit,materialize`), Hash-Kette.

**`Certificate`** — `cert(K)=(idCan, σ(K), RD, GateReport, BoundaryReport, ResidualReport, τ_head, Replaymanifest, PathReport)`.

**`Artifact`** — `Artifact(K)=(K, cert(K), hash(Can(K)), ledgerIndex)`; das materialisierte, closure-zertifizierte Ergebnis.

### 3.7 Domänen-Neutralität: die Wunsch-Normalform

Jede Zielkonfiguration ist domänenagnostisch als **normalisierte Wunsch-Form** (Metaarchitektur §6) modelliert:
`W = (X, H, K, G, Res, Π, τ, Replay, Goal, Materialize)`.
Domänen sind Spezialfälle über die Achse `domain_mode ∈ {agnostic, software, math, research, document, evidence, execution, mixed}`. Software ist `Dom=software` — ein Fall unter vielen, **nie** die Voreinstellung.

### 3.8 Objektlebenszyklus-Achse (`axis:closure`)

Ein einziger Ordnungsachsenwert steuert alle Objekte durch die Formel:
```
raw → triangulated → gated → sealed → closed
 └Obs┘      └TAT┘      └Gate┘   └Commit┘  └Reanalyse⟹≃⟹Crystal┘
```
`raw` (Rohfeld) → Collect sedimentiert → `Crystal` (`sealed/closed`) → Encode/Project/LOOM/Materialize → `Artifact` → Reanalyze → `q(Obs(A))=q(C)` schließt den Ring.


---

## TEIL 4 — KANONISCHES OPERATORMODELL

Alle Operatoren sind implementierungsneutrale, deterministische Abbildungen mit fixer Signatur. Sie zerfallen in die zwei Sweeps des CCC plus eine gemeinsame Fundamentalschicht. **Pflicht (aus CL/CCC/BCIK):** `Can, propagate, gate, commit, replay` sind deterministisch, ohne Wall-Clock und ohne ungeseedete Zufälligkeit; `gate` ist fail-closed; jeder `commit` trägt ein Closure-Zertifikat mit sichtbarem Residuum; jede Planänderung ist ein dokumentiertes Delta; die Signatur ist auf dem genutzten Träger vollständig.

### 4.1 Fundamentaloperatoren (an jedem Knoten, beide Sweeps)

```
canonicalize : State                → CanonicalState      // idempotent, deterministisch (Can∘Can=Can)
signature    : CanonicalState       → Sig                 // σ = σ∘Can; vollständig auf Träger
reflect      : State                → (p⁺, p⁻, seam)      // tripolare Faser; B=½(I+R)
closed?      : State                → Bool                 // B⁻x = 0
residue      : State                → Residue              // ρ = B⁻x, stets sichtbar
gate         : (Projection|State, Candidate) → {Fire, Hold}// K ∧ Ev ∧ Topo ∧ Replay; fail-closed
commit       : Crystal              → Ledger               // append-only Monolith am Index T1
replay       : (Ledger, RunDescriptor) → Ledger            // byte-identisch mod Can
```

### 4.2 Skelett-Operatoren (CCC — verbinden lokal zu global)

```
triangulate  : Graph                → ChordalGraph         // Fill-in; minimiere Baumweite
junctionTree : ChordalGraph         → Tree(Clique)         // Running-Intersection-Property
separators   : Tree, Edge           → SeparatorSet         // Naht zwischen Fasern
plan         : (Cube, K, N, Cost)   → UnfoldGraph          // = gewurzelter Junction Tree
```

### 4.3 Collect-Sweep-Operatoren (= `Obs` / `Reanalyze`; Constraint Lattice + beobachtender Halbraum)

Der Collect-Sweep ist die **zentripetale Sedimentation**: Rohfeld → Crystal. Er realisiert `Reanalyze`.

```
embed        : Raw                  → State                // Einbettung in Zustandsraum (DZ)
mark         : (State, MarkerSet)   → ResponseCuts         // Marker-Response-Schnitte (TAT/PIO)
aperture     : (Raw, WheelWindow)   → GatedBeam            // Apertur vor Fokus (PIO Axiom 2.1)
spectralize  : State                → SpectralRegister     // QLOGIC Ω→SG; spektral vor symbolisch
horizon      : ResponseCuts         → (Horizon, CounterHorizon) // TP/TAT
triangulate_attr : (Horizon, CounterHorizon) → AttractorMap    // orthogonale Triangulation
sediment     : (State, AttractorMap)→ AcceptedPole         // schwere Invarianten sinken → B x
project_collect : (State, Cell)     → Projection           // Chameleon-Projektion (CL 5.1)
collect      : RootedTree           → (node → AcceptedPole)// = Constraint-Lattice-Konstruktionslauf
crystallize  : (AcceptedPole, Gate, Trace, Residue) → Crystal // Gate+Trace+Replay+Residuum ⇒ Kristall
observe      : Artifact             → MatrixCrystal        // === Reanalyze; komponiert die obige Kette
```

Collect-Normalform (TAT): `Obs = MatrixCrystal ∘ Gate ∘ Triangulate ∘ Horizon ∘ Respond ∘ Mark ∘ Embed`.
Optische Normalform (PIO): `X0 →Apt→Lens→Split→Track→Dump→Wave→Pulse→Gate→ XC`.
Monotone Kontraktion: `F_{t+1} ⊆ F_t` (CL Satz 10.1) — ein Kristall entsteht, weil der akzeptierte Anteil geschlossen/tracebar/replayfähig ist, nicht weil alles gelöst ist.

### 4.4 Distribute-Sweep-Operatoren (= `Project ∘ LOOM ∘ Materialize`; LOOM + PHC + NTC)

Der Distribute-Sweep ist die **zentrifugale Auswebung**: Crystal → Artefakt.

```
encode       : Crystal              → PHCPackage           // === PHC(·); Encode(H,W,G,R,L,E)
decode       : PHCPackage           → Lattice              // Loader; validiert Root-Hash, Achsen, Zellen
read_crystal : Crystal              → (Pattern, Ontology, Goal, ConstraintFamily) // LOOM 7.1
weave_plan   : Crystal              → Weave                // Threads, Separatoren, Workcells
project      : (Lattice, WorkcellId)→ Projection           // === Project(·); lokal, gatebar, klein
needle       : (Crystal, WheelWindow) → Thread             // generative Nadel (LOOM 8)
distribute   : RootedTree           → (node → ClosedMarginal) // = NTC-Engine; Closure nach außen
execute      : Projection           → Candidate            // lokaler Ausführer (Agent/Solver/Mensch)
gate_cell    : (Workcell, Candidate)→ GateReport           // fail-closed; Hold ⇒ repair_or_reweave
weave_join   : (Weave, Ledger)      → Artifact             // verklebt akzeptierte Zellen über Separatoren
migrate      : (λ_i, λ_j)           → PartialMap           // Phase-Ladder; kein Stop/Reset
emit         : (Weave, Ledger, ExportProfile) → Artifact   // === Materialize(·)
export       : Crystal              → Sink                 // closure-erhaltender Funktor
```

LOOM-Zyklus: `Crystal → Decode → Workcell-DAG → LocalProjection → AgentExecution → Gate → Commit → Replay`.
LOOM-Referenzalgorithmus (verbindliche Rollen, siehe `loom_generate` in Teil 7.4).

### 4.5 Monolith-Operator (MERKABA — Gesamtumlauf)

Die monolithische Komposition orchestriert beide Sweeps als **einen** Operator:
```
F = Commit ∘ C ∘ G ∘ Δ ∘ I ∘ Q ∘ Θ ∘ P ∘ Can
```
Umlauf: `x →Can→ P →H→ Ω →I→ S →Δ→ R →G→ A →C→ K →Commit`.
Emission monolithisch ⟺ `E ∈ L ∧ Replay(E)=1 ∧ Pass(G_E)=1 ∧ Res(E) ≤ ε`.
Organische Komposition: `O_j ∘_Can O_i = α_j ∘ Can_ij ∘ α_i` — jedes Modul ist ein Organ `O=(X_O,Y_O,α_O,β_O,σ_O,γ_O)`; getrennte Crates/Services, ein Körper.

### 4.6 Import-Operator (H/F-Grenze)

```
import_F_to_H : Hypothesis → {H-Artifact | ⊥}
  = Typed ∧ BoundaryDefined ∧ Closed ∧ Pass(G) ∧ τ ∧ Replay ∧ NoOverclaim
```
Reflektor `L: F→H`; auf `H` ist `L` Identität (idempotente Monade, `Can²=Can`). Der Import-Operator ist der **Admission-Controller** an jeder Systemgrenze (Eingang, Agentenausgang, Export).

### 4.7 Operator ↔ Formel-Zuordnung (Vollständigkeitsnachweis)

| Formelterm | Operatorkette |
|---|---|
| `PHC(Crystal)` | `encode` |
| `Project(·)` | `decode ∘ project` (mit `plan`, `separators`) |
| `LOOM(·)` | `read_crystal ∘ weave_plan ∘ (∀cell: project ∘ execute ∘ gate_cell ∘ commit) ∘ weave_join`; = `distribute` über den gewurzelten Junction Tree |
| `Materialize(·)` | `emit` (via `ExportProfile`, `reobserve=true`) |
| `Reanalyze(·)` | `observe` = `crystallize ∘ gate ∘ triangulate_attr ∘ horizon ∘ mark ∘ spectralize ∘ embed`; = `collect`-Sweep |
| `≃` | `q(observe(A)) = q(C)` unter deklariertem Quotient (`equivalent`) |

Jeder Operator der Abschlussformel ist damit auf eine konkrete, testbare Signatur zurückgeführt. Kein Formelterm bleibt undefiniert.

---

## TEIL 5 — ARCHITEKTUR-SCHLUSS

Hier werden **LOOM, PHC, BCIK, Constraint Lattice, CCC und MatrixCrystal** zu **einer** Architektur geschlossen. Der Schluss ist nicht additiv (nicht „sechs Module nebeneinander"), sondern ein **einziges fibriertes System**, dessen Rundlauf die Abschlussformel ist.

### 5.1 Das vereinheitlichende Bild (CCC als Klammer)

Der Crystalline Closure Calculus liefert die Klammer, die alles trägt:
- **Faser an jedem Knoten** = der irreduzible Kern (BCIK): `(H_i, R_i, B_i, Γ_i)`, Closure-Gesetz `B⁻_i x_i = 0`.
- **Skelett** = chordaler Kopplungsgraph → **Junction Tree mit RIP**; Separatoren = Nähte = Mandorlas.
- **Collect-Pass** = **Constraint Lattice** (Konstruktion, → `Bx`, die aufwickelnde Spule).
- **Distribute-Pass** = **NTC-Engine / LOOM** (Closure-Verteilung, Materialisierung, Migration, die abwickelnde Spule).
- **Übergabe-Token** an jedem Separator = der **closure-zertifizierte Kristall** (bzw. **MatrixCrystal** im Collect).
- **Transportform** des Kristalls über Träger/Runner hinweg = **PHC**.
- **Integritätsschnitt**, den jeder Kristall bestehen muss = **BCIK**.

```
  Blatt C_ℓ  ── collect ──▶  Knoten C_i  ── collect ──▶  Wurzel C_r
     ▲                          │                           │
     └──────── distribute ──────┴──────── distribute ───────┘
   Separator = Naht = Kristall-Übergabe  (die Zähne, an denen die zwei Spulen greifen)
   Faser je Knoten = BCIK        Skelett = Junction Tree = Constraint-Lattice-Entfaltung
   Transport der Kristalle = PHC   Collect-Karte = MatrixCrystal (TAT)
```

### 5.2 Schluss-Identitäten (die sechs Werke fallen zusammen)

Diese Identitäten sind vom Korpus bewiesen bzw. definitorisch und bilden das Rückgrat des Schlusses:

1. **Adäquatheit = Chordalität** (CCC Satz 19.2 / CL §5.2). Lokale Gate-Akzeptanz = globale genau dann, wenn die Kopplung chordal ist. → **Grund, warum lokale Workcell-Prüfung global gültig ist** und `q(Obs(A))=q(C)` erreichbar wird.
2. **Entfaltungsgraph (CL) = Junction Tree (CCC).** Ein Baum ist maximal azyklisch; Subcubes = maximale Cliquen; Nähte/Mandorlas/geteilte Voids = Separatoren; Running Intersection = QSR-Stabilität.
3. **Closure-Komposition** (CCC Satz 19.3). Lokale Closure jeder Faser + Separator-Konsistenz ⇒ globale Closure, Verklebung eindeutig. → **Grund, warum viele lokale `Commit`s einen globalen Kristall ergeben.**
4. **Zwei-Sweep-Korrektheit** (CCC Satz 19.4). Collect∘Distribute bringt jeden Knoten auf die global konsistente Marginale; bei separator-konsistenter, lokal geschlossener Familie ist sie global geschlossen. Collect = Constraint Lattice, Distribute = NTC/LOOM.
5. **BCIK ⊆ QSR ∩ Int(H⁻_Σ)** (BCIK Satz 11.3). Jeder Kristall ist ein boundary-gelesener QSR-Fixpunkt. → **Grund, warum `Crystal` und `BCIK(A)` derselbe Integritätsschnitt sind.**
6. **PHC = serialisierte Radialspindel-Instanz** (PHC §25, §34). PHC speichert nicht Daten, sondern die Bedingungen, unter denen ein Kristall als Arbeitswelt ausgespannt und wieder auf seinen Kern geprüft werden kann. → **Grund, warum `PHC(·)` und `Project(·)` und `Reanalyze(·)` denselben Kristall referenzieren.**
7. **Radfenster = Nadelapertur** (LOOM §21, PIO). Beobachtung und Materialisierung sind zwei Richtungen **einer** Aperturstruktur. → **Grund, warum `Obs` und `LOOM` denselben Apparat bilden — der mechanische Kern des `≃`.**

### 5.3 Die geschlossene Architektur (Schichten + Fluss)

```
┌──────────────────────────────────────────────────────────────────────────────────┐
│                      MERKABA — MONOLITHISCHER UMLAUF (F = Commit∘…∘Can)             │
│  orchestriert beide Sweeps als einen Operator; Emission bei Gesamt-Closure         │
└───────────▲───────────────────────────────────────────────────────▲───────────────┘
            │                                                         │
   COLLECT-SWEEP (Reanalyze/Obs)                          DISTRIBUTE-SWEEP (Project∘LOOM∘Materialize)
   ┌────────┴─────────────────────────┐                   ┌───────────┴─────────────────────────────┐
   │ PIO  Apertur/Radfenster          │◀── WheelWindow ──▶│ LOOM  Nadelapertur/Weave/Workcell        │
   │ TP   Horizontlandschaft          │   (bidirektional) │ PHC   Encode/Decode/Project/Materialize   │
   │ QLOGIC Spektralregister/PoR      │                   │ NTC   Distribute-Nachrichten/Migration    │
   │ TAT  Marker→…→MatrixCrystal      │                   │ Materialize  Export-Profile               │
   │ DZ   dyadische Zellbasis         │                   │                                           │
   └────────┬─────────────────────────┘                   └───────────┬─────────────────────────────┘
            │                 ┌────────── KRISTALL (Token an jedem Separator) ──────────┐
            └────────────────▶│  Crystal / MatrixCrystal  =  [Can(c)]_σ                  │◀────────────┘
                              └────────────────────────────┬─────────────────────────────┘
                                                           │
   ┌───────────────────────────────────────────────────────┴──────────────────────────────────────┐
   │ GRUNDSTOCK (an jedem Knoten und jeder Kante wirksam)                                           │
   │  CCC  chordales Skelett · Junction Tree · Faser · zwei Sweeps · Verklebung · Bi-Temporalität   │
   │  Constraint Lattice  Cube G0–G3 · Propagationshülle · Entfaltung · Projektion · Collapse-Cert  │
   │  BCIK  Fix∩Boundary∩Closed∩Gate∩Trace∩Replay∩PathInv · Maschine · Erhaltung/Fortschritt · VC   │
   └───────────────────────────────────────────────────────────────────────────────────────────────┘
```

### 5.4 Der Rundlauf als Fixpunkt (Nachweis-Skizze für `ClosedCCE`)

Sei `C` ein Root-Crystal (`BCIK(C)=1`). Dann:

1. `P = encode(C)`. PHC-Kanonisierung liefert stabilen Root-Hash; `Can(P)` gilt (PHC V2). Da PHC eine serialisierte Radialspindel ist, trägt `P` Ontologie, Cube, Weave, Nadelaperturen, Gates, Residuen, Ledger, Reanalyse-Regel.
2. `{πc} = project(decode(P))`. Jede Workcell-Projektion ist **adäquat**, weil die Entfaltung ein Junction Tree einer chordalen (bzw. triangulierten) Kopplung ist (CCC §9). Adäquatheit ⇒ lokale Gate-Akzeptanz = globale (CL Satz 10.5).
3. `A = emit(loom({πc}))`. Der Distribute-Sweep webt, führt lokal aus, gated fail-closed, committed. Verklebung über Separatoren ist eindeutig (CCC Satz 13.1); lokale Closure ⇒ globale Closure. Jeder committete Zustand ist ein BCIK (Erhaltung, BCIK Satz 39.2). Residuen bleiben sichtbar (BCIK A4).
4. `Obs(A) = reanalyze(A)`. Der Collect-Sweep sedimentiert das Artefakt zurück in eine `MatrixCrystal`-Klasse. Weil beide Sweeps **dieselbe** Kanonisierung/Signatur und denselben Closure-Begriff verwenden (CCC §17, Radfenster-Bidirektionalität), ist der Übergang identitätserhaltend.
5. **Abschluss:** `q(Obs(A)) = q(C)` unter dem deklarierten Quotienten `q` (PHC §16). Damit `ClosedCCE(C)=1`.

**Grenze (ehrlich ausgewiesen, BCIK/CCC):** Existiert kein endlicher triangulierter Baum, der `Fix(R)` erreicht (kontinuierlich-irrationaler Fall, CCC §6.5/§10.3), ist globale Closure durch endliche Sweeps unerreichbar; dann greift metrische Konvergenz zum QSNA-Attraktor mit a-priori-Schranke `d(x_n,x*) ≤ λⁿ/(1−λ)·d(x0,Φx0)` (BCIK §42) als zertifizierbares Abbruchkriterium — der Nicht-Abschluss wird als Residuum/Counter-Horizon sichtbar, nie still absorbiert.

### 5.5 Warum kein Werk isoliert bleibt (Gesamtschluss-Nachweis)

| Werk | Ohne den Schluss | Im Schluss (an die Formel gebunden) |
|---|---|---|
| BCIK | abstrakter Integritätsschnitt | `BCIK(A)` = Pflichtkonjunkt in `ClosedCCE`; jeder `commit` ist BCIK-bewacht |
| Constraint Lattice | Substrat ohne Bewegung | = Collect-Sweep; liefert `Bx`/AcceptedPole, den Kristall an der Wurzel |
| CCC | Kalkül ohne Runtime | die Klammer: Faser+Skelett+zwei Sweeps = die ganze Architektur |
| LOOM | Semantik ohne Transport | = Distribute-Sweep; webt Kristall → Artefakt |
| PHC | Container ohne Semantik | serialisierte Radialspindel; trägt `Project`/`Reanalyze`-Bedingungen |
| MatrixCrystal | Karte ohne Rundlauf | = `Obs(A)`; der Kristall, auf den `≃` prüft |
| MERKABA | Monolith ohne Unterbau | orchestriert `F` = beide Sweeps als ein Organkörper |


---

## TEIL 6 — VOLLSTÄNDIGE REPOSITORY-ZIELSTRUKTUR

Ein **einziges** zusammenhängendes Repository, aus einem Guss. Sprachwahl folgt den Korpus-Pflichten (byte-identischer Replay, kanonisches Hashing, fail-closed Gates, keine Wall-Clock, keine ungeseedete Zufälligkeit): **Rust-Kern** (Determinismus, Content-Addressing, Typensicherheit als Closure-Invariante) + **dünne Adapterschicht** (Runner/Agenten/Materialisierung). Jedes Crate ist ein MERKABA-**Organ** `O=(X_O,Y_O,α_O,β_O,σ_O,γ_O)`; Kopplung nur über kanonisierte Typen.

### 6.1 Monorepo-Baum

```
cce/                                     # Crystalline Closure Engine — Wurzel
├─ README.md                             # Zweck, Abschlussformel, Bau-Einstieg
├─ BAUVERFASSUNG.md                      # dieses Dokument (spec of record)
├─ LICENSE / license_policy.toml         # declared|private|internal (PHC §5)
├─ rust-toolchain.toml                   # gepinnte Toolchain (Determinismus)
├─ Cargo.toml                            # Workspace-Manifest (alle Crates)
├─ deny.toml                             # cargo-deny: keine unkontrollierten deps
├─ Makefile / justfile                   # build · test · replay · reanalyze · closure-report
│
├─ crates/
│  ├─ cce-core/                          # GRUNDSTOCK: BCIK-Kern (F-01..F-06, I-*)
│  │  ├─ src/
│  │  │  ├─ lib.rs
│  │  │  ├─ canonical.rs                 # Can (idempotent, deterministisch); Fix(Can)
│  │  │  ├─ signature.rs                 # σ = σ∘Can; Vollständigkeit auf Träger; SHA-256
│  │  │  ├─ reflection.rs                # R=R*, R²=I; B=½(I+R); B⁻; tripolare Faser Γ
│  │  │  ├─ closure.rs                   # closed?(x) ⟺ B⁻x=0; Closure-Zertifikat
│  │  │  ├─ residue.rs                   # ρ=B⁻; Residue-Typ; ResidueVisible; Counter-Horizon
│  │  │  ├─ gate.rs                      # Gate-Familie G (typed/boundary/closure/residue/…); fail-closed
│  │  │  ├─ trace.rs                     # τ, Trace-Comonade; append-only Event-Log
│  │  │  ├─ replay.rs                    # RunDescriptor; byte-identischer Re-Run mod Can
│  │  │  ├─ pathinv.rs                   # Pfadinvarianz; Konfluenz mod ≡σ
│  │  │  ├─ bcik.rs                      # BCIK(K)=Fix(T)∩Int(H⁻Σ)∩Closed∩Pass(G)∩τ∩Replay∩PathInv
│  │  │  ├─ machine.rs                   # Konf (x,Σ,ℱ,ℓ,g) + Regeln Canon/ExCal/Seam/Sediment/Assim/Commit/Hold
│  │  │  ├─ hf_import.rs                 # H/F-Instanz; import_F_to_H; Reflektor L; Admission-Controller
│  │  │  └─ qsna.rs                      # QSNA; Banach-Fixpunkt; a-priori-Abbruchschranke
│  │  └─ tests/ (Erhaltung, Fortschritt, Determinismus, VC1–VC10)
│  │
│  ├─ cce-lattice/                       # GRUNDSTOCK/COLLECT: Constraint Lattice (F-07..F-14)
│  │  ├─ src/
│  │  │  ├─ cube.rs                      # Cube G0–G3; Dimension/Domain/Coupling
│  │  │  ├─ propagation.rs               # Γ_𝒦 Hüllenoperator; kleinster Fixpunkt (Knaster–Tarski)
│  │  │  ├─ feasible.rs                  # Feasible Set; Kandidaten (monoton schrumpfend)
│  │  │  ├─ unfold.rs                    # Entfaltungsgraph U (DAG); Traversierungsplanung
│  │  │  ├─ boundary_contract.rs         # (interface,assump,oblig,verif,expiry); SCC-Kondensation
│  │  │  ├─ projection.rs                # Chameleon-Projektion πv=Base‖Focus‖N‖K‖Ev; Adäquatheit
│  │  │  ├─ collapse.rs                  # Kollaps-Schwellwert p_c; Collapse-Zertifikat
│  │  │  └─ export_functor.rs            # Ex: Commit_M → Sink (closure-erhaltend)
│  │  └─ tests/ (K1–K18 Abnahmekatalog)
│  │
│  ├─ cce-ccc/                           # GRUNDSTOCK: Crystalline Closure Calculus (F-05,F-10..F-15)
│  │  ├─ src/
│  │  │  ├─ chordal.rs                   # Chordalität; PEO; simpliziale Knoten
│  │  │  ├─ triangulate.rs               # Triangulierung; Fill-in; Baumweite tw
│  │  │  ├─ junction_tree.rs             # Junction Tree; RIP; Separatoren
│  │  │  ├─ fiber.rs                     # fibriertes Closure-System; Restriktionen ρ_ij
│  │  │  ├─ glue.rs                      # Verklebung über Separatoren; Lemma 12.2; Satz 13.1
│  │  │  ├─ sweeps.rs                    # collect (Bx) / distribute (ClosedMarginal); Zwei-Pass
│  │  │  ├─ bitemporal.rs                # T2 (intrinsisch) / T1 (Commit-Index) / n0 (Null-Anker)
│  │  │  └─ crystal_protocol.rs          # Kristall ⟺ Closed∧QSR∧Gate∧Replay∧ResidueVisible
│  │  └─ tests/ (C1–C14 Abnahmekatalog)
│  │
│  ├─ cce-crystal/                       # gemeinsamer Kristalltyp (Objektmodell Teil 3.4)
│  │  ├─ src/ crystal.rs · matrix_crystal.rs · monolith.rs · quotient.rs (q, equivalent)
│  │  └─ tests/
│  │
│  ├─ cce-loom/                          # GENERATIV: Generative Inversion (G-01..G-08)
│  │  ├─ src/
│  │  │  ├─ radial_spindle.rs            # Rad=(Fin,Fout,Z0,Σ,M,Replay); 4π; Nullanker
│  │  │  ├─ mandorla.rs                  # M=Eq(Fin,F†out); Umschlagkern; Boundary-Regularisierung
│  │  │  ├─ weave.rs                     # Weave/Thread; Hypercube-Strickung W_{n,k}=⊔W_{n+1,2k+ε}
│  │  │  ├─ workcell.rs                  # ω=(c,πc,Kc,Oc,Gc,Rc,Qc); Ausführungsstatus
│  │  │  ├─ needle.rs                    # NeedleAperture=(a,n,p,g); = WheelWindow (out)
│  │  │  ├─ distribute.rs               # loom_generate Referenzalgorithmus; repair_or_reweave
│  │  │  └─ artifact_classes.rs          # Code/Spec/Repo/Research/DataPipeline/AgentRun
│  │  └─ tests/ (Nullanker/Singularität/Dyaden/Collect/Distribute/Nadel/Workcell/Replay/Reanalyse)
│  │
│  ├─ cce-phc/                           # GENERATIV: Projektiver Hypercube-Codec (G-09..G-20)
│  │  ├─ src/
│  │  │  ├─ manifest.rs                  # M: version/codec_id/status/domain_mode/root_crystal/canon
│  │  │  ├─ canon.rs                     # PHC-CANON-0.1 (UTF-8/Keysort/Refs/Zeit/Zahlen/SHA-256)
│  │  │  ├─ axes.rs                      # semantic/topological/temporal/control/evidence/material
│  │  │  ├─ cells.rs                     # Zelle + Zelladresse phc://…; Faser/Boundary/Seam
│  │  │  ├─ projection_calc.rs           # πc:H→Hc; Projektionsprofil; No-Horizon-Leakage
│  │  │  ├─ gate_matrix.rs               # G1–G7 Pflichtgates; fail-closed
│  │  │  ├─ residue.rs                   # Residuum + Counter-Horizon CH
│  │  │  ├─ ledger.rs                    # append-only Events; RunDescriptor
│  │  │  ├─ materialize.rs               # Export/Materialisierungsprofile; reobserve
│  │  │  ├─ loader.rs                    # load_phc/project/commit_result; V0–V9
│  │  │  ├─ migration.rs                 # append-only Migration; proof
│  │  │  └─ profiles.rs                  # PHC-CORE-0.1 / PHC-LOOM-0.1 / PHC-MERKABA-0.1
│  │  └─ tests/ (V0–V9; Fehlermodell; Konformitätsstufen Readable→Portable)
│  │
│  ├─ cce-observe/                       # BEOBACHTEND (Collect/Reanalyze): O-*
│  │  ├─ src/
│  │  │  ├─ substrate.rs                 # DZ: dyadische Zellen; Filtration; Zellzustand λ_n
│  │  │  ├─ qlogic.rs                    # QLOGIC: Ω/SG/Ξ/PoR/ΦU,ΦV/C/M; Spektralregister; Q-Zustand
│  │  │  ├─ panoptikum.rs                # TP: NullCenter/Hypertorus/PhaseRays/Windows/Möbius-Lift
│  │  │  ├─ optics.rs                    # PIO: Apt/Lens/Split/Track/Dump/Wave/Pulse; Radfenster
│  │  │  ├─ tat.rs                       # TAT: Mark→Respond→Horizon→Triangulate→Gate→MatrixCrystal
│  │  │  ├─ wheel_window.rs              # WheelWindow (Apertur↔Nadel) — Bridge zu cce-loom::needle
│  │  │  └─ reanalyze.rs                 # observe(Artifact)→MatrixCrystal (=== Reanalyze)
│  │  └─ tests/ (P1–P7 TAT-Abnahme; Reanalyse-Divergenz; Nachführung erhält Fixpunkt)
│  │
│  ├─ cce-merkaba/                       # MONOLITH: Gesamtumlauf (G-21..G-24)
│  │  ├─ src/
│  │  │  ├─ organ.rs                     # Organ O=(X,Y,α,β,σ,γ); organische Komposition ∘_Can
│  │  │  ├─ center.rs                    # Zentrum Z; Digest η; Bindungsfunktion ζ
│  │  │  ├─ scaffold.rs                  # markierter Graph; Scaffold-Einbettung; σ_loc/rel/sym
│  │  │  ├─ circulate.rs                 # F = Commit∘C∘G∘Δ∘I∘Q∘Θ∘P∘Can; E=F(x)
│  │  │  ├─ organ_registry.rs            # bindet cce-observe + cce-loom + cce-phc als Organe
│  │  │  └─ monolith.rs                  # MERKABA-Monolith; Emission ⟺ E∈L∧Replay∧Pass(G_E)∧Res≤ε
│  │  └─ tests/ (Master-Normalform; Konvergenz aller Projektionsflächen auf eine Signaturklasse)
│  │
│  ├─ cce-materialize/                   # ADAPTER: Export-/Materialisierungsprofile (= Materialize)
│  │  ├─ src/ latex_monograph.rs · markdown.rs · cargo_workspace.rs · repo.rs · data_pipeline.rs
│  │  └─ tests/ (Materialisierung nur nach Gate/Evidence/Trace/Replay; reobserve-Pflicht)
│  │
│  └─ cce-runner/                        # ADAPTER/CLI: Reference Runner + Orchestrierung
│     ├─ src/
│     │  ├─ main.rs                      # CLI: encode/project/run/materialize/reanalyze/closure
│     │  ├─ runner.rs                    # R1–R13 (load→canon→project→exec→gate→commit→materialize→reanalyze→closure)
│     │  ├─ agent_input.rs               # A1–A4 (Ziel/Kontext/Constraints/Ausgabevertrag)
│     │  ├─ admission.rs                 # Import-Gate als Admission-Controller (fail-closed)
│     │  └─ closure_report.rs            # erzeugt ClosureReport gegen ClosedCCE(C)
│     └─ tests/
│
├─ agents/                              # ADAPTER (Nicht-Kern): lokale Ausführer (Agent/Solver/Mensch)
│  ├─ agent_contract.md                 # Agent = lokaler Operator, NICHT Autorität
│  ├─ llm_operator/                     # gekapselter LLM-Operator (sandboxed, scope-begrenzt)
│  ├─ solver_operator/                  # SAT/SMT/Constraint-Solver-Operator
│  └─ human_operator/                   # menschliche Workcell-Bearbeitung (Review-UI-Stub)
│
├─ schemas/                             # NORMATIV: JSON-Schemas (Teil 7.1)
│  ├─ phc.schema.json                   # Reference Schema (kompakte Pflichtoberfläche)
│  ├─ crystal.schema.json
│  ├─ matrix_crystal.schema.json
│  ├─ gate_matrix.schema.json
│  ├─ residue_report.schema.json
│  ├─ run_descriptor.schema.json
│  ├─ ledger_event.schema.json
│  └─ certificate.schema.json
│
├─ reference-cubes/                     # kleine, perfekte Beispiele je Domäne (Metaarchitektur §8.7)
│  ├─ math/            (z. B. PrimeKernel: Φ(13)=e13, ρ_comp=0)
│  ├─ software/        (minimaler cargo-workspace-Export)
│  ├─ document/        (Monographie-Materialisierung → tex+pdf)
│  └─ graph/           (chordaler Kopplungsgraph mit Junction Tree)
│
├─ negative-cubes/                      # absichtlich falsche Cubes/Gates/Residuen/Driftfälle (§8.8)
│  ├─ boundary_crossing_without_seam.phc.json
│  ├─ silent_residue_absorption.phc.json
│  ├─ score_as_gate.phc.json
│  ├─ reanalysis_divergence.phc.json
│  └─ prompt_regression.phc.json
│
├─ spec/                                # der Korpus (Quellen) + Ableitungen
│  ├─ corpus/  (13 Quell-PDFs, read-only)
│  ├─ glossary.md                       # Begriffsmatrix (bildhaft → formaler Typ → harte Interpretation)
│  └─ terminology_contract.md           # erlaubte/ausgeschlossene Begriffe; Antichimären-Regel
│
├─ tests/                               # repo-weite Integration
│  ├─ closure_roundtrip/                # Reanalyze(Materialize(LOOM(Project(PHC(C))))) ≃ C
│  ├─ replay/                           # gleicher RunDescriptor ⇒ gleiche kanonische Klasse
│  ├─ property/                         # Erhaltung/Fortschritt/Konfluenz (proptest)
│  └─ conformance/                      # PHC-CORE/LOOM/MERKABA-Konformität
│
├─ ci/                                  # deterministische Pipeline (keine Wall-Clock in Gates)
│  ├─ pipeline.yml                      # build→test→replay→closure-report→conformance
│  └─ gates.yml                         # G1–G7 als CI-Gates; fail-closed
│
└─ docs/
   ├─ ARCHITECTURE.md                   # Teil 5 als lebendes Dokument
   ├─ OBJECT_MODEL.md                   # Teil 3
   ├─ OPERATOR_MODEL.md                 # Teil 4
   └─ DEVELOPER_CONTRACT.md            # Schnittstellen für Agenten/Tools (Metaarchitektur §8.9)
```

### 6.2 Crate-Abhängigkeitsgraph (azyklisch — selbst ein DAG/Junction Tree)

```
cce-core ──▶ cce-lattice ──▶ cce-ccc ──▶ cce-crystal
   │             │              │             │
   ├─────────────┴──────────────┴─────────────┤
   ▼                                          ▼
cce-phc ◀── cce-loom                    cce-observe
   │            │                            │
   └────────────┴─────────► cce-merkaba ◀────┘
                                │
                    cce-materialize ─── cce-runner ─── agents/
```
Regel: **keine Rückkanten** (die Architektur ist selbst ein azyklischer Entfaltungsgraph). `cce-core` hängt von nichts ab; alles hängt (transitiv) von `cce-core`. Jede sonst nötige Rückkante ist ein dokumentierter `BoundaryContract` mit Expiry.

### 6.3 Datenkörper-Konvention (organische Kopplung)

Alle Organe koppeln über **sechs kanonisierte Datenkörper** (MERKABA §100): kanonischer Zustand, Zentrum/Nullanker, Phase, Signatur, Residuum, Trace/Commit. Dadurch dürfen Implementierungen getrennte Crates/Services verwenden, während die Semantik **ein Körper** bleibt.


---

## TEIL 7 — SCHEMAS, MODULE, TESTS, GATES, RESIDUEN, REPLAY-PFADE

### 7.1 Schemas (normativ)

#### 7.1.1 `phc.schema.json` — Pflichtoberfläche (PHC §32)

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "cce/schemas/phc.schema.json",
  "title": "Projektiver Hypercube-Codec v0.1",
  "type": "object",
  "required": ["phc_version","manifest","canonicalization","types","axes","cells",
               "workcells","gates","residue","ledger"],
  "properties": {
    "phc_version": {"const": "0.1"},
    "manifest": {
      "type": "object",
      "required": ["codec_id","status","root_crystal","canonicalization"],
      "properties": {
        "codec_id": {"type":"string","pattern":"^phc:sha256:[0-9a-f]{64}$"},
        "title": {"type":"string"},
        "status": {"enum":["draft","sealed","closed","archived"]},
        "domain_mode": {"enum":["agnostic","software","math","research","document","evidence","execution","mixed"]},
        "root_crystal": {"type":"string","pattern":"^crystal:"},
        "canonicalization": {"const":"PHC-CANON-0.1"},
        "created_at": {"type":"string","format":"date-time"},
        "license_policy": {"enum":["declared","private","internal","unspecified"]},
        "entrypoints": {"type":"object"}
      }
    },
    "canonicalization": {"type":"object"},
    "types": {"type":"object","properties":{"quotient":{"type":"string"}}},
    "quotients": {"type":"array"},
    "axes": {"type":"array","items":{
      "type":"object","required":["id"],
      "properties":{"id":{"type":"string"},"name":{"type":"string"},
        "kind":{"enum":["semantic","topological","temporal","control","evidence","material"]},
        "scale":{"enum":["nominal","ordered","interval"]},
        "values":{"type":"array"},"role":{"type":"string"}}}},
    "cells": {"type":"array","items":{
      "type":"object","required":["id","axis_values"],
      "properties":{"id":{"type":"string"},"address":{"type":"string"},
        "axis_values":{"type":"object"},
        "fiber":{"type":"object","properties":{
          "claims":{"type":"array"},"constraints":{"type":"array"},
          "artifacts":{"type":"array"},"residue":{"type":"array"}}},
        "boundary":{"type":"object","properties":{
          "imports":{"type":"array"},"exports":{"type":"array"},"seams":{"type":"array"}}},
        "signature":{"type":"string"}}}},
    "seams": {"type":"array","items":{
      "type":"object","required":["id","cells","direction","rule","gate"],
      "properties":{"id":{"type":"string"},"kind":{"type":"string"},
        "cells":{"type":"array"},"direction":{"enum":["inbound","outbound","bidirectional"]},
        "rule":{"type":"string"},"gate":{"type":"string"},
        "residue_policy":{"const":"visible"}}}},
    "crystals": {"type":"array"},
    "constraints": {"type":"array"},
    "projections": {"type":"array","items":{
      "type":"object","required":["id","target_cell","include","allowed_ops"],
      "properties":{"id":{"type":"string"},"target_cell":{"type":"string"},
        "include":{"type":"object"},"exclude":{"type":"array"},
        "max_tokens":{"type":"integer"},"allowed_ops":{"type":"array"}}}},
    "workcells": {"type":"array","items":{
      "type":"object","required":["id","cell","projection","allowed_operations","gate_chain"],
      "properties":{"id":{"type":"string"},"cell":{"type":"string"},
        "projection":{"type":"string"},"intent":{"type":"string"},
        "input_contract":{"type":"object","properties":{
          "required":{"type":"array"},"forbidden":{"type":"array"}}},
        "allowed_operations":{"type":"array"},
        "output_contract":{"type":"object","properties":{
          "type":{"type":"string"},"format":{"type":"string"},
          "must_emit":{"type":"array"},"must_not_emit":{"type":"array"}}},
        "gate_chain":{"type":"array"},
        "residue_policy":{"type":"string"},"replay":{"type":"string"}}}},
    "weaves": {"type":"array","items":{
      "type":"object","required":["id","kind","workcells","closure"],
      "properties":{"id":{"type":"string"},"kind":{"const":"partial_order"},
        "threads":{"type":"array"},"workcells":{"type":"array"},
        "closure":{"type":"string"}}}},
    "needle_apertures": {"type":"array"},
    "radial_spindle": {"type":"object","properties":{
      "zero_anchor":{"type":"string"},"orientation":{"const":"4pi"},
      "dyads":{"type":"array"},"flows":{"type":"object"},
      "boundary_regularization":{"type":"object","properties":{
        "nullpoint_traversal":{"const":"forbidden"},
        "boundary_trace_required":{"const":true},
        "mandorla_seam":{"type":"string"}}}}},
    "gates": {"type":"array","minItems":7,"items":{
      "type":"object","required":["id","failure"],
      "properties":{"id":{"type":"string"},"kind":{"type":"string"},
        "rule":{"type":"string"},"failure":{"const":"fail_closed"}}}},
    "residue": {"type":"object","required":["policy"],
      "properties":{"policy":{"const":"visible"}}},
    "ledger": {"type":"object","required":["mode"],
      "properties":{"mode":{"const":"append_only"}}},
    "exports": {"type":"array"},
    "diagnostics": {"type":"object","properties":{
      "nullmodels":{"enum":["optional","required"]},
      "falsification":{"enum":["optional","required"]},
      "proof_horizon":{"enum":["optional","required"]},
      "matrix_crystal":{"const":"compatible"}}},
    "reanalysis": {"type":"object"},
    "migration": {"type":"object"}
  }
}
```

#### 7.1.2 Weitere Schemas (Feldverträge aus Teil 3)

- **`crystal.schema.json`**: `{id (crystal:<hash>), signature, type_status, provenance, boundary_state, residue_state, replay_contract, closure_certificate}` — alle Pflicht.
- **`matrix_crystal.schema.json`**: `crystal` + `{attractor_map, horizons, triangulation, gates, residues, null_models, trace, replay_hash}`.
- **`gate_matrix.schema.json`**: Array von `{id, kind∈{admission,topological,type,integrity,determinism,export,reanalysis}, rule, failure:"fail_closed"}`; MUSS G1–G7 enthalten.
- **`residue_report.schema.json`**: `{id, origin, kind, severity, content, visible_to[], resolution_status}`.
- **`run_descriptor.schema.json`**: `{version, canon, signature, gate_matrix, tolerances, operators, seed, input, domain, exportprofile}` — keine Wall-Clock.
- **`ledger_event.schema.json`**: `{seq, event∈{decode,project,execute,gate,residue,commit,materialize}, …, digest}`.
- **`certificate.schema.json`**: `{idCan, signature, run_descriptor, gate_report, boundary_report, residual_report, trace_head, replay_manifest, path_report}`.

### 7.2 Module (Verantwortungen, verbindlich)

Jedes Modul ist ein Organ mit Ein-/Ausgangstyp, Gate/Boundary-Abbildung, lokaler Signatur und Kopplung. Kern-Verantwortungen (Auswahl):

| Modul | Eingang → Ausgang | Kern-Invariante | Korpus-Anker |
|---|---|---|---|
| `cce-core::canonical` | `State → CanonicalState` | `Can∘Can=Can`, deterministisch | BCIK A1 |
| `cce-core::signature` | `CanonicalState → Sig` | `σ=σ∘Can`, vollständig; sonst Kollision→Residuum | BCIK A2, CCC §1 |
| `cce-core::bcik` | `Kandidat → {BCIK|Reject}` | Materialisierung nur bei `BCIK=1 ∧ cert≠∅` | BCIK §9 |
| `cce-core::machine` | `Conf → Conf` | Erhaltung+Fortschritt; Determinismus mod Can | BCIK §36–39 |
| `cce-lattice::propagation` | `DetState → DetState` | extensiv/monoton/idempotent; lfp | CL §3.2 |
| `cce-lattice::projection` | `(State,Cell) → Projection` | Adäquatheit; No-Horizon-Leakage | CL §5 |
| `cce-ccc::junction_tree` | `ChordalGraph → Tree` | RIP; Separatoren=Nähte | CCC §8 |
| `cce-ccc::sweeps` | `RootedTree → node→pole/marginal` | Collect=Bx, Distribute=Closure; ein Hin/Rücklauf | CCC §14 |
| `cce-loom::distribute` | `Crystal → Artifact` | jeder Faden reanalysierbar; Reanalysis-Gate | LOOM §7,§20 |
| `cce-phc::loader` | `Bundle → Lattice` | V0–V9; Root-Hash-Prüfung | PHC §17,§18 |
| `cce-observe::reanalyze` | `Artifact → MatrixCrystal` | `q(Obs(A))=q(C)` prüfbar | TAT §10, PHC §16 |
| `cce-merkaba::circulate` | `x → Emission E` | `E∈L∧Replay∧Pass(G_E)∧Res≤ε` | MERKABA §96 |
| `cce-runner::closure_report` | `C → ClosureReport` | wertet `ClosedCCE(C)` aus | Teil 0.3 |

### 7.3 Gates (Pflichtfamilie, fail-closed)

Sieben Pflichtgates (PHC §12.2), erweitert um die BCIK-Gate-Familie (BCIK §7). Jedes Gate ist ein Prädikat, **kein Score**; bei Unsicherheit `Hold`, nie `Fire`.

| Gate | Regel | Fehlerverhalten | Blockiert |
|---|---|---|---|
| **G1 Scope** | alle Outputs im deklarierten Scope | fail_closed | Materialisierung außerhalb Exportroot |
| **G2 Boundary** | kein Boundary-Übergang ohne Seam; `lim_{r→0}I(r)∉C ⇒ ∂I∈R∪G∪L` | fail_closed | stiller Nullpunkt-Durchgang |
| **G3 Type** | alle Objekte typisiert, Rolle+Lebenszyklus | fail_closed | untypisierte Fusion |
| **G4 Residue** | jeder fallengelassene Inhalt sichtbar als Residuum | fail_closed | stille Absorption |
| **G5 Replay** | gleicher RunDescriptor ⇒ gleiche Digest-Klasse | fail_closed | nicht-reproduzierbarer Lauf |
| **G6 Export** | Materialisierung nur nach Gate/Evidence/Trace/Replay | fail_closed | ungegatete Emission |
| **G7 Reanalysis** | `q(Obs(A))=q(C_P)` | fail_closed | Reanalysis-Divergenz |
| G-boundary/closure/residue/evidence/trace/replay/path/scope (BCIK) | Konjunktion `G` | fail_closed | Nicht-BCIK-Commit |
| G-PoR (QLOGIC) | Proof-of-Resonance | fail_closed (Hold) | „Gate vor Emission" |
| G-null (Counter-Horizon) | `Gate_null(n_i,A)=0 ∨ Residue(n_i) sichtbar` | fail_closed | reine Selbstbestätigung |

**Gate-Kette pro Workcell:** `[G1 Scope → G2 Boundary → G3 Type → G4 Residue → G5 Replay]`; bei Materialisierung zusätzlich `[G6 Export → G7 Reanalysis]`. Ein `gate_failed`-Ergebnis darf **niemals** verschmolzen werden.

### 7.4 Referenzalgorithmen (verbindliche Rollen)

**Distribute (LOOM §20):**
```python
def loom_generate(crystal, runtime):
    assert gate(crystal)                          # BCIK/Closure-Vorbedingung
    plan   = weave_plan(crystal)                  # Threads/Workcells/Separatoren
    ledger = []
    for cell in topological_order(plan.workcells):# Junction-Tree-Ordnung (DAG)
        view   = project(cell, crystal, ledger)   # lokal, adäquat, No-Horizon-Leakage
        result = runtime.execute(view)            # Agent/Solver/Mensch = lokaler Operator
        report = gate_cell(cell, result)          # fail-closed
        if not report.pass:
            ledger.append(hold(cell, report))     # Hold = Diagnose
            return repair_or_reweave(crystal, ledger)
        ledger.append(commit(cell, result, report))
    artifact = assemble(plan, ledger)             # Verklebung über Separatoren
    reobs    = observe(artifact)                  # === Reanalyze / Collect-Sweep
    if equivalent(reobs, crystal):                # q(Obs(A)) = q(C)
        return close(artifact, ledger, reobs)     # ClosedCCE=1
    return residue(artifact, ledger, reobs)       # Divergenz → sichtbares Residuum
```

**Loader (PHC §17):** `load_phc → canonicalize → assert root_hash → validate(schema,refs,axes,cells,gates,residue) → build_lattice`; `project → apply_projection → assert gate_scope`; `commit_result → run_gates → collect_residue → append_ledger → seal_if_passed`.

**BCIK-Validate (BCIK §25):** `Can-Idempotenz → Signatur → Boundary nichtleer → B⁻K=0 (Residuum reported) → Pass(G) sonst Materialize=0 → Trace∧Replay → Pfadinvarianz → emit cert`.

**Reanalyze/Collect (TAT):** `Embed → Mark → Respond → Horizon → Triangulate → Gate → Crystalize(MatrixCrystal)`.

### 7.5 Tests (Teststrategie, vollständig)

Sechs Testklassen, jede an die Abschlussformel gebunden:

1. **Unit-Tests** je Modul gegen die Modul-Invariante (Tabelle 7.2).
2. **Property-Tests** (proptest/quickcheck): Idempotenz `Can∘Can=Can`; Monotonie/Extensivität/Idempotenz der Propagationshülle; Erhaltung (Subject Reduction); Fortschritt; Konfluenz mod ≡σ; Verklebungs-Eindeutigkeit.
3. **Negative Tests** (`negative-cubes/`, BCIK §26): jede der folgenden Bedingungen MUSS die Engine ablehnen — Score-als-Gate; Residuum ohne Report; Boundary-Crossing ohne Seam; Reanalysis-Divergenz; Prompt-Regression; Prime-Power als Atom; Import ohne Gate; append-only nachträglich verändert; Physik-/RH-/`π,ζ`-Operator-Behauptung.
4. **Reference-Cube-Tests** (`reference-cubes/`): je Domäne (math/software/document/graph) ein kleiner, perfekter Cube, der `ClosedCCE(C)=1` erfüllt.
5. **Replay-Tests** (`tests/replay/`): gleicher `RunDescriptor` ⇒ byte-identische kanonische Klasse; kein Wall-Clock, keine ungeseedete Zufälligkeit.
6. **Closure-Roundtrip-Tests** (`tests/closure_roundtrip/`): **der Kerntest** —
   ```
   assert equivalent( reanalyze(materialize(loom(project(encode(C))))), C )   # für jeden Reference-Cube
   ```

Abnahme-Kataloge, die als Testsuiten materialisiert werden: **CL K1–K18**, **CCC C1–C14**, **PHC V0–V9 + Konformitätsstufen**, **LOOM Abnahmekatalog (10 Tests)**, **TAT P1–P7**, **BCIK VC1–VC10**.

### 7.6 Residuen (Residuenvertrag, hart)

- **Sichtbarkeit ist Pflicht** (BCIK A4, CL K12). `ρ=B⁻x` ist stets als Feld ausgewiesen; Closure heißt `B⁻x=0` **mit** ausgewiesenem (dann leerem) Residuum, nicht stilles Absorbieren.
- **Counter-Horizon** (PHC §13.1): geordnete Menge von Gegenbedingungen/Nullmodellen/Pathologien; verhindert Schließung durch reine Selbstbestätigung.
- **Nicht referenzierte Nutzdaten** in einem PHC-Bundle gelten als Residuum oder müssen entfernt werden (PHC §4.1).
- **Severity** `{info, warning, blocking}`: `blocking`-Residuen verhindern `sealed`/`closed`.
- **Nicht-Erreichbarkeit** (CCC §6.5): wo endliche Sweeps `Fix(R)` nicht erreichen, wird der Nicht-Abschluss als Residuum/Counter-Horizon geführt + a-priori-Schranke; **niemals** still absorbiert.

### 7.7 Replay-Pfade

- **RunDescriptor** bindet alle replay-relevanten Parameter (7.1.2). Ohne vollständigen RD: Fehler `ReplayDescriptorIncomplete`.
- **Ledger** ist append-only Hash-Kette; Monolithen unveränderlich. Nachträgliche Änderung = Abnahmeverletzung.
- **Replay-Identität** (CL Satz 10.6, CCC Satz 19.5): gleicher content-addressierter Input + gleicher RD + deterministische Operatoren ⇒ dieselbe Commit-Klasse `[x]` im Quotienten `D=X/≡σ`.
- **Replay-Manifest** (`replay_index.json`): Zuordnung Event→Digest→RunDescriptor, erlaubt exakte Wiederholung auf anderem Runner (Konformitätsstufe `PHC-Portable`).
- **Determinismus modulo Kanonisierung** (BCIK Satz 37.1) ist die Replay-Garantie auf Maschinenebene: divergierende Einzelschritte konvergieren mod ≡σ.


---

## TEIL 8 — Agentic Build Constitution

Diese Bauverfassung ist die **oberste normative Instanz** für jeden Agenten (menschlich oder maschinell), der `cce/` errichtet, erweitert oder wartet. Sie steht über jeder Einzelentscheidung, jedem Ticket, jeder lokalen Optimierung. Wo eine konkrete Anweisung dieser Verfassung widerspricht, gilt die Verfassung; der Widerspruch selbst ist als Residuum zu vermerken (§8.6), nicht still aufzulösen.

Die Verfassung ist selbst **abschlussgebunden**: Ihr einziger Existenzgrund ist, dass am Ende des Baus die Abschlussformel

```
Reanalyze(Materialize(LOOM(Project(PHC(Crystal))))) ≃ Crystal
```

als lauffähiger, geprüfter, replaybarer Sachverhalt im Repository steht — nicht als Behauptung, sondern als grüner Kerntest (§7.5.6).

### 8.1 Präambel — der Bauvertrag

Der bauende Agent verpflichtet sich auf **einen einzigen Satz**, der die Master-Normalform (Teil 0) in eine Bauhaltung übersetzt:

> *Jedes Artefakt, das ich erzeuge, entsteht durch Beobachten → Kristallisieren → Kodieren → Projizieren → Materialisieren → Replay → Re-Analyse, und ist erst dann fertig, wenn die Re-Analyse den Ausgangskristall bis auf kanonische Äquivalenz zurückgibt.*

Alles Weitere in dieser Verfassung ist Explikation dieses Satzes. Der Agent baut **nicht** Software, die zufällig die Dokumente zitiert; er baut **die eine Apparatur**, deren beobachtender und generativer Halbraum durch dieselbe Radfenster/Nadelapertur-Mechanik gekoppelt sind (Teil 5, Identität I-7).

### 8.2 Leitprinzipien (P1–P12)

Diese Prinzipien sind **Entscheidungsregeln**: Bei jeder Designfrage prüft der Agent sie in Reihenfolge und wählt die Option, die das erste zutreffende Prinzip erfüllt.

| # | Prinzip | Operative Bedeutung |
|---|---------|---------------------|
| **P1** | **Abschluss vor Fläche** | Ein vollständig geschlossener Pfad (Encode→…→Reanalyze) für **eine** Domäne schlägt zehn halbe Pfade für zehn Domänen. Erst der Kerntest grün, dann Breite. |
| **P2** | **Kanon vor Repräsentation** | Jeder Zustand wird vor jeder Operation kanonisiert (`Can∘Can=Can`). Gleichheit ist **immer** Gleichheit kanonischer Klassen, nie Byte-Gleichheit roher Formen. |
| **P3** | **Fail-closed vor Fail-open** | Fehlt eine Prüfung, ein Gate, ein Nachweis, eine Kanonisierung — wird **abgelehnt**, nie durchgelassen. Kein `unwrap()` auf Grenzverträgen, kein „später prüfen“. |
| **P4** | **Residuum sichtbar vor Residuum bequem** | `B⁻x` ist stets ausgewiesenes Feld. Schließen heißt `B⁻x=0` **mit** ausgewiesenem Residuum. Ein still absorbiertes Residuum ist ein Verfassungsbruch (§8.4). |
| **P5** | **Naht vor Sprung** | Jeder Übergang über eine Grenze (Zelle→Zelle, Cube→Cube, Domäne→Domäne) geht durch einen Separator/Seam/Mandorla mit Grenzvertrag. Kein direkter Boundary-Crossing. |
| **P6** | **Ein Apparat, zwei Sweeps** | Collect (Reanalyze) und Distribute (Project∘LOOM∘Materialize) teilen sich **eine** Skeleton-/Junction-Tree-Struktur. Beobachtung und Generierung werden nie als getrennte Subsysteme gebaut. |
| **P7** | **Nullanker unüberschreitbar** | `Z0/Π0/NullCenter/NullAnchor` markiert, nie durchlaufen (`τ`-Regel). Jeder Pfad, der den Nullpunkt traversiert, ist ein Fehler, kein Sonderfall. |
| **P8** | **Score misst, Gate entscheidet** | Numerische Kennzahlen dürfen **nie** als Abnahme-Gate wirken (BCIK A6, CCC C13). Gates sind boolesche, begründete, fail-closed Prädikate. |
| **P9** | **Determinismus vor Tempo** | Kein Wall-Clock, keine ungeseedete Zufälligkeit, keine nichtdeterministische Nebenläufigkeit auf dem Abschlusspfad. Gleicher `RunDescriptor` ⇒ gleiche Commit-Klasse. |
| **P10** | **Content-Address vor Referenz** | Artefakte werden über Inhalts-Digest identifiziert, nicht über Pfad/Namen/Zeitpunkt. Der Ledger ist append-only Hash-Kette. |
| **P11** | **Import nur durch Gate** | Fremdes Material (Babylon/Barbara/externe Domänen) betritt den Kern **ausschließlich** durch das H/F-Import-Gate mit Kanonisierung + Residuum-Kennung (BCIK §H/F). Kein Rohimport. |
| **P12** | **Ehrliche Grenze vor stiller Reichweite** | Wo endliche Sweeps `Fix(R)` nicht erreichen (CCC §6.5), wird der Nicht-Abschluss als Counter-Horizon + a-priori-Schranke geführt. Es wird **nie** Abschluss behauptet, wo keiner bewiesen ist. |

### 8.3 Bau-Invarianten (INV-1 … INV-14)

Invarianten sind **maschinell prüfbare** Aussagen, die **nach jedem Merge** gelten müssen. Sie werden als CI-Gates (§7 Gates, `ci/`) materialisiert; ihr Bruch blockiert den Merge (fail-closed).

1. **INV-1 (Kanon-Idempotenz).** Für jeden Zustandstyp gilt `Can(Can(x)) = Can(x)` als grüner Property-Test.
2. **INV-2 (Signatur-Konsistenz).** Kein Objekt mit `Σsig`-Verletzung erreicht `gated`/`sealed`. Typ-/Träger-/Grad-Signaturen stimmen entlang jeder Kopplung überein.
3. **INV-3 (Nichtleerer Rand).** Jeder Kristall/Cube/Cell besitzt einen definierten, nichtleeren Boundary-Operator-Kontext `(B,B⁻,Seam)`.
4. **INV-4 (Residuum-Gesetz).** `Materialize=1 ⟹ B⁻x=0`, und `B⁻x` ist als Feld vorhanden. Kein `sealed` mit `blocking`-Residuum.
5. **INV-5 (Gate-Vollständigkeit).** Jedes der sieben Pflicht-Gates G1–G7 (§7.4) ist auf dem Abschlusspfad ausgeführt und protokolliert; fehlt ein Gate-Report, gilt der Lauf als abgelehnt.
6. **INV-6 (Naht-Erhaltung / Running Intersection).** Der Kopplungsgraph ist chordal; der Junction Tree erfüllt die Running-Intersection-Property; jeder Separator trägt einen Grenzvertrag. (CCC Adäquatheit = Chordalität, Identität I-1.)
7. **INV-7 (Zwei-Sweep-Korrektheit).** Collect erzeugt korrekte Konstruktion `Bx`; Distribute erzeugt korrekte Schließungsverteilung; ihre Komposition ist der geprüfte Round-Trip. (Identität I-4.)
8. **INV-8 (Subject Reduction & Progress).** Jede Maschinenregel erhält die Signatur (Erhaltung); ein wohlgeformter, nicht-terminaler Zustand hat stets einen nächsten Schritt (Fortschritt). (BCIK Metatheorie.)
9. **INV-9 (Konfluenz modulo ≡σ).** Divergierende Reduktionsreihenfolgen konvergieren in der kanonischen Klasse; die Commit-Klasse `[x] ∈ X/≡σ` ist reihenfolgeunabhängig.
10. **INV-10 (Replay-Identität).** Gleicher content-addressierter Input + gleicher `RunDescriptor` ⇒ byte-identische kanonische Klasse (CL Satz 10.6, CCC Satz 19.5).
11. **INV-11 (Acyclischer Kern).** Der Crate-/Modul-Abhängigkeitsgraph (Teil 6) ist azyklisch; keine adapterseitige Abhängigkeit zeigt in den Kern hinein außer über deklarierte Ports.
12. **INV-12 (Ledger-Unveränderlichkeit).** Der Ledger ist append-only; jede nachträgliche Änderung eines Monolithen/Eintrags ist eine Abnahmeverletzung und wird von `verify_ledger` erkannt.
13. **INV-13 (Import-Gate-Zwang).** Kein Symbol aus `precursors/`/externen Domänen erscheint im Kern ohne Durchlauf durch das H/F-Import-Gate mit Kanonisierung + Residuum.
14. **INV-14 (Claim-Schranke).** Der Code enthält **keine** Behauptung über Physik, Riemann-Hypothese, `π/ζ`-als-Operator o. Ä. jenseits der im Korpus zugelassenen Reichweite (BCIK A8). Dokumentation und Kommentare halten dieselbe Schranke.

### 8.4 Verbotsachse (V1–V10, hart, nicht verhandelbar)

Diese Verbote sind **Abbruchbedingungen**. Verletzt ein Commit eines davon, wird er **nicht** gemerged; der Versuch wird als Negative-Cube-Fall (`negative-cubes/`) fixiert, damit die Engine ihn dauerhaft ablehnt.

1. **V1 — Kein Score-als-Gate.** Eine numerische Kennzahl darf niemals über Abnahme entscheiden.
2. **V2 — Kein stilles Residuum.** Kein `B⁻x` wird absorbiert, verschwiegen, weggerundet oder in „Sonstiges“ versteckt.
3. **V3 — Kein Nullpunkt-Durchlauf.** Kein Pfad traversiert `Z0/Π0`.
4. **V4 — Kein Boundary-Crossing ohne Seam.** Kein Übergang über eine Grenze ohne Separator + Grenzvertrag.
5. **V5 — Keine Software-Reduktion.** Die Apparatur wird nicht auf „nur ein Programm“ reduziert; die topologisch-kristalline Semantik (Fiber/Skeleton/zwei Sweeps/Kristall-Übergabe) bleibt im Bau erhalten und geprüft.
6. **V6 — Kein Rohimport.** Kein Fremdmaterial ohne H/F-Gate.
7. **V7 — Keine Fail-open-Lücke.** Keine fehlende Prüfung wird durch Durchlassen überbrückt.
8. **V8 — Keine Prompt-/Prozess-Regression** auf dem Abschlusspfad (Determinismus, deterministische Operatorreihenfolge).
9. **V9 — Kein Prime-Power-als-Atom** bzw. keine Verwechslung von zusammengesetzter Struktur mit irreduziblem Kern (BCIK/CL Atomizitäts-Regel).
10. **V10 — Keine Überreichweiten-Behauptung** (Physik/RH/`π,ζ`-Operator) in Code, Test, Kommentar oder Doku.

### 8.5 Erweiterte Verifikationsbedingungen (VC1–VC10⁺)

Die BCIK-Verifikationsbedingungen VC1–VC10 werden hier zu **Bau-Verifikationsbedingungen** erweitert: Jede ist an ein konkretes Artefakt im Repository gebunden, das ihre Erfüllung nachweist.

| VC | Bedingung (BCIK) | Bau-Nachweis im Repo |
|----|------------------|----------------------|
| **VC1** | Kanonisierung idempotent | Property-Test `can_idempotent` je Zustandstyp (`tests/property/`) |
| **VC2** | Signatur wohlgeformt & erhalten | `Σsig`-Checker + Subject-Reduction-Test (`cce-core`, `tests/property/`) |
| **VC3** | Rand nichtleer & Boundary-Operatoren definiert | Schema-Constraint + `boundary_nonempty`-Test |
| **VC4** | `B⁻K=0` mit ausgewiesenem Residuum | `reanalyze`-Nachweis + Residuenfeld-Assertion (`tests/closure_roundtrip/`) |
| **VC5** | Alle Pflicht-Gates ausgeführt, sonst `Materialize=0` | Gate-Report-Vollständigkeitstest (`cce-runner`, `ci/`) |
| **VC6** | Trace vorhanden & konsistent | Ledger-Trace-Test, `verify_ledger` |
| **VC7** | Replay reproduziert kanonische Klasse | Replay-Test gleicher RD (`tests/replay/`) |
| **VC8** | Pfadinvarianz (reihenfolgeunabhängig mod ≡σ) | Konfluenz-Property-Test (`path_invariance`) |
| **VC9** | Import nur durch Gate, mit Residuum | Import-Gate-Test + Negative-Cube „Rohimport“ |
| **VC10** | Zertifikat korrekt & prüfbar | `Certificate`-Roundtrip: erzeugen → unabhängig prüfen (`cce-core`, `tests/`) |
| **VC1⁺** | **Closure-Roundtrip** grün für jeden Reference-Cube | **Kerntest** `equivalent(reanalyze(materialize(loom(project(encode(C))))),C)` |

`VC1⁺` ist die maschinelle Form der Abschlussformel und die **oberste** Bau-Verifikationsbedingung: Ist sie rot, ist der Bau per Definition unfertig, unabhängig vom Zustand aller anderen Artefakte.

### 8.6 Umgang mit Konflikt, Lücke und Nicht-Abschluss

Der Agent trifft auf drei wiederkehrende Situationen; die Verfassung schreibt für jede das Verhalten vor:

- **Konflikt zweier Dokumente/Rollen.** Auflösung ausschließlich nach Teil 2 §Rollenkonflikte (fünf bereits aufgelöste Identitäten) bzw. nach der CCC-Bracket-Regel (Teil 5): Das vereinheitlichende Kalkül CCC entscheidet, welche Sicht die kanonische ist. Der unterlegene Begriff wird nicht gelöscht, sondern als importierte Sicht mit Gate geführt.
- **Fehlende Spezifikation (Lücke).** Kein stilles Erfinden. Die Lücke wird als **Counter-Horizon-Eintrag** notiert (`docs/residues/`), mit der kleinsten verfassungskonformen Default-Entscheidung (Prinzipienreihenfolge P1–P12) und einem Test, der die Annahme fixiert. So bleibt die Lücke sichtbar und rückverfolgbar.
- **Nicht-Abschluss (CCC §6.5).** Erreicht ein endlicher Sweep `Fix(R)` nicht, wird **nicht** Abschluss vorgetäuscht: Es werden die a-priori-Schranke (Banach/QSNA), der verbleibende Abstand und der Counter-Horizon protokolliert. `ClosedCCE(C)` ist dann ehrlich `0` mit ausgewiesenem Grund — ein gültiger, verfassungskonformer Endzustand, der die Abschlussformel als **nicht erfüllt** ausweist statt sie zu fälschen.

### 8.7 Definition of Done (DoD) — der einzige Fertigstellungsbegriff

Der Bau — oder jede einzelne Domänen-Erweiterung — heißt **fertig** genau dann, wenn folgender Prädikatsblock als grüne CI erfüllt ist:

```
DoD(cce) = 1  ⟺
      ClosedCCE(C_ref) = 1            für jeden Reference-Cube C_ref
    ∧ VC1..VC10 erfüllt               (Tabelle 8.5)
    ∧ VC1⁺  grün                      (Closure-Roundtrip = Abschlussformel)
    ∧ INV-1..INV-14 als CI-Gates grün (§8.3)
    ∧ kein V1..V10 verletzt           (§8.4, Negative-Cubes rot-getestet = korrekt abgelehnt)
    ∧ AcceptLOOM ∧ ClosedPHC ∧ Kristall(CCC) ∧ BCIK   koinzident erfüllt (Teil 0 Theorem)
    ∧ jedes Residuum sichtbar         (P4/V2)
    ∧ jede Nicht-Erreichbarkeit als Counter-Horizon + Schranke ausgewiesen (§8.6, P12)
```

Wobei `ClosedCCE(C)=1` die in Teil 0 bewiesene Konjunktion ist:

```
ClosedCCE(C) = 1
  ⟺ Closed(C) ∧ QSR(C) ∧ Gate(C) ∧ Replay(C) ∧ ResidueVisible(C)
    ∧ q(Obs(Materialize(C))) = q(C)          (Beobachtungsäquivalenz)
    ∧ Reanalyze(Materialize(LOOM(Project(PHC(C))))) ≃ C   (Abschlussformel)
```

Es gibt **keinen anderen** Fertigstellungsbegriff. „Läuft durch“, „sieht gut aus“, „Score hoch“ sind explizit **keine** DoD (V1). Die Engine ist fertig, wenn sie ihren eigenen Ausgangskristall beweisbar und replaybar zurückgibt — und sonst nicht.

### 8.8 Governance des Agenten selbst

- **Selbstbindung.** Der Agent liest diese Verfassung als erste Handlung jedes Bauzyklus und prüft seine geplanten Änderungen gegen P1–P12, INV-1..14, V1–V10.
- **Änderung der Verfassung.** Die Verfassung selbst ist ein Monolith im Ledger: Änderungen sind append-only, begründet, mit Datum und Grund; keine stille Umschrift. Eine Lockerung eines Verbots (V1–V10) ist unzulässig, solange der Korpus sie trägt.
- **Kein Reichweiten-Drift.** Über lange Bauzyklen bleibt die Claim-Schranke (INV-14/V10) invariant; der Agent überschreitet sie nicht, auch nicht unter Nützlichkeitsdruck.
- **Abschlussbindung als Endzweck.** Jede Handlung des Agenten ist daran zu messen, ob sie den Kerntest (VC1⁺) grün macht oder grün hält. Handlungen, die das nicht tun und auch nicht vorbereiten, gehören nicht in den Bauzyklus.

---

## TEIL 9 — One-Shot-Implementierungsanweisung für einen Coding-Agenten

Dieser Teil ist der **ausführbare Auftrag**. Ein autonomer Coding-Agent liest ihn **einmal** und errichtet daraus `cce/` in einem zusammenhängenden Zug — aus einem Guss, ohne Rückfrage, ohne MVP-Zwischenstufe, ohne isolierte Module. Die Teile 0–8 sind die Spezifikation; dieser Teil ist die **Reihenfolge und die Abnahme**.

> **Oberste Regel für den gesamten Lauf:** Der Bau ist fertig genau dann, wenn `DoD(cce)=1` (§8.7). Insbesondere muss am Ende der Kerntest grün sein:
> ```
> assert equivalent( reanalyze(materialize(loom(project(encode(C))))), C )   # für jeden Reference-Cube
> ```
> Alles unten dient ausschließlich diesem Ziel.

### 9.0 Ausführungsvertrag des Agenten

1. Lies Teil 8 (Verfassung) vor jeder Phase erneut. Prüfe jede geplante Datei gegen P1–P12, INV-1..14, V1–V10.
2. Baue **eine** Domäne vollständig geschlossen zuerst (Empfehlung: `document` oder `graph`, weil klein und deterministisch), erreiche den grünen Kerntest, **dann** erst weitere Domänen (Prinzip P1).
3. Committe fail-closed: Kein Modul gilt als fertig, dessen Modul-Invariante (Tabelle 7.2) und zugehörige Gates nicht grün sind.
4. Schreibe **keinen** Code, der ein Verbot V1–V10 verletzt. Wo du versucht wärst, fixiere den Fall stattdessen als Negative-Cube (rot-getestet = korrekt abgelehnt).
5. Verwende die kanonische Sprache: **Rust** für den Kern (`cce-*` Crates), dünne Adapter darüber. Determinismus, Content-Addressing, byte-identischer Replay, fail-closed Gates sind Korpus-Pflicht und in Rust unmittelbar umsetzbar.
6. Halte den Abhängigkeitsgraphen azyklisch (INV-11) und die Kristall-Semantik erhalten (V5): Fiber/Skeleton/zwei Sweeps/Kristall-Übergabe sind reale Strukturen im Code, nicht Metaphern.

### 9.1 Build-Phasen (deterministische Reihenfolge)

Jede Phase endet mit einem **Phasen-Gate**. Ist es rot, wird die nächste Phase nicht begonnen.

**Phase A — Skelett & Verfassung materialisieren.**
- Lege die vollständige Verzeichnisstruktur aus Teil 6 an (`cce/` mit allen Crates, `agents/`, `schemas/`, `reference-cubes/`, `negative-cubes/`, `spec/`, `tests/`, `ci/`, `docs/`).
- Lege `spec/BAUVERFASSUNG.md` (diese Datei) als Monolith ab; `docs/residues/` initialisieren.
- Workspace-`Cargo.toml` mit allen Crates; Crate-DAG exakt nach Teil 6 (acyclic).
- **Phasen-Gate A:** `cargo build` grün (leere, aber typkorrekte Crate-Gerüste); Crate-DAG von einem Lint (`ci/check_acyclic`) als azyklisch bestätigt (INV-11).

**Phase B — Kern: Objektmodell & Kanonisierung (`cce-core`).**
- Implementiere die kanonischen Objekte aus Teil 3: `CanonicalState, NullAnchor, TripolarFiber, Reflection, Separator/Seam, JunctionTree, BoundaryContract, Marker, Response, Horizon/CounterHorizon, Crystal, MatrixCrystal, Monolith, Gate, GateReport, Evidence, Residue, RunDescriptor, Ledger, Certificate, Artifact` sowie `Wunsch-Normalform W=(X,H,K,G,Res,Π,τ,Replay,Goal,Materialize)`.
- Implementiere `Can` (Kanonisierung) je Typ mit `Can∘Can=Can`; `Σsig` (Signatur) mit Wohlgeformtheit/Erhaltung; content-addressierten Digest; append-only `Ledger` + `verify_ledger`.
- Implementiere die BCIK-Fiber `(B,B⁻,seam)` mit Closure-Gesetz `B⁻x=0` und **stets ausgewiesenem** Residuenfeld.
- **Phasen-Gate B:** VC1 (`can_idempotent`), VC2 (Signatur+Subject-Reduction), VC3 (Rand nichtleer), VC6 (Trace), VC10 (Zertifikat-Roundtrip), INV-1/2/3/8/12 grün.

**Phase C — Substrat: Hypercube & Constraint Lattice (`cce-lattice`).**
- Implementiere `Cube (G0–G3)`, `Cell`, Kopplungsgraph, **Chordalität-Prüfung**, **Junction-Tree**-Konstruktion mit Running-Intersection-Property; Propagationshülle `Γ_𝒦` als lfp (Knaster-Tarski: Monotonie/Extensivität/Idempotenz); Chameleon-Projektion.
- Materialisiere den Abnahmekatalog **CL K1–K18** als Testsuite.
- **Phasen-Gate C:** INV-6 (Chordalität=Adäquatheit, Running Intersection), Hüllen-Property-Tests, K1–K18 grün.

**Phase D — Bracket: Crystalline Closure Calculus (`cce-ccc`).**
- Implementiere Fiber+Skeleton+**zwei Sweeps** über der Junction-Tree-Struktur aus Phase C: **Collect** (Konstruktion `Bx`, Aufwickeln) und **Distribute** (Schließungsverteilung, Abwickeln), die sich **eine** Struktur teilen (P6, INV-7).
- Implementiere `Kristall ⟺ Closed ∧ QSR ∧ Gate ∧ Replay ∧ ResidueVisible`; Konfluenz mod ≡σ; Commit-Klasse `[x]∈X/≡σ`; Nicht-Erreichbarkeits-Behandlung (§8.6, CCC §6.5) mit a-priori-Schranke + Counter-Horizon.
- Materialisiere **CCC C1–C14**.
- **Phasen-Gate D:** INV-7 (Zwei-Sweep-Korrektheit), VC8 (Pfadinvarianz/Konfluenz), INV-9/10, C1–C14 grün.

**Phase E — Übergabe-Token & Transport: Crystal + PHC (`cce-crystal`, `cce-phc`).**
- `cce-crystal`: Kristall als Übergabe-Token an jedem Separator; `MatrixCrystal=(A,H,Tri,Gates,Residues,NullModels,Trace,ReplayHash)` als Teilmenge/Spezialform von Crystal.
- `cce-phc`: `PHC=(M,T,A,C,W,P,G,R,L,E)`, `PHC-CANON-0.1`, Loader `load_phc→canonicalize→assert root_hash→validate(schema,refs,axes,cells,gates,residue)→build_lattice`, Projektion `project→apply_projection→assert gate_scope`, `commit_result→run_gates→collect_residue→append_ledger→seal_if_passed`.
- Implementiere `phc.schema.json` (Teil 7) und die sieben Pflicht-Gates-Bindung; Profile CORE/LOOM/MERKABA; Konformitätsstufen inkl. `PHC-Portable`.
- Materialisiere **PHC V0–V9**.
- **Phasen-Gate E:** `ClosedPHC(P,A)=1 ⟺ Can(P)∧Gate(A)∧Replay(A)∧Visible(Res(A))∧q(Obs(A))=q(C_P)`; V0–V9; VC5 (Gate-Vollständigkeit) grün.

**Phase F — Generativer Halbraum: LOOM + Materialisierung (`cce-loom`, `cce-materialize`).**
- `cce-loom`: radiale Spindel, `Nullanker Z0` (markiert, nie durchlaufen — P7/V3), Mandorla, Nadelapertur; `loom_generate`-Referenzalgorithmus; `AcceptLOOM(C,A)=1 ⟺ BCIK(A) ∧ Obs(A)≃C ∧ Res(A) sichtbar`.
- **Implementiere die Identität Radfenster = Nadelapertur** (Teil 5, I-7) als **geteilten** Code-Pfad mit `cce-observe`: dieselbe Struktur wirkt als Apertur (Collect) und als Nadel (Distribute). Dies ist der mechanische Kern von `≃` und darf **nicht** dupliziert werden.
- `cce-materialize`: Migration/Materialisierung des geschlossenen Kristalls in ein Domänen-Artefakt (Datei, Graph, Dokument, Modul), mit Gate-Durchlauf und Residuen-Sammlung.
- Materialisiere **LOOM Abnahmekatalog (10 Tests)**.
- **Phasen-Gate F:** `AcceptLOOM` grün; Materialisierung erzeugt nur bei `B⁻x=0` (INV-4); LOOM-Abnahme (10) grün.

**Phase G — Beobachtender Halbraum & Re-Analyse (`cce-observe`).**
- Implementiere die beobachtende Instrumentenoptik/Triangulation (TP/PIO/TAT/QLOGIC/DZ als Rollen-Importe, **nicht** als getrennte Subsysteme): optische Normalform `X0→Apt→…→Gate→XC`; `TAT=MatrixCrystal∘Gate∘Triangulate∘Horizon∘Respond∘Mark∘Embed`.
- Implementiere **`reanalyze` (Collect-Pfad)**: `Embed→Mark→Respond→Horizon→Triangulate→Gate→Crystalize(MatrixCrystal)` — teilt die Skeleton-Struktur mit Phase D und die Radfenster-Mechanik mit Phase F.
- Materialisiere **TAT P1–P7**, QLOGIC-/DZ-Rollen-Tests.
- **Phasen-Gate G:** Reanalyze erzeugt aus einem materialisierten Artefakt wieder einen Kristall; `q(Obs(Materialize(C)))=q(C)` (Beobachtungsäquivalenz) grün; P1–P7 grün.

**Phase H — Monolith & Orchestrierung (`cce-merkaba`, `cce-runner`).**
- `cce-merkaba`: Monolith-Fassung `F=Commit∘C∘G∘Δ∘I∘Q∘Θ∘P∘Can`; Organe `O=(X_O,Y_O,α_O,β_O,σ_O,γ_O)` als deterministische Kompositionsebene über den Crates.
- `cce-runner`: End-to-End-Pipeline, `RunDescriptor`-Bindung, Ledger-Anbindung, Gate-Report-Aggregation, Replay-Manifest (`replay_index.json`).
- **Phasen-Gate H:** VC7 (Replay reproduziert kanonische Klasse), INV-10 (Replay-Identität) grün; Runner führt den vollständigen Abschlusspfad deterministisch aus.

**Phase I — Abschluss-Nachweis (Kerntest) & Härtung.**
- Lege für die zuerst gewählte Domäne einen **perfekten Reference-Cube** in `reference-cubes/` an, der `ClosedCCE(C)=1` erfüllt.
- Implementiere `tests/closure_roundtrip/` mit dem Kerntest **VC1⁺**.
- Fülle `negative-cubes/` mit je einem Fall pro Verbot V1–V10 (Score-als-Gate, stilles Residuum, Nullpunkt-Durchlauf, Boundary ohne Seam, Software-Reduktion, Rohimport, Fail-open, Prompt-Regression, Prime-Power-Atom, Überreichweiten-Claim) — jeder MUSS abgelehnt werden.
- Materialisiere **BCIK VC1–VC10** vollständig; verdrahte **alle** Invarianten INV-1..14 als CI-Gates in `ci/`.
- **Phasen-Gate I (= Haupt-Abnahme):** `DoD(cce)=1` nach §8.7 — insbesondere **VC1⁺ grün**.

**Phase J — Breite (weitere Domänen).**
- Erst jetzt (P1): weitere Domänen-Adapter (math/software/document/graph, je nachdem was in Phase I noch fehlt), jeweils mit eigenem Reference-Cube und grünem Kerntest, ohne den Kern zu verändern (nur über deklarierte Ports).
- **Phasen-Gate J:** `DoD` bleibt `1` für **alle** Reference-Cubes; kein Kern-Crate wurde für eine Domäne aufgeweicht (V5).

### 9.2 Abnahme-Reihenfolge (was zuerst grün sein muss)

```
A(build+acyclic)  →  B(core:VC1,2,3,6,10)  →  C(lattice:K1–K18, RIP)
      →  D(ccc:2-sweep, konfluenz, C1–C14)  →  E(crystal+phc:ClosedPHC, V0–V9)
      →  F(loom+materialize:AcceptLOOM, Radfenster=Nadel)  →  G(observe:reanalyze, Obs-Äquiv, P1–P7)
      →  H(merkaba+runner:Replay, VC7)  →  I(KERNTEST VC1⁺ = Abschlussformel, DoD=1)
      →  J(weitere Domänen, DoD bleibt 1)
```

Der Übergang I ist der Moment, in dem die Abschlussformel vom Papier in den grünen Test übergeht. Vor I ist der Bau **unfertig** (per Definition), nach J ist er **domänenagnostisch geschlossen**.

### 9.3 Minimaler Kern-Kontrakt, den der erste grüne Kerntest berührt

Damit der Agent den kürzesten geschlossenen Pfad zuerst trifft, hier die **eine** Kette, die für die zuerst gewählte Domäne durchgehend funktionieren muss (alle Glieder aus Teil 4 §Formel↔Operator):

```
encode      : Crystal            → PHC(Crystal)                     (cce-phc)
project     : PHC(Crystal)       → Projektion                       (cce-phc)
loom        : Projektion         → Gewebe/Weave (radial, Nullanker) (cce-loom)
materialize : Weave              → Artefakt (Domäne)                (cce-materialize)
reanalyze   : Artefakt           → Crystal'                         (cce-observe)
equivalent  : (Crystal', Crystal)→ bool  (≃ , kanonische Klasse)    (cce-core/cce-ccc)
```

mit den drei nicht verhandelbaren Kontrakten auf dieser Kette: (i) jede Stufe kanonisiert vor Operation (P2); (ii) `materialize` liefert nur bei `B⁻x=0` mit ausgewiesenem Residuum (P4/INV-4); (iii) `loom` und `reanalyze` teilen die Radfenster/Nadelapertur-Struktur (I-7, V5). Ist diese Kette grün, ist `VC1⁺` erfüllbar; ist sie es nicht, benennt das rote Glied exakt die offene Baustelle.

### 9.4 Schluss der Bauverfassung

Diese Bauverfassung ist damit **geschlossen** im eigenen Sinn: Sie beginnt mit der Abschlussformel als Bauachse (Teil 0), entfaltet Register, Rollen, Objekte, Operatoren, Architektur, Repository, Schemas/Gates/Tests/Residuen/Replay (Teile 1–7), bindet den bauenden Agenten an die Master-Normalform (Teil 8) und führt ihn in deterministischer Reihenfolge bis zu dem grünen Test, der die Abschlussformel maschinell verkörpert (Teil 9). 

Kein Baustein bleibt isoliert (jeder ist in Teil 2 einer Rolle, in Teil 6 einer Datei, in Teil 7 einem Gate zugeordnet); kein Residuum bleibt still (P4/V2 überall); keine Reichweite wird überschritten (INV-14/V10); und die einzige Definition von „fertig“ ist:

```
Reanalyze(Materialize(LOOM(Project(PHC(Crystal))))) ≃ Crystal    —    grün, geprüft, replaybar.
```

*Ende der Bauverfassung.*
