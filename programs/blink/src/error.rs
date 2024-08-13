/// Errors that may be returned by the TokenSwap program.
use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Close Already")]
    CloseAlready,
    #[msg("Opening")]
    Opening,

    #[msg("Invalid Claim")]
    InvalidClaim,
    #[msg("Invalid Index")]
    InvalidIndex,
    #[msg("Reward Zero")]
    RewardZero,
    #[msg("Claim Already")]
    ClaimAlready,
    #[msg("Input account owner is not the program address")]
    InvalidOwner,
    #[msg("Invalid OpenTime")]
    InvalidOpenTime,
    #[msg("Invalid CloseTime")]
    InvalidCloseTime,
}
