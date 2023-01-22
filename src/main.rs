use anyhow::Result;
use clap::Parser;

use goose::{args, processor::*};

fn main() -> Result<()> {
    solana_logger::setup_with_default("solana=error");

    let args = args::Args::parse();

    let keypair_path = args.keypair_path.clone();
    let rpc_url = args.rpc_url.clone();

    match args.command {
        args::Commands::Init {
            collection_mint,
            unlock_method,
            size,
        } => process_initialize(keypair_path, rpc_url, collection_mint, unlock_method, size),
        args::Commands::InitMsg {
            payer,
            authority,
            collection_mint,
            unlock_method,
            size,
        } => process_initialize_msg(payer, authority, collection_mint, unlock_method, size),
        args::Commands::Cancel { collection_mint } => {
            process_close(keypair_path, rpc_url, collection_mint)
        }
        args::Commands::GetState { collection_mint } => {
            process_get_state(keypair_path, rpc_url, collection_mint)
        }
    }
}
