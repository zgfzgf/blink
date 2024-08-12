pub mod error;
pub mod instructions;
pub mod state;
pub mod utils;

use anchor_lang::prelude::*;
use instructions::*;

pub const AUTH_SEED: &str = "auth_seed";

declare_id!("GuxhyMkqSAfUa1JxqrDqR7yVtH5nPJ4zjf2Nm3U3ejpX");

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
