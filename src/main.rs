use std::io::BufRead;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let file_result = std::fs::File::open(&args.path);
    match file_result {
        Ok(file) => {
            let content = std::io::BufReader::new(file);
            for line_result in content.lines() {
                match line_result {
                    Ok(text) => {
                        if text.contains(&args.pattern) {
                            println!("Fonded: {}", text);
                        }
                    }
                    Err(error) => {
                        println!("Error reading the line {}", error);
                    }
                }
            }
        }
        Err(error) => { panic!("Can't deal with {}, just exit here", error); }
    };
}
