use anchor_lang::prelude::*;

#[derive(InitSpace)]
#[account]
pub struct Receipt {
    pub authority: Pubkey,
    pub token_amount: u64,
    pub fee_amount: u64,
    pub bump: u8, // For PDA bump seed
}