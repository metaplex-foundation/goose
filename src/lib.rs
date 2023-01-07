use std::fmt;

pub mod args;
pub mod errors;
pub mod methods;
pub mod processor;
pub mod setup;
pub mod utils;

pub const EXPLORER_URL: &str = "https://explorer.solana.com";

pub enum Cluster {
    Devnet,
    Mainnet,
}

impl fmt::Display for Cluster {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cluster::Devnet => write!(f, "devnet"),
            Cluster::Mainnet => write!(f, "mainnet-beta"),
        }
    }
}
