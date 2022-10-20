use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use decc::{Options, DEFAULT_PATTERN};
use regex::Regex;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Input file.
    input_file: PathBuf,
    /// Output file (optional).
    output_file: Option<PathBuf>,
    /// Encoding to use.
    #[clap(long, short)]
    encoding: Option<String>,
    /// Force parsing even if encoding is incorrect.
    #[clap(long, short)]
    force: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let options = Options {
        output_file: cli.output_file.unwrap_or_else(|| cli.input_file.clone()),
        input_file: cli.input_file,
        encoding: cli.encoding,
        force: cli.force,
        pattern: Regex::new(DEFAULT_PATTERN)?,
    };

    decc::deccify_file(options)
}
