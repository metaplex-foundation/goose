use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("Invalid unlock method. Must be one of: Timed, Vote")]
    InvalidUnlockMethod,
    #[error("No Solana CLI config file found.")]
    MissingSolanaConfig,
}
