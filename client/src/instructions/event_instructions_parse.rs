use anchor_client::ClientError;
use anchor_lang::Discriminator;
use base64::prelude::*;
use blink::instruction;
use blink::state::*;
use colorful::Color;
use colorful::Colorful;
use regex::Regex;
use solana_transaction_status::{
    option_serializer::OptionSerializer, EncodedTransaction, UiTransactionStatusMeta,
};

const PROGRAM_LOG: &str = "Program log: ";
const PROGRAM_DATA: &str = "Program data: ";

#[allow(dead_code)]
pub enum InstructionDecodeType {
    BaseHex,
    Base64,
    Base58,
}

pub fn handle_program_log(
    self_program_str: &str,
    l: &str,
    with_prefix: bool,
) -> Result<(Option<String>, bool), ClientError> {
    // Log emitted from the current program.
    if let Some(log) = if with_prefix {
        l.strip_prefix(PROGRAM_LOG)
            .or_else(|| l.strip_prefix(PROGRAM_DATA))
    } else {
        Some(l)
    } {
        if l.starts_with(&"Program log: ".to_string()) {
            // not log event
            return Ok((None, false));
        }
        let borsh_bytes = match BASE64_STANDARD.decode(log) {
            Ok(borsh_bytes) => borsh_bytes,
            _ => {
                println!("Could not base64 decode log: {}", log);
                return Ok((None, false));
            }
        };

        let mut slice: &[u8] = &borsh_bytes[..];
        let disc: [u8; 8] = {
            let mut disc = [0; 8];
            disc.copy_from_slice(&borsh_bytes[..8]);
            slice = &slice[8..];
            disc
        };
        match disc {
            InitializeEvent::DISCRIMINATOR => {
                println!("{:#?}", decode_event::<InitializeEvent>(&mut slice)?);
            }
            SubmitEvent::DISCRIMINATOR => {
                println!("{:#?}", decode_event::<SubmitEvent>(&mut slice)?);
            }
            CloseEvent::DISCRIMINATOR => {
                println!("{:#?}", decode_event::<CloseEvent>(&mut slice)?);
            }
            ClaimEvent::DISCRIMINATOR => {
                println!("{:#?}", decode_event::<ClaimEvent>(&mut slice)?);
            }
            _ => {
                println!("unknow event: {}", l);
            }
        }
        Ok((None, false))
    } else {
        let (program, did_pop) = handle_system_log(self_program_str, l);
        Ok((program, did_pop))
    }
}

fn decode_event<T: anchor_lang::Event + anchor_lang::AnchorDeserialize>(
    slice: &mut &[u8],
) -> Result<T, ClientError> {
    let event: T = anchor_lang::AnchorDeserialize::deserialize(slice)
        .map_err(|e| ClientError::LogParseError(e.to_string()))?;
    Ok(event)
}

fn handle_system_log(this_program_str: &str, log: &str) -> (Option<String>, bool) {
    if log.starts_with(&format!("Program {this_program_str} invoke")) {
        (Some(this_program_str.to_string()), false)
    } else if log.contains("invoke") {
        (Some("cpi".to_string()), false) // Any string will do.
    } else {
        let re = Regex::new(r"^Program (.*) success*$").unwrap();
        if re.is_match(log) {
            (None, true)
        } else {
            (None, false)
        }
    }
}

pub fn parse_program_instruction(
    self_program_str: &str,
    encoded_transaction: EncodedTransaction,
    meta: Option<UiTransactionStatusMeta>,
) -> Result<(), ClientError> {
    let ui_raw_msg = match encoded_transaction {
        solana_transaction_status::EncodedTransaction::Json(ui_tx) => {
            let ui_message = ui_tx.message;
            // println!("{:#?}", ui_message);
            match ui_message {
                solana_transaction_status::UiMessage::Raw(ui_raw_msg) => ui_raw_msg,
                _ => solana_transaction_status::UiRawMessage {
                    header: solana_sdk::message::MessageHeader::default(),
                    account_keys: Vec::new(),
                    recent_blockhash: "".to_string(),
                    instructions: Vec::new(),
                    address_table_lookups: None,
                },
            }
        }
        _ => solana_transaction_status::UiRawMessage {
            header: solana_sdk::message::MessageHeader::default(),
            account_keys: Vec::new(),
            recent_blockhash: "".to_string(),
            instructions: Vec::new(),
            address_table_lookups: None,
        },
    };
    // append lookup table keys if necessary
    if meta.is_some() {
        let mut account_keys = ui_raw_msg.account_keys;
        let meta = meta.clone().unwrap();
        if let OptionSerializer::Some(addresses) = meta.loaded_addresses {
            let mut writeable_address = addresses.writable;
            let mut readonly_address = addresses.readonly;
            account_keys.append(&mut writeable_address);
            account_keys.append(&mut readonly_address);
        }

        let program_index = account_keys
            .iter()
            .position(|r| r == self_program_str)
            .unwrap();
        // println!("{}", program_index);
        // println!("{:#?}", account_keys);
        for (i, ui_compiled_instruction) in ui_raw_msg.instructions.iter().enumerate() {
            if (ui_compiled_instruction.program_id_index as usize) == program_index {
                let out_put = format!("instruction #{}", i + 1);
                println!("{}", out_put.gradient(Color::Green));
                handle_program_instruction(
                    &ui_compiled_instruction.data,
                    InstructionDecodeType::Base58,
                )?;
            }
        }

        if let OptionSerializer::Some(inner_instructions) = meta.inner_instructions {
            for inner in inner_instructions {
                for (i, instruction) in inner.instructions.iter().enumerate() {
                    if let solana_transaction_status::UiInstruction::Compiled(
                        ui_compiled_instruction,
                    ) = instruction
                    {
                        if (ui_compiled_instruction.program_id_index as usize) == program_index {
                            let out_put =
                                format!("inner_instruction #{}.{}", inner.index + 1, i + 1);
                            println!("{}", out_put.gradient(Color::Green));
                            handle_program_instruction(
                                &ui_compiled_instruction.data,
                                InstructionDecodeType::Base58,
                            )?;
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn parse_program_event(
    self_program_str: &str,
    meta: Option<UiTransactionStatusMeta>,
) -> Result<(), ClientError> {
    let logs: Vec<String> = if let Some(meta_data) = meta {
        if let OptionSerializer::Some(log_messages) = meta_data.log_messages {
            log_messages
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };
    let mut logs = &logs[..];
    if !logs.is_empty() {
        if let Ok(mut execution) = Execution::new(&mut logs) {
            for l in logs {
                let (new_program, did_pop) =
                    if !execution.is_empty() && self_program_str == execution.program() {
                        handle_program_log(self_program_str, l, true).unwrap_or_else(|e| {
                            println!("Unable to parse log: {e}");
                            std::process::exit(1);
                        })
                    } else {
                        let (program, did_pop) = handle_system_log(self_program_str, l);
                        (program, did_pop)
                    };
                // Switch program context on CPI.
                if let Some(new_program) = new_program {
                    execution.push(new_program);
                }
                // Program returned.
                if did_pop {
                    execution.pop();
                }
            }
        }
    } else {
        println!("log is empty");
    }
    Ok(())
}

struct Execution {
    stack: Vec<String>,
}

impl Execution {
    pub fn new(logs: &mut &[String]) -> Result<Self, ClientError> {
        let l = &logs[0];
        *logs = &logs[1..];

        let re = Regex::new(r"^Program (.*) invoke.*$").unwrap();
        let c = re
            .captures(l)
            .ok_or_else(|| ClientError::LogParseError(l.to_string()))?;
        let program = c
            .get(1)
            .ok_or_else(|| ClientError::LogParseError(l.to_string()))?
            .as_str()
            .to_string();
        Ok(Self {
            stack: vec![program],
        })
    }

    pub fn program(&self) -> String {
        assert!(!self.stack.is_empty());
        self.stack[self.stack.len() - 1].clone()
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    pub fn push(&mut self, new_program: String) {
        self.stack.push(new_program);
    }

    pub fn pop(&mut self) {
        assert!(!self.stack.is_empty());
        self.stack.pop().unwrap();
    }
}

pub fn handle_program_instruction(
    instr_data: &str,
    decode_type: InstructionDecodeType,
) -> Result<(), ClientError> {
    let data = match decode_type {
        InstructionDecodeType::BaseHex => hex::decode(instr_data).unwrap(),
        InstructionDecodeType::Base64 => match BASE64_STANDARD.decode(instr_data) {
            Ok(borsh_bytes) => borsh_bytes,
            _ => {
                println!("Could not base64 decode instruction: {}", instr_data);
                return Ok(());
            }
        },
        InstructionDecodeType::Base58 => match bs58::decode(instr_data).into_vec() {
            Ok(borsh_bytes) => borsh_bytes,
            _ => {
                println!("Could not base58 decode instruction: {}", instr_data);
                return Ok(());
            }
        },
    };

    let mut ix_data: &[u8] = &data[..];
    let disc: [u8; 8] = {
        let mut disc = [0; 8];
        disc.copy_from_slice(&data[..8]);
        ix_data = &ix_data[8..];
        disc
    };
    // println!("{:?}", disc);

    match disc {
        instruction::Initialize::DISCRIMINATOR => {
            let ix = decode_instruction::<instruction::Initialize>(&mut ix_data).unwrap();
            #[allow(dead_code)]
            #[derive(Debug)]
            pub struct Initialize {
                pub index: u16,
                pub amount: u64,
                pub pic: String,
                pub content: String,
                pub option1: String,
                pub option2: String,
                pub option3: String,
                pub option4: String,
            }
            impl From<instruction::Initialize> for Initialize {
                fn from(instr: instruction::Initialize) -> Initialize {
                    Initialize {
                        index: instr.index,
                        amount: instr.amount,
                        pic: instr.pic,
                        content: instr.content,
                        option1: instr.option1,
                        option2: instr.option2,
                        option3: instr.option3,
                        option4: instr.option4,
                    }
                }
            }
            println!("{:#?}", Initialize::from(ix));
        }
        instruction::Submit::DISCRIMINATOR => {
            let ix = decode_instruction::<instruction::Submit>(&mut ix_data).unwrap();
            #[allow(dead_code)]
            #[derive(Debug)]
            pub struct Submit {
                pub index: u16,
                pub answer: u8,
            }
            impl From<instruction::Submit> for Submit {
                fn from(instr: instruction::Submit) -> Submit {
                    Submit {
                        index: instr.index,
                        answer: instr.answer,
                    }
                }
            }
            println!("{:#?}", Submit::from(ix));
        }
        instruction::Close::DISCRIMINATOR => {
            let ix = decode_instruction::<instruction::Close>(&mut ix_data).unwrap();
            #[allow(dead_code)]
            #[derive(Debug)]
            pub struct Close {
                pub index: u16,
                pub answer: u8,
            }
            impl From<instruction::Close> for Close {
                fn from(instr: instruction::Close) -> Close {
                    Close {
                        index: instr.index,
                        answer: instr.answer,
                    }
                }
            }
            println!("{:#?}", Close::from(ix));
        }
        instruction::Claim::DISCRIMINATOR => {
            let ix = decode_instruction::<instruction::Claim>(&mut ix_data).unwrap();
            #[allow(dead_code)]
            #[derive(Debug)]
            pub struct Claim {
                pub index: u16,
            }
            impl From<instruction::Claim> for Claim {
                fn from(instr: instruction::Claim) -> Claim {
                    Claim { index: instr.index }
                }
            }
            println!("{:#?}", Claim::from(ix));
        }

        _ => {
            println!("unknow instruction: {}", instr_data);
        }
    }
    Ok(())
}

fn decode_instruction<T: anchor_lang::AnchorDeserialize>(
    slice: &mut &[u8],
) -> Result<T, anchor_lang::error::ErrorCode> {
    let instruction: T = anchor_lang::AnchorDeserialize::deserialize(slice)
        .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
    Ok(instruction)
}
