use anchor_lang::prelude::*;
use crate::{state::Config, error::CustomError};

pub fn set_authority(ctx: Context<SetAuthority>) -> Result<()> {
    // Update the authority in the config account
    let config = &mut ctx.accounts.config;
    config.authority = ctx.accounts.new_authority.key();
    Ok(())
}

#[derive(Accounts)]
pub struct SetAuthority<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [b"config".as_ref()],
        bump = config.bump,
        has_one = authority @ CustomError::Unauthorized,
    )]
    pub config: Account<'info, Config>,

    /// CHECK: This is the new authority that will be set
    pub new_authority: UncheckedAccount<'info>,
}