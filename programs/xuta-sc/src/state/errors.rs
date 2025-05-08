use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
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
}