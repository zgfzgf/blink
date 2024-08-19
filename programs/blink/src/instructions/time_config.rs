use crate::error::ErrorCode;
use crate::state::*;

use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock;
use std::ops::DerefMut;

pub fn create_time(ctx: Context<CreateTimeConfig>, open_time: u64, period: u64) -> Result<()> {
    let time_config = ctx.accounts.time_config.deref_mut();
    time_config.owner = ctx.accounts.owner.key();

    let block_timestamp = clock::Clock::get()?.unix_timestamp as u64;
    if open_time < block_timestamp {
        return err!(ErrorCode::InvalidOpenTime);
    }
    let close_time = open_time.checked_add(period).unwrap();

    time_config.open_time = open_time;
    time_config.close_time = close_time;
    time_config.bump = ctx.bumps.time_config;

    Ok(())
}

#[derive(Accounts)]
pub struct CreateTimeConfig<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        seeds = [
            TIME_SEED.as_bytes(),
        ],
        bump,
        payer = owner,
        space = ANCHOR_DISCRIMINATOR + TimeConfig::INIT_SPACE
    )]
    pub time_config: Account<'info, TimeConfig>,

    pub system_program: Program<'info, System>,
}
