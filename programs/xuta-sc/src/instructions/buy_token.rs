use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    burn, mint_to, transfer_checked, Mint, TokenAccount, TokenInterface,
};

use crate::state::{campaign, receipt, Campaign, CampaignStatus, Config, ErrorCode, Receipt};

impl<'info> BuyToken<'info> {
    pub fn buy_token(ctx: Context<BuyToken>, amount: u64, receipt_bump: u8) -> Result<()> {
        let campaign = &ctx.accounts.campaign;
        let config = &ctx.accounts.config;
        let vault = &ctx.accounts.vault_quote;
        let user = &ctx.accounts.user;

        // Check vault matches stored vault
        require!(vault.key() == campaign.vault, ErrorCode::InvalidVault);

        // Check campaign status is Active
        require!(
            campaign.status == CampaignStatus::Active,
            ErrorCode::CampaignNotActive
        );

        // Check current time is within [initial_date, due_date]
        let clock = Clock::get()?;
        let now = clock.unix_timestamp;
        require!(
            now >= campaign.initial_date && now <= campaign.due_date,
            ErrorCode::CampaignNotStarted
        );
        require!(
            now >= campaign.initial_date && now <= campaign.due_date,
            ErrorCode::CampaignEnded
        );

        let mut totalAmount  = amount; 

        let mut refund= false;
        // Calculate tokens to mint based on ratio
        let mut token_number = totalAmount
            .checked_div(campaign.ratio as u64)
            .ok_or(ErrorCode::InvalidRatioOrAmount)?;

        if ( token_number + campaign.current_tokens > campaign.target_amount) {
            token_number = campaign.target_amount - campaign.current_tokens;
            totalAmount = token_number.checked_mul(campaign.ratio as u64)
            .ok_or(ErrorCode::InvalidRatioOrAmount)?;
            refund = true;
        }

        let fee_amount = token_number
            .checked_mul(config.fee_pre as u64)
            .ok_or(ErrorCode::FeeError)?;

        let token_amount = totalAmount
            .checked_sub(fee_amount)
            .ok_or(ErrorCode::MathError)?;

        // Transfer quote from user to vault (checked)
        let cpi_accounts = anchor_spl::token_interface::TransferChecked {
            from: ctx.accounts.user_quote_ata.to_account_info(),
            to: vault.to_account_info(),
            authority: user.to_account_info(),
            mint: ctx.accounts.mint_quote.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer_checked(cpi_ctx, totalAmount, 6)?; // maybe decimals should be in the campaign
                                               
        //  Update campaign stats
        let mut campaign_data = &mut ctx.accounts.campaign;
        campaign_data.current_tokens = campaign_data
            .current_tokens
            .checked_add(token_amount)
            .ok_or(ErrorCode::MathError)?;
        campaign_data.current_fees = campaign_data
            .current_fees
            .checked_add(fee_amount)
            .ok_or(ErrorCode::MathError)?;

        // Initialize receipt
        let receipt = &mut ctx.accounts.receipt;
        receipt.authority = user.key();
        receipt.token_amount = token_amount;
        receipt.fee_amount = fee_amount;
        receipt.bump = receipt_bump;

        // implementation to be made
        Ok(())
    }
}

#[derive(Accounts)]
pub struct BuyToken<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mint::token_program = token_program
    )]
    pub mint_quote: InterfaceAccount<'info, Mint>,

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
        associated_token::mint = mint_quote,
        associated_token::authority = campaign,
    )]
    pub vault_quote: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = user,
        seeds = [b"receipt", user.key().as_ref(), campaign.key().as_ref()],
        bump,
        space = 8 + Receipt::INIT_SPACE,
    )]
    pub receipt: Account<'info, Receipt>,

    #[account(mut)]
    pub user_quote_ata: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
