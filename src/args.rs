use clap::{Parser, Subcommand};
use solana_program::pubkey::Pubkey;

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Args {
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
}
