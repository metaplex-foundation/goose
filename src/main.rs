use anyhow::Result;
use clap::Parser;

use goose::{
    args::{self, Commands},
    processor::*,
};

fn main() -> Result<()> {
    solana_logger::setup_with_default("solana=error");

    let args = args::Args::parse();

    let keypair_path = args.keypair_path.clone();
    let rpc_url = args.rpc_url.clone();

    match args.command {
        Commands::Init {
            collection_mint,
            unlock_method,
            size,
        } => process_initialize(keypair_path, rpc_url, collection_mint, unlock_method, size),
<<<<<<< Updated upstream
        args::Commands::Cancel { collection_mint } => {
=======
        Commands::InitMsg {
            payer,
            authority,
            collection_mint,
            unlock_method,
            size,
        } => process_initialize_msg(payer, authority, collection_mint, unlock_method, size),
        Commands::Cancel { collection_mint } => {
>>>>>>> Stashed changes
            process_close(keypair_path, rpc_url, collection_mint)
        }
        Commands::GetState { collection_mint } => {
            process_get_state(keypair_path, rpc_url, collection_mint)
        }
        Commands::GetAllStates => process_get_all_states(keypair_path, rpc_url),
        Commands::Update {
            collection_mint,
            rule_set,
            size,
        } => process_update(keypair_path, rpc_url, collection_mint, rule_set, size),
        Commands::Start { collection_mint } => {
            process_start(keypair_path, rpc_url, collection_mint)
        }
        Commands::Migrate {
            collection_mint,
            mint_list,
        } => process_migrate(keypair_path, rpc_url, collection_mint, mint_list),
    }
}
