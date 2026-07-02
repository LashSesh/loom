Stand: 2026-07-02 · Letzte abgeschlossene Phase: G2 (Nachweis: reports/G00–G02_bericht.md;
`cargo build --workspace` grün; `cargo test --workspace` = 101 Tests grün in 100 Suiten;
CI-Pipeline `cce/ci/run_ci.sh` vollständig grün inkl. Integritätscheck 24/24, clippy -D warnings,
check_acyclic)
Aktuelle Phase: G3 (Transport, Weben, Materialisieren + Motor-Kerntest) · Offene
Ausgangs-Gate-Punkte: Motor-Kerntest ≃ · Dokument-Adapter (11 Punkte) · 7 Dokument-Gates
grün/rot an Referenz-/Negativ-Cubes · Zwei-Digest-Trennung
Halbfertige Einheiten: crates/cce-core/src/wheel_window.rs (WheelWindow, Identität I-7):
fortführen — soeben angelegt, noch nicht in lib.rs registriert; alle übrigen G3-Crates
(cce-phc, cce-loom, cce-observe, cce-merkaba, cce-materialize) sind leere G0-Gerüste: fortführen.
Abweichungen vom Spec-Stand: keine (siehe reports/residuen.md R-Agent-1..4 für sichtbare
Auslegungen). Der Overlay `05_RESUME_INFERENCE_AMENDMENT.md` wurde ins Repo-Root übernommen
und gilt ab jetzt als Leseordnungs-Position 25a; Konsequenzen:
- Neue Phase G8a (cce-inference, cce-toolgateway) nach G8 — als Task eingeplant.
- Egress-Vierteilung (Teil B) gilt ab sofort als Bau-Invariante.
- G9 nimmt Kinds 0x0060–0x0064 + Zeugen R8/N15/N16 auf (24 .loom-Zeugen).
- G10 verdrahtet S3-A7..A10, COCK-INV-7/8, schreiblose Kanzel-API.
- MASTER-DoD gilt in der F.3-Fassung (Matrix a–i, PROD-INV-9..20, 19 Inference-Zeugen).
