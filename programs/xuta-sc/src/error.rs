use anchor_lang::error_code;

#[error_code]
pub enum CustomError {
    #[msg("Unauthorized Access")]
    Unauthorized,
    #[msg("Vault invalid")]
    InvalidVault,
    #[msg("Campaign is not Active")]
    CampaignNotActive,
    #[msg("Campaign didn't start yet")]
    CampaignNotStarted,
    #[msg("Campaign already ended")]
    CampaignEnded,
    #[msg("Invalid amount or ratio")]
    InvalidRatioOrAmount,
    #[msg("Fee Campaign parameter error")]
    FeeError,
    #[msg("Error performing math operation")]
    MathError,
    #[msg("Receipt has no value")]
    NoReceiptAmount,
    #[msg("Campaign is not open for refund")]
    CampaignNotOpenForRefund,
    #[msg("Vault has insufcient funds")]
    InsuficientFunds,
    #[msg("Invalid Token amount to mint")]
    InvalidTokenAmount,
    #[msg("Institution has active campaigns")]
    InstitutionHasActiveCampaigns,
    #[msg("Institution is disabled")]
    InstitutionDisabled,
    #[msg("Invalid fee value")]
    InvalidFeeValue,
    #[msg("Earnings not active")]
    EarningsNotActive,
}
