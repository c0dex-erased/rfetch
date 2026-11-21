mod cli;
mod colors;
mod config;
mod logo;
mod modules;
mod system;

fn main() {
    if let Err(error) = cli::run() {
        eprintln!("rfetch: {error}");
        std::process::exit(1);
    }
}
