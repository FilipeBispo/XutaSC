use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, TokenAccount, Token}
};
            

use crate::{state::{Campaign, CampaignStatus, Config, Institution, }, error::CustomError};

impl<'info> CreateCampaign<'info>
{
    pub fn create_campaign(&mut self, 
        name: String,
        contract: String,
        image: String,
        description: String,
        ratio: u16,
        target_amount: u64,
        initial_date: i64,
        due_date: i64, 
        campaign_bump: u8) -> Result<()> {
        
        require!(self.institution.disabled == false,
                    CustomError::InstitutionDisabled);
        
        self.campaign.set_inner(
            Campaign{
                authority: self.authority.key(),
                name,
                description,
                contract,
                image,
                ratio,
                vault: self.vault.key(),
                mint_player: self.mint_player.key(),
                mint_quote: self.mint_quote.key(),
                target_amount,
                current_tokens: 0,
                current_fees: 0,
                initial_date,
                due_date,
                status: CampaignStatus::Active,
                campaign_bump,
            }
        );
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateCampaign<'info>{
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        mint::token_program = token_program,
        mint::decimals = 6,
        mint::authority = campaign.key(),
    )]
    pub mint_player: Account<'info, Mint>,

    #[account(
        mint::token_program = token_program,
    )]
    pub mint_quote: Account<'info, Mint>,
    
    #[account(
        init,
        payer = authority,
        space = Campaign::DISCRIMINATOR.len() + Campaign::INIT_SPACE,
        seeds = [b"campaign", mint_player.key().as_ref()],
        bump
    )]    
    pub campaign: Account<'info, Campaign>,

    #[account(
        has_one = authority,
    )]
    pub institution: Account<'info, Institution>,

    #[account(
        seeds=[b"config".as_ref()],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,
    
    #[account(
        init,
        payer = authority,
        token::mint = mint_quote,
        token::authority = campaign,
        token::token_program = token_program,
        seeds = [b"vault", campaign.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}