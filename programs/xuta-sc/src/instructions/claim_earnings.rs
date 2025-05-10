use anchor_lang::prelude::*;

use anchor_spl::token::{Mint, Token, TokenAccount, Burn, burn, TransferChecked, transfer_checked};

use crate::{
    error::CustomError,
    state::{Campaign, CampaignStatus, Earnings, Config},
};

impl<'info> ClaimEarnings<'info> {
    pub fn claim_earnings(ctx: Context<ClaimEarnings>, token_amount: u64) -> Result<()> {
        let campaign = &ctx.accounts.campaign;
        let vault = &ctx.accounts.vault_quote;
        let earnings = &ctx.accounts.earnings;
        let config = &ctx.accounts.config;

        // Check vault matches stored vault
        require!(vault.key() == earnings.vault, CustomError::InvalidVault);

        // Check campaign status is open to refund

        require!(
            earnings.status != CampaignStatus::Successful,
            CustomError::EarningsNotActive
        );

        let fee_amount= token_amount
        .checked_mul(config.fee_pos as u64)
        .ok_or(CustomError::MathError)?;

        let total_earnings = token_amount
            .checked_mul(earnings.earnings_ratio as u64)
            .ok_or(CustomError::MathError)?;

        let user_earnings = token_amount
            .checked_sub(fee_amount)
            .ok_or(CustomError::MathError)?;

        // Transfer quote from vault to user (checked)
        let cpi_accounts = TransferChecked {
            from: vault.to_account_info(),
            to: ctx.accounts.user_quote_token_account.to_account_info(),
            authority: earnings.to_account_info(),
            mint: ctx.accounts.mint_quote.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program.clone(), cpi_accounts);

        transfer_checked(cpi_ctx, user_earnings, 6)?;

        let cpi_burn_acounts= Burn{
            mint: ctx.accounts.mint_player.to_account_info(),
            from: ctx.accounts.user_player_token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };

        let cpi_burn_ctx = CpiContext::new(cpi_program, cpi_burn_acounts);

        burn(cpi_burn_ctx,token_amount);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct ClaimEarnings<'info> {

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub mint_player: Account<'info, Mint>,

    #[account(mut)]
    pub mint_quote: Account<'info, Mint>,

    // The campaign state PDA, storing conversion ratio and configuration.
    #[account(
        mut,
        has_one = mint_player,
        has_one = mint_quote,
        seeds = [b"campaign", campaign.mint_player.key().as_ref()],
        bump = campaign.campaign_bump,
    )]
    pub campaign: Account<'info, Campaign>,

    #[account(
        mut,
        associated_token::mint = mint_player,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub user_player_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_quote,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub user_quote_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        close = user,
        seeds = [b"earnings", campaign.key().as_ref()],
        bump = earnings.bump,
    )]
    pub earnings: Account<'info, Earnings>,

    #[account(
        seeds=[b"config".as_ref()],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        associated_token::mint = mint_quote,
        associated_token::authority = earnings,
    )]
    pub vault_quote: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}