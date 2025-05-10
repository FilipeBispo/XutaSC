use anchor_lang::prelude::*;

#[derive(InitSpace)]
#[account]
pub struct Institution {
    pub authority: Pubkey,
    #[max_len(64)]
    pub name: String,
    #[max_len(100)]
    pub contract: String, // might be an NFT if we have time
    pub disabled: bool,
    pub has_active_campaigns: bool,
    pub bump: u8, // For PDA bump seed
}