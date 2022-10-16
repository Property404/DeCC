use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Input file.
    input_file: PathBuf,
    /// Output file (optional).
    output_file: Option<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let output_file = cli.output_file.unwrap_or_else(|| cli.input_file.clone());

    decc::deccify_file(cli.input_file, output_file, None)
}
