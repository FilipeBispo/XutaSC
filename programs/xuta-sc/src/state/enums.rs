use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum CampaignStatus {
    Active,
    Paused,
    Successful,
    Failed,
    Finalized,
    Canceled,
}