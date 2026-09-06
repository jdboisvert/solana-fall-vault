use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Custom error message")]
    CustomError,
    #[msg("Exceeds max withdrawal limit of the vault")]
    ExceedsMaxWithdraw,
}
