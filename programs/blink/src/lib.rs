pub mod error;
pub mod instructions;
pub mod state;
pub mod utils;

use anchor_lang::prelude::*;
use instructions::*;

pub const AUTH_SEED: &str = "auth_seed";

declare_id!("45wPxt3utVDUdsHzu6Wq2e65xKSgwq6r9JEdSq9heAfb");

#[program]
pub mod blink {
    use super::*;

    #[allow(clippy::too_many_arguments)]
    pub fn create_config(
        ctx: Context<CreateBlinkConfig>,
        index: u16,
        pic: String,
        content: String,
        option1: String,
        option2: String,
        option3: String,
        option4: String,
    ) -> Result<()> {
        instructions::create_config(ctx, index, pic, content, option1, option2, option3, option4)
    }

    pub fn initialize(ctx: Context<Initialize>, amount: u64, answer: u8) -> Result<()> {
        instructions::initialize(ctx, amount, answer)
    }

    pub fn submit(ctx: Context<Submit>, answer: u8) -> Result<()> {
        instructions::submit(ctx, answer)
    }

    pub fn close(ctx: Context<Close>) -> Result<()> {
        instructions::close(ctx)
    }

    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        instructions::claim(ctx)
    }
}
