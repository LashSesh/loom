fn main() {
    match nexus_cli::reference_run() {
        Ok(s) => {
            for (gate, ok) in &s.gates {
                println!("{} {}", if *ok { "PASS" } else { "HOLD" }, gate);
            }
            println!("CSUs: {}", s.csu_count);
        }
        Err(e) => {
            eprintln!("HOLD {e}");
            std::process::exit(1);
        }
    }
}
