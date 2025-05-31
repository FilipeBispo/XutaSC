use anchor_lang::prelude::*;
use anchor_spl::token::{
    transfer_checked, Mint, TokenAccount, Token, TransferChecked,
};

use crate::{state::{Campaign, CampaignStatus, Config, Receipt}, error::CustomError};

impl<'info> BuyToken<'info> {
    pub fn buy_token(ctx: Context<BuyToken>, amount: u64) -> Result<()> {
        let campaign = &ctx.accounts.campaign;
        let config = &ctx.accounts.config;
        let vault = &ctx.accounts.vault_quote;
        let user = &ctx.accounts.user;

        // Check vault matches stored vault
        require!(vault.key() == campaign.vault, CustomError::InvalidVault);

        // Check campaign status is Active
        require!(
            campaign.status == CampaignStatus::Active,
            CustomError::CampaignNotActive
        );

        // Check current time is within [initial_date, due_date]
        let clock = Clock::get()?;
        let now = clock.unix_timestamp;
        require!(
            now >= campaign.initial_date && now <= campaign.due_date,
            CustomError::CampaignNotStarted
        );
        require!(
            now >= campaign.initial_date && now <= campaign.due_date,
            CustomError::CampaignEnded
        );

        let mut total_amount  = amount; 

        // Calculate tokens to mint based on ratio
        let mut token_number = total_amount
            .checked_div(campaign.ratio as u64)
            .ok_or(CustomError::InvalidRatioOrAmount)?;

        if (token_number + campaign.current_tokens > campaign.target_amount) {
            token_number = campaign.target_amount - campaign.current_tokens;
            total_amount = token_number.checked_mul(campaign.ratio as u64)
            .ok_or(CustomError::InvalidRatioOrAmount)?;
        }

        let fee_amount = token_number
            .checked_mul(config.fee_pre as u64)
            .ok_or(CustomError::FeeError)?;

        let token_amount = total_amount
            .checked_sub(fee_amount)
            .ok_or(CustomError::MathError)?;

        // Transfer quote from user to vault (checked)
        let cpi_accounts = TransferChecked {
            from: ctx.accounts.user_quote_ata.to_account_info(),
            to: vault.to_account_info(),
            authority: user.to_account_info(),
            mint: ctx.accounts.mint_quote.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer_checked(cpi_ctx, total_amount, 6)?; // maybe decimals should be in the campaign
                                               
        //  Update campaign stats
        let mut campaign_data = &mut ctx.accounts.campaign;
        campaign_data.current_tokens = campaign_data
            .current_tokens
            .checked_add(token_amount)
            .ok_or(CustomError::MathError)?;
        campaign_data.current_fees = campaign_data
            .current_fees
            .checked_add(fee_amount)
            .ok_or(CustomError::MathError)?;

        // Initialize receipt
        let receipt = &mut ctx.accounts.receipt;
        receipt.authority =  user.key();
        receipt.token_amount = receipt.token_amount + token_amount;
        receipt.fee_amount = receipt.fee_amount + fee_amount;
        receipt.bump = ctx.bumps.receipt;

        msg!("amount: {:?}", amount);
        msg!("token_amount: {:?}", token_amount);
        msg!("fee_amount: {:?}", fee_amount);
        msg!("token_number: {:?}", token_number);
        // implementation to be made
        Ok(())
    }
}

#[derive(Accounts)]
pub struct BuyToken<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub mint_quote: Account<'info, Mint>,

    #[account(
        mut,
        has_one = mint_quote,
        seeds = [b"campaign", campaign.mint_player.key().as_ref()],
        bump = campaign.campaign_bump,
    )]
    pub campaign: Account<'info, Campaign>,

    #[account(
        seeds=[b"config".as_ref()],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [b"vault", campaign.key().as_ref()],
        bump
    )]
    pub vault_quote: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = user,
        seeds = [b"receipt", user.key().as_ref(), campaign.key().as_ref()],
        bump,
        space = 8 + Receipt::INIT_SPACE,
    )]
    pub receipt: Account<'info, Receipt>,

    #[account(mut)]
    pub user_quote_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
