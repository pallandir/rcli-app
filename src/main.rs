use clap::Parser;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

#[derive(Parser)]
struct CliArgs {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = CliArgs::parse();
    let file = File::open(&args.path)?;
    let reader = BufReader::new(file);
    println!("Pattern: {:?}, Path: {:?}", args.pattern, args.path);
    find_pattern(reader, args.pattern);
    Ok(())
}

fn find_pattern(reader: BufReader<File>, pattern: String) {
    for line in reader.lines() {
        match line {
            Ok(line) => {
                if line.contains(&pattern) {
                    println!("Line: {:?}", line);
                }
            }
            Err(e) => eprintln!("Failed to read line: {}", e),
        }
    }
}
