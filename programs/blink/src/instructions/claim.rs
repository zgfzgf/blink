use crate::error::ErrorCode;
use crate::state::*;

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

pub fn claim(ctx: Context<Claim>, index: u16) -> Result<()> {
    let blink_state = &ctx.accounts.blink_state.load()?;
    if !blink_state.closed {
        return err!(ErrorCode::Opening);
    }
    let reward = blink_state.reward;
    if reward == 0 {
        return err!(ErrorCode::RewardZero);
    }

    if blink_state.index != index {
        return err!(ErrorCode::InvalidIndex);
    }

    let submit_state = &mut ctx.accounts.submit_state.load_mut()?;
    if submit_state.claim {
        return err!(ErrorCode::ClaimAlready);
    }
    if blink_state.answer != submit_state.answer {
        return err!(ErrorCode::InvalidClaim);
    }

    if blink_state.index != submit_state.index {
        return err!(ErrorCode::InvalidIndex);
    }

    submit_state.claim = true;

    let signer_seeds: [&[&[u8]]; 1] = [&[crate::AUTH_SEED.as_bytes(), &[blink_state.auth_bump]]];

    let transfer_accounts = TransferChecked {
        from: ctx.accounts.vault.to_account_info(),
        mint: ctx.accounts.token_mint.to_account_info(),
        to: ctx.accounts.user_account.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };

    let cpi_context = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        transfer_accounts,
        &signer_seeds,
    );

    transfer_checked(cpi_context, reward, ctx.accounts.token_mint.decimals)?;

    Ok(())
}

#[derive(Accounts)]
#[instruction(index: u16)]
pub struct Claim<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [
                SUBMIT_SEED.as_bytes(),
                &index.to_le_bytes().as_ref(),
                user.key().as_ref(),
            ],
        bump = submit_state.load()?.bump,
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

    /// CHECK: pool vault authority
    #[account(
        seeds = [
            crate::AUTH_SEED.as_bytes(),
        ],
        bump = blink_state.load()?.auth_bump,
    )]
    pub authority: UncheckedAccount<'info>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token_mint,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub user_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = authority,
        associated_token::token_program = token_program,
        constraint = vault.key() == blink_state.load()?.vault
    )]
    pub vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
    mint::token_program = token_program
    )]
    pub token_mint: Box<InterfaceAccount<'info, Mint>>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
