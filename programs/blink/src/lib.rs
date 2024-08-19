pub mod error;
pub mod instructions;
pub mod state;
pub mod utils;

use anchor_lang::prelude::*;
use instructions::*;

pub const AUTH_SEED: &str = "auth_seed";

declare_id!("5JpxgkxQ2euSxEVWrpY52biJRCYAWp2f6KaZSiiCetc2");

#[program]
pub mod blink {
    use super::*;

    pub fn create_time(ctx: Context<CreateTimeConfig>, open_time: u64, period: u64) -> Result<()> {
        instructions::create_time(ctx, open_time, period)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn initialize(
        ctx: Context<Initialize>,
        index: u16,
        amount: u64,
        pic: String,
        content: String,
        option1: String,
        option2: String,
        option3: String,
        option4: String,
    ) -> Result<()> {
        instructions::initialize(
            ctx, index, amount, pic, content, option1, option2, option3, option4,
        )
    }

    pub fn submit(ctx: Context<Submit>, index: u16, answer: u8) -> Result<()> {
        instructions::submit(ctx, index, answer)
    }

    pub fn close(ctx: Context<Close>, index: u16, answer: u8) -> Result<()> {
        instructions::close(ctx, index, answer)
    }

    pub fn claim(ctx: Context<Claim>, index: u16) -> Result<()> {
        instructions::claim(ctx, index)
    }
}
