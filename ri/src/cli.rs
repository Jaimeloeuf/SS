use clap::Parser;

/// CLI data structure
// @todo Include a verbose flag https://crates.io/crates/clap-verbosity-flag
#[derive(Parser)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
pub struct Cli {
    /// The path of the entry SimpleScript file to execute
    pub file_path: String,
}
