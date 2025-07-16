// src/main.rs
/*
 * Main executable for SynapticEngine
 */

use clap::Parser;
use synapticengine::{Result, run};

#[derive(Parser)]
#[command(version, about = "SynapticEngine - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
