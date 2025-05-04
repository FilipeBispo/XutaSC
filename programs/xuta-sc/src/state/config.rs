use anchor_lang::prelude::*;

#[derive(InitSpace)]
#[account]
pub struct Config {
    pub authority: Pubkey,
    pub institution_authority: Pubkey,
    pub fee_pre: u16,
    pub fee_pos: u16,
    pub bump: u8, // For PDA bump seed
}