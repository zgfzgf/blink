use anchor_client::{Client, Cluster};
use anyhow::{format_err, Result};
use clap::Parser;
use configparser::ini::Ini;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use std::rc::Rc;
use std::str::FromStr;

mod instructions;
use instructions::blink_instructions::*;
use instructions::rpc::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ClientConfig {
    http_url: String,
    ws_url: String,
    owner_path: String,
    creator_path: String,
    user_path: String,
    blink_program: Pubkey,
}

fn load_cfg(client_config: &String) -> Result<ClientConfig> {
    let mut config = Ini::new();
    let _map = config.load(client_config).unwrap();
    let http_url = config.get("Global", "http_url").unwrap();
    if http_url.is_empty() {
        panic!("http_url must not be empty");
    }
    let ws_url = config.get("Global", "ws_url").unwrap();
    if ws_url.is_empty() {
        panic!("ws_url must not be empty");
    }
    let owner_path = config.get("Global", "owner_path").unwrap();
    if owner_path.is_empty() {
        panic!("owner_path must not be empty");
    }
    let creator_path = config.get("Global", "creator_path").unwrap();
    if creator_path.is_empty() {
        panic!("creator_path must not be empty");
    }
    let user_path = config.get("Global", "user_path").unwrap();
    if user_path.is_empty() {
        panic!("user_path must not be empty");
    }

    let blink_program_str = config.get("Global", "blink_program").unwrap();
    if blink_program_str.is_empty() {
        panic!("blink_program must not be empty");
    }
    let blink_program = Pubkey::from_str(&blink_program_str).unwrap();

    Ok(ClientConfig {
        http_url,
        ws_url,
        owner_path,
        creator_path,
        user_path,
        blink_program,
    })
}

fn read_keypair_file(s: &str) -> Result<Keypair> {
    solana_sdk::signature::read_keypair_file(s)
        .map_err(|_| format_err!("failed to read keypair from {}", s))
}

#[derive(Debug, Parser)]
pub struct Opts {
    #[clap(subcommand)]
    pub command: BlinkCommands,
}

#[derive(Debug, Parser)]
pub enum BlinkCommands {
    CreateConfig {
        index: u16,
        pic: String,
        content: String,
        option1: String,
        option2: String,
        option3: String,
        option4: String,
    },
    Initialize {
        index: u16,
        token_mint: Pubkey,
        amount: u64,
        answer: u8,
    },
    Submit {
        blink_state: Pubkey,
        answer: u8,
    },
    Close {
        blink_state: Pubkey,
    },
    Claim {
        submit_state: Pubkey,
    },
}
fn main() -> Result<()> {
    let client_config = "client_config.ini";
    let pool_config = load_cfg(&client_config.to_string())?;
    // cluster params.
    let payer = read_keypair_file(&pool_config.owner_path)?;
    // solana rpc client
    let rpc_client = RpcClient::new(pool_config.http_url.to_string());

    let anchor_config = pool_config.clone();
    let url = Cluster::Custom(anchor_config.http_url, anchor_config.ws_url);
    let wallet = read_keypair_file(&anchor_config.owner_path)?;
    let anchor_client = Client::new(url, Rc::new(wallet));
    let _program = anchor_client.program(anchor_config.blink_program)?;

    let opts = Opts::parse();
    match opts.command {
        BlinkCommands::CreateConfig {
            index,
            pic,
            content,
            option1,
            option2,
            option3,
            option4,
        } => {
            let create_config = create_config_instr(
                &pool_config,
                index,
                pic,
                content,
                option1,
                option2,
                option3,
                option4,
            )?;

            let signers = vec![&payer];
            let recent_hash = rpc_client.get_latest_blockhash()?;
            let txn = Transaction::new_signed_with_payer(
                &create_config,
                Some(&payer.pubkey()),
                &signers,
                recent_hash,
            );
            let signature = send_txn(&rpc_client, &txn, true)?;
            println!("{}", signature);
        }
        BlinkCommands::Initialize {
            index,
            token_mint,
            amount,
            answer,
        } => {
            let initialize = initialize_instr(&pool_config, index, token_mint, amount, answer)?;
            let payer = read_keypair_file(&pool_config.creator_path)?;

            let signers = vec![&payer];
            let recent_hash = rpc_client.get_latest_blockhash()?;
            let txn = Transaction::new_signed_with_payer(
                &initialize,
                Some(&payer.pubkey()),
                &signers,
                recent_hash,
            );
            let signature = send_txn(&rpc_client, &txn, true)?;
            println!("{}", signature);
        }
        BlinkCommands::Submit {
            blink_state,
            answer,
        } => {
            let sumbit = submit_instr(&pool_config, blink_state, answer)?;
            let payer = read_keypair_file(&pool_config.user_path)?;

            let signers = vec![&payer];
            let recent_hash = rpc_client.get_latest_blockhash()?;
            let txn = Transaction::new_signed_with_payer(
                &sumbit,
                Some(&payer.pubkey()),
                &signers,
                recent_hash,
            );
            let signature = send_txn(&rpc_client, &txn, true)?;
            println!("{}", signature);
        }
        BlinkCommands::Close { blink_state } => {
            let close = close_instr(&pool_config, blink_state)?;
            let payer = read_keypair_file(&pool_config.creator_path)?;

            let signers = vec![&payer];
            let recent_hash = rpc_client.get_latest_blockhash()?;
            let txn = Transaction::new_signed_with_payer(
                &close,
                Some(&payer.pubkey()),
                &signers,
                recent_hash,
            );
            let signature = send_txn(&rpc_client, &txn, true)?;
            println!("{}", signature);
        }
        BlinkCommands::Claim { submit_state } => {
            let claim = claim_instr(&pool_config, submit_state)?;
            let payer = read_keypair_file(&pool_config.user_path)?;

            let signers = vec![&payer];
            let recent_hash = rpc_client.get_latest_blockhash()?;
            let txn = Transaction::new_signed_with_payer(
                &claim,
                Some(&payer.pubkey()),
                &signers,
                recent_hash,
            );
            let signature = send_txn(&rpc_client, &txn, true)?;
            println!("{}", signature);
        }
    }
    Ok(())
}
