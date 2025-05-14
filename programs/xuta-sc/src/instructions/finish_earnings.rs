use anchor_lang::prelude::*;

use anchor_spl::token::{transfer_checked, Mint, Token, TokenAccount, TransferChecked};

use crate::{state::{Campaign, CampaignStatus, Institution, Earnings}, error::CustomError};

pub fn finish_earnings(ctx: Context<FinishEarnings>) -> Result<()> {
    let vault = &ctx.accounts.vault_quote;
    let earnings = &ctx.accounts.earnings;
    
    // Check vault matches stored vault
    require!(vault.key() == earnings.vault, CustomError::InvalidVault);

    // Check campaign status is open to refund
    require!(
        earnings.status == CampaignStatus::Successful,
        CustomError::EarningsNotActive
    );

    
    let mut earnings_data = &mut ctx.accounts.earnings;
    earnings_data.status = CampaignStatus::Successful;

    Ok(())
}


#[derive(Accounts)]
pub struct FinishEarnings<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub mint_quote: Account<'info, Mint>,
    
    #[account(
        mut,
        associated_token::mint = mint_quote,
        associated_token::authority = authority,
        associated_token::token_program = token_program
    )]
    pub user_token_account_quote: Account<'info, TokenAccount>,

    /// The campaign state PDA, storing conversion ratio and configuration.
    #[account(
        mut,
        has_one = mint_quote,
        seeds = [b"campaign", campaign.mint_player.key().as_ref()],
        bump = campaign.campaign_bump,
    )]
    pub campaign: Account<'info, Campaign>,

    #[account(
        mut,
        close = authority,
        seeds = [b"Earnings", campaign.key().as_ref()],
        bump = earnings.bump,
    )]
    pub earnings: Account<'info, Earnings>,

    #[account(
        mut,
        associated_token::mint = mint_quote,
        associated_token::authority = earnings,
    )]
    pub vault_quote: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}