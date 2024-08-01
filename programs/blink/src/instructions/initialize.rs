use crate::state::*;

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

pub fn initialize(ctx: Context<Initialize>, amount: u64, answer: u8) -> Result<()> {
    let transfer_accounts = TransferChecked {
        from: ctx.accounts.creator_token.to_account_info(),
        mint: ctx.accounts.token_mint.to_account_info(),
        to: ctx.accounts.vault.to_account_info(),
        authority: ctx.accounts.creator.to_account_info(),
    };

    let cpi_context = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        transfer_accounts,
    );

    transfer_checked(cpi_context, amount, ctx.accounts.token_mint.decimals)?;

    let blink_state = &mut ctx.accounts.blink_state.load_init()?;

    blink_state.auth_bump = ctx.bumps.blink_state;
    blink_state.blink_config = ctx.accounts.blink_config.key();
    blink_state.pool_creator = ctx.accounts.creator.key();
    blink_state.vault = ctx.accounts.vault.key();
    blink_state.token_mint = ctx.accounts.token_mint.key();
    blink_state.token_program = ctx.accounts.token_program.key();
    blink_state.closed = false;
    blink_state.answer = answer;
    blink_state.rights = 0;
    blink_state.amount = amount;
    blink_state.reward = 0;

    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// Address paying to create the pool. Can be anyone
    #[account(mut)]
    pub creator: Signer<'info>,

    /// CHECK: pool vault authority
    #[account(
        seeds = [
            crate::AUTH_SEED.as_bytes(),
        ],
        bump,
    )]
    pub authority: UncheckedAccount<'info>,


    pub blink_config: Box<Account<'info, BlinkConfig>>,

    #[account(
        init,
        seeds = [
            BLINK_SEED.as_bytes(),
            blink_config.key().as_ref(),
            token_mint.key().as_ref(),
        ],
        bump,
        payer = creator,
        space = BlinkState::INIT_SPACE
    )]
    pub blink_state: AccountLoader<'info, BlinkState>,

    #[account(
        mint::token_program = token_program,
    )]
    pub token_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = creator,
        associated_token::token_program = token_program
    )]
    pub creator_token: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = creator,
        associated_token::mint = token_mint,
        associated_token::authority = authority,
        associated_token::token_program = token_program
    )]
    pub vault: Box<InterfaceAccount<'info, TokenAccount>>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
