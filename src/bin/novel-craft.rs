fn main() {
    if let Err(error) = novel_craft::main() {
        eprintln!("{error:#}");
        std::process::exit(1);
    }
}
