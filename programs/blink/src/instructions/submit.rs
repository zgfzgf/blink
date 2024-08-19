use crate::{error::ErrorCode, state::*};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock;

pub fn submit(ctx: Context<Submit>, index: u16, answer: u8) -> Result<()> {
    let blink_state = &mut ctx.accounts.blink_state.load_mut()?;

    let block_timestamp = clock::Clock::get()?.unix_timestamp as u64;
    if blink_state.open_time > block_timestamp {
        return err!(ErrorCode::InvalidOpenTime);
    } else if block_timestamp > blink_state.close_time {
        return err!(ErrorCode::InvalidCloseTime);
    }

    if blink_state.closed {
        return err!(ErrorCode::CloseAlready);
    }
    if blink_state.index != index {
        return err!(ErrorCode::InvalidIndex);
    }
    if 1 == answer {
        blink_state.right1 = blink_state.right1.checked_add(1).unwrap();
    } else if 2 == answer {
        blink_state.right2 = blink_state.right2.checked_add(1).unwrap();
    } else if 3 == answer {
        blink_state.right3 = blink_state.right3.checked_add(1).unwrap();
    } else if 4 == answer {
        blink_state.right4 = blink_state.right4.checked_add(1).unwrap();
    }

    let submit_state = &mut ctx.accounts.submit_state.load_init()?;
    submit_state.index = index;
    submit_state.blink_state = ctx.accounts.blink_state.key();
    submit_state.user = ctx.accounts.user.key();
    submit_state.answer = answer;
    submit_state.claim = false;
    submit_state.bump = ctx.bumps.submit_state;

    emit!(SubmitEvent {
        index,
        user: ctx.accounts.user.key(),
        answer
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(index: u16)]
pub struct Submit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        seeds = [
            SUBMIT_SEED.as_bytes(),
            &index.to_le_bytes().as_ref(),
            user.key().as_ref(),
        ],
        bump,
        payer = user,
        space = ANCHOR_DISCRIMINATOR + SubmitState::INIT_SPACE
    )]
    pub submit_state: AccountLoader<'info, SubmitState>,

    #[account(
        mut,
        seeds = [
            BLINK_SEED.as_bytes(),
            &index.to_le_bytes().as_ref(),
        ],
        bump=blink_state.load()?.bump,
    )]
    pub blink_state: AccountLoader<'info, BlinkState>,
    pub system_program: Program<'info, System>,
}
