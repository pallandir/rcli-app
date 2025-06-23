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

    for line in reader.lines() {
        println!("Line: {:?}", line);
    }
    Ok(())
}
