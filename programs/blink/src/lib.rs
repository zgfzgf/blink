pub mod error;
pub mod instructions;
pub mod state;
pub mod utils;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("BAFSUnhrUETfEd8AitHGBgPxBaNrAvXyq9x184HKLuYp");

pub const AUTH_SEED: &str = "auth_seed";

pub mod admin {
    use anchor_lang::prelude::declare_id;
    declare_id!("FDdjfxEvFjQhgnWbYQeo4GoHb7Kd4RcXVRbqpN4kBc9M");
}

#[program]
pub mod blink {
    use super::*;

    #[allow(clippy::too_many_arguments)]
    pub fn create_config(
        ctx: Context<CreateBlinkConfig>,
        index: u8,
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
