use crate::state::*;

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};
use std::ops::DerefMut;

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

    let blink_config = ctx.accounts.blink_config.deref_mut();
    blink_config.owner = ctx.accounts.creator.key();
    blink_config.index = index;
    blink_config.pic = pic;
    blink_config.content = content;
    blink_config.option1 = option1;
    blink_config.option2 = option2;
    blink_config.option3 = option3;
    blink_config.option4 = option4;
    blink_config.bump = ctx.bumps.blink_config;

    let blink_state = &mut ctx.accounts.blink_state.load_init()?;
    blink_state.index = index;
    blink_state.creator = ctx.accounts.creator.key();
    blink_state.blink_config = ctx.accounts.blink_config.key();
    blink_state.vault = ctx.accounts.vault.key();
    blink_state.token_mint = ctx.accounts.token_mint.key();
    blink_state.closed = false;
    blink_state.answer = 0;
    blink_state.right1 = 0;
    blink_state.right2 = 0;
    blink_state.right3 = 0;
    blink_state.right4 = 0;
    blink_state.amount = amount;
    blink_state.reward = 0;
    blink_state.auth_bump = ctx.bumps.authority;
    blink_state.bump = ctx.bumps.blink_state;

    emit!(InitializeEvent {
        index,
        creator: ctx.accounts.creator.key(),
        valut: ctx.accounts.vault.key(),
        token_mint: ctx.accounts.token_mint.key(),
        config: ctx.accounts.blink_config.key(),
        amount
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(index: u16)]
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

    #[account(
        init,
        seeds = [
            CONFIG_SEED.as_bytes(),
            &index.to_le_bytes().as_ref(),
        ],
        bump,
        payer = creator,
        space = ANCHOR_DISCRIMINATOR + BlinkConfig::INIT_SPACE
    )]
    pub blink_config: Account<'info, BlinkConfig>,

    #[account(
        init,
        seeds = [
            BLINK_SEED.as_bytes(),
            &index.to_le_bytes().as_ref(),
        ],
        bump,
        payer = creator,
        space = ANCHOR_DISCRIMINATOR + BlinkState::INIT_SPACE
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
}
