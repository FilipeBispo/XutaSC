use anchor_lang::prelude::*;

use anchor_spl::token::{Mint, Token, TokenAccount, MintTo, mint_to};

use crate::{
    error::CustomError,
    state::{Campaign, CampaignStatus, Receipt,},
};

impl<'info> RedeemToken<'info> {
    pub fn redeem_token(ctx: Context<RedeemToken>) -> Result<()> {
        let campaign = &ctx.accounts.campaign;
        let receipt = &ctx.accounts.receipt;
        let user_token_account = &ctx.accounts.user_token_account;
        let mint = &ctx.accounts.mint_player;

        // Check campaign status is open to Successfull
        require!(
            campaign.status == CampaignStatus::Successful,
            CustomError::CampaignNotOpenForRefund
        );
        
        let total_amount = receipt.fee_amount + receipt.token_amount;
        require!(total_amount > 0, CustomError::NoReceiptAmount);
        
        // Calculate the amount of tokens to mint: total_amount\ratio
        let tokens_to_mint = total_amount
            .checked_div(campaign.ratio as u64)
            .ok_or(CustomError::MathError)?;

        // Ensure we are minting valid amount of tokens later we might want to decide a max of tokens per user
        require!(tokens_to_mint > 0, CustomError::InvalidTokenAmount);
        
        let cpi_program = ctx.accounts.token_program.to_account_info();

        let cpi_accounts = MintTo{
            mint: mint.to_account_info(),
            to: user_token_account.to_account_info(),
            authority: campaign.to_account_info(),
        };

        let authority_seeds = &[
            b"campaign",
            campaign.mint_player.as_ref(),
            &[campaign.campaign_bump],
        ];
        let signer_seeds = &[&authority_seeds[..]];

        let ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        mint_to(ctx, tokens_to_mint)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct RedeemToken <'info>{
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
    )]
    pub mint_player: Account<'info, Mint>,

    /// The campaign state PDA, storing conversion ratio and configuration.
    #[account(
        mut,
        has_one = mint_player,
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
    pub user_token_account: Account<'info, TokenAccount>,

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
