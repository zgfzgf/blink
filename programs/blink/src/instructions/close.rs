use crate::error::ErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock;

pub fn close(ctx: Context<Close>, index: u16, answer: u8) -> Result<()> {
    let blink_state = &mut ctx.accounts.blink_state.load_mut()?;

    let block_timestamp = clock::Clock::get()?.unix_timestamp as u64;
    if block_timestamp < blink_state.close_time {
        return err!(ErrorCode::InvalidCloseTime);
    }

    if blink_state.index != index {
        return err!(ErrorCode::InvalidIndex);
    }
    if blink_state.closed {
        return err!(ErrorCode::CloseAlready);
    }

    blink_state.closed = true;
    blink_state.answer = answer;

    let mut rights = 0;

    if 1 == answer {
        rights = blink_state.right1;
    } else if 2 == answer {
        rights = blink_state.right2;
    } else if 3 == answer {
        rights = blink_state.right3;
    } else if 4 == answer {
        rights = blink_state.right4;
    }

    if rights > 0 {
        blink_state.reward = blink_state.amount.checked_div(rights.into()).unwrap()
    }

    emit!(CloseEvent { index });

    Ok(())
}

#[derive(Accounts)]
#[instruction(index: u16)]
pub struct Close<'info> {
    pub payer: Signer<'info>,

    #[account(mut,
        seeds = [
            BLINK_SEED.as_bytes(),
            &index.to_le_bytes().as_ref(),
        ],
        bump=blink_state.load()?.bump,
    )]
    pub blink_state: AccountLoader<'info, BlinkState>,
}
