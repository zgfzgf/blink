use crate::{error::ErrorCode, state::*};
use anchor_lang::prelude::*;
use std::ops::DerefMut;

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
    let blink_config = ctx.accounts.blink_config.deref_mut();
    blink_config.admin = ctx.accounts.owner.key();
    blink_config.bump = ctx.bumps.blink_config;
    blink_config.index = index;
    blink_config.pic = pic;
    blink_config.content = content;
    blink_config.option1 = option1;
    blink_config.option2 = option2;
    blink_config.option3 = option3;
    blink_config.option4 = option4;

    Ok(())
}

#[derive(Accounts)]
#[instruction(index: u8)]
pub struct CreateBlinkConfig<'info> {
    #[account(
        mut,
        address = crate::admin::id() @ ErrorCode::InvalidOwner
    )]
    pub owner: Signer<'info>,

    #[account(
        init,
        seeds = [
            CONFIG_SEED.as_bytes(),
            &index.to_be_bytes()
        ],
        bump,
        payer = owner,
        space = BlinkConfig::INIT_SPACE
    )]
    pub blink_config: Account<'info, BlinkConfig>,

    pub system_program: Program<'info, System>,
}
