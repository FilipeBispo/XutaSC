use anchor_lang::prelude::*;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer_checked, TransferChecked, Mint, TokenAccount, Token}
};
            

use crate::{error::CustomError, state::{Campaign, CampaignStatus, Config, Earnings, Institution }};

impl<'info> InitEarnings<'info> {
    pub fn init_earnings(&mut self, ratio: u16) -> Result<()> {
        
        require!(self.institution.disabled == false,
            CustomError::InstitutionDisabled);
        
        let total_amount = self.campaign.current_tokens
            .checked_mul(ratio as u64)
            .ok_or(CustomError::InvalidRatioOrAmount)?;

        // Transfer quote from user to vault (checked)
        let cpi_accounts = TransferChecked {
            from: self.owner_quote_account.to_account_info(),
            to: self.vault_quote.to_account_info(),
            authority: self.authority.to_account_info(),
            mint: self.mint_quote.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer_checked(cpi_ctx, total_amount, 6)?; // maybe decimals should be in the campaign

        self.earnings.set_inner(Earnings { 
            authority: self.authority.key(), 
            campaign: self.campaign.key(), 
            vault: self.vault_quote.key(), 
            status: CampaignStatus::Active, 
            earnings_ratio: ratio, 
            bump: self.earnings.bump }); 

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitEarnings<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    pub mint_player: Account<'info, Mint>,

    pub mint_quote: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_quote,
        associated_token::authority = authority,
        associated_token::token_program = token_program
    )]
    pub owner_quote_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        // has_one = authority, // what if the authority for this campaign changes?
        has_one = mint_player,
        has_one = mint_quote,
        seeds = [b"campaign", mint_player.key().as_ref()],
        bump 
    )]    
    pub campaign: Account<'info, Campaign>,

    #[account(
        init,
        payer = authority,
        space = Earnings::DISCRIMINATOR.len() + Earnings::INIT_SPACE,
        seeds = [b"Earnings", campaign.key().as_ref()],
        bump
    )]    
    pub earnings: Account<'info, Earnings>,

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
        associated_token::mint = mint_quote,
        associated_token::authority = campaign,
        associated_token::token_program = token_program,
    )]
    pub vault_quote: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
