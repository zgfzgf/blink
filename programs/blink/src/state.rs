use anchor_lang::prelude::*;

pub const CONFIG_SEED: &str = "config_seed";
pub const ANCHOR_DISCRIMINATOR: usize = 8;

#[account]
#[derive(InitSpace)] // automatically calculate the space required for the struct
pub struct BlinkConfig {
    pub index: u16,
    pub owner: Pubkey,
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

#[cfg(feature = "open-time")]
pub const PERIOD: u64 = 259200;

#[cfg(feature = "open-time")]
#[account(zero_copy(unsafe))]
#[derive(InitSpace)] // automatically calculate the space required for the struct
pub struct BlinkState {
    pub index: u16,
    pub creator: Pubkey,
    pub blink_config: Pubkey,
    pub vault: Pubkey,
    pub token_mint: Pubkey,
    pub right1: u32,
    pub right2: u32,
    pub right3: u32,
    pub right4: u32,
    pub amount: u64,
    pub closed: bool,
    pub answer: u8,
    pub reward: u64,
    pub open_time: u64,
    pub close_time: u64,
    pub auth_bump: u8,
    pub bump: u8,
}

#[cfg(not(feature = "open-time"))]
#[account(zero_copy(unsafe))]
#[derive(InitSpace)] // automatically calculate the space required for the struct
pub struct BlinkState {
    pub index: u16,
    pub creator: Pubkey,
    pub blink_config: Pubkey,
    pub vault: Pubkey,
    pub token_mint: Pubkey,
    pub right1: u32,
    pub right2: u32,
    pub right3: u32,
    pub right4: u32,
    pub amount: u64,
    pub closed: bool,
    pub answer: u8,
    pub reward: u64,
    pub auth_bump: u8,
    pub bump: u8,
}

pub const SUBMIT_SEED: &str = "submit_seed";
#[account(zero_copy(unsafe))]
#[derive(InitSpace)] // automatically calculate the space required for the struct
pub struct SubmitState {
    pub index: u16,
    pub blink_state: Pubkey,
    pub user: Pubkey,
    pub answer: u8,
    pub claim: bool,
    pub bump: u8,
}

#[event]
#[cfg_attr(feature = "client", derive(Debug))]
pub struct InitializeEvent {
    #[index]
    pub index: u16,
    pub creator: Pubkey,
    pub valut: Pubkey,
    pub token_mint: Pubkey,
    pub config: Pubkey,
    pub amount: u64,
}

#[event]
#[cfg_attr(feature = "client", derive(Debug))]
pub struct SubmitEvent {
    #[index]
    pub index: u16,
    pub user: Pubkey,
    pub answer: u8,
}

#[event]
#[cfg_attr(feature = "client", derive(Debug))]
pub struct CloseEvent {
    #[index]
    pub index: u16,
}

#[event]
#[cfg_attr(feature = "client", derive(Debug))]
pub struct ClaimEvent {
    #[index]
    pub index: u16,
    #[index]
    pub user: Pubkey,
    pub reward: u64,
}
