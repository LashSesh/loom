fn main() {
    let path = match std::env::args().nth(1) {
        Some(p) => p,
        None => {
            eprintln!("verwendung: loom-viewer <datei.loom>");
            std::process::exit(2);
        }
    };
    let bytes = match std::fs::read(&path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("lesefehler: {e}");
            std::process::exit(1);
        }
    };
    match loom_viewer::render_views(&bytes) {
        Ok(views) => {
            for v in views {
                println!("{v}");
            }
        }
        Err(e) => {
            eprintln!("reject: {e}");
            std::process::exit(1);
        }
    }
}
