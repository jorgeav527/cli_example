use std::io::BufRead;
use std::fs::File;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let file = File::open(&args.path)?;
    let reader = std::io::BufReader::new(file);

    for line in reader.lines() {
        let text = line?;
        if text.contains(&args.pattern) {
            println!("Fonded: {}", text);
        }
    }
    Ok(())
}
