use std::path::PathBuf;

use anyhow::Result;
use console::style;
use mpl_migration_validator::state::UnlockMethod;
use solana_program::pubkey::Pubkey;
use solana_sdk::signer::Signer;

use crate::{
    methods::{
        close, get_state, initialize, initialize_msg, CloseParams, GetStateParams,
        InitializeMsgParams, InitializeParams,
    },
    setup,
    utils::{get_cluster, spinner_with_style},
};

pub fn process_initialize(
    keypair: Option<PathBuf>,
    rpc_url: Option<String>,
    collection_mint: Pubkey,
    unlock_method: String,
    collection_size: u32,
) -> Result<()> {
    let config = setup::CliConfig::new(keypair, rpc_url)?;

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

    let get_state_params = GetStateParams {
        client: &config.client,
        collection_mint,
    };
    let state = get_state(get_state_params)?;
    spinner.finish();

    println!("Migration state:\n {:#?}", style(state).green());

    Ok(())
}

pub fn process_initialize_msg(
    payer: Pubkey,
    authority: Pubkey,
    collection_mint: Pubkey,
    unlock_method: String,
    collection_size: u32,
) -> Result<()> {
    let unlock_method = match unlock_method.to_lowercase().as_str() {
        "timed" => UnlockMethod::Timed,
        "vote" => UnlockMethod::Vote,
        _ => {
            return Err(anyhow::anyhow!(
                "Invalid unlock method. Must be one of: Timed, Vote"
            ))
        }
    };

    let params = InitializeMsgParams {
        payer,
        authority,
        rule_set: None,
        collection_mint,
        unlock_method,
        collection_size,
    };
    let spinner = spinner_with_style();
    spinner.set_message("Initializing migration state...");
    let message = initialize_msg(params)?;
    spinner.finish();

    println!("Transaction message:\n {:#?}", style(message).green());

    Ok(())
}

pub fn process_close(
    keypair: Option<PathBuf>,
    rpc_url: Option<String>,
    collection_mint: Pubkey,
) -> Result<()> {
    let config = setup::CliConfig::new(keypair, rpc_url)?;

    println!("keypair: {}", style(&config.keypair.pubkey()).green());

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

pub fn process_get_state(
    keypair: Option<PathBuf>,
    rpc_url: Option<String>,
    collection_mint: Pubkey,
) -> Result<()> {
    let config = setup::CliConfig::new(keypair, rpc_url)?;

    let get_state_params = GetStateParams {
        client: &config.client,
        collection_mint,
    };
    let state = get_state(get_state_params)?;

    println!("Migration state:\n {:#?}", style(state).green());

    Ok(())
}
