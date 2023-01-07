use anyhow::Result;
use console::style;
use mpl_migration_validator::state::UnlockMethod;
use solana_program::pubkey::Pubkey;

use crate::{
    methods::{close, get_state, initialize, CloseParams, GetStateParams, InitializeParams},
    setup,
    utils::{find_migrate_state_pda, get_cluster, spinner_with_style},
};

pub fn process_initialize(
    collection_mint: Pubkey,
    unlock_method: String,
    collection_size: u32,
) -> Result<()> {
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
        collection_size,
    };
    let spinner = spinner_with_style();
    spinner.set_message("Initializing migration state...");
    let sig = initialize(params)?;
    spinner.finish();

    let cluster = get_cluster(&config.client)?;
    let link = format!("https://explorer.solana.com/tx/{}?cluster={cluster}", sig);
    println!(
        "Intialized migration state successfully in tx: {}",
        style(link).green()
    );

    // Delay before fetching the state.
    let spinner = spinner_with_style();
    spinner.set_message("Waiting for migration state to be initialized...");
    std::thread::sleep(std::time::Duration::from_secs(3));

    let pubkey = find_migrate_state_pda(collection_mint).0;

    let get_state_params = GetStateParams {
        client: &config.client,
        migration_state: &pubkey,
    };
    let state = get_state(get_state_params).unwrap();
    spinner.finish();

    println!("Migration state:\n {:#?}", style(state).green());

    Ok(())
}

pub fn process_close(collection_mint: Pubkey) -> Result<()> {
    let config = setup::CliConfig::new()?;

    let params = CloseParams {
        client: &config.client,
        authority: &config.keypair,
        collection_mint,
    };
    let spinner = spinner_with_style();
    spinner.set_message("Canceling migration...");
    let sig = close(params)?;
    spinner.finish();

    let cluster = get_cluster(&config.client)?;
    let link = format!("https://explorer.solana.com/tx/{}?cluster={cluster}", sig);
    println!(
        "Canceled migration successfully in tx: {}",
        style(link).green()
    );

    Ok(())
}
