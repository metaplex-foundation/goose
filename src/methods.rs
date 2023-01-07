use anyhow::Result;
use borsh::BorshDeserialize;
use mpl_migration_validator::{
    instruction::InitializeArgs,
    state::{MigrationState, UnlockMethod},
};
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_sdk::{
    signature::{Keypair, Signature, Signer},
    transaction::Transaction,
};

use crate::utils::{find_metadata_pda, find_migrate_state_pda};

pub struct InitializeParams<'a> {
    pub client: &'a RpcClient,
    pub payer: &'a Keypair,
    pub authority: &'a Keypair,
    pub rule_set: Option<Pubkey>,
    pub collection_mint: Pubkey,
    pub unlock_method: UnlockMethod,
}

pub fn initialize(params: InitializeParams) -> Result<Signature> {
    let InitializeParams {
        client,
        payer,
        authority,
        rule_set,
        collection_mint,
        unlock_method,
    } = params;

    let collection_metadata = find_metadata_pda(&collection_mint).0;
    let migrate_state_pubkey = find_migrate_state_pda(collection_mint).0;

    let args = InitializeArgs {
        rule_set: Some(rule_set.unwrap_or_default()),
        unlock_method,
        collection_size: 0,
    };

    let instruction = mpl_migration_validator::instruction::initialize(
        payer.pubkey(),
        authority.pubkey(),
        collection_mint,
        collection_metadata,
        migrate_state_pubkey,
        args,
    );

    let recent_blockhash = client.get_latest_blockhash()?;

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[payer, authority],
        recent_blockhash,
    );

    let sig = client.send_and_confirm_transaction(&transaction)?;

    Ok(sig)
}

pub struct GetStateParams<'a> {
    pub client: &'a RpcClient,
    pub migration_state: &'a Pubkey,
}

pub fn get_state(params: GetStateParams) -> Result<MigrationState> {
    let GetStateParams {
        client,
        migration_state,
    } = params;

    let account = client.get_account_data(migration_state)?;

    let state = MigrationState::deserialize(&mut account.as_slice())?;

    Ok(state)
}
