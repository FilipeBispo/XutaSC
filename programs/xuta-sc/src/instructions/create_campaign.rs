use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked}
                };
            

use crate::state::{Campaign, CampaignStatus};


pub fn create_campaign(_ctx: Context<CreateCampaign>, amount; u64) -> Result<()> {
    // implementation to be made
    Ok(())
}

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct CreateCampaign<'info>{
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mint::token_program = token_program
    )]
    pub mint_player_receipt: InterfaceAccount<'info, Mint>,

    #[account(
        mint::token_program = token_program
    )]
    pub mint_usdc: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_player_receipt,
        associated_token::authority = owner,
        associated_token::token_program = token_program,
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = owner,
        seeds = [b"campaign", owner.key().as_ref(), seed.to_le_bytes().as_ref()],
        space = 8+ Campaign::INIT_SPACE,
        bump
    )]
    pub campaign: Account<'info, Campaign>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>
}