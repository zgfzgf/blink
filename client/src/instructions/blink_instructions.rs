use anchor_client::{Client, Cluster};
use anyhow::Result;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey, signature::Signer, system_program};
use spl_associated_token_account as spl_associated;

use blink::accounts as blink_accounts;
use blink::instruction as blink_instructions;
use blink::{state::*, AUTH_SEED};
use std::rc::Rc;

use crate::{read_keypair_file, ClientConfig};

#[allow(clippy::too_many_arguments)]
pub fn create_config_instr(
    config: &ClientConfig,
    index: u16,
    pic: String,
    content: String,
    option1: String,
    option2: String,
    option3: String,
    option4: String,
) -> Result<Vec<Instruction>> {
    let payer = read_keypair_file(&config.owner_path)?;
    let url = Cluster::Custom(config.http_url.clone(), config.ws_url.clone());
    // Client.
    let client = Client::new(url, Rc::new(payer));
    let program = client.program(config.blink_program)?;

    let (blink_config_key, _bump) = Pubkey::find_program_address(
        &[CONFIG_SEED.as_bytes(), &index.to_le_bytes()[..]],
        &program.id(),
    );
    let instructions = program
        .request()
        .accounts(blink_accounts::CreateBlinkConfig {
            owner: program.payer(),
            blink_config: blink_config_key,
            system_program: system_program::id(),
        })
        .args(blink_instructions::CreateConfig {
            index,
            pic,
            content,
            option1,
            option2,
            option3,
            option4,
        })
        .instructions()?;
    Ok(instructions)
}

pub fn initialize_instr(
    config: &ClientConfig,
    index: u16,
    token_mint: Pubkey,
    amount: u64,
    answer: u8,
) -> Result<Vec<Instruction>> {
    let creator = read_keypair_file(&config.creator_path)?;
    let payer = read_keypair_file(&config.creator_path)?;
    let url = Cluster::Custom(config.http_url.clone(), config.ws_url.clone());
    // Client.
    let client = Client::new(url, Rc::new(payer));
    let program = client.program(config.blink_program)?;

    let (blink_config_key, _bump) = Pubkey::find_program_address(
        &[CONFIG_SEED.as_bytes(), &index.to_le_bytes()[..]],
        &program.id(),
    );
    let (authority, __bump) = Pubkey::find_program_address(&[AUTH_SEED.as_bytes()], &program.id());

    let (blink_state_key, _bump) = Pubkey::find_program_address(
        &[
            BLINK_SEED.as_bytes(),
            blink_config_key.to_bytes().as_ref(),
            token_mint.to_bytes().as_ref(),
        ],
        &program.id(),
    );

    let creator_token =
        spl_associated::get_associated_token_address(&creator.pubkey(), &token_mint);
    let vault = spl_associated::get_associated_token_address(&authority, &token_mint);

    let instructions = program
        .request()
        .accounts(blink_accounts::Initialize {
            creator: creator.pubkey(),
            authority,
            blink_config: blink_config_key,
            blink_state: blink_state_key,
            token_mint,
            creator_token,
            vault,
            token_program: spl_token::id(),
            associated_token_program: spl_associated::id(),
            system_program: system_program::id(),
        })
        .args(blink_instructions::Initialize { amount, answer })
        .instructions()?;
    Ok(instructions)
}

pub fn submit_instr(
    config: &ClientConfig,
    blink_state: Pubkey,
    answer: u8,
) -> Result<Vec<Instruction>> {
    let user = read_keypair_file(&config.user_path)?;
    let payer = read_keypair_file(&config.creator_path)?;
    let url = Cluster::Custom(config.http_url.clone(), config.ws_url.clone());
    // Client.
    let client = Client::new(url, Rc::new(payer));
    let program = client.program(config.blink_program)?;

    let (submit_state_key, _bump) = Pubkey::find_program_address(
        &[
            SUBMIT_SEED.as_bytes(),
            blink_state.to_bytes().as_ref(),
            user.pubkey().to_bytes().as_ref(),
        ],
        &program.id(),
    );

    let instructions = program
        .request()
        .accounts(blink_accounts::Submit {
            user: user.pubkey(),
            submit_state: submit_state_key,
            blink_state,
            system_program: system_program::id(),
        })
        .args(blink_instructions::Submit { answer })
        .instructions()?;
    Ok(instructions)
}

pub fn close_instr(config: &ClientConfig, blink_state: Pubkey) -> Result<Vec<Instruction>> {
    let creator = read_keypair_file(&config.creator_path)?;
    let payer = read_keypair_file(&config.creator_path)?;
    let url = Cluster::Custom(config.http_url.clone(), config.ws_url.clone());
    // Client.
    let client = Client::new(url, Rc::new(payer));
    let program = client.program(config.blink_program)?;

    let instructions = program
        .request()
        .accounts(blink_accounts::Close {
            owner: creator.pubkey(),
            blink_state,
        })
        .args(blink_instructions::Close {})
        .instructions()?;
    Ok(instructions)
}

pub fn claim_instr(config: &ClientConfig, submit_key: Pubkey) -> Result<Vec<Instruction>> {
    let user = read_keypair_file(&config.user_path)?;
    let payer = read_keypair_file(&config.creator_path)?;
    let url = Cluster::Custom(config.http_url.clone(), config.ws_url.clone());
    // Client.
    let client = Client::new(url, Rc::new(payer));
    let program = client.program(config.blink_program)?;

    let sumbit_state: blink::state::SubmitState = program.account(submit_key)?;
    let blink_key = sumbit_state.blink_state;
    let blink_state: blink::state::BlinkState = program.account(blink_key)?;
    let (authority, __bump) = Pubkey::find_program_address(&[AUTH_SEED.as_bytes()], &program.id());
    let user_token =
        spl_associated::get_associated_token_address(&user.pubkey(), &blink_state.token_mint);

    let instructions = program
        .request()
        .accounts(blink_accounts::Claim {
            user: user.pubkey(),
            submit_state: submit_key,
            blink_state: blink_key,
            authority,
            user_account: user_token,
            vault: blink_state.vault,
            token_mint: blink_state.token_mint,
            token_program: spl_token::id(),
            associated_token_program: spl_associated::id(),
            system_program: system_program::id(),
        })
        .args(blink_instructions::Claim {})
        .instructions()?;
    Ok(instructions)
}
