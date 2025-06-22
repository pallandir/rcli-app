struct CliArgs {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let pattern = std::env::args()
        .nth(1)
        .expect("No argument provided.\n --help for usage.");
    let path = std::env::args().nth(2).expect("No file path given.");
    let args = CliArgs {
        pattern,
        path: std::path::PathBuf::from(path),
    };
    println!("Pattern: {:?}, Path: {:?}", args.pattern, args.path);
}
