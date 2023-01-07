use anyhow::Result;
use clap::Parser;
use console::style;
use mpl_migration_validator::state::UnlockMethod;
use solana_program::pubkey::Pubkey;

use goose::{
    args,
    methods::{get_state, initialize, GetStateParams, InitializeParams},
    setup,
    utils::find_migrate_state_pda,
};

fn main() -> Result<()> {
    solana_logger::setup_with_default("solana=error");

    let args = args::Args::parse();

    match args.command {
        args::Commands::Initialize {
            collection_mint,
            unlock_method,
        } => process_initialize(collection_mint, unlock_method),
    }
}

fn process_initialize(collection_mint: Pubkey, unlock_method: String) -> Result<()> {
    let config = setup::CliConfig::new()?;

    let unlock_method = match unlock_method.to_lowercase().as_str() {
        "timed" => UnlockMethod::Timed,
        "vote" => UnlockMethod::Vote,
        _ => {
            return Err(anyhow::anyhow!(
                "Invalid unlock method. Must be one of: Timed, Vote"
            ))
        }
    };

    let params = InitializeParams {
        client: &config.client,
        payer: &config.keypair,
        authority: &config.keypair,
        rule_set: None,
        collection_mint,
        unlock_method,
    };
    let sig = initialize(params)?;
    println!("Intialized migration state: {}", style(sig).green());

    let pubkey = find_migrate_state_pda(collection_mint).0;

    let get_state_params = GetStateParams {
        client: &config.client,
        migration_state: &pubkey,
    };
    let state = get_state(get_state_params).unwrap();

    println!("Migration state: {:?}", style(state).green());

    Ok(())
}
