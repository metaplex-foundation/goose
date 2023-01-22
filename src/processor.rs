use std::{fs::File, path::PathBuf, str::FromStr};

use anyhow::Result;
use borsh::BorshDeserialize;
use console::style;
use mpl_migration_validator::{
    state::{MigrationState, UnlockMethod},
    utils::find_migration_state_pda,
};
use serde::{Deserialize, Serialize};
use solana_program::{
    bpf_loader_upgradeable::UpgradeableLoaderState, program_pack::Pack, pubkey::Pubkey,
};
use solana_sdk::{signer::Signer, transaction::Transaction};
use spl_token::state::Account as TokenAccount;

use crate::{
    methods::{
        close, get_state, initialize, initialize_msg, migrate_item, start, update, CloseParams,
        GetStateParams, InitializeMsgParams, InitializeParams, MigrateParams, StartParams,
        UpdateParams,
    },
    setup,
    utils::{get_cluster, get_nft_token_account, spinner_with_style},
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
    spinner.finish();

    let get_state_params = GetStateParams {
        client: &config.client,
        collection_mint,
    };
    spinner.set_message("Fetching migration state...");
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

pub fn process_initialize_signer(keypair: Option<PathBuf>, rpc_url: Option<String>) -> Result<()> {
    let config = setup::CliConfig::new(keypair, rpc_url)?;

    let instruction = mpl_migration_validator::instruction::init_signer(config.keypair.pubkey());
    let spinner = spinner_with_style();
    spinner.set_message("Initializing program signer...");
    let recent_blockhash = config.client.get_latest_blockhash()?;

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&config.keypair.pubkey()),
        &[&config.keypair],
        recent_blockhash,
    );

    let sig = config.client.send_and_confirm_transaction(&transaction)?;
    spinner.finish();
    println!(
        "Initialized program signer successfully in tx: {}",
        style(sig).green()
    );

    Ok(())
}

pub fn process_close(
    keypair: Option<PathBuf>,
    rpc_url: Option<String>,
    collection_mint: Pubkey,
) -> Result<()> {
    let config = setup::CliConfig::new(keypair, rpc_url)?;

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

pub fn process_get_all_states(keypair: Option<PathBuf>, rpc_url: Option<String>) -> Result<()> {
    let config = setup::CliConfig::new(keypair, rpc_url)?;

    // Get all the program accounts for mpl-migration-validator.
    let account_results = config
        .client
        .get_program_accounts(&mpl_migration_validator::ID)?;

    let cluster = get_cluster(&config.client)?;

    println!(
        "Found: {}",
        style(format!("{} states", account_results.len())).green()
    );

    let file_name = format!("{}_migration_states.json", cluster);

    let mut states = Vec::new();

    for (_pubkey, account) in account_results {
        let state =
            <MigrationState as BorshDeserialize>::deserialize(&mut account.data.as_slice())?;
        states.push(state);
    }

    let f = File::create(&file_name)?;
    serde_json::to_writer_pretty(f, &states)?;

    println!(
        "{}",
        style(format!("Wrote migration states to {file_name}")).green()
    );

    Ok(())
}

pub fn process_update(
    keypair: Option<PathBuf>,
    rpc_url: Option<String>,
    collection_mint: Pubkey,
    rule_set: Option<Pubkey>,
    collection_size: Option<u32>,
) -> Result<()> {
    let config = setup::CliConfig::new(keypair, rpc_url)?;

    let (migration_state, _) = find_migration_state_pda(&collection_mint);

    let params = UpdateParams {
        client: &config.client,
        authority: &config.keypair,
        migration_state,
        collection_size,
        rule_set,
    };
    let spinner = spinner_with_style();
    spinner.set_message("Updating migration state...");
    let sig = update(params)?;
    spinner.finish();

    let cluster = get_cluster(&config.client)?;
    let link = format!("https://explorer.solana.com/tx/{}?cluster={cluster}", sig);
    println!(
        "Updated migration state successfully in tx: {}",
        style(link).green()
    );

    Ok(())
}

pub fn process_start(
    keypair: Option<PathBuf>,
    rpc_url: Option<String>,
    collection_mint: Pubkey,
) -> Result<()> {
    let config = setup::CliConfig::new(keypair, rpc_url)?;

    let params = StartParams {
        client: &config.client,
        authority: &config.keypair,
        collection_mint,
    };

    let spinner = spinner_with_style();
    spinner.set_message("Enabling migration...");
    let sig = start(params)?;
    spinner.finish();

    let cluster = get_cluster(&config.client)?;
    let link = format!("https://explorer.solana.com/tx/{}?cluster={cluster}", sig);
    println!(
        "Started migration successfully in tx: {}",
        style(link).green()
    );

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MigratedMint {
    sig: String,
    item_mint: String,
}

pub fn process_migrate(
    keypair: Option<PathBuf>,
    rpc_url: Option<String>,
    collection_mint: Pubkey,
    mint_list: PathBuf,
) -> Result<()> {
    let config = setup::CliConfig::new(keypair, rpc_url)?;

    let f = File::open(mint_list)?;
    let mints: Vec<String> = serde_json::from_reader(f)?;
    let mints: Vec<Pubkey> = mints
        .into_iter()
        .map(|s| Pubkey::from_str(&s).unwrap())
        .collect();

    let migrate_state = get_state(GetStateParams {
        client: &config.client,
        collection_mint,
    })?;

    let rule_set = migrate_state.collection_info.rule_set;

    let mut completed_mints = Vec::new();

    let spinner = spinner_with_style();
    spinner.set_message("Migrating...");
    for item_mint in mints {
        let item_token = get_nft_token_account(&config.client, item_mint)?;
        let account = config.client.get_account(&item_token)?;
        let token_account = TokenAccount::unpack(&account.data)?;
        let token_owner = token_account.owner;
        let token_owner_program = config.client.get_account(&token_owner)?.owner;
        let token_owner_program_info = config.client.get_account(&token_owner_program)?;

        // We need to pass the program data buffer to the migration program
        // if the token owner program is an upgradeable program.
        let state_opt: Option<UpgradeableLoaderState> =
            bincode::deserialize(&token_owner_program_info.data).ok();

        let token_owner_program_buffer = if let Some(state) = state_opt {
            match state {
                UpgradeableLoaderState::Program {
                    programdata_address,
                } => Some(programdata_address),
                _ => None,
            }
        } else {
            None
        };

        let params = MigrateParams {
            client: &config.client,
            payer: &config.keypair,
            item_mint,
            item_token,
            token_owner,
            token_owner_program,
            token_owner_program_buffer,
            collection_mint,
            rule_set,
        };

        let sig = migrate_item(params)?;
        completed_mints.push(MigratedMint {
            sig: sig.to_string(),
            item_mint: item_mint.to_string(),
        });

        let cluster = get_cluster(&config.client)?;
        let link = format!("https://explorer.solana.com/tx/{}?cluster={cluster}", sig);
        println!("Migrated successfully in tx: {}", style(link).green());
    }
    spinner.finish();

    let file_name = format!("{}_migrated_mints.json", collection_mint);
    let f = File::create(file_name)?;
    serde_json::to_writer_pretty(f, &completed_mints)?;

    Ok(())
}

pub fn process_sudo(
    keypair: Option<PathBuf>,
    rpc_url: Option<String>,
    collection_mint: Pubkey,
    ts: i64,
) -> Result<()> {
    let config = setup::CliConfig::new(keypair, rpc_url)?;

    let (migration_state, _) = find_migration_state_pda(&collection_mint);

    let instruction =
        mpl_migration_validator::instruction::sudo(config.keypair.pubkey(), migration_state, ts);

    let recent_blockhash = config.client.get_latest_blockhash()?;

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&config.keypair.pubkey()),
        &[&config.keypair],
        recent_blockhash,
    );

    let sig = config.client.send_and_confirm_transaction(&transaction)?;
    println!("Tx: {}", style(sig).green());

    Ok(())
}
