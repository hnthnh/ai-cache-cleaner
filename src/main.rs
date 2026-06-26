mod cleaner;
mod cli;
mod display;
mod error;
mod locations;
mod scanner;

/// CLI Application entry point.
fn main() {
    // Dispatch and run CLI commands, exit with status code 1 on errors
    if let Err(e) = cli::run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
