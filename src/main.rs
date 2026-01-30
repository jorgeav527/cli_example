use std::io::BufRead;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let file = std::fs::File::open(&args.path).expect("could not be opened");
    let content = std::io::BufReader::new(file);
//     let content = std::fs::read_to_string(&args.path).expect("could not read the file");
    for line in content.lines() {
        let line = line.expect("could not read line");
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
