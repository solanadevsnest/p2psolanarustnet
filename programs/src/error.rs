use anchor_lang::prelude::*;

#[error_code]
pub enum EscrowError {
    #[msg("Invalid stage for exchange or cancellation.")]
    InvalidStage,
    #[msg("Insufficient funds to perform this operation.")]
    InsufficientFunds,
    #[msg("Invalid mint account specified for trade.")]
    InvalidMint,
    #[msg("Mint for the trade is missing.")]
    MissingMint,
    #[msg("Invalid trade type, possibly due to missing mint addresses.")]
    InvalidTradeType,
    #[msg("Invalid mint association between the token accounts.")]
    InvalidAccount,
    #[msg("Duplicate mint accounts are not allowed.")]
    DuplicateMint,
    #[msg("Account does not have a valid owner.")]
    InvalidOwner,
    #[msg("Invalid partner specified in create trade.")]
    InvalidPartner,
    #[msg("Trade and receive values must be greater than zero.")]
    ZeroValue,
    #[msg("Instruction data is missing required parameters.")]
    MissingParams,
}
