use anchor_lang::prelude::*;
use crate::{state::Config, error::CustomError};

pub fn set_fee(ctx: Context<SetFee>, fee_pre: u16, fee_pos: u16) -> Result<()> {
    // Validate fee values (assuming fees are in basis points, max 100%)
    require!(fee_pre <= 10000, CustomError::InvalidFeeValue);
    require!(fee_pos <= 10000, CustomError::InvalidFeeValue);

    // Update fee values in config
    let config = &mut ctx.accounts.config;
    config.fee_pre = fee_pre;
    config.fee_pos = fee_pos;

    Ok(())
}

#[derive(Accounts)]
pub struct SetFee<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [b"config".as_ref()],
        bump = config.bump,
        has_one = authority @ CustomError::Unauthorized,
    )]
    pub config: Account<'info, Config>,
}