use anchor_lang::prelude::*;

pub const CONFIG_SEED: &str = "config_seed";

#[account]
#[derive(InitSpace)] // automatically calculate the space required for the struct
pub struct BlinkConfig {
    pub index: u8,
    pub admin: Pubkey,
    #[max_len(200)]
    pub pic: String, // 4 bytes + 200 bytes

    #[max_len(200)]
    pub content: String, // 4 bytes + 200 bytes

    #[max_len(100)]
    pub option1: String, // 4 bytes + 100 bytes
    #[max_len(100)]
    pub option2: String, // 4 bytes + 100 bytes
    #[max_len(100)]
    pub option3: String, // 4 bytes + 100 bytes
    #[max_len(100)]
    pub option4: String, // 4 bytes + 100 bytes
    pub bump: u8,
}

pub const BLINK_SEED: &str = "blink_seed";
pub const BLINK_VAULT_SEED: &str = "blink_vault_seed";

#[account(zero_copy(unsafe))]
#[derive(InitSpace)] // automatically calculate the space required for the struct
pub struct BlinkState {
    pub blink_config: Pubkey,
    pub pool_creator: Pubkey,
    pub vault: Pubkey,
    pub token_mint: Pubkey,
    pub token_program: Pubkey,
    pub closed: bool,
    pub answer: u8,
    pub rights: u32,
    pub amount: u64,
    pub reward: u64,
    pub auth_bump: u8,
    pub bump: u8,
}

pub const SUBMIT_SEED: &str = "submit_seed";
#[account(zero_copy(unsafe))]
#[derive(InitSpace)] // automatically calculate the space required for the struct
pub struct SubmitState {
    pub blink_state: Pubkey,
    pub user: Pubkey,
    pub answer: u8,
    pub claim: bool,
    pub bump: u8,
}
