use anchor_lang::error_code;

#[error_code]
pub enum CustomError {
    #[msg("Unauthorized Access")]
    Unauthorized,
    #[msg("Invalid Type")]
    InvalidType,
    #[msg("Duplicated Type")]
    DuplicatedType,
    #[msg("Insufficient Stock")]
    InsufficientStock,
}
