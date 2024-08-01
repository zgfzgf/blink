use crate::{error::ErrorCode, state::*};
use anchor_lang::prelude::*;

pub fn submit(ctx: Context<Submit>, answer: u8) -> Result<()> {
    let blink_state = &mut ctx.accounts.blink_state.load_mut()?;
    if blink_state.closed {
        return err!(ErrorCode::CloseAlready);
    }
    if blink_state.answer == answer {
        blink_state.rights = blink_state.rights.checked_add(1).unwrap();
    }

    let submit_state = &mut ctx.accounts.submit_state.load_init()?;

    submit_state.blink_state = ctx.accounts.blink_state.key();
    submit_state.user = ctx.accounts.user.key();
    submit_state.answer = answer;
    submit_state.claim = false;

    Ok(())
}

#[derive(Accounts)]
pub struct Submit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        seeds = [
            SUBMIT_SEED.as_bytes(),
            blink_state.key().as_ref(),
            user.key().as_ref(),
        ],
        bump,
        payer = user,
        space = SubmitState::INIT_SPACE
    )]
    pub submit_state: AccountLoader<'info, SubmitState>,

    #[account(mut)]
    pub blink_state: AccountLoader<'info, BlinkState>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
