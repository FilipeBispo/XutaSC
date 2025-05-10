use anchor_lang::prelude::*;
use crate::{state::Config, error::CustomError};

pub fn set_institutions_authority(ctx: Context<SetInstitutionsAuthority>) -> Result<()> {
    // Update the institutions authority in the config account
    let config = &mut ctx.accounts.config;
    config.institution_authority = ctx.accounts.new_institution_authority.key();
    Ok(())
}

#[derive(Accounts)]
pub struct SetInstitutionsAuthority<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [b"config".as_ref()],
        bump = config.bump,
        has_one = authority @ CustomError::Unauthorized,
    )]
    pub config: Account<'info, Config>,

    /// CHECK: This is the new institutions authority that will be set
    pub new_institution_authority: UncheckedAccount<'info>,
}