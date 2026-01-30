fn main() {
    if let Err(error) = dnap::cli::run_from_env() {
        eprintln!("dna: {error}");
        std::process::exit(1);
    }
}
