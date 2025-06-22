fn main() {
    let pattern = std::env::args()
        .nth(1)
        .expect("No argument provided.\n --help for usage.");
    let path = std::env::args().nth(2).expect("No file path given.");
    println!("Pattern: {:?}, Path: {:?}", pattern, path);
}
