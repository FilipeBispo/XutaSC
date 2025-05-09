use anchor_lang::prelude::*;

use anchor_spl::token::{
    transfer_checked, Mint, TokenAccount, Token, TransferChecked,
};

use crate::{state::{Campaign, CampaignStatus, Receipt}, error::CustomError};

pub fn refund_receipt(ctx: Context<RefundReceipt>, amount: u64) -> Result<()> {
    let campaign = &ctx.accounts.campaign;
    let vault = &ctx.accounts.vault;
    let receipt = &ctx.accounts.receipt;


        // Check vault matches stored vault
    require!(vault.key() == campaign.vault, CustomError::InvalidVault);

            // Check campaign status is Active

    require!(
        campaign.status != CampaignStatus::Successful 
        || campaign.status != CampaignStatus::Paused,
        CustomError::CampaignNotOpenForRefund
    );

    // Get the total number of receipt amount the user holds in their account
    let redeem_amount = receipt.fee_amount + receipt. token_amount;
    require!(redeem_amount > 0, CustomError::NoReceiptAmount);

    // Calculate the amount of tokens to remove: receipt_amount\ratio
    let tokens_to_remove = redeem_amount
        .checked_div(campaign.ratio as u64)
        .ok_or(CustomError::MathError)?;
    // Ensure the vault has enough USDC to fulfill the redemption
    require!(vault.amount >= redeem_amount, CustomError::InsuficientFunds);

    // Transfer the corresponding USDC from the vault account back to the user's USDC account (CPI to SPL Token program)
    let authority_seeds = &[
        b"campaign",
        campaign.mint_player.as_ref(),
        &[campaign.campaign_bump]
    ];
    let signer_seeds = &[&authority_seeds[..]];
    let cpi_transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        TransferChecked {
            from: vault.to_account_info(),
            to: ctx.accounts.user_token_account_quote.to_account_info(),
            authority: campaign.to_account_info(),
            mint: ctx.accounts.mint_quote.to_account_info(),
        },
        signer_seeds,
    );
    transfer_checked(cpi_transfer_ctx, redeem_amount, 6)?;

    Ok(())
}

/// Accounts for the `redeem_receipt` instruction.
#[derive(Accounts)]
pub struct RefundReceipt<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub mint_quote: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_quote,
        associated_token::authority = user,
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
        associated_token::mint = mint_quote,
        associated_token::authority = campaign,
        associated_token::token_program = token_program,
    )]
    pub vault: Box<Account<'info, TokenAccount>>,


    /// The mint of the custom receipt token.
    #[account(
        mut,
        close = user,
        seeds = [b"receipt", user.key().as_ref(), campaign.key().as_ref()],
        bump,
    )]
    pub receipt: Account<'info, Receipt>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}