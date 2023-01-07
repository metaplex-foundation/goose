use std::{str::FromStr, time::Duration};

use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use solana_client::rpc_client::RpcClient;
use solana_program::{pubkey, pubkey::Pubkey};
use solana_sdk::hash::Hash;

use crate::Cluster;

const TOKEN_METADATA_ID: Pubkey = pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

// Hash for devnet cluster
pub const DEVNET_HASH: &str = "EtWTRABZaYq6iMfeYKouRu166VU2xqa1wcaWoxPkrZBG";

/// Hash for mainnet-beta cluster
pub const MAINNET_HASH: &str = "5eykt4UsFv8P8NJdTREpY1vzqKqZKvdpKuc147dw2N9d";

pub fn find_metadata_pda(mint: &Pubkey) -> (Pubkey, u8) {
    let seeds = &[b"metadata", TOKEN_METADATA_ID.as_ref(), mint.as_ref()];
    Pubkey::find_program_address(seeds, &TOKEN_METADATA_ID)
}

pub fn find_migrate_state_pda(mint: Pubkey) -> (Pubkey, u8) {
    let seeds = &[b"migration", mint.as_ref()];
    Pubkey::find_program_address(seeds, &mpl_migration_validator::ID)
}

pub fn find_program_signer_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"signer"], &mpl_migration_validator::ID)
}

pub fn get_cluster(rpc_client: &RpcClient) -> Result<Cluster> {
    let devnet_hash = Hash::from_str(DEVNET_HASH).unwrap();
    let mainnet_hash = Hash::from_str(MAINNET_HASH).unwrap();
    let genesis_hash = rpc_client.get_genesis_hash()?;

    Ok(if genesis_hash == devnet_hash {
        Cluster::Devnet
    } else if genesis_hash == mainnet_hash {
        Cluster::Mainnet
    } else {
        return Err(anyhow::anyhow!("Unknown or unsupported cluster"));
    })
}

pub fn spinner_with_style() -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&[
                "▹▹▹▹▹",
                "▸▹▹▹▹",
                "▹▸▹▹▹",
                "▹▹▸▹▹",
                "▹▹▹▸▹",
                "▹▹▹▹▸",
                "▪▪▪▪▪",
            ])
            .template("{spinner:.dim} {msg}")
            .expect("failed to set progressbar template"),
    );
    pb
}
