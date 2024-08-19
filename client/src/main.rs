use anchor_client::{Client, Cluster};
use anyhow::{format_err, Result};
use clap::Parser;
use configparser::ini::Ini;
use solana_client::{rpc_client::RpcClient, rpc_config::RpcTransactionConfig};
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signature, Signer},
    transaction::Transaction,
};
use solana_transaction_status::UiTransactionEncoding;

use std::rc::Rc;
use std::str::FromStr;

mod instructions;
use instructions::blink_instructions::*;
use instructions::event_instructions_parse::*;
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
    CreateTime {
        open_time: u64,
        period: u64,
    },
    Initialize {
        index: u16,
        token_mint: Pubkey,
        amount: u64,
        pic: String,
        content: String,
        option1: String,
        option2: String,
        option3: String,
        option4: String,
    },
    Submit {
        index: u16,
        answer: u8,
    },
    Close {
        index: u16,
        answer: u8,
    },
    Claim {
        index: u16,
    },
    DecodeInstruction {
        instr_hex_data: String,
    },
    DecodeEvent {
        log_event: String,
    },
    DecodeTxLog {
        tx_id: String,
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
        BlinkCommands::CreateTime { open_time, period } => {
            let create_config = create_config_instr(&pool_config, open_time, period)?;

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
            pic,
            content,
            option1,
            option2,
            option3,
            option4,
        } => {
            let initialize = initialize_instr(
                &pool_config,
                index,
                token_mint,
                amount,
                pic,
                content,
                option1,
                option2,
                option3,
                option4,
            )?;
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
        BlinkCommands::Submit { index, answer } => {
            let sumbit = submit_instr(&pool_config, index, answer)?;
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
        BlinkCommands::Close { index, answer } => {
            let close = close_instr(&pool_config, index, answer)?;
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
        BlinkCommands::Claim { index } => {
            let claim = claim_instr(&pool_config, index)?;
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
        BlinkCommands::DecodeInstruction { instr_hex_data } => {
            handle_program_instruction(&instr_hex_data, InstructionDecodeType::BaseHex)?;
        }
        BlinkCommands::DecodeEvent { log_event } => {
            handle_program_log(&pool_config.blink_program.to_string(), &log_event, false)?;
        }
        BlinkCommands::DecodeTxLog { tx_id } => {
            let signature = Signature::from_str(&tx_id)?;
            let tx = rpc_client.get_transaction_with_config(
                &signature,
                RpcTransactionConfig {
                    encoding: Some(UiTransactionEncoding::Json),
                    commitment: Some(CommitmentConfig::confirmed()),
                    max_supported_transaction_version: Some(0),
                },
            )?;
            let transaction = tx.transaction;
            // get meta
            let meta = if transaction.meta.is_some() {
                transaction.meta
            } else {
                None
            };
            // get encoded_transaction
            let encoded_transaction = transaction.transaction;
            // decode instruction data
            parse_program_instruction(
                &pool_config.blink_program.to_string(),
                encoded_transaction,
                meta.clone(),
            )?;
            // decode logs
            parse_program_event(&pool_config.blink_program.to_string(), meta.clone())?;
        }
    }
    Ok(())
}
