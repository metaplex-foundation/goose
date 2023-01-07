use anyhow::Result;
use clap::Parser;

use goose::{args, processor::*};

fn main() -> Result<()> {
    solana_logger::setup_with_default("solana=error");

    let args = args::Args::parse();

    match args.command {
        args::Commands::Init {
            collection_mint,
            unlock_method,
            size,
        } => process_initialize(collection_mint, unlock_method, size),
        args::Commands::Cancel { collection_mint } => process_close(collection_mint),
    }
}
