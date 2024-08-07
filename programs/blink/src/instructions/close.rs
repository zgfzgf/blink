use crate::state::*;
use anchor_lang::prelude::*;

pub fn close(ctx: Context<Close>) -> Result<()> {
    let blink_state = &mut ctx.accounts.blink_state.load_mut()?;
    blink_state.closed = true;
    if blink_state.rights > 0 {
        blink_state.reward = blink_state
            .amount
            .checked_div(blink_state.rights.into())
            .unwrap()
    }

    Ok(())
}

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(address = blink_state.load()?.pool_creator)]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub blink_state: AccountLoader<'info, BlinkState>,
}
