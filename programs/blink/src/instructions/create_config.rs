use crate::state::*;
use anchor_lang::prelude::*;
use std::ops::DerefMut;

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
    let blink_config = ctx.accounts.blink_config.deref_mut();
    blink_config.owner = ctx.accounts.owner.key();
    blink_config.index = index;
    blink_config.pic = pic;
    blink_config.content = content;
    blink_config.option1 = option1;
    blink_config.option2 = option2;
    blink_config.option3 = option3;
    blink_config.option4 = option4;
    blink_config.bump = ctx.bumps.blink_config;

    Ok(())
}

#[derive(Accounts)]
#[instruction(index: u16)]
pub struct CreateBlinkConfig<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        seeds = [
            CONFIG_SEED.as_bytes(),
            &index.to_le_bytes().as_ref()
        ],
        bump,
        payer = owner,
        space = ANCHOR_DISCRIMINATOR + BlinkConfig::INIT_SPACE
    )]
    pub blink_config: Account<'info, BlinkConfig>,

    pub system_program: Program<'info, System>,
}
