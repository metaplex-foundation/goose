use solana_program::{pubkey, pubkey::Pubkey};

const TOKEN_METADATA_ID: Pubkey = pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

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
