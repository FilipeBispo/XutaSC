use anchor_lang::prelude::*;
use crate::state::enums::CampaignStatus;

#[derive(InitSpace)]
#[account]
pub struct Earnings {
    pub owner: Pubkey,
    pub campaign: Pubkey,
    pub vault: Pubkey,
    pub status: CampaignStatus,
    pub bump: u8, // For PDA bump seed
}