use anchor_lang::prelude::*;


pub fn redeem_receipt(_ctx: Context<RedeemReceipt>, amount: u64) -> Result<()> {
    let campaign = &ctx.accounts.campaign;
    let user_receipt_ata = &ctx.accounts.user_receipt_ata;
    let vault = &ctx.accounts.vault;

    // Get the total number of receipt tokens the user holds in their account
    let user_receipt_balance = user_receipt_ata.amount;
    require!(user_receipt_balance > 0, ErrorCode::NoReceiptTokens);

    // Calculate the amount of USDC to return: receipt_amount * ratio
    let ratio = campaign.ratio;
    let redeem_amount = user_receipt_balance
        .checked_mul(ratio)
        .ok_or(ErrorCode::MathOverflow)?;
    // Ensure the vault has enough USDC to fulfill the redemption
    require!(vault.amount >= redeem_amount, ErrorCode::VaultInsufficientFunds);

    // Burn all receipt tokens from the user's receipt account (the user is the authority of their token account)
    let cpi_burn_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        token::Burn {
            mint: ctx.accounts.receipt_mint.to_account_info(),
            from: user_receipt_ata.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        },
    );
    token::burn(cpi_burn_ctx, user_receipt_balance)?;

    // Transfer the corresponding USDC from the vault account back to the user's USDC account (CPI to SPL Token program)
    let vault_authority_bump = *ctx.bumps.get("vault_authority").ok_or(ErrorCode::MissingBump)?;
    let authority_seeds = &[
        b"vault_authority",
        campaign.key().as_ref(),
        &[vault_authority_bump]
    ];
    let signer_seeds = &[&authority_seeds[..]];
    let cpi_transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        token::Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.user_deposit_ata.to_account_info(),
            authority: ctx.accounts.vault_authority.to_account_info(),
        },
        signer_seeds,
    );
    token::transfer(cpi_transfer_ctx, redeem_amount)?;

    Ok(())
}

/// Accounts for the `redeem_receipt` instruction.
#[derive(Accounts)]
pub struct RedeemReceipt<'info> {
    /// The campaign state PDA, storing conversion ratio and configuration.
    #[account(
        seeds = [b"campaign"],
        bump
    )]
    pub campaign: Account<'info, Campaign>,

    /// The user's wallet account (signer) redeeming their receipt tokens.
    #[account(mut)]
    pub user: Signer<'info>,

    /// The mint of the deposit token (USDC).
    #[account(address = campaign.deposit_mint)]
    pub deposit_mint: Account<'info, Mint>,

    /// The vault token account holding USDC, controlled by the program.
    #[account(
        mut,
        address = campaign.vault,
        token::mint = deposit_mint,
        token::authority = vault_authority
    )]
    pub vault: Account<'info, TokenAccount>,

    /// The PDA acting as authority over the vault and receipt mint.
    /// Must match the PDA derived in the campaign.
    #[account(
        seeds = [b"vault_authority", campaign.key().as_ref()],
        bump
    )]
    pub vault_authority: UncheckedAccount<'info>,

    /// The mint of the custom receipt token.
    #[account(
        mut,
        address = campaign.receipt_mint,
        mint::authority = vault_authority
    )]
    pub receipt_mint: Account<'info, Mint>,

    /// The user's token account for USDC where the refunded tokens will be transferred.
    #[account(
        mut,
        token::mint = deposit_mint,
        token::authority = user
    )]
    pub user_deposit_ata: Account<'info, TokenAccount>,

    /// The user's token account for the receipt token from which all tokens will be burned.
    #[account(
        mut,
        token::mint = receipt_mint,
        token::authority = user
    )]
    pub user_receipt_ata: Account<'info, TokenAccount>,

    /// The SPL Token program.
    pub token_program: Program<'info, Token>,
}