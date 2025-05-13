use anchor_lang::prelude::*;

use anchor_spl::token::{transfer_checked, Mint, Token, TokenAccount, TransferChecked};

use crate::{state::{Campaign, CampaignStatus, Institution, Earnings}, error::CustomError};

pub fn FinishEarnings(ctx: Context<FinishCampaign>) -> Result<()> {
    let vault = &ctx.accounts.vault_quote;
    let earnings = &ctx.accounts.earnings;

    // Check vault matches stored vault
    require!(vault.key() == campaign.vault, CustomError::InvalidVault);
    
    // Check vault matches stored vault
    require!(vault.key() == earnings.vault, CustomError::InvalidVault);

    // Check campaign status is open to refund
    require!(
        earnings.status == CampaignStatus::Successful,
        CustomError::EarningsNotActive
    );

    earnings.status = CampaignStatus::Successful;

    // Update institution's active campaigns status
    institution.has_active_campaigns = false;

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

        #[account(
        mut,
        close = user,
        seeds = [b"earnings", campaign.key().as_ref()],
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