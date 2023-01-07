use clap::{Parser, Subcommand};
use solana_program::pubkey::Pubkey;

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Args {
    #[arg(short = 'T', long, global = true, default_value = "60")]
    timeout: u32,

    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Clone, Subcommand)]
pub enum Commands {
    Initialize {
        #[arg(short, long)]
        collection_mint: Pubkey,

        #[arg(short, long, default_value = "Timed")]
        unlock_method: String,
    },
}
