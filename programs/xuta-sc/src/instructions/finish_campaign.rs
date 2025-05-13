use anchor_lang::prelude::*;

use anchor_spl::token::{transfer_checked, Mint, Token, TokenAccount, TransferChecked};

use crate::{state::{Campaign, CampaignStatus, Institution}, error::CustomError};

pub fn finish_campaign(ctx: Context<FinishCampaign>) -> Result<()> {
    let campaign = &mut ctx.accounts.campaign;
    let institution = &mut ctx.accounts.institution;
    let vault = &ctx.accounts.vault;

    // Check vault matches stored vault
    require!(vault.key() == campaign.vault, CustomError::InvalidVault);
    
    // Check if the campaign is active
    require!(
        campaign.status == CampaignStatus::Active,
        CustomError::CampaignNotActive
    );

    // Check if the campaign has reached its target
    if campaign.current_tokens >= campaign.target_amount {
        campaign.status = CampaignStatus::Successful;
        // Transfer the corresponding quote from the vault account back to the user's USDC account (CPI to SPL Token program)
        
        let authority_seeds = &[
            b"campaign",
            ctx.accounts.campaign.mint_player.as_ref(),
            &[ctx.accounts.campaign.campaign_bump],
        ];
        let signer_seeds = &[&authority_seeds[..]];
        let cpi_transfer_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.vault.to_account_info(),
                to: ctx.accounts.user_token_account_quote.to_account_info(),
                authority: ctx.accounts.campaign.to_account_info(),
                mint: ctx.accounts.mint_quote.to_account_info(),
            },
            signer_seeds,
        );

        let amount = ctx.accounts.campaign.target_amount.checked_sub(ctx.accounts.campaign.current_fees)
        .ok_or(CustomError::MathError)?;

        let decimals = ctx.accounts.mint_quote.decimals;

        transfer_checked(cpi_transfer_ctx, amount, decimals)?;
    } else {
        campaign.status = CampaignStatus::Failed;
    }

    // Update institution's active campaigns status
    institution.has_active_campaigns = false;

    Ok(())
}

#[derive(Accounts)]
pub struct FinishCampaign<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

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
        associated_token::mint = mint_quote,
        associated_token::authority = campaign,
        associated_token::token_program = token_program,
    )]
    pub vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        has_one = authority,
        has_one = vault,
        seeds = [b"campaign", campaign.mint_player.key().as_ref()],
        bump = campaign.campaign_bump,
    )]
    pub campaign: Account<'info, Campaign>,

    #[account(
        mut,
        seeds = [b"institution".as_ref(), institution.name.as_ref()],
        bump = institution.bump,
    )]
    pub institution: Account<'info, Institution>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}