use std::path::PathBuf;

use clap::{Parser, Subcommand};
use solana_program::pubkey::Pubkey;

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Args {
    /// Path to the keypair file.
    #[arg(short, long, global = true)]
    pub keypair_path: Option<PathBuf>,

    /// RPC URL for the Solana cluster.
    #[arg(short, long, global = true)]
    pub rpc_url: Option<String>,

    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Clone, Subcommand)]
pub enum Commands {
    Init {
        /// Mint of the collection parent NFT.
        #[arg(short, long)]
        collection_mint: Pubkey,

        /// Unlock method for the collection: Timed or Vote.
        #[arg(short = 'm', long, default_value = "Timed")]
        unlock_method: String,

        /// Number of items in the collection.
        #[arg(short, long)]
        size: u32,
    },
    InitMsg {
        /// Payer Pubkey
        #[arg(short, long)]
        payer: Pubkey,

        /// Update Authority Pubkey
        #[arg(short, long)]
        authority: Pubkey,

        /// Mint of the collection parent NFT.
        #[arg(short, long)]
        collection_mint: Pubkey,

        /// Unlock method for the collection: Timed or Vote.
        #[arg(short = 'm', long, default_value = "Timed")]
        unlock_method: String,

        /// Number of items in the collection.
        #[arg(short, long)]
        size: u32,
    },
    InitSigner,
    Cancel {
        /// Mint of the collection parent NFT.
        #[arg(short, long)]
        collection_mint: Pubkey,
    },
    GetState {
        /// Mint of the collection parent NFT.
        #[arg(short, long)]
        collection_mint: Pubkey,
    },
    GetAllStates,
    Update {
        /// Mint of the collection parent NFT.
        #[arg(short, long)]
        collection_mint: Pubkey,

        /// Rule set to use for the collection.
        #[arg(short, long)]
        rule_set: Option<Pubkey>,

        /// New number of items in the collection.
        #[arg(short, long)]
        size: Option<u32>,
    },
    Start {
        /// Mint of the collection parent NFT.
        #[arg(short, long)]
        collection_mint: Pubkey,
    },
    Migrate {
        /// Mint of the collection parent NFT.
        #[arg(short, long)]
        collection_mint: Pubkey,

        /// Mint list
        #[arg(short, long)]
        mint_list: PathBuf,
    },
    Sudo {
        #[arg(short, long)]
        collection_mint: Pubkey,

        #[arg(long)]
        ts: i64
    },
    }
