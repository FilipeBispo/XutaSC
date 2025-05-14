use anchor_lang::prelude::*;
use crate::state::enums::CampaignStatus;

#[derive(InitSpace)]
#[account]
pub struct Campaign {
    pub authority: Pubkey,
    #[max_len(64)]
    pub name: String,
    #[max_len(1000)]
    pub description: String,
    #[max_len(300)]
    pub contract: String, // might be an NFT if we have time
    #[max_len(300)]
    pub image: String, // might be an NFT if we have time
    pub ratio: u16,
    pub vault: Pubkey,
    pub mint_player: Pubkey,
    pub mint_quote: Pubkey,
    pub target_amount: u64,
    pub current_tokens: u64,
    pub current_fees: u64,
    pub initial_date:i64,
    pub due_date: i64,
    pub status: CampaignStatus,
    pub campaign_bump: u8,
}